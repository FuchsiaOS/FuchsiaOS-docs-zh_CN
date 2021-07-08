<!-- # Fuchsia Namespaces

Namespaces are the backbone of file access and service discovery in Fuchsia. -->

# Fuchsia 的命名空间

命名空间是 Fuchsia 中文件访问与服务发现的基石。

<!-- ## Definition

A namespace is a composite hierarchy of files, directories, sockets, services,
devices, and other named objects provided to a component by its
environment.

Let's unpack that a little bit. -->

## 定义

命名空间是文件、目录、套接字、服务、设备和其它由环境提供给某一组件的命名对象之间的复合层级结构。  

下面我们展开进行一些解释。

<!-- **Objects are named**: The namespace contains _objects_ which can be enumerated
and accessed by name, much like listing a directory or opening a file. -->

**对象已被命名**：包含 _对象_ 的命名空间能被枚举或通过名称访问，这与列出一个目录或打开一个文件非常相似。

<!-- **Composite hierarchy**: The namespace is a _tree_ of objects that has been
assembled by _combining_ together subtrees of objects from other namespaces
into a composite structure where each part has been assigned a path prefix
by convention. -->

**复合层级结构**：命名空间是一个对象组成的 _树_，这棵 _树_ 是由其它命名空间的对象 _组合_ 在一起的子树构成的一个复合结构，每个组成部分都按照规则约定了一个路径前缀。

<!-- **Namespace per component**: Every component receives its own namespace
tailored to meet its own needs.  It can also publish objects of its own
to be included in other namespaces. -->

**每个组件的命名空间**：每个组件接收一个属于它自己的，根据其需求定制的命名空间。组件同样能够将它自己的对象发布并包含到其它的命名空间中去。

<!-- **Constructed by the environment**: The environment, which instantiates a
component, is responsible for constructing an appropriate namespace for that
component within that scope. -->

**由环境构建**：如果某个环境实例化了一个组件，那么那个组件需要在其作用域内为那个组件构建一个合适的命名空间。

<!-- Namespaces can also be created and used independently from components although
this document focuses on typical component-bound usage. -->

命名空间同样能独立于组件之外被创建和使用，即使它的文档主要聚焦在与组件绑定的典型用法之上。

<!-- ## Namespaces in Action

You have probably already spent some time exploring a Fuchsia namespace;
they are everywhere.  If you type `ls /` at a command-line shell prompt
you will see a list of some of the objects that are accessible from the
shell's namespace. -->

## 命名空间实战

你可能已经花了一些时间来探索 Fuchsia 的命名空间——它们无处不在。如果你在命令提示符中敲下 `ls /`，你会发现该终端命名空间下的一系列可访问的对象。

<!-- Unlike other operating systems, Fuchsia does not have a "root filesystem".
As described earlier, namespaces are defined per-component rather than
globally or per-process. -->

与其他操作系统不同，Fuchsia 并没有“根文件系统”。正如先前所说的，命名空间是为每一个组件定义的，而非全局定义或为每个进程定义。  

<!-- This has some interesting implications:

- There is no global "root" namespace.
- There is no concept of "running in a chroot-ed environment" because every
  component [effectively has its own private "root"](/docs/concepts/filesystems/dotdot.md).
- Components receive namespaces tailored to their specific needs.
- Object paths may not be meaningful across namespace boundaries.
- A process may have access to several distinct namespaces at once.
- The mechanisms used to control access to files can also be used to control
  access to services and other named objects on a per-component basis. -->

下面有一些很有趣的概念：

- 不存在全局的“根”命名空间。
- 不存在“运行在切换过根目录（chroot 命令）的环境中”的概念，因为事实上每个组件[都有它自己私有的“根”](/docs/concepts/filesystems/dotdot.md)。
- 组件的命名空间是按照其需求定制的。
- 对象的路径在跨越命名空间时可能并不起作用。
- 一个进程可能会同时访问多个不同的命名空间。
- 用于控制文件访问的机制同样可以用于控制服务和其它命名对象的访问，前提是在同一个组件上。

<!-- ## Objects

The items within a namespace are called objects.  They come in various flavors,
including:

- Files: objects that contain binary data
- Directories: objects that contain other objects
- Sockets: objects that establish connections when opened, like named pipes
- Services: objects that provide FIDL services when opened
- Devices: objects that provide access to hardware resources -->

## 对象

命名空间中的内容被称为对象。它们分为以下几种：

- 文件：包含二进制数据的对象。
- 目录：包含其它对象的对象。
- 套接字：打开时能建立连接的对象，与命名管道相似。
- 服务：打开时能提供 FIDL 服务的对象。
- 设备：提供硬件资源访问的对象。

<!-- ### Accessing Objects

To access an object within a namespace, you must already have another object
in your possession.  A component typically receives channel handles for
objects in the scope of its namespace during
[Namespace Transfer](#namespace_transfer). -->

### 访问对象

为了访问一个命名空间中的对象，你必须已经拥有另外一个对象。一个组件，通常是在[命名空间转移](#namespace_transfer)时才在其命名空间作用域内接受对象的管道处理。

<!-- You can also create new objects out of thin air by implementing the
appropriate FIDL protocols. -->

同样，你也能通过实现合适的 FIDL 协议来无中生有地创造一个新对象。

<!-- Given an object's channel, you can open a channel for one of its sub-objects
by sending it a FIDL message that includes an object relative path expression
which identifies the desired sub-object.  This is much like opening files
in a directory. -->

给出一个对象的管道，通过传递一个包含某个对象的相关路径表达式（以此指明子对象）的 FIDL 信息，就能打开该对象的一个子对象的管道。这与打开一个目录中的文件非常相似。

<!-- Notice that you can only access objects that are reachable from the ones
you already have access to.  There is no ambient authority. -->

注意，你只能访问那些能从你已经访问过的对象处访问的对象，没有环境权限（Ambient Authority）。

<!-- We will now define how object names and paths are constructed. -->

下面我们将介绍对象名和路径是如何构建的。

<!-- ### Object Names

An object name is a locally unique label by which an object can be located
within a container (such as a directory).  Note that the name is a property
of the container's table of sub-objects rather than a property of the object
itself. -->

### 对象名

一个对象名是一个本地的唯一标签，用于在某一容器（如目录）内定位一个对象。注意，对象名是其容器的子对象表中的属性，而非该对象本身的属性。

<!-- For example, `cat` designates a furry object located within some unspecified
recipient of an `Open()` request. -->

例如，`猫`在某个`Open()`请求的接收器内定位了一个毛绒绒的对象。

<!-- Objects are fundamentally nameless but they may be called many names by others. -->

对象从根本上就被设计成无名的，但是它们能被其它对象用很多名字来指示。

<!-- Object names are represented as binary octet strings (arbitrary sequences
of bytes) subject to the following constraints:

- Minimum length of 1 byte.
- Maximum length of 255 bytes.
- Does not contain NULs (zero-valued bytes).
- Does not contain `/`.
- Does not equal `.` or `..`.
- Always compared using byte-for-byte equality (implies case-sensitive). -->

对象名由具有以下规定的八位字节二进制字符串（任意的字节序列）表示：

- 最短为 1 字节
- 最长为 255 字节
- 不包含 NUL （0 值字节）
- 不包含 `/`
- 不包含 `.` 或 `..`
- 逐比特对比是否相等（即大小写敏感）

<!-- Object names are valid arguments to a container's `Open()` method.
See [FIDL Protocols](/docs/concepts/fidl/overview.md). -->

对象名是容器的 `Open()` 方法的合法参数。详见 [FIDL 协议](/docs/concepts/fidl/overview.md)。

<!-- It is intended that object names be encoded and interpreted as human-readable
sequences of UTF-8 graphic characters, however this property is not enforced
by the namespace itself. -->

为使其具有可读性，对象名可以使用 UTF-8 进行编解码，但是这一特性并非强制。

<!-- Consequently clients are responsible for deciding how to present names
which contain invalid, undisplayable, or ambiguous character sequences to
the user. -->

因此，由客户端负责决定如何将含有非法、无法显示、有歧义的字符序列呈现给用户。

<!-- _TODO(jeffbrown): Document a specific strategy for how to present names._ -->

### Object Relative Path Expressions

An object relative path expression is an object name or a `/`-delimited
sequence of object names designating a sequence of nested objects to be
traversed in order to locate an object within a container (such as a
directory).

For example, `house/box/cat` designates a furry object located within its
containing object called `box` located within its containing object called
`house` located within some unspecified recipient of an `Open()` request.

An object relative path expression always traverses deeper into the namespace.
Notably, the namespace does not directly support upwards traversal out of
containers (e.g. via `..`) but this feature may be partially emulated by
clients (see below).

Object relative path expressions have the following additional constraints:

- Minimum length of 1 byte.
- Maximum length of 4095 bytes.
- Does not begin or end with `/`.
- All segments are valid object names.
- Always compared using byte-for-byte equality (implies case-sensitive).

Object relative path expressions are valid arguments to a container's `Open()`
method.  See [FIDL Protocols](/docs/concepts/fidl/overview.md).

### Client Interpreted Path Expressions

A client interpreted path expression is a generalization of object relative
path expressions, but includes optional features that may be emulated
by client code to enhance compatibility with programs that expect a rooted
file-like interface.

Technically these features are beyond the scope of the Fuchsia namespace
protocol itself but they are often used so we describe them here.

- A client may designate one of its namespaces to function as its "root".
  This namespace is denoted `/`.
- A client may construct paths relative to its designated root namespace
  by prepending a single `/`.
- A client may construct paths that traverse upwards from containers using
  `..` path segments by folding segments together (assuming the container's
  path is known) through a process known as client-side "canonicalization".
- These features may be combined together.

For example, `/places/house/box/../sofa/cat` designates a furry object
located at `places/house/sofa/cat` within some client designated "root"
container.

Client interpreted path expressions that contain these optional features
are not valid arguments to a container's `Open()` method; they must be
translated by the client prior to communicating with the namespace.
See [FIDL Protocols](/docs/concepts/fidl/overview.md).

For example, `fdio` implements client-side interpretation of `..` paths
in file manipulation APIs such as `open()`, `stat()`, `unlink()`, etc.

## Namespace Transfer

When a component is instantiated in an environment (e.g. its process is
started), it receives a table that maps one or more namespace path prefixes
to object handles.

The path prefixes in the table encode the intended significance of their
associated objects by convention.  For example, the `pkg` prefix should
be associated with a directory object, which contains the component's own
binaries and assets.

More on this in the next section.

## Namespace Conventions

This section describes the conventional layout of namespaces for typical
components running on Fuchsia.

The precise contents and organization of a component's namespace varies
greatly depending on the component's role, type, identity, scope,
relation to other components, and rights. See [Sandboxing](sandboxing.md) for
information about how namespaces are used to create sandboxes for components.

_For more information about the namespace your component can expect to
receive from its environment, please consult the documentation related to
the component type you are implementing._

### Typical Objects

There are some typical objects that a component namespace might contain:

- Read-only executables and assets from the component's package.
- Private local persistent storage.
- Private temporary storage.
- Services offered to the component by the system, the component framework,
  or by the client that started it.
- Device nodes (for drivers and privileged components).
- Configuration information.

### Typical Directory Structure

- `pkg/`: the contents of the current program's package
  - `bin/`: executable binaries within the package
  - `lib/`: shared libraries within the package
  - `data/`: data, such as assets, within the package
- `data/`: local persistent storage (read-write, private to the package)
- `tmp/`: temporary storage (read-write, private to the package)
- `svc/`: services offered to the component
  - `fuchsia.process.Launcher`: launch processes
  - `fuchsia.logger.Log`: log messages
  - `vendor.topic.Interface`: service defined by a _vendor_
- `dev/`: device tree (relevant portions visible to privileged components as needed)
  - `class/`, ...
- `hub/`: introspect the system, see [Hub][hub] (privileged components only)
- `config/`: configuration data for the component

## Namespace Participants

Here is some more information about a few abstractions that interact with
and support the Fuchsia namespace protocol.

### Filesystems

Filesystems make files available in namespaces.

A filesystem is simply a component that publishes file-like objects
from someone else's namespace.

### Services

Services live in namespaces.

A service is a well-known object that provides an implementation of a FIDL
protocol, which can be discovered using the namespace.

A service name corresponds to a path within the `svc` branch of the namespace
from which a component can access an implementation of the service.

For example, the name of the default Fuchsia logging service is
`fuchsia.logger.Log` and its location in the namespace is
`svc/fuchsia.logger.Log`.

### Components

Components consume and extend namespaces.

A component is an executable program object that has been instantiated
within some environment and given a namespace.

A component participates in the Fuchsia namespace in two ways:

1. It can use objects from the namespace received from its environment,
   notably to access its own package contents and incoming services.

2. It can publish objects through its environment in the form of a namespace,
   parts of which its environment may subsequently make available to other
   components upon request.  This is how services are implemented by
   components.

### Environments

Environments construct namespaces.

An environment is a container of components.  Each environment is responsible
for _constructing_ the namespace for its components.

The environment decides what objects a component may access and how the
component's request for services by name will be bound to specific
implementations.

### Configuration

Components may have different kinds of configuration data exposed to them
depending on the features listed in their
[component manifest](/docs/concepts/components/v1/component_manifests.md)
which are exposed as files in the `/config` namespace entry. These are
defined by the feature set of the component.

[hub]: /docs/concepts/components/v2/hub.md
