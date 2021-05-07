# zx_vmo_set_cache_policy

## NAME

<!-- Updated by update-docs-from-fidl, do not edit. -->

Set the caching policy for pages held by a VMO.

## SYNOPSIS

<!-- Updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_vmo_set_cache_policy(zx_handle_t handle, uint32_t cache_policy);
```

## DESCRIPTION

`zx_vmo_set_cache_policy()` sets caching policy for a VMO. Generally used on VMOs
that point directly at physical memory. Such VMOs are generally only handed to
userspace via bus protocol interfaces, so this syscall will typically only be
used by drivers dealing with device memory. This call can also be used on a
regular memory backed VMO with similar limitations and uses.

A handle must have the **ZX_RIGHT_MAP** right for this call to be
permitted. Additionally, the VMO must not presently be mapped by any process,
have any children, be a child itself, or have any pinned pages. Having committed
pages is only allowed if transitioning from a cached state, and the pages will be
cleaned and invalidated.

*cache_policy* cache flags to use:

**ZX_CACHE_POLICY_CACHED** - Use hardware caching. On Aarch64 this corresponds
to the Normal Memory, Outer Write-back non-transient Read and Write allocate, Inner
Write-back non-transient Read and Write allocate memory attributes

**ZX_CACHE_POLICY_UNCACHED** - Disable caching. On Aarch64 this corresponds to
the Device-nGnRnE memory attributes.

**ZX_CACHE_POLICY_UNCACHED_DEVICE** - Disable cache and treat as device memory.
This is architecture dependent and may be equivalent to
**ZX_CACHE_POLICY_UNCACHED** on some architectures. On Aarch64 this corresponds to
the Device-nGnRE memory attributes.

**ZX_CACHE_POLICY_WRITE_COMBINING** - Uncached with write combining. On Aarch64
this corresponds to the Normal memory, uncached memory attributes.


## RIGHTS

<!-- Updated by update-docs-from-fidl, do not edit. -->

*handle* must be of type **ZX_OBJ_TYPE_VMO** and have **ZX_RIGHT_MAP**.

## RETURN VALUE

`zx_vmo_set_cache_policy()` returns **ZX_OK** on success. In the event of
failure, a negative error value is returned.

## ERRORS

**ZX_ERR_ACCESS_DENIED** Cache policy has been configured for this VMO already and
may not be changed, or *handle* lacks the **ZX_RIGHT_MAP** right.

**ZX_ERR_BAD_HANDLE** *handle* is not a valid handle.

**ZX_ERR_INVALID_ARGS** *cache_policy* contains flags outside of the ones listed
above, or *cache_policy* contains an invalid mix of cache policy flags.

**ZX_ERR_NOT_SUPPORTED** The VMO *handle* corresponds to is not one holding
physical memory.

**ZX_ERR_BAD_STATE** Cache policy cannot be changed because the VMO is presently
mapped, has children, is a child itself, has pinned pages, or has committed pages
and the VMO is currently not cached.

## SEE ALSO

 - [`zx_vmo_create()`]
 - [`zx_vmo_get_size()`]
 - [`zx_vmo_op_range()`]
 - [`zx_vmo_read()`]
 - [`zx_vmo_set_size()`]
 - [`zx_vmo_write()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_vmo_create()`]: vmo_create.md
[`zx_vmo_get_size()`]: vmo_get_size.md
[`zx_vmo_op_range()`]: vmo_op_range.md
[`zx_vmo_read()`]: vmo_read.md
[`zx_vmo_set_size()`]: vmo_set_size.md
[`zx_vmo_write()`]: vmo_write.md
