# Flatland scene dump

This document describes how to dump information about the
[Flatland][glossary.flatland] scene and reading the output. This guide is useful
if you want to debug the state of the Flatland scene while developing for
Fuchsia and using Flatland to create a graphical view. For more information
about Flatland, see [Flatland][flatland-concepts].

## Inspect your Flatland scene

You can use
[Component Inspection](/development/diagnostics/inspect/README.md) to dump
information about the Flatland scene:

```posix-terminal
ffx inspect show core/ui/scenic:root/scenic/FlatlandEngine:scene_dump
```

## Understanding the scene dump output

The scene information can be categorized into three different sections:

-   [Topology](#topology)
-   [All instances](#all-instances)
-   [Image and image-rectangles](#image-and-image-rectangles)

### Topology {#topology}

Example output:

```none {:.devsite-disable-click-to-copy}

Topology:

2:0-| <-- (FlatlandDisplay)
 | 2:1-|
 |  | 4:1-|
 |  |  | 4:0-| <-- (SceneManager Display)
 |  |  |  | 4:2-|
 |  |  |  |  | 4:3-|
 |  |  |  |  |  | 4:4-|
 |  |  |  |  |  |  | 3:1-|
 |  |  | 4:5-|
 |  |  |  | 4:6-|
 |  |  |  |  | 5:1

```

The topology section creates an ASCII representation of the entire Flatland
scene.

Each Flatland [Transform][glossary.transform] node is represented by a pair of
numbers. The first number in the pair is the *Flatland **Instance Id***. It
represents which *Flatland Instance* created that particular *Transform*. The
second number in the pair is the ***Transform Id***. This is a user-defined
identifier for the the Transform, unique among all Transforms created by an
Instance. In the example above, `(2:0)`, `(2:1)`, `(4:1)` (and so on) represent
Transform nodes. The Flatland Instance with the identifier `2` has created two
Transforms (`0` and `2`).

Each line only contains one Transform node, and may include a **debug name**.
The debug name may be specified using
[fuchsia.ui.composition.Flatland.SetDebugName](/sdk/fidl/fuchsia.ui.composition/flatland.fidl).
In the example above, node `(2:0)` has the debug name `FlatlandDisplay`.

The "`-|`" symbol represents a parent/child relationship between the nodes,
creating a graph structure. For instance, node `(2:1)` is a child of node
`(2:0)`. Please see
[fuchsia.ui.composition.Flatland](/sdk/fidl/fuchsia.ui.composition/flatland.fidl)
for further details on the implications. In short, this means that node `(2:1)`
will be rendered on top of node `(2:0)`. Similarly, the relationship between two
Flatland instances is captured. node `(2:1)` and node `(4:1)` have a
Viewport/View relationship, in which the content of the root Transform node
`(4:1)` (and the resulting child node graph rendered) is displayed as content in
node `(2:1)`.

Note that, in the example above, node `(4:1)` has two child nodes -- `(4:0)` and
`(4:5)`. Direct children nodes are represented at the same indentation column.
The lines between those two nodes represent the sub-graph of child node `(4:0)`.
The lines following node `(4:1)` represent the sub-graph of node `(4:1)`.

The topology will always begin at the root node of the Display. This means that
any *Flatland Instances* and *Transforms* that are not connected to the Display
graph will not be shown in the ASCII representation.

### All instances {#all-instances}

Example output:

```none {:.devsite-disable-click-to-copy}

All Instances:

Instance 2 (FlatlandDisplay):
2:0-|
 | 2:1-|

Instance 4 (SceneManager Display):
4:1-|
 | 4:0-|
 |  | 4:2-|
 |  |  | 4:3-|
 |  |  |  | 4:4-|
   4:5-|
      4:6

Instance 3:
3:1-|

Instance 5:
5:1-|

Instance 6:
6:1-|
   6:2-|
      6-3-|

```

This section lists the topology of the *Flatland Instances*, without showing the
Viewport/View connection between Instances. The ASCII representation is similar
except that each *Flatland Instance* is listed separately.

Remember that the [Topology](#topology) section noted that only the Topology of
Instances connected to the root display node are shown. In this output, all
Instances are listed, regardless if they are connected to the root display
topology or not. In the example above, Instance 6 is not connected to the root
node topology which is why it is not shown in the example from the
[Topology](#topology) section.

### Image and image-rectangles {#image-and-image-rectangles}

Example output:

```none {:.devsite-disable-click-to-copy}

Frame display-list contains 2 images and image-rectangles.
        image: size=1280x800  multiply_color=(1,1,1,1)  blend_mode=SRC
        transform: (4:3)
        rect: Rectangle2D[origin:(0, 0) extent:(1280, 800) clockwise_uvs:[(1, 0),(1, 1),(0, 1),(0, 0)]]
        image: size=64x64  multiply_color=(1,1,1,1)  blend_mode=SRC_OVER
        transform: (3:1)
        rect: Rectangle2D[origin:(128, 128) extent:(64, 64) clockwise_uvs:[(1, 0),(1, 1),(0, 1),(0, 0)]]

```

This section lists *Flatland images* created using
[fuchsia.ui.composition.Flatland.CreateImage](/sdk/fidl/fuchsia.ui.composition/flatland.fidl).
Information about each image is represented on three lines.

The first line shows the image properties. This includes:

*   The size in pixels, represented as "`[width]x[height]`"
*   The RGBA color, with each value in the range [0.0, 1.0]
*   The
    [fuchsia.ui.composition.Flatland.BlendMode](/sdk/fidl/fuchsia.ui.composition/flatland.fidl)

The second line shows the Transform node which created the image.

The third line lists the properties of the Image Rectangle. This includes:

*   The origin, representing the top-left corner of the Rectangle.
*   The extent, representing the width and the height of the Rectangle.
*   The clockwise mapping UVs. Starting at the top-left corner and rotating
    clockwise, each corner's `(x, y)` sample point is represented with a value
    in the range [0.0, 1.0]. The clockwise UVs therefore contain information
    about the sample region, clipping region and rotation.

[flatland-concepts]: /concepts/ui/scenic/flatland/index.md
[glossary.flatland]: /glossary/README.md#flatland
[glossary.transform]: /glossary/README.md#transform
