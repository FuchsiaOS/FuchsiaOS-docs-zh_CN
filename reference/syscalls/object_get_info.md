# zx_object_get_info

## NAME

<!-- Updated by update-docs-from-fidl, do not edit. -->

Query information about an object.

## SYNOPSIS

<!-- Updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_object_get_info(zx_handle_t handle,
                               uint32_t topic,
                               void* buffer,
                               size_t buffer_size,
                               size_t* actual,
                               size_t* avail);
```

## DESCRIPTION

`zx_object_get_info()` requests information about the provided handle (or the
object the handle refers to). The *topic* parameter indicates what specific
information is desired.

*buffer* is a pointer to a buffer of size *buffer_size* to return the
information.

*actual* is an optional pointer to return the number of records that were
written to buffer.

*avail* is an optional pointer to return the number of records that are
available to read.

If the buffer is insufficiently large, *avail* will be larger than *actual*.

[TOC]

## TOPICS

### ZX_INFO_HANDLE_VALID

*handle* type: **Any**

*buffer* type: **n/a**

Returns **ZX_OK** if *handle* is valid, or **ZX_ERR_BAD_HANDLE** otherwise. No
records are returned and *buffer* may be NULL.

### ZX_INFO_HANDLE_BASIC

*handle* type: **Any**

*buffer* type: `zx_info_handle_basic_t[1]`

```
typedef struct zx_info_handle_basic {
    // The unique id assigned by kernel to the object referenced by the
    // handle.
    zx_koid_t koid;

    // The immutable rights assigned to the handle. Two handles that
    // have the same koid and the same rights are equivalent and
    // interchangeable.
    zx_rights_t rights;

    // The object type: channel, event, socket, etc.
    uint32_t type;                // zx_obj_type_t;

    // If the object referenced by the handle is related to another (such
    // as the other end of a channel, or the parent of a job) then
    // |related_koid| is the koid of that object, otherwise it is zero.
    // This relationship is immutable: an object's |related_koid| does
    // not change even if the related object no longer exists.
    zx_koid_t related_koid;
} zx_info_handle_basic_t;
```

### ZX_INFO_HANDLE_COUNT

*handle* type: **Any**

*buffer* type: `zx_info_handle_count_t[1]`

```
typedef struct zx_info_handle_count {
    // The number of outstanding handles to a kernel object.
    uint32_t handle_count;
} zx_info_handle_count_t;
```

The *handle_count* should only be used as a debugging aid. Do not use it to
check that an untrusted processes cannot modify a kernel object. Due to
asynchronous nature of the system scheduler, there might be a time window during
which it is possible for an object to be modified by a previous handle owner
even as the last handle is transferred from one process to another.

### ZX_INFO_PROCESS_HANDLE_STATS

*handle* type: **Process**

*buffer* type: `zx_info_process_handle_stats_t[1]`

```
typedef struct zx_info_process_handle_stats {
    // The number of outstanding handles to kernel objects of each type.
    uint32_t handle_count[ZX_OBJ_TYPE_UPPER_BOUND];
} zx_info_process_handle_stats_t;
```

### ZX_INFO_HANDLE_TABLE

*handle* type: **Process**

*buffer* type: `zx_info_handle_extended_t[n]`

Returns an array of `zx_info_handle_extended_t` one for each handle in the
Process at the moment of the call. The kernel ensures that the handles returned
are consistent.

```
typedef struct zx_info_handle_extended {
    // The object type: channel, event, socket, etc.
    zx_obj_type_t type;

    // The handle value, which is only valid for the process that
    // was passed to ZX_INFO_HANDLE_TABLE.
    zx_handle_t handle_value;

    // The immutable rights assigned to the handle. Two handles that
    // have the same koid and the same rights are equivalent and
    // interchangeable.
    zx_rights_t rights;

    uint32_t reserved;

    // The unique id assigned by kernel to the object referenced by the
    // handle.
    zx_koid_t koid;

    // If the object referenced by the handle is related to another (such
    // as the other end of a channel, or the parent of a job) then
    // |related_koid| is the koid of that object, otherwise it is zero.
    // This relationship is immutable: an object's |related_koid| does
    // not change even if the related object no longer exists.
    zx_koid_t related_koid;

    // If the object referenced by the handle has a peer, like the
    // other end of a channel, then this is the koid of the process
    // which currently owns it.
    zx_koid_t peer_owner_koid;
} zx_info_handle_extended_t;
```

Note that a process might have live references to objects for which the process
does not have a handle to. For example, running threads for which all handles
have been closed.

### ZX_INFO_JOB

*handle* type: **Job**

*buffer* type: `zx_info_job_t[1]`

```
typedef struct zx_info_job {
    // The job's return code; only valid if |exited| is true.
    // If the job was killed, it will be one of the ZX_TASK_RETCODE values.
    int64_t return_code;

    // If true, the job has exited and |return_code| is valid.
    bool exited;

    // True if the ZX_PROP_JOB_KILL_ON_OOM property was set.
    bool kill_on_oom;

    // True if a debugger is attached to the job.
    bool debugger_attached;
} zx_info_job_t;
```

Note that |exited| will immediately report that the job has exited following a
|zx_task_kill| or equivalent (e.g. an OOM kill), but child jobs and processes
may still be in the process of exiting.

### ZX_INFO_PROCESS (a.k.a. ZX_INFO_PROCESS_V1)

*handle* type: **Process**

*buffer* type: `zx_info_process_t[1]`

TODO(fxbug.dev/30751): Deprecated in favor of ZX_INFO_PROCESS_V2.

```
typedef struct zx_info_process {
    // The process's return code; only valid if |exited| is true.
    // Guaranteed to be non-zero if the process was killed by |zx_task_kill|.
    int64_t return_code;

    // True if the process has ever left the initial creation state,
    // even if it has exited as well.
    bool started;

    // If true, the process has exited and |return_code| is valid.
    bool exited;

    // True if a debugger is attached to the process.
    bool debugger_attached;
} zx_info_process_t;
```

Note that |exited| will immediately report that the process has exited following
a |zx_task_kill|, but child threads may still be in the process of exiting.

### ZX_INFO_PROCESS_V2

*handle* type: **Process**

*buffer* type: `zx_info_process_v2_t[1]`

TODO(fxbug.dev/30751): This will replace `ZX_INFO_PROCESS_V1` and will be
renamed to `ZX_INFO_PROCESS` later in the transition.

```
typedef struct zx_info_process_v2 {
    // The process's return code; only valid if the
    // |ZX_PROCESS_INFO_FLAG_EXITED| flag is set. If the process was killed, it
    // will be one of the |ZX_TASK_RETCODE| values.
    int64_t return_code;

    // The monotonic time at which `zx_process_start()` was called, only valid
    // if the |ZX_INFO_PROCESS_FLAG_STARTED| flag is set.
    zx_time_t start_time;

    // Bitwise OR of ZX_INFO_PROCESS_FLAG_* values.
    uint32_t flags;
} zx_info_process_v2_t;
```

Note that |flags| will immediately report that the process has exited (i.e. it
will contain ZX_INFO_PROCESS_FLAG_EXITED) following a |zx_task_kill|, but child
threads may still be in the process of exiting.

### ZX_INFO_PROCESS_THREADS

*handle* type: **Process**

*buffer* type: `zx_koid_t[n]`

Returns an array of `zx_koid_t`, one for each running thread in the Process at
that moment in time.

N.B. Getting the list of threads is inherently racy. This can be somewhat
mitigated by first suspending all the threads, but note that an external thread
can create new threads. *actual* will contain the number of threads returned in
*buffer*. *avail* will contain the total number of threads of the process at the
time the list of threads was obtained, it could be larger than *actual*.

### ZX_INFO_THREAD

*handle* type: **Thread**

*buffer* type: `zx_info_thread_t[1]`

```
typedef struct zx_info_thread {
    // One of ZX_THREAD_STATE_* values.
    uint32_t state;

    // If |state| is ZX_THREAD_STATE_BLOCKED_EXCEPTION, the thread has gotten
    // an exception and is waiting for the exception to be handled by the
    // specified channel.
    // The value is one of ZX_EXCEPTION_CHANNEL_TYPE_*.
    uint32_t wait_exception_channel_type;

    // CPUs this thread may be scheduled on, as specified by
    // a profile object applied to this thread.
    //
    // The kernel may not internally store invalid CPUs in the mask, so
    // this may not exactly match the mask applied to the thread for
    // CPUs beyond what the system is able to use.
    zx_cpu_set_t cpu_affinity_mask;
} zx_info_thread_t;
```

The values in this struct are mainly for informational and debugging purposes at
the moment.

The various **ZX_THREAD_STATE_** values are defined by

```
#include <zircon/syscalls/object.h>
```

*   **ZX_THREAD_STATE_NEW**: The thread has been created but it has not started
    running yet.
*   **ZX_THREAD_STATE_RUNNING**: The thread is running user code normally.
*   **ZX_THREAD_STATE_SUSPENDED**: Stopped due to [`zx_task_suspend()`].
*   **ZX_THREAD_STATE_BLOCKED**: In a syscall or handling an exception. This
    value is never returned by itself. See **ZX_THREAD_STATE_BLOCKED_\*** below.
*   **ZX_THREAD_STATE_DYING**: The thread is in the process of being terminated,
    but it has not been stopped yet.
*   **ZX_THREAD_STATE_DEAD**: The thread has stopped running.

When a thread is stopped inside a blocking syscall, or stopped in an exception,
the value returned in **state** is one of the following:

*   **ZX_THREAD_STATE_BLOCKED_EXCEPTION**: The thread is stopped in an
    exception.
*   **ZX_THREAD_STATE_BLOCKED_SLEEPING**: The thread is stopped in
    [`zx_nanosleep()`].
*   **ZX_THREAD_STATE_BLOCKED_FUTEX**: The thread is stopped in
    [`zx_futex_wait()`].
*   **ZX_THREAD_STATE_BLOCKED_PORT**: The thread is stopped in
    [`zx_port_wait()`].
*   **ZX_THREAD_STATE_BLOCKED_CHANNEL**: The thread is stopped in
    [`zx_channel_call()`].
*   **ZX_THREAD_STATE_BLOCKED_WAIT_ONE**: The thread is stopped in
    [`zx_object_wait_one()`].
*   **ZX_THREAD_STATE_BLOCKED_WAIT_MANY**: The thread is stopped in
    [`zx_object_wait_many()`].
*   **ZX_THREAD_STATE_BLOCKED_INTERRUPT**: The thread is stopped in
    [`zx_interrupt_wait()`].

The various **ZX_EXCEPTION_CHANNEL_TYPE_** values are defined by

```
#include <zircon/syscalls/exception.h>
```

*   **ZX_EXCEPTION_CHANNEL_TYPE_NONE**
*   **ZX_EXCEPTION_CHANNEL_TYPE_DEBUGGER**
*   **ZX_EXCEPTION_CHANNEL_TYPE_THREAD**
*   **ZX_EXCEPTION_CHANNEL_TYPE_PROCESS**
*   **ZX_EXCEPTION_CHANNEL_TYPE_JOB**
*   **ZX_EXCEPTION_CHANNEL_TYPE_JOB_DEBUGGER**

### ZX_INFO_THREAD_EXCEPTION_REPORT

*handle* type: **Thread**

*buffer* type: `zx_exception_report_t[1]`

```
#include <zircon/syscalls/exception.h>
```

If the thread is currently in an exception and is waiting for an exception
response, then this returns the exception report as a single
`zx_exception_report_t`, with status **ZX_OK**.

Returns **ZX_ERR_BAD_STATE** if the thread is not in an exception and waiting
for an exception response.

### ZX_INFO_THREAD_STATS {#zx-info-thread-stats}

*handle* type: **Thread**

*buffer* type: `zx_info_thread_stats[1]`

```
typedef struct zx_info_thread_stats {
    // Total accumulated running time of the thread.
    //
    // Note: See zx_info_task_runtime for queue time in addition to runtime.
    zx_duration_t total_runtime;

    // CPU number that this thread was last scheduled on, or ZX_INFO_INVALID_CPU
    // if the thread has never been scheduled on a CPU. By the time this call
    // returns, the thread may have been scheduled elsewhere, so this
    // information should only be used as a hint or for statistics.
    uint32_t last_scheduled_cpu;
} zx_info_thread_stats_t;
```

Returns **ZX_ERR_BAD_STATE** if the thread has exited.

### ZX_INFO_CPU_STATS

Note: many values of this topic are being retired in favor of a different
mechanism.

*handle* type: **Resource** (Specifically, the root resource)

*buffer* type: `zx_info_cpu_stats_t[1]`

```
typedef struct zx_info_cpu_stats {
    uint32_t cpu_number;
    uint32_t flags;

    zx_duration_t idle_time;

    // kernel scheduler counters
    uint64_t reschedules;
    uint64_t context_switches;
    uint64_t irq_preempts;
    uint64_t preempts;
    uint64_t yields;

    // cpu level interrupts and exceptions
    uint64_t ints;          // hardware interrupts, minus timer interrupts
                            // inter-processor interrupts
    uint64_t timer_ints;    // timer interrupts
    uint64_t timers;        // timer callbacks
    uint64_t page_faults;   // (deprecated, returns 0)
    uint64_t exceptions;    // (deprecated, returns 0)
    uint64_t syscalls;

    // inter-processor interrupts
    uint64_t reschedule_ipis;
    uint64_t generic_ipis;
} zx_info_cpu_stats_t;
```

### ZX_INFO_VMAR

*handle* type: **VM Address Region**

*buffer* type: `zx_info_vmar_t[1]`

```
typedef struct zx_info_vmar {
    // Base address of the region.
    uintptr_t base;

    // Length of the region, in bytes.
    size_t len;
} zx_info_vmar_t;
```

This returns a single `zx_info_vmar_t` that describes the range of address space
that the VMAR occupies.

### ZX_INFO_VMO

*handle* type: **VM Object**

*buffer* type: `zx_info_vmo_t[1]`

```
typedef struct zx_info_vmo {
    // The koid of this VMO.
    zx_koid_t koid;

    // The name of this VMO.
    char name[ZX_MAX_NAME_LEN];

    // The size of this VMO.
    uint64_t size_bytes;

    // If this VMO is a child, the koid of its parent. Otherwise, zero.
    zx_koid_t parent_koid;

    // The number of children of this VMO, if any.
    size_t num_children;

    // The number of times this VMO is currently mapped into VMARs.
    size_t num_mappings;

    // An estimate of the number of unique address spaces that
    // this VMO is mapped into.
    size_t share_count;

    // Bitwise OR of ZX_INFO_VMO_* values.
    uint32_t flags;

    // If |ZX_INFO_VMO_TYPE(flags) == ZX_INFO_VMO_TYPE_PAGED|, the amount of
    // memory currently allocated to this VMO.
    uint64_t committed_bytes;

    // If |flags & ZX_INFO_VMO_VIA_HANDLE|, the handle rights.
    // Undefined otherwise.
    zx_rights_t handle_rights;

    // VMO mapping cache policy. One of ZX_CACHE_POLICY_*
    uint32_t cache_policy;

    // Amount of kernel memory, in bytes, allocated to track metadata
    // associated with this VMO.
    uint64_t metadata_bytes;

    // Running counter of the number of times the kernel, without user request,
    // performed actions on this VMO that would have caused |committed_bytes| to
    // report a different value.
    uint64_t committed_change_events;
} zx_info_vmo_t;
```

This returns a single `zx_info_vmo_t` that describes various attributes of the
VMO.

### ZX_INFO_SOCKET

*handle* type: **Socket**

*buffer* type: `zx_info_socket_t[1]`

```
typedef struct zx_info_socket {
    // The options passed to zx_socket_create().
    uint32_t options;

    // The maximum size of the receive buffer of a socket, in bytes.
    //
    // The receive buffer may become full at a capacity less than the maximum
    // due to overhead.
    size_t rx_buf_max;

    // The size of the receive buffer of a socket, in bytes.
    size_t rx_buf_size;

    // The amount of data, in bytes, that is available for reading in a single
    // zx_socket_read call.
    //
    // For stream sockets, this value will match |rx_buf_size|. For datagram
    // sockets, this value will be the size of the next datagram in the receive
    // buffer.
    size_t rx_buf_available;

    // The maximum size of the transmit buffer of a socket, in bytes.
    //
    // The transmit buffer may become full at a capacity less than the maximum
    // due to overhead.
    //
    // Will be zero if the peer endpoint is closed.
    size_t tx_buf_max;

    // The size of the transmit buffer of a socket, in bytes.
    //
    // Will be zero if the peer endpoint is closed.
    size_t tx_buf_size;
} zx_info_socket_t;
```

### ZX_INFO_TIMER

*handle* type: **Timer**

*buffer* type: `zx_info_timer_t[1]`

```
typedef struct zx_info_timer {
    // The options passed to zx_timer_create().
    uint32_t options;

    // The deadline with respect to ZX_CLOCK_MONOTONIC at which the timer will
    // fire next.
    //
    // This value will be zero if the timer is not set to fire.
    zx_time_t deadline;

    // Specifies a range from deadline - slack to deadline + slack during which
    // the timer is allowed to fire. The system uses this parameter as a hint to
    // coalesce nearby timers.
    //
    // The precise coalescing behavior is controlled by the options parameter
    // specified when the timer was created.
    //
    // This value will be zero if the timer is not set to fire.
    zx_duration_t slack;
} zx_info_timer_t;
```

### ZX_INFO_JOB_CHILDREN

*handle* type: **Job**

*buffer* type: `zx_koid_t[n]`

Returns an array of `zx_koid_t`, one for each direct child Job of the provided
Job handle.

### ZX_INFO_JOB_PROCESSES

*handle* type: **Job**

*buffer* type: `zx_koid_t[n]`

Returns an array of `zx_koid_t`, one for each direct child Process of the
provided Job handle.

### ZX_INFO_TASK_STATS

*handle* type: **Process**

*buffer* type: `zx_info_task_stats_t[1]`

Returns statistics about resources (e.g., memory) used by a task.

```
typedef struct zx_info_task_stats {
    // The total size of mapped memory ranges in the task.
    // Not all will be backed by physical memory.
    size_t mem_mapped_bytes;

    // For the fields below, a byte is considered committed if it's backed by
    // physical memory. Some of the memory may be double-mapped, and thus
    // double-counted.

    // Committed memory that is only mapped into this task.
    size_t mem_private_bytes;

    // Committed memory that is mapped into this and at least one other task.
    size_t mem_shared_bytes;

    // A number that estimates the fraction of mem_shared_bytes that this
    // task is responsible for keeping alive.
    //
    // An estimate of:
    //   For each shared, committed byte:
    //   mem_scaled_shared_bytes += 1 / (number of tasks mapping this byte)
    //
    // This number is strictly smaller than mem_shared_bytes.
    size_t mem_scaled_shared_bytes;
} zx_info_task_stats_t;
```

Additional errors:

*   **ZX_ERR_BAD_STATE**: If the target process has terminated

### ZX_INFO_TASK_RUNTIME

*handle* type: **Job**, **Process**, or **Thread**

*buffer* type: `zx_info_task_runtime_t[1]`

Returns statistics about the runtime of a task.

```
// Info on the runtime of a task.
typedef struct zx_info_task_runtime {
    // The total amount of time this task and its children were
    // running on a CPU (not blocked).
    // * Threads include only their own runtime.
    // * Processes include the runtime for all of their threads (including threads that previously
    // exited).
    // * Jobs include the runtime for all of their processes (including processes that previously
    // exited).
    zx_duration_t cpu_time;

    // The total amount of time this task and its children were queued
    // to run (ready) but not actually using a CPU.
    // * Threads include only their own queue time.
    // * Processes include the queue time for all of their threads (including threads that
    // previously exited).
    // * Jobs include the queue time for all of their processes (including processes that previously
    // exited).
    zx_duration_t queue_time;

    // The total amount of time this task and its children spent handling page faults.
    // * Threads include only their own page fault handling time.
    // * Processes include the page fault time for all of their threads (including threads that
    // previously exited).
    // * Jobs include the page fault time for all of their processes (including processes that
    // previously exited).
    zx_duration_t page_fault_time;

    // The total amount of time this task and its children spent waiting on contended kernel locks.
    // * Threads include only their own wait time.
    // * Processes include the wait time for all of their threads (including threads that
    // previously exited).
    // * Jobs include the wait time for all of their processes (including processes that
    // previously exited).
    zx_duration_t lock_contention_time;
} zx_info_task_runtime_t;
```

The run time of a task does not include the time spent suspended or blocked
waiting on events or I/O. These stats may be used to:

1.  Estimate how much CPU time a task has used.
2.  Estimate how much latency a task is experiencing due to other tasks (queue
    time), page fault handlers, and kernel lock contention.

### ZX_INFO_PROCESS_MAPS

*handle* type: **Process**, with **ZX_RIGHT_READ**

*buffer* type: `zx_info_maps_t[n]`

The `zx_info_maps_t` array is a depth-first pre-order walk of the target
process's Aspace/VMAR/Mapping tree. As per the pre-order traversal base
addresses will be in ascending order.

```
typedef struct zx_info_maps {
    // Name if available; empty string otherwise.
    char name[ZX_MAX_NAME_LEN];
    // Base address.
    zx_vaddr_t base;
    // Size in bytes.
    size_t size;

    // The depth of this node in the tree.
    // Can be used for indentation, or to rebuild the tree from an array
    // of zx_info_maps_t entries, which will be in depth-first pre-order.
    size_t depth;
    // The type of this entry; indicates which union entry is valid.
    uint32_t type; // zx_info_maps_type_t
    union {
        zx_info_maps_mapping_t mapping;
        // No additional fields for other types.
    } u;
} zx_info_maps_t;

typedef struct zx_info_maps_mapping {
    // MMU flags for the mapping.
    // Bitwise OR of ZX_VM_PERM_{READ,WRITE,EXECUTE} values.
    zx_vm_option_t mmu_flags;
    uint8_t padding1[4];
    // koid of the mapped VMO.
    zx_koid_t vmo_koid;
    // Offset into the above VMO.
    uint64_t vmo_offset;
    // The number of PAGE_SIZE pages in the mapped region of the VMO
    // that are backed by physical memory.
    size_t committed_pages;
} zx_info_maps_mapping_t;
```

The *depth* field of each entry describes its relationship to the nodes that
come before it. Depth 0 is the root Aspace, depth 1 is the root VMAR, and all
other entries have depth 2 or greater.

To get a full picture of how a process uses its VMOs and how a VMO is used by
various processes, you may need to combine this information with
ZX_INFO_PROCESS_VMOS.

See the `vmaps` command-line tool for an example user of this topic, and to dump
the maps of arbitrary processes by koid.

Additional errors:

*   **ZX_ERR_ACCESS_DENIED**: If the appropriate rights are missing.
*   **ZX_ERR_BAD_STATE**: If the target process has terminated, or if its
    address space has been destroyed

### ZX_INFO_PROCESS_VMOS

*handle* type: **Process**, with **ZX_RIGHT_READ**

*buffer* type: `zx_info_vmo_t[n]`

The `zx_info_vmo_t` array is list of all VMOs pointed to by the target process.
Some VMOs are mapped, some are pointed to by handles, and some are both.

Note: The same VMO may appear multiple times due to multiple mappings or
handles, or because a handle to the VMO has been removed and then readded
concurrently with this call. VMOs can change as the target process runs, which
may result in the same VMO having different values each time it appears. The
caller must resolve any duplicate values.

To get a full picture of how a process uses its VMOs and how a VMO is used by
various processes, you may need to combine this information with
ZX_INFO_PROCESS_MAPS.

```
// Describes a VMO.
typedef struct zx_info_vmo {
    // The koid of this VMO.
    zx_koid_t koid;

    // The name of this VMO.
    char name[ZX_MAX_NAME_LEN];

    // The size of this VMO; i.e., the amount of virtual address space it
    // would consume if mapped.
    uint64_t size_bytes;

    // If this VMO is a child , the koid of its parent. Otherwise, zero.
    // See |flags| for the type of child.
    zx_koid_t parent_koid;

    // The number of child of this VMO, if any.
    size_t num_children;

    // The number of times this VMO is currently mapped into VMARs.
    // Note that the same process will often map the same VMO twice,
    // and both mappings will be counted here. (I.e., this is not a count
    // of the number of processes that map this VMO; see share_count.)
    size_t num_mappings;

    // An estimate of the number of unique address spaces that
    // this VMO is mapped into. Every process has its own address space,
    // and so does the kernel.
    size_t share_count;

    // Bitwise OR of ZX_INFO_VMO_* values.
    uint32_t flags;

    // If |ZX_INFO_VMO_TYPE(flags) == ZX_INFO_VMO_TYPE_PAGED|, the amount of
    // memory currently allocated to this VMO; i.e., the amount of physical
    // memory it consumes. Undefined otherwise.
    uint64_t committed_bytes;

    // If |flags & ZX_INFO_VMO_VIA_HANDLE|, the handle rights.
    // Undefined otherwise.
    zx_rights_t handle_rights;

    // VMO mapping cache policy. One of ZX_CACHE_POLICY_*
    uint32_t cache_policy;

    // Amount of kernel memory, in bytes, allocated to track metadata
    // associated with this VMO.
    uint64_t metadata_bytes;

    // Running counter of the number of times the kernel, without user request,
    // performed actions on this VMO that would have caused |committed_bytes| to
    // report a different value.
    uint64_t committed_change_events;
} zx_info_vmo_t;
```

See the `vmos` command-line tool for an example user of this topic, and to dump
the VMOs of arbitrary processes by koid.

### ZX_INFO_KMEM_STATS

*handle* type: **Resource** (Specifically, the root resource)

*buffer* type: `zx_info_kmem_stats_t[1]`

Returns information about kernel memory usage.

```
typedef struct zx_info_kmem_stats {
    // The total amount of physical memory available to the system.
    // Note, the values below may not exactly add up to this total.
    size_t total_bytes;

    // The amount of unallocated memory.
    size_t free_bytes;

    // The amount of memory reserved by and mapped into the kernel for reasons
    // not covered by other fields in this struct. Typically for readonly data
    // like the ram disk and kernel image, and for early-boot dynamic memory.
    size_t wired_bytes;

    // The amount of memory allocated to the kernel heap.
    size_t total_heap_bytes;

    // The portion of |total_heap_bytes| that is not in use.
    size_t free_heap_bytes;

    // The amount of memory committed to VMOs, both kernel and user.
    // A superset of all userspace memory.
    // Does not include certain VMOs that fall under |wired_bytes|.
    size_t vmo_bytes;

    // The amount of memory used for architecture-specific MMU metadata
    // like page tables.
    size_t mmu_overhead_bytes;

    // Non-free memory that isn't accounted for in any other field.
    size_t other_bytes;
} zx_info_kmem_stats_t;
```

### ZX_INFO_KMEM_STATS_EXTENDED

*handle* type: **Resource** (Specifically, the root resource)

*buffer* type: `zx_info_kmem_stats_extended_t[1]`

Returns information about kernel memory usage - includes information returned by
the ZX_INFO_KMEM_STATS topic, plus some additional information that is more
expensive to collect.

```
typedef struct zx_info_kmem_stats_extended {
    // The total amount of physical memory available to the system.
    uint64_t total_bytes;

    // The amount of unallocated memory.
    uint64_t free_bytes;

    // The amount of memory reserved by and mapped into the kernel for reasons
    // not covered by other fields in this struct. Typically for readonly data
    // like the ram disk and kernel image, and for early-boot dynamic memory.
    uint64_t wired_bytes;

    // The amount of memory allocated to the kernel heap.
    uint64_t total_heap_bytes;

    // The portion of |total_heap_bytes| that is not in use.
    uint64_t free_heap_bytes;

    // The amount of memory committed to VMOs, both kernel and user.
    // A superset of all userspace memory.
    // Does not include certain VMOs that fall under |wired_bytes|.
    uint64_t vmo_bytes;

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

    // The amount of memory used for architecture-specific MMU metadata
    // like page tables.
    uint64_t mmu_overhead_bytes;

    // The amount of memory in use by IPC.
    uint64_t ipc_bytes;

    // Non-free memory that isn't accounted for in any other field.
    uint64_t other_bytes;
} zx_info_kmem_stats_extended_t;
```

### ZX_INFO_RESOURCE

*handle* type: **Resource**

*buffer* type: `zx_info_resource_t[1]`

Returns information about a resource object via its handle.

```
typedef struct zx_info_resource {
    // The resource kind; resource object kinds are described in resource.md
    uint32_t kind;
    // Resource's creation flags
    uint32_t flags;
    // Resource's base value (inclusive)
    uint64_t base;
    // Resource's length value
    size_t size;
    char name[ZX_MAX_NAME_LEN];
} zx_info_resource_t;
```

The resource kind is one of

*   **ZX_RSRC_KIND_ROOT**
*   **ZX_RSRC_KIND_MMIO**
*   **ZX_RSRC_KIND_IOPORT**
*   **ZX_RSRC_KIND_IRQ**
*   **ZX_RSRC_KIND_HYPERVISOR**
*   **ZX_RSRC_KIND_VMEX**
*   **ZX_RSRC_KIND_SMC**

### ZX_INFO_BTI

*handle* type: **Bus Transaction Initiator**

*buffer* type: `zx_info_bti_t[1]`

```
typedef struct zx_info_bti {
    // zx_bti_pin will always be able to return addresses that are contiguous for at
    // least this many bytes. E.g. if this returns 1MB, then a call to
    // zx_bti_pin() with a size of 2MB will return at most two physically-contiguous runs.
    // If the size were 2.5MB, it will return at most three physically-contiguous runs.
    uint64_t minimum_contiguity;

    // The number of bytes in the device's address space (UINT64_MAX if 2^64).
    uint64_t aspace_size;

    // The count of the pinned memory object tokens. Requesting this count is
    // racy, so this should only be used for informative reasons.
    uint64_t pmo_count;

    // The count of the quarantined pinned memory object tokens. Requesting this count is
    // racy, so this should only be used for informative reasons.
    uint64_t quarantine_count;
} zx_info_bti_t;
```

## RIGHTS

<!-- Updated by update-docs-from-fidl, do not edit. -->

If *topic* is **ZX_INFO_PROCESS**, *handle* must be of type
**ZX_OBJ_TYPE_PROCESS** and have **ZX_RIGHT_INSPECT**.

If *topic* is **ZX_INFO_JOB**, *handle* must be of type **ZX_OBJ_TYPE_JOB** and
have **ZX_RIGHT_INSPECT**.

If *topic* is **ZX_INFO_PROCESS_THREADS**, *handle* must be of type
**ZX_OBJ_TYPE_PROCESS** and have **ZX_RIGHT_ENUMERATE**.

If *topic* is **ZX_INFO_JOB_CHILDREN**, *handle* must be of type
**ZX_OBJ_TYPE_JOB** and have **ZX_RIGHT_ENUMERATE**.

If *topic* is **ZX_INFO_JOB_PROCESSES**, *handle* must be of type
**ZX_OBJ_TYPE_JOB** and have **ZX_RIGHT_ENUMERATE**.

If *topic* is **ZX_INFO_THREAD**, *handle* must be of type
**ZX_OBJ_TYPE_THREAD** and have **ZX_RIGHT_INSPECT**.

If *topic* is **ZX_INFO_THREAD_EXCEPTION_REPORT**, *handle* must be of type
**ZX_OBJ_TYPE_THREAD** and have **ZX_RIGHT_INSPECT**.

If *topic* is **ZX_INFO_THREAD_STATS**, *handle* must be of type
**ZX_OBJ_TYPE_THREAD** and have **ZX_RIGHT_INSPECT**.

If *topic* is **ZX_INFO_TASK_STATS**, *handle* must be of type
**ZX_OBJ_TYPE_PROCESS** and have **ZX_RIGHT_INSPECT**.

If *topic* is **ZX_INFO_PROCESS_MAPS**, *handle* must be of type
**ZX_OBJ_TYPE_PROCESS** and have **ZX_RIGHT_INSPECT**.

If *topic* is **ZX_INFO_PROCESS_VMOS**, *handle* must be of type
**ZX_OBJ_TYPE_PROCESS** and have **ZX_RIGHT_INSPECT**.

If *topic* is **ZX_INFO_VMO**, *handle* must be of type **ZX_OBJ_TYPE_VMO**.

If *topic* is **ZX_INFO_VMAR**, *handle* must be of type **ZX_OBJ_TYPE_VMAR**
and have **ZX_RIGHT_INSPECT**.

If *topic* is **ZX_INFO_CPU_STATS**, *handle* must have resource kind
**ZX_RSRC_KIND_ROOT**.

If *topic* is **ZX_INFO_KMEM_STATS**, *handle* must have resource kind
**ZX_RSRC_KIND_ROOT**.

If *topic* is **ZX_INFO_RESOURCE**, *handle* must be of type
**ZX_OBJ_TYPE_RESOURCE** and have **ZX_RIGHT_INSPECT**.

If *topic* is **ZX_INFO_HANDLE_COUNT**, *handle* must have **ZX_RIGHT_INSPECT**.

If *topic* is **ZX_INFO_BTI**, *handle* must be of type **ZX_OBJ_TYPE_BTI** and
have **ZX_RIGHT_INSPECT**.

If *topic* is **ZX_INFO_PROCESS_HANDLE_STATS**, *handle* must be of type
**ZX_OBJ_TYPE_PROCESS** and have **ZX_RIGHT_INSPECT**.

If *topic* is **ZX_INFO_SOCKET**, *handle* must be of type
**ZX_OBJ_TYPE_SOCKET** and have **ZX_RIGHT_INSPECT**.

If *topic* is **ZX_INFO_MSI**, *handle* must be of type **ZX_OBJ_TYPE_MSI** and
have **ZX_RIGHT_INSPECT**.

If *topic* is **ZX_INFO_TASK_RUNTIME**, *handle* must be of type
**ZX_OBJ_TYPE_THREAD**, **ZX_OBJ_TYPE_PROCESS**, or **ZX_OBJ_TYPE_JOB**, and
have **ZX_RIGHT_INSPECT**.

## RETURN VALUE

`zx_object_get_info()` returns **ZX_OK** on success. In the event of failure, a
negative error value is returned.

## ERRORS

**ZX_ERR_BAD_HANDLE** *handle* is not a valid handle.

**ZX_ERR_WRONG_TYPE** *handle* is not an appropriate type for *topic*

**ZX_ERR_ACCESS_DENIED**: If *handle* does not have the necessary rights for the
operation.

**ZX_ERR_INVALID_ARGS** *buffer*, *actual*, or *avail* are invalid pointers.

**ZX_ERR_NO_MEMORY** Failure due to lack of memory. There is no good way for
userspace to handle this (unlikely) error. In a future build this error will no
longer occur.

**ZX_ERR_BUFFER_TOO_SMALL** The *topic* returns a fixed number of records, but
the provided buffer is not large enough for these records.

**ZX_ERR_NOT_SUPPORTED** *topic* does not exist.

## EXAMPLES

```
bool is_handle_valid(zx_handle_t handle) {
    return zx_object_get_info(
        handle, ZX_INFO_HANDLE_VALID, NULL, 0, NULL, NULL) == ZX_OK;
}

zx_koid_t get_object_koid(zx_handle_t handle) {
    zx_info_handle_basic_t info;
    if (zx_object_get_info(handle, ZX_INFO_HANDLE_BASIC,
                           &info, sizeof(info), NULL, NULL) != ZX_OK) {
        return 0;
    }
    return info.koid;
}

void examine_threads(zx_handle_t proc) {
    zx_koid_t threads[128];
    size_t count, avail;

    if (zx_object_get_info(proc, ZX_INFO_PROCESS_THREADS, threads,
                           sizeof(threads), &count, &avail) != ZX_OK) {
        // Error!
    } else {
        if (avail > count) {
            // More threads than space in array;
            // could call again with larger array.
        }
        for (size_t n = 0; n < count; n++) {
            do_something(thread[n]);
        }
    }
}
```

## SEE ALSO

-   [`zx_handle_close()`]
-   [`zx_handle_duplicate()`]
-   [`zx_handle_replace()`]
-   [`zx_object_get_child()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_channel_call()`]: channel_call.md
[`zx_futex_wait()`]: futex_wait.md
[`zx_handle_close()`]: handle_close.md
[`zx_handle_duplicate()`]: handle_duplicate.md
[`zx_handle_replace()`]: handle_replace.md
[`zx_interrupt_wait()`]: interrupt_wait.md
[`zx_nanosleep()`]: nanosleep.md
[`zx_object_get_child()`]: object_get_child.md
[`zx_object_wait_many()`]: object_wait_many.md
[`zx_object_wait_one()`]: object_wait_one.md
[`zx_port_wait()`]: port_wait.md
[`zx_task_suspend()`]: task_suspend.md
