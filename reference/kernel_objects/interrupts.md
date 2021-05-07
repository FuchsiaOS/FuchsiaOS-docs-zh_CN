# Interrupts

## NAME

interrupts - Usermode I/O interrupt delivery

## SYNOPSIS

Interrupt objects allow userspace to create, signal, and wait on
hardware interrupts.

## DESCRIPTION

TODO

## NOTES

Interrupt Objects are private to the DDK and not generally available
to userspace processes.

## SYSCALLS

 - [`zx_interrupt_create()`] - Create an interrupt handle
 - [`zx_interrupt_destroy()`] - Destroy an interrupt handle
 - [`zx_interrupt_bind()`] - Bind an interrupt vector to interrupt handle
 - [`zx_interrupt_wait()`] - Wait for an interrupt on an interrupt handle
 - [`zx_interrupt_trigger()`] - Triggers a virtual interrupt on an interrupt handle
 - [`zx_interrupt_ack()`] - Acknowledge an interrupt and re-arm it

[`zx_interrupt_ack()`]: /docs/reference/syscalls/interrupt_ack.md
[`zx_interrupt_bind()`]: /docs/reference/syscalls/interrupt_bind.md
[`zx_interrupt_create()`]: /docs/reference/syscalls/interrupt_create.md
[`zx_interrupt_destroy()`]: /docs/reference/syscalls/interrupt_destroy.md
[`zx_interrupt_trigger()`]: /docs/reference/syscalls/interrupt_trigger.md
[`zx_interrupt_wait()`]: /docs/reference/syscalls/interrupt_wait.md
