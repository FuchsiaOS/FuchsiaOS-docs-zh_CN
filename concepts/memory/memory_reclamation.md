# Memory reclamation in Fuchsia

Most operating systems employ memory reclamation strategies to ensure that the
working set of running processes at any point in time can efficiently utilize
all available physical memory. The operating system has a fixed amount of
physical memory (RAM) to distribute amongst all running processes, and it might
not be possible to accommodate them all at the same time.

In its simplest form, memory reclamation is page replacement, where pages that
are not as important for the current user activity are replaced with pages that
might be more important. Most operating systems maintain a pool of free pages,
so that incoming memory allocations  are quickly fulfilled, rather than blocked
while waiting for a page in use to be freed.

Fuchsia also employs a similar strategy where the system tries to keep the
amount of free memory larger than a certain threshold. Fuchsia uses several
memory reclamation techniques, both within the kernel and in userspace. This
guide describes how these memory reclamation techniques work. Fuchsia also
provides a set of tools to analyze and dump memory usage (see
[Memory usage analysis tools](/development/kernel/memory/memory.md#userspace_memory)).

## Pager-backed memory eviction

Userspace filesystems use
[pagers](/reference/kernel_objects/pager.md)
to [page](https://en.wikipedia.org/wiki/Memory_paging) in files on demand from
an external source, like a disk. Filesystems represent files in memory using
VMOs, whose pages are populated by the pager service as and when they are
accessed.

On Fuchsia,
[blobfs](/concepts/filesystems/blobfs.md) is
an immutable filesystem that hosts all executable files using a pager to
populate pages on demand. When the system comes under memory pressure, i.e. the
amount of available free memory starts running low, the kernel evicts pages
backed by blobfs in order to reclaim memory. Since these pages exist on disk,
they can be fetched back in when required.

The kernel tracks all pager-backed memory. When free memory is low, it finds
suitable candidates to evict. Pages are tracked in several LRU (least recently
used) page queues, which a kernel background thread rotates periodically, in
order to "age" pages. Another background thread evicts pages from the oldest
page queue under memory pressure.

As free memory dips lower, the kernel adjusts its aging and eviction policies
to be more aggressive, aging pages quicker in order to find more evictable
candidates. In order to prevent thrashing, pages in the MRU (most recently used)
page queue are never evicted. The length of this queue varies based on the
amount of churn on the system.

If the system is relatively quiet and the memory usage is stable, the kernel
ages pages slower and more pages accumulate in the MRU queue. On the other hand,
if the user is cycling through several activities, constantly switching the
working set, the kernel tries to keep up by aging pages more aggressively.

Userspace processes can also use
[eviction hints](/contribute/governance/rfcs/0068_eviction_hints.md)
to influence the kernel eviction strategy. Processes can use the `DONT_NEED`
hint to indicate pages are no longer in use and would be good candidates for
eviction. They can also use `ALWAYS_NEED` to indicate pages are important and
should not be considered for eviction, thereby avoiding the cost of fetching
them back in when they're accessed again.

Learn more about eviction hints in the reference docs:
[`zx_vmo_op_range`](/reference/syscalls/vmo_op_range.md)
and
[`zx_vmar_op_range`](/reference/syscalls/vmar_op_range.md).

## Zero page deduplication

Pages in anonymous VMOs (non-pager-backed) get populated / committed only on a
write. Reads are fulfilled by the kernel using a singleton zero page. Even after
pages have been committed on a write, the kernel tries to deduplicate pages that
are filled only with zeros back to the singleton zero page in order to save
memory. The kernel periodically scans physical pages in anonymous VMOs, looking
for opportunities to deduplicate zero pages.

## Page table reclamation

As explained in [Address spaces](/concepts/memory/address_spaces.md),
the VMAR hierarchy helps the kernel track virtual to physical memory mappings.
When a virtual address is accessed for the first time,
the address space's VMAR tree is used to look up the underlying physical page.
The virtual-to-physical mapping is then stored in the hardware page tables,
which the MMU uses for future lookups.
Under memory pressure,
the kernel reclaims memory in hardware page tables that hasn't been accessed for a while.
When those mappings are needed again,
they can be reconstructed from the VMAR tree.

## Discardable VMOs

Userspace processes can create a special flavor of
[VMOs that are discardable](/contribute/governance/rfcs/0012_zircon_discardable_memory.md).
Clients can
[lock and unlock ](/reference/syscalls/vmo_op_range.md)discardable
VMOs depending on whether or not they are being used. When the system is under
memory pressure, the kernel finds discardable VMOs that are unlocked and frees
them.

Sample code (modulo error handling):

```cpp
// Create a discardable VMO.
zx_handle_t vmo_handle;
zx_vmo_create(vmo_size, ZX_VMO_DISCARDABLE, &vmo);

// Lock the VMO.
zx_vmo_lock_state_t lock_state = {};
zx_vmo_op_range(vmo, ZX_VMO_OP_LOCK, 0, vmo_size, &lock_state,
                sizeof(lock_state));

// Use the VMO as desired.
zx_vmo_read(vmo, buf, 0, sizeof(buf));

// Unlock the VMO. The kernel is free to discard it now.
vmo_op_range(vmo, ZX_VMO_OP_UNLOCK, 0, vmo_size, nullptr, 0);

// Lock the VMO again before use.
zx_vmo_op_range(vmo, ZX_VMO_OP_LOCK, 0, vmo_size, &lock_state,
                sizeof(lock_state));

if (lock_state.discarded_size > 0) {
  // The kernel discarded the VMO. Re-initialize it if required.
  zx_vmo_write(vmo, data, 0, sizeof(data));
} else {
  // The kernel did not discard the VMO. Previous contents were preserved.
}
```

## Memory pressure signals

Fuchsia provides userspace processes the ability to directly control their
memory consumption in response to system-wide available memory. Clients can
register to receive
[memory pressure
signals](https://fuchsia.dev/reference/fidl/fuchsia.memorypressure.md)
and take actions depending on the observed memory pressure level. There are
[three memory pressure levels](https://cs.opensource.google/fuchsia/fuchsia/+/main:sdk/fidl/fuchsia.memorypressure/memorypressure.fidl;l=8):

<table>
    <tr><th>Name</th><th>Value</th><th>Description</th></tr>
        <tr id="Level.NORMAL">
<td><h3 id="Level.NORMAL" class="add-link hide-from-toc">NORMAL</h3></td>
            <td><code>1</code></td>
            <td><p>The memory pressure level is healthy.</p>
<p>Registered clients are free to hold on to caches and allocate memory
unrestricted.</p>
<p>However, clients should take care to not proactively re-create caches on a
transition back to the NORMAL level, causing a memory spike that immediately
pushes the level over to WARNING again.</p>
</td>
        </tr>
        <tr id="Level.WARNING">
<td><h3 id="Level.WARNING" class="add-link hide-from-toc">WARNING</h3></td>
            <td><code>2</code></td>
            <td><p>The memory pressure level is somewhat constrained, and might cross over to
the critical pressure range if left unchecked.</p>
<p>Registered clients are expected to optimize their operation to limit memory
usage, rather than for best performance, for example, by reducing cache sizes
and non-essential memory allocations.</p>
<p>Clients must take care to regulate the amount of work they undertake in
order to reclaim memory, and ensure that it does not cause visible
performance degradation. There exists some memory pressure, but not enough
to justify trading off user responsiveness to reclaim memory.</p>
</td>
        </tr>
        <tr id="Level.CRITICAL">
<td><h3 id="Level.CRITICAL" class="add-link hide-from-toc">CRITICAL</h3></td>
            <td><code>3</code></td>
            <td><p>The memory pressure level is very constrained.</p>
<p>Registered clients are expected to drop all non-essential memory, and refrain
from allocating more memory. Failing to do so might result in the job
getting terminated, or the system being rebooted in the case of global
memory pressure.</p>
<p>Clients may undertake expensive work to reclaim memory if required, since
failing to do so might result in termination. The client might decide that a
performance hit is a fair tradeoff in this case.</p>
</td>
        </tr>
</table>

### Comparing memory pressure signals to discardable VMOs

Userspace clients can pick between memory pressure signals and discardable
VMOs, or use a combination of both reclamation mechanisms based on their
needs.These are some things to consider when making the choice:

-   Memory pressure signals allow clients to do more than just trim caches.
    For example, jobs can tear down non-essential processes in their job tree.
    They can also stop certain memory-intensive activities, or hold off on
    starting new ones until the pressure level is Normal.
-   With discardable VMOs, the userspace client gives up control over when
    the memory is freed to the kernel. The kernel decides when to free the
    memory based on various factors: the amount of available memory, memory
    that can be reclaimed by other means, etc. If the client wishes to finely
    control the lifetimes of its caches, when to trim what, etc., memory
    pressure signals might be more suitable.
-   It is possible that discardable VMOs end up preserving their contents
    for longer than if the process was tearing down the VMOs itself in response
    to memory pressure signals. The kernel drives freeing of discardable VMOs,
    and the kernel has more global context around the amount of free memory, so
    it knows exactly how much to reclaim. The kernel also has other means of
    reclaiming memory at its disposal, so it's possible that not all
    discardable VMOs need to be freed up. On the other hand, if the userspace
    client is responding to memory pressure itself, it will likely react in the
    same manner every time, trimming all its caches.
-   Discardable memory can also allow the kernel to reclaim memory more
    quickly so that the system recovers faster. With memory pressure signals,
    there can be some IPC and scheduling latency involved, between the kernel
    signaling the pressure level change, and the userspace process responding to it.

## OOM (Out-of-memory) reboot

It is possible for all memory reclamation strategies to fail to free up enough
memory in the face of certain aggressive memory allocation patterns. When that
happens, the kernel opts to reboot after cleanly shutting down filesystems to
prevent data loss. When the free memory level falls below a preconfigured OOM
threshold, an OOM reboot is triggered.

## Tools to test memory pressure response

### Observing and testing kernel memory reclamation

Use the `k scanner` command to observe and test reclamation techniques the
kernel uses: pager-backed eviction, discardable VMO reclamation, zero page
deduplication, and page table reclamation. It can also be used to test the page
queue rotation / aging strategy used to drive eviction. Run `k scanner` on a
serial console to see all available options:

```posix-terminal
k scanner
usage:
scanner dump                    : dump scanner info
scanner push_disable            : increase scanner disable count
scanner pop_disable             : decrease scanner disable count
scanner reclaim_all             : attempt to reclaim all possible memory
scanner rotate_queue            : immediately rotate the page queues
scanner reclaim <MB> [only_old] : attempt to reclaim requested MB of memory.
scanner pt_reclaim [on|off]     : turn unused page table reclamation on or off
scanner harvest_accessed        : harvest all page accessed information
```

`k scanner dump` dumps the current state of the page queues and other relevant
memory counters the kernel uses for reclamation:

```posix-terminal
k scanner dump
[SCAN]: Scanner enabled. Triggering informational scan
[SCAN]: Found 4303 zero pages across all of memory
[SCAN]: Found 8995 user-pager backed pages in queue 0
[SCAN]: Found 3278 user-pager backed pages in queue 1
[SCAN]: Found 8947 user-pager backed pages in queue 2
[SCAN]: Found 10776 user-pager backed pages in queue 3
[SCAN]: Found 3981 user-pager backed pages in queue 4
[SCAN]: Found 0 user-pager backed pages in queue 5
[SCAN]: Found 0 user-pager backed pages in queue 6
[SCAN]: Found 0 user-pager backed pages in queue 7
[SCAN]: Found 1347 user-pager backed pages in DontNeed queue
[SCAN]: Found 40 zero forked pages
[SCAN]: Found 0 locked pages in discardable vmos
[SCAN]: Found 0 unlocked pages in discardable vmos
pq: MRU generation is 12 set 10.720698018s ago due to "Active ratio", LRU generation is 6
pq: Pager buckets [8995],[3278],8947,10776,3981,0,{0},0, evict first: 1347, live active/inactive totals: 12273/25051
```

Test reclaiming memory with `k scanner reclaim` or `k scanner reclaim_all`:

```posix-terminal
k scanner reclaim_all
[EVICT]: Free memory before eviction was 7161MB and after eviction is 7290MB
[EVICT]: Evicted 33004 user pager backed pages
[SCAN]: De-duped 25 pages that were recently forked from the zero page
```

Test page table reclamation with `k pmm drop_user_pt`:

```posix-terminal
k pmm
…
pmm drop_user_pt                             : drop all user hardware page tables
```

### Observing and generating memory pressure

Use the `k pmm mem_avail_state` command to generate memory pressure on the
system, by allocating memory to reach the specified memory pressure level. This
is useful for testing system-wide response to memory pressure:

```posix-terminal
k pmm mem_avail_state
pmm mem_avail_state info                     : dump memory availability state info
pmm mem_avail_state [step] <state> [<nsecs>] : allocate memory to go to memstate <state>, hold the state for <nsecs> (10s by default). Only works if going to <state> from current state requires allocating memory, can't free up pre-allocated memory. In optional [step] mode, allocation pauses for 1 second at each intermediate memory availability state until <state> is reached.
```

`k pmm mem_avail_state info` dumps the current memory pressure state.

```posix-terminal
k pmm mem_avail_state info
watermarks: [50M, 60M, 150M, 300M]
debounce: 1M
current state: 4
current bounds: [299M, 16.0E]
free memory: 7253.5M
```

The memory availability states are numbered starting from 0, and are a superset
of the levels mentioned previously for [memory pressure
signals](#memory_pressure_signals).

-   `OOM` is state 0. This is the free memory level below which the kernel
    decides to reboot the system.
-   `Imminent-OOM` is state 1. This is a diagnostic-only memory level, set
    at a small delta from the OOM level. Its sole purpose is to provide a means
    to collect OOM diagnostic information safely, as it might be too late to
    gather diagnostics at the OOM level. Learn more about this level in
    [RFC-0091](/contribute/governance/rfcs/0091_getevent_imminent_oom.md).
-   `Critical` is state 2. This is the level that triggers the CRITICAL
    memory pressure signal.
-   `Warning` is state 3. This is the level that triggers the WARNING memory
    pressure signal.
-   `Normal` is state 4. This is the level that triggers the NORMAL memory
    pressure signal.

In the example above, the `current state` is 4, i.e. Normal.

The `watermarks` show the memory thresholds that delineate the different memory
availability states. The output in the above example shows these memory
thresholds:

```none {:.devsite-disable-click-to-copy}
OOM: 50MB, Imminent-OOM: 60MB, Critical: 150MB, Warning: 300MB
```

The `debounce` is the slack or error margin used when computing memory state
boundaries. In this example, it is 1MB.

The `current bounds` shows the free memory bounds applicable to the current
memory state. Given the current state is `Normal`, referring to the
`watermarks`, `Normal` starts at the 300MB threshold. Using the 1MB debounce,
the lower limit is 299MB. There isn't an applicable upper limit for the `Normal`
level, which is set to `UINT64_MAX` here.

Lastly, the total `free memory` on the system is currently 7253.5MB.

Use the command `k pmm mem_avail_state X` to transition to memory availability
state `X`, where `X` is the numerical memory state as described above.
Optionally provide a duration for which the requested state is to be held. There
is also an option to "step" through intermediate states, pausing at each of
them.

For example,  this triggers a transition to the `Critical` memory state:

```posix-terminal
k pmm mem_avail_state 2
memory-pressure: memory availability state - Critical
pq: MRU generation is 714 set 4.144414945s ago due to "Active ratio", LRU generation is 708
pq: Pager buckets [3482],[115],317,0,199,0,{6939},0, evict first: 0, live active/inactive totals: 3597/7455
memory-pressure: set target memory to evict 1MB (free memory is 149MB)
Leaked 1817528 pages
Sleeping for 10 seconds...
[EVICT]: Free memory before eviction was 147MB and after eviction is 151MB
[EVICT]: Evicted 986 user pager backed pages
Freed 1817528 pages
memory-pressure: memory availability state - Normal
pq: MRU generation is 717 set 1.213355379s ago due to "Timeout", LRU generation is 711
pq: Pager buckets [4351],[258],149,37,0,1,{5798},0, evict first: 0, live active/inactive totals: 4609/5985
```

Here the system transitioned to `Critical` by allocating 1817528 pages (the
page size is 4KB). Then there was a sleep for 10 seconds (default for holding
the state) during which the `Critical` pressure persisted. Finally, the 1817528
allocated pages were freed up, and the memory pressure dropped back to `Normal`.
The `Critical` state transition caused some pager-backed memory to be evicted as
well, as can be seen by the `[EVICT]` lines.

The `k pmm mem_avail_state` command is a useful tool to test memory pressure
response of the system as a whole. Since it works by allocating actual physical
memory, it exercises all the reclamation mechanisms the system has at its
disposal, both within the kernel and in userspace.

These are additional `k pmm oom` commands used to test system response
specifically at the OOM level.

```none {:.devsite-disable-click-to-copy}l
pmm oom [<rate>]                             : leak memory until oom is triggered, optionally specify the rate at which to leak (in MB per second)
pmm oom hard                                 : leak memory aggressively and keep on leaking
pmm oom signal                               : trigger oom signal without leaking memory
```

Sample output with `k pmm oom`:

```posix-terminal
k pmm oom
Disabling VM scanner
memory-pressure: free memory is 49MB, evicting pages to prevent OOM...
pq: MRU generation is 13 set 7.979442243s ago due to "Active ratio", LRU generation is 7
pq: Pager buckets [4538],[4517],3624,4606,13716,4976,{0},0, evict first: 1347, live active/inactive totals: 9055/28269
memory-pressure: found no pages to evict
memory-pressure: free memory after OOM eviction is 49MB
…
memory-pressure: pausing for 8s after OOM mem signal
[00028.317] 02811:03481> [fshost] INFO: [admin-server.cc(33)] received shutdown command over admin interface
[00028.317] 02811:03481> [fshost] INFO: [fs-manager.cc(281)] filesystem shutdown initiated
[00028.317] 02811:38032> [fshost] INFO: [fs-manager.cc(310)] Shutting down /data
[00028.318] 12900:12902> [minfs] INFO: [minfs.cc(1471)] Shutting down
[00028.340] 12900:12902> [minfs] WARNING: [src/storage/bin/minfs/main.cc(53)] Unmounted
[00028.341] 02811:03481> [fshost] INFO: [admin-server.cc(39)] shutdown complete
[00028.342] 02811:02813> [fshost] INFO: [main.cc(309)] terminating
[00028.342] 02687:02689> [driver_manager.cm] INFO: [suspend_handler.cc(205)] Successfully waited for VFS exit completion

memory-pressure: rebooting due to OOM
memory-pressure: stowing crashlog
ZIRCON REBOOT REASON (OOM)
Shutting down debuglog
platform_halt suggested_action 1 reason 3
Rebooting...
```

### Simulating memory pressure signals in userspace

Use the `ffx profile memory signal` command to simulate memory pressure signals in
userspace without creating actual memory pressure. This is useful when the goal is
to test the response of a particular userspace process to memory pressure
signals without altering the memory state of the system.

```posix-terminal
Signals userspace clients with specified memory pressure level. Clients can use this
command to test their response to memory pressure. Does not affect the real memory
pressure level on the system, or trigger any kernel reclamation tasks.

Positional Arguments:
  level             memory pressure level. Can be CRITICAL, WARNING or NORMAL.
```

For example, with `ffx profile memory signal WARNING`, the following shows in the `ffx
log` output:

```none {:.devsite-disable-click-to-copy}
[00213.059579][26701][26703][memory_monitor] INFO: [pressure_notifier.cc:106] Simulating memory pressure level WARNING
```

Note that this command does not actually allocate any memory. It simply
simulates a one-time memory pressure signal for the requested level in
userspace, without affecting the kernel's memory availability state. As such, it
will not trigger any kernel memory reclamation, like eviction of pager-backed
memory.
