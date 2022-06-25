<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0155" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Summary

A new field is added to the offer and use sections of component manifests to
denote when a given capability may not exist.

## Motivation

### Core realm shards

When core realm shards are used, this introduces scenarios where a capability
route's source or target do not exist. For example,

- The component `/core/wlancfg` uses the protocol
  `fuchsia.location.sensor.WlanBaseStationWatcher`, but this protocol is not
  present on some products.
- The component `/core/bt-a2dp` must use the
  `fuchsia.power.battery.BatteryManager` capability if it is available, but it
  is not available on all products.
- The `/core/omaha-client-service` component must use the
  `fuchsia.update.config.OptOut` protocol if it is available, but it is not
  available on all products.
- The `CommsAgent` package must use video if it's available, but there is no
  video available on some products.
- Trace provider is a component which provides the
  `fuchsia.tracing.provider.Registry` capability. This is useful for development
  purposes, but this component should not be included on user builds.

The above scenarios are component configurations we want to support going
forward, but there are currently a few places in the Component Framework that
introduce rough edges related to this:

- Offering to or from a component that does not exist within the manifest will
  cause manifest validation to fail.
- Offering a capability from a component that does not expose it will cause
  scrutiny validation to fail.
- Failing a capability route (e.g. attempting to use a capability that was not
  offered) causes component manager to (correctly) emit a warn-level log, which
  can look concerning and thus be misleading when investigating issues on
  products where the capability is intentionally omitted.

### Route validation through collections

Scrutiny is a component route validation tool that ensures that all used
capabilities within a given component tree have valid capability routes. Aside
from the system correctness assertions it provides, this is a useful tool for
development as it provides for shorter development cycles by moving route
validation from run-time to buildtime.

Today an [allowlist is maintained][scrutiny-allowlist] that disables scrutiny
checking of specific capability routes. This allows builds to succeed when a
component uses an unavailable capability, but it unfortunately also allows
builds to succeed when the capability route is misconfigured on builds where the
capability _is_ available.

Additionally once [fxbug.dev/92889][fxb/92889] is resolved, scrutiny will also
be validating capability routes that originate in the session realms. Oftentimes
the session components depend on capabilities originating from outside of the
session, which will add friction if one of these components wishes to be added
to the scrutiny allowlist as these components may be developed outside of
fuchsia.git.

## Stakeholders

Who has a stake in whether this RFC is accepted? (This section is optional but
encouraged.)

_Facilitator:_ Hunter Freyer (hjfreyer@google.com)

The person appointed by FEC to shepherd this RFC through the RFC
process.

_Reviewers:_

- Mark Dittmer (markdittmer@google.com) - Security
- Gary Bressler (geb@google.com) - Component framework
- Marc Khouri (mnck@google.com) - WLAN
- Ani Ramakrishnan (aniramakri@google.com) - Bluetooth
- Yaar Schnitman (yaar@google.com) - Products

_Socialization:_

After receiving requirements from
[fxbug.dev/87164](https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=87164) and
by soliciting feedback from some specific individuals, an early form of this RFC
was shared among stakeholders.

## Design

A new field named `availability` will be added to use declarations. The default
value for this field will be `required`, which means that the component is
expecting the capability to be provided and will likely malfunction if it is not
available. This RFC doesn't propose any changes to the behavior of `required`
capabilities - they'll behave the way all capabilities do today

```
{
    use: [
        {
            protocol: "fuchsia.logger.LogSink",
            availability: "required",
        },
        {
            directory: "config-data",
            path: "/config",
            rights: [ "r*" ],
            // The `availability` field defaults to `required` when omitted.
        },
    ],
}
```

The `availability` field on use declarations may also be set to `optional`, to
reflect when a component can function properly when the capability is not
available (likely with modified behavior).

```
{
    use: [
        {
            // Emergency location is not present in all products.
            protocol: "fuchsia.location.sensor.WlanBaseStationWatcher",
            availability: "optional",
        },
    ],
}
```

A new source named `void` will be added to the possible list of sources for
offer declarations.

```
{
    offer: [
        {
            protocol: "fuchsia.update.config.OptOut",
            from: "void",
        },
    ],
}
```

A capability whose use declaration is optional can either be offered from an
existing component, or from 'void'. A capability whose use declaration is
required may _not_ be offered from void, as this would likely cause the
component to malfunction.

Intermediate offers (from parent to a child) may also set the `availability`
field to one of the following values:

- `required` (default): the child _must_ receive access to this capability,
  regardless of if the child can handle its absence or not (it may not be
  offered from void in an ancestor).
- `optional`: the child _must_ be able to handle the absence of this capability
  (the use declaration at the end of the offer chain must be optional).
- `same_as_target`: the offer will have the same optionality as the target. If
  the target requires the capability, then this offer stipulates that the target
  must receive the capability. If the target has an optional use for the
  capability, then this offer stipulates that the target may or may not receive
  this capability.

```
{
    offer: [
        {
            protocol: "fuchsia.power.battery.BatterManager",
            from: "parent",
            to: "bt-a2dp",
            // Emit a build-time error if this protocol is not correctly routed
            // to this child (including offers from void).
            availability: "required",
        },
        {
            protocol: "fuchsia.location.sensor.WlanBaseStationWatcher",
            from: "parent",
            to: "wlancfg",
            // Emit a build-time error if this child is unable to handle the
            // absence of this protocol.
            availability: "required",
        },
    ],
}
```

If a component has an `optional` use and its parent offers the capability as
`required`, then a build-time error will be emitted if the route for the
capability is invalid or ends in an "offer from `void`". If a component has a
`required` use and its parent offers the capability as `optional`, then a
build-time error will be emitted. This field may be unset, in which case it is
ignored.

### Core realm assembly

As described in [RFC-0089][rfc-0089], the core realm is assembled from "core
shards", which are CML snippets that get merged in with the core realm at build
time. The exact set of core shards is set by the product definition.

It is generally advised to include the offers targeting a child in the same core
shard as the child declaration. This helps to ensure that if a child is included
in the build, then the capabilities it needs to function are also made
available to it.

Including a child's offers in the same shard as the child gets complicated if
the offers wish to have a source that is also optionally included in the build.
If an offer with a target of child `a` has a source of child `b`, but `b` is
also in a shard, then `a`'s shard may only be included on builds where `b`'s
shard is included as well, or a manifest validation error is emitted at
build-time due to the offer from a non-existent child.

To allow a child's offers to live in the same core shard as the child's
declaration, regardless of the source, a new field will be introduced to offer
declarations named `source_availability`. This field will default to the value
`present`, but may be set to `unknown`.

When `source_availability` is set to `unknown` the offer declaration will pass
manifest validation if it references a source that is not in the manifest.
During manifest compilation the missing source of the offer declaration will be
replaced with the `void` source, allowing any use declarations whose routes end
at this offer declaration to be able to pass route validation by setting their
availability to `optional`, to reflect that the capability is not present on all
products.

```
{
    offer: [ {
        protocol: "fuchsia.examples.Echo",
        from: "#child-that-might-not-be-declared",
        to: "#echo-user",
        source_availability: "unknown",
    } ],
}
```

This allows the platform configuration maintainer to rely on the set of core
shards as the single source of truth for what capabilities and components are
present in the core realm. Omitting a core shard not only removes a subsystem
from the core realm, but also accurately updates any offers with a source of
that subsystem to offer from `void`, so that any components that have optional
usage on that subsystem will gracefully not receive access to those subsystems
when they are intentionally excluded from the system (while any required usages
do still cause errors in this case).

## Examples

### A component in the core realm with an optional capability

For this example, let's look at how the manifest and core realm shard for
`wlancfg` would change based on the proposed design.

The offer for the `fuchsia.location.sensor.WlanBaseStationWatcher` protocol
would move from `emergency.core_shard.cml` to `wlancfg.core_shard.cml`, and the
`source_availability` field is set to `unknown` (because the emergency core
shard, and with it the emergency component, may or may not be included in the
same platform configurations as `wlancfg`).

```
// src/connectivity/location/emergency/meta/emergency.core_shard.cml
{
    offer: [
        // This is removed
        {
            protocol: "fuchsia.location.sensor.WlanBaseStationWatcher",
            from: "#emergency",
            to: "#wlancfg",
        },
    ],
}
```

```
// src/connectivity/wlan/wlancfg/meta/wlancfg.core_shard.cml
{
    offer: [
        // This is added
        {
            protocol: "fuchsia.location.sensor.WlanBaseStationWatcher",
            from: "#emergency",
            to: "#wlancfg",
            source_availability: "unknown",
        },
    ],
}
```

The move of the offer declaration to the core shard for `wlancfg` is necessary
for the `source_availability` field to work correctly, and is also in-line with
core shard best practices.

In addition to the core shard changes, the use declaration for this protocol in
`wlancfg` would be updated to be `optional`.

```
// src/connectivity/wlan/wlancfg/meta/wlancfg.cml
{
    use: [
        {
            protocol: "fuchsia.location.sensor.WlanBaseStationWatcher",
            // This line is added
            availability: "optional",
        },
    ],
}
```

Now when the `emergency.core_shard.cml` file is not included in the build there
will be no build-time error due to `wlancfg` not being able to access the
`fuchsia.location.sensor.WlanBaseStationWatcher` protocol. This means the
protocol may be removed from scrutiny's allowlist, and any configuration errors
that cause the protocol to become unavailable to `wlancfg` on platform
configurations where it _is_ available will cause build-time errors.

### A component in the session with an optional capability

For this example, let's look at how the manifests and core realm shard involved
in getting the `fuchsia.power.battery.BatteryManager` protocol from
`/core/battery_manager` to
`/core/session_manager/session:session/workstation_session/login_shell/ermine_shell`
would change based on the proposed design.

The offer for the protocol in the `workstation.core_shard.cml` file would be
updated to set `source_availability` to `unknown`:

```
// src/session/bin/session_manager/meta/workstation.core_shard.cml
{
    offer: [
            protocol: "fuchsia.power.battery.BatteryManager",
            from: "#battery_manager",
            to: "#session-manager",
            // This line is added
            source_availability: "unknown",
    ],
}
```

The offers between `core` and `ermine_shell` would not need to be altered, and
the `ermine_shell` manifest would be updated to set the `availability` field to
`optional` for the protocol:

```
// src/experiences/session_shells/ermine/shell/meta/ermine.cml
{
    use: [
        {
            protocol: "fuchsia.power.battery.BatteryManager",
            availability: "optional",
        },
    ],
}
```

With these changes the `workstation.core.shard.cml` file may be included in
platform configurations without `battery_manager.core_shard.cml`, without
causing build-time scrutiny errors or adding this capability to the allowlist.
This means the battery manager component may be safely excluded from platform
configurations intended to run the workstation session on hardware without a
battery.

### A session owner wants to ensure that an optional capability is always
### available

Building on the preceding example, consider a fictional session named
`workstation-on-laptops` which is identical to the `workstation` session except
it has additional laptop-specific abilities. If the owner of this session wishes
to use the same `ermine` component that is present on the `workstation` session,
but wanted to ensure that it _always_ has access to the
`fuchsia.power.battery.BatteryManager` protocol (enforced by a build-time
error), then the session owner may offer this protocol as `required` to
`ermine`.

```
// src/experiences/session_shells/ermine/session/meta/workstation_on_laptops_session.cml
{
    offer: [
            protocol: "fuchsia.power.battery.BatteryManager",
            from: "parent",
            // login_shell offers the capability to ermine
            to: "#login_shell",
            availability: "required",
    ],
}
```

In this example if the `workstation-on-laptops` session is compared against a
platform configuration that offers the `fuchsia.power.battery.BatteryManager`
protocol from `void` (i.e. the `battery_manager.core_shard.cml` is not
included), then a build-time error will be emitted, despite `ermine.cml` having
an optional use for the protocol.

### A session owner wants to ensure that a component can handle the absence of
### an optional capability

Again building on the "ermine optionally uses `BatteryManager`" example, if the
owner of the workstation session wishes to ensure that `ermine` will always be
able to handle the absence of the `fuchsia.power.battery.BatteryManager`
protocol regardless of the protocol's current availability, they may offer this
protocol as `optional` to `ermine`.

```
// src/experiences/session_shells/ermine/session/meta/workstation_session.cml
{
    offer: [
            protocol: "fuchsia.power.battery.BatteryManager",
            from: "parent",
            // login_shell offers the capability to ermine
            to: "#login_shell",
            availability: "optional",
    ],
}
```

This optional offer has no impact on the build or run-time behavior with the
given arrangement, as ermine also has an optional use of the capability. If
ermine is ever altered to require this capability however, then a build-time
error will be emitted by scrutiny, regardless of if the capability route is
valid or ends in an "offer from `void`".

## Implementation

To implement these changes, the `fuchsia.component.decl.Component` FIDL table
and CML json schema will have to be updated to include the new fields, along
with CMC, `cm_fidl_validator`, and Scrutiny being updated to correctly handle
the `optional` semantics described in the design section.

## Performance

The proposed changes will not have any significant impact on build times or
sizes, as the cost of carrying an additional enum in these manifests is very
small. Additionally the proposed run-time behavior change in component manager
will not have any significant impact on performance, as the information needed
to implement the new namespace assembly logic is all in memory by the time this
work begins.

## Ergonomics

This change should significantly improve the ergonomics for any component author
who currently needs to interact with an allowlist to suppress scrutiny
validation errors, as the context around expected validation errors will be
improved such that scrutiny won't emit these expected errors at all.

## Backwards Compatibility

This change is purely additive to a FIDL table and a JSON format, and thus has
no backwards compatibility concerns.

## Security considerations

This change will make it possible for component authors to disable Scrutiny
errors for unrouted capabilities without receiving approval from the Security
team, as an allowlist will no longer be used to control Scrutiny error
suppression for unrouted capabilities.

This is deemed to be acceptable because the review processes for creating or
altering the component routes themselves is unchanged by this proposal.

## Privacy considerations

The proposed changes do not impact how user data is collected, processed, or
stored, and thus do not have any privacy concerns.

## Testing

There are already extensive tests covering the component manifest pipeline.
These tests will be altered to include coverage for this new feature.

## Documentation

The component manifest documentation will be updated to describe the new field
and its impacts on validation.

## Drawbacks, alternatives, and unknowns

## Prior art and references

TODO

[rfc-0089]: ./0089_core_realm_variations.md
[scrutiny-allowlist]: /src/security/policy/build/verify_routes_exceptions_allowlist.json5
[fxb/92889]: https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=92889
