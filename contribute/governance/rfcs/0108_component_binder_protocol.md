<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0108" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

## Summary

This RFC introduces a new framework-provided protocol,
`fuchsia.component.Binder`, that will allow components to start other
components that expose it.

## Motivation

The [fuchsia.sys2.Realm protocol][fidl-realm] is a framework-provided API that
allows components to manipulate their [realm][doc-realm] at runtime. With this
protocol, components can create child components and bind to their exposed
capabilities, driven by runtime decisions rather than just static declarations.
Components can bind to the child's capabilities by invoking the `BindChild`
method. This method, upon successful execution, starts the provided child
component, if not already started, and opens a connection to an instance of
`fuchsia.io.Directory` [protocol][fidl-directory] that is backed by the child
component's exposed directory. The child's exposed directory is a directory
containing all capabilities exposed by the child in its manifest.

This method has two drawbacks. First, it is currently overloaded because it
fulfills two use cases. It allows components to bind to their child
component's capabilities, *and* it also allows components to start the child
components. Secondly, it is incongruous with Component Framework's capability
model. Most components start in order to satisfy a request for a capability.
Said differently, components bind to capabilities, not to the components
providing those capabilities. This is an important feature of encapsulation
because if components interact with capabilities, as opposed to components
directly, the implementing component can be swapped out without any changes to
the client.

Consequently, `BindChild` will be deprecated. A replacement
method, `OpenExposedDir`, will be added to the `fuchsia.sys2.Realm` protocol
that will allow parents to bind to their child's capabilities. The notable
difference between this method and `BindChild` is that in the new method,
the child will be started if and only if the parent binds to one of its
capabilities. This change in semantics better aligns with Component Framework's
design principles. Work for this migration has already begun and can be tracked
at fxr/531142.

However, simply replacing `BindChild` with `OpenExposedDir` will not suffice.
There are a significant, in terms of quantity and importance to the platform,
number of use cases that rely on the automatic starting behavior of `BindChild`.
In these instances, the parent component doesn't bind to any of the
child's exposed capabilities or the parent starts a component that doesn't
expose any capabilities. This pattern can be observed in certain integration
tests, drivers, and session elements. For this use case, the Component
Framework team must provide a solution for customers to start components.

## Design

Component Framework will introduce a new framework-provided protocol,
`fuchsia.component.Binder`. This capability will allow authors to declare
components as directly bindable. Components wishing to start other components
then use that protocol as they would do for any other capability. Component
Manager will be on the server end of this protocol, and once a connection is
made, it will start the component exposing this capability. The target
component's termination can be captured by observing the `ZX_CHANNEL_PEER_CLOSED`
on the client end of the connection.

```fidl
library fuchsia.component;

/// A framework-provided protocol that allows components that use it to start the
/// component that exposes it.
///
/// Note: The component doesn't need to serve this protocol, it is implemented
/// by the framework.
[Discoverable]
protocol Binder {};
```

Component authors need only expose the protocol:

```json5
{
    expose: [
        {
            protocol: "fuchsia.component.Binder",
            from: "framework", // Note that this is implemented by the framework on the component's behalf.
        },
    ],
}
```

Components that will start such components will do so by binding to the exposed
capability:

```json5
{
    use: [
        {
            protocol: "fuchsia.component.Binder",
            from: "parent",
        },
    ],
}
```

The major benefit of this proposal is that direct starting becomes part of a
component's API. Components that are directly startable, especially those that
don't expose capabilities, can be audited in manifest files. Furthermore,
starting another component isn't restricted to parent components starting
direct children. Instead, the `fuchsia.component.Binder` protocol can be
routed anywhere like other capabilities.

## Implementation

Implementing this design won't require many changes. Introducing this protocol
should take one or two Gerrit changes.

After this feature is published, users of `BindChild` will be migrated to
using `OpenExposedDir` or the proposed `fuchsia.component.Binder` protocol
depending on their use case.

Once all use cases of `BindChild` have been migrated, the method will be
removed from the `fuchsia.sys2.Realm` protocol.

## Performance

This protocol will add a performance regression. Currently, parent components
may call `BindChild` to start child components. After this proposal, parent
components will have to call `OpenExposedDir`, _then_ open `fuchsia.component.Bind`
to achieve the same effect. However, this regression should be nominal as these
events are rare anyway.

## Security considerations

This protocol shouldn't raise security concerns. This protocol will be routable,
so which components start which other components are auditable by inspecting
the manifest files.

## Privacy considerations

This protocol shouldn't raise privacy concerns as this just allows a mechanism
to start other components.

## Testing

This feature will be tested with unit and integration tests.

Furthermore, this feature will allow component authors to test the side-effects
of other components more easily. For example, in Diagnostics, several
integration tests assert the state of an Inspect VMO. In such cases,
the driver component starts a puppet component that manipulates the VMO and
then the driver component asserts on the contents of the VMO. The proposed
feature will allow the driver component to start the puppet component without
requiring extraneous FIDL protocols.

## Documentation

The FIDL API will be documented and this feature will be documented more
broadly in the [Component Framework Realm Docs][doc-realm].

## Drawbacks, alternatives, and unknowns

### Single-run Components
A related [RFC][single-run-rfc] has recently been accepted that will also allow
components to start child components. That proposal will be able to satisfy some
of the use cases filled by `BindChild`, but not all. Notably, components
started via the proposed mechanism must reside in a collection, whereas
`fuchsia.component.Binder` will work for all components.

### fuchsia.sys2.Realm/StartChild

Another option is to extend the`fuchsia.sys2.Realm` protocol by adding a method
to start a child. This method, `StartChild`, will take one parameter,
`ChildRef`, that will be a reference to either a statically-declared child or
dynamically-created one (collections). This is the same parameter that
`BindChild` receives and that `OpenExposedDir` will take. This method will
return a Zircon Event [object][doc-event] that will be signaled once the child
component stops. The lifecycle of the child component will be tied to the
parent component. If the parent component is stopped, then the child will be
stopped as well.

```
library fuchsia.sys2;

[Discoverable]
protocol Realm {
    /// Start child component instance, if it isn't already. Returns a Zircon
    /// Event object that clients can use to observe when the child component stops.
    /// When the child component stops, Component Manager will set this object's
    /// ZX_EVENT_SIGNALED bit.
    StartChild(ChildRef child) -> (zx.handle:EVENT event) error fuchsia.component.Error;
};
```

Ultimately, such an alternative is undesirable because it allows any components
to be directly startable. Direct starting, as opposed to starting as a side-effect of a
capability binding, should only be allowed if a component declares it.
Since `StartChild` would take a `ChildRef`, it'll allow *any* component to be
started. With collections, what components are started is only observable at
runtime.

### Bind Method

Lastly, instead of introducing the empty Binder protocol proposed above, a
method can be used to trigger the start.

```
library fuchsia.component;

[Discoverable]
protocol Binder {
    /// Start the associated component instance.
    /// This method is reentrant and safe for concurrency.
    /// Calling `Bind` on an already-binded component is a no-op.
    /// When the child component stops, the `ZX_EVENTPAIR_PEER_CLOSED` signal
    /// will be asserted on the `event` object.
    Bind(zx.handle:EVENTPAIR event) -> () error fuchsia.component.Error;
};
```
While this option makes the start trigger more explicit, i.e. as a response
to a method call as opposed to connection to a protocol, it's less viable
than the proposal because it adds extra hurdles for clients. The have to setup
an event pair object before calling the method without getting any added
functionality.

### Binder Capability
Component Framework can also introduce a new capability that is backed by the
proposed `fuchsia.component.Binder` protocol.

```
// a.cml
{
    capabilities: [
        {
            binder: "a",
        },
    ],
    expose: [
        {
            binder: "a",
            from: "self",
        },
    ],
}
```
A new capability provides many benefits, such as:

1. A distinct type helps distinguish startup capabilities better. For example,
we can promote naming conventions for binder capabilities that diverge from
protocols.
1. Consistency with other capabilities in terms of what translates to a
bind (connecting == bind)
1. No chance of forgetting the `framework` keyword (although we could add cmc
checks for this).

However, despite the benefits, introducing a new capability adds conceptual
overhead, i.e. a new thing for developers to learn and understand and for CF to
support. A framework-provided protocol is more straightforward and easier to
conceptualize.

[doc-realm]: /docs/concepts/components/v2/realms.md
[doc-event]: /docs/reference/syscalls/event_create.md
[fidl-realm]: https://fuchsia.dev/reference/fidl/fuchsia.sys2#Realm
[fidl-directory]: https://fuchsia.dev/reference/fidl/fuchsia.io#Directory
[fxr-realm]: https://fuchsia-review.googlesource.com/c/fuchsia/+/531142
[single-run-rfc]: /docs/contribute/governance/rfcs/0101_dynamic_components_with_numbered_handles.md