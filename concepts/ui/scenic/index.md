# Scenic, the Fuchsia system compositor

## Introduction

Scenic is a system service whose primary responsibility is managing a global
scene graph which is shared by all apps on the platform, along with the SysUI.
This scene graph mediates how apps display content on-screen, how they receive
input events from the user, and also supports the functionality of other Fuchsia
system components. It is the "source of truth" for the geometric/topological
relationships between clients.

## Scenic responsibilities {#responsibilities}

### Composition {#responsibility-composition}

Scenic provides APIs for clients (apps, SysUI) to manipulate a subgraph of the
global scene graph, which is rooted at a [View](views.md#view). Within this subgraph,
they can attach graphical content, and embed [Views](views.md#view) provided by other
components. This embedding is recursive: a component is linked to its parent the same
way as it links to its child components.

(Note: "parent" and "child" here refer to relative positions within the scene graph
topology, which may not correspond to the
[component topology](/docs/glossary#component-topology))

### Rendering/Display {#responsibility-rendering-display}

For maximum efficiency, Scenic can present client images directly to the display
controller, without powering up the GPU! However, for a variety of reasons this
is not always possible. Scenic dynamically decides each frame whether
client-provided images can be displayed directly, or whether they must be
composited with the GPU.

### Visual effects {#responsibility-visual-effects}

Scenic is responsible for visual effects that cannot be baked into the content
provided by clients. For example, Scenic's (deprecated) [Gfx](gfx/index.md) API
allows clients to insert content into a 3D scene, where objects from different
apps can cast shadows and reflect light onto each other. Similarly,
[Flatland](flatland/index.md) is designed to support effects such as "group
opacity".

The list of such effects is currently limited, but will grow in the future.

### Display management {#responsibility-display-management}

Fuchsia's display controller API is limited to a single connected client. Since
Scenic needs to display content every frame with low latency, it makes sense for
Scenic to have the sole connection to the display controller. However, this
implies that Scenic must act as a proxy for any other parties who need to
interact with the display controller. For example, the SysUI must enable the
user to:
- see which screens are connected
- choose the resolution and refresh rate for each screen
- arrange the screens' relative positions in a virtual desktop

### Frame scheduling {#responsibility-frame-scheduling}

Scenic, clients, and the display controller must all cooperate to schedule frames,
in order to minimize latency, memory use, etc. Scenic supports this by:
- notifying when shared resources are available for reuse (e.g. when it is safe for the
  client to render into a previously-used buffer)
- communicating client deadlines for having content presented on the next vsync
- waking itself up early enough to do the work to display a frame, including:
  - atomically update the scene graph
  - render with Vulkan, if necessary
  - tell the display controller which images to display

[Frame scheduling](frame_scheduling.md) explores this topic in greater detail.

### Input {#responsibility-input}

An app's position in the scene graph affects how input events, such as mouse
touch events, are routed to it. For example:
- clients are delivered input events in their view's coordinate system; Scenic transforms
  inbound events from the input pipeline into the proper coordinate system.
- position in the scene graph topology is important: a view that is "on top" of another
  will have preferential access to receive that event
- ongoing gestures like a "tap and drag" send all of the gesture events to the same
  target view, even if the user's finger moves outside of the bounds of the target view.

### Accessibility {#responsibility-accessibility}

Scenic provides functionality which is used by the Accessibility Manager to implement
[various features](/docs/concepts/accessibility/accessibility_framework.md).
For example, Scenic exposes a global view of the scene graph which supports
"screen reader" functionality. It also supports magnification for visually
challenged users.

### Diagnostics {#responsibility-diagnostics}

Scenic provides diagnostic interfaces to help developers debug their models and
measure performance.

{# TODO(fxbug.dev/109267): Link to a not-yet-existing page with more details. #}

### Screen capture {#responsibility-screen-capture}

Scenic provides support for individual screenshots, as well as ongoing screen
capture.

## Scene graph API

Scenic implements a variety of FIDL protocols to fulfill the responsibilities
above. Due to the central role the scene graph plays, the APIs for manipulating
the scene graph are among the most important.

### Two scene graph APIs?!? Why?

The current situation is complicated because, for historical reasons, Scenic
supports two distinct *kinds* of scene graphs, the deprecated
[Gfx](gfx/index.md) API and the more "future proof"
[Flatland](flatland/index.md) API.

Scenic supports instantiation of one scene graph at a time: there can either be
a Gfx scene graph or a Flatland scene graph, but not both simultaneously.

### Future scene graph APIs

In the future, there will likely be additional APIs that manipulate the scene
graph. The difference will be that, instead of each API referring to a different
kind of scene graph which is incompatible with the others, these futures APIs
will be designed to compose together nicely with Flatland APIs. An example is a
visual effect API to apply a blur effect a sub-tree of the Flatland scene graph.

### Scenic and Fuchsia

![Diagram of Scenic within Fuchsia](/docs/concepts/ui/scenic/images/scenic_within_fuchsia_diagram.png)

Scenic's API allows any client to insert its UI into the global scene graph.
Processes using the [*Flutter*](https://flutter.io/) UI framework are one
example; the lower layer of Flutter, called
[*Flutter Engine*](https://github.com/flutter/engine), contains code responsible
for communicating with Scenic.

Scenic has several internal subsystems. One of either *Flatland* or *Gfx* owns
the scene graph and is responsible for rendering. *Input* is responsible for
routing input events to clients, which also involves coordinating gesture
recognition across clients, and also managing focus.

*Scene Manager* is an independent service that is responsible for *presenting*
the system's UI; using the Scenic APIs, it creates the root of a Scenic scene
graph, embeds the window managers's UI, and reads input events using its *Input
Pipeline* library and continually forwards them to Scenic.

Scenic is a client of the [*Vulkan graphics driver*](/src/graphics/lib/magma/)
and the system *Display Driver*.

## Sessions

Scenic's two composition protocols ([Flatland](flatland/index.md) and
[Gfx](gfx/index.md)) have commonalities which are discussed here; see the linked
pages for details which pertain to one or the other.

In both cases, a "session" refers to a FIDL channel that a client uses to
achieve the following goals:
- Participate in the [View Tree](/docs/contribute/governance/rfcs/0147_view_system.md) by
forming links with other sessions (by mutual agreement).
  - provide a view which can be embedded by a parent component
  - conversely, embed child views provided by other components via their own sessions
- specify visual content which will linked into the global scene graph, consisting of:
  - images
  - spatial transforms
  - visual effects such as blurs or "group opacity"
- synchronize presentation with the display frame rate, and with other components'
  sessions
  - events from Scenic notify the client when it's OK to present another frame
  - fences allow efficient signaling to/from APIs such as Vulkan

A session provides a "retained mode" connection to the global scene graph. For
example, if a component is quiescent, its most recently-provided frame continues
to be displayed by Scenic.

When a session is destroyed, its links to child and parent views in other
sessions are broken,, and Scenic releases all memory and other resources that
were associated with the session.
