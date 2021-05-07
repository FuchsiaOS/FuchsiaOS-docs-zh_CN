# zx_system_mexec

## NAME

<!-- Updated by update-docs-from-fidl, do not edit. -->

Soft reboot the system with a new kernel and bootimage.

## SYNOPSIS

<!-- Updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_system_mexec(zx_handle_t resource,
                            zx_handle_t kernel_vmo,
                            zx_handle_t bootimage_vmo);
```

## DESCRIPTION

`zx_system_mexec()` accepts two vmo handles: *kernel_vmo* should contain a
kernel image and *bootimage_vmo* should contain an initrd whose address shall
be passed to the new kernel as a kernel argument.

To supplant the running kernel, a *resource* of **ZX_RSRC_KIND_ROOT** must be
supplied.

Upon success, `zx_system_mexec()` shall supplant the currently running kernel
image with the kernel image contained within *kernel_vmo*, load the ramdisk
contained within *bootimage_vmo* to a location in physical memory and branch
directly into the new kernel while providing the address of the loaded initrd
to the new kernel.

To use the `zx_system_mexec()` function, you must specify
`kernel.enable-debugging-syscalls=true` on the kernel command line. Otherwise,
the function returns **ZX_ERR_NOT_SUPPORTED**.

## RIGHTS

<!-- Updated by update-docs-from-fidl, do not edit. -->

*resource* must have resource kind **ZX_RSRC_KIND_ROOT**.

*kernel_vmo* must be of type **ZX_OBJ_TYPE_VMO** and have **ZX_RIGHT_READ**.

*bootimage_vmo* must be of type **ZX_OBJ_TYPE_VMO** and have **ZX_RIGHT_READ**.

## RETURN VALUE

`zx_system_mexec()` shall not return upon success.

**ZX_ERR_NOT_SUPPORTED**  `kernel.enable-debugging-syscalls` is not set to `true`
on the kernel command line.

## SEE ALSO

 - [`zx_system_mexec_payload_get()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_system_mexec_payload_get()`]: system_mexec_payload_get.md
