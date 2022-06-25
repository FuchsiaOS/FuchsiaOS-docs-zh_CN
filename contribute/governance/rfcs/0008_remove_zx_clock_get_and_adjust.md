{% set rfcid = "RFC-0008" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

## Summary

We propose a five step process to deprecate the `zx_clock_get` and
`zx_clock_adjust` syscalls, migrate the remaining users to the replacement
syscalls that have been available since 2019, and finally remove the original
syscalls.


## Motivation

Fuchsia currently includes two independent sets of syscalls for interacting with
time:

1. The original design used `zx_clock_get` to read monotonic, UTC, and thread
   clocks maintained by the kernel and used `zx_clock_adjust` to write to the
   UTC clock. This design did not provide any way to slew clocks, define
   additional clocks on different products, or communicate the status of clocks.
2. In 2019 the `zx_clock_create`, `zx_clock_read`, `zx_clock_update` and
   `zx_clock_get_details` syscalls were added to perform rich management of
   clock objects, letting userspace define and maintain any number of clocks.
   `zx_clock_get_monotonic` was also added to read the kernel's monotonic clock.

Fuchsia currently defines two different UTC clocks; a kernel UTC clock that may
be read using `zx_clock_get` and a userspace UTC clock that may be read using
`zx_clock_read`. The time synchronization system attempts to keep these clocks
aligned, but since they offer different features some discrepancy is inevitable.

This proposal defines the deprecation plan for the original time syscalls,
completing a migration that began in 2019 during the design of the replacement
syscalls were added. Removing these syscalls addresses the ongoing (and
increasing) operational and cognitive costs of maintaining two semi-compatible
ways to manage time.


## Implementation

`zx_clock_get` and `zx_clock_adjust` each accept a `clock_id` argument to
specify which of three possible timelines should be used. It is helpful to
consider these migration for these three timelines separately:

* `ZX_CLOCK_MONOTONIC` - Calls to `zx_clock_get(ZX_CLOCK_MONOTONIC)` may be
  directly replaced by a call to `zx_clock_get_monotonic`. This replacement
  syscall is simpler to use and frequently offers better performance.
* `ZX_CLOCK_THREAD` - Calls to `zx_clock_get(ZX_CLOCK_THREAD)` may be replaced
  by a call to `zx_object_get_info` using the topic `ZX_INFO_THREAD_STATS`. This
  replacement provides more flexibility and better aligns with thread execution
  time as a thread property.
* `ZX_CLOCK_UTC` - In the 2019 time syscalls UTC is managed in userspace by
  the Fuchsia platform instead of directly by the kernel. Most language runtimes
  have already been modified to read the userspace UTC clock so in most cases
  calls to `zx_clock_get(ZX_CLOCK_UTC)` should be replaced by the standard UTC
  calls in the language runtime. Where necessary the `zx_utc_reference_get`
  function may be used to acquire a read only clock handle to pass into
  `zx_clock_read`. Calls to `zx_clock_adjust(ZX_CLOCK_UTC)` may be replaced by a
  call to `zx_clock_update` using a read/write clock handle acquired from
  `fuchsia.time.Maintenance`.

We propose a five step process for removing `zx_clock_get` and
`zx_clock_adjust`:

1. Update documentation to mark the syscalls as deprecated
2. Migrate all known users to the replacement syscalls
3. Stop maintaining kernel UTC clock and remove `zx_clock_adjust`
4. Remove `zx_clock_get` declaration from the SDK
5. Remove `zx_clock_get` implementation from Zircon

### Step 1 - Update documentation to mark the syscalls as deprecated

This step is complete. The `zx_clock_get` and `zx_clock_adjust` calls are
clearly marked as deprecated and the documentation includes the alternative
solutions that we recommend above.

### Step 2 - Migrate all known users to the replacement syscalls

This step is in progress. [fxr/433865](https://fxrev.dev/433865) recently moved
the standard language runtime UTC functions to the userspace UTC clock. This has
removed the majority of `zx_clock_get` usages but a long tail remains from a
wide range of different call sites across different repositories.

We will burn down these remaining usages across stem and the petals in Global
Integration using code search to locate the calls and the
`syscalls_zx_clock_get_type_*` kcounters to track our progress. This will be a
lengthy process given the resources available and the fact that rolling an
upstream change to downstream repositories can often take weeks or months.

Where a language runtime provides a wrapper around `zx_clock_get` or
`zx_clock_adjust` (for example the `fuchsia_zircon::Time::get` function
in Rust) we will remove each wrapper once no more clients are using it.

### Step 3 - Stop maintaining kernel UTC clock and remove `zx_clock_adjust`

Maintaining an accurate time in `ZX_CLOCK_UTC` requires calls to `zx_clock_get`
and `zx_clock_adjust` from drivers and from several test components. These calls
will no longer be possible after step 4 so we cease maintaining kernel UTC as a
separate step.

In step 3 we will remove the `ZX_CLOCK_UTC` synchronization, cause a crash
for any clients that attempt to read `ZX_CLOCK_UTC`, and remove the
`zx_clock_adjust` syscall entirely (this syscall is only used for setting UTC
and only invoked by a small number of privileged clients).

After step 2 we should have already high confidence that there are no remaining
components using UTC in Global Integration, but causing a client crash on read
attempts lets us detect any omissions quickly.

Specifically we will:

1. Remove all tests that verify `ZX_CLOCK_UTC`.
2. Modify `zx_clock_get(ZX_CLOCK_UTC)` to flag the caller as a policy violator.
2. Remove the calls to `zx_clock_adjust` from the real time clock drivers (and
   remove the fallback RTC driver entirely since this was its only purpose)
3. Remove `zx_clock_adjust` entirely.

### Step 4 - Remove `zx_clock_get` declaration from the SDK

After step 3 we may have succeeded in removing all calls to `zx_clock_get` at
head but still need to support older prebuilt binaries that depend on
`zx_clock_get`. In this case we will remove the `zx_clock_get` declaration in
the SDK without removing the zircon implementation. At this stage any attempt to
compile code using the `zx_clock_get` will lead to a compilation failure.

If no prebuilt binaries depend on `zx_clock_get` we will proceed directly to
step 5.

### Step 5 - Remove `zx_clock_get` implementation from Zircon

Once no prebuilt binaries depend on `zx_clock_get` (as determined by their
symbol imports) we will remove `zx_clock_get` entirely, along with the
associated documentation.


## Performance

This proposal will lead to a negligible increase in overall system performance
by encouraging more widespread use of the more efficient
`zx_clock_get_monotonic` syscall and simplifying the operation the RTC drivers.


## Security considerations

This proposal does not impact the security of managing monotonic or thread time.

In other operating systems UTC management vulnerabilities have been exploited to
perform various rollback attacks. This proposal improves the security of
managing UTC time by providing more fine grained access control.

When using the `zx_clock_adjust` syscall the root resource is required to change
UTC time. This means a component that needs the ability to adjust time also
gains powerful unrelated capabilities and it means any component with the root
resource has the power to modify UTC whether it needs this power or not.

Once this proposal is implemented, the only way to modify UTC will be through
the read/write clock handle distributed via `fuchsia.utc.Maintenance`. This
protocol is explicitly routed to only the components that need it and they
gain no additional capabilities along with this handle.


## Privacy considerations

This proposal does not impact privacy at the time of implementation.

This proposal would enable the removal of UTC time as an ambient authority in
the future (a process could be launched without the userspace UTC clock handle
and would no longer have access to the kernel UTC clock). Potentially this could
be used to increase privacy for certain types of data processing.


## Testing

Each of the time management syscalls and the time synchronization infrastructure
that depends on them are covered by unit, integration, and end to end tests
today. These tests would be simplified somewhat by the removal of the two
syscalls and the second UTC clock.


## Documentation

Documentation updates are included in the implementation section above.


## Drawbacks, alternatives, and unknowns

The costs of this proposal are small - we estimate a few hours a week work from
2-3 engineers in the component platform and kernel teams over a period of 1-3
quarters.

The alternatives mainly revolve around not cleaning up this technical debt or
cleaning up less of this technical debt; for example it would be possible to
stop maintaining the kernel UTC clock but not remove the associated syscalls.
These alternatives reduce cost in the short term but significantly increase it
in the long term.


## Prior art and references

[kernel_objects/clock](reference/kernel_objects/clock.md) provides a clear
overview of the operation of userspace clocks and is recommended reading.
