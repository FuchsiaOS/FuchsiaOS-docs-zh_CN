<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0103" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Summary
Criteria for changes which require RFCs in the Software Delivery area.

## Motivation

The [Software Delivery][swd] (SWD) system has wide scope over system behaviors,
including over the air (OTA) system updates, development and testing flows,
device security, and eventually package updates without a system update. Changes
to the SWD stack can modify behavior of the system in subtle ways and incur cost
to the Fuchsia program. To best adhere to the
[Fuchsia-wide RFC process][rfc-process], we seek to disambiguate the changes
which have 'broad impact' in the SWD area by putting forth a set of clear
criteria. These criteria are an attempt to achieve a balance between execution
velocity and proper stakeholder communication and approval processes.


## Design

### Changes which require an RFC

*   **Changes to the system update flow which add restrictions or gates, or new
    error conditions to the OTA process.** For example, adding additional checks
    before completing an OTA, or making OTA downgrade checking more strict. This
    is important because these changes have the potential to reduce the
    reliability of the OTA process.
*   **Changes to how we use package repositories, or the [structure of package
    repositories we expect][tuf-structure].** We expect that both internal and
    external developers may create or host their own TUF servers, and we should
    provide appropriate notice of changes to the expected format.
*   **Changes to the Fuchsia package format**. Anything that modifies the
    [schema for Fuchsia packages][package-schema] or the [Fuchsia Archive
    format][far-format]. This is important because many different tools interact
    with the package format and we should provide appropriate notice of changes.
*   **Adding or removing requirements for a product or build type to support OTA
    system updates**. For example, requiring vbmeta to OTA successfully, or
    removing a requirement for vbmeta. This is important because these changes
    may incur costs for product developers.
*   **Modifications to enforcement of security policies**. For example, changing
    executability restriction strategies. This is important because these
    changes have the potential to impact the security of Fuchsia.
*   **Changes which may make old systems fail to update to newer system
    versions, or require stepping-stone builds.** This is important because
    stepping-stone releases incur a long-term cost. When a device is set up for
    the first time, or reconnected after a long time, it must update through
    every stepping-stone release, costing additional time and network traffic
    for the end user. A bug in a stepping-stone release has the potential to
    impact the security or availability of devices moving through it even years
    after the original release, meaning that stepping-stone releases must be
    tested and maintained for significantly longer than regular releases.
*   **Changes which will substantially increase resource usage.** Whether
    memory, disk, CPU, network, etc. This gives broader visibility to changes in
    the use of shared device resources.
*   **Modifications to privacy policies or their enforcement.** This is
    important because these changes have the potential to impact the privacy
    Fuchsia offers to users. We don't have much ability to transact PII in the
    codebase today, but if we someday want to, that should go through an RFC.


### Examples: past changes which would now require an RFC

*   [OTA backstops][ota-backstops]; changes to the OTA process.
*   Post-OTA health checks; changes to the OTA process.
*   Changes to the package format, like a `meta/contents` overhaul.
*   Migrating to channel in vbmeta; changes to OTA process and requirements for
    OTAs.


### Examples: past changes which would still not require an RFC

*   CFv2 migrations; they are covered by other artifacts.
*   Blob download resumption; optional feature unobservable outside of
    pkg-resolver, risks were mitigated in code review process.
*   Changes to metrics or inspect emitted by the SWD stack.
*   Additions of channels for products.
*   Reliability improvements, like making `fx ota` work well for kernel
    developers. PSAs are sufficient here if the risk of regression is small.
*   Deprecating `amberctl` and moving developers over to `pkgctl`. This should
    be handled by the LSC process, as the workflows and goals of the tools are
    almost entirely the same.


## Implementation

Ongoing work that already has design approval through other processes and is in
implementation will not be _required_ to submit retroactive RFCs in order to go
ahead. However, we should submit retroactive RFCs for portions of the Software
Delivery stack which are sufficiently direction-setting and are under-documented
in other places. We'll evaluate ongoing or nearly-landed work on a
project-by-project basis to determine if we should write a retroactive RFC for
it.

Work which is not yet past the design stage will be required to adhere to
these guidelines.

These guidelines will be effective as of their RFC publication date, and we'll
add them to the [current process page][current-process] in the RFC
documentation. If we update these guidelines, we'll do so as an addendum to this
RFC in a subsequent CL.

If a contributor wishes to work with the SWD team on an RFC, they should feel
free to reach out to <pkg-dev@fuchsia.dev> and we'll assign them a designated
contact.

## Performance

No impact, process-only change.

## Ergonomics

We'll revisit these criteria if we find that changes are going through which
would have been better communicated by RFCs, or if we find in some other way we
need to modify these criteria. We need to strike a balance of execution velocity
and communication, regardless of the criteria.

## Backwards Compatibility

No impact.

## Security Considerations

Changes to the SWD area which modify security strategy will require an RFC,
per these criteria.

## Privacy considerations

Changes to the SWD area which modify privacy considerations will require an RFC,
per these criteria.

## Testing

No impact.

## Documentation

We'll update the [current RFC process documentation][current-process] if this
RFC is accepted.

## Drawbacks, alternatives, and unknowns

There are many alternative criteria we could use. We should amend this list if
we find that too many "small" changes require RFCs based on this criteria, or we
find that not enough "large" changes are going through the RFC process.

## Prior art and references

* [Zircon RFC criteria][zircon-criteria]
* [FIDL RFC criteria and process][fidl-criteria]

[current-process]: contribute/governance/rfcs/rfc_process.md
[far-format]: development/source_code/archive_format.md
[fidl-criteria]: 0049_fidl_tuning_process_evolution.md#criteria
[ota-backstops]: contribute/governance/rfcs/0071_ota_backstop.md
[package-schema]: concepts/packages/package.md#structure-of-a-package
[rfc-process]: contribute/governance/rfcs/0001_rfc_process.md
[swd]: contribute/governance/areas/README.md#software-delivery
[tuf-structure]: concepts/packages/software_update_system.md#merkle-root
[zircon-criteria]: 0006_addendum_to_rfc_process_for_zircon.md