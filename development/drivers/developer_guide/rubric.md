# Drivers rubric

Caution: This page may contain information that is specific to the legacy
version of the driver framework (DFv1). Also the workflows documented on
this page may only be specific to the Fuchsia source checkout
(`fuchsia.git`) environment.

## Overview

This document describes the rules for writing new drivers.

## Location

Many drivers are located under /src/devices in folders named
according to the driver type. For instance
[/src/devices/clock/drivers](/src/devices/clock/drivers),
[/src/devices/usb/drivers](/src/devices/usb/drivers), etc. Some
functional areas include their drivers within their own directories,
for instance [/src/media/audio/drivers](/src/media/audio/drivers),
[/src/graphics/drivers](/src/graphics/drivers), etc. New drivers must
be located alongside other drivers of the same type. If there is no
existing folder were a new driver should be logically placed under,
then a new folder needs to be added to [/src/devices](/src/devices),
named appropriately and include a drivers folder under it.

## OWNERS

As with any other code in Fuchsia an OWNER must approve the addition
of a new driver. The OWNERS file to check for approval depends on the
location where the driver is added.

## Driver Info

When adding a driver to the Fuchsia tree, a driver info file must be created, and the
`fuchsia_driver_component` target must include an `info = <some-driver-info.json>` entry. The JSON
file listed in `info` must include at least a `short_description` text and an `areas` list with
entries from [areas](/build/drivers/areas.txt). For example,
[aml-g12-tdm-dai-info.json](/src/media/audio/drivers/aml-g12-tdm/aml-g12-tdm-dai-info.json)
includes:

```json
{
    "short_description": "AMLogic g12 audio subsystem DAI TDM driver",
    "manufacturer": "AMLogic",
    "families": [
        "g12"
    ],
    "models": [
        "S905D2",
        "T931",
        "S905D3"
    ],
    "areas": [
        "Media",
        "Audio"
    ]
}
```

Note that this file automatically created by `fx create driver`, see [Creating a new
driver][creating-a-new-drider].


<!-- xrefs -->

[creating-a-new-drider]: /docs/development/drivers/developer_guide/driver-development.md#creating_a_new_driver
