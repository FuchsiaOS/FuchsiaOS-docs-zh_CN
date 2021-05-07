{% set rfcid = "RFC-0012" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

## Summary

This RFC describes a mechanism for userspace applications to indicate to
the kernel that certain memory buffers are eligible for reclamation. The kernel
is then free to discard these buffers when the system is running low on
available memory.

## Motivation

Managing free memory is a complex problem in an overcommit system like Zircon,
where user applications are allowed to allocate more memory than might currently
be available on the system.  This is accomplished by using Virtual Memory
Objects (VMOs) that are lazily backed by physical pages as portions within them
are committed.

Overestimating the amount of physical memory that will be in use at any point in
time, and failing further memory allocation requests based on that, can leave
free memory on the table. This can affect performance, as a lot of this memory
is used by applications for caching purposes. On the other hand, underestimating
the amount of free memory in use, can cause us to quickly use up all of the
available memory on the system, leading to an out-of-memory (OOM) scenario.
Furthermore, the definition of "free" memory itself is complex.

The Zircon kernel monitors the amount of free physical memory and generates
memory pressure signals at various levels. The purpose of these signals is to
allow userspace applications to scale back (or grow) their memory footprint
based on system-wide free memory levels. While this helps keep the system from
running out of memory, the decoupling of the initiator of these signals (the
kernel) from the responder (user applications) is not ideal. Processes that
respond to memory pressure do not have enough context around how much memory
they should be freeing; the kernel has a better picture of global memory usage
on the system, and it can also take into consideration other forms of
reclaimable memory, e.g. user pager backed memory that can be evicted.

This RFC proposes a mechanism by which the kernel will directly be able
to reclaim userspace memory buffers under memory pressure. There are a few
advantages to this approach:

- It allows for greater control over how much memory is evicted; the kernel can
  look at free memory levels and evict only as much memory as required.
- The kernel can use an LRU scheme to discard memory, which might work better at
  accommodating the current working set in memory.
- Userspace can sometimes be slow to drop memory in response to memory pressure
  signals. In some cases, it might be too late for the system to recover.
- Userspace clients waking up to respond to memory pressure can sometimes
  require more memory.

## Design

### Overview

The discardable memory protocol would roughly work as follows:

1. A userspace process creates a VMO and marks it as *discardable*.
2. Before accessing the VMO either directly (`zx_vmo_read`/`zx_vmo_write`), or
   through a mapping in its address space (`zx_vmar_map`), the process *locks*
   the VMO indicating that it is in use.
3. The process *unlocks* the VMO when done, indicating that it is no longer
   needed. The kernel will consider all unlocked discardable VMOs as eligible
   for reclamation, and will be free to discard them under memory pressure.
4. When the process needs to access the VMO again, it will try to lock it. This
   lock can now succeed in one of two ways.
    - The lock can succeed with the pages of the VMO still intact, i.e. the
      kernel has not discarded it yet.
    - If the kernel has discarded the VMO, the lock will succeed whilst also
      indicating to the client that its pages have been discarded, so that
      they can reinitialize it or take other necessary actions.
5. The process will unlock the VMO again when done. Locking and unlocking can
   repeat in this fashion as often as required.

Note that discardable memory is not meant as a direct replacement for memory
pressure signals.  Watching for memory pressure changes is still valuable for
other component-level decisions, like choosing when to launch memory intensive
activities or threads. In the future, we could also use these signals to kill
idle processes within a component. Memory pressure signals also provide
components greater control over exactly what memory to free and when.

### Discardable Memory API

We can extend the existing `zx_vmo_create()` and `zx_vmo_op_range()` syscalls to
support this feature.

- `zx_vmo_create()` will be extended to support a new `options` flag -
  `ZX_VMO_DISCARDABLE`. This flag can be used in combination with
  `ZX_VMO_RESIZABLE`. However, the general advice about resizable VMOs also
  applies to discardable VMOs - sharing resizable VMOs between processes can be
  dangerous and should be avoided.

- `zx_vmo_op_range()` will be extended to support new operations to provide
  locking and unlocking - `ZX_VMO_OP_LOCK`, `ZX_VMO_OP_TRY_LOCK`, and
  `ZX_VMO_OP_UNLOCK`.

- Locking and unlocking will apply to the entire VMO, so `offset` and `size`
  should span the whole range of the VMO. It is an error to lock and unlock a
  smaller range within the VMO. While the current implementation does not
  strictly require an `offset` and `size`, ensuring that only the entire range
  of the VMO is considered valid allows for adding sub-range support in the
  future without changing the behavior for clients.

- The `ZX_VMO_OP_TRY_LOCK` operation will attempt to lock the VMO and can fail.
  It will succeed if the kernel has not discarded the VMO, and fail with
  `ZX_ERR_NOT_AVAILABLE` if the kernel has discarded it. In case of failure, the
  client is expected to try again with `ZX_VMO_OP_LOCK`, which is guaranteed to
  succeed as long as the arguments passed in are valid. The `ZX_VMO_OP_TRY_LOCK`
  operation is provided as a lightweight option to try locking the VMO without
  having to set up the buffer argument. Clients can also choose to not take any
  action following failure to lock the VMO.

- The `ZX_VMO_OP_LOCK` operation will also require the `buffer` argument, an out
  pointer to a `zx_vmo_lock_state` struct. This struct is meant for the kernel
  to pass back information that the client might find useful, and consists of:
  - `offset` and `size` tracking the locked range: These are the `size` and
    `offset` arguments that were passed in by the client. These are returned
    purely for convenience, so that the client does not need to keep track of
    ranges separately, and instead can directly use the returned struct. If the
    call succeeds, they will always be the same as the `size` and `offset`
    values passed into the `zx_vmo_op_range()` call.
  - `discarded_offset` and `discarded_size` tracking the discarded range: This
    is the maximal range within the locked range that contains discarded pages.
    Not all pages within this range might have been discarded - it is simply a
    union of all the discarded sub-ranges within this range, and can contain
    pages that have not been discarded as well. With the current API, the
    discarded range will span the entire VMO if the kernel has discarded it. If
    undiscarded, both `discarded_offset` and `discarded_size` will be set to
    zero.

- Locking itself does not commit any pages in the VMO. It just marks the state
  of the VMO as "undiscardable" by the kernel. The client can commit pages in
  the VMO using any of the existing methods that apply to regular VMOs, e.g.
  `zx_vmo_write()`, `ZX_VMO_OP_COMMIT`, mapping the VMO and directly writing to
  mapped addresses.

```
// |options| supports a new flag - ZX_VMO_DISCARDABLE.
zx_status_t zx_vmo_create(uint64_t size, uint32_t options, zx_handle_t* out);

// |op| is ZX_VMO_OP_LOCK, ZX_VMO_OP_TRY_LOCK, and ZX_VMO_OP_UNLOCK to
// respectively lock, try lock and unlock a discardable VMO.
// |offset| must be 0 and |size| must the size of the VMO.
//
// ZX_VMO_OP_LOCK requires |buffer| to point to a |zx_vmo_lock_state| struct,
// and |buffer_size| to be the size of the struct.
//
// Returns ZX_ERR_NOT_SUPPORTED if the vmo has not been created with the
// ZX_VMO_DISCARDABLE flag.
zx_status_t zx_vmo_op_range(zx_handle_t handle,
                            uint32_t op,
                            uint64_t offset,
                            uint64_t size,
                            void* buffer,
                            size_t buffer_size);

// |buffer| for ZX_VMO_OP_LOCK is a pointer to struct |zx_vmo_lock_state|.
typedef struct zx_vmo_lock_state {
  // The |offset| that was passed in.
  uint64_t offset;
  // The |size| that was passed in.
  uint64_t size;
  // Start of the discarded range. Will be 0 if undiscarded.
  uint64_t discarded_offset;
  // The size of discarded range. Will be 0 if undiscarded.
  uint64_t discarded_size;
} zx_vmo_lock_state_t;

```

The `zx::vmo` interface will be extended to support the `ZX_VMO_OP_LOCK`,
`ZX_VMO_OP_TRY_LOCK` and `ZX_VMO_OP_UNLOCK` ops with `op_range()`. Rust, Go and
Dart bindings will be updated as well.

This API provides clients with the flexibility to share the discardable VMO
across multiple processes. Each process that needs to access the VMO can do so
independently, locking and unlocking the VMO as required. There is no careful
coordination required amongst processes based on assumptions about the locked
state. The kernel will only consider a VMO eligible for reclamation when no one
has it locked.

#### Restrictions on VMOs

- The discardable memory API is supported only for `VmObjectPaged` types, as
  `VmObjectPhysical` cannot be discarded by definition.

- The API is not compatible with VMO clones (both snapshots and COW clones) and
  slices, since discarding VMOs in a clone hierarchy can lead to surprising
  behaviors. The `zx_vmo_create_child()` syscall will fail on discardable VMOs.

- The `ZX_VMO_DISCARDABLE` flag cannot be used in the `options` argument for
  `zx_pager_create_vmo()`. A major reason for this is that pager-backed VMOs can
  be cloned, and discardable VMOs cannot.  Moreover, discardability is implied
  for pager-backed VMOs, so an additional flag is not required.

#### Interaction with existing VMO operations

The semantics of existing VMO operations will remain the same as before. For
example, `zx_vmo_read()` will not verify that a discardable VMO is locked before
permitting the operation. It is the client's responsibility to ensure that they
have the VMO locked when they are accessing it, to ensure that the kernel does
not discard it from under them. This limits the surface area of this change. The
only guarantee the kernel provides is that it won't discard a VMO's pages while
it is locked.

Any mappings for the VMO will continue to be valid even if the VMO is discarded,
as long as the client locks the VMO before accessing the mappings. Clients do
not need to recreate mappings if the VMO has been discarded.

After the kernel has discarded a VMO, any further operations on it without first
locking it, will fail as if the VMO had no committed pages, and there exists no
mechanism to commit pages on demand.  For example, a `zx_vmo_read()` will fail
with `ZX_ERR_OUT_OF_RANGE`. If the VMO was mapped in a process' address space,
unlocked accesses to mapped addresses will result in fatal page fault
exceptions.

### Kernel Implementation

#### Tracking metadata

- The `options_` bitmask in `VmObjectPaged` will be extended to support a
  `kDiscardable` flag; we're currently only using 4 bits out of 32.
- A new `lock_count` field will be added to `VmObjectPaged`, which will track
  the number of outstanding lock operations on the VMO.
- The kernel will maintain a global list of *reclaimable* VMOs, i.e. all
  unlocked discardable VMOs on the system. The list will be updated as follows:
    - A `ZX_VMO_OP_LOCK` will increment the VMO's `lock_count`. If `lock_count`
      goes from 0->1, the VMO will be removed from the global reclaimable list.
    - A `ZX_VMO_OP_UNLOCK` will decrement the VMO's `lock_count`. If
      `lock_count` drops to 0, the VMO will be added to the global reclaimable
      list.

#### Reclamation logic

Discardable VMOs are added to the global reclaimable list when their
`lock_count` drops to zero, and are removed when locked again. This maintains an
LRU order of all unlocked discardable VMOs on the system. When under memory
pressure, the kernel can dequeue VMOs from this list in order and discard them,
checking the free memory level after each. This is a very simplistic version of
what the reclamation logic might look like in practice. A few more things to
consider are mentioned later.

#### Discard operation

A "discard" is implemented on the kernel side by decommitting all the pages of
the VMO, and setting the internal state of the VMO as `discarded`.
`VmObjectPaged::GetPageLocked()` will fail with `ZX_ERR_NOT_FOUND` if the VMO's
state is `discarded`. The `discarded` state is cleared on a subsequent
`ZX_VMO_OP_LOCK` operation. `GetPageLocked()` is the function all accesses to a
VMO's pages funnel down to, both through `zx_vmo_read/write` syscalls and page
access via VM mappings. This gives us the ability to fail syscalls on a
discarded unlocked VMO, and also to generate exceptions when a discarded
unlocked VMO is accessed through a mapping.

## Implementation

This is a new API so there are no dependencies at this stage. The kernel-side
implementation can be done in isolation. Once the API has been implemented,
userspace clients can start adopting it.

## Performance

The performance implications will vary based on the client side use cases. There
a few things clients can keep in mind when using the API.

- The `zx_vmo_op_range()` syscalls to lock and unlock discardable VMOs before
  access can add noticeable latency on performance critical paths. So the
  syscalls should be used on code paths where an increased latency can be
  tolerated or hidden.
- Clients could also see a boost in performance, due to caches being held in
  memory for longer periods. Buffers that were necessarily dropped by clients
  under memory pressure, can now be held for longer as the kernel will only
  discard as much memory as required. Clients can track this change with cache
  hit rates, number of times buffers need to be re-initialized etc.

## Security considerations

None.

## Privacy considerations

None.

## Testing

- Core tests / unit tests that exercise the new API from multiple threads.
- Unit tests that verify the reclamation behavior on the kernel side, i.e. only
  unlocked VMOs can be discarded.

## Documentation

The Zircon syscall documentation will need to be updated to include the new API.

## Drawbacks, alternatives, and unknowns

### Locking ranges within a VMO

The granularity of reclamation is chosen as the entire VMO, instead of
supporting finer-grained discard operations of ranges within a VMO. There are a
few reasons behind this.

- Reconstructing a VMO which has some pages discarded can be tricky. Considering
  the generic use case, where a VMO is used to represent an anonymous memory
  buffer, repopulating discarded pages would likely be zero fills, which might
  not always make sense w.r.t. the remaining pages that were left undiscarded.
  It might also not be valuable to hold on to only a subset of the VMO's pages,
  i.e. the VMO is meaningful only when it is fully populated.
- VMO granularity keeps the `VmObjectPaged` implementation simple, requiring
  minimal tracking metadata. We don't need to track locked ranges to later match
  with unlocks. There is no complicated range merging involved either.
- It also keeps the reclamation logic fairly lightweight, allowing for large
  chunks of memory to be freed at once. Supporting page granularity instead
  would likely require maintaining page queues, and aging discardable pages,
  similar to the mechanism we use to evict user pager backed pages.

The proposed API does leave the door open to indicate reclaimable ranges in the
future if required, with the `offset` and `size` arguments in
`zx_vmo_op_range()` that are currently unused. Adding range support to the
locking API (page granularity locking) seems like a natural extension to the
current proposal. This will benefit clients where the cost of backing small
discardable regions with individual VMOs can be prohibitive.

### Kernel implementation of discard

When the kernel reclaims a discardable VMO, it decommits its pages and tracks
its state as `discarded`. Future unlocked requests for pages will fail in the
`discarded` state; once the VMO is locked again, the `discarded` state is
cleared. The other alternative here would be to simply decommit pages without
explicitly tracking a state. However tracking the `discarded` state allows for a
stricter failure model. For example, consider the case where a client had a
discardable VMO mapped in its address space, which the kernel discarded at some
point. If the client now tries to access the VMO via the mapping without first
locking the VMO, it will incur a fatal page fault. Whereas if the kernel were to
only decommit pages, a subsequent unlocked access would simply result in a zero
page being silently handed to the client. This could either go undetected, or
result in more subtle errors due to unexpected zero pages.

Another alternative here would be to internally resize the VMO to zero. This
gives us the failure model we want by default, without having to do any explicit
state tracking. However, this requires tracking an internal
implementation-defined size of a VMO, in addition to an external size which is
what the user sees. While having an internal implementation-defined size is a
useful trick which could potentially also benefit other use cases in the future,
having two separate notions of size is confusing and prone to bugs. So until we
have other concrete use cases that would clearly benefit from having an internal
size in addition to an external size, we choose to avoid taking that approach.

### Faster locking API with atomics

This locking optimization provides an alternate low-latency option to lock and
unlock discardable VMOs, and is meant to be used by clients that expect to lock
and unlock fairly frequently. It is purely a performance optimization, and as
such can be a feature we add in the future if required.

The API uses a locking primitive called a Metex, which is similar to a Zircon
futex, in that it allows fast locking via userspace atomics, thereby saving on
the cost of a syscall.

A discardable VMO can be associated with a metex, which will be used to lock and
unlock it, instead of the `zx_vmo_op_range()` syscall.  A metex can have three
states: locked (in use by the userspace client), discardable (eligible for
reclamation by the kernel), and "needs syscall" (might have been reclaimed by
the kernel, a syscall is required to check the state).  Locking and unlocking
the VMO can be performed without entering the kernel by atomically flipping the
state of the metex between locked and discardable.  When the kernel discards the
VMO, it will atomically flip its state to "needs syscall", indicating that the
client needs to synchronize with the kernel to check on the discarded state.
More details of this proposal are out of the scope of this RFC, and will be
provided in a separate one.

### Pager based creation API

Any VMO that is backed by a pager is essentially a discardable VMO, because the
pager provides a mechanism to repopulate discarded pages on demand. The type of
discardable memory being proposed in this RFC is anonymous discardable
memory; the other type is file-backed discardable memory, an example of which is
the in-memory representation of blobs populated by the blobfs user pager.
Keeping this in mind, we can consider an alternate creation API where
discardable VMOs are associated with a pager. The VMO creation call might look
something like this:

```
zx_pager_create(0, &pager_handle);

zx_pager_create_vmo(pager_handle, 0, pager_port_handle, vmo_key, vmo_size,
                    &vmo_handle);
```

Locking and unlocking would work as proposed earlier with `zx_vmo_op_range()`.
The kernel would be free to discard pages from a VMO only when unlocked.

The advantage here is that it provides us with a unified creation API applicable
to all kinds of discardable memory - irrespective of whether it is file-backed
or anonymous.

However, the pager in this case does not really serve a special purpose. Since
it deals with generic anonymous memory, it is likely only going to provide zero
pages on demand. A pager is more suited for cases where pages need to be
populated in a specialized manner with certain specific content.  Introducing an
additional layer of indirection, both in terms of technical complexity and
performance overhead, just for the purpose of creating zero pages on demand
seems unnecessary; this functionality already exists in the kernel for regular
(non pager-backed) VMOs.

### Locking with a retainer object

The locking API proposed here leaves room for bugs where a discardable VMO can
be unintentionally (or maliciously) unlocked. We could have situations where a
process thinks that a VMO is locked, but another process has unlocked it, i.e.
the second process issues an extra unlock. This would cause the first process to
error out or crash when it accesses the VMO, even though it did correctly lock
it before access.

Instead of lock and unlock operations, we could implement locking with a
retainer object, which would lock the VMO when created and unlock it when
destroyed.

```
zx_vmo_create_retainer(vmo_handle, &retainer_handle);
```

The VMO would remain locked as long as the retainer handle is open. In the
example above, each of the two processes would use their own retainers to lock
the VMO, removing the possibility of an erroneous extra unlock. This locking
model reduces the likelihood of such bugs, and makes them easy to diagnose when
they occur.

The downside here is that the kernel will need to store more metadata to track
the locked state of a VMO. We now have a list of retainer objects associated
with a discardable VMO, instead of a single `lock_count` field. We might also
want to cap the length of this list if we want to eliminate the possibility of a
malicious user causing unbounded growth in the kernel.

### Priorities for reclamation order

To keep things simple to begin with, the kernel will reclaim unlocked
discardable VMOs in LRU order.  We could explore having clients explicitly
specify a priority order of reclamation in the future if required (VMOs in each
priority band could still be reclaimed in LRU order). The proposed API leaves
the door open to support this in the future, via the currently unused `buffer`
parameter in `zx_vmo_op_range()` for `ZX_VMO_OP_UNLOCK`.

This level of control is something we might not require though; a global LRU
order might be sufficient. If clients did want to exercise more control over
when certain buffers are reclaimed, they could instead opt into memory pressure
signals, and drop those buffers themselves.

### Interaction with other reclamation strategies

Currently there are two other mechanisms by which we can reclaim memory:

- Page eviction of user pager backed memory (in-memory blobs), which is done by
  the kernel at the CRITICAL  memory pressure level (and near OOM).
- Memory pressure signals, where userspace components themselves free memory at
  CRITICAL and WARNING memory pressure levels.

We will need to figure out where discardable memory sits in this scheme,
ensuring that no single reclamation strategy takes the majority of the burden.
For example, we might want to maintain some kind of eviction ratio of
file-backed memory to discardable memory.

### Locking pager-backed VMOs

We could extend the `ZX_VMO_OP_LOCK` and `ZX_VMO_OP_UNLOCK` operations to
pager-backed VMOs in the future. There has been a desire to support locking of
user pager backed VMOs in the past, which we might want to provide if a concrete
use case arises. For example, blobfs could lock VMOs in memory for blobs that it
deems important, or that do not fit the kernel LRU eviction scheme too well,
thereby avoiding the performance cost of re-paging them.

Locking pager-backed VMOs would tie in nicely with the discardable memory API,
since user pager backed VMOs can essentially be viewed as a type of discardable
memory, where the user pager provides a specialized mechanism to repopulate
pages. Locking and unlocking would then apply to both types of discardable
memory, the major difference between the two types being the way they are
created and populated.

### Deciding when to repopulate discarded VMOs

Clients might need a way to figure out when it is safe to repopulate a discarded
VMO. If the VMO is repopulated under memory pressure, the additional pages
committed might worsen the memory pressure on the system, pushing it closer to
OOM. Also, once the VMO is subsequently unlocked, there is a chance it might get
discarded if the memory pressure persists. This can lead to thrashing, where the
client repeatedly repopulates the VMO, only to see the kernel discard it soon
after.

Currently the only mechanism to observe system memory pressure levels is by
subscribing to the `fuchsia.memorypressure` service, which can be pretty
expensive for this use case. We could consider extending this service to provide
a way to perform one-off queries. We could also consider passing back an
indicator of the pressure level through the `zx_vmo_lock_state` struct - either
the current memory pressure level itself, or a boolean that coarsely captures
whether the system is under memory pressure.

### Debug aid to track unlocked VMO accesses

It might be useful to enable additional checks behind a build flag, that fail
syscalls on unlocked discardable VMOs. This would help developers easily find
bugs where a VMO access is not preceded by a lock, without having to rely on the
VMO being discarded under memory pressure, and only then resulting in failures.
Such checks on the locked state of a VMO can quickly become expensive as we add
range support in the future, so they are not feasible to enable in production,
but they might prove useful as a debug tool.

Catching unlocked VMO accesses through mappings might be more tricky to
implement. A couple of approaches that we could explore to accomplish this:

 - Unmap a mapped discardable VMO when it is unlocked. With this approach, we
   would need to make sure that existing VMO / VMAR semantics remain unchanged.
 - Teach wrappers around lock / unlock calls to tell ASAN that an unlocked VMO's
   mapping should be considered poisoned until it is locked again, using the
   [`ASAN_POISON_MEMORY_REGION`](https://github.com/llvm-mirror/compiler-rt/blob/master/include/sanitizer/asan_interface.h)
   interface.

## Prior art and references

- [`ashmem`](https://android.googlesource.com/platform/system/core/+/dd7bc3319deb2b77c5d07a51b7d6cd7e11b5beb0/include/cutils/ashmem.h) on Android
- [`ReclaimVirtualMemory`](https://docs.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-reclaimvirtualmemory) on Windows
- [`NSCache`](https://developer.apple.com/documentation/foundation/nscache) on macOS
