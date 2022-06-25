<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0123" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->
<!-- This should begin with an H2 element (for example, ## Summary).-->

## Summary

This RFC proposes a mechanism by which a userspace agent may interact with the
kernel regarding CPU performance, both to update the performance scales used by
the kernel scheduler and to query its state.

## Motivation

In order to schedule work effectively across CPUs in heterogeneous
architectures such as big.LITTLE, the Zircon kernel scheduler models the
relative performances of CPUs. At time of writing, the [performance
scales](#performance-scale) that describe these relative performances are
static, provided by data in the ZBI.

When performing thermal CPU throttling of a big.LITTLE system, the frequencies
of big and little cores are typically not scaled by identical factors, so their
relative performances change dynamically. Unlike most other operating systems,
in Fuchsia, modifications to core frequencies are performed in userspace, and
the scheduler must be notified across the kernel boundary of changes to relative
CPU performances. That communication necessitates new syscalls.

## Design

### Performance scale {#performance-scale}

#### Concept {#performance-scale-concept}

Before considering the proposed syscalls, it is useful to understand the concept
of performance scale, which already exists within the kernel scheduler.
Performance scale describes the ratio of the performance of a CPU operating at
its current speed to a system-dependent reference performance, where performance
can be measured using any suitable metric, such as
[DMIPS](https://en.wikipedia.org/wiki/Dhrystone). At time of writing &mdash; but
not necessarily in the future &mdash; the reference performance is that of the
most powerful CPU operating at its maximum speed, so 1.0 is the maximum
performance scale value. Typically, a vendor provides a performance value for
each CPU operating at a nominal speed, and performance is assumed to vary
linearly with CPU frequency.

For example, on a big.LITTLE system, a vendor might provide performance data
indicating that a big core at its maximum speed performs twice the DMIPS as a
little core operating at its own maximum speed. If the reference performance
corresponds to a big core running at its maximum speed, then that operating
condition corresponds to performance scale 1.0, while a little core at its
maximum speed would have performance scale 0.5. Reducing a big core's speed
by 25% gives it a new performance scale of 0.75, while reducing the little
core's speed by 25% changes its performance scale to 0.375.

More precisely, if f<sub>ref</sub> is a reference frequency with known
performance scale s<sub>ref</sub>, then frequency f<sub>new</sub> has
performance scale
s<sub>new</sub>=s<sub>ref</sub>f<sub>new</sub>/f<sub>ref</sub>. In general, one
reference frequency is required for each distinct CPU architecture in the
system.

Typically, only a fixed number of frequency combinations are supported by a
given system. For example, it is typical that CPUs in the same cluster must have
the same frequency, and that each cluster only supports a relatively small
number of distinct frequencies. However, it is beyond the scope of the kernel to
track which performance scales are valid. As such, the kernel trusts userspace
to provide realistic values, and it will use values provided via the proposed
API to the best of its ability.

#### Fixed point representation {#performance-scale-representation}

To avoid using floating point numbers, performance scales are represented using
fixed point numbers, specified by a struct

```c
  typedef struct zx_cpu_performance_scale {
    uint32_t integral_part;
    uint32_t fractional_part;  // Increments of 2**-32
  } zx_cpu_performance_scale_t;
```

`integral_part` and `fractional_part` describe the integer and fractional parts,
respectively, with `fractional_part` specifying increments of 2<sup>-32</sup>.
Conversion between real and fixed point representations should be done according
to the following functions:

```c++
zx_status_t ToFixedPoint(double real, zx_cpu_performance_scale_t* scale) {
  double integer;
  double fraction = std::modf(real, &integer);

  // Converting from double to fixed point should fail if the input's integer
  // part is too large.
  if (integer > static_cast<double>(UINT32_MAX)) {
    return ZX_ERR_INVALID_ARGS;
  }

  scale->integral_part = static_cast<uint32_t>(integer);

  // Rounding down the fractional part is suggested but should not matter
  // much in practice. A difference of 1 in the output is a difference of only
  // 2**-32 in the corresponding real value.
  scale->fractional_part = static_cast<uint32_t>(std::ldexp(fraction, 32));

  return ZX_OK;
}

double FromFixedPoint(zx_cpu_performance_scale_t scale) {
  return static_cast<double>(scale.integral_part)
    + std::ldexp(scale.fractional_part, -32);
}
```

### Syscall 1: `zx_system_set_performance_info`

The first syscall allows a userspace agent to set performance scales used by the
kernel scheduler:

```c
zx_status_t zx_system_set_performance_info(
    zx_handle_t resource,
    uint32_t topic,
    const void* new_info,
    size_t info_count
);
```

Its arguments are:

- `resource`: A resource that grants permission to this call. Must be
  `ZX_RSRC_SYSTEM_CPU_BASE`, a new resource introduced specifically for this
  API, or the call will fail.

- `topic`: The type of performance referenced by this call. Must be
  `ZX_CPU_PERF_SCALE`, which will be defined upon proposal implementation.

- `new_info`: A valid `zx_cpu_performance_info_t[]`, whose elements are
  specified by

  ```c
  typedef struct zx_cpu_performance_info {
      uint32_t logical_cpu_number;
      zx_cpu_performance_scale_t performance_scale;
  } zx_cpu_performance_info_t;
  ```

  where `zx_cpu_performance_t` is [defined
  above](#performance-scale-representation).

  `logical_cpu_number` specifies the CPU whose info is described by the struct,
  using the same numbering scheme utilized by the kernel. Each
  `logical_cpu_number` must be a valid CPU identifier. Elements of `new_info`
  must be sorted in order of strictly increasing `logical_cpu_number` (and
  consequently, each `logical_cpu_number` may appear only once).

  `performance_scale` represents the new performance scale for the indicated
  CPU, and it should correspond to the CPU's new frequency as [described
  previously](#performance-scale-concept). However, the kernel does not validate
  inputs against supported CPU frequencies; any positive value is allowed as an
  input.

  An input scale of `{.integral_part = 0, .fractional_part = 0}` is invalid so
  as not to be confused with a request to offline a core, a procedure with a
  distinct mechanism that is expected to have a different API in the future.

  The kernel may internally override a valid input with the nearest value that
  the scheduler can utilize. For example, at time of writing, the maximum
  supported performance scale is 1.0. Therefore, if `performance_scale`
  represents a value larger than 1.0, then the kernel will internally clamp it
  to `{.integral_part = 1, .fractional_part = 0}`.

  If the call to `zx_system_set_performance_info` fails, then the kernel takes
  no action, and `new_info` has no effect.

  If the call succeeds, then the kernel scheduler will utilize modified
  performance scales corresponding to `new_info` beginning with the next
  reschedule operation, which in general occurs sometime after the call returns.
  The kernel will not modify its performance scales for CPUs not referenced in
  `new_info`.

  Changes made by this call will persist until reboot or until they
  are overridden by further use of this API.

- `info_count`: The number of elements in `new_info`. Must be positive and no
  greater than the number of CPUs in the system.

#### Error conditions

`ZX_ERR_BAD_HANDLE`

- `resource` is not a valid handle.

`ZX_ERR_WRONG_TYPE`

- `resource` is not a valid resource handle or is not of kind
  `ZX_RSRC_KIND_SYSTEM`.

`ZX_ERR_INVALID_ARGS`

- `topic` is not `ZX_CPU_PERF_SCALE`.
- `new_info`  is an invalid pointer.
- `new_info` is not sorted by strictly increasing `logical_cpu_number`.

`ZX_ERR_OUT_OF_RANGE`

- `resource` is of kind `ZX_RSRC_KIND_SYSTEM` but is not equal to
  `ZX_RSRC_SYSTEM_CPU_BASE`.
- `info_count` is `0` or exceeds the number of CPUs.
- A `logical_cpu_number` was invalid.
- An input `performance_scale` was `{.integral_part = 0, .fractional_part = 0}`.

#### Intended usage {#intended-usage-set}

`zx_system_set_performance_info` should be used to notify the kernel of
changes in CPU performance whenever CPU frequency is changed. The API supports
specification of performance scales for only a subset of CPUs because different
CPUs may be controlled by different entities.

If a CPU's frequency is to be decreased, it is recommended that
`zx_system_set_performance_info` be called before the frequency change has
occurred. Doing so gives the kernel scheduler the opportunity to reduce load on
that CPU before its capacity is decreased. (The scheduler is expected to respond
quickly enough that no further coordination is needed; this expectation will be
confirmed once support is implemented.)

Conversely, if a CPU's frequency is to be increased, it is recommended that
`zx_system_set_performance_info` be called after the frequency change has
occurred, notifying the scheduler of new capacity only once it is available.

In either case, should an update to CPU frequency fail, the caller must update
the kernel scheduler based on the resulting CPU state. The caller should attempt
to determine the post-failure CPU frequency and use that to inform a separate
call to `zx_system_set_performance_info`. If the frequency cannot be determined
(e.g. if an associated driver has failed outright), the caller should make a
pessimistic (low) guess as to the resulting CPU speed. This recommendation may
evolve as it is given further consideration; see for example
[fxbug.dev/84685](https://fxbug.dev/84685).

The new API will ultimately be utilized by a to-be-developed "CPU Manager"
component that will be responsible for userspace administration of CPUs. Rather
than interacting directly with CPU drivers, agents that wish to modify CPU
frequency will register requests with CPU Manager, which will coordinate
frequency changes with updates to the kernel as described in this proposal.

CPU Manager will also take over responsibility for thermal throttling of CPU
&mdash; the motivating use case for this proposal &mdash; from Power Manager.

### Syscall 2: `zx_system_get_performance_info`

The second syscall retrieves performance information for all CPUs:

```c
zx_status_t zx_system_get_performance_info(
    zx_handle_t resource,
    uint32_t topic,
    void* info,
    size_t info_count
    size_t* output_count
);
```

Its arguments are:

- `resource`: A resource that grants permission to this call. Must be
  `ZX_RSRC_SYSTEM_CPU_BASE`.

- `topic`: Either `ZX_CPU_PERF_SCALE` or `ZX_CPU_DEFAULT_PERF_SCALE`, which will
  be defined upon proposal implementation. The topic determines the content
  written to `info`, described below.

- `info`: A valid `zx_cpu_performance_info_t[]` with length equal to the
  number of CPUs in the system.

  If the call fails, `info` is unmodified.

  If the call succeeds, then upon return `info` contains one element for each
  CPU, ordered by increasing `logical_cpu_number`. Each element's
  `performance_scale` is populated based on `topic`:

     - `ZX_CPU_PERF_SCALE`: `performance_scale` stores the kernel's current
        performance scale for the indicated CPU. The value provided reflects the
        most recent call to `zx_system_set_performance_info` even if the next
        reschedule operation has not yet taken place.

     - `ZX_CPU_DEFAULT_PERF_SCALE`: `performance_scale` stores the default
        performance scale used by the kernel on boot for the indicated CPU.

- `info_count`: Length of the `info` array; must equal the number of CPUs in the
  system.

- `output_count`: If the call succeeds, this will contain the number of elements
  written to `info`. If the call fails, its value is unspecified.

#### Error conditions

`ZX_ERR_BAD_HANDLE`

- `resource` is not a valid handle.

`ZX_ERR_WRONG_TYPE`

- `resource` is not a valid resource handle or is not of kind
  `ZX_RSRC_KIND_SYSTEM`.

`ZX_ERR_INVALID_ARGS`

- `topic` is not `ZX_CPU_PERF_SCALE` or `ZX_CPU_DEFAULT_PERF_SCALE`.
- `info` is an invalid pointer.

`ZX_ERR_OUT_OF_RANGE`

- `resource` is of kind `ZX_RSRC_KIND_SYSTEM` but is not equal to
  `ZX_RSRC_SYSTEM_CPU_BASE`.
- `info_count` does not equal the total number of CPUs in the system.

#### Intended usage

The behavior under `ZX_CPU_PERF_SCALE` allows a userspace agent to query
performance scales for diagnostic purposes. This may be useful, for example, for
an agent to assess system state when it first starts or as a signal to a crash
report.

The behavior under `ZX_CPU_DEFAULT_PERF_SCALE` allows an agent to
confirm that the performance scales with which it is configured agree with those
in use by the kernel.

## Implementation

### Kernel

- The new syscalls must be implemented, gated by a new resource
  `ZX_RSRC_SYSTEM_CPU_BASE`.

- The kernel scheduler must be modified to support dynamic performance scales,
  updating them to use the most recent values provided by
  `zx_system_set_performance_info`, and additionally exposing its currently-used
  and default performance scales to `zx_system_get_performance_info`.

### Component manager

A new protocol `CpuResource` must be defined and must be implemented by
Component Manager to provide the `ZX_RSRC_SYSTEM_CPU_BASE` resource. This
follows a pre-existing pattern for resources that gate syscalls.

## Performance

The new syscalls themselves will take a negligible amount of time to execute, as
they simply touch a small amount of data proportional to the number of CPUs.

Use of `zx_cpu_set_performance_info` will cause the scheduler to distribute work
differently, shifting work towards cores whose performance scales increase
relative to the sum of all performance scales, and away from those whose
performance scales similarly decrease. The rescheduling process itself will not
place a significant amount of load on the scheduler.

Rescheduling will lead to expected changes in system performance. Testing of
these changes is equivalent to testing the scheduler for functional correctness
and is addressed in [Testing](#testing).

## Security considerations

Both new syscalls are gated by the new resource handle
`ZX_RSRC_SYSTEM_CPU_BASE`. For `zx_system_set_performance_info`, this protection
addresses the clear concern of malicious interference with the scheduler. For
`zx_system_get_performance_info`, there is the subtler concern of data leakage;
an untrusted entity should not be trusted to know the kernel's performance
scales, which will typically provide information about the system's supported
P-states.

## Privacy considerations

This proposal has no meaningful impact on privacy.

## Testing {#testing}

- Core tests will be added to exercise basic success and failure criteria.
- Unit tests will be added to validate the scheduler's handling of updated
  performance scales. They will verify that if a deadline thread is pinned to a
  CPU, and that CPU's performance scale is modified by factor &alpha;, then the
  actual time allotted to the thread is multiplied by 1/&alpha;.

## Documentation

The Zircon syscall documentation will be updated to include the new API.

## Drawbacks, alternatives, and unknowns

### Generality

A more general interface was considered, such as a `zx_set_cpu_properties`
syscall that could eventually handle additional interactions between the kernel
and CPUs, like offlining. Ultimately, we opted for a narrow interface because
very few clients of this interface are expected, keeping the cost of future
changes to the proposed interface relatively small. Requirements placed on a
more general interface would be largely guesswork at this point.

### Alternative call structure

As an alternative to the set-only operation of `zx_system_set_performance_info`,
a combined get/set operation was considered that returns the prior performance
scales for CPUs whose scales were modified. This was intended as a means of
ensuring that the caller is capable of reverting performance scale changes
should lower-level execution of the associated frequency change fail.

However, further consideration revealed that a simple reversion of changes would
not be sufficient. This resulted in a more complex set of [failure-handling
recommendations](#intended-usage-set) and led back to the simpler set-only
operation.

Finally, `zx_system_get_performance_info` is needed to support hermetic testing,
in which case direct reversion of changes *is* appropriate, and supports
diagnostic use cases.

### Alternative CPU indexing

We considered using an alternative scheme for indexing CPUs, such as referring
to them by physical CPU number. However, since the kernel has no other need for
such a scheme, it is most consistent with Zircon's limited scope to have the API
use the kernel's existing logical CPU numbers. These numbers are consistent on a
given system, and a client could either maintain a static per-board
configuration to refer to them or potentially access their configuration data
from the ZBI.

### Alternative to performance scale

We considered that, rather than referring to performance scale directly, the new
API might utilize a "speed factor" that the scheduler would apply to the base
performance scale for a given CPU. Doing so would reduce the amount of
context-specific information a client would need to know; rather than
understanding the relative performances between CPUs, it would only need to know
the ratio between a CPU's new frequency and its nominal frequency.

We opted against this approach because performance scale is intended to be used
in a fundamental way for CPU thermal throttling on a heterogeneous system, so
the one anticipated client of this API would receive no meaningful benefit from
using speed factors instead. Meanwhile, we would incur the cost both of defining
the new concept and modifying the scheduler to utilize it.

### Maximum performance scale

This proposal originally represented performance scale using a `uint32_t` that
represented real values in \[0.0, 1.0\]. In particular, this allowed
representation of a maximum value of 1.0.

While 1.0 is the maximum performance scale supported by the Zircon scheduler at
time of writing, we decided to allow inputs that represent values greater than
1.0 to support future use cases, such as a turbo mode. Additionally, the
previous representation was not fixed point, so it led to values
that could not be directly used by the scheduler.

#### Representation of `performance_scale`

`performance_scale` was originally a `uint64_t`, with the upper 32 bits holding
the integer part and the lower 32 bits holding the fractional part. This would
have produced 32 bits of padding between fields in `zx_cpu_performance_info_t`,
which introduced a potential leakage vector. The new representation avoids that
pitfall.

### Allowed values for `performance_scale`

Careful consideration was given to what values `zx_system_set_performance_info`
should allow as inputs for `performance_scale`. A value representing 0.0 was
determined to be too easily confused for an instruction to offline a CPU &mdash;
an action that Zircon does not currently support but is expected to in the
future using a different API. As such, a value representing 0.0 was determined
to be an error.

Very small values warranted special attention as well. For example, an input of
`{.integral_part = 0, .fractional_part = 1}` would represent 2<sup>-32</sup>,
which could reasonably be treated as 0.0, effectively rendering the
corresponding core offline. While this would be possible to address by enforcing
a minimum allowed value, any such threshold would currently be arbitrary and
would further complicate the contract between the kernel and userspace. We felt
it most straightforward to treat the new API as a hinting mechanism and leave
the kernel with the freedom to override inputs if it needs to do so without
exposing internal details related to such a choice.

### Future work

#### Configuration management

Ideally, userspace agents would use the ZBI to share the exact same CPU
configuration data utilized by the kernel scheduler. It is unclear whether doing
so is currently practical.

Additionally, care must be taken to ensure that both the kernel and userspace
agents associate default performance scales with the same nominal frequencies.

#### Lower bounds on performance scales

In principle, the scheduler can determine minimum performance scales that the
system should maintain based on current deadline threads and CPU load. Dynamic
versions of these bounds would be an important input to a userspace agent that
attempts to utilize lower CPU frequencies for energy efficiency. An additional
option to `zx_system_get_performance_info` would provide a natural means to
expose them.

#### CPU attribution

Some means should be established to associate a thread's attributed CPU time
with the performance of the CPU on which it was scheduled. Such association is
already relevant to the establishment of performance metrics that are robust to
scheduling on big cores versus little cores, and it becomes even more relevant
as we develop the machinery surrounding frequency modifications, as with this
proposal.

#### Guaranteed execution of throttling agent

Reduction of CPU frequencies when performing thermal throttling may lead to CPU
starvation, which in turn may make the throttling agent's process less likely to
be scheduled in a timely fashion. Execution of the throttling agent should be
prioritized in an appropriate manner.

## Prior art and references

Delegation of responsibility for CPU frequency control to userspace is unusual
for operating systems, making prior art on this topic unavailable.
