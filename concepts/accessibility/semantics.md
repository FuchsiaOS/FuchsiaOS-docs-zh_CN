# Semantics

## Overview

This document describes the semantics API and associated infrastructure for
accessibility on Fuchsia. This is a part of the Accessibility Manager that
gathers information about UI elements from client view on Fuchsia and makes that
information available to accessibility services such as the screen reader.

## Background

### Links

* [Flutter Semantics Documentation][flutter]
* [Chromium AXNode Implementation][chromium]

* _View_ - a [Scenic view][view] corresponding to a particular region of the
  screen on a device. This is drawn by a UI Framework.

* _UI Framework_ - for the purpose of this doc, a "UI Framework" is something
  that draws a user-visible view on Fuchsia. Typically these are Flutter or
  Chromium views. Ermine, the sysUI on Workstation, uses the Flutter UI
  Framework.

## Design

### Semantic trees

A semantic tree is an acyclic graph of nodes that corresponds to UI elements. In
aggregate, the semantic tree represents the entirety of the currently available
UI. This is a widely used concept in accessibility frameworks, for example
Chrome, Windows, iOS, Flutter, etc.

On Fuchsia, the UI may be composed of multiple Scenic views and the
accessibility framework tracks a separate semantic tree for each view on
screen. This diagram shows an example UI with multiple semantic trees.

![This figure shows the hierarchy of a Fuchsia UI from Scenic's perspective
only.

At the top are the many views that Scenic manages.  Each view is linked to a
runtime displaying some UI, including both Flutter and Chromium runtimes.

Each Flutter and Chromium runtime maintains within itself a semantic hierarchy
that would be supplied to the Accessibility Manager.](scenic_semantics.png)

Internally, both Flutter and Chromium have the ability to produce semantic trees
for UI they are rendering. The Fuchsia [accessibility
framework](accessibility_framework.md) provides an API to runtimes so that views
can expose their semantic trees to the OS.

### The semantics API

The semantics API can be found in [fuchsia.accessibility.semantics][semantics].
This API allows a UI framework to register with the Accessibility Manager to
provide semantic updates, and then call the UpdateSemanticNodes,
DeleteSemanticNodes and CommitUpdates methods to send information about its
semantics.

#### Semantic tree update sequence

Because semantic trees can be quite large (for example, on a complex website), a
change in the semantic tree may be broken up into multiple calls by the UI
framework. The Accessibility Manager stores these updates until the UI framework
calls "Commit" and then changes its local tree and performs validation. This
allows the Accessibility Manager to have a consistent (though possibly out of
date) view of the semantics that can be accessed locally.

#### UI framework contract

Anyone launching a view on Fuchsia is expected to register with the
SemanticsManager protocol and provide a SemanticListener. Typically, view owners
use a UI framework such as flutter or chromium to create the view. The
framework's Fuchsia integration is responsible for implementing Fuchsia-specific
details. The framework integration also supplies a
[ViewRef][viewref],which is a kernel object
that uniquely identifies the view.

The SemanticsManager protocol is shown below:

```fidl
[Discoverable]
protocol SemanticsManager {
   RegisterViewForSemantics(fuchsia.ui.views.ViewRef view_ref,
                            SemanticListener listener,
                            request<SemanticTree> semantic_tree_request);
};
```

The semantic listener allows the Accessibility Manager to enable or disable
semantic update and to perform actions such as hit testing. In practice, this is
handled by the Flutter and Chromium UI framework integrations which instantiate
Scenic views.

The UI framework implementation is expected to maintain this FIDL connection for
the lifetime of the view, even when semantics are disabled.

If the UI framework issues an invalid update, the Accessibility Manager will
close the channel. An update is considered invalid if committing it does not
result in a well-formed, acyclic tree. Some examples include referencing a
non-existent child node or lacking a root node.

The UI framework is responsible for re-establishing the registration for that
view (if the error is recoverable) or crash/restart (if not recoverable). Care
must be taken in the UI framework implementation to avoid repeated attempts to
connect with an invalid semantic tree.

### Hit testing

Hit testing is the process of translating a location on screen to a particular
semantic node. In order to hit test on Fuchsia, the framework must solve two
problems

* Find the view corresponding to the location
* Find the node inside the view at that location

Accessibility Manager receives pointer events from Scenic (see details in the
[accessibility input documentation][a11yinput]). Scenic annotates each pointer
event with the KOID (kernel object ID) associated with the [viewRef][viewref] of
the containing view. This allows the screen reader to map the touch to a
particular semantic tree and route the touch to the correct SemanticListener for
hit testing. The hit testing action is described below.

### Semantic listener actions

The Semantic Listener provided by the UI framework allows the accessibility
framework to execute an action in the client view. These can provide the
framework with information (for example, when using hit testing to determine
which node the user has tapped) or to make changes in the client view (for
example, using the default action to click a button). The following are
supported listener actions (see the ['Action'
enum][node] in the API):

* Hit testing - framework queries with view-local (x,y) coordinates and the
  client view returns the id for the node at that position as well as
  (optionally) the path to that node from the tree's root.
* Default Action - Takes a node ID as input. The logical equivalent of tapping
  or clicking a button.
* Show on screen - Takes a node ID as input. Scrolls UI to bring that
  node into view. The client view is responsible for determining how exactly to
  do this.
* Increment/Decrement - adjust the current value of a slider.

### Tree navigation & ordering

The accessibility framework provides basic utilities for navigating a view's
semantic tree by finding the next or previous node in the tree. This makes
linear navigation through the UI possible. This navigation happens in
depth-first order.

The caller may provide a filter function for the next/previous function to allow
iterating through all nodes of a certain type (for example headers, links,
etc.). This can also be used to skip nodes that are not "describable" according
to a given metric.


[flutter]: https://api.flutter.dev/flutter/widgets/Semantics-class.html
[chromium]: https://source.chromium.org/chromium/chromium/src/+/master:ui/accessibility/ax_node_data.h
[view]: /development/graphics/scenic/concepts/view_ref.md
[a11yinput]: https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=78638
[viewref]: https://cs.opensource.google/fuchsia/fuchsia/+/main:sdk/fidl/fuchsia.ui.views/view_ref.fidl
[node]: https://cs.opensource.google/fuchsia/fuchsia/+/main:sdk/fidl/fuchsia.accessibility.semantics/node.fidl
[semantics]: https://cs.opensource.google/fuchsia/fuchsia/+/main:sdk/fidl/fuchsia.accessibility.semantics/semantics_manager.fidl
