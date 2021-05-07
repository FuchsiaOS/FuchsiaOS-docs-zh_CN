# zx_vmar_allocate

## NAME

<!-- Updated by update-docs-from-fidl, do not edit. -->

Allocate a new subregion.

## SYNOPSIS

<!-- Updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_vmar_allocate(zx_handle_t parent_vmar,
                             zx_vm_option_t options,
                             size_t offset,
                             size_t size,
                             zx_handle_t* child_vmar,
                             zx_vaddr_t* child_addr);
```

## DESCRIPTION

Creates a new VMAR within the one specified by *parent_vmar*.

*options* is a bit vector that contains one more of the following:

- **ZX_VM_COMPACT**  A hint to the kernel that allocations and mappings
  within the newly created subregion should be kept close together.   See the
  NOTES section below for discussion.
- **ZX_VM_SPECIFIC**  Use the *offset* to place the mapping, invalid if
  vmar does not have the **ZX_VM_CAN_MAP_SPECIFIC** permission.  *offset*
  is an offset relative to the base address of the parent region.  It is an error
  to specify an address range that overlaps with another VMAR or mapping.
- **ZX_VM_CAN_MAP_SPECIFIC**  The new VMAR can have subregions/mappings
  created with **ZX_VM_SPECIFIC**.  It is NOT an error if the parent does
  not have **ZX_VM_CAN_MAP_SPECIFIC** permissions.
- **ZX_VM_CAN_MAP_READ**  The new VMAR can contain readable mappings.
  It is an error if the parent does not have **ZX_VM_CAN_MAP_READ** permissions.
- **ZX_VM_CAN_MAP_WRITE**  The new VMAR can contain writable mappings.
  It is an error if the parent does not have **ZX_VM_CAN_MAP_WRITE** permissions.
- **ZX_VM_CAN_MAP_EXECUTE**  The new VMAR can contain executable mappings.
  It is an error if the parent does not have **ZX_VM_CAN_MAP_EXECUTE** permissions.

*offset* must be 0 if *options* does not have **ZX_VM_SPECIFIC** set.

In addition, the following power-of-two alignment flags can added:

- **ZX_VM_ALIGN_1KB** aligns *child_addr* to a power-of-2 at least 1K bytes.
- **ZX_VM_ALIGN_2KB** aligns *child_addr* to a power-of-2 at least 2K bytes.
- **ZX_VM_ALIGN_4KB** aligns *child_addr* to a power-of-2 at least 4K bytes.
- **ZX_VM_ALIGN_8KB** aligns *child_addr* to a power-of-2 at least 8K bytes.

and continues up to

- **ZX_VM_ALIGN_4GB** aligns *child_addr* to a power-of-2 at least 4G bytes.

Using **ZX_VM_ALIGN** flags with **ZX_VM_SPECIFIC** will fail if the
*parent_vmar* base address + *offset* are not aligned to the requested
value.

## RIGHTS

<!-- Updated by update-docs-from-fidl, do not edit. -->

If *options* & **ZX_VM_CAN_MAP_READ**, *parent_vmar* must be of type **ZX_OBJ_TYPE_VMAR** and have **ZX_RIGHT_READ**.

If *options* & **ZX_VM_CAN_MAP_WRITE**, *parent_vmar* must be of type **ZX_OBJ_TYPE_VMAR** and have **ZX_RIGHT_WRITE**.

If *options* & **ZX_VM_CAN_MAP_EXECUTE**, *parent_vmar* must be of type **ZX_OBJ_TYPE_VMAR** and have **ZX_RIGHT_EXECUTE**.

## RETURN VALUE

`zx_vmar_allocate()` returns **ZX_OK**, the absolute base address of the
subregion (via *child_addr*), and a handle to the new subregion (via
*child_vmar*) on success.  The base address will be page-aligned and non-zero.
In the event of failure, a negative error value is returned.

## ERRORS

**ZX_ERR_BAD_HANDLE**  *parent_vmar* is not a valid handle.

**ZX_ERR_WRONG_TYPE**  *parent_vmar* is not a VMAR handle.

**ZX_ERR_BAD_STATE**  *parent_vmar* refers to a destroyed VMAR.

**ZX_ERR_INVALID_ARGS**  *child_vmar* or *child_addr* are not valid, *offset* is
non-zero when **ZX_VM_SPECIFIC** is not given, *offset* and *size* describe
an unsatisfiable allocation due to exceeding the region bounds, *offset*
or *size* is not page-aligned, or *size* is 0.

**ZX_ERR_NO_MEMORY**  This may be due to the following:

* A free address range of the requested size is not available within
*parent_vmar*.
* The system is out of memory resources.

**ZX_ERR_ACCESS_DENIED**  Insufficient privileges to make the requested allocation.

## NOTES

### Deallocation

The address space occupied by a VMAR will remain allocated (within its
parent VMAR) until the VMAR is destroyed by calling [`zx_vmar_destroy()`].

Note that just closing the VMAR's handle does not deallocate the address
space occupied by the VMAR.

### The COMPACT flag

The kernel interprets this flag as a request to reduce sprawl in allocations.
While this does not necessitate reducing the absolute entropy of the allocated
addresses, there will potentially be a very high correlation between allocations.
This is a trade-off that the developer can make to increase locality of
allocations and reduce the number of page tables necessary, if they are willing
to have certain addresses be more correlated.

## SEE ALSO

 - [`zx_vmar_destroy()`]
 - [`zx_vmar_map()`]
 - [`zx_vmar_protect()`]
 - [`zx_vmar_unmap()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_vmar_destroy()`]: vmar_destroy.md
[`zx_vmar_map()`]: vmar_map.md
[`zx_vmar_protect()`]: vmar_protect.md
[`zx_vmar_unmap()`]: vmar_unmap.md
