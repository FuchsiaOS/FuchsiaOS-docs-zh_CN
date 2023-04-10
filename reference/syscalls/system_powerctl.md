<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_system_powerctl

## Summary

TODO(fxbug.dev/32938)

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_system_powerctl(zx_handle_t resource,
                               uint32_t cmd,
                               const zx_system_powerctl_arg_t* arg);
```

## Description

`zx_system_powerctl` changes the power state of the system based on the value of
*cmd*.  Zircon may not be able to put the system in a given power state,
depending on its level of support for the architecture being used.

*arg* is an optional pointer to a struct which provides further information
about the command to be executed.

## COMMANDS

### ZX_SYSTEM_POWERCTL_ENABLE_ALL_CPUS

Sets all processor cores as active.

*arg* type: **n/a**

### ZX_SYSTEM_POWERCTL_DISABLE_ALL_CPUS_BUT_PRIMARY

Set only the primary CPU as active.

*arg* type: **n/a**

### ZX_SYSTEM_POWERCTL_ACPI_TRANSITION_S_STATE

Only defined for x86-64.

Currently only transitions to the S3 state are supported.

Before calling this syscall the following steps should be taken:
1. Enter ACPICA noncontested mode
2. Shut down the secondary CPUs
3. Execute the `_PTS` control method
4. (Optional) Execute the `_SST` control method
5. Clear the ACPI wake status bit register
6. Disable all ACPI GPEs
7. Enabled all ACPI wakeup GPEs

*arg* type: `zx_system_powerctl_arg_t` with only the `acpi_transition_s_state`
union element considered valid.

```
struct {
    uint8_t target_s_state;  // Value between 1 and 5 indicating which S-state
    uint8_t sleep_type_a;    // Value from ACPI VM (SLP_TYPa)
    uint8_t sleep_type_b;    // Value from ACPI VM (SLP_TYPb)
} acpi_transition_s_state;
```

Returns **ZX_ERR_INVALID_ARGS** if the target S state is not in the range 1-5.

Returns **ZX_ERR_NOT_SUPPORTED** if transitioning to the target S state is not
supported.

Returns **ZX_ERR_BAD_STATE** if the target S state is not 5 and the secondary
CPUs have not been shut down.

Returns **ZX_ERR_NO_MEMORY** if there are not enough resources to run the
thread.

Returns **ZX_ERR_INTERNAL** if the S state transition fails.

### ZX_SYSTEM_POWERCTL_X86_SET_PKG_PL1

Only defined for x86-64.

Set CPU to power level 1.

*arg* type: `zx_system_powerctl_arg_t` with only the `x86_power_limit` union
element considered valid.

```
struct {
    uint32_t power_limit;  // PL1 value in milliwatts
    uint32_t time_window;  // PL1 time window in microseconds
    uint8_t clamp;         // PL1 clamping enable
    uint8_t enable;        // PL1 enable
    uint8_t padding2[2];
} x86_power_limit;
```

### ZX_SYSTEM_POWERCTL_REBOOT

Restart the system, control should go through any relevant firmware and
bootloaders.

*arg* type: **n/a**

### ZX_SYSTEM_POWERCTL_REBOOT_BOOTLOADER

Restart the system, but stop in the bootloader instead of loading the operating
system.

*arg* type: **n/a**

### ZX_SYSTEM_POWERCTL_REBOOT_RECOVERY

Restart the system, but load the recovery operating system instead of the
primary OS.

*arg* type: **n/a**

### ZX_SYSTEM_POWERCTL_SHUTDOWN

Turn the system off.

*arg* type: **n/a**

### ZX_SYSTEM_POWERCTL_ACK_KERNEL_INITIATED_REBOOT

Used by userspace when it is ready for a reboot in response to a previous signal
from the kernel that the kernel wanted to reboot the system.

*arg* type: **n/a**

Returns **ZX_ERR_BAD_STATE** if the kernel has not previously signaled a desire
to reboot.

## Rights

*resource* must have resource kind **ZX_RSRC_KIND_SYSTEM** with base **ZX_RSRC_SYSTEM_POWER_BASE**.

## Return value

**ZX_OK**

## Errors

Returns **ZX_ERR_INVALID_ARGS** when an unrecognized `cmd` value is supplied.

## See also

TODO(fxbug.dev/32938)
