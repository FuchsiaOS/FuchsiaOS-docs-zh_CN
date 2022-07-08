# Zircon System Calls

[Life of a Fuchsia syscall](/concepts/kernel/life_of_a_syscall.md)

## Handles
+ [handle_close](handle_close.md) - close a handle
+ [handle_close_many](handle_close_many.md) - close several handles
+ [handle_duplicate](handle_duplicate.md) - create a duplicate handle (optionally with reduced rights)
+ [handle_replace](handle_replace.md) - create a new handle (optionally with reduced rights) and destroy the old one

## Objects
+ [object_get_child](object_get_child.md) - find the child of an object by its koid
+ [object_get_info](object_get_info.md) - obtain information about an object
+ [object_get_property](object_get_property.md) - read an object property
+ [object_set_profile](object_set_profile.md) - apply a profile to a thread
+ [object_set_property](object_set_property.md) - modify an object property
+ [object_signal](object_signal.md) - set or clear the user signals on an object
+ [object_signal_peer](object_signal_peer.md) - set or clear the user signals in the opposite end
+ [object_wait_many](object_wait_many.md) - wait for signals on multiple objects
+ [object_wait_one](object_wait_one.md) - wait for signals on one object
+ [object_wait_async](object_wait_async.md) - asynchronous notifications on signal change

## Threads
+ [thread_create](thread_create.md) - create a new thread within a process
+ [thread_exit](thread_exit.md) - exit the current thread
+ [thread_read_state](thread_read_state.md) - read register state from a thread
+ [thread_start](thread_start.md) - cause a new thread to start executing
+ [thread_write_state](thread_write_state.md) - modify register state of a thread

## Processes
+ [process_create](process_create.md) - create a new process within a job
+ [process_read_memory](process_read_memory.md) - read from a process's address space
+ [process_start](process_start.md) - cause a new process to start executing
+ [process_write_memory](process_write_memory.md) - write to a process's address space
+ [process_exit](process_exit.md) - exit the current process

## Jobs
+ [job_create](job_create.md) - create a new job within a job
+ [job_set_critical](job_set_critical.md) - set a process as critical to a job
+ [job_set_policy](job_set_policy.md) - modify policies for a job and its descendants

## Tasks (Thread, Process, or Job)
+ [task_create_exception_channel](task_create_exception_channel.md) - create an exception channel on a task
+ [task_kill](task_kill.md) - cause a task to stop running
+ [task_suspend](task_suspend.md) - cause a task to be suspended

## Profiles
+ [profile_create](profile_create.md) - create a new profile object

## Exceptions
+ [exception_get_thread](exception_get_thread.md) - create a handle for the exception thread
+ [exception_get_process](exception_get_process.md) - create a handle for the exception process

## Channels
+ [channel_call](channel_call.md) - synchronously send a message and receive a reply
+ [channel_call_etc](channel_call_etc.md) - synchronously send a message and receive a reply with handle information
+ [channel_create](channel_create.md) - create a new channel
+ [channel_read](channel_read.md) - receive a message from a channel
+ [channel_read_etc](channel_read_etc.md) - receive a message from a channel with handle information
+ [channel_write](channel_write.md) - write a message to a channel
+ [channel_write_etc](channel_write_etc.md) - write a message to the channel and modify the handles

## Sockets
+ [socket_create](socket_create.md) - create a new socket
+ [socket_read](socket_read.md) - read data from a socket
+ [socket_set_disposition](socket_set_disposition.md) - set write disposition of a socket
+ [socket_write](socket_write.md) - write data to a socket

## Stream
+ [stream_create](stream_create.md) - create a stream from a VMO
+ [stream_readv](stream_readv.md) - read data from the stream at the current seek offset
+ [stream_readv_at](stream_readv_at.md) - read data from the stream at a given offset
+ [stream_writev](stream_writev.md) - write data to the stream at the current seek offset
+ [stream_writev_at](stream_writev_at.md) - write data to the stream at a given offset
+ [stream_seek](stream_seek.md) - modify the current seek offset of the stream

## Fifos
+ [fifo_create](fifo_create.md) - create a new fifo
+ [fifo_read](fifo_read.md) - read data from a fifo
+ [fifo_write](fifo_write.md) - write data to a fifo

## Events and Event Pairs
+ [event_create](event_create.md) - create an event
+ [eventpair_create](eventpair_create.md) - create a connected pair of events
+ [system_get_event](system_get_event.md) - retrieve a handle to a system event

## Ports
+ [port_create](port_create.md) - create a port
+ [port_queue](port_queue.md) - send a packet to a port
+ [port_wait](port_wait.md) - wait for packets to arrive on a port
+ [port_cancel](port_cancel.md) - cancel notifications from async_wait

## Futexes
+ [futex_wait](futex_wait.md) - wait on a futex
+ [futex_wake](futex_wake.md) - wake waiters on a futex
+ [futex_requeue](futex_requeue.md) - wake some waiters and requeue other waiters

## Virtual Memory Objects (VMOs)
+ [vmo_create](vmo_create.md) - create a new vmo
+ [vmo_read](vmo_read.md) - read from a vmo
+ [vmo_write](vmo_write.md) - write to a vmo
+ [vmo_create_child](vmo_create_child.md) - creates a child of a vmo
+ [vmo_get_size](vmo_get_size.md) - obtain the size of a vmo
+ [vmo_set_size](vmo_set_size.md) - adjust the size of a vmo
+ [vmo_op_range](vmo_op_range.md) - perform an operation on a range of a vmo
+ [vmo_replace_as_executable](vmo_replace_as_executable.md) - add execute rights to a vmo
+ [vmo_create_physical](vmo_create_physical.md) - create a VM object referring to a specific contiguous range of physical memory
+ [vmo_set_cache_policy](vmo_set_cache_policy.md) - set the caching policy for pages held by a VMO

## Virtual Memory Address Regions (VMARs)
+ [vmar_allocate](vmar_allocate.md) - create a new child VMAR
+ [vmar_map](vmar_map.md) - map a VMO into a process
+ [vmar_unmap](vmar_unmap.md) - unmap a memory region from a process
+ [vmar_protect](vmar_protect.md) - adjust memory access permissions
+ [vmar_op_range](vmar_op_range.md) - perform an operation on VMOs mapped into a range of a VMAR
+ [vmar_destroy](vmar_destroy.md) - destroy a VMAR and all of its children

## Userspace Pagers
+ [pager_create](pager_create.md) - create a new pager object
+ [pager_create_vmo](pager_create_vmo.md) - create a pager owned vmo
+ [pager_detach_vmo](pager_detach_vmo.md) - detaches a pager from a vmo
+ [pager_supply_pages](pager_supply_pages.md) - supply pages into a pager owned vmo
+ [pager_op_range](pager_op_range.md) - perform an operation on a range of a pager owned vmo

## Cryptographically Secure RNG
+ [cprng_add_entropy](cprng_add_entropy.md)
+ [cprng_draw](cprng_draw.md)

## Time
+ [nanosleep](nanosleep.md) - sleep for some number of nanoseconds
+ [clock_get_monotonic](clock_get_monotonic.md) - read the monotonic system clock
+ [clock_create](clock_create.md) - Create a new clock object
+ [clock_get_details](clock_get_details.md) - Fetch all of the low level details of the clock's current status
+ [clock_update](clock_update.md) - Make adjustments to a clock object
+ [ticks_get](ticks_get.md) - read high-precision timer ticks
+ [ticks_per_second](ticks_per_second.md) - read the number of high-precision timer ticks in a second
+ [deadline_after](deadline_after.md) - Convert a time relative to now to an absolute deadline

## Timers
+ [timer_create](timer_create.md) - create a timer object
+ [timer_set](timer_set.md) - start a timer
+ [timer_cancel](timer_cancel.md) - cancel a timer

## Hypervisor guests
+ [guest_create](guest_create.md) - create a hypervisor guest
+ [guest_set_trap](guest_set_trap.md) - set a trap in a hypervisor guest

## Virtual CPUs
+ [vcpu_create](vcpu_create.md) - create a VCPU
+ [vcpu_enter](vcpu_enter.md) - enter a VCPU, and start or continue execution
+ [vcpu_kick](vcpu_kick.md) - kick a VCPU, and stop execution
+ [vcpu_interrupt](vcpu_interrupt.md) - raise an interrupt on a VCPU
+ [vcpu_read_state](vcpu_read_state.md) - read state from a VCPU
+ [vcpu_write_state](vcpu_write_state.md) - write state to a VCPU
+ [interrupt_bind_vcpu](interrupt_bind_vcpu.md) - bind an interrupt object to a VCPU

## Global system information
+ [system_get_dcache_line_size](system_get_dcache_line_size.md)
+ [system_get_features](system_get_features.md) - get hardware-specific features
+ [system_get_num_cpus](system_get_num_cpus.md) - get number of CPUs
+ [system_get_page_size](system_get_page_size.md) - get memory page size
+ [system_get_physmem](system_get_physmem.md) - get physical memory size
+ [system_get_version_string](system_get_version_string.md) - get version string

## Debug Logging
+ [debuglog_create](debuglog_create.md) - create a kernel managed debuglog reader or writer
+ [debuglog_write](debuglog_write.md) - write log entry to debuglog
+ [debuglog_read](debuglog_read.md) - read log entries from debuglog
+ [debug_read](debug_read.md) - TODO(fxbug.dev/32938)
+ [debug_write](debug_write.md) - TODO(fxbug.dev/32938)
+ [debug_send_command](debug_send_command.md) - TODO(fxbug.dev/32938)

## Multi-function
+ [vmar_unmap_handle_close_thread_exit](vmar_unmap_handle_close_thread_exit.md) - three-in-one
+ [futex_wake_handle_close_thread_exit](futex_wake_handle_close_thread_exit.md) - three-in-one

## System
+ [system_mexec](system_mexec.md) - Soft reboot the system with a new kernel and bootimage
+ [system_mexec_payload_get](system_mexec_payload_get.md) - Return a ZBI containing ZBI entries necessary to boot this system
+ [system_powerctl](system_powerctl.md)
+ [system_get_performance_info](system_get_performance_info.md) - Get CPU performance info
+ [system_set_performance_info](system_set_performance_info.md) - Set CPU performance info

## DDK
+ [bti_create](bti_create.md) - create a new bus transaction initiator
+ [bti_pin](bti_pin.md) - pin pages and grant devices access to them
+ [bti_release_quarantine](bti_release_quarantine.md) - releases all quarantined PMTs
+ [cache_flush](cache_flush.md) - Flush CPU data and/or instruction caches
+ [interrupt_ack](interrupt_ack.md) - Acknowledge an interrupt object
+ [interrupt_bind](interrupt_bind.md) - Bind an interrupt object to a port
+ [interrupt_create](interrupt_create.md) - Create a physical or virtual interrupt object
+ [interrupt_destroy](interrupt_destroy.md) - Destroy an interrupt object
+ [interrupt_trigger](interrupt_trigger.md) - Trigger a virtual interrupt object
+ [interrupt_wait](interrupt_wait.md) - Wait on an interrupt object
+ [iommu_create](iommu_create.md) - create a new IOMMU object in the kernel
+ [pmt_unpin](pmt_unpin.md) - unpin pages and revoke device access to them
+ [resource_create](resource_create.md) - create a resource object
+ [smc_call](smc_call.md) - Make an SMC call from user space

## Display drivers
+ [framebuffer_get_info](framebuffer_get_info.md)
+ [framebuffer_set_range](framebuffer_set_range.md)

## Tracing
+ [ktrace_control](ktrace_control.md)
+ [ktrace_read](ktrace_read.md)
+ [ktrace_write](ktrace_write.md)
+ [mtrace_control](mtrace_control.md)

## Others/Work in progress
+ [ioports_release](ioports_release.md)
+ [pc_firmware_tables](pc_firmware_tables.md)
+ [pci_add_subtract_io_range](pci_add_subtract_io_range.md)
+ [pci_cfg_pio_rw](pci_cfg_pio_rw.md)
+ [pci_config_read](pci_config_read.md)
+ [pci_config_write](pci_config_write.md)
+ [pci_enable_bus_master](pci_enable_bus_master.md)
+ [pci_get_bar](pci_get_bar.md)
+ [pci_get_nth_device](pci_get_nth_device.md)
+ [pci_init](pci_init.md)
+ [pci_map_interrupt](pci_map_interrupt.md)
+ [pci_query_irq_mode](pci_query_irq_mode.md)
+ [pci_reset_device](pci_reset_device.md)
+ [pci_set_irq_mode](pci_set_irq_mode.md)


+ [syscall_test_0](syscall_test_0.md)
+ [syscall_test_1](syscall_test_1.md)
+ [syscall_test_2](syscall_test_2.md)
+ [syscall_test_3](syscall_test_3.md)
+ [syscall_test_4](syscall_test_4.md)
+ [syscall_test_5](syscall_test_5.md)
+ [syscall_test_6](syscall_test_6.md)
+ [syscall_test_7](syscall_test_7.md)
+ [syscall_test_8](syscall_test_8.md)
+ [syscall_test_wrapper](syscall_test_wrapper.md)
+ [syscall_test_handle_create](syscall_test_handle_create.md)

## Syscall generation

Syscall support is generated from `//zircon/vdso`. The FIDL files in that
directory are first run through `fidlc`, which produces an intermediate format.
That intermediate format is consumed by [kazoo](/zircon/tools/kazoo), which
produces output for both the kernel and userspace in a variety of languages.
This output includes C or C++ headers for both the kernel and userspace, syscall
entry points, other language bindings, and so on.

This tool is invoked as a part of the build, rather than checking in its output.

