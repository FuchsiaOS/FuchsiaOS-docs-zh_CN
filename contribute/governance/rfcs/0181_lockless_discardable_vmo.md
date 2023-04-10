<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0181" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
{# Fuchsia RFCs use templates to display various fields from _rfcs.yaml. View the #}
{# fully rendered RFCs at https://fuchsia.dev/fuchsia-src/contribute/governance/rfcs #}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Summary

Lockless discardable VMOs are a sub-type of [discardable VMOs]
(https://fuchsia.dev/fuchsia-src/contribute/governance/rfcs/0012_zircon_discardable_memory)
where they are allowed to be accessed even if not locked. The VMO may still be
discarded at any time, and users must be tolerant of sudden complete data loss.
To improve usage in the motivating use case the VMO hinting operations are
extended to also cover discardable VMOs.


## Motivation

Tracing is a workload that wants to have a minimal performance overhead and
generates large bursts of data. Current discardable VMOs are not suitable for
this due to the overhead of locking and unlocking. Using regular anonymous VMOs
has the current downside that due to the bursts of data produced, tracing may
cause the system to OOM.

The tracing engine is able to tolerate arbitrary data loss. That is, the tracing
VMO can be arbitrarily decommitted, even mid read or write, without causing
problems, beyond the loss of the trace data. Further, it is considered
preferable for some trace data to be lost, than have the whole system OOM.

Allowing discardable VMOs to be used when unlocked solves all these problems
since:

 * System OOM will be avoided by tracing VMO contents being discarded.
 * Writing to the trace VMOs incurs no additional overhead.
 * Tracing is already tolerant of data loss and will perceive VMO discard as
   data loss.

Although the tracing system is tolerant of data loss, it is desirable to only
discard tracing if needed to avoid OOM. There is no mechanism to control when
the kernel will perform reclamation on discardable VMOs, or how it balances
discarding VMOs versus other forms of reclamation. To improve the reliability of
trace generation, without compromising OOM prevention, we therefore would like a
way to inform the kernel to reclaim from these only if needed.

## Stakeholders

_Facilitator:_

TBD

_Reviewers:_

TBD

_Consulted:_

TBD

_Socialization:_

eieio@ and rashaeqbal@

## Design

### Creation Flag

Lockless discardable VMOs are a sub-type of regular discardable VMOs and shall
be created by passing the option `ZX_VMO_DISCARDABLE_LOCKLESS` to
`zx_vmo_create`, instead of the regular `ZX_VMO_DISCARDABLE` option. The created
discardable VMO still starts in the unlocked state, the only difference is that
whilst in the unlocked state reads and writes to the VMO will succeed instead of
generating errors.

### Controlling Reclamation

The VMO range operations `ZX_VMO_OP_DONT_NEED` and `ZX_VMO_OP_ALWAYS_NEED` will
have their API definitions extended to cover discardable VMOs, and not just
pager backed VMOs.

### Locking behavior

Although a lockless discardable can be used without being locked, it is still
allowed to be locked through the `ZX_VMO_OP_LOCK` and related operations.
Locking behaves just like a regular discardable VMO, and while locked the
contents cannot be discarded.

## Implementation

The syscall flag of `ZX_VMO_DISCARDABLE_LOCKLESS` needs to get passed down
through VMO creation and become a flag in the internal VMO.

The existing discardable VMOs implementation explicitly cause faults when the
VMO is unlocked by the couple of relevant code paths checking for the
discardable state, and generating an error if unlocked. These checks just need
to be extended to then not generate the fault if it is a LOCKLESS variant.

Movement of discardable VMOs on the internal discardable lists also needs to be
changed slightly. Since lockless VMO might always have pages to discard, and not
just if they have been locked since last discarded, they need to be left on the
discardable list after being discarded.

For reclamation changes the range for `ZX_VMO_OP_ALWAYS_NEED` will be ignored
and always promoted to full VMO. Separate discardable list will be created, with
hinted VMOs being placed on it. This list will be evicted from under ALMOST_OOM
pressure. `ZX_VMO_OP_DONT_NEED` will

 * Immediately discard the VMO if it's unlocked
 * Move the VMO to the regular discard list if it wasn't already.

Lockless discardable VMOs will not require the ZX_VM_ALLOW_FAULTS flag when
mapped.

This could all be done as a single small CL to fuchsia.git with no other
dependencies.

## Performance

No performance impact is expected to VMO operations, with a single additional
boolean check being added to what are already slowpath scenarios.

## Security considerations

None

## Privacy considerations

None

## Testing

Tests will be added to the Zircon core tests suite.

## Documentation

Relevant syscall docs need to be updated:

 * `zx_vmo_create` - Document `ZX_VMO_DISCARDABLE_LOCKLESS`.
 * `zx_vmo_op_range` - Update statements about VMO type supported for
   `ZX_VMO_OP_ALWAYS_NEED` and `ZX_VMO_OP_DONT_NEED`.
 * `zx_vmar_map` - Update requirements of `ZX_VM_ALLOW_FAULTS` for lockless
   discardable VMOs.


## Drawbacks, alternatives, and unknowns

The original discardable RFC describes an [optimizing using atomics]
(https://fuchsia.dev/fuchsia-src/contribute/governance/rfcs/0012_zircon_discardable_memory#faster_locking_api_with_atomics)
that could provide the majority of the desired performance increase. There are
two drawbacks to this approach:

 1. An atomic API between kernel and userspace is untrodden ground and would be
    a complicated space to explore, which is why it was not done in the original
    proposal.
 2. Locking pages, even if its efficient, is still not needed and actually goes
    against our memory desires of these VMOs to be discardable absolutely
    anytime.

## Prior art and references

None
