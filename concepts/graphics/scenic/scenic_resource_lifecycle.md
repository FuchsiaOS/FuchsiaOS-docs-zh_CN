<!-- # Lifecycle of a Scene Graph Resource -->

# Scene Graph 中 Resource 的生命周期

<!-- This document describes the lifecycle of the basic resources that a client adds
to and removes from the scene graph. It focuses on resources that are attached
as Nodes and Views, and is a reference for Scenic's internal handling of these
resources. -->

本文档描述了客户端添加到场景图（scene graph）中的 Resource（资源）的生命周期。本文主要关注
Node（节点） 和 View（视图） 类型的 Resource，对应 Scenic 内部的 Resource 处理机制。

<!-- Many Resources follow the Node lifecycle: Resources are created and added
to the ResourceMap. Resources are kept alive by reference counting. As such,
inter-scene graph references will keep resources in the scene graph alive, even
if the client calls `ReleaseResource`. The release command removes the Resource
from the client's internally-managed `ResourceMap`, which means that the client
cannot apply any future commands to that Resource. This document highlights how
the scene graph can keep Resources added to it alive, with the key exception of
the View resource  -- which is solely held onto via the client's ResourceMap. -->

多数 Resource 遵循 Node 的生命周期：Resource 在创建后，会加入到 ResourceMap 中，由
引用计数来确保其存活。这样，如果 Resource 被其他 Scene Graph 所引用，即使客户调用
`ReleaseResource` 来释放 Resource， Scene Graph 中的 Resource 仍会存活。
`ReleaseResource` 命令会在客户端内部管理的 `ResourceMap` 中删除对 Resource 的引用，
这样，客户端调用这一命令后，将无法在该 Resource 上施加其他命令。
需要着重注意的是，Scene Graph 会保证加入其中的 Resource 保持存活，但 View 类型的 Resource 除外
—— View 的引用只在客户端的 `ResourceMap` 中计入。

<!-- This follows a simple embedded-embeddee client pair, and assumes the clients
have set up all [necessary resources](scenic.md#scenic-resource-graph) to add
Nodes and Views to the global, retained scene graph. -->

下面的例子是一个简单的“父视图—子视图”客户端模型，且我们假设客户端在将 Node 和 View 
添加到全局 Scene Graph 时，已经配置好所有[必要资源](scenic.md#scenic-resource-graph)。

<!-- > Note: the code shown below is typically handled by the UI C++ SDK wrappers. All
code examples show FIDL commands, as they are seen by Scenic. -->

> Note: 通常 UI C++ SDK 会封装下文的代码实现。但为了方便从 Scenic 视角描述，
此处所有的代码范例均为原始 FIDL 命令。

<!-- ## Node Lifecycle -->

## Node 生命周期

<!-- ### Adding a Node to the Scene -->

### 将 Node 添加到 Scene

<!-- Say the embedder client (Client A) has created a root Scene node in its Session,
accessed via `root_id`. Client A can create children and add that to the root
node via the following commands: -->

例如，一个父视图客户端（客户端 A）在其 Session 中创建了一个根 Scene 节点，标号为
`root_id`。客户端 A 可以利用下述命令创建子节点，并将其添加到根节点下：

```c++
CreateScene(root_id);
CreateEntityNode(entity_node_id);
AddChild(root_id, entity_node_id);
```

<!-- Internally, this creates a Node, adds it to the Session's ResourceMap, and sets
the EntityNode as the child of the root node in the scene graph: -->

在 Scenic 内部，该操作添加了一个 Node，将其添加到 Session 的 ResourceMap 中，并将
这个 EntityNode 设为根节点的子节点。

<!-- ![Image of a simple scene graph. There is a root Scene node with a strong link
to its child entity node. Client A's ResourceMap also has a strong reference to
both the root node and the entity node. There is a second image to the right,
labeled "projected scene", that shows a blank screen.](meta/scene_graph_lifecycle_root.png) -->

![此处为一个简单 Scene Graph 的示意图。其中有一个根 Scene 节点，与其子节点 EntityNode 有强
链接。客户端 A 的 ResourceMap 也有对根节点和 EntityNode 的强引用。右侧的图象
为“显示的场景”，是空白屏幕。](meta/scene_graph_lifecycle_root.png)

<!-- Client A can apply commands to the EntityNode as long as it maintains a
reference to it in the ResourceMap. For example: -->

只要客户端 A 在 ResourceMap 中含有对 EntityNode 的引用，它就可以在这个
EntityNode 上应用命令。例如：

```c++
SetTranslation(entity_node_id, {0, h/2, 0});
CreateShapeNode(shape_node_id);
CreateShape(shape_node_id, triangle);
AddChild(entity_node_id, shape_node_id);
```

<!-- ![Image of the expanded scene graph. There is a root Scene node with a strong
link to its child entity node. The entity node has a strong link to is child,
a shape node with a triangle shape. Client A's ResourceMap also has a strong
reference to all the nodes in the scene. There is a second image to the right,
labeled "projected scene", that shows a triangle on the bottom half of the
screen.](meta/scene_graph_lifecycle_node_scene.png) -->

![此处为一个扩展后的 Scene Graph 的示意图。其中有一个根 Scene 节点，与其子节点 EntityNode 有强
链接。EntityNode 与其子节点 —— 一个三角形的 ShapeNode 有强链接。客户端 A 的 ResourceMap
上述所有节点的强引用。右侧的图象为“显示的场景”，在屏幕的下半部分，显示有一个三
角形。](meta/scene_graph_lifecycle_node_scene.png)


<!-- ### Removing a Node -->

### 删除节点

<!-- Releasing the Resource releases it from the ResourceMap. It does not release it
from the Scene graph, due to a strong reference from the parent. Client A can
release the Resources backing the "triangle dialog", and it will still remain
on the screen: -->

释放 Resource 时，也会将 Resource 的引用从客户端的 ResourceMap 中释放。但 Scene Graph 中
的该 Resource 还未被释放，因为它的父节点仍含有它的强引用。客户端 A 可以释放“三角形
对话框”的 Resource，但它仍然显示在屏幕上。

```c++
ReleaseResource(entity_node_id);
```

<!-- ![Image of the scene graph in the image above. Client A's ResourceMap no longer
has a strong reference to the entity node. The "projected scene" image is
unchanged.](meta/scene_graph_lifecycle_node_scene_2.png) -->

![此处为上述 Scene Graph 的示意图。客户端 A 的 ResourceMap 中不再含有对 EntityNode 的强引用。
右侧“显示的场景”保持不变。](meta/scene_graph_lifecycle_node_scene_2.png)

<!-- To remove the triangle from the screen, the client would have to explicitly
detach the nodes from the scene graph. When the `Resource` is removed from both
the ResourceMap and from the scene graph, the resource is destroyed. -->

要从屏幕上删除这个三角形，客户端还需要将这个 Node 同 Scene Graph 分离。在 `Resource`
从 ResourceMap 和 Scene Graph 中都已被删除后，该 Resource 才被销毁。

```c++
DetachChildren(root_id);
```

<!-- ![Image of the scene graph. Its only node is the root scene node. The
ResourceMap has a strong reference to the root node and the shape node
containing the triangle shape. There is no entity node. The "projected scene"
image is a blank screen](meta/scene_graph_lifecycle_node_scene_detach.png) -->

![此处为上述 Scene Graph 的示意图。现在其中只有根 Scene 节点。ResourceMap 有对根节点
和表示三角形的 ShapeNode 的强引用，但没有 EntityNode。右侧“显示的场景”
为空。](meta/scene_graph_lifecycle_node_scene_detach.png)

<!-- ## Embedding a View -->

## 嵌入 View

<!-- ### Add a ViewHolder to the SceneGraph -->

### 将 ViewHolder 添加到 Scene Graph 中

<!-- To embed a View from another Session, Client A must make a `ViewHolder` Resource,
and add it as a child of a node to add it to the scene graph. -->

要添加来自其他 Session 的 View，客户端 A 必须创建一个 `ViewHolder` 类 Resource，
并将其作为一个 Node 的子节点添加到 Scene Graph 中。

```c++
CreateEntityNode(entity_node_id);
AddChild(root_id, entity_node_id);

CreateViewHolder(view_holder_id, view_holder_token);
AddChild(entity_node_id, view_holder_id);
```

<!-- ![Image of the scene graph containing a scene root node with a child EntityNode.
The EntityNode has a ViewHolder child. Client A's ResourceMap has a strong
reference to the Scene, EntityNode, and the ViewHolder.](meta/scene_graph_lifecycle_viewholder.png) -->

![此处为上述 Scene Graph 的示意图。其中包含根 Scene 节点和子节点 EntityNode。EntityNode
有 ViewHolder 类型的子节点。客户端 A 的 ResourceMap 中有对 Scene、EntityNode 和 ViewHolder 
的强引用。](meta/scene_graph_lifecycle_viewholder.png)

<!-- The ViewHolder follows the same lifecycle rules as a Node, [described above](#node-lifecycle).
It will remain part of the scene graph as long as it is connected to something in
the scene graph. However, the client cannot add children to the ViewHolder:
instead, its corresponding View is linked by Scenic. -->

[如前所述](#node-lifecycle)，ViewHolder 的生命周期与 Node 相同。只要它同 Scene Graph 内的某个
节点相连接，它就仍然是 Scene Graph 的一部分。然而，客户端不能向 ViewHolder 添加子节点；
Scenic 会将 ViewHolder 与对应的 View 相连接。 

<!-- ### Link a View to a ViewHolder -->

### 将 View 连接到 ViewHolder

<!-- The `view_token` from the ViewHolder/View token pair is passed to the embedded
Session (Client B). When Client B creates a View from that token, a `View` is
created and added to the client's ResourceMap. Scenic creates links between the
View to the ViewHolder to establish this cross-Session connection. The View
then creates a "phantom `ViewNode`", and sets that as the child of the
ViewHolder. The ViewNode represents Client B's root node in the scene graph. -->

一对 ViewHolder/View token 中的 `view_token` 会被传入被嵌入视图（子视图）的 Session
（客户端 B）。当客户端 B 使用  `view_token` 创建 View 后，被创建的 `View` 会加入
该客户端的 ResourceMap。Scenic 会创建 View 和 ViewHolder 之间的 Link （链接）来建立
跨 Session 连接。`View` 会（在 Scene Graph 中）创建一个不可见的 `ViewNode`，并将 `ViewNode`
设为 ViewHolder 的子节点。在 Scene Graph 中，`ViewNode` 就代表了客户端 B 的根节点。 

```c++
CreateView(view_id, view_token);
```

<!-- ![Image of the scene graph above: Client A's ResourceMap maintains a strong
reference to the ViewHolder, EntityNode, and Scene, all added to the scene
graph. Client B's View and ViewNode are also added to the scene graph: the
ViewHolder maintains a strong reference to the ViewNode, and a weak reference to
the View. The View also maintains a strong reference to the ViewNode. Client B's
ResourceMap only points to the View.](meta/scene_graph_lifecycle_embedded_view.png) -->

![此处为上述 Scene Graph 的示意图: 客户端 A 的 ResourceMap 保有对 ViewHolder、EntityNode 及
Scene 的强引用，这些节点均被添加到 Scene Graph 中。客户端 B 的 View 和 ViewNode 也同样被添加
到 Scene Graph 中：ViewHolder 保有对 ViewNode 的强引用，以及对 View 的弱引用。View 保有
对 ViewNode 的强引用。客户端 B 的 ResourceMap 只指向 
View。](meta/scene_graph_lifecycle_embedded_view.png)

<!-- Client B can then add children to the View, just like it can to Nodes. Under the
hood, the ViewNode maintains the children's connections to the scene graph: -->

此后，客户端 B 可以向 View 中添加子节点，就像往 Node 下添加子节点一样。在 Scenic 内部实现中，
 Scene Graph 中的 ViewNode 同 View 的各子节点相连。

```c++
CreateShapeNode(shape_node_id);
CreateShape(shape_node_id, rectangle);
AddChild(view_id, shape_node_id);
```

<!-- ![Image of the scene graph above. A ShapeNode containing a rectangle is added to
the scene graph as the child of the ViewNode. Client B's ResourceMap also has a
strong reference to the ShapeNode. The "projected scene" image shows a rectangle
on the screen.](meta/scene_graph_lifecycle_embedded_view_with_nodes.png) -->

![此处为上述 Scene Graph 的示意图：一个含有矩形的 ShapeNode 加入到 Scene Graph 中，成为 ViewNode
的子节点。客户端 B 的 ResourceMap 也包含对这个 ShapeNode 的强引用。在“显示的场景”
一图中，屏幕上显示了一个矩形。](meta/scene_graph_lifecycle_embedded_view_with_nodes.png)

<!-- ### Removing a View -->

### 删除 View

<!-- A View is a viable Resource and added to the scene as long as it is in the
client's ResourceMap. It differs from a traditional Node because the scene graph
does not maintain a strong reference to the View Resource. To perform removal of
a View, Client B must command Scenic to release the View resource. Unlike a
node, the client does not have to command Scenic to detach it from the scene
graph. Releasing the View Resource destroys the View and its phantom ViewNode,
and detaches the View and its subtree from the global scene graph: -->

对于 View 来说，只要它包含在客户端的 ResourceMap 中，View 就是可用的 Resource，且在
场景中存活。与一般 Node 不同的是， Scene Graph 中不包含 View Resource 的强引用。要删除一个
View，客户端需要命令 Scenic 释放该 View Resource 。但与 Node 不同的是，客户端不需要显式
要求 Scenic 将 View 从 Scene Graph 中取出。释放 View Resource 就会销毁 View 和它拥有的
“隐形“ ViewNode，并将 View 与其子树从全局 Scene Graph 中移除。

```c++
ReleaseResource(view_id);
```

<!-- ![Image of the scene graph with Client A's nodes still attached. Client B's View
and ViewNode are destroyed, but its ResourceMap maintains a strong reference to
the ShapeNode.](meta/scene_graph_lifecycle_embedded_view_detach.png) -->

![此处为上述 Scene Graph 的示意图：客户端 A 的各节点仍然连接在 Scene Graph 和 ResourceMap 中。客户端 B
的 View 和 ViewNode 被销毁，但其 ResourceMap 中仍保有对 ShapeNode 的强
引用。](meta/scene_graph_lifecycle_embedded_view_detach.png)

<!-- > Note: if either the View or ViewHolder is destroyed, its pair is delivered a
disconnected event (i.e. `fuchsia.ui.gfx.ViewHolderDisconnected` or
`fuchsia.ui.gfx.ViewDisconnected`, respectively). -->

> Note: 如果 View 或 ViewHolder 中的一个被销毁，仍存活的节点将接收到“连接断开”
事件（`fuchsia.ui.gfx.ViewHolderDisconnected` 或 `fuchsia.ui.gfx.ViewDisconnected`）

<!-- ### Removing a ViewHolder -->

### 删除 ViewHolder

<!-- A ViewHolder is treated as another child node in a Session, and so follows the
same [lifecycle rules](#removing-a-node). If a ViewHolder is released as a
Resource, and detached from the scene graph, the ViewHolder is destroyed. -->

ViewHolder 也被视作 Session 中的子节点，因此与 Node 拥有相同的[生命周期](#removing-a-node)。
如果一个 ViewHolder 作为 Resource 被释放，并且从 Scene Graph 中脱离，则该 ViewHolder
会被销毁。

<!-- Say that Client B has not released its View. When the ViewHolder is destroyed,
this breaks any link to the View, and destroys the strong reference to the child
ViewNode. The embedded View is thus detached from the scene. The embedded
Session and embedded View's subtree may still be intact, though no longer
visible. -->

例如：客户端 B 还未释放其 View，当 ViewHolder 被销毁后，ViewHolder 与 View 间
的链接也被切断，因此 ViewHolder 到 ViewNode 子节点的强引用也被销毁。被嵌套的 View
（子视图）会从场景中脱离。子视图的 Session 及 View 的子树不受影响，但子视图将不可见。

```c++
Detach(view_holder_id);
ReleaseResource(view_holder_id);
```

<!-- ![Image of the scene graph shows just the Scene root in the graph; Client A
maintains a strong reference to the Scene node. There is no ViewHolder. Client
B's subtree maintains the strong reference between the ViewNode and its child
ShapeNode, and Client B's ResourceMap maintains its links to the View and the
ShapeNode. The "projected scene" image is a blank screen.](meta/scene_graph_lifecycle_destroyed_viewholder.png) -->

![此处为上述 Scene Graph 的示意图，图中只有根节点 Scene；客户端 A 保有该节点的强引用。图中
不含 ViewHolder。客户端 B 的子树保有 ViewNode 和其子节点 ShapeNode 的强引用，
且客户端 B 的 ResourceMap 仍与 View 和 ShapeNode 保持连接。“展示的画面”
为空。](meta/scene_graph_lifecycle_destroyed_viewholder.png)

<!-- > Note: Any embedded Sessions are notified if they are detached from the scene via
the `fuchsia.ui.gfx.ViewDetachedFromSceneEvent`. -->

> Note: 如果子视图从场景中脱离，其所属的 Session 将会收到
`fuchsia.ui.gfx.ViewDetachedFromSceneEvent` 类型的通知。




