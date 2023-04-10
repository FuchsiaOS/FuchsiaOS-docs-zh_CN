<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0150" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

## Summary

We describe a new ability for product owners to allow users of their products to
opt out of receiving software updates. We describe the policies associated with
this mechanism, the ways in which it should be used, and the security mechanisms
associated with storing the setting. Finally, we describe how other components
on the system may observe whether the system is opted out of updates.

## Motivation

We have a requirement from customers to allow users of Fuchsia devices to opt
out of receiving updates. We must satisfy that requirement in a way that is as
secure as possible for all users, regardless of their opt-out status.

## Stakeholders

_Facilitator:_

pascallouis@google.com

_Reviewers:_

* kevinwells@google.com - Software Delivery (SWD)
* ampearce@google.com - Security
* pascallouis@google.com - Fuchsia Engineering Council (FEC)

_Consulted:_

* borthakur@google.com
* marvinpaul@google.com - Server implementation
* gtsai@google.com - Server implementation
* enoharemaien@google.com - Privacy
* hjfreyer@google.com - Component Platform / FEC
* Software Delivery team

_Socialization:_

This RFC went through a design phase with the Software Delivery team, the
Security and Privacy teams, and customers.

## Requirements

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD",
"SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be
interpreted as described in
[IETF RFC 2119](https://tools.ietf.org/html/rfc2119).

* Product owners may allow users to opt out of updates, which means no updates
  are downloaded or installed to a user's device, except for critical
  updates designated by the product owner, including critical security updates.
* If a device is opted out of updates and has a Factory Data Reset (FDR) run, it
  should be opted back into updates.
* The setting must persist across reboots without having to be re-set on every
  boot by a product-level component (which would cause race conditions between
  the product-level component and update checks).
* The enablement and storage of the setting must be as secure as possible, to
  prevent an attacker finding a privilege escalation, opting the device out of
  updates, and persisting that vulnerability indefinitely.
* It must not be possible for someone to enable the opt-out option at runtime if
  the product owner has not decided to include it in a build (there must be a
  static compile-time flag to disable the feature entirely).
* We must have metrics on how many users (though not which specific users) are
  opted out of updates, to ensure that this option is not being exploited by
  attackers to persist vulnerabilities.
* If the system is in recovery mode, updates should always be allowed.
* If a user manually requests an update, even while updates are disabled, that
  update should be allowed.
* This feature should only be enabled on devices with tamper-resistant storage,
  such as Replay Protected Memory Blocks (RPMB). We have no requirements to
  enable this feature on devices without tamper-resistant storage, but we can
  revisit this RFC if the need arises in future.
    * _Note_: The proposed design does not limit us only to secure storage
      implementations, but removing tamper-resistant storage as a requirement
      would lead to security tradeoffs for the product owner.

## Design

`omaha-client` is our production system update checker. It communicates with an
Omaha server run by the product owner or a delegate. Together, the Omaha client
and server periodically negotiate a system or package version to install.

We propose that a product owner can enable the existence of this feature on a
product by statically building in a flag in the Software Delivery configuration.

### Reading and writing the opt-out option value

A new SWD component called `update-settings-storage` will serve a FIDL API
called `fuchsia.update.config.OptOut` to read this option and an API called
`fuchsia.update.config.OptOutAdmin` to write this option. This API will need to
be exposed in the SDK to allow product-level components to toggle the option's
value on and off.

The `fuchsia.update.config.OptOutAdmin` API must be strictly protected by
capability routing and Scrutiny verification, to ensure that no unauthorized
components gain access to it.

_Reading_ the value of the opt-out setting using
`fuchsia.update.config.OptOut` should be allowed to allowlisted components on
the system with looser controls than `OptOutAdmin`. This will allow components
at the system and product level to make decisions based on whether the device is
opted out of updates, and provide settings and troubleshooting views of this
option.

### Opt-out persistence and security
The component serving the `OptOut` API, `update-settings-storage`, must persist
the value of the opt-out setting across reboots, and must persist the setting in
integrity-protected storage. For instance, `minfs` storage of this
setting without an accompanying hash and signature on the storage would be
insecure, and is disallowed.

We propose using hardware tamper-resistant storage (currently the only option on
Fuchsia devices is RPMB) to store this opt-out setting. The property we desire
from tamper-resistant storage is that it cannot be written to except by a signed
Trusted Application, or rogue writes can be detected.

These storage APIs exist on the required [products][glossary.product] and should
be exposed through the Verified Execution Trusted Application (VX TA) which is
signed and authenticated via hardware measures at boot.

### Update Checks
On each scheduled update check `omaha-client` should read from
`update-settings-storage` using the `OptOut` FIDL API, and send the opt-out
value to Omaha using the existing [`updatedisabled`][omaha-updatedisabled] field
in the Omaha protocol.

If the system is running on the recovery partition, `updatedisabled` should
always be `false`. Similarly, if the update check was user-initiated,
`updatedisabled` should always be `false`.

If the Omaha server receives an update check with `updatedisabled` equal to
`true`, it should return `NoUpdate` for that Omaha Application's response,
except for critical updates designated by the product owner.

There is an alternative here, which is to not send any Omaha update checks at
all if the device is opted out of updates. However, that alternative denies the
product owner metrics on how many users are opted out of updates, and denies the
product owner the ability to push critical updates (by overriding
`updatedisabled` field if required, on the server).

The opt-out must apply to all applications for which `omaha-client` is checking
for updates, including system updates and single packages.

## Implementation

Implementation will take place between the SWD, firmware, security, and update
server teams. The task breakdown is roughly as follows, and should correspond to
CL chains to be submitted.

### Software Delivery

* Create `fuchsia.update.config.OptOut` FIDL API
* Implement `update-settings-storage` component
* Read `fuchsia.update.config.OptOut` from `omaha-client` at update check time,
  forward to Omaha server as `updatedisabled` flag (unless the device is in
  recovery or the check was user-initiated)
* Expose `fuchsia.update.config` in the SDK, and allowlist both protocols.
* Add a Cobalt metric to `omaha-client` to count the number of opted-out devices
* Expose the opt-out setting in Inspect data for `omaha-client`

### Firmware / Security

* Expose an API to RPMB via the Verified Execution Trusted Application (VX TA),
  statically include that API only on supported devices
* Modify the VX TA to clear the opt-out flag during Factory Data Reset (which
  it already mediates)

### Update Server

* Modify generation of Omaha rules to return `NoUpdate` for all Application
  Update Check Requests if `updatedisabled` is true for that Application.

## Performance

There should be no noticeable impact on system or update check performance.
Update checks are infrequent (on the order of hours), and not particularly
latency sensitive.

## Ergonomics

Our read API for this setting will help reduce possible developer or user
confusion, as will Inspect and Cobalt logging of the opt-out state.

## Backwards Compatibility

This RFC presents no backwards compatibility issues that we know of.

## Security considerations

Updates are Fuchsia's most important security feature. Without updates, the
Fuchsia team cannot patch vulnerabilities in the platform. More importantly,
having this code exist on the device at all is a risk to **all users** whether
or not they've opted out of updates: if an attacker gains sufficient privileges,
they can change the update opt-out setting and increase their chances of
persisting forever.

This design attempts to mitigate risk in the following ways:

*   Storing the setting in tamper-resistant memory (RPMB)
*   Only allowing a specific Verified Execution Trusted Application (TA) to
    modify the setting
    *   TAs are only accessible when a device is in locked mode with an
        authenticated user logged in (after OOBE)
    *   The VX TA is much smaller and simpler than, e.g., minfs, and is almost
        exclusively modified by the security and firmware teams
*   Auditing and allowlisting the routing of the TA
*   Retaining the ability to push critical updates (as defined by the product
    owner) to devices
    *   The criteria defining absolutely necessary updates are out of scope of
        this RFC
*   We will continually know how many devices are opted out and can create
    alerts if that number becomes suspicious

The following risks still exist:

*   We have no control over the product-level code which will call the opt-out
    API, and we must ensure that only highly-privileged components can access
    it
*   An attacker who has gained sufficient kernel privileges could still take
    control of `update-settings-storage` or the VX TA and ask them to modify the
    setting

The following security improvements are not scoped at this time, but could be
considered for future iterations:

*   Adding a physical presence requirement (user must interact with the device
    in some way, not just through a product-level component)
*   Making the setting writeable from the bootloader only. This defends the
    setting against a kernel compromise at the expense of UX and cross-platform
    consistency

## Privacy considerations

This design does not significantly impact user privacy, as all logging or
propagation of opt-out status will go through a privacy-protected logging
service: either our Crash database via Inspect, or Cobalt.

## Testing

We'll integration test the `update-settings-storage` component, its interaction
with `omaha-client`, and the final requests to an Omaha server that
`omaha-client` produces in various states of opt-out.

We'll integration test the RPMB API via the Verified Execution Trusted
Application.

We'll also unit test the implementations in each of the components individually.

Finally, we'll engage with the testing teams to ensure that products which
enable this feature have end to end testing of their entire integrations,
similar to other update features.

## Documentation

We'll need to add documentation to the new FIDL API, as well as to our [OTA flow
documentation][ota-flow].

## Drawbacks, alternatives, and unknowns

### 'Tombstone' builds
We could implement an update-opt out by pushing a 'tombstone' build to devices
which indicates that it will no longer update.

This has a notable upside:

* The opt-out setting would be stored in Verified Boot Metadata, which is
  immutable. This implies better security properties.

This also has some notable downsides:

* We'd need to generate a specific 'tombstone' build for every normal build we
  push, for every product which enables this feature.
* The actual application of the opt-out would require an OTA update, which
  violates our requirements.

### Store the opt-out value in mutable storage
We could avoid some complexity by storing the opt-out value in minfs, which
would allow this feature to be deployed to a greater range of products. However,
this makes it trivial for an attacker with privilege escalation to persist their
attacks indefinitely if they can gain write access to minfs. Gaining write
access to minfs is likely substantially easier than modifying the Verified
Execution Trusted Application's state, since the VX TA is much smaller than
minfs in terms of surface area, and easier to audit.

### If a user is opted out, omit update checks entirely
If a user is opted out, we could modify `omaha-client` to not check for updates
at all. This has a couple of drawbacks: we can't get Omaha-based metrics on how
many users are opted out, and a [product owner][glossary.product-owner] with
sufficient authorization can't ask devices to take critical updates.

## Prior art and references

The [Omaha protocol includes][omaha-updatedisabled] the `updatedisabled` flag
for essentially this reason: a client device telling the server that it's
checking for an update, but that download and installation should not be
performed.

The Chrome browser [supports disabling updates][chrome-disable-updates] via
enterprise policy. This may be a use case for potential Fuchsia devices, but we
have no enterprise policy requirements at the moment.

[chrome-disable-updates]: https://support.google.com/chrome/a/answer/9838774
[glossary.product]: /glossary/README.md#product
[glossary.product-owner]: /glossary/README.md#product-owner
[omaha-updatedisabled]: https://github.com/google/omaha/blob/ebc25b2b3d77eed3d9a122bcfd89a66f6f192e4b/doc/ServerProtocolV3.md#updatecheck-request
[ota-flow]: /concepts/packages/ota.md
