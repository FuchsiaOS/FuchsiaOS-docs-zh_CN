<!-- Generated with `fx rfc` -->
<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0191" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
{# Fuchsia RFCs use templates to display various fields from _rfcs.yaml. View the #}
{# fully rendered RFCs at https://fuchsia.dev/fuchsia-src/contribute/governance/rfcs #}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Summary

This document proposes the addition of GCE (Google Compute Engine) as a
supported system configuration by the Fuchsia project. This simply formalizes
our existing usage of GCE as a platform for testing Fuchsia, and codifies our
commitment to keep GCE working at ToT.

We should note, however, that while we do utilize this at scale, codifying its
usage as a supported Fuchsia platform will require that we maintain support for
ACPI and UEFI in the Zircon kernel. This is a relatively well-exercised path
today for x64, but less so for ARM64.

## Motivation

Fuchsia-on-GCE is already utilized by infrastructure to improve the scalability,
diversity, and performance of CI/CQ testing. Concretely, we already test the
following configurations:

* bringup.x64
* bringup.x64 on AMD
* bringup.arm64
* core.x64

Now that we are performing a significant amount of work on GCE, we want to
formalize this as supported hardware by the Fuchsia project by adding it to the
supported hardware YAML file in `/docs/reference/hardware`.
[RFC-0130: Supported Hardware](0130_supported_hardware.md) states that "The
process for adding new entries to this YAML file, and hence to add new hardware
in the Supported category, is to create a new RFC."

## Stakeholders

_Facilitator:_ rlb@

The person appointed by FEC to shepherd this RFC through the RFC process.

_Reviewers:_

-   cpu@google.com
-   curtisgalloway@google.com
-   tkilbourn@google.com
-   travisg@google.com

_Consulted:_

-   maniscalco@google.com
-   venkateshs@google.com
-   nmulcahey@google.com
-   andresoportus@google.com
-   simonshields@google.com

_Socialization:_

This RFC was socialized with all of Zircon and Engprod, and was then shared with
all of Fuchsia.

## Implementation

Adding GCE to the list of supported system configurations simply requires
editing the `/docs/reference/hardware/_supported.yaml` file. We will add two
entries to the file, one for x86 and one for ARM.

### x86 entry

*   name: 'GCE x86_64'
*   description: 'Google Compute Engine emulated x86 board'
*   architecture: 'x86'
*   manufacturer_link: 'https://cloud.google.com/compute'
*   board_driver_location: '/src/devices/board/drivers/x86'

### ARM entry

*   name: 'GCE ARM64'
*   description: 'Google Compute Engine emulated ARM64 board'
*   architecture: 'ARM'
*   manufacturer_link: 'https://cloud.google.com/compute'
*   board_driver_location: 'src/devices/board/drivers/acpi-arm64'

## Performance

Fuchsia on GCE is faster than running Fuchsia in an emulator on a Linux host.
This is especially true on x86, where we use nested virtualization, so running
directly on GCE removes 2 layers of virtualization. It is also only slightly
(~10%) slower than running on a physical NUC.

## Ergonomics

Infrastructure would like to reduce its dependence on inelastic hardware fleets
in presubmit testing, as these often increase pending times and have a
disproportionate impact on CI/CQ latency. Having a supported, representative,
and virtualized platform that we can scale easily and quickly will reduce this
dependency. This will, in turn, improve the developer experience by decreasing
the latency to submission.

We have also invested in tooling to improve the ergonomics of Fuchsia-on-GCE for
developers at their desk. Specifically, the build system already produces the
UEFI disk image needed to create a compute instance, and the `fx gce` commands
provide a simple way to create, connect to, and delete GCE VMs.

## Backwards Compatibility

Supporting GCE does not require any breaking changes within the platform.

## Testing

As mentioned earlier, infrastructure will run a variety of Fuchsia
configurations on GCE in CI/CQ to ensure that it is stable. We will also run in
the GCE staging environment to make sure that we get early notice of any cloud
changes that affect/break Fuchsia.

## Drawbacks, alternatives, and unknowns

The main drawback of supporting GCE is dealing with unexpected changes from
Google Compute that break Fuchsia. To avoid this situation, infra will run all
of the configurations we plan to run on GCE (bringup.x64, bringup.arm64,
core.x64, etc.) in the GCE staging environment. This environment receives
changes about 4 weeks before they enter production, meaning that we will be able
to detect breakages a month ahead of time. We can then use this time to either:

1. Work with cloud to find a solution, or
1. Modify Fuchsia to work with whatever new changes cloud released

It's worth noting that in the ~18 months we have been running on GCE, cloud has
launched only one feature that broke Fuchsia, so the rate of breakages should be
low.
