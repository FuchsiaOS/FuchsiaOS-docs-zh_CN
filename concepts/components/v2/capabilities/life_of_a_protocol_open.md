# Life of a protocol open

This document describes the steps that occur when a component attempts to
connect to a protocol in its namespace.

These steps apply to the [Components v2][glossary.components-v2] model as run
under component manager.
Portions of it also apply to the [Components v1][glossary.components-v1] model
as run under appmgr.

At a high level these steps are:

-   Component manager will [construct a component's namespace][ns-construction]
    based on the `use` declarations in its manifest.
-   Once running, a component will attempt to [open a protocol][protocol-open]
    in its namespace.
-   This `Open` request is received by component manager, which performs the
    [capability routing][cap-routing] necessary to find the component providing
    the protocol.
-   Component manager [binds to the component providing the protocol][binding]
    and connects the `Open` request to it

## Constructing a component's namespace

A [_namespace_][namespaces] is a set of directories that are offered to a
component when it is started. Each directory is associated with a file system
path through which the component may access files and protocols offered by other
components.

These directories take the form of [handles][handle] to [channels][channel],
over which the component can use
[the `fuchsia.io.Directory` FIDL protocol][fuchsia.io].

For example, all components will receive a handle to the contents of the package
from which they were created at `/pkg`. This means that a component can see what
binaries are available in their package by reading the contents of `/pkg/bin`.

The `use` declarations in [the component's manifest][component-manifests]
determine how the namespace is populated. When a protocol capability is used...

```json5
use: [
    {
        protocol: "fuchsia.example.Foo",
    },
]
```

...component manager will add an entry to the component's namespace for the
parent directory of the protocol. In this example, the namespace path of the
protocol is `/svc/fuchsia.example.Foo` (the default path assignment), which
means that component manager will add a handle for `/svc` to the namespace.

The `/svc` directory is provided by component manager itself, and component
manager will respond to requests for protocols to this directory for the
lifetime of the component.

The exact semantics of what appears in the namespace varies based on capability
type. For example if a directory capability is used instead of the protocol
capability...

```json5
use: [
    {
        directory: "example-data",
        rights: [ "r*" ],
        path: "/example/data",
    },
]
```

...a handle for the directory itself appears in the namespace instead of a
handle for the parent directory. In this example, this means that a handle for
`/example/data` will appear in the namespace, whereas if this path was used for
a protocol capability `/example` would appear in the namespace.

## A component opens a protocol

When a component wants to open a protocol it creates a new channel pair, and
sends one end of this pair via an `Open` request over a channel in its
namespace. For example, if the component wanted to open a connection to
`/svc/fuchsia.example.Foo`, one end of the new channel pair would be sent over
the `/svc` handle in its namespace. The component may then call the
`fuchsia.example.Foo` protocol over the channel.

Since the directory containing the protocol (`/svc`) is provided by component
manager, it is component manager that will receive the server end of the new
channel via the `Open` request sent by the component. Component manager then
must identify the component providing the protocol over this channel.

## The `Open` triggers capability routing

To determine the component that provides the protocol over the channel,
component manager must walk the tree of components, following `offer` and
`expose` declarations to find the capability's source. This process is referred
to as _capability routing_.

Starting at the parent of the component that triggered the capability routing,
component manager will inspect each component's manifest, looking for an `offer`
declaration whose destination matches the child. The offer will specify a source
of either `parent`, `self`, or the name of a child. If the offer came from the
component's realm it will continue to walk up the tree, and if the offer came
from one of the component's children it will walk down the tree to that child.

Once the routing begins walking down the tree it will look for `expose`
declarations, which will specify a source of either `self` or the name of a
child. If the capability came from a child then component manager will continue
to walk down the tree.

Once an `offer` or `expose` declaration with a source of `self` is found, then
component manager can hand off the channel to that component.

If at any step of the way the chain is invalid, component manager will log an
error and close the channel it received from the `Open` call. This can be caused
by various situations, such as:

-   A component `C` offered a capability from `parent`, but its parent `R` did
    not offer the capability to `C`.
-   A component `C` offered a capability from its child `D`, but child `D` did
    not expose the capability to `C`.

For example, consider the following tree of components and their manifests
(`program` blocks and runner setup omitted for brevity):

```
    C
   / \
  B   D
 /
A

A.cml:
{
    // ...
    capabilities: [
        {
            protocol: "fuchsia.example.Foo",
        },
    ],
    expose: [
        {
            protocol: "fuchsia.example.Foo",
            from: "self",
        },
    ],
}

B.cml:
{
    // ...
    expose: [
        {
            protocol: "fuchsia.example.Foo",
            from: "#A",
        },
    ],
    children: [
        {
            name: "A",
            url: "fuchsia-pkg://fuchsia.com/a#meta/a.cm",
        },
    ]
}

C.cml:
{
    // ...
    offer: [
        {
            protocol: "fuchsia.example.Foo",
            from: "#B",
            to: [ "#D" ],
        },
    ]
    children: [
        {
            name: "B",
            url: "fuchsia-pkg://fuchsia.com/b#meta/b.cm",
        },
        {
            name: "D",
            url: "fuchsia-pkg://fuchsia.com/d#meta/d.cm",
        },
    ]
}

D.cml:
{
    // ...
    use: [
        {
            protocol: "fuchsia.example.Foo",
        },
    ],
}
```

When `D` calls `Open` on `/svc/fuchsia.example.Foo` in its namespace, component
manager will walk the tree to find the component that should provide this
protocol. It will start at `D`'s parent, `C`, and from there:

-   Look for the `offer` declaration for `fuchsia.example.Foo` to `D`, and see
    that it comes from child `B`.
-   Look for the `expose` declaration for `fuchsia.example.Foo` from `B`, and
    see that it comes from `A`.
-   Look for the `expose` declaration for `fuchsia.example.Foo` from `A`, and
    see that it comes from `self`. This means that `A` is the component
    providing the capability that `D` is attempting to use.

Now that the provider component has been found, component manager can attempt to
hand off the channel it received via the `Open` request.

## Binding to a component and sending a protocol channel

With the provider found the client component is now bound to the provider. This
will cause the component to start running if it is currently stopped.

Every component upon being started receives a server handle to an
[outgoing directory][glossary.outgoing-directory] in its handle table.
When a component is bound, component manager forwards the server end of the
protocol channel to the providing component's outgoing directory, under the
source path in the providing component's `offer` or `expose` declaration.

In the above example component manager will send an `Open` request over the
outgoing directory handle for component `A` to the `/svc/fuchsia.example.Foo`
path, providing the channel handle that it received from component `D` when it
called `Open` to component manager.

It is then up to component `A` to receive this request and start responding to
messages over the channel it was given.

Since component manager directly forwards the server end of the protocol channel
to the provider component's outgoing directory, it is not involved in message
proxying and is entirely out of the picture after capability routing is
completed. Once a connection to another component is established, they talk
directly to each other with no arbiter in the middle.

## Caveats

### Runtime unpredictability

Due to the runtime nature of capability routing and the behavior of the
components providing capabilities, there is no way to know if a given component
can successfully access a capability in its namespace before it attempts to do
so. Even if a valid offer/expose chain exists for the capability, package
updates could break this chain at runtime, and it's entirely possible a
component that claims to provide a capability in its manifest will fail to do so
when run.

### Offered vs ambient capabilities

Some capabilities are provided by the component framework itself, and can be
directly used by (or will be implicitly provided to) components without their
parent offering these capabilities. Currently these are:

-   `/pkg`: a handle to the package from which the component was created.
-   [`/svc/fuchsia.component.Realm`][realm.fidl]: a protocol which components can use
    to manage their own realm.

[binding]: #binding-to-a-component-and-sending-a-protocol-channel
[cap-routing]: #the-open-triggers-capability-routing
[channel]: /docs/reference/kernel_objects/channel.md
[component-manifests]: /docs/concepts/components/v2/component_manifests.md
[fuchsia.io]: https://fuchsia.dev/reference/fidl/fuchsia.io
[glossary.components-v1]: /docs/glossary/README.md#components-v1
[glossary.components-v2]: /docs/glossary/README.md#components-v2
[glossary.outgoing-directory]: /docs/glossary/README.md#outgoing-directory
[handle]: /docs/concepts/kernel/handles.md
[namespaces]: /docs/concepts/process/namespaces.md
[ns-construction]: #constructing-a-components-namespace
[protocol-open]: #a-component-opens-a-protocol
[realm.fidl]: /sdk/fidl/fuchsia.component/realm.fidl
