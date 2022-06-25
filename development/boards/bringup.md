# Adding support for new boards

## Overview

In Fuchsia, hardware support can be divided per architecture, board and drivers, see [RFC-0111:
Initial Fuchsia hardware platform specifications][rfc-0111]. Adding support for a new architecture
is not described on this page. This page describes adding support for a new board.

Note that the x64 architecture is considered in itself a board (highly configurable one)
hence it is not described here since it is already supported (although its implementation can be
improved and support for more configurations that don’t currently work added).

To add support for a new board, first we need to add support for the
[bringup](https://fuchsia.googlesource.com/fuchsia/+/main/products/bringup.gni) build
[product](https://fuchsia.googlesource.com/fuchsia/+/main/products/README.md) configuration.

## Add bringup build configuration support

To get a bringup build working, you need:

1. [A supported architecture](#supported-architecture)
1. [Bootloader and Kernel support](#bootloader-kernel)
1. [Build assembly](#build-assembly)
1. [A board driver defined](#board-driver)
1. [Non-kernel drivers](#non-kernel-drivers)

### Supported architectures {#supported-architecture}

See [RFC-0111: Initial Fuchsia hardware platform specifications][rfc-0111] for the architecture
support process.

### Bootloader and kernel support {#bootloader-kernel}

A new board requires a way to load the kernel, ideally with a bootloader supporting Fuchsia. Also
the kernel requires support for at least an early stage debugging mechanism (e.g. serial port),
interrupt controller and timers, this in the form of kernel drivers.

The board must include a mechanism to program it (e.g. store contents in flash). This is needed to
make updates to the bootloader. The Fuchsia image could be programmed with this mechanism or with
support provided by the bootloader.

#### Bootloader

In this document, the bootloader refers to the final stage bootloader, which is the software component of the boot process that loads the Fuchsia kernel.

There are multiple configurations that would allow a bootloader to load a Fuchsia kernel:

*  The bootloader supports Fuchsia: Adding support for Fuchsia in the bootloader is the best
   option. In this case the Zircon Boot Image (ZBI)
   [specification](https://fuchsia.googlesource.com/fuchsia/+/main/zircon/system/public/zircon/boot/image.h)
   is implemented by the [Firmware
   SDK](https://fuchsia.googlesource.com/fuchsia/+/main/src/firmware/README.md). An example of this
   is the [support added](https://third-party-mirror.googlesource.com/u-boot/+log/refs/heads/vim3)
   to U-Boot for the VIM3.
*  If the bootloader does not support Fuchsia, then a boot shim mechanism has to be added to the
   kernel making the Fuchsia build compatible with the bootloader boot facilities (e.g. fastboot).
   Example for the MediaTek 8167 board
   [here](https://fuchsia.googlesource.com/fuchsia/+/291c26919aa033c472717699cc957b5c8c138bca%5E%21/#F3).
   This boot shim will specify all the drivers in the following items. Note that boot shims must also
   be listed
   [here](https://fuchsia.googlesource.com/fuchsia/+/737dd45d7674dfa8941a865710d2ac2c72aa336c/zircon/kernel/target/arm64/boot-shim/BUILD.gn#118).

Notes:

1. Partition mapping needs to provide big enough partitions to account for Fuchsia’s requirements.
1. Reserved resources between the bootloader, the Fuchsia kernel and any secure side software need
   to be accounted for, in particular reserved memory regions and reserved HW blocks. For instance
   some hardware may be configured by the bootloader and may be under control of code running at
   EL3/EL2. Manipulating such devices from EL1/0 through drivers may result unstable behavior. Be
   sure to understand what hardware/peripherals may be in use by secure monitors or hypervisors.

#### Kernel drivers

In Fuchsia most drivers are in user space, however the ones listed in this section must be present
in the kernel.

Some early stage debugging mechanism is required for bringup of a new board, for instance serial
port or JTAG. If serial port is the mechanism then a UART driver must be added if not already
available in the kernel, for instance
[here](https://fuchsia.googlesource.com/fuchsia/+/main/zircon/kernel/dev/uart/dw8250). ARM’s interrupt
controller and timer support are also added with drivers, for instance
[gicv3](https://fuchsia.googlesource.com/fuchsia/+/main/zircon/kernel/dev/interrupt/arm_gic/v3/) and
[generic timer](https://fuchsia.googlesource.com/fuchsia/+/main/zircon/kernel/dev/timer/arm_generic/).

All these drivers need to be configured to be used by the kernel. If the kernel was booted by a
bootloader that supports fuchsia, the drivers will be configured in the ZBI, for example in U-Boot
[here](https://third-party-mirror.googlesource.com/u-boot/+/0f7b78a526a42235d0edfcfd17290c545b5d80c3/board/khadas/kvim3/zircon.c#551). If
the kernel was booted with a boot shim, the drivers will be configured in the shim itself, for
instance for the MediaTek 8167 board
[here](https://fuchsia.googlesource.com/fuchsia/+/5ddb969fbe644c34c7391e58733e50e2f16cc3f6/zircon/kernel/target/arm64/board/mt8167s_ref/boot-shim-config.h#115).

Notes:

1. Additional drivers may be added for instance to control power with PSCI.

### Build assembly {#build-assembly}

In order to add support for a new board, a new
[board](https://fuchsia.googlesource.com/fuchsia/+/main/boards/) configuration needs to be added to
the build system (gn). For instance for
[VIM3](https://fuchsia.googlesource.com/fuchsia/+/main/boards/vim3.gni) the `board_bootfs_labels` gn variable
(defined [here](https://cs.opensource.google/fuchsia/fuchsia/+/main:build/board.gni)) defines what
gets loaded in a bringup (into [bootfs][glossary.bootfs]) build including the
non-kernel drivers described below.

With the addition of the board configuration it is possible to instruct the build system to create
an image for the new board. For example: `fx set bringup.vim3` followed by `fx build` (see [fx
workflows][fx]).

With this in place a bringup build for the new board can be loaded into the target and we can get a
shell in the serial port (if the kernel serial port driver was added). The next step is to add a
board driver and its corresponding non-kernel drivers for additional functionality.

### Board driver {#board-driver}

With kernel support added for the board and build system assembly in place, a board driver (examples
[here](https://fuchsia.googlesource.com/fuchsia/+/main/src/devices/board/drivers/)) must be created
to, at runtime, add and configure the non-kernel drivers to be used. The board driver describes and
configures a board’s HW that can’t be located/configured by probing, which is common in ARM64 based
systems.

Notes:

1. Configuration of drivers must be done in the board driver not in the actual driver to allow for
   reuse and modularity in assembly.

### Non-Kernel drivers {#non-kernel-drivers}

Regular non-kernel [drivers][glossary.driver] can then be added to support the board, for example
to support GPIOs, interrupts, clocks, power, storage devices, audio devices, displays, GPUs,
etc. For more information see [here][driver-development].

Notes:

1. Sometimes HW is purposely initialized in the bootloader for instance to display a logo. This
   needs to be accounted for in the regular drivers. Information on this could be passed around with
   kernel arguments in the ZBI.
1. Reuse SW by creating libraries that can be shared across drivers or by making drivers highly
   customizable from board drivers avoiding duplication.
1. Name drivers and variables using families, not specific SoCs to allow for more clear reuse and
   aggregation.
1. Don’t make assumptions about the state of the HW as left by the bootloader (except for well known
   cases as in item 1). HW may or may not be in a reset state.
    1. Specifically make sure that all GPIOs are in a safe state.
    1. Specifically account for slow configurations that the bootloader may have left the hardware
       in for DDR, DVFS, display, etc.

## Add core build configuration support

Once the bringup builds work, you need to add support for the
[core](https://fuchsia.googlesource.com/fuchsia/+/main/products/core.gni) build
[product](https://fuchsia.googlesource.com/fuchsia/+/main/products/README.md) configuration to
create a self-updating system with core system services, connectivity, and metrics reporting.

In order to enable a core build you need to have a working network stack given that this is how the
core build allows for system debugging (for instance with `fx log` and `fx shell`), updates (for
instance with `fx ota`), and metrics reporting (for instance with `fx snapshot`).

## Add full system configuration support

Once core builds work, a full system can be integrated by defining a new product configuration. For
instance for x64 we already define
[workstation](https://fuchsia.googlesource.com/fuchsia/+/main/products/workstation.gni) which
includes a web browser and many other features. Similarly new product configurations can be added
for other architectures, for instance for ARM64 based systems.

<!-- xrefs -->
[glossary.bootfs]: glossary#bootfs
[glossary.driver]: glossary#driver
[fx]: development/build/fx.md
[driver-development]: development/drivers/developer_guide/driver-development.md
[rfc-0111]: contribute/governance/rfcs/0111_fuchsia_hardware_specifications.md
