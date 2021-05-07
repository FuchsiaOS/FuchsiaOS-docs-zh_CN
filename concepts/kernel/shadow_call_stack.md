# ShadowCallStack in Zircon & Fuchsia

[TOC]

## Introduction

LLVM's [shadow-call-stack feature][shadow-call-stack] is a compiler mode
intended to harden the generated code against stack-smashing attacks such as
exploits of buffer overrun bugs.

The Clang/LLVM documentation page linked above describes the scheme.  The
capsule summary is that the function return address is never reloaded from the
normal stack but only from a separate "shadow call stack".  This is an
additional stack, but rather than containing whole stack frames of whatever
size each function needs, it contains only a single address word for each call
frame it records: just the return address.  Since the shadow call stack is
allocated independently of other stacks or heap blocks with its own randomized
address to which pointers are rare, it is much less likely that some sort of
buffer overrun or use-after-free exploit will overwrite a return address in
memory so that it can cause the program to return to an instruction by the
attacker.

The [shadow-call-stack] and [safe-stack] instrumentation schemes and ABIs are
related and similar but also orthogonal.  Each can be enabled or disabled
independently for any function.  Fuchsia's compiler ABI and libc always
interoperate with code built with or without either kind of instrumentation,
regardless of what instrumentation was or wasn't used in the particular libc
build.

[shadow-call-stack]: https://clang.llvm.org/docs/ShadowCallStack.html
[safe-stack]: safestack.md

## Interoperation and ABI Effects

In general, shadow-call-stack does not affect the ABI.  The machine-specific
calling conventions are unchanged.  It works fine to have some functions in a
program built with shadow-call-stack and some not.  It doesn't matter if
combining the two comes from directly compiled `.o` files, from archive
libraries (`.a` files), or from shared libraries (`.so` files), in any
combination.

While there is some additional per-thread state (the *shadow call stack
pointer*, [see below](#implementation-details)), code not using
shadow-call-stack does not need to do anything about this state to keep it
correct when calling, or being called by, code that does use safe-stack.  The
only potential exceptions to this are for code that is implementing its own
kinds of non-local exits or context-switching (e.g. coroutines).  The Zircon C
library's `setjmp`/`longjmp` code saves and restores this additional state
automatically, so anything that is based on `longjmp` already handles everything
correctly even if the code calling `setjmp` and `longjmp` doesn't know about
shadow-call-stack.

For AArch64 (ARM64), the `x18` register is already reserved as "fixed" in the
ABI generally.  Code unaware of the shadow-call-stack extension to the ABI is
interoperable with the shadow-call-stack ABI by default if it simply never
touches `x18`.

The feature is not yet supported on any other architecture.

## Use in Zircon & Fuchsia

Zircon on Aarch64 (ARM64) supports shadow-call-stack both in the kernel and
for user-mode code.  This is enabled in the Clang compiler by the
`-fsanitize=shadow-call-stack` command-line option.  For `aarch64-fuchsia`
(ARM64) targets, it is enabled by default.  To disable it for a specific
compilation, use the `-fno-sanitize=shadow-call-stack` command-line option.

As with [safe-stack], there is no separate facility for specifying the size of
the shadow call stack.  Instead, the size specified for "the stack" in legacy
APIs (such as `pthread_attr_setstacksize`) and ABIs (such as `PT_GNU_STACK`) is
used as the size for **each** kind of stack.  Because the different kinds of
stack are used in different proportions according to the particular program
behavior, there is no good way to choose the shadow call stack size based on
the traditional single stack size.  So each kind of stack is as big as it might
need to be in the worst case expected by the tuned "unitary" stack size.  While
this seems wasteful, it is only slightly so: at worst one page is wasted per
kind of stack, plus the page table overhead of using more address space for
pages that are never accessed.

## Implementation details

The essential addition to support shadow-call-stack code is the *shadow call
stack pointer*.  This is a register with a global use, like the traditional
stack pointer.  But each call frame pushes and pops a single return address
word rather than arbitrary data as in the normal stack frame.

For AArch64 (ARM64), the `x18` register holds the shadow call stack pointer at
function entry.  The shadow call stack grows upwards with post-increment
semantics, so `x18` always points to the next free slot.  The compiler never
touches the register except to spill and reload the return address register
(`x30`, aka LR).  The Fuchsia ABI requires that `x18` contain a valid shadow
stack pointer at all times.  That is, it must **always** be valid to push a
new address onto the shadow call stack at `x18` (modulo stack overflow).

### Notes for low-level and assembly code

Most code, even in assembly, does not need to think about shadow-call-stack
issues at all.  The calling conventions are not changed.  All use of the stack
(and/or the [unsafe stack][safe-stack]) is the same with or without
shadow-call-stack; *when frame pointers are enabled*, the return address will
be stored on the machine stack next to the frame pointer as expected.  For
AArch64 (ARM64), function calls still use `x30` for the return address as
normal, though functions that clobber `x30` can choose to spill and reload it
using different memory.  Non-leaf functions written in assembly should ideally
make use of the shadow-call-stack ABI by spilling and reloading the return
address register there instead of on the machine stack.

The main exception is code that is implementing something like a non-local
exit or context switch.  Such code may need to save or restore the shadow call
stack pointer.  Both the `longjmp` function and C++ `throw` already handle
this directly, so C or C++ code using those constructs does not need to do
anything new.

New code implementing some new kind of non-local exit or context switch will
need to handle the shadow call stack pointer similarly to how it handles the
traditional machine stack pointer register and the [unsafe stack][safe-stack]
pointer.  Any such code should use `#if __has_feature(shadow_call_stack)` to
test at compile time whether shadow-call-stack is being used in the particular
build.  That preprocessor construct can be used in C, C++, or assembly (`.S`)
source files.
