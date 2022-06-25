# Resource

## NAME

resource - Address space rights and accounting

## SYNOPSIS

A resource is an immutable object that is used to validate access to syscalls
that create objects backed by address space, or permit access to address space.
These include [vm objects](vm_object.md), [interrupts](interrupts.md), and x86
ioports.

## DESCRIPTION

Resources are used to gate access to specific regions of address space and are
required to create VMOs and IRQs, as well as accessing x86 ioports.

A resource object consists of a single resource *kind*, with *base* address and
*len* parameters that define a range of address space the holder of the resource
is granted access to. The range covers *base* up to but not including *base* +
*len*.  These objects are immutable after creation. Valid *kind*  values are
**ZX_RSRC_KIND_ROOT**, **ZX_RSRC_KIND_MMIO**, **ZX_RSRC_KIND_IOPORT**,
**ZX_RSRC_KIND_IRQ**, **ZX_RSRC_KIND_SMC**, and **ZX_RSRC_KIND_SYSTEM**.

The system resource is a special case that contains other resources, all of which have *len*
one. These resources each have their own base within the system resource. Valid *base*
values for the system resource are **ZX_RSRC_SYSTEM_HYPERVISOR_BASE**,
**ZX_RSRC_SYSTEM_VMEX_BASE**, **ZX_RSRC_SYSTEM_DEBUG_BASE**,**ZX_RSRC_SYSTEM_INFO_BASE**,
**ZX_RSRC_SYSTEM_CPU_BASE**, and **ZX_RSRC_SYSTEM_POWER_BASE**.

New resources may be created with an appropriate parent resource by calling
[`zx_resource_create()`]. An initial resource of each *kind* is created by the kernel
during boot and handed off to the first userspace process started by userboot.

Appropriate parent resources are the root resource, or a resource whose own range
from *base* to *base+len* contains the range requested for the new resource. The
*kind* of a parent resource must match the *kind* of the resource being created.
At this time, *exclusive* resources cannot be used to create new resources. After
creation there is no relation between the resource parent used and the new resource
created.

Resource allocations can be either *shared* or *exclusive*. A shared resource
grants the permission to access the given address space, but does not reserve
that address space exclusively for the owner of the resource. An exclusive
resource grants access to the region to only the holder of the exclusive
resource.  Exclusive and shared resource ranges may not overlap.

Resources are lifecycle tracked and upon the last handle being closed will be
freed. In the case of exclusive resources this means the given address range
will be released back to the allocator for the given *kind* of resource. Objects
created through a resource do not hold a reference to the resource and thus do
not keep it alive.

## NOTES

Resources are typically private to the DDK and platform bus drivers. Presently,
this means ACPI and platform bus hold the root resource respectively and hand
out more fine-grained resources to other drivers.

## SYSCALLS

 - [`zx_interrupt_create()`]
 - [`zx_ioports_request()`]
 - [`zx_resource_create()`]
 - [`zx_vmo_create_physical()`]

[`zx_interrupt_create()`]: reference/syscalls/interrupt_create.md
[`zx_ioports_request()`]: reference/syscalls/ioports_request.md
[`zx_resource_create()`]: reference/syscalls/resource_create.md
[`zx_vmo_create_physical()`]: reference/syscalls/vmo_create_physical.md
