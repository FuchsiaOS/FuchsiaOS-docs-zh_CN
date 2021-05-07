## Scenic Views, view focus, and focus chain

## View focus

When a user starts interacting with a region of the UI, Fuchsia's input system
treats it as a statement of intent. To reflect that intent, Scenic defines a
concept of "view focus", where one Scenic View is determined to "have view
focus". Only one view may have view focus at any time.

Note: Other notions of focus may co-exist on Fuchsia, at different
granularities. One example is "accessibility focus".

Scenic notifies a newly-focused view with a "view focus gained" event, and
notifies a previously-focused view with a "view focus lost" event.

Clients may use view focus to drive interactions, such as hover animations or
cursor blinking. Manager programs may use view focus to coordinate interactions,
such as determining which client may use the IME, or determining which shortcut
to fire.

## Transfer of view focus

### Initiated by user input

View focus can shift from view to view, as the user interacts with the UI.
Scenic's policy is to transfer view focus when a pointer starts interacting with
a view. The authority to transfer view focus to an arbitrary view arises from
the user (more specifically, the user's input).

For touch devices, the start of a finger's touch triggers a view focus transfer.

For mouse devices, a primary button click triggers a view focus transfer. This
policy follows the UI principle of "direct manipulation".

Note: Scenic does not trigger a view focus transfer with mouse hover.

### Initiated programmatically

In some cases, it is desirable to shift view focus in a programmatic manner. For
example, the System UI may wish to start the UI with view focus on a view that
owns a search box. Another example is support for alt-tab keyboard navigation,
where each invocation of alt-tab cycles through a set of views and transfers
view focus to each in turn.

Clearly, the capability to programmatically transfer view focus is a powerful
one. To limit abuses of power, Scenic defines a view focus transfer policy tied
to the [view tree](view_ref.md). Informally, view focus transfer is scoped to
the requestor view's subtree, hence requestor and requestee are related by view
tree ancestry.

The policy details are listed below, but requires exposition of the focus chain.

## Focus chain: focus path in a view tree

If we think about just the view parts of the scene graph, they form a tree
hierarchy; we call this the view tree. Parent views have tremendous power over
child views: the power to reposition the child's view, enforce clip boundaries
on the child's view, hide the child's UI content, etc. Because of the inherent
power of the view hierarchy, we use it as a basis of hierarchy outside of
Scenic. This hierarchy, which changes dynamically based on view focusing, is
represented with a "focus chain".

In a data structure sense, a focus chain is merely a vector of ViewRefs. The
canonical definition is
[fuchsia.ui.focus.FocusChain](/sdk/fidl/fuchsia.ui.focus/focus_chain.fidl).

However, it's more than just that. The focus chain is tied intimately to the
view tree. It captures a snapshot of view authority, starting from the root
view, walking down through descendants, and terminating at the focused view. It
is thus ordered in amount of authority, from most to least. Views outside the
focus chain have no view-based authority.

Intuitively, the focus chain is the path from view tree's root node to the
focused node.

When focus is transferred to another view, the focus chain changes. Scenic
manages these changes to the focus chain, as part of managing the view tree and
view focus.

A focus chain holds sensitive information, since it embeds ViewRefs and encodes
hierarchical information about the view tree. Hence, access to the focus chain
must be restricted to trusted entities, such as Accessibility Manager.

### Race between view focus event and focus chain

We note that the view focus event, dispatched to UI clients, and the focus
chain, dispatched to manager programs, can race. There is no global ordering
between these events. Fuchsia components are typically exposed to this
distributed system complexity as part of manipulating discrete FIDL protocols.

## Transfer of view focus: policy

Transfer of view focus can be thought of as a combination of three operations,
described below. We talk about a focus chain `F`, with ViewRefs `[v0, v1, ...,
vi, ..., vn]`, where view focus is currently at `vn` (the terminal element). The
`RequestFocus` method does not require access to the focus chain to function
properly; the focus chain is expository and not part of the transfer interface.

### Take view focus

An ancestor view can take view focus from a descendant view: `vi` may invoke
`RequestFocus(vi)` and Scenic will honor the request: `vi` will receive a view
focus event. The focus chain then becomes `[v0, v1, v2, ..., vi]`, where the
successor elements after `vi` are simply dropped.

This operation pulls view focus "up" the view tree to the caller; note that the
caller remains in the focus chain.

### Grant view focus

A focused view can grant view focus to a descendant view: if `vm` is a
descendant of `vn`, then `vn` may invoke `RequestFocus(vm)` and Scenic will
honor the request: `vm` will receive a view focus event. The focus chain then
becomes `[v0, v1, v2, ..., vi, ..., vn, ..., vm]`, where successor elements are
appended to `vn`.

This operation pushes view focus "down" the view tree from the caller; note that
the caller remains in the focus chain.

### Release view focus

A view in the focus chain can release view focus to its direct ancestor: if `vi`
has descendants terminating at `vn`, and `vi` has a direct ancestor `vh`, then
`vi` may invoke `RequestFocus()` (with no argument) and Scenic will honor the
request: `vh` will receive a view focus event. The focus chain then becomes
`[v0, v1, ..., vh]`, where the successor elements starting with `vi` are simply
dropped.

This operation is similar to a stack pop, since the caller disppears from the
focus chain.

While Take and Grant are more obvious, Release needs some explanation. An
explicit, voluntary Release of view focus may be desired for various reasons,
such as:

-   Hiding elements of the UI, without necessarily destroying them
-   Dismissal of a modal dialog, as part of returning view focus to where it was
    previously
-   User taps on a non-interactive part of the UI, indicating they want view
    focus to return to where it was previously

Today, Release is an implicit operation, handled by Scenic during lifecycle
events, such as view disconnect or view destruction.

### Composition

Take, Grant, and Release can happen in arbitrary order, as long as the
preconditions hold for each operation.

## Out of scope

We have simplified the problem space by considering these topics "out of scope":

-   Modal dialogs and focus stealing. When a view wishes to create a modal
    dialog, instead of creating a sub view or creating a UI rectangle in its
    view, it should apply to higher authority, such as System UI, to create a
    modal dialog view on its behalf. System UI can then grant view focus to that
    modal dialog, along with a scrim that prevents the user's touch from moving
    the view focus away. With this setup, a view not in the focus chain cannot
    steal the modal dialog's view focus.
-   Secure input. The system must guarantee the integrity of the "secure input"
    dispatch path from the kernel to the client.
