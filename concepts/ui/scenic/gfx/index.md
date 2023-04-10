> ***ATTENTION:*** This document describes the behavior of Scenic's GFX API
> which is currently being replaced by the
> [Flatland API](/docs/concepts/ui/scenic/flatland/index.md). Workstation
> currently uses Flatland only, and Smart Display
> [will be converted](https://fxbug.dev/93979) to use Flatland as well.
> If working with Flatland, please refer to the
> [Flatland documentation](/docs/concepts/ui/scenic/flatland/index.md).

# Gfx - Legacy Scenic 3D API

Gfx is [Scenic's](/docs/concepts/ui/scenic/index.md) legacy 3D API. Like other
APIs such as [Flatland](/docs/concepts/ui/scenic/flatland/index.md), it provides
a retained-mode scene graph consisting of content that is independently
generated and linked together by its clients. Like Flatland, it supports
Scenic's core responsibilities of Composition, Rendering, Scheduling, and
Diagnostics.

A unique feature of the 3D objects managed by the Gfx API is that they are
rendered within a unified lighting environment. This means that the object can
cast shadows or reflect light onto each other, even if the originating processes
have no knowledge of each other.

## Sessions

The [`fuchsia.ui.scenic.Session`](https://fuchsia.dev/reference/fidl/fuchsia.ui.scenic#Session)
FIDL protocol is the primary API used by clients of Scenic to contribute graphical
content in the form of `Resources`. Each session has its own resource table and is
unable to directly interact with resources belonging to other sessions.

Each session provides the following operations:

- Submit operations to add, remove, or modify resources.
- Commit a sequence of operations to be presented atomically.
- Awaiting and signaling fences.
- Schedule subsequent frame updates.
- Form links with other sessions (by mutual agreement).

When a session is destroyed, all of its resources are released and all of its
links become inoperable.

## Resources

`Resources` represent scene elements such as nodes, shapes, materials, and
animations that belong to particular `Sessions`.

The list of Scenic resources is described by the API:
[//sdk/fidl/fuchsia.ui.gfx/resources.fidl](/sdk/fidl/fuchsia.ui.gfx/resources.fidl)

Clients of Scenic generate graphical content to be rendered by queuing and
submitting operations to add, remove, or modify resources within their session.

Each resource is identified within its session by a locally unique id which is
assigned by the owner of the session (by arbitrary means). Sessions cannot
directly refer to resources that belong to other sessions (even if they happen
to know their id) therefore content embedding between sessions is performed
using `Link` objects as intermediaries.

To add a resource, perform the following steps:

- Enqueue an operation to add a resource of the desired type and assign it a
  locally unique id within the session.
- Enqueue one or more operations to set that resource's properties given its id.

Certain more complex resources may reference the ids of other resources within
their own definition. For instance, a `Node` references its `Shape` thus the
`Shape` must be added before the `Node` so that the node may reference it as
part of its definition.

To modify a resource, enqueue one or more operations to set the desired
properties in the same manner used when the resource was added.

The remove a resource, enqueue an operation to remove the resource.

Removing a resource causes its id to become available for reuse. However, the
session maintains a reference count for each resource that is internally
referenced. The underlying storage will not be released (and cannot be reused)
until all remaining references to the resource have been cleared *and* until the
next frame that does not require the resource has been presented. This is
especially important for `Memory` resources. See also [Fences](#fences).

This process of addition, modification, and removal may be repeated indefinitely
to incrementally update resources within a session.

### Nodes

A `Node` resource represents a graphical object that can be assembled into a
hierarchy called a `node tree` for rendering.

[Here](scenic_resource_lifecycle.md) is a walk-through on how Scenic internally
manages the lifecycle of Node-like resources and embedded Views.

### Scenes

A `Scene` resource combines a tree of nodes with the scene-wide parameters
needed to render it. A Scenic instance may contain multiple scenes but each
scene must have its own independent tree of nodes.

A scene resource has the following properties:

- The scene's root node.
- The scene's global parameters such as its lighting model.

In order to render a scene, a `Camera` must be pointed at it.

### Compositors

Compositors are resources that come in two flavors: `DisplayCompositor` and
`ImagePipeCompositor`; their job is to draw the content of a `LayerStack` into
their render target. For `DisplayCompositor`, the target display may have
multiple hardware overlays; in this case the compositor may choose associate
each of these with a separate layer, rather than flattening the layers into a
single image.

A `LayerStack` resource consists of an ordered list of `Layers`. Each layer can
contain either an `Image` (perhaps transformed by a matrix), or a `Camera` that
points at a `Scene` to be rendered (as described above).

### Scenic Resource Graph

![Scenic Resource Graph](/docs/concepts/ui/scenic/images/scenic_resource_graph.png)

### Coordinate Frames and Units

Scenic manages a global scene graph in a three dimensional space. Some of the
characteristics of this space are defined by Scenic itself, whereas some are
defined by the root presenter or even other clients.

![Scenic Axes](/docs/concepts/ui/scenic/images/scenic_axes.png)

### Units

Units are configured by the root presenter. The default root presenter uses a
device-independent scalable unit called "pips" for the root space. See
[Units and Metrics](units_and_metrics.md) for details. What units are used for
your view space depends on what transforms are applied to your view by your
parent.

### World Space

The Scenic world space is a right handed Cartesian space. It is configured by
the root presenter which configures the view and projection parameters of the
camera. The default root presenter will put the origin at the top left of the
screen and make +X point right, +Y point down, and +Z point into the screen.

### View Space

Ultimately the space of a given view depends on what transforms are applied to
it by its parent View and the parent View's parent and so on. If no rotation
transform is applied and all scale transforms are positive along all axes then
the View's axes will align with the axes of the root presenter and the
handedness will match.

The bounds of the root view are defined by a min and a max point as follows:

![Scenic Root View Bounds](/docs/concepts/ui/scenic/images/scenic_root_view_bounds.png)

## Views and Bounds

[View Bounds](/docs/development/graphics/scenic/concepts/view_bounds.md) shows
how to set up your view bounds, how to debug them with wireframe rendering, and
explains how view bounds interact with hit testing.

## Frame Scheduling

[Frame scheduling](/docs/concepts/ui/scenic/frame_scheduling.md) explains how
the frame scheduling API work and contains examples of how to use it.

{# ## Fences {#fences} #}

{# TODO(fxbug.dev/24431): Talk about synchronization. #}

## Examples of using Scenic

* See the examples located in [`//src/ui/examples`](/src/ui/examples); each
  example has documentation describing what it does and how to run it. There
  examples for both the Flatland and Gfx APIs.

## API Guide

### Scenic client libraries

Scenic has convenience wrapper libraries for some languages. These can be used
instead of using the FIDL API directly.

* [C++ client library](/sdk/lib/ui/scenic/cpp)
* [Rust client library](/src/lib/ui/fuchsia-scenic)

### FIDL protocols

The following files define and document the collection of FIDL protocols that
make up Scenic.

* [Scenic top-level protocols](/sdk/fidl/fuchsia.ui.scenic)
  (`fuchsia.ui.scenic`)
  * [scenic.fidl](/sdk/fidl/fuchsia.ui.scenic/scenic.fidl)
  * [session.fidl](/sdk/fidl/fuchsia.ui.scenic/session.fidl)
  * [commands.fidl](/sdk/fidl/fuchsia.ui.scenic/commands.fidl)
  * [events.fidl](/sdk/fidl/fuchsia.ui.scenic/events.fidl)

* [Gfx](/sdk/fidl/fuchsia.ui.gfx) (`fuchsia.ui.gfx`)
  * [commands.fidl](/sdk/fidl/fuchsia.ui.gfx/commands.fidl)
  * [events.fidl](/sdk/fidl/fuchsia.ui.gfx/events.fidl)
  * [resources.fidl](/sdk/fidl/fuchsia.ui.gfx/resources.fidl)
  * [nodes.fidl](/sdk/fidl/fuchsia.ui.gfx/nodes.fidl)
  * [shapes.fidl](/sdk/fidl/fuchsia.ui.gfx/shapes.fidl)
  * [...](/sdk/fidl/fuchsia.ui.gfx)

* [Views](/sdk/fidl/fuchsia.ui.views) (`fuchsia.ui.views`)
  * [commands.fidl](/sdk/fidl/fuchsia.ui.views/commands.fidl)

* [Input](/sdk/fidl/fuchsia.ui.input) (`fuchsia.ui.input`)
  * [commands.fidl](/sdk/fidl/fuchsia.ui.input/commands.fidl)
  * [input_events.fidl](/sdk/fidl/fuchsia.ui.input/input_events.fidl)

* [Policy](/sdk/fidl/fuchsia.ui.policy) (`fuchsia.ui.policy`)
  * [presenter.fidl](/sdk/fidl/fuchsia.ui.policy/presenter.fidl)
  * [presentation.fidl](/sdk/fidl/fuchsia.ui.policy/presentation.fidl)
  * [...](/sdk/fidl/fuchsia.ui.policy)

* [App](/sdk/fidl/fuchsia.ui.app) (`fuchsia.ui.app`)
  * [view_provider.fidl](/sdk/fidl/fuchsia.ui.app/view_provider.fidl)
