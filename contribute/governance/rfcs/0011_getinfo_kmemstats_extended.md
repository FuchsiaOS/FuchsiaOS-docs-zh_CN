{% set rfcid = "RFC-0011" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

## Summary

This RFC proposes adding a new topic - `ZX_INFO_KMEM_STATS_EXTENDED`, to the
`zx_object_get_info()` syscall. The new topic will return a single record of
type `zx_info_kmem_stats_extended_t`, a struct consisting of all the fields in
`zx_info_kmem_stats_t`, plus some additional fields that are more expensive to
collect.

## Motivation

The `ZX_INFO_KMEM_STATS` topic does not expose any metrics for the amount of
memory that can be reclaimed by the kernel under memory pressure. There is
currently only a counter for `free_bytes`, which is the amount of physical free
memory on the system. This number alone is not very useful, and can be
misleading.  In practice the amount of memory "available" is more than free
memory, because the kernel can reclaim memory by evicting pages under memory
pressure.

Exposing metrics for the various kinds of available memory on the system will
allow for more useful memory diagnostics.

## Design

The `zx_info_kmem_stats_extended_t` struct contains all of the fields present in
`zx_info_kmem_stats_t` and the following additional fields:

```
    // The amount of memory committed to pager-backed VMOs.
    uint64_t vmo_pager_total_bytes;

    // The amount of memory committed to pager-backed VMOs, that has been most
    // recently accessed, and would not be eligible for eviction by the kernel
    // under memory pressure.
    uint64_t vmo_pager_newest_bytes;

    // The amount of memory committed to pager-backed VMOs, that has been least
    // recently accessed, and would be the first to be evicted by the kernel
    // under memory pressure.
    uint64_t vmo_pager_oldest_bytes;

    // The amount of memory committed to discardable VMOs that is currently
    // locked, or unreclaimable by the kernel under memory pressure.
    uint64_t vmo_discardable_locked_bytes;

    // The amount of memory committed to discardable VMOs that is currently
    // unlocked, or reclaimable by the kernel under memory pressure.
    uint64_t vmo_discardable_unlocked_bytes;

```

The `ZX_INFO_KMEM_STATS_EXTENDED` topic has the same constraints as
`ZX_INFO_KMEM_STATS`, i.e. it requires the root resource.

The `vmo_pager_*` fields will be populated by computing the lengths of the
pager-backed queues. The `vmo_discardable_*` fields are currently unimplemented
and will be set to 0.

## Implementation

The `zx_object_get_info()` syscall can be extended in a single CL.

## Performance

This is a new topic, so the performance of existing callers of
`zx_object_get_info()` will remain unaffected. New callers should choose between
using `ZX_INFO_KMEM_STATS` and `ZX_INFO_KMEM_STATS_EXTENDED` depending on the
level of detail they require. `ZX_INFO_KMEM_STATS_EXTENDED` provides more
detailed metrics and is therefore more expensive, so it should be used only when
the additional metrics are required.

## Security considerations

None.

## Privacy considerations

None.

## Testing

Zircon core-tests will be written that query the `ZX_INFO_KMEM_STATS_EXTENDED`
topic.

## Documentation

The syscall documentation for `zx_object_get_info()` will need to be updated.

## Drawbacks, alternatives, and unknowns

The alternative to adding a new topic is simply extending the existing
`ZX_INFO_KMEM_STATS` topic to expose the additional metrics. This would involve
extending the `zx_info_kmem_stats_t` struct to include the new fields. The
downside with this approach is that existing users of `ZX_INFO_KMEM_STATS` would
take a performance hit, since the new fields that are exposed by
`ZX_INFO_KMEM_STATS_EXTENDED` can be expensive to compute. For example,
computing the pager-backed counts is linear in the total number of pages across
all VMOs that are backed by the pager. Existing clients of `ZX_INFO_KMEM_STATS`
would end up unnecessarily incurring this additional performance cost for
metrics they might not even require. Instead, the `ZX_INFO_KMEM_STATS_EXTENDED`
topic is meant to be used only when clients require this additional level of
detail.

## Prior art and references

On Linux [`/proc/meminfo`](https://man7.org/linux/man-pages/man5/proc.5.html)
includes counters for MemAvailable, Active and Inactive.

