<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0101" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Summary

This document proposes adding a new parameter to
`fuchsia.sys2.Realm/CreateChild`. This parameter will contain a set of numbered
handles. The created component's [runner][runners] will receive these handles
when it is asked to run the component. The provided handles will only be
available to components that run in collections with a new type of durability:
`single-run`. Components in a collection with a [durability][collections] of
`single-run` are started when created and destroyed when stopped. This scopes
the provided handles to a single run of the component.

## Motivation

Starnix is a Component Framework v2 [runner][runners] that runs Linux binaries
on Fuchsia. Starnix implements the Linux system interface to run these binaries
without modification.

Starnix provides a [ffx plugin][ffx] that allows users to run Linux components
from the host command line. Starnix wants to connect the
`stdin`/`stdout`/`stderr` of the Linux component to a triple of socket handles.
This component does not interact with the system by exposing FIDL protocols.
Instead, it interacts with the user via the provided sockets.

## Design

### Background

There are two dimensions of lifecycle transitions in the component framework.
The first has to do with existence (`Created` and `Destroyed`). The second
relates to the component's execution (`Started` and `Stopped`). In other
operating systems, which only have processes and not components, these
dimensions are equivalent: creating a process is the same as starting it, and a
process is destroyed when it stops running.

The distinction between `Created` and `Started` is relevant when providing
arguments to components. Ordinarily a single component instance can be started
and stopped multiple times, so the component manager must store the arguments to
provide them each time the component runs. If the arguments are handles this can
be problematic since not all handles can be duplicated. Any non-duplicable
handles are thus "consumed" by a single run of the component.

### Protocol updates

As mentioned in [Background](#background), creating a component is distinct from
starting it. Thus the handles provided to `CreateChild` must be either:

  1. Available each time the component is started.
  2. Scoped to a single run of the component.

Since not all handles can be duplicated (stored in the component manager for
subsequent runs), (1) would only be possible if new handles were fetched from
their source for each run. See [routing
handles](#routing-numbered-handles-as-capabilities) for why an approach that
fetches handles at startup was not chosen. Fortunately (2) is a viable solution
for the motivating use case.

A new table, `ChildArgs` is created and added as a parameter to
`Realm/CreateChild`:

```
protocol Realm {
  /// If args contains numbered_handles, the collection must have a durability
  /// of type `single-run`.
  CreateChild(CollectionRef collection, ChildDecl decl, ChildArgs args)
      -> () error fuchsia.component.Error;
}

resource table ChildArgs {
  /// The numbered handles for the component instance.
  ///
  /// Only PA_FD and PA_USER* handles are valid arguments, and inclusion of any other
  /// handles will result in an error.
  1: vector<fuchsia.process.HandleInfo>:N numbered_handles;
}
```

In addition, the `fuchsia.component.runner.ComponentStartInfo` table will be
updated to contain the numbered handles:

```
resource table ComponentStartInfo {
  6: vector<fuchsia.process.HandleInfo>:N numbered_handles;
}
```

The runner will provide those handles to the component, or close them and return
an error if the runner does not support providing numbered handles to
components.

### Collection durability

Components that are created via the `Realm` protocol live in collections.
Collections have a `durability` annotation that indicates what the lifecycle
semantics of components within the collection are.

A new collection `durability` value, `single-run`, will be added to indicate
that components in the collection are started immediately when they are created,
and destroyed when they are stopped. `ChildArgs.numbered_handles` can only be
used with collections that are marked `single-run`. This scopes the arguments in
`ChildArgs` to a single run of the component.

```
collections: [
    {
         name: "playground",
         durability: "single-run",
    }
],
```

## Implementation

Component manager will be updated to store the `ChildArgs` until they are passed
to the runner, and to handle `single-run` collection semantics.

## Backwards Compatibility

The change is backwards compatible with regards to runners: runners are not
required to use the numbered handles that are provided. If a runner does not
support numbered handles it is expected to close the handles.

The change is not backwards compatible for `Realm` clients.

The change is backwards compatible with regards to CML: the only change is an
added durability enum.

## Performance

No performance impact is expected as the handles are provided directly to the
component manager.

## Security considerations

This change introduces a way for parents to pass arbitrary numbered handles to
children. The exchange of these handles is mediated by both the component
manager and the runner. Only components that are run in a collection marked
`single-run` can receive handles in this fashion.

## Testing

The existing tests for `CreateChild` will be exapnded to cover the new
arguments.

## Drawbacks, alternatives, and unknowns

### Drawbacks

There are a couple drawbacks with the proposed solution compared to the
alternatives listed below:

  - It is a new way of starting a component: the component's lifecycle is not
    influenced by capability binding.
  - The provided handles are opaque to the component framework's static CML
    analysis.
  - Since the components receiving the handles are destroyed when stopped, their
    persistent storage is also wiped out. Thus components that use persistent
    storage are not a good candidate for this feature.

### Routing numbered handles as capabilities

Routing numbered handles could be done explicitly by the component framework.

There are several different ways that this could be done, but they all have the
following "shape."

Introduce a protocol that would be implemented by the source of the numbered
handle(s):

```
protocol HandleProvider {
    Get(string handle_name) -> (fuchsia.process.HandleInfo handle);
}
```

This protocol is then turned into a capability:

```
capabilities: [
    {
        handle_provider: "stdin",
        path: "/svc/fuchsia.component.HandleProvider",
    },
],
expose: [
    {
        handle_provider: "stdin",
        from: "self",
    },
],
```

This could then be routed, via CML, to the destination just like other
capabilities are.

#### Benefits

  - Could be evolved to support more type information about the handles.
  - Makes the routing of the handles explicitly visible to the component
    framework.

#### Drawbacks

The component would not be able to start until the handles have been fetched.
Even if the performance impact could be amortized by caching handles in
component manager, the motivating use case would not benefit from this because
the handles would be different for each run of the component. In the proposed
design, the host code could "fire and forget" the request to start a component
and continue executing as if the call was successful. In this alternative, the
host code would need to sit and spin, waiting for the associated handle request
to come back.

The handle provider is not always reachable via static routing. In the
motivating use case the handle provider is on the host machine, connected to the
Fuchsia device via Overnet. This could be solved by routing "as far as possible"
and then have the "edge" component fetch the handles via an ad hoc mechanism
before returning them to the component framework. This is an additional burden
on developers that want to use the feature.

A handle provider has no way of distinguishing between handle requests from a
given capability route. Specifically, consider the motivating use case:

  1. The user starts two Linux components from the host, in different terminals.
  1. The components are instantiated in a collection.
  1. The handle provider receives two requests for the same handle.

At this point, there is no way for the handle provider to know which component
is associated with which handle request. This could be solved by introducing
additional mechanisms for client identification, but it is considerably more
complex than the proposed design.

This solution is a larger commitment and more complex than the proposed design.
That said, the proposed design does not prevent or conflict with explicit
routing of numbered handles in the future.

### StartChild

This alternative proposes adding the numbered handles as an argument to the
`StartChild` call on `Realm`. This is similar to the proposed design but has the
drawback of introducing a race between a component binding to a capability
offered by the component (which would start the component) and the `StartChild`
call being made. Specifically, the component would only receive the numbered
handles if the `StartChild` call won the race since it's not clear how the
handles would be delivered if the component was already started.

### Starter protocol

Passing numbered handles could be done by a `Starter` protocol. The starter
protocol can be used to start a component, is implemented by the component
manager on behalf of a component, and can be routed like any other protocol
(i.e., the component can be started by a component that is not its parent).

This protocol can be routed like any other protocol, so a client can use it to
start components that are located in arbitrary locations in the component
hierarchy.

A starter protocol contains a method that accepts numbered handles as arguments:

```
[Discoverable]
protocol StarterWithArgs {
    /// Start the component that is bound to this protocol.
    /// If the component is already running, the call returns an error.
    Start(StartArgs start_args) -> () error fuchsia.component.Error;
};
```

This proposal is very similar to [StartChild](#StartChild). A protocol exposed
in this fashion has the benefit of being easier to audit and allowlist, but also
introduces a race between calling `Start` and binding to any other capability
exposed by the component. In addition, the ordering concerns are worsened by the
fact that the client isn't always the parent, and thus can't necessarily restart
the component to provide the arguments. This could be solved by adding a stop
method to the protocol.

Clients would also need to coordinate child management between the starter
protocol and the realm protocol, instead of managing the child exclusively
through the realm protocol.

[runners]: concepts/components/v2/capabilities/runners.md
[ffx]: development/tools/ffx/overview.md
[collections]: concepts/components/v2/realms.md#collections