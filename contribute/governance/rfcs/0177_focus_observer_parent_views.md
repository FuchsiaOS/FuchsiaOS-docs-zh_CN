<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0177" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
{# Fuchsia RFCs use templates to display various fields from _rfcs.yaml. View the #}
{# fully rendered RFCs at https://fuchsia.dev/fuchsia-src/contribute/governance/rfcs #}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Summary

This RFC proposes an API design for [view focus][view-focus] that is safe to use
out of tree, by ordinary [UI clients][ui-client], and clarifies the security
constraints around focus observability. The emphasis is on minimality of
information exposure and an elegant developer experience.

## Motivation

In order to create a user experience (graphics, input, etc) from multiple
components, it is a common pattern for [UI clients][ui-client] to delegate
content production to other UI clients by setting up a view tree, where a
[parent view][child-views] manages one or more child views. The Ermine system
shell is one such example; Google's Smart Display is another. One key
responsibility of a parent view is to monitor [view focus][view-focus] state:

1.  To identify *when* the parent view may programmatically move view focus to a
    child view.
    *   For example, a parent view's request to move focus to its child view
        fails if the parent's view is not in the view tree's
        [focus chain][focus-chain] ([fxbug.dev/87577](https://fxbug.dev/87577)).
1.  If view focus moved to a child view, to identify *which child* presently has
    view focus.
    *   For example, if the user moved focus to a view by touching it, a parent
        view may want to decorate that child view with a focus boundary, and
        needs to know both when it happened and the child view's identity.

Focus may change without the parent view's involvement (user touch, view detach,
etc). The parent view must be kept informed of how view focus moves around, but
in a way that respects the [information limits][view-tree-security] set up by
the global view tree.

This RFC proposes a "focus observer" design which (1) allows a parent view to
correctly respond to view focus changes, (2) is safe to use out of tree, and (3)
improves the security posture of the Fuchsia View system, with minimal
information exposure.

## Stakeholders

*Facilitator:*

*Reviewers:* sanjayc@google.com (Workstation), quiche@google.com (HCI),
neelsa@google.com (HCI), akbiggs@google.com (Flutter)

*Consulted:* shiveshganju@google.com, fmil@google.com, emircan@google.com,
jsankey@google.com

*Socialization:*

This RFC went through socialization with leads of affected teams.

## Requirements

*   Minimal information exposure of focus observability
*   Well-defined security mechanism to obtain an observation channel
*   SDK inclusion to "partner" level or higher
*   Ease of developer experience

## Design

The core proposal for this focus observer is the following FIDL protocol.

```
library fuchsia.ui.observation.focus;
using zx;

protocol ScopedProvider {
  Watch() -> (ScopedResponse);
};

type ScopedResponse = table {
  1: observation_end zx.time;
  2: focused zx.koid;
};
```

The "Scoped" in the name indicates that the protocol provides focus information
that is scoped to, or constrained within, the focus.ScopedProvider client's view
tree. The focus.ScopedProvider client's view is the root of this observable view
tree.

The `observation_end` time marks the end of the Watch period, so that the client
knows when the returned focus was accurate. For example, it allows the client to
distinguish between distinct returns of the same focus value, if a series of
focus changes happened to return back to a previous focus in a single Watch
period.

The `focused` KOID is either a view ref KOID, or the
[special sentinel value][koid-invalid] `ZX_KOID_INVALID` that indicates view
focus is outside the focus.ScopedProvider client's view tree. The possible
values and semantics are discussed in more detail below.

### Example view topology

Consider the following view topology, where each circle represents a View, and
view "U" is a client of focus.ScopedProvider.

<!-- mdformat off(alt text) -->
![L1 Example view topology.
  L2 Nodes U, V, W, X, Y.
  L3 U parent of V and W.
  L4 V parent of X and Y.
  L5 U has unspecified parent in a larger triangle labeled "rest of view tree".
](resources/0177_focus_observer_parent_views/local-view-tree.png)
<!-- mdformat on -->

### Focus visibility scoped to view tree

A client of focus.ScopedProvider has limited visibility into the global view
tree (see Security Considerations for details). It can learn that view focus is
either *in its view tree* (rooted at the focus.ScopedProvider client's view) or
*outside its view tree*, but the specifics are intentionally elided.

When focus is outside the focus.ScopedProvider client's view tree, the client is
informed of only of that very general fact, with the `ZX_KOID_INVALID` sentinel
value. The client does not learn the identity of the new view focus.

Wen focus is held within the focus.ScopedProvider client's view tree, the client
is informed of only the following information:

*   If focus is on the focus.ScopedProvider client's view itself, then the KOID
    of the client's view.
*   If focus is on a direct child view of the client's view, then the KOID of
    that direct child view.
*   If focus is on an indirect child view of the client's view, then the KOID of
    the direct child view which is the ancestor of that indirect child view.

The parent view needs to know *when* it has the power to move focus amongst its
children. It has this power when view focus is *in its view tree*. Otherwise, a
call to fuchsia.ui.views.Focuser.RequestFocus() will always fail.

It's worth noting that the focus.ScopedProvider's information is a snapshot
propagated over a channel, so a request to change focus may race with the next
snapshot update. For example, one snapshot might indicate that focus is in the
focus.ScopedProvider client's view tree, and a request to change focus to a
direct child may get denied if an ancestor view successfully requested a focus
change to outside this view tree.

In this sequence diagram, U is notified when focus moved to U, and again when
focus moved entirely out of U's view tree.

<!-- mdformat off(alt text) -->
![L1 Title: Focus observer usage.
  L2 participant U.
  L3 participant ScopedProvider as S.
  L4 U -> S: Watch.
  L5 Note right of U: focus moved to U.
  L6 S -> U: response(U).
  L7 U -> S: Watch.
  L8 Note right of U: focus moved outside of U.
  L9 S -> U: response("invalid").
  L10 U -> S: Watch.
  L11 Note right of U: waiting for focus change.
](resources/0177_focus_observer_parent_views/sequence-in-u-out-u.png)
<!-- mdformat on -->

### Focus values reported to client

`focused` is one of three value classes, which includes the `ZX_KOID_INVALID`
sentinel value. If `focused` is valid (i.e., not the sentinel), then the view
has the power to move focus arbitrarily between itself and its child views.

Specifically:

*   If `focused` is `ZX_KOID_INVALID`, then focus has left this view tree. This
    situation can arise for multiple reasons. For example, the view tree at U
    might be connected to the global view tree, but an ancestor view may have
    moved focus out, to U's sibling view. Or, U might have become disconnected
    from the global view tree, meaning U is no longer eligible to hold focus.
    Or, an ancestor of U may itself be disconnected, in which case all
    descendants of that ancestor cannot hold focus. See
    [focus policy][focus-policy].
*   If it is the parent's view ref KOID, then the parent view itself has focus.
    This usage is identical to [fuchsia.ui.views.ViewRefFocused][vrf-api], which
    allows us to deprecate that protocol.
*   If it is a KOID that is not invalid or the parent, it is a direct child's
    view ref KOID. Only direct children are mentioned in this field, *even if*
    the focused view is a descendant of a direct child.

In this example, focused moved to X, a child of V under U. The focus observer
reports the direct child of U, which is V.

<!-- mdformat off(alt text) -->
![L1 Title: Focus observer usage.
  L2 participant U.
  L3 participant ScopedProvider as S.
  L4 U -> S: Watch.
  L5 Note right of U: focus moved to X.
  L6 S -> U: response(V).
  L7 U -> S: Watch.
  L8 Note right of U: waiting for focus change.
](resources/0177_focus_observer_parent_views/sequence-to-x-observe-v.png)
<!-- mdformat on -->

### Summary semantics

If there were multiple focus changes during the past Watch period, this API will
return only the final focus. A client typically cannot act on past focus
changes, hence the API was simplified to return just a "summary".

Typically, if a hanging-get client parks a callback via Watch, a focus change
will result in an immediate return to the client. However, it's possible for a
client to get delayed parking the next hanging-get, so the server may see
multiple focus changes to summarize on the next return. It's also possible for
the server to receive a flurry of focus changes, so depending on thread or task
scheduling, a parked hanging-get may get serviced after a number of focus
changes.

In these examples, U gets the same notification, regardless of specific Watch()
call timing.

<!-- mdformat off(alt text) -->
![L1 Title: Focus observer usage.
  L2 participant U.
  L3 participant ScopedProvider as S.
  L4 U -> S: Watch.
  L5 Note right of U: focus moved to X.
  L6 Note right of U: focus moved to Y.
  L7 Note right of U: focus moved to W.
  L8 S -> U: response(W).
  L9 U -> S: Watch.
  L10 Note right of U: waiting for focus change.
](resources/0177_focus_observer_parent_views/sequence-watch-then-multiple-change.png)
<!-- mdformat on -->

<!-- mdformat off(alt text) -->
![L1 Title: Focus observer usage.
  L2 participant U.
  L3 participant ScopedProvider as S.
  L4 Note right of U: focus moved to X.
  L5 Note right of U: focus moved to Y.
  L6 Note right of U: focus moved to W.
  L7 U -> S: Watch.
  L8 S -> U: response(W).
  L9 U -> S: Watch.
  L10 Note right of U: waiting for focus change.
](resources/0177_focus_observer_parent_views/sequence-multiple-change-then-watch.png)
<!-- mdformat on -->

### State-change semantics

The Watch call is driven by state changes on a per-client basis.

*   On the first call after connection, the current state is returned
    immediately.
*   Between the return of one Watch call and the start of the next Watch call,
    if multiple state changes happened, the next Watch call will return
    immediately with the last change observed.

With level changes, the server notifies the client only when a change happens
*after* a Watch call was received, and ignores changes prior to receiving a
Watch call. The client would miss a focus change summary during that period,
which is not appropriate for the intended use case.

Making it based on state change creates a larger burden on the server
implementation, since it needs to track the last issued state for each observer
channel. However, it leads to a more intuitive developer experience, since state
change is robust against ordering swaps between the client's Watch call and any
focus changes. For example, after a Watch call is parked on the server, several
focus changes can happen in the interim before the callback is processed,
depending on the server's threadedness and implementation details.

## Implementation

View focus is closely tied to view lifecycle and maintenance of view topology.
Scenic is the view manager component, so the implementation of this protocol
belongs in Scenic.

## Performance

Focus changes can be frequent, but realistically move at "human scale". Thus
FIDL call frequency is not perceived to be an issue. The FIDL payload is also
very light, and the flow control pattern avoids channel stuffing.

## Ergonomics

This API strives to improve DX over its predecessors. Simplified error handling,
lossy summary semantics, and the absence of container data types should mean
easier adoption.

## Evolution

This API is intended to be consumed in OOT repositories, and the server
implementation resides in a platform component, Scenic. The API will evolve
safely, and retain backward compatibility, by adding newer hanging-get methods.
When all repositories that use a deprecated method get updated to a newer
method, the deprecated method can be marked as deleted.

## Security considerations

This API hooks into the fuchsia.ui.composition.Flatland.ViewBoundProtocols
table, which tightly associates this API's server endpoint to the specific
ViewRef associated with the parent view, at view creation time.

<!-- mdformat off(alt text) -->
![L1 Title: Focus observer hookup.
  L2 participant UI client as U.
  L3 participant Flatland as S.
  L4 participant focus provider as F.
  L5 Note right of U: 'U': view ref for view U.
  L6 U -> S: Flatland.CreateView2(U, server_end:f.u.o.focus.ScopedProvider).
  L7 U -> S: Flatland.Present().
  L8 U -> F: F: f.u.o.focus.ScopedProvider.Watch().
  L9 Note right of U: waiting for focus change.
](resources/0177_focus_observer_parent_views/sequence-create-view-hookup.png)
<!-- mdformat on -->

The API client cannot ask for more detailed information deep in its own view
tree, or outside its view tree. The view ref KOID information received is scoped
to itself and its direct children, which improves the view's security posture.

### Focus stealing

In a more permissive system, a malicious view could "steal" focus from any other
view merely by asking for it. The Fuchsia View system's
[focus policy][focus-policy] mitigates this possibility by defining the
circumstance and scope of focus movement: a view can move focus only if it was
granted focus by an ancestor view, and it can only move focus within its view
subtree, not outside of it.

This focus observer design follows the scoped approach of this focus policy, by
limiting observability to the observed view and its direct children.

### KOIDs not capabilities

Another small improvement is that the focus observer protocol hands out the
child view ref's KOID, instead of a copy of the view ref itself. Some UI
protocols act on a view ref, so returning a KOID reduces the possibility of
misuse. For example, if Ermine's focus.ScopedProvider channel endpoint was
delegated to another component 'C', it is a safe delegation, because 'C' cannot
impersonate Ermine or any child view of Ermine, to the KeyboardListener
protocol.

The typical usage is to identify which view has gained focus, for which the
view's view ref KOID is sufficient. Note that requesting focus requires a live
view ref, not just a view ref's KOID. Clients are expected to maintain their own
list of child view refs (i.e., obtained over the Flatland protocol), and these
view refs can be used to request focus.

## Privacy considerations

The [FocusChainListener][focus-chain-api] protocol gave full visibility of the
view tree, right up to the root view. This focus observer protocol intentionally
limits the scope of visibility, where the visible view tree is rooted at the
client view itself.

The [ViewRefFocused][vrf-api] is already scoped to the client's view. This focus
observer protocol extends the client's visibility to the client view's direct
child views only.

Because of these mitigations, we expect privacy impact to be minimal.

## Testing

The implementation will have unit tests and platform-side integration tests.
Additionally, as with any other SDK-visible FIDL, it will have an CTF test.

## Documentation

There will be a usage documentation guide in fuchsia.dev.

## Drawbacks, alternatives, and unknowns

This API does not fit all known usages for observing focus. However, a previous
socialization effort reinforced the need to create separate APIs for separate
needs. A follow-on RFC will tackle other "focus observer" APIs.

## Prior art and references

*   [fuchsia.ui.focus.FocusChainListener][focus-chain-api] protocol
*   [fuchsia.ui.views.ViewRefFocused][vrf-api] protocol
*   [View System RFC][view-rfc]

### Problems with the legacy FocusChainListener

Today's only option for observing view focus movement across a view tree is the
[fuchsia.ui.focus.FocusChainListener][focus-chain-api] protocol. It is
deprecated, due to these problems:

*   It gives a client *global visibility* into where view focus has moved, which
    leaks platform implementation details. For example, all the views in the
    root scene get exposed in this focus chain, which allows clients to assert
    on the root scene's structure, thus preventing platform-internal
    implementation changes.
*   It gives out [fuchsia.ui.views.ViewRef][view-ref-api] tokens (backed by
    Zircon eventpair objects), which allow clients of weaker protocols to pose
    as another view.

[child-views]: /docs/contribute/governance/rfcs/0147_view_system.md#child_views
[focus-chain-api]: https://fuchsia.googlesource.com/fuchsia/+/refs/heads/main/sdk/fidl/fuchsia.ui.focus/focus_chain.fidl
[focus-chain]: /docs/development/graphics/scenic/concepts/focus_chain.md
[focus-policy]: /docs/development/graphics/scenic/concepts/focus_chain.md#transfer_of_view_focus_policy
[koid-invalid]: /docs/concepts/kernel/concepts.md#kernel_object_ids
[ui-client]: /docs/concepts/ui/ui-client.md
[view-focus]:  /docs/development/graphics/scenic/concepts/focus_chain.md#view_focus
[view-ref-api]: https://fuchsia.dev/reference/fidl/fuchsia.ui.views#ViewRef
[view-rfc]: /docs/contribute/governance/rfcs/0147_view_system.md
[view-tree-security]: /docs/contribute/governance/rfcs/0147_view_system.md#view-security
[vrf-api]: https://fuchsia.dev/reference/fidl/fuchsia.ui.views.md#ViewRefFocused
