<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0110" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Summary

A proposal to introduce a "reboot-on-termination" option to the component
manifest's child declaration, which provides parity to sysmgr's
`critical_components` feature.

## Motivation

In Components v1, sysmgr supports a
feature called `critical_components` which lets
system service components mark themselves as "critical". This means that, if the
component terminates for any reason (including normal exit), sysmgr will trigger
a reboot of the system. This reboot is a [graceful reboot][doc-graceful-reboot]
driven by `power_manager`, which causes the component topology to go through
orderly shutdown. Graceful reboot tears down the system in a consistent manner
and gives components a chance to shut down cleanly, allowing diagnostics to be
preserved and filesystems to shut down cleanly.

Clients typically set this option on their component if they are not confident
that normal system behavior can proceed if their component fails.
Unsurprisingly, this option tends to be set on components whose services play a
central role in the system's operation, for example:

-   `netstack`
-   `wlanstack`
-   `omaha-client-service`
-   `system-update-checker`

There are many more possible strategies for crash recovery besides the
relatively simple one implemented by `critical_components`. This design is
focused on solving that use case. Crash recovery beyond what
`critical_components` provides is out of scope (but see
[Future work](#future-work)).

### Requirements

The primary requirement is to provide feature parity with `critical_components`.
This means that it should be possible for components under `core`, or a
sub-realm of `core`, to opt into triggering graceful reboot if their component
terminates.

### Why now? {#why-now}

The components mentioned in the [Motivation](#motivation) which use
`critical_components` are blocked from migrating to Components v2 until an
equivalent feature is available.

## Design

We will add an `on_terminate` enum to [`ChildDecl`][fidl-childdecl] (equivalent
to the [component manifest][doc-manifests]'s `children` section), providing
semantics equivalent to `critical_component`. There are two options: `none`
(default) or `reboot`. When a child component with `on_terminate: reboot`
terminates for any reason (including a normal exit), `component_manager` will
invoke the [`Admin/Reboot`][fidl-reboot] method from
[`fuchsia.hardware.power.statecontrol.Admin`][fidl-reboot] protocol exposed by
`power_manager` to trigger a graceful system reboot.

This necessitates a dependency cycle between `component_manager` and
`power_manager`. However, both are in the ZBI, so there's no significant
layering problem. In any case, there's no avoiding some degree of dependency
inversion because reboot causes a change in the device's power state, which is
the responsibility of a driver.

If the call to `Admin/Reboot` fails, `component_manager` will fall back to
panicking, triggering an ungraceful reboot.

This is a sensitive feature; we don't want arbitrary components to unilaterally
decide to trigger a reboot when they terminate. Thus, its use will be restricted
by an allowlist in `component_manager`'s security policy, which will be checked
at runtime when the component starts. Also, we can use the
[`restricted_features`][git-restricted-features] GN allowlist to produce a
build-time failure when the option is set on
a child in a realm that's not authorized to use the feature.

## Implementation

### `on_terminate` option

We need to add the `on_terminate` option to the [manifest][doc-manifests]'s
`child` section. This will require changes to `cmc`, `cmc_fidl_validator`, and
`cm_rust` to plumb the option through. Since this is a special feature, we will
allow it to be set to `None` in the `ComponentDecl` (defaulting, of course, to
`on_terminate: none`).

We will add a new `restricted_feature` to `cmc` for `on_terminate`. Only CML
files in this allowlist will be able to set `on_terminate: reboot` on their
children. To start, this allowlist will consist of the `core` and `network`
realms.

We will also add a `reboot_on_terminate_enabled` bool to `component_manager`'s
config so it can be disabled for non-root instances of component manager (for
example, nested instances in tests).

### Detecting termination of reboot-on-terminate components

Logic must be added to `component_manager` to detect when reboot-on-terminate
components terminate. During the `Stop` action, `component_manager` can check
the `on_terminate` option. If it is set, and the component is not shutting down,
`component_manager` [calls](#calling-reboot) `Admin/Reboot`. Shutdown means that
the component is stopping and will never be started again, which happens in the
following scenarios:

1.  During [system shutdown][fidl-shutdown], which itself is triggered by the
    `Admin/Reboot` protocol. In this case the system is already shutting down,
    so there is no point in triggering shutdown again.
2.  When a component is destroyed. This can happen from either (a) an explicit
    call to [`DestroyChild`][fidl-destroy], (b) the parent of a `transient`
    collection stopping, or (c) a component in a `single-run` collection
    exiting. In the cases of (a) and (b), not triggering reboot seems like the
    right decision, since it was an action external to the component rather than
    a termination from within the component that caused it to stop. In the case
    of (c), we can still ensure the component exiting triggers reboot if we
    implement the feature carefully, by triggering the destruction procedure
    only once the component has terminated.

### Calling the `fuchsia.hardware.power.statecontrol.Admin` protocol {#calling-reboot}

To trigger a graceful reboot, one connects to the protocol
[`fuchsia.hardware.power.statecontrol.Admin`][fidl-reboot] and calls
`Admin/Reboot`. This protocol is implemented by the `power_manager` component.
(It is actually proxied by `shutdown_shim`, for historical reasons.) Since this
protocol is implemented by a component, how does `component_manager` get access
to it? To accomplish this, we can have `root` expose the protocol from
`#bootstrap` to its parent. This means that root is exposing the protocol to the
node _above_ the root, i.e. `component_manager`. See the [Design](#design) for
more explanation of this inversion.

### Prototype

A prototype can be found [here][gerrit-system-critical-prototype].

## Performance

This design has no performance considerations. `component_manager` will only
open a connection to `fuchsia.hardware.power.statecontrol.Admin` if an
`on_terminate: reboot` component actually terminates.

## Ergonomics

This design has simple ergonomics: all that's required to set
reboot-on-terminate on a component is to do the following:

-   Set `on_terminate: reboot` in the parent's `ChildDecl` (`children`
    declaration in CML).
-   If not already present, add the parent's CML to the `cmc`
    `restricted_features` allowlist for `on_terminate: reboot`.
-   Add the component's moniker to the policy allowlist for reboot-on-terminate.

Because the `on_terminate` option is set by the parent, not the component
itself, a component that should trigger reboot in production can be harnessed in
a test without having to modify the CML. Furthermore, this makes it possible to
include the component in different product configurations that wish to set the
option differently, without having to change the component.

## Backwards Compatibility

This change does not break compatibility. Clients must explicitly opt in to
reboot-on-terminate.

## Security considerations

Hypothetically, a user could abuse this feature by marking a component as
reboot-on-terminate that shouldn't be, triggering a reboot inappropriately.
However, because uses are restricted by a security policy allowlist, new uses
must receive explicit approval. Note that it is impossible for an untrusted
component to trick `component_manager` into granting it reboot privileges by
embedding an allowlisted component, because the component is allowlisted by its
moniker (topological path), not URL.

## Privacy considerations

This proposal introduces no new privacy considerations.

## Testing

We can easily integration test this feature by mocking the
`fuchsia.hardware.power.statecontrol.Admin` protocol. We should remember to test
unhappy paths like when the protocol is missing or fails.

Ideally, E2E test coverage should be added for reboot-on-terminate components,
to verify that their termination indeed triggers a graceful reboot.

## Documentation

The following documentation changes must be made:

-   Add a doc for the `on_terminate` option to parallel
    `critical components`.
-   Update the [migration guide][doc-migration-guide] to explain how to migrate
    `critical_component`s

## Drawbacks, alternatives, and unknowns

### Benefits and drawbacks

*Benefits*

-   Very simple to configure.
-   Direct parity with v1, making it easy to migrate.
-   Because the feature lives entirely in `component_manager`, it's
    straightforward to implement and doesn't carry as much risk of failure modes
    like lost events.
-   Could allow us to replace some uses of `main_process_critical` with
    `on_terminate: reboot`, which is strictly superior.
-   Allows clients to harness components that have `on_terminate: reboot` set in
    production without modification.

*Drawbacks*

-   Not capability based, which diverges from the orthodox framework model.
-   Encodes some crash recovery policy directly in `component_manager`. While
    this is not something we want to encourage in general, in this case the
    policy is simple, so the cost, while nonzero, seems small.
-   Introduces an inverted dependency on `power_manager` by `component_manager`.
    However, they are both in the ZBI so it's not a major layering violation.
-   Since a CML schema change is involved, this option needs to be plumbed
    through several places: `cmc`, `cm_fidl_validator`, `cm_rust`, and clients
    of `cm_rust`, even though it's a niche feature.

### Alternative: `system_critical` bit on `program`

Instead of adding the option to `ChildDecl`, we could add it to the
[component manifest][doc-manifests]'s`program` section. The primary difference
with this approach is that the option is set on the component itself, rather
than the child declaration in the parent.

Putting the bit in `program` has the advantage of keeping a specialized feature
out of `ComponentDecl` proper. Since `program`, from `ComponentDecl`'s
perspective, has freeform syntax, we don't need to change `cmc`, the validators,
or the rust bindings to account for the new option. We only need to add logic in
`component_manager` itself that retrieves the option from `program` when the
component stops (to determine if a reboot is needed).

However, this approach has one notable downside: if a `system_critical`
component is harnessed in a test, its CML must be altered to remove the
`system_critical` bit (because the bit is not allowed to be set in the test
realm, and we don't want tests to trigger system reboot). This increases the
maintenance burden on clients who write integration tests that harness the
component.

### Alternative: Use `main_process_critical`

The ELF runner supports a feature called
[`main_process_critical`][doc-main-process-critical] which causes
component_manager's root job to terminate when the component exits with a
non-zero status or is killed. This has the effect of causing an ungraceful
reboot. Because the reboot is ungraceful, this causes the system to shut down
uncleanly and doesn't give the system a chance to persist diagnostics or
metrics.

`main_process_critical` should only be used in places where triggering graceful
reboot is not possible. For example, `power_manager` itself is marked
`main_process_critical`. Since this is not the case for any critical component,
this option is not seen as a viable alternative, but is listed here for
completeness.

### Alternative: Supervisor {#alternative-supervisor}

Instead of managing crash recovery in `component_manager`, we could manage it in
the `core` realm. This alternative consists of two parts. First, introduce
"component-scoped" events which allow consumers to monitor events (in
particular, `Started` and `Stopped` events) scoped to a single component
instance. Second, introduce a component called a _supervisor_ which consumes
those events to monitor for abnormal termination or failed start and reboot the
system in response.

#### Component-scoped events {#component-scoped-events}

An idea that's been discussed among the Component Framework team is to provide a
way to allow event capabilities to be scoped to a single component instance,
rather than an entire realm. This design provides a concrete application for
this idea. The supervisor only needs to monitor particular components, so it
makes sense for it to receive events about those components in particular, not
the entire realm.

For velocity, we propose introducing the smallest possible change to CML
necessary to enable this feature. In the future, we're likely to make more
substantial syntax revisions that designate an event's scope in a different way
(see [Component events RFC][gerrit-rfc-component-events]). We will add a `scope`
field to the `offer event` declaration, which can specify a `#child`, or `realm`
(default).

```json5
// core.cml
offer: [
    {
        event: "started",
        from: "framework",
        scope: "#wlanstack",
        to: "#supervisor",
        as: "started-wlanstack",
    },
    {
        event: "stopped",
        from: "framework",
        scope: "#wlanstack",
        to: "#supervisor",
        as: "stopped-wlanstack",
    },
],
```

Given that future revisions to the syntax are likely, we can have `cmc`
allowlist the `scope` feature to `core.cml` and integration tests.

Component-scoped events will not carry information about the identity of the
component in its payload, such as the moniker or URL. In general, events can
carry sensitive information in their payloads such as component monikers or
URLs, which we wish to expose only on a need-to-know basis. Because the
supervisor does not need this information,
[component-scoped events](#component-scoped events) will not provide information
about the identity of the component that generated the event. The remainder of
the information in the payload is a timestamp and the termination status, which
is not sensitive.

#### The supervisor

The supervisor itself is simple. It is a component under `core` that does the
following:

-   Use a static `event_stream` with a list of `Started` and `Stopped` events.
-   If over this event_stream it receives either a `Started` event with an
    error, or a `Stopped` event with a payload that contains a non-ok status,
    trigger a graceful reboot by calling
    [fuchsia.hardware.power.statecontrol/Admin.Reboot][fidl-reboot].

This is a simple implementation target to the `critical_components` feature. In
the future, the supervisor may evolve to support more use cases, or there may be
multiple supervisors -- see [Future work](#future-work).

#### Routing events to the supervisor

Component-scoped events must be routed from every critical component to the
supervisor. For critical components that are a child of `core`, this requires
two changes:

-   A modification to core.cml to route the Started and Stopped events from the
    component to the supervisor (see
    [Component-scoped events](#component-scoped-events))
-   A modification to the supervisor's CML to consume the events in a static
    event stream.

If the critical component is nested under a sub-realm of `core`, another step is
required:

-   Modify every intermediate component to expose the event from the child to
    its parent.

For example, this is likely to be the case for netstack because it is planned
for `netstack` to live in a `network` sub-realm under `core`.

Here's an example of what the supervisor's CML could look like:

```json5
// supervisor.cml
use: [
    {
        events: [
            "netstack-started",
            "netstack-stopped",
            "wlan-started",
            "wlan-stopped",
        ],
    },
    // The supervisor will trigger reboot under the following conditions:
    // - It receives a `started` event with an error.
    // - It receives a `stopped` event with a non-ok status.
    {
        event_stream: "EventStream",
        subscriptions: [
            {
                event: [
                    "netstack-started",
                    "netstack-stopped",
                    "wlan-started",
                    "wlan-stopped",
                ],
                on_receive: "start",
            },
        ],
    },
],
...
```

Note that the component being watched doesn't need to be modified. This is
intentional: supervision is considered a function of how a realm manages its
components, not the components themselves. In other words, it's not the
component's responsibility to decide whether or how it is to be supervised.

#### Starting the supervisor

We need to ensure the supervisor is always started in time to receive an event.
To accomplish this, we propose adding an option to `event_stream` subscriptions
called `on_receive: "start"`. `on_receive: "start"` causes `component_manager`
to automatically start the component when it receives that event. In this way,
`component_manager` guarantees that events are never lost. The default option,
`"dispatch_if_started"`, dispatches the event to the component only if it's
already running (default behavior).

This will require changes to the event dispatch system. Specifically, when an
event is dispatched, `component_manager` must follow any routed event
capabilities in case they are consumed by a static event stream. Otherwise, a
component may miss an event even if it has marked `on_receive: "start"` if it's
not been resolved yet.

There may be an argument for making `on_receive: "start"` the default behavior
of static event streams, but that's beyond the scope of this proposal.

#### Benefits and drawbacks

*Benefits*

-   Avoids encoding crash recovery policy in `component_manager`. This promotes
    better separation of concerns because, as a general rule, we don't have a
    strong understanding of what sorts of crash recovery policies are
    generalizable enough to justify having direct support in
    `component_manager`.
-   This approach is more adaptable than the `recovery` option. In basemgr and
    sessionmgr it will be necessary to implement crash recovery policies
    different from reboot-recovery, for agents and the session itself.

*Drawbacks*

-   Requires support from the events system that needs to be built. This adds
    complexity to the events system and likely requires more time and effort
    than implementing a solution directly in `component_manager`.
-   Requires more boilerplate than `recovery`. Events for each critical
    component must be routed from each critical component to the supervisor.
-   We will need to solve similar problems in `basemgr`/`sessionmgr` eventually.
    If we defer designing a more general solution until then, we may at that
    time have a better understanding of the problem space.

## Future work

`basemgr` and `sessionmgr` implement their own crash recovery strategies which
could utilize an approach along the lines of the
[supervisor](#alternative-supervisor) alternative.

`fshost` and `archivist` currently use `main_process_critical`. It's possible
that instead they could use terminate-on-reboot. That would allow us to limit
`main_process_critical` to components involved in the reboot process
(`driver_manager` and `power_manager`).

Some paths still trigger an ungraceful reboot:

-   This design creates an inverted dependency of `component_manager` on
    `power_manager`, and indirectly `driver_manager`. For this reason, these
    components cannot use terminate-on-reboot so they are marked
    `main_process_critical` instead, meaning that a crash of either of these
    components will trigger an ungraceful reboot.
-   If the `Reboot` call itself fails, `component_manager` panics which also
    triggers an ungraceful reboot.

It's possible that we could execute a more graceful shutdown in these
circumstances; for example, component_manager could perform normal system
shutdown and then exit. On the other hand, since `power_manager` and
`driver_manager` are so critical to system operation, we may not wish to let the
system continue running for any length of time if they crash.

We could potentially revisit how power management responsibilities are
distributed; for example, perhaps `component_manager` could be capable of
driving reboot itself (it would still need to rely on `driver_manager` to set
the power state).

After the system is fully migrated to Components v2, there is the potential for
component manager to support more intelligent recovery strategies by leveraging
its knowledge of the dependency graph.

## Prior art and references

Private design documents exist for the `critical_components` feature and
revisions for the events API.

[fidl-childdecl]: https://fuchsia.dev/reference/fidl/fuchsia.sys2#fuchsia.sys2/ChildDecl
[fidl-destroy]: https://fuchsia.dev/reference/fidl/fuchsia.sys2#fuchsia.sys2/Realm.DestroyChild
[fidl-reboot]: https://fuchsia.dev/reference/fidl/fuchsia.hardware.power.statecontrol#fuchsia.hardware.power.statecontrol/Admin.Reboot
[fidl-shutdown]: https://fuchsia.dev/reference/fidl/fuchsia.sys2#fuchsia.sys2/SystemController.Shutdown
[doc-graceful-reboot]: /docs/concepts/process/why_fuchsia_devices_reboot.md#graceful_reboot
[doc-main-process-critical]: /docs/concepts/components/v2/elf_runner.md#using_the_elf_runner
[doc-manifests]: /docs/concepts/components/v2/component_manifests.md
[doc-migration-guide]: /docs/development/components/v2/migration/README.md
[gerrit-rfc-component-events]: https://fuchsia-review.googlesource.com/c/fuchsia/+/535692
[gerrit-system-critical-prototype]: https://fuchsia-review.googlesource.com/c/fuchsia/+/539915
[git-restricted-features]: https://fuchsia.googlesource.com/fuchsia/+/refs/heads/main/tools/cmc/build/restricted_features
