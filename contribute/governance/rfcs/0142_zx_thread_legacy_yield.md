<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0142" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}

# {{ rfc.name }}: {{ rfc.title }}

<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Summary

We propose adding a new syscall `zx_thread_legacy_yield` , that causes
the calling thread to yield the cpu to another waiting thread. Additionally,
we would remove the special case of `zx_nanosleep` when the deadline is
equal to zero. The `legacy` in the syscall's name exists as an indication
of the availability of more suitable mechanisms to achieve the desired
behavior.

## Motivation

The behavior of `zx_nanosleep` deviates from how deadlines are treated in all
other syscalls, that is if the deadline is before the current time, then it
returns immediately. Currently, this is a special case where a deadline value
of zero means yield.

Furthermore, it is impossible to determine at runtime, whether the deadline
was intentionally set to zero or was the result of a calculation. This would
simplify how `zx_nanosleep` works, reducing the special cases a developer
needs to be aware of. A deadline can be manually set to zero, in order to
cause the thread to yield, while it can also be zero as the result of the
next wake up (which means to continue right now).

## Stakeholders

Who has a stake in whether this RFC is accepted? (This section is optional but
encouraged.)

_Facilitator:_

The person appointed by FEC to shepherd this RFC through the RFC
process.

_Reviewers:_

_Consulted:_

_Socialization:_ The design was initially discussed in a design doc.

## Design

This proposal consist of the following changes to the Zircon syscall API:

1. Addition of `zx_thread_legacy_yield`
2. Simplification of `zx_nanosleep`

Where `zx_thread_legacy_yield` is defined as:

```c
/// Yields cpu to another waiting thread.
/// `options`: Reserved for future extension. Must be zero.
/// @blocking
zx_status_t zx_thread_legacy_yield(uint32_t options);
```

When `zx_thread_legacy_yield` is called on a user space application,
the calling thread yields the cpu. The `options` parameter could be
used to introduce hints later.

Calling `zx_thread_legacy_yield` with `options` equal to zero is
guaranteed to return `ZX_OK`.

When `zx_nanosleep` is called with a deadline that is greater than the time in
the monotonic clock, then calling thread is put to sleep until the deadline
(plus slack) is met. On the other hand, when the deadline has a
value equal or prior to the time in the monotonic clock, then it returns
immediately. But if the deadline is zero, it will cause the calling thread
to yield. After introducing `zx_thread_legacy_yield` the goal is to remove
the special handling of zero deadlines.

Additionally, the `syscalls_zx_nanosleep_zero_duration` kernel counter must be
removed, while `syscalls_zx_thread_legacy_yield` counter is introduced.

## Implementation

`zx_nanosleep(0)` has 2 callsites(which provide an implementation of
`sched_yield`) outside the fuchsia tree, and 7 callsites in-tree.
Which involves splitting the work in four CLs.

1. Introduce new syscall `zx_thread_legacy_yield`.
2. Migrate in-tree users to `zx_thread_legacy_yield`.
3. Migrate out of tree users.
4. Remove yield overload from `zx_nanosleep`.

CL (1) will include documentation, language binding updates, and CL (4) will
update the documentation as well. (1) and (2) could be merged into a single CL,
but separating implementation from migration seems like a cleaner path.

## Performance

There is no known impact on performance, other than a slight improvement in
`zx_nanosleep` path, since a branch is removed.

## Backwards Compatibility

The users of `zx_nanosleep(0)` are not limited to the stem repository.

## Security considerations

The behavior of the existing code and the new code will be exactly the same,
since `zx_nanosleep(0)` (now) is equivalent to `zx_thread_legacy_yield(0)`.
In order to prevent any security issues from arising, `zx_nanosleep` will
remain unchanged until all use cases of `zx_nanosleep(0)` have been migrated
to `zx_thread_legacy_yield(0)`.

## Privacy considerations

This proposal has no impact on privacy.

## Testing

The nature of `zx_thread_legacy_yield` makes it hard to test reliably, but
at the
very least we can verify that the return codes match the expectations.

## Documentation

We will add additional documentation for `zx_thread_legacy_yield` and
update the entry for `zx_nanosleep` to remove the mention of zero deadline
being a special case.

## Drawbacks, alternatives, and unknowns

The biggest drawback and concern is the existence of
`zx_thread_legacy_yield` encouranging busy waits patterns, but
`sched_yield` provides this mechanism already. These concerns can be mitigated
through proper documentation and best practices. Since the target users of
this syscall are drivers, any other usage should be looked at carefully.

Another alternative that was considered was changing the magic value to
another (e.g. `INT_MIN` ). This would indeed address the calculated deadline
problem, while still leaving a special case. This alternative would require
updating out of tree user as well,  which is why it requires the same effort as
adding a new syscall. The biggest drawback is that doing so, will leave
`zx_nanosleep` deadline handling being special from another operations that
deal with deadlines, such as `zx_object_wait` .

## Prior art and references

*  `zx_nanosleep` uses 0 as a special value, to indicate "yield". The problem
is that there is code that effectively does a deadline calculation which can
compute 0, in this case the intention is not to yield, quite the opposite:
execute the next statement "now".

* Code with `yield` is usually an indication of something done wrong.
Unfortunately it is not uncommon to see it in drivers whose hardware lacks
the proper signaling mode.
