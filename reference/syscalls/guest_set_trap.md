# zx_guest_set_trap

## NAME

<!-- Updated by update-docs-from-fidl, do not edit. -->

Sets a trap within a guest.

## SYNOPSIS

<!-- Updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_guest_set_trap(zx_handle_t handle,
                              uint32_t kind,
                              zx_vaddr_t addr,
                              size_t size,
                              zx_handle_t port_handle,
                              uint64_t key);
```

## DESCRIPTION

`zx_guest_set_trap()` sets a trap within a guest, which generates a packet when
there is an access by a VCPU within the address range defined by *addr* and
*size*, within the address space defined by *kind*.

*kind* may be either **ZX_GUEST_TRAP_BELL**, **ZX_GUEST_TRAP_MEM**, or
**ZX_GUEST_TRAP_IO**. If **ZX_GUEST_TRAP_BELL** or **ZX_GUEST_TRAP_MEM** is
specified, then *addr* and *size* must both be page-aligned.
**ZX_GUEST_TRAP_BELL** is an asynchronous trap, and both **ZX_GUEST_TRAP_MEM**
and **ZX_GUEST_TRAP_IO** are synchronous traps.

Packets for synchronous traps will be delivered through [`zx_vcpu_resume()`] and
packets for asynchronous traps will be delivered through *port_handle*.

*port_handle* must be **ZX_HANDLE_INVALID** for synchronous traps. For
asynchronous traps *port_handle* must be valid and a packet for the trap will be
delivered through *port_handle* each time the trap is triggered. A fixed number
of packets are pre-allocated per trap. If all the packets are exhausted,
execution of the VCPU that caused the trap will be paused. When at least one
packet is dequeued, execution of the VCPU will resume. To dequeue a packet from
*port_handle*, use [`zx_port_wait()`]. Multiple threads may use
[`zx_port_wait()`] to dequeue packets, enabling the use of a thread pool to
handle traps.

*key* is used to set the key field within `zx_port_packet_t`, and can be used to
distinguish between packets for different traps.


**ZX_GUEST_TRAP_BELL** is a type of trap that defines a door-bell. If there is
an access to the memory region specified by the trap, then a packet is generated
that does not fetch the instruction associated with the access. The packet will
then be delivered asynchronously via *port_handle*.

To identify what *kind* of trap generated a packet, use
**ZX_PKT_TYPE_GUEST_MEM**, **ZX_PKT_TYPE_GUEST_IO**, **ZX_PKT_TYPE_GUEST_BELL**,
and **ZX_PKT_TYPE_GUEST_VCPU**. **ZX_PKT_TYPE_GUEST_VCPU** is a special packet,
not caused by a trap, that indicates that the guest requested to start an
additional VCPU.

## RIGHTS

<!-- Updated by update-docs-from-fidl, do not edit. -->

*handle* must be of type **ZX_OBJ_TYPE_GUEST** and have **ZX_RIGHT_WRITE**.

*port_handle* must be of type **ZX_OBJ_TYPE_PORT** and have **ZX_RIGHT_WRITE**.

## RETURN VALUE

`zx_guest_set_trap()` returns **ZX_OK** on success. On failure, an error value is
returned.

## ERRORS

**ZX_ERR_ACCESS_DENIED** *handle* or *port_handle* do not have the
**ZX_RIGHT_WRITE** right.

**ZX_ERR_ALREADY_EXISTS** A trap with the same *kind* and *addr* already exists.

**ZX_ERR_BAD_HANDLE** *handle* or *port_handle* are invalid handles.

**ZX_ERR_INVALID_ARGS** *kind* is not a valid address space, *addr* or *size*
do not meet the requirements of *kind*, *size* is 0, or **ZX_GUEST_TRAP_MEM** was
specified with a *port_handle*.

**ZX_ERR_NO_MEMORY**  Failure due to lack of memory.
There is no good way for userspace to handle this (unlikely) error.
In a future build this error will no longer occur.

**ZX_ERR_OUT_OF_RANGE** The region specified by *addr* and *size* is outside of
of the valid bounds of the address space *kind*.

**ZX_ERR_WRONG_TYPE** *handle* is not a handle to a guest, or *port_handle* is
not a handle to a port.

## NOTES

**ZX_GUEST_TRAP_BELL** shares the same address space as **ZX_GUEST_TRAP_MEM**.

On x86-64, if *kind* is **ZX_GUEST_TRAP_BELL** or **ZX_GUEST_TRAP_MEM** and *addr*
is the address of the local APIC, then *size* must be equivalent to the size of
a page. This is due to a special page being mapped when a trap is requested at the
address of the local APIC. This allows us to take advantage of hardware
acceleration when available.

## SEE ALSO

 - [`zx_guest_create()`]
 - [`zx_port_create()`]
 - [`zx_port_wait()`]
 - [`zx_vcpu_create()`]
 - [`zx_vcpu_interrupt()`]
 - [`zx_vcpu_read_state()`]
 - [`zx_vcpu_resume()`]
 - [`zx_vcpu_write_state()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_guest_create()`]: guest_create.md
[`zx_port_create()`]: port_create.md
[`zx_port_wait()`]: port_wait.md
[`zx_vcpu_create()`]: vcpu_create.md
[`zx_vcpu_interrupt()`]: vcpu_interrupt.md
[`zx_vcpu_read_state()`]: vcpu_read_state.md
[`zx_vcpu_resume()`]: vcpu_resume.md
[`zx_vcpu_write_state()`]: vcpu_write_state.md
