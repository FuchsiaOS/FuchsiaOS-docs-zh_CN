# zx_system_get_page_size

## SUMMARY

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

Get the page size for the system.

## DECLARATION

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

uint32_t zx_system_get_page_size(void);
```

## DESCRIPTION

`zx_system_get_page_size()` returns the base memory page size of the system in
bytes. This number cannot change during a run of the system, only at boot time,
and is guaranteed to be an exact power of 2.

The page size represents the allocation and alignment granularity of VMOs in
`zx_vmo_create()` and the smallest unit that can be mapped via `zx_vmar_map()`.

For every architecture there are well defined minimum and maximum values,
`ZX_MIN_PAGE_SIZE` and `ZX_MAX_PAGE_SIZE`, that this will return.

| Architecture | `ZX_MIN_PAGE_SIZE` | `ZX_MAX_PAGE_SIZE` |
| ------------ | ------------------ | ------------------ |
| ARM          | 4KiB               | 64KiB              |
| X86-64       | 4KiB               | 2MiB               |

## RIGHTS

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

TODO(fxbug.dev/32253)

## RETURN VALUE

`zx_system_get_page_size()` returns the page size in bytes.

## ERRORS

`zx_system_get_page_size()` cannot fail.

## NOTES

## SEE ALSO

 - [`zx_vmar_map()`]
 - [`zx_vmo_create()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_vmar_map()`]: vmar_map.md
[`zx_vmo_create()`]: vmo_create.md
