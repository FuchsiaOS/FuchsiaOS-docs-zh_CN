<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0130" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Summary

This RFC lists the hardware currently in the Fuchsia project's **Supported
category**. The **Supported category** is defined in [RFC-0111: Initial Fuchsia
hardware platform specifications][rfc-0111].


## Motivation

The Fuchsia project needs to have an official list of hardware in the
**Supported category**, to ensure that:

* A specific level of support is provided, e.g. testing is done by the Fuchsia
  project.
* There is a source of truth about what specific configurations are in the
  **Supported category**.

## Stakeholders

_Facilitator:_ cpu.

_Reviewers:_ nicoh (Infra), nmulcahey (Infra), smithdave (Developer Relations),
nickvander (Developer Relations) and curtisgalloway (Drivers).

_Socialization:_ A draft of this RFC was sent to the Infra and TQ-Drivers
mailing lists.

## Design

Now that the Fuchsia project has defined the **Supported category** in
[RFC-0111: Initial Fuchsia hardware platform specifications][rfc-0111], we can
now list such hardware in a central database. Rather than listing individual
hardware blocks being in the **Supported category**, the Fuchsia project will
provide support as defined by [RFC-0111: Initial Fuchsia hardware platform
specifications][rfc-0111] and create a list (from here on referred to as
'central database') for specific combinations of hardware (from here on referred
to as 'system configurations'). Individual hardware blocks in a system
configuration are in the **Supported category** only in the context of such
configuration.

This document only refers to Fuchsia repositories at Top of Tree (latest
revision available), i.e. hardware in the **Supported category** and hosted by
the Fuchsia project.

Drivers for hardware in the **Supported category** are hosted by the Fuchsia
project. These drivers are tested as part of Fuchsia continuous integration and
the testing results can be seen by external contributors.

In Fuchsia, system configurations are defined by board drivers. Board drivers
start and configure all other drivers. Board drivers may define multiple system
configurations, for example the x64 board driver defines multiple PC system
configurations via ACPI (Advanced Configuration and Power Interface). To fully
define a system configuration the central database will list the board driver
and a description that provides details about the configuration like optional
hardware modules making it clear what is being tested in Fuchsia continuous
integration.

The system configurations listed in the central database do not include generic
external peripherals like any USB device, but rather list hardware blocks
integrated into a very specific system that is tested by the Fuchsia project.

The central database does not prevent future initiatives like hardware
certification labs, hardware self certification, hardware conformance testing,
etc.

The central database will be a YAML file in `reference/hardware`. YAML
makes it easy to process this data and present it in an easy to navigate format
similar to the one listing individual drivers at [Fuchsia hardware
drivers](reference/hardware/drivers.md).

The process for adding new entries to this YAML file, and hence to add new
hardware in the **Supported category**, is to create a new RFC. This is in
compliance with the RFC-0111 requirement for a new RFC in order to have any new
hardware added to the Fuchsia project **Supported category**.

## Implementation

The list of system configurations in the **Supported category** will be
maintained at `reference/hardware/_supported_sys_config.yaml`. The entries
in that YAML file will include:

* name: Provides a name or model.
* description: Provides details about the system configuration with enough
  description to fully define the exact system that is being tested by the
  Fuchsia project.
* Architecture: For instance ARM, x86 or RISC-V, optional.
* RAM: Amount and type of RAM, optional.
* storage: Amount and type of storage, optional.
* manufacturer_link: A link to the manufacturer, optional.
* board_driver_location: The location of the board driver that defines this
  board configuration. URL showing the repository and path within that
  repository.

Board drivers are uniquely identified by their repository and path within that
repository.

## System configurations epitaphs

For system configurations removed from the central database, a
`reference/hardware/_supported_sys_config_epitaphs.yaml` file will list
all system configuration removed from the **Supported category**. This file will
include the same YAML entries in
`reference/hardware/_supported_sys_config.yaml`, plus:

* `gerrit_change_id`: The ID of the Gerrit change used to delete the system
  configuration from _supported_sys_config.yaml and hence from the **Supported
  category**.
* `available_in_git`: The last known git SHA that still includes the system
  configuration _supported_sys_config.yaml and hence in the **Supported
  category**.

## Initial _supported_sys_config.yaml list

```
- name: 'VIM3'
  description: 'Board computer based on a Amlogic A311D SoC'
  Architecture: 'ARM'
  RAM: '2GB+ DDR4'
  storage: '16GB+ eMMC5.1'
  manufacturer_link: ‘https://www.khadas.com/vim3'
  board_driver_location: https://fuchsia.googlesource.com/fuchsia/+/refs/heads/main/src/devices/board/drivers/vim3'
- name: 'NUC7i5DNHE'
  description: 'Intel NUC based on 7th Gen i5'
  Architecture: 'x86'
  RAM: '4GB+ DDR4'
  storage: 'Samsung 860 EVO SSD 250GB'
  manufacturer_link: ‘https://www.intel.com/content/www/us/en/products/sku/122488/intel-nuc-kit-nuc7i5dnhe/specifications.html'
  board_driver_location: 'https://fuchsia.googlesource.com/fuchsia/+/refs/heads/main/src/devices/board/drivers/x86'
- name: 'NUC11TNHi5'
  description: 'Intel NUC based on 11th Gen i5'
  Architecture: 'x86'
  RAM: '4GB+ DDR4'
  storage: '128GB NVMe PCIe M.2 SSD'
  manufacturer_link: 'https://www.intel.com/content/www/us/en/products/sku/205594/intel-nuc-11-pro-kit-nuc11tnhi5/specifications.html'
  board_driver_location: 'https://fuchsia.googlesource.com/fuchsia/+/refs/heads/main/src/devices/board/drivers/x86'
- name: 'Astro'
  description: 'Based on the AMLogic S509D2G SoC, a complete system used for IoT applications'
  Architecture: 'ARM'
  board_driver_location: 'https://fuchsia.googlesource.com/fuchsia/+/refs/heads/main/src/devices/board/drivers/astro'
```

## Reverts

No changes should break hardware in the **Supported category** and hence a CL
that does break any of this hardware even after passing continuous integration
tests may be reverted by any member of the Fuchsia team, for instance build
gardeners.

## Backwards Compatibility

The initial list will include system configurations that are already in the
**Supported category**. As listed above this list includes VIM3, specific NUCs,
and Astro.

## Documentation

We will create a fuchsia.dev page similar to
[fuchsia.dev/fuchsia-src/reference/hardware/drivers](reference/hardware/drivers.md)
in fuchsia.dev/fuchsia-src/reference/hardware/supported. Other pages that
specify hardware in the **Supported category** will point to this new page, for
instance [Install Fuchsia on a NUC](development/hardware/intel_nuc.md)
will be updated to point to this new page.

<!-- xrefs -->
[rfc-0111]: contribute/governance/rfcs/0111_fuchsia_hardware_specifications.md
