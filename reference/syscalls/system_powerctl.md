# zx_system_powerctl

## SUMMARY

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

TODO(fxbug.dev/32938)

## DECLARATION

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_system_powerctl(zx_handle_t resource,
                               uint32_t cmd,
                               const zx_system_powerctl_arg_t* arg);
```

## DESCRIPTION

Changes the power state of the system based on the value of `arg`. Zircon may
not be able to put the system in a given power state, depending on its level of
support for the architecture being used.

If the architecture is fully support the following are the behaviors of the
given value:

* **ZX_SYSTEM_POWERCTL_ENABLE_ALL_CPUS** - set all processor cores as active
* **ZX_SYSTEM_POWERCTL_DISABLE_ALL_CPUS_BUT_PRIMARY** - set only the primary CPU
  as active
* **ZX_SYSTEM_POWERCTL_ACPI_TRANSITION_S_STATE** - TODO(fxbug.dev/32938)
* **ZX_SYSTEM_POWERCTL_X86_SET_PKG_PL1** - set CPU to power level 1 (x86 only)
* **ZX_SYSTEM_POWERCTL_REBOOT** - restart the system, control should go through
  any relevant firmware and bootloaders
* **ZX_SYSTEM_POWERCTL_REBOOT_BOOTLOADER** - restart the system, but stop in the
  bootloader instead of loading the operating system
* **ZX_SYSTEM_POWERCTL_REBOOT_RECOVERY** - restart the system, but load the
  recovery operating system instead of the primary OS
* **ZX_SYSTEM_POWERCTL_SHUTDOWN** - turn the system off
* **ZX_SYSTEM_POWERCTL_ACK_KERNEL_INITIATED_REBOOT** - used by userspace when it
  is ready for a reboot in response to a previous signal from the kernel that
  the kernel wanted to reboot the system

## RIGHTS

<!-- Contents of this heading updated by update-docs-from-fidl, do not edit. -->

*resource* must have resource kind **ZX_RSRC_KIND_SYSTEM** with base **ZX_RSRC_SYSTEM_POWER_BASE**.

## RETURN VALUE

**ZX_OK**

## ERRORS

Returns **ZX_ERR_BAD_STATE** if
**ZX_SYSTEM_POWERCTL_ACK_KERNEL_INITIATED_REBOOT** is supplied and the kernel
had not previously signaled a desire to reboot.

Returns **ZX_ERR_INVALID_ARGS** when an unrecognized `cmd` value is supplied.

## SEE ALSO


TODO(fxbug.dev/32938)
