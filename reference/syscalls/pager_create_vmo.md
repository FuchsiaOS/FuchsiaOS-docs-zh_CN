# zx_pager_create_vmo

## SUMMARY

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

Create a pager owned vmo.

## DECLARATION

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_pager_create_vmo(zx_handle_t pager,
                                uint32_t options,
                                zx_handle_t port,
                                uint64_t key,
                                uint64_t size,
                                zx_handle_t* out);
```

## DESCRIPTION

Creates a VMO owned by a pager object. *size* will be rounded up to the next page size
boundary, and *options* must be zero or any combination of the following flags:

**ZX_VMO_RESIZABLE** - if the VMO can be resized.

**ZX_VMO_TRAP_DIRTY** - if writes to clean pages in the VMO should be trapped by the kernel and
forwarded to the pager service for acknowledgement before proceeding with the write.

On success, the returned vmo has the same rights as a vmo created with [`zx_vmo_create()`], as well
as having the same behavior with respect to **ZX_VMO_ZERO_CHILDREN**. Syscalls that operate on VMOs
require an explicit flag to allow blocking IPC to the userspace pager service; beyond this, whether
or not a VMO is owned by a pager does not affect the semantics of syscalls.

TODO(stevend): Update differences after updates to cloning and decommit

Page requests will be delivered to *port* when certain conditions are met. Those packets will have
*type* set to **ZX_PKT_TYPE_PAGE_REQUEST** and *key* set to the value provided to
`zx_pager_create_vmo()`. The packet's union is of type `zx_packet_page_request_t`:

```
typedef struct zx_packet_page_request {
    uint16_t command;
    uint16_t flags;
    uint32_t reserved0;
    uint64_t offset;
    uint64_t length;
    uint64_t reserved1;
} zx_packet_page_request_t;
```

*offset* and *length* are always page-aligned. The value of any bits in *flags* for which flags
are not defined is unspecified - currently no flags are defined. The trigger and meaning of
the packet depends on *command*, which can take one of the following values:

**ZX_PAGER_VMO_READ**: Sent when an application accesses a non-resident page in a pager's VMO. The
pager service should populate the range [offset, offset + length) in the registered vmo with
[`zx_pager_supply_pages()`]. Supplying pages is an implicit positive acknowledgement of the request.

**ZX_PAGER_VMO_DIRTY**: Sent when an application writes to a resident clean page in a pager's VMO
created with the **ZX_VMO_TRAP_DIRTY** flag. The pager service should acknowledge that the range
[offset, offset + length) can be dirtied, allowing the write to proceed, with
[`zx_pager_op_range()`] **ZX_PAGER_OP_DIRTY**.

**ZX_PAGER_VMO_COMPLETE**: Sent when no more pager requests will be sent for the corresponding
VMO, either because of [`zx_pager_detach_vmo()`] or because no references to the VMO remain.

If *pager* is closed, then no more packets will be delivered to *port* (including no
**ZX_PAGER_VMO_COMPLETE** message). Furthermore, all future accesses will behave as if
[`zx_pager_detach_vmo()`] had been called.

## RIGHTS

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

*pager* must be of type **ZX_OBJ_TYPE_PAGER**.

*port* must be of type **ZX_OBJ_TYPE_PORT** and have **ZX_RIGHT_WRITE**.

## RETURN VALUE

`zx_pager_create_vmo()` returns ZX_OK on success, or one of the following error codes on failure.

## ERRORS

**ZX_ERR_INVALID_ARGS** *out* is an invalid pointer or NULL, or *options* is any value other than
0 or **ZX_VMO_RESIZABLE**.

**ZX_ERR_BAD_HANDLE** *pager* or *port* is not a valid handle.

**ZX_ERR_ACCESS_DENIED** *port* does not have **ZX_RIGHT_WRITE**.

**ZX_ERR_WRONG_TYPE** *pager* is not a pager handle or *port* is not a port handle.

**ZX_ERR_OUT_OF_RANGE** The requested size is larger than the maximum vmo size.

**ZX_ERR_NO_MEMORY**  Failure due to lack of memory.

## SEE ALSO

 - [`zx_pager_detach_vmo()`]
 - [`zx_pager_op_range()`]
 - [`zx_pager_supply_pages()`]
 - [`zx_port_wait()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_pager_detach_vmo()`]: pager_detach_vmo.md
[`zx_pager_op_range()`]: pager_op_range.md
[`zx_pager_supply_pages()`]: pager_supply_pages.md
[`zx_port_wait()`]: port_wait.md
[`zx_vmo_create()`]: vmo_create.md
