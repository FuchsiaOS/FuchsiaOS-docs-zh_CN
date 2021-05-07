# SafeStack in Zircon & Fuchsia

[TOC]

## Introduction

LLVM's [safe-stack feature](https://clang.llvm.org/docs/SafeStack.html)
is a compiler mode intended to harden the generated code against
stack-smashing attacks such as exploits of buffer overrun bugs.

The Clang/LLVM documentation page linked above describes the general
scheme.  The capsule summary is that that each thread has two stacks
instead of the usual one: a "safe stack" and an "unsafe stack".  The
unsafe stack is used for all purposes where a pointer into the stack
memory might be used, while the safe stack is used only for purposes
where no code should ever see a pointer into the stack memory.  So, the
unsafe stack is used for arrays or variables that are passed by
reference to another function or have their addresses stored in the
heap--memory that could be subject to buffer overrun or use-after-free
bugs and their exploits.  The safe stack is used for the compiler's
register spills, and for the return address of a function call.  Thus,
for example, a simple buffer overrun bug cannot be exploited to
overwrite the return address of the containing function, which is the
basis of exploits and attacks using the so-called *ROP*
("return-oriented programming") technique.

The **Compatibility** section of that page does not apply to Zircon (or
Fuchsia).  In Zircon user-mode code (including all of Fuchsia), the
runtime support for SafeStack is included directly in the standard C
runtime library, and everything works fine in shared libraries (DSOs).

The [safe-stack](https://clang.llvm.org/docs/SafeStack.html) and
[shadow-call-stack](shadow_call_stack.md) instrumentation schemes and ABIs are
related and similar but also orthogonal.  Each can be enabled or disabled
independently for any function.  Fuchsia's compiler ABI and libc always
interoperate with code built with or without either kind of instrumentation,
regardless of what instrumentation was or wasn't used in the particular libc
build.

## Interoperation and ABI Effects

In general, safe-stack does not affect the ABI.  The machine-specific
calling conventions are unchanged.  It works fine to have some
functions in a program built with safe-stack and some not.  It doesn't
matter if combining the two comes from directly compiled `.o` files,
from archive libraries (`.a` files), or from shared libraries (`.so`
files), in any combination.

While there is some additional per-thread state (the *unsafe stack
pointer*, see below under *Implementation details*), code not using
safe-stack does not need to do anything about this state to keep it
correct when calling, or being called by, code that does use
safe-stack.  The only potential exceptions to this are for code that
is implementing its own kinds of non-local exits or context-switching
(e.g. coroutines).  The Zircon C library's `setjmp`/`longjmp` code
saves and restores this additional state automatically, so anything
that is based on `longjmp` already handles everything correctly even
if the code calling `setjmp` and `longjmp` doesn't know about
safe-stack.

## Use in Zircon & Fuchsia

This is enabled in the Clang compiler by the `-fsanitize=safe-stack`
command-line option.  This is the default mode of the compiler for `*-fuchsia`
targets.  To disable it for a specific compilation, use the
`-fno-sanitize=safe-stack` option.

Zircon supports safe-stack for both user-mode and kernel code.
In the Zircon build, safe-stack is always enabled when building
with Clang (pass `variants = [ "clang" ]` to `GN`).

## Implementation details

The essential addition to support safe-stack code is the *unsafe stack
pointer*.  In the abstract, this can be thought of as an additional
register just like the machine's normal stack pointer register.  The
machine's stack pointer register is used for the safe stack, just as it
always has been.  The unsafe stack pointer is used as if it were another
register with a fixed purpose in the ABI, but of course the machines
don't actually have a new register, and for compatibility safe-stack
does not change the basic machine-specific calling conventions that
assign uses to all the machine registers.

The C and C++ ABI for Zircon and Fuchsia stores the unsafe stack
pointer in memory that's at a fixed offset from the thread pointer.
The [`<zircon/tls.h>`](/zircon/system/public/zircon/tls.h) header defines
the offset for each machine.

For x86 user-mode, the thread pointer is the `fsbase`, meaning access
in assembly code looks like `%fs:ZX_TLS_UNSAFE_SP_OFFSET`.
For the x86 kernel, the thread pointer is the `gsbase`, meaning access
in assembly code looks like `%gs:ZX_TLS_UNSAFE_SP_OFFSET`.

For Aarch64 (ARM64), in C or C++ code, `__builtin_thread_pointer()`
returns the thread pointer.  In user-mode, the thread pointer is in the
`TPIDR_EL0` special register and must be fetched into a normal register
(with `mrs *reg*, TPIDR_EL0`) to access the memory, so it's not a single
instruction in assembly code.  In the kernel, it's just the same but
using the `TPIDR_EL1` special register instead.

### Notes for low-level and assembly code

Most code, even in assembly, does not need to think about safe-stack
issues at all.  The calling conventions are not changed.  Using the
stack for saving registers, finding return addresses, etc. is all the
same with or without safe-stack.  The main exception is code that is
implementing something like a non-local exit or context switch.  Such
code may need to save or restore the unsafe stack pointer.  Both the
`longjmp` function and C++ `throw` already handle this directly, so
C or C++ code using those constructs does not need to do anything new.

The context-switch code in the kernel handles switching the unsafe stack
pointer.  On x86, this is explicit in the code: `%gs` points to the
`struct x86_percpu`, which has a member `kernel_unsafe_sp` at
`ZX_TLS_UNSAFE_SP_OFFSET`; `arch_context_switch` copies this into the
`unsafe_sp` field of the old thread's `struct arch_thread` and then
copies the new thread's `unsafe_sp` into `kernel_unsafe_sp`.  On ARM64,
this is implicitly done by `set_current_thread`, because that changes
the `TPIDR_EL1` special register, which points directly into the
per-thread `struct thread` rather than a per-CPU structure like on x86.

New code implementing some new kind of non-local exit or context switch
will need to handle the unsafe stack pointer similarly to how it handles
the traditional machine stack pointer register.  Any such code should
use `#if __has_feature(safe_stack)` to test at compile time whether
safe-stack is being used in the particular build.  That preprocessor
construct can be used in C, C++, or assembly (`.S`) source files.
