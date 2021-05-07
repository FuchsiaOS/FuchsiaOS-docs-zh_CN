# zx_system_get_features

## NAME

<!-- Updated by update-docs-from-fidl, do not edit. -->

Get supported hardware capabilities.

## SYNOPSIS

<!-- Updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_system_get_features(uint32_t kind, uint32_t* features);
```

## DESCRIPTION

`zx_system_get_features()` populates *features* with a bit mask of
hardware-specific features.  *kind* indicates the specific type of features
to retrieve, e.g. **ZX_FEATURE_KIND_CPU**.  The supported kinds and the meaning
of individual feature bits is hardware-dependent.

## RIGHTS

<!-- Updated by update-docs-from-fidl, do not edit. -->

TODO(fxbug.dev/32253)

## RETURN VALUE

`zx_system_get_features()`  returns **ZX_OK** on success.

## ERRORS

**ZX_ERR_NOT_SUPPORTED**  The requested feature kind is not available on this
platform.

## NOTES
Refer to [Architecture Support](/docs/concepts/architecture/architecture_support.md) for supported
processor architectures.

Refer to [zircon/features.h](/zircon/system/public/zircon/features.h) for kinds
of features and individual feature bits.

## SEE ALSO

 - [`zx_system_get_num_cpus()`]
 - [`zx_system_get_physmem()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_system_get_num_cpus()`]: system_get_num_cpus.md
[`zx_system_get_physmem()`]: system_get_physmem.md
