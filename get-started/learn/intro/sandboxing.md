{% import 'docs/_common/_doc_widgets.md' as widgets %}
<!-- # Software isolation model -->
# 软件隔离模型

<<../../_common/intro/_sandboxing_intro.md>>

<<../../_common/intro/_sandboxing_sandboxing.md>>

<<../../_common/intro/_sandboxing_namespaces.md>>

<!-- ## Exercise: Namespaces -->
## 练习：命名空间

<!-- 
In this exercise, you'll explore the contents of a component's namespace in
more detail using the shell.
 -->
在本练习中，您将使用命令行来更详细地探索组件的命名空间（namespace）的内容。

<<../_common/_start_femu.md>>

<!-- ### Find a component in the hub -->
### 在 hub 中找到一个组件

<!-- 
Fuchsia provides the [Hub](/concepts/components/v2/hub.md) as a
diagnostic interface to obtain information about component instances running
on the system. You can explore the components and their namespaces using the
hub's directory structure.
 -->
Fuchsia 提供了 [Hub](/concepts/components/v2/hub.md) 作为诊断接口，
用于获取系统中运行的组件实例的信息。
您可以使用 hub 的目录结构来探索组件及其命名空间。

<aside class="key-point">
<!-- 
The contents of the hub are organized according to the hierarchy of
{{ widgets.glossary_simple ('realm', 'component realms') }}in the system.
You'll explore more about what this structure means shortly.
 -->
hub 的内容按照系统中
{{ widgets.glossary_simple ('realm', '组件领域') }}的层次结构组织。
您将很快探索更多关于这种结构的含义。
</aside>


<!-- 
Connect to a device shell prompt and enter the following `ls` command to list
the components of the `core` realm under `/hub-v2/children/core/children`:
 -->
连接到设备命令行并输入以下 `ls` 命令
来列出 `/hub-v2/children/core/children` 下的 `core` 领域（realm）的组件:

```posix-terminal
fx shell ls /hub-v2/children/core/children
```

```none {:.devsite-disable-click-to-copy}
activity
appmgr
brightness_manager
bt-avrcp
build-info
...
```

<!-- 
This is a list of many of the core Fuchsia system components. To see
more details about a specific component, list its directory contents.
 -->
这是许多核心 Fuchsia 系统组件的一个列表。要查看更多关于特定组件的详细信息，可以列出它的目录内容。

<!-- 
Try this for the `http-client` component:
 -->
对 `http-client` 组件试试这个:

```posix-terminal
fx shell ls /hub-v2/children/core/children/network/children/http-client
```

```none {:.devsite-disable-click-to-copy}
children
component_type
debug
deleting
exec
id
resolved
url
```

<!-- ### Explore the namespace and outgoing directory -->
### 探索命名空间和出口目录

<!-- 
You'll find a running component's **namespace** under the `exec/in` path inside
the hub.
 -->
您将在 hub 内部的 `exec/in` 路径下找到运行中组件的**命名空间**。

```posix-terminal
fx shell ls /hub-v2/children/core/children/network/children/http-client/exec/in
```

```none {:.devsite-disable-click-to-copy}
config
pkg
svc
```

<!-- Here are some quick highlights of each element: -->
对每个元素简单说明如下:

<!-- 
*   `config/`: configuration data for the component
*   `pkg/`: the contents of the component's package
*   `svc/`: system services available to the component
 -->
*  `config/`: 组件的配置数据
*  `pkg/`: 组件的包的内容
*  `svc/`: 可供组件使用的系统服务

<!-- 
List the contents of the incoming `svc/` directory. This
directory contains
[service nodes](https://fuchsia.dev/reference/fidl/fuchsia.io#NodeInfo)
representing the system services provided to this component.
 -->
列出 `svc/` 目录的内容。这个目录包含
[服务节点](https://fuchsia.dev/reference/fidl/fuchsia.io#NodeInfo)，
表示提供给这个组件的系统服务。

```posix-terminal
fx shell ls /hub-v2/children/core/children/network/children/http-client/exec/in/svc
```

```none {:.devsite-disable-click-to-copy}
fuchsia.logger.LogSink
fuchsia.net.name.Lookup
fuchsia.posix.socket.Provider
```

<!-- 
Each of these services is accessible over a well-known protocol defined by a
[Fuchsia Interface Definition Language (FIDL)][glossary.FIDL] interface.
Components provide system services through their **outgoing directory**, which
is mapped to the `exec/out` path inside the hub.
 -->
每个服务都通过一个公有协议访问，其由一个
 [Fuchsia 接口定义语言（Fuchsia Interface Definition Language，FIDL)][glossary.FIDL]接口定义。
组件通过其**出口目录**（outgoing directory）来提供系统服务，
这个目录被映射到 hub 内部的 `exec/out` 路径。

<!-- 
List the contents of the outgoing `svc/` directory to see the system services
this component provides.
 -->
列出 `svc/` 出口目录来查看这个组件提供的系统服务。

```posix-terminal
fx shell ls /hub-v2/children/core/children/network/children/http-client/exec/out/svc
```

```none {:.devsite-disable-click-to-copy}
fuchsia.net.http.Loader
```

<!-- 
We'll explore FIDL protocols and how to access various services in more detail
later on.
 -->
我们将在以后更详细地探索 FIDL 协议及如何访问各种服务。

<aside class="key-point">
  <!-- <b>Extra Credit</b> -->
  <b>附加题</b>
<!-- 
  <p>Take a look at the other directory entries in the hub and see what else
  you can discover!</p>
   -->
  <p>看看 hub 中的其他目录项，试试看还能发现什么！</p>
</aside>
