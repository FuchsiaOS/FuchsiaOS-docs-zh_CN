# zx_vmar_protect

## NAME

<!-- Updated by update-docs-from-fidl, do not edit. -->

Set protection of virtual memory pages.

## SYNOPSIS

<!-- Updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_vmar_protect(zx_handle_t handle,
                            zx_vm_option_t options,
                            zx_vaddr_t addr,
                            size_t len);
```

## DESCRIPTION

`zx_vmar_protect()` alters the access protections for the memory mappings
in the range of *len* bytes starting from *addr*. The *options* argument should
be a bitwise-or of one or more of the following:

- **ZX_VM_PERM_READ**  Map as readable.  It is an error if *handle*
  does not have **ZX_VM_CAN_MAP_READ** permissions or *handle* does
  not have the **ZX_RIGHT_READ** right.  It is also an error if the VMO handle
  used to create the mapping did not have the **ZX_RIGHT_READ** right.
- **ZX_VM_PERM_WRITE**  Map as writable.  It is an error if *handle*
  does not have **ZX_VM_CAN_MAP_WRITE** permissions or *handle* does
  not have the **ZX_RIGHT_WRITE** right.  It is also an error if the VMO handle
  used to create the mapping did not have the **ZX_RIGHT_WRITE** right.
- **ZX_VM_PERM_EXECUTE**  Map as executable.  It is an error if *handle*
  does not have **ZX_VM_CAN_MAP_EXECUTE** permissions or *handle* does
  not have the **ZX_RIGHT_EXECUTE** right.  It is also an error if the VMO handle
  used to create the mapping did not have the **ZX_RIGHT_EXECUTE** right.

*len* must be page-aligned.

## RIGHTS

<!-- Updated by update-docs-from-fidl, do not edit. -->

If *options* & **ZX_VM_PERM_READ**, *handle* must be of type **ZX_OBJ_TYPE_VMAR** and have **ZX_RIGHT_READ**.

If *options* & **ZX_VM_PERM_WRITE**, *handle* must be of type **ZX_OBJ_TYPE_VMAR** and have **ZX_RIGHT_WRITE**.

If *options* & **ZX_VM_PERM_EXECUTE**, *handle* must be of type **ZX_OBJ_TYPE_VMAR** and have **ZX_RIGHT_EXECUTE**.

## RETURN VALUE

`zx_vmar_protect()` returns **ZX_OK** on success.

## ERRORS

**ZX_ERR_BAD_HANDLE**  *handle* is not a valid handle.

**ZX_ERR_WRONG_TYPE**  *handle* is not a VMAR handle.

**ZX_ERR_INVALID_ARGS**  *prot_flags* is an unsupported combination of flags
(e.g., **ZX_VM_PERM_WRITE** but not **ZX_VM_PERM_READ**), *addr* is
not page-aligned, *len* is 0, or some subrange of the requested range is
occupied by a subregion.

**ZX_ERR_NOT_FOUND**  Some subrange of the requested range is not mapped.

**ZX_ERR_ACCESS_DENIED**  *handle* does not have the proper rights for the
requested change, the original VMO handle used to create the mapping did not
have the rights for the requested change, or the VMAR itself does not allow
the requested change.

## NOTES

## SEE ALSO

 - [`zx_vmar_allocate()`]
 - [`zx_vmar_destroy()`]
 - [`zx_vmar_map()`]
 - [`zx_vmar_unmap()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_vmar_allocate()`]: vmar_allocate.md
[`zx_vmar_destroy()`]: vmar_destroy.md
[`zx_vmar_map()`]: vmar_map.md
[`zx_vmar_unmap()`]: vmar_unmap.md
