{% set rfcid = "RFC-0068" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

## Summary

This RFC describes a mechanism for userspace applications to hint to the kernel
the relative importance of user pager backed memory, so that the kernel can take
those hints into consideration when deciding which pages to evict under memory
pressure.

## Motivation

Most executables in Fuchsia are served out of blobfs, an immutable file system,
which employs a user pager to read in pages from the disk on demand. Having
blobs populated in memory by a user pager enables us to evict pages when the
system comes under memory pressure; when those pages are accessed again, the
user pager will simply read them back in.

Repaging comes at a performance cost however - threads that require the pages
have to block until the page fault is serviced by the user pager, which involves
context switches and disk IO. To try to reduce the performance impact of
repaging, the kernel uses a least-recently-used scheme to find pages to evict.

This eviction scheme is not always optimal, and can sometimes cause us to evict
pages that haven't been accessed in a while but are required on performance
critical paths, e.g. the audio stack. On the other end of the spectrum, we have
pages that are not required by applications anymore and could be safely evicted,
e.g. "inactive" blobs that do not have any clients. These inactive pages get
mixed in with pages from active blobs that currently do have clients but have
simply not been accessed in a while. We could benefit from moving these inactive
pages to the front of the line for eviction.

## Design

### Overview

Userspace applications that access blobs, and blobfs which serves these blobs,
have more context than the kernel around the relative importance of pages. They
could pass along this additional information to the kernel as eviction hints.

This RFC proposes an API which can be used to hint in two directions - "consider
these pages for eviction first" and "protect these pages from being evicted".
Using the previous examples, inactive blobs fall under the former category,
while pages required for performance critical tasks fall under the latter.

### The Hinting API

- The `zx_vmo_op_range()` syscall will be extended to support two new ops.
  `ZX_VMO_OP_DONT_NEED` hints that the specified range is not needed anymore and
  should be considered for eviction first. `ZX_VMO_OP_ALWAYS_NEED` hints that
  the specified range is important and should be protected from eviction for as
  long as possible. The kernel will only consider evicting pages tagged with the
  `ZX_VMO_OP_ALWAYS_NEED` hint when the system is about to reboot due to an OOM.
  (Motivation behind the op names is covered in more detail in the Alternatives
  section.)

- Similarly the `zx_vmar_op_range()` syscall will be extended to support the ops
  `ZX_VMAR_OP_DONT_NEED` and `ZX_VMAR_OP_ALWAYS_NEED`.

- These hints will only apply to VMOs and mappings where the kernel can reclaim
  pages under memory pressure, currently only user pager backed VMOs. As more
  reclaimable VMO types are added in the future, e.g. discardable memory, the
  hinting API can be extended to cover them as well. The somewhat generic op
  names allow for future expansion to cases where pages are not strictly evicted
  from memory. For example, we might consider using the hinting API for
  compressible anonymous memory in the future, where reclamation is simply
  in-memory compression.

- Eviction hints will be a no-op for VMOs and mappings that do not support
  kernel reclamation. Hinting on a generic anonymous VMO or a physical VMO will
  not alter how its pages are committed or decommitted. This allows for ease of
  use on the client side, where the VMO type does not need to be determined
  beforehand. Since this API is only being used to pass along hints, without any
  requirement for concrete guarantees from the kernel, the kernel can choose to
  ignore the hints where not applicable.

### Current eviction strategy

The kernel tracks pages committed in user pager backed VMOs with a set of 4 LRU
page queues. When pages are first committed, they start off in queue 1. Every 10
seconds, the page scanner rotates the queues, moving pages from the `i`-th queue
to the `i+1`-th queue. When the system comes under memory pressure, the kernel
evicts pages from the 4th queue. Any time a page in any of the queues is
accessed, it is moved to the head of queue 1 - queue 1 tracks the most recently
used pages.

### OP_DONT_NEED

- An additional `inactive` pager queue will be introduced to track inactive
  pages in LRU order. This queue is not a part of the existing LRU page queues,
  and so the page scanner will not rotate pages into or out from it. It only
  tracks inactive pages in LRU order but does not fit within the larger LRU
  scheme across all user pager backed pages.

- `OP_DONT_NEED` will move any committed pages in the specified range to the
  `inactive` queue. When memory pressure kicks in, the kernel will first
  consider evicting pages from the `inactive` queue before moving on to the
  oldest LRU page queue.

- If a page in the `inactive` queue is accessed, it will be moved out of that
  queue into the first LRU page queue, losing the `OP_DONT_NEED` hint that was
  previously indicated. From that point on, it will move down the other LRU
  queues as any other "active" page. This ensures that the `OP_DONT_NEED` hint
  does not erroneously override the actual access patterns of a page.

### OP_ALWAYS_NEED

- Pages in the specified range will be committed and a new `always_need` flag
  will be set in the corresponding `vm_page_t`.

  - The `always_need` flag does not have any effect on how the page scanner
    rotates queues. This helps retain information about how active an
    `always_need` tagged page is, so that if it is required to be evicted to
    prevent an OOM, we only evict the oldest pages, preventing the memory
    reclamation from being too disruptive.

  - The `always_need` flag only affects removal of the page by eviction under
    non-OOM conditions - when the kernel is evicting pages under memory
    pressure, it will simply skip over pages with this flag set. The
    `always_need` flag does *not* prevent a page from being freed by means other
    than eviction (e.g. a decommit, VMO resize, or VMO destruction).

- `OP_ALWAYS_NEED` will move all pages in the specified range to the first LRU
  page queue. Newly committed pages start off in the first LRU queue anyway. For
  pages that were already committed, this op is counted as a new access, to keep
  the behavior consistent with newly committed pages.

- Pages with `always_need` set will only be considered for eviction when the
  kernel is trying to prevent an OOM. This approach helps us achieve a good
  balance, ensuring that page eviction does not hamper performance during the
  normal working of the system, and at the same time does not lock down memory,
  thereby increasing OOM rates.

### Interaction with VMO clones

The hinting ops will only apply to pages in the root VMO in a pager backed
hierarchy, since the root VMO is the only one that is directly backed by a pager
source. Clones get their initial content from the root VMO. Any pages that are
committed in clones are forked copies that are owned by the clone, and cannot be
evicted. So when the hinting ops are used on a VMO clone (or a mapping that maps
a VMO clone), the hints will apply to pages in the root VMO that the clone can
see within the specified range, i.e. pages that have not been forked in the
clone.

### Interaction between the two hinting ops

An `OP_ALWAYS_NEED` hint will have precedence over `OP_DONT_NEED`. The
`always_need` flag is sticky and cannot be unset once it has been set. This
prevents an `OP_DONT_NEED` coming from a clone from overriding an
`OP_ALWAYS_NEED` by a different clone.

- Per the description of the ops above, an `OP_DONT_NEED` that follows an
  `OP_ALWAYS_NEED` will move the page to the inactive queue, but since the
  `always_need` flag is set, it won't be evicted.

- An `OP_ALWAYS_NEED` following an `OP_DONT_NEED` will move the page from the
  inactive page queue to the first LRU queue, and set the `always_need` flag.

### Handle rights for the op_range syscalls

The hinting operations do not require any specific rights on the VMO / VMAR
handle; the syscalls will succeed with any or no rights. However, the kernel
has the freedom to ignore the hints where they are not meaningful, based on a
combination of the handle rights, the underlying VMO type, the backing page
source, the mapping permissions etc. In other words, the syscall will always
succeed, but the hints can effectively be a no-op in some cases.

This approach gives us flexibility with the implementation and would be easier
to extend in the future to more VMO types. It also aligns with the larger intent
behind having hints implemented as no-ops for unsupported cases, rather than
failing the syscall. A client can always hint - the kernel will decide how to
interpret that hint and may even choose to ignore it.

### Relation with discardable memory

[Discardable memory](0012_zircon_discardable_memory.md) is another means for
userspace to influence the kernel's memory reclamation strategy. It allows
clients to create anonymous VMOs that are marked "discardable", and are locked
and unlocked to indicate when they are in use or are eligible for reclamation
respectively. While eviction hints serve a similar goal as discardable memory,
i.e. providing the kernel with more information around memory reclamation, there
are some key differences.

 - Discardable memory only pertains to anonymous VMOs. Eviction hints as
   proposed in this RFC apply to pager-backed VMOs.

 - The lock / unlock operations (`zx_vmo_op_range` with `ZX_VMO_OP_LOCK/UNLOCK`)
   used with discardable VMOs have stricter semantics. If a VMO is locked, the
   kernel *cannot* discard its pages. On the other hand, even if a VMO's pages
   are indicated as `OP_ALWAYS_NEED`, the kernel can choose to evict them
   regardless, if the memory pressure conditions are dire. This is because the
   VMO is pager-backed and discarded pages can be repopulated on demand.

 - Locking can be viewed as a completely separate operation from hinting; the
   two are not interchangeable and can coexist. If eviction hints are extended
   in the future to support discardable memory, locking will still continue to
   remain the means for clients to indicate when a VMO is in use, prohibiting
   the kernel from discarding it. Eviction hints can then just layer on top to
   express a relative priority order for reclamation *only when applicable* -
   unlocked discardable VMOs tagged with `OP_DONT_NEED` can be discarded before
   those tagged with `OP_ALWAYS_NEED`.

## Implementation

This is a new API so there are no dependencies at this stage. The kernel-side
implementation can be done in isolation. Once the API has been implemented,
userspace clients can start adopting it.

## Performance

`OP_ALWAYS_NEED` will improve performance on paths where paging in evicted pages
currently results in observable user impact. A known use case today is playing
audio after eviction, which sometimes results in glitches because of paging
activity.

`OP_DONT_NEED` will cause the specified pages to be evicted sooner, resulting in
delays when they are paged in later. However, this is intended and the client is
expected to be aware of this impact. Besides, these pages might already be
getting evicted today as well. `OP_DONT_NEED` is meant to be used to explicitly
indicate inactive pages, which will not be accessed anyway, making them eligible
for eviction.

The hinting ops are meant to make the memory reclamation system more robust,
where evicted pages are more likely to stay evicted for longer. The memory
health of the system should improve as a result.

## Security considerations

None.

## Privacy considerations

None.

## Testing

- Core tests / unit tests that exercise the new API from multiple VMO clones.
- Unit tests that verify the eviction behavior on the kernel side.

## Documentation

The Zircon syscall documentation will need to be updated to include the new API.

## Drawbacks, alternatives, and unknowns

### Locking instead of OP_ALWAYS_NEED

Using `zx_vmo_op_range()` with `ZX_VMO_OP_LOCK` could be an alternative to using
`ZX_VMO_OP_ALWAYS_NEED`. However, the kernel is required to provide stronger
guarantees with locking - the committed pages need to be pinned in memory,
preventing the kernel from reclaiming them until they are unlocked. This can
exert additional memory pressure on the system causing it to OOM at a faster
rate.

Figuring out exactly what pages to lock can be quite challenging as well. In
most cases clients will likely be more conservative and lock more pages than
required. The memory cost of this can be prohibitive, and taken to the extreme
can undermine much of the memory benefit of having demand paging in the first
place.

### Eviction specific op names

We could use more specific op names like `ZX_VMO_OP_EVICT_FIRST/LAST` and
`ZX_VMO_OP_RECLAIM_FIRST/LAST`, which describe the associated eviction behavior
more precisely. However, these cannot be extended to more generic future
applications. `RECLAIM_FIRST/LAST` might be a little broader than
`EVICT_FIRST/LAST` and could be applied to various definitions of "reclamation",
e.g. in-memory compression which does not strictly evict the pages, but it still
ties the ops with a notion of memory reclamation.

`OP_ALWAYS_NEED` and `OP_DONT_NEED` allow us to better capture user intent,
instead of defining the associated kernel action that is expected to occur. This
affords greater flexibility of interpretation for these ops in the future. It
also allows us to retain some similarity with `madvise` that developers might be
familiar with.

### Potential abuse of hints

It is possible for clients to abuse the API, accidentally or intentionally, to
try and prevent a lot of pages from being evicted with unrestrained use of
`OP_ALWAYS_NEED`. While this memory will still be reclaimed to prevent an OOM,
it can exert additional memory pressure on the system at other times.

This risk however is along the same lines as clients creating a large VMO today
and committing all its pages. Once we have some policy in the future around
controlling memory usage in general (e.g. space banks), we can have the same
policy feed into the hinting logic.

Modeling the API purely as hints also provides the kernel flexibility to simply
ignore hints if needed. We could do something like ignore `OP_ALWAYS_NEED` hints
beyond a certain limit on `always_need` tagged pages, or skip a page during
eviction only a certain number of times. In the extreme case we could also
deprecate the hinting ops altogether by turning them into no-ops.

### Future work

As mentioned earlier, the hinting ops can be extended to apply to use cases
beyond user pager backed VMOs. Other types of reclaimable memory like
discardable memory seem like a natural extension. We could also extend it to
generic anonymous memory, and use it to drive actions other than strict
eviction.

The interaction between `OP_DONT_NEED` and `OP_ALWAYS_NEED` (and the
`always_need` flag being sticky) might evolve in the future as the VM system
becomes more mature. The current choice is an implementation simplification
driven by current use cases, not a fundamental requirement of the hinting API
itself.

`OP_DONT_NEED` and `OP_ALWAYS_NEED` leave room for a midpoint `OP_WILL_NEED`
which can serve as a prefetch hint, indicating that a range is required in the
future, but does not necessarily need to be protected from eviction beyond that
point. A user pager could use this hint to read ahead pages.

## Prior art and references

On Linux [`madvise`](https://man7.org/linux/man-pages/man2/madvise.2.html)
supports `MADV_WILLNEED` and `MADV_DONTNEED`.

