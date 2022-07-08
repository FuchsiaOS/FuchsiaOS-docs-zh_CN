{% set rfcid = "RFC-0075" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!--
*** This should begin with an H2 element (for example, ## Summary).
-->

## Summary

This document proposes deprecating and eventually removing zedboot for
device provisioning flows. Instead, replacing the flow via the fastboot
based flashing, thereby improving stability and reliability of the device
provisioning process.

## Motivation

Zedboot based provisioning or 'paving' is commonly used to bootstrap a
Fuchsia device. Zedboot is actually an instance of the Zircon kernel with
a minimal set of drivers and services.

[Fastboot](https://android.googlesource.com/platform/system/core/+/master/fastboot/README.md)
is a mechanism for communicating with bootloaders over USB and Ethernet.
It is also used to provision a Fuchsia device. It is commonly referred
to as 'flashing' a device.

Note: For the remainder of the doc, 'paving' is in reference to zedboot,
'flashing' is in reference to fastboot.

Zedboot based paving workflows depend on a large amount of the Fuchsia system
to work properly, specifically:

 * the Kernel
 * the Driver stack
 * Volume Management (FVM)
 * a Network stack (netsvc)

These subsystems must be functioning before a paving workflow can be used for
provisioning a Fuchsia system. On the other hand, the fastboot protocol
is implemented directly in the bootloader and does not require any other
dependencies to be able to bootstrap and provision devices with Fuchsia.

Some of the advantages to using fastboot include:

 * Single step provisioning flows for devices since the device is only required
   to be in bootloader mode to initiate the flashing process.

   * For paving flows, zedboot needs to be present on the device
     partitions (either R, A, or B partitions). To get zedboot on the
     partition, a flashing flow needs to be used. Finally, zedboot is
     eventually overwritten after paving the Fuchsia system on to the
     device.

 * A boot server serving over UDP is not required to obtain the assets for
   provisioning. Only a set of prebuilt image assets are needed.
 * Compatibility and support across different versions and branches of Fuchsia
   platform. Avoiding the need for Large Scale Changes that impact developer and
   platform release processes.
 * Does not require careful build-time replacement of images to run in the
   `Recovery (R)` partition in Fuchsia devices.

Some of the issues with Zedboot based provisioning are as follows:

 * When FVM changes format such as documented in [RFC-0005 Blobfs snapshots](0005_blobfs_snapshots.md)
   , a corresponding roll-forward change to Zedboot is required. Devices
   are required to be bootstrapped with a different zedboot version to prevent
   users from overwriting device FVM partitions with an incompatible version.
 * When users switch between branches of Fuchsia, incompatible versions of
   drivers, blobfs, or fvm can unexpectedly fail for developers.
 * Zedboot is required to be flashed using fastboot to a partition on the
   device as a pre-requisite. Thus, the user is required to perform a flashing
   step before paving.
 * The Fuchsia surface area exposed via Zedboot exposes additional functionality
   that is not required for provisioning a device and thus deemed a security
   risk.

Simplifying the provisioning process around 'fastboot' has advantages to the
developer workflow, consolidates provisioning across all Fuchsia devices, and
unblocks changes in low-level areas of the platform such as the the storage
layer.

## Background

### Zedboot versioning

Paving requires Zedboot version compatibility between the Fuchsia device
and the Host which is executing the paving workflows, as well as FVM
format compatibility between the Fuchsia device and the images the Host
sends.

These versioning constraints are a significant pain point for developers
when FVM and zedboot versions are rolled, especially for developers which may
be switching between branches (which also implies the devices are frequently
switching between older and newer versions of FVM or Zedboot).

The primary issue arises when developers need to downgrade devices
across a Zedboot version boundary; in this case, the developer will need
to reinitialize Zedboot on the device first, which either requires
re-flashing the device or performing a two-step paving sequence that
involves a best-effort repaving of Zedboot itself, ignoring the version
mismatch. For the upgrade path, developers opt to use the standard system
over the air (OTA) update procedure.

### Fastboot versioning

Fastboot also has version compatibility requirements and as additional Fuchsia
images or files need to be written as part of the provisioning process the
bootloader in some cases must be updated to support the new image format.

However, this format is much simpler and more stable than changes to higher
level storage formats (e.g. FVM). Any changes to the bootloader, for example,
can be handled by an OTA or a flashing workflow via fastboot. In most
cases additional images can be added with no bootloader changes.

## Design

`fastboot` is officially supported in bootloaders in Fuchsia products. It
is used today for provisioning Fuchsia images onto devices.

## Implementation

After going through deprecation phase, Zedboot will be removed from
the provisioning workflows and from relevant documentation.

The implementation will follow a deprecation and then migration process.
Thus, various scripts and developer tooling that use paving will be:

 * marked as deprecated in the Fuchsia SDK.
 * updated to display a warning when using paving.
 * migrated to flashing tools that are already available in the SDK.
 * removed after deprecation period is over.

## Performance

No performance impact in the system.

## Security considerations

Deprecating and removing Zedboot reduces the overall area of concern for Security.
Primarily this is the broad attack surface and the level on control available
to the operator, including access to the underlying filesystem.

Fastboot has gone through the appropriate security approval process and is approved
for use in provisioning Fuchsia production devices.

## Privacy considerations

Like with security, removing Zedboot reduces the area of concern for
Privacy, and fastboot is approved for use with production devices.

## Testing

Fastboot based provisioning is used by developers in Fuchsia today. Appropriate
infrastructure in Fuchsia's CI/CQ systems will be requested to add support
for Fastboot based provisioning. This ensures the fastboot flows are tested
regularly.

## Documentation

Documentation for developer workflows will need to be updated to reflect the
new flashing based flows.

## Drawbacks, alternatives, and unknowns

### Drawbacks

The primary drawbacks are deprecating and migration of existing paving scripts
and workflows to use flashing.

There also needs to be a commitment to support fastboot in other Fuchsia
bootloaders besides U-Boot which has support already. This means, as new
boards are brought up in Fuchsia, fastboot support is required during
early bringup.

 * Support for Gigaboot bootloaders on the Intel NUC product is not fully complete
   but is in progress: [fxb/68692: NUC: support full device flashing](http://fxb/68692).

 * SeaBIOS is the default bootloader for qemu but there are no paving workflows
   supported in qemu. Thus, fastboot support is not required.

 * Coreboot is used for booting Fuchsia on Pixelbook devices. Paving is the
   only supported provisioning flow for Pixelbook devices. Fastboot can be
   supported via the use of the [depthcharge](https://chromium.googlesource.com/chromiumos/platform/depthcharge) payload [mechanism](https://www.chromium.org/chromium-os/developer-information-for-chrome-os-devices/custom-firmware#Depthcharge).

   Note: Pixelbook devices are not currently supported in the Fuchsia CI/CQ
   infrastructure.

### Other Notes and Considerations

The RFC is written with the assumption that the primary workflow is provisioning
devices with Fuchsia as the primary operating system. However, for general
purpose x64 systems such as the Intel NUC, Zedboot supports setting up partition
tables to enable this use case.

 * Fastboot supports writing GPT tables. However, the preferred approach here
   is using fastboot to write an installer or recovery payload and using that
   to perform initial bootstrapping of a Fuchsia system.

Zedboot is also used for the purposes of netbooting (booting devices from the
Network).

 * Fastboot has a `boot` command that supports local boot of images. This can be
   to be extended to support a netboot like functionality.
 * Or leveraging facilities in [depthcharge](https://chromium.googlesource.com/chromiumos/platform/depthcharge)
   to support netbooting on specific hardware targets.

The flashing operation wipes the FTL (Flash Translation Layer) state on the
device.

 * An ability to track FTL wear leveling data over time is useful information
   to certain teams. fastboot contains support for oem subcommands that allow
   certain product specific commands to be implemented. Exporting FTL information
   via fastboot oem can be an option here.

[`mexec`](/reference/syscalls/system_mexec.md) is a
flow that allows to soft reboot a system with a new kernel and bootimage. Moving
to a fastboot based provisioning flow does not impact this feature.

## Prior art and references

There is a large amount prior art on devices using Fastboot for provisioning.
There are two examples listed below:

### Android

Android devices rely completely on fastboot based flows as documented [here.](https://source.android.com/setup/build/running)

As Android devices go through various upgrades and changes, fastboot based
flashing and provisioning provides a consistent developer flow and experience
for bootstrapping systems and for restoring systems to factory state.

### Linaro

[Linaro](https://www.linaro.org/) is a consortium that funds and promotes
various projects around accelerating the product deployment in the Arm
ecosystem. Fastboot is the common protocol and method used in
provisioning linaro Linux firmwares across various Arm development and
prototype boards such as [96boards](https://www.96boards.org/).
