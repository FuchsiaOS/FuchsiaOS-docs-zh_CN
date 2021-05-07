# Component lifecycle (Components v2)

<<../_v2_banner.md>>

Component instances progress through four major lifecycle events: create, start,
stop, and destroy.

Component instances may retain isolated persistent state on a storage medium
while they are not running, which helps them maintain the
[illusion of continuity][principle-continuity] across restarts.

## Creating a component instance {#creating}

A component instance may be created in the following ways:

-   Configuring it as the root component of component manager.
-   Statically declaring it as the [child][doc-manifests-children] of another
    component.
-   Dynamically creating it at runtime in a [collection][doc-collections].

Every component instance has a [moniker][doc-monikers] that uniquely identifies
it, determined by its path from the root.

Once created, a component instance can then be [bound to](#binding),
[started](#starting), or [destroyed](#destroying).

## Binding to a component instance {#binding}

A component instance `A` _binds_ to another component instance `B` when `A`
connects to some capability that is provided by `B`. When this happens,
component instance `B` is started, unless it was already started. In most cases,
this is the most common reason for a component instance to [start](#starting).

Concretely, there are two ways that `A` can bind to `B`:

-   `A` connects to a capability in its namespace which is
    [exposed][doc-manifests-expose] or [offered][doc-manifests-offer] by `B`.
    This is the most common way.
-   `A` binds to one of its children using the [`Realm.BindChild`][realm.fidl]
    protocol.

The word "bind" is meant to imply that a component is run on account of being
"bound" by its clients. In theory, when no more clients are bound to a
component, the framework could stop running it, but this behavior isn't
currently implemented.

## Starting a component instance {#starting}

Starting a component instance loads and runs the component's program and
provides it access to the capabilities that it requires.

[Every component runs for a reason][principle-accountability]. The component
framework only starts a component instance when it has work to do, such as when
another component requests to use its instance's capabilities.

Once started, a component instance continues to run until it is
[stopped](#stopping).

## Stopping a component instance {#stopping}

Stopping a component instance terminates the component's program but preserves
its [persistent state][doc-storage] so that it can continue where it left off
when subsequently restarted.

The component framework may stop a component instance for the following reasons:

-   When the component is about to be destroyed.
-   When the system is shutting down.

A component can implement a lifecycle handler ([example][handler-example]) to be
notified of its impending termination and other events on a best effort basis.
Note that a component can be terminated involuntarily and without notice in
circumstances such as resource exhaustion, crashes, or power failure.

Components can stop themselves by exiting. The means by which a component exits
depend on the runner that runs the component.

Once stopped, a component instance can then be [restarted](#starting) or
[destroyed](#destroying).

### Destroying a component instance {#destroying}

Destroying a component instance permanently deletes all of its associated state
and releases the system resources it consumed.

Once destroyed, a component instance ceases to exist and cannot be restarted.
New instances of the same component can still be created but they will each have
their own identity and state distinct from all prior instances.

## Legacy features {#legacy}

### Eager binding {#eager}

[Component manifests][doc-manifests] let you mark a child as
[`eager`][doc-manifests-children], which causes the component framework to
implicitly bind to that child when any component binds to the parent. In other
words, this causes the child to be immediately started whenever the parent is
started. This is a legacy feature, and additional uses should be limited to
tests. The future of this feature is being tracked at
[fxb/61721](https://fxbug.dev/61721).

`eager` primarily has two uses: to start the first component, and as a
convenience in tests to run components without having to explicitly bind to
them.

If the eager child fails to start for any reason (such as a missing component),
component manager exhibits the following behavior:

-   If the parent is not the root component, the parent will start but the
    component that bound to it will observe a dropped connection (just like any
    other failed binding).
-   If the parent is the root component, component manager will crash.

[doc-collections]: realms.md#collections
[doc-lifecycle]: lifecycle.md
[doc-manifests-children]: component_manifests.md#children
[doc-manifests-expose]: component_manifests.md#expose
[doc-manifests-offer]: component_manifests.md#offer
[doc-manifests]: component_manifests.md
[doc-monikers]: monikers.md
[doc-storage]: capabilities/storage.md
[doc-topology]: topology.md
[handler-example]: /examples/components/basic/src/lifecycle_full.rs
[principle-accountability]: design_principles.md#accountability
[principle-continuity]: design_principles.md#illusion-of-continuity
[realm.fidl]: https://fuchsia.dev/reference/fidl/fuchsia.sys2#Realm
