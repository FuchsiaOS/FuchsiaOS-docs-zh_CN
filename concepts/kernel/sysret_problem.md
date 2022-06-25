# Avoiding a problem with the SYSRET and IRETQ instructions

On x86-64, the kernel uses the `SYSRET` and `IRETQ` instructions to return
from system calls and interrupts, respectively. We must be careful not to
use a non-canonical return address in these instructions, at least on Intel
CPUs, because this causes the instructions to fault in kernel mode, which
is unsafe. In contrast, on AMD CPUs, `SYSRET` faults in user mode when used
with a non-canonical return address.

One of the problems with these instructions faulting in kernel mode is that
the instructions occur at the end of the interrupt or syscall handling
mechanism, after the `gs` register has been swapped from the kernel
`x86_percpu` variable to a value that is controlled by userspace. When an
exception occurs in kernel mode, the `gs` register is not changed because
it assumes that the current `gs` register belongs to the kernel. This would
lead to the kernel handling the fault using a `x86_percpu` structure
controlled by the user and could easily lead to kernel code execution.

Usually, the lowest non-negative non-canonical address is `0x0000800000000000`
(== 1 << 47).  One way that a user process could cause the syscall return
address to be non-canonical is by mapping a 4k executable page immediately
below that address (at `0x00007ffffffff000`), putting a `SYSCALL` instruction
at the end of that page, and executing the `SYSCALL` instruction.

To avoid this problem:

*   We disallow mapping a page when the virtual address of the following page
    will be non-canonical.

*   We disallow setting the `RIP` register to a non-canonical address using
    [`zx_thread_write_state()`][thread_write_state].

*   We disallow setting the thread entry point to a non-canonical address in
    `ThreadDispatcher::MakeRunnable()`.

*   We disallow setting non-userspace addresses in
    [`zx_thread_start()`][thread_start] and [`zx_process_start()`][process_start].

[thread_write_state]: reference/syscalls/thread_write_state.md
[thread_start]: reference/syscalls/thread_start.md
[process_start]: reference/syscalls/process_start.md
