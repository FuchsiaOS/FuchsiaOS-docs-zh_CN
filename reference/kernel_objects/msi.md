# MSI

## NAME

msi - Message Signaled Interrupt

## SYNOPSIS

Messaged Signaled Interrupts are used in modern PCI as well as
some ARM GIC controllers. MSI objects allow a privileged userspace
process to allocate a range of MSIs and associated IRQ vectors for
use with device drivers to allocate [`interrupt`] objects.

## DESCRIPTION

The most common use for an MSI object is to allocate a range of MSIs
to provide to a PCI device's Message Signaled Interrupt Capability.
Subsequently, the platform or PCI bus driver may use this object to
allocate [`interrupt`] objects corresponding to those MSIs for use
in downstream device drivers.

## SYSCALLS

 - [`zx_msi_allocate()`] - allocate a range of message-signaled interrupts
 - [`zx_msi_create()`] - create a message-signaled interrupt object

[`interrupt`]: /reference/kernel_objects/interrupts.md
[`zx_msi_allocate()`]: /reference/syscalls/msi_allocate.md
[`zx_msi_create()`]: /reference/syscalls/msi_create.md
