{% set rfcid = "RFC-0081" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- *** This should begin with an H2 element (for example, ##Summary).  -->

## Summary

This document proposes deprecating RAM loading via Zedboot's
netsvc. Instead, it proposes replacing the netsvc based RAM loading
flows with flows based on `fastboot boot`.

## Motivation

Without netsvc RAM loading we can avoid the duplication of this
functionality between Zedboot and bootloaders. This RFC is inline with
RFC [Deprecate Zedboot-based paving for provisioning
devices](/contribute/governance/rfcs/0075_deprecate_zedboot_paving.md),
converging around the use of fastboot instead of Zedboot, allowing for
the eventual deprecation and removal of Zedboot.

## Background

Netsvc RAM loading is commonly used for bringup workflows using
Zedboot. However bootloaders supporting `fastboot boot` already
provide this functionality, e.g. RAM loading a build deployed to a
device via the network, and also provide RAM loading via USB. Zedboot
was initially developed as a simple networking alternative to using
bootloaders because UEFI bootloaders vary in quality and in some
fastboot would not be reliable enough. This is less of a problem for
targets currently supported by Fuchsia with good enough support or
support in the works
(e.g. [fxbug.dev/59695](https://fxbug.dev/59695)).  Platforms where
fastboot is not available have the backup option of flashing Fuchsia
to a USB device and booting from it.

## Design

Fastboot is a mature mechansim and in use in Android
devices. Bootloaders that support Fuchsia MUST follow the [fastboot
protocol](https://android.googlesource.com/platform/system/core/+/refs/heads/master/fastboot).

## Implementation

* Bootloaders will need to be updated for example, Gigaboot will need
  to be modified to implement `fastboot boot`
  [fxbug.dev/59695](https://fxbug.dev/59695).
* fx scripts will need to be updated to redirect `fx netboot` users to
  `fastboot boot` (we likely want to wrap these flows in an fx/ffx
  helper command to make it as straightforward as possible).
* Infra will need to be transitioned to fastboot
  [fxbug.dev/47531](https://fxbug.dev/47531).
* Zedboot deprecation as described in RFC [Deprecate Zedboot-based
  paving for provisioning
  devices](/contribute/governance/rfcs/0075_deprecate_zedboot_paving.md).

## Backwards Compatibility

Once fx scripts are updated and Zedboot is removed, there will be no
backward compatibility with netsvc.

## Security and Privacy considerations

`fastboot boot` will be supported in unlocked "dev" bootloaders.
`fastboot boot` support for locked bootloaders and unlocked "prod"
bootloaders is under consideration but not defined in this RFC.

## Documentation

Documentation for developer workflows will need to be updated to
reflect the new `fasboot boot` based flows.

## Drawbacks, alternatives, and unknowns

A drawback is that this creates a commitment to support fastboot in
other Fuchsia bootloaders besides those that already support it. This
means, as new boards are brought up in Fuchsia, fastboot support is
required during early bringup. This includes support for `fastboot
boot` on Gigaboot bootloaders used on Intel NUCs
([fxbug.dev/59695](https://fxbug.dev/59695)) and on coreboot based
systems like Pixelbooks.

## Prior art and references

Fastboot is a mature mechansim and in use in Android devices.
