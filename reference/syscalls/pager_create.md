# zx_pager_create

## NAME

<!-- Updated by update-docs-from-fidl, do not edit. -->

Create a new pager object.

## SYNOPSIS

<!-- Updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_pager_create(uint32_t options, zx_handle_t* out);
```

## DESCRIPTION

`zx_pager_create()` creates a new pager object.

When a pager object is destroyed, any accesses to its vmos that would have required communicating
with the pager will fail as if [`zx_pager_detach_vmo()`] had been called. Furthermore, the kernel
will make an effort to ensure that the faults happen as quickly as possible (e.g. by evicting
present pages), but the precise behavior is implementation dependent.

## RIGHTS

<!-- Updated by update-docs-from-fidl, do not edit. -->

None.

## RETURN VALUE

`zx_pager_create()` returns ZX_OK on success, or one of the following error codes on failure.

## ERRORS

**ZX_ERR_INVALID_ARGS** *out* is an invalid pointer or NULL or *options* is
any value other than 0.

**ZX_ERR_NO_MEMORY** Failure due to lack of memory.

## SEE ALSO

 - [`zx_pager_create_vmo()`]
 - [`zx_pager_detach_vmo()`]
 - [`zx_pager_supply_pages()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_pager_create_vmo()`]: pager_create_vmo.md
[`zx_pager_detach_vmo()`]: pager_detach_vmo.md
[`zx_pager_supply_pages()`]: pager_supply_pages.md
