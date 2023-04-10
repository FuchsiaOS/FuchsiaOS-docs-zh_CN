<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0213" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
{# Fuchsia RFCs use templates to display various fields from _rfcs.yaml. View the #}
{# fully rendered RFCs at https://fuchsia.dev/fuchsia-src/contribute/governance/rfcs #}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Summary

This proposal outlines a process for removing a devfs feature where multiple
FIDL protocols are multiplexed over a single connection. It explains the reason
why removing this feature will unblock new driver features, and standardize
driver FIDL connections with the rest of Fuchsia.

## Motivation

The Driver Framework manages the connection between a hardware device and a
client. Historically, this connection has served three FIDL protocols:

- `fuchsia.io/Node`
- `fuchsia.device/Controller`
- the device's actual FIDL protocol

This multiplexing requires that the Driver Framework owns the channel, so that
it can serve the Node and Controller protocols. This prevents the underlying
driver from owning the channel. This design means that the Driver Framework must
maintain a C FIDL boundary between the Driver and the Driver Host. This also
means that drivers are unable to disambiguate between multiple connections or
store per-connection state. This restriction has led to a number of "trampoline
protocols", or protocols where the driver and client exchange a new pair of
channels simply to avoid the multiplexed channels.

This multiplexing goes against FIDL design principles for
[composition][composition]. FIDL design principles state that a protocol can be
`composed` of multiple protocols at compile time. However, the Driver Framework
is doing this composition at run time and is unaware of the FIDL protocol being
spoken by the device. Multiplexing at runtime also runs into the possibility of
conflicting ordinals in the protocols that are being multiplexed.

The multiplexing of the Controller protocol means it isn't possible to use
capability routing to restrict access to `fuchsia.device/Controller`. For
example, clients of `/dev/class/input-report` mostly want to access
`fuchsia.input.report/Device`. However, `fuchsia.device/Controller` is served
on the same channel, so are able to unbind, rebind, or set power states on these
devices. It is not possible to solve this routing problem when the device's FIDL
and the Controller FIDL are multiplexed on the same channel.

The multiplexing of the Node protocol is used throughout the codebase to Clone a
connection to the device. While the concept of cloning a device connection is
fine, this is a capability that the underlying driver is not aware of.
Additionally, the API for `fuchsia.io/Node.Clone` takes in a server end of a
`fuchsia.io/Node`, so using this API to clone a device's channel is lying about
the underlying type of the channel. There is a substantial amount of client
code today that does this, either by calling the FIDL directly or by using file
descriptors. Fuchsia has an established best practice of using typed channels,
and we should be enforcing these best practices in this area.

## Stakeholders

_Facilitator:_

- abarth

_Reviewers:_

- abarth
- cja
- csuter
- surajmalhotra
- tamird
- yifei

_Consulted:_

_Socialization:_

This RFC originated as a design document which went through a design review with
the Driver Framework team.

## Requirements

The requirements are split into two categories: Migration and End State.

### Migration

The migration plan must be a soft migration with an allowlist. Fuchsia contains
a lot of code that interacts with driver clients, both in-tree and out-of-tree.
It will not be possible to remove multiplexing from all of devfs at once.
Having an allowlist with discrete tasks will allow us to make incremental
progress and prevent the migration from backsliding.

The migration should be as mechanical as possible. Each client update should be
simple enough that folks unfamiliar with drivers or the driver framework would
be able to perform a migration. The more mechanical each update, the faster the
migration will be.

### End State

Connections to devices should be explicit. It should be clear if a client is
requesting the device controller or the underlying device protocol. There will
be no FIDL multiplexing, so clients will be able to use typed channels.

The Driver Framework will not serve `fuchsia.io/Node` on a connection to a
device.

Drivers will be able to own their own connection to a client. After the FIDL
multiplexing is removed, there will be another migration to move Drivers to a
DDK API that will provide the driver with a channel whenever a client attempts
to connect. This migration will allow drivers to have per-connection state and
use the FIDL bindings that the rest of the Fuchsia codebase uses.

## Design

### Connecting to fuchsia.device/Controller

In order to connect to `fuchsia.device/Controller` a client will have to open a
specific node in the devfs filesystem. This node will be named
`device_controller` and will be available in both devfs class paths and devfs
topological paths.

If a client is getting the device protocol from `/dev/class/input-report/abcd`
the controller will be available at
`/dev/class/input-report/abcd/device_controller`.

If a client is getting the device protocol from `/dev/sys/platform/pci/00:12`
the controller will be available at
`/dev/sys/platform/pci/00:12/device_controller`.

### Removing multiplexing from /dev/class/

There will be two allowlists for devfs class paths: one for `fuchsia.io/Node`
and one for `fuchsia.device/Controller`. These allowlists will contain the names
of `/dev/class/{protocol}` that are still multiplexing the respective protocol.

Entries from this allowlist will be removed after clients have been updated to
stop relying on the multiplexed behavior.

### Removing multiplexing from topological paths

When an entry in the allowlist is removed for a class path, the corresponding
entries in the topological paths will also have multiplexing removed from them.

For example, if `/dev/class/input-report` has `fuchsia.io/Node` removed, the
topological paths corresponding to those input devices will also no longer be
serving `fuchsia.io/Node`.

## Implementation

The allowlists will be implemented in the driver framework. Removing an entry
from the allowlist will require updating clients that rely on this multiplexing.

## Performance

There should be no significant performance impact for removing FIDL
multiplexing. There may be a slight performance increase since a FIDL message
to a device does not need to attempt to be dispatched to the Node or Controller
API.

## Ergonomics

The Driver Framework may decide to add a helper library for connecting to the
Controller API. This will depend on the first couple of migrations.

The Driver Framework will also be updated to log an error if a client is
attempting to call an unknown FIDL protocol on a device. This means there will
be ERROR logs if a client is incorrectly still relying on multiplexing.
Unfortunately it is not simple to correctly attribute the error log to the
specific client.

## Backwards Compatibility

There is no way to remove the FIDL multiplexing and retain backwards
compatibility.

## Security considerations

There are no security considerations for this proposal. This work may slightly
improve the security of devfs by making client behavior more explicit and
understandable.

In the future, the Driver Framework will want to restrict access to `fuchsia.device/Controller`.
For example, clients that get access to `/dev/class/input-report` have no need to access
the controller protocol. Restricting this will improve security, and removing FIDL multiplexing
gives the Driver Framework the ability to restrict this in the future. The actual plan for
restricting this protocol is outside the scope of this RFC and can be taken as a followup project.

## Privacy considerations

There are no privacy considerations for this proposal.

## Testing

The Driver Framework features for the allowlist and the removal of multiplexing
will be tested.

Each removal of the allowlist will unfortunately have to rely on existing test
coverage in CQ. Because this multiplexing is happening outside of the FIDL type
system, there are no good ways to statically determine which clients rely on
the multiplexing behavior.

## Documentation

Documentation has been added in the form of an [Open Project Page][project]
This page outlines the allowlist, motivation, and update steps. When the project
is complete, no clients will depend on this behavior and so no documentation
will be needed.

## Drawbacks, alternatives, and unknowns

### Using service capabilities in DFv2

The main alternative to doing this work now is to wait until the drivers as
components (DFv2) effort is finished and then migrate clients directly to using
[Service Capabilities][service-capabilities].

However, migrating clients to service capabilities cannot start until DFv2 is
enabled on all possible boards. More clients that depend on multiplexing will be
added in the meantime, including out of tree clients that will be more difficult
to migrate. Additionally, the DFv1 compatibility shim will need to support FIDL
multiplexing in order to preserve existing behavior, and DFv2 drivers that
export themselves to devfs will also need to support multiplexing. Removing this
tech debt from DFv1 will simplify the DFv2 design, instead of requiring that
this tech debt be carried forward.

Moving clients to service capabilities will be more difficult if the client also
needs to untangle FIDL multiplexing at the same time. These migrations will move
faster if one issue is fixed at a time.

## Prior art and references

FIDL has outlined the [best practices for protocol composition][composition].

<!-- xrefs -->
[composition]: /contribute/governance/rfcs/0023_compositional_model_protocols.md
[project]: /contribute/open_projects/drivers/fidl_multiplexing.md
[service-capabilities]: /concepts/components/v2/capabilities/service.md