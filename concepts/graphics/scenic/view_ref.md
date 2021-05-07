## Scenic Views, view tree, and ViewRefs

### Scenic View

A UI client that wishes to vend user-visible content must place it in a Scenic
View, which is a [Session](scenic.md#sessions)-based
[resource](scenic.md#resources), local to that client. No resource can be
directly referenced outside the scope of their owning Scenic Session.

### ViewRef refers to a Scenic View

For a view resource in particular, it is very useful to have a stable and
consistent view reference, that can be used across
[component](/docs/concepts/components/v2/introduction.md#components_and_the_component_framework)
boundaries.

We define a [fuchsia.ui.views.ViewRef](/sdk/fidl/fuchsia.ui.views/view_ref.fidl)
FIDL datatype that has some desirable properties:

*   A ViewRef is globally unique across the operating system, it cannot be
    forged. This property arises from the ViewRef's underlying kernel object, an
    [eventpair](/docs/concepts/kernel/concepts.md#events_event_pairs).
*   A ViewRef is globally unique over the lifetime of the operating system, it
    is never reused. This property arises from Zircon's guarantee of
    [KOID uniqueness](/docs/concepts/kernel/concepts.md#kernel_object_ids) for
    kernel objects.
*   A ViewRef can be used in a
    [feed-forward](/docs/concepts/api/fidl.md#feed_forward-dataflow) pattern for
    Scenic View creation. Protocols and components that participate in View
    creation do not have to "tunnel back" the corresponding ViewRef.
*   ViewRef holders can implement lifecycle management by listening for a
    [`ZX_EVENTPAIR_PEER_CLOSED`](/docs/reference/syscalls/eventpair_create.md)
    zircon signal on the underlying eventpair object.

Each Scenic View has an associated ViewRef.

### View tree

The global scene graph can be thought of as a tree of Views, each containing UI
content and embedding other Views. Because we have a ViewRef for each View, we
can also think of the view tree as a tree of ViewRefs.

In a view tree, parent views have tremendous power over child views: the power
to reposition the child's view, enforce clip boundaries on the child's view,
hide the child's UI content, etc. Because of the inherent power of the view
hierarchy, we use it as a basis of hierarchy outside of Scenic. This hierarchy,
which changes dynamically based on view focusing, is represented with a
"[focus chain](focus_chain.md)".

### How is a ViewRef used?

Typically, a UI client uses a ViewRef to self-identify itself to manager-type
programs.

Because of the feed-forward nature of view creation, a manager program can start
handling View-specific logic prior to the View's creation in Scenic.

Here are some example usages. Accessibility Manager uses ViewRefs to identify
and manage client content; IME Manager uses ViewRefs to identify IME clients,
Shortcuts Manager uses ViewRefs to identify and manage client-authored keyboard
shortcuts, Sys UI uses ViewRefs to identify and manage focus of child views.

### Are ViewRef and ViewToken the same thing?

No. These FIDL datatypes are both backed by eventpairs, but serve different
uses.

The [ViewToken](/sdk/fidl/fuchsia.ui.views/view_token.fidl) is used internally
by Scenic. It connects a View resource to a corresponding ViewHolder resource in
the scene graph. The semantics work because each side of the eventpair is held
uniquely by View (child) and ViewHolder (parent).

The ViewRef is used outside of Scenic. UI clients and manager components use it
to refer to a View.

### ViewRef not for authentication

A ViewRef can easily be propagated across protocol boundaries. Hence, it is
important to *not* use a ViewRef as an authentication mechanism: merely holding
a ViewRef should not grant the holder powers. Instead, use
[capability routing](/docs/concepts/components/v2/capabilities/protocol.md) to
distribute a ViewRef-consuming protocol securely to trusted components.

One example of this is the
[fuchsia.ui.views.ViewRefInstalled](/sdk/fidl/fuchsia.ui.views/view_ref_installed.fidl)
protocol, which allows a client to determine when Scenic has installed a ViewRef
in the view tree.

### ViewRef design tradeoffs

The feedforward pattern is not inherently secure; the client is trusted to not
make a secret copy of the
[ViewRefControl](/sdk/fidl/fuchsia.ui.views/view_ref.fidl) when feeding the
ViewRef and ViewRefControl pair to Scenic.
