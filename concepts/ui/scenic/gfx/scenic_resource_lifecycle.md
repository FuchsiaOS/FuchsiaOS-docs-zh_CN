> **_ATTENTION:_** This document describes the behavior of Scenic's GFX API which is currently being replaced by the [Flatland API](concepts/ui/scenic/flatland/index.md). Workstation currently uses Flatland only, and Smart Display will be converted to use Flatland as well. If working with Flatland, please refer to the [Flatland documentation](concepts/ui/index.md).

# Lifecycle of a Scene Graph Resource

This document describes the lifecycle of the basic resources that a client adds
to and removes from the scene graph. It focuses on resources that are attached
as Nodes and Views, and is a reference for Scenic's internal handling of these
resources.

Many Resources follow the Node lifecycle: Resources are created and added
to the ResourceMap. Resources are kept alive by reference counting. As such,
inter-scene graph references will keep resources in the scene graph alive, even
if the client calls `ReleaseResource`. The release command removes the Resource
from the client's internally-managed `ResourceMap`, which means that the client
cannot apply any future commands to that Resource. This document highlights how
the scene graph can keep Resources added to it alive, with the key exception of
the View resource  -- which is solely held onto via the client's ResourceMap.

This follows a simple embedded-embeddee client pair, and assumes the clients
have set up all [necessary resources](concepts/ui/scenic/index.md#scenic-resource-graph) to add
Nodes and Views to the global, retained scene graph.

> Note: the code shown below is typically handled by the UI C++ SDK wrappers. All
code examples show FIDL commands, as they are seen by Scenic.

## Node Lifecycle

### Adding a Node to the Scene

Say the embedder client (Client A) has created a root Scene node in its Session,
accessed via `root_id`. Client A can create children and add that to the root
node via the following commands:

```c++
CreateScene(root_id);
CreateEntityNode(entity_node_id);
AddChild(root_id, entity_node_id);
```

Internally, this creates a Node, adds it to the Session's ResourceMap, and sets
the EntityNode as the child of the root node in the scene graph:

![Image of a simple scene graph. There is a root Scene node with a strong link
to its child entity node. Client A's ResourceMap also has a strong reference to
both the root node and the entity node. There is a second image to the right,
labeled "projected scene", that shows a blank screen.](development/graphics/scenic/meta/scene_graph_lifecycle_root.png)

Client A can apply commands to the EntityNode as long as it maintains a
reference to it in the ResourceMap. For example:

```c++
SetTranslation(entity_node_id, {0, h/2, 0});
CreateShapeNode(shape_node_id);
CreateShape(shape_node_id, triangle);
AddChild(entity_node_id, shape_node_id);
```

![Image of the expanded scene graph. There is a root Scene node with a strong
link to its child entity node. The entity node has a strong link to is child,
a shape node with a triangle shape. Client A's ResourceMap also has a strong
reference to all the nodes in the scene. There is a second image to the right,
labeled "projected scene", that shows a triangle on the bottom half of the
screen.](development/graphics/scenic/meta/scene_graph_lifecycle_node_scene.png)

### Removing a Node

Releasing the Resource releases it from the ResourceMap. It does not release it
from the Scene graph, due to a strong reference from the parent. Client A can
release the Resources backing the "triangle dialog", and it will still remain
on the screen:

```c++
ReleaseResource(entity_node_id);
```

![Image of the scene graph in the image above. Client A's ResourceMap no longer
has a strong reference to the entity node. The "projected scene" image is
unchanged.](development/graphics/scenic/meta/scene_graph_lifecycle_node_scene_2.png)

To remove the triangle from the screen, the client would have to explicitly
detach the nodes from the scene graph. When the `Resource` is removed from both
the ResourceMap and from the scene graph, the resource is destroyed.

```c++
DetachChildren(root_id);
```

![Image of the scene graph. Its only node is the root scene node. The
ResourceMap has a strong reference to the root node and the shape node
containing the triangle shape. There is no entity node. The "projected scene"
image is a blank screen](development/graphics/scenic/meta/scene_graph_lifecycle_node_scene_detach.png)

## Embedding a View

### Add a ViewHolder to the SceneGraph

To embed a View from another Session, Client A must make a `ViewHolder` Resource,
and add it as a child of a node to add it to the scene graph.

```c++
CreateEntityNode(entity_node_id);
AddChild(root_id, entity_node_id);

CreateViewHolder(view_holder_id, view_holder_token);
AddChild(entity_node_id, view_holder_id);
```

![Image of the scene graph containing a scene root node with a child EntityNode.
The EntityNode has a ViewHolder child. Client A's ResourceMap has a strong
reference to the Scene, EntityNode, and the ViewHolder.](development/graphics/scenic/meta/scene_graph_lifecycle_viewholder.png)

The ViewHolder follows the same lifecycle rules as a Node, [described above](#node-lifecycle).
It will remain part of the scene graph as long as it is connected to something in
the scene graph. However, the client cannot add children to the ViewHolder:
instead, its corresponding View is linked by Scenic.

### Link a View to a ViewHolder

The `view_token` from the ViewHolder/View token pair is passed to the embedded
Session (Client B). When Client B creates a View from that token, a `View` is
created and added to the client's ResourceMap. Scenic creates links between the
View to the ViewHolder to establish this cross-Session connection. The View
then creates a "phantom `ViewNode`", and sets that as the child of the
ViewHolder. The ViewNode represents Client B's root node in the scene graph.

```c++
CreateView(view_id, view_token);
```

![Image of the scene graph above: Client A's ResourceMap maintains a strong
reference to the ViewHolder, EntityNode, and Scene, all added to the scene
graph. Client B's View and ViewNode are also added to the scene graph: the
ViewHolder maintains a strong reference to the ViewNode, and a weak reference to
the View. The View also maintains a strong reference to the ViewNode. Client B's
ResourceMap only points to the View.](development/graphics/scenic/meta/scene_graph_lifecycle_embedded_view.png)

Client B can then add children to the View, just like it can to Nodes. Under the
hood, the ViewNode maintains the children's connections to the scene graph:

```c++
CreateShapeNode(shape_node_id);
CreateShape(shape_node_id, rectangle);
AddChild(view_id, shape_node_id);
```

![Image of the scene graph above. A ShapeNode containing a rectangle is added to
the scene graph as the child of the ViewNode. Client B's ResourceMap also has a
strong reference to the ShapeNode. The "projected scene" image shows a rectangle
on the screen.](development/graphics/scenic/meta/scene_graph_lifecycle_embedded_view_with_nodes.png)

### Removing a View

A View is a viable Resource and added to the scene as long as it is in the
client's ResourceMap. It differs from a traditional Node because the scene graph
does not maintain a strong reference to the View Resource. To perform removal of
a View, Client B must command Scenic to release the View resource. Unlike a
node, the client does not have to command Scenic to detach it from the scene
graph. Releasing the View Resource destroys the View and its phantom ViewNode,
and detaches the View and its subtree from the global scene graph:

```c++
ReleaseResource(view_id);
```

![Image of the scene graph with Client A's nodes still attached. Client B's View
and ViewNode are destroyed, but its ResourceMap maintains a strong reference to
the ShapeNode.](development/graphics/scenic/meta/scene_graph_lifecycle_embedded_view_detach.png)

> Note: if either the View or ViewHolder is destroyed, its pair is delivered a
disconnected event (i.e. `fuchsia.ui.gfx.ViewHolderDisconnected` or
`fuchsia.ui.gfx.ViewDisconnected`, respectively).

### Removing a ViewHolder

A ViewHolder is treated as another child node in a Session, and so follows the
same [lifecycle rules](#removing-a-node). If a ViewHolder is released as a
Resource, and detached from the scene graph, the ViewHolder is destroyed.

Say that Client B has not released its View. When the ViewHolder is destroyed,
this breaks any link to the View, and destroys the strong reference to the child
ViewNode. The embedded View is thus detached from the scene. The embedded
Session and embedded View's subtree may still be intact, though no longer
visible.

```c++
Detach(view_holder_id);
ReleaseResource(view_holder_id);
```

![Image of the scene graph shows just the Scene root in the graph; Client A
maintains a strong reference to the Scene node. There is no ViewHolder. Client
B's subtree maintains the strong reference between the ViewNode and its child
ShapeNode, and Client B's ResourceMap maintains its links to the View and the
ShapeNode. The "projected scene" image is a blank screen.](development/graphics/scenic/meta/scene_graph_lifecycle_destroyed_viewholder.png)

> Note: Any embedded Sessions are notified if they are detached from the scene via
the `fuchsia.ui.gfx.ViewDetachedFromSceneEvent`.




