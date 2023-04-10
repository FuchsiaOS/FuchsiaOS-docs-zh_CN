# Zircon

Zircon is the core platform that powers Fuchsia. Zircon is
composed of a kernel (source in [/zircon/kernel](/zircon/kernel))
as well as a small set of userspace services, drivers, and libraries
(source in [/zircon/system/](/zircon/system)) necessary for the system
to boot, talk to hardware, load userspace processes and run them, etc.
Fuchsia builds a much larger OS on top of this foundation.

The canonical Zircon repository is part of the Fuchsia project
at: [https://fuchsia.googlesource.com/fuchsia/+/HEAD/zircon/](/zircon/)

The Zircon Kernel provides syscalls to manage processes, threads,
virtual memory, inter-process communication, waiting on object state
changes, and locking (via futexes).

Currently there are some temporary syscalls that have been used for early
bringup work, which will be going away in the future as the long term
syscall API and ABI surface is finalized. The expectation is that there will
be about 100 syscalls.

Zircon syscalls are generally non-blocking. The `wait_one`, `wait_many`
`port_wait` and `thread sleep` being the notable exceptions.

This page is a non-comprehensive index of the zircon documentation.

+ [Getting Started](/docs/development/kernel/getting_started.md)
+ [Contributing
  Patches to Zircon](/docs/development/source_code/contribute_changes.md#contributing-patches-to-zircon)

+ [Zircon Concepts](/docs/concepts/kernel/concepts.md)
+ [Kernel Objects](/docs/reference/kernel_objects/objects.md)
+ [Kernel Invariants](kernel_invariants.md)
+ [Kernel Scheduling](kernel_scheduling.md)
+ [Kernel Thread Signaling](kernel_thread_signaling.md)
+ [Fair Scheduler](fair_scheduler.md)
+ [Errors](errors.md)
+ [Time units](/docs/development/kernel/time.md)

+ [Process Objects](/docs/reference/kernel_objects/process.md)
+ [Thread Objects](/docs/reference/kernel_objects/thread.md)
+ [Thread local storage](/docs/development/kernel/threads/tls.md)
+ [Thread annotations](/docs/development/kernel/threads/thread_annotations.md)
+ [Handles](/docs/concepts/kernel/handles.md)
+ [Lock validation](lockdep.md)
+ [System Calls](/docs/reference/syscalls/README.md)
+ [zxcrypt](/docs/concepts/filesystems/zxcrypt.md)

+ [Fuchsia Driver Framework](/docs/development/drivers/concepts/getting_started.md)
+ [Driver interfaces - audio overview](/docs/development/audio/drivers/overview.md)

+ [libc](/docs/development/languages/c-cpp/libc.md)
+ [C++ fpromise::promise<> guide](/docs/development/languages/c-cpp/fpromise_promise_guide.md)

+ [Testing](/docs/development/testing/testing.md)
+ [Kernel tracing](/docs/development/tracing/advanced/recording-a-kernel-trace.md)
+ [Block device testing](/docs/development/testing/block_device_testing.md)
+ [nand Testing](/docs/development/testing/nand_testing.md)

+ [Compile-time object collections](/docs/development/languages/c-cpp/compile_time_object_collections.md)
+ [ACPI debugging](/docs/development/debugging/acpi.md)
+ [Entropy collection TODOs](/docs/concepts/kernel/jitterentropy/entropy_collection_todos.md)
+ [Memory usage analysis tools](/docs/development/kernel/memory/memory.md)
+ [Symbolizer](/docs/reference/kernel/symbolizer_markup.md)
+ [Relationship with LK](zx_and_lk.md)
+ [Viewing microbenchmarks with Chromeperf](/docs/development/performance/chromeperf_user_guide.md)
+ [Avoiding a problem with the SYSRET instruction](sysret_problem.md)
