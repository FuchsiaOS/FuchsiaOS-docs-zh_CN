<!--
# Storage capabilities
 -->
# 存储能力

<!--
[Storage capabilities][glossary.storage-capability] allocate per-component
*isolated* storage within a filesystem directory. This prevents component
instances from accessing files belonging to other components, including their
own children.
 -->
[存储能力][glossary.storage-capability]（storage capability）在文件系统目录中分配每个组件的“隔离”存储。这可以防止组件实例访问属于其他组件的文件，这包括其自己的子组件。

<!--
Different storage capabilities may be backed by different filesystems. A
component should not assume atomic IO operations are possible across storage
capabilities.
 -->
不同存储能力可能由不同文件系统支持。组件不应假设原子 IO 操作可以跨存储能力进行。

<!--
For information on directories that can be shared between components, see
[directory capabilities][directory-capabilities].
 -->
要获取关于可在组件间共享的目录的信息，请参阅[目录能力][directory-capabilities]。

<!--
## Standard storage capability names {#standard-names}
 -->
## 标准存储能力名称 {#standard-names}

<!--
Standard names are commonly used for storage capabilities. Each of these
standard names implies the storage capability should be used for a particular
purpose and provides a particular behavior. Any component that receives a
storage capability with one of these standard names may assume it provides the
behavior described below.
 -->
标准名称在存储能力中很常用。这些标准名称每一个都暗示着存储能力应该用于特定目的，并提供特定行为。任何接收具有标准名称存储能力的组件都可以假设它提供了如下所述的行为。

<!--
Note that a storage capability name does **not** necessarily globally identify a
storage capability. For example, on some products several different storage
capabilities named `data` exist at different locations in the component instance
topology. These storage capabilities are backed by different directories on
different storage volumes, but they all serve the same purpose for the component
instances using them.
 -->
请注意，存储能力名称**不一定**全局标识存储能力。例如，在某些产品上，名为 `data`（数据）的几项不同存储能力存在于组件实例拓扑中的不同位置。这些存储能力由不同存储卷上的不同目录支持，但均为使用它们的组件实例提供相同目的的服务。

<!--
Not all storage capabilities use one of these standard names. In these cases any
expectations about the behavior of the storage capability should be documented
where the storage capability is defined and at every place in the component
instance topology that the capability is renamed.
 -->
并非所有存储能力都使用这些标准名称。在这种情况下，应当将该存储能力的任何预期行为都记录在其定义的位置，以及组件实例拓扑中对其重命名的各个位置。

<!--
Note that during tests storage capabilities may be created that do not match
these behaviors. For example, an integration test may provide a `data`
capability that is wiped between test cases.
 -->
请注意，在测试期间，可以创建不匹配这些行为的存储能力。例如，集成测试可以提供 `data` 能力，该能力在不同测试用例之间会被擦除。

### `data`

<!--
Storage capabilities named "data" are intended to store general purpose
persistent data.
 -->
名为“data”（数据）的存储能力旨在存储通用持久数据。

<!--
A component may assume that files in these storage capabilities
will not be deleted by the system. Components must be conservative in their use
of `data` because the contract does not let system delete files when the
limited disk space is exhausted. In many cases using `cache` is preferable.
 -->
组件可以假设此类存储能力中的文件不会被系统删除。组件在使用 `data` 时必须保守，因为当有限的磁盘空间耗尽时，合约（contract）不允许系统删除文件。许多情况下，使用 `cache` 更为可取。

### `cache`

<!--
Storage capabilities named "cache" are intended to store data that could be
discarded or regenerated if necessary. For example, a downloaded picture that
could be re-fetched.
 -->
名为“cache”（缓存）的存储能力旨在存储可以在必要时丢弃或重新生成的数据。例如，可以重新获取的已下载图片。

<!--
Files stored in `cache` are usually persisted between different runs of same
component instance but this is not guaranteed. Files may be deleted by the
system at any time, even while the component is running.
 -->
存储在 `cache` 中的文件通常在同一组件实例的不同运行之间保持不变，但并不保证。文件可能随时被系统删除，即使在组件运行时。

### `tmp`

<!--
Storage capabilities named "tmp" are intended to store temporary or intermediate
data.
 -->
名为“tmp”（临时）的存储能力旨在存储临时或中间数据。

<!--
Files stored in `tmp` may be deleted by the system between runs of a component.
Files will not be deleted by the system while the component is running. `tmp`
will often be empty when a component is started but this is not guaranteed.
Components must not assume `tmp` will be empty on start but also should not use
any files that are present on start.
 -->
存储在 `tmp` 中的文件可能会在组件的不同运行之间被系统删除。组件运行时，系统不会删除文件。组件启动时，`tmp` 通常为空，但并不保证。组件不得假定 `tmp` 在启动时为空，也不应使用启动时刻前存在的任何文件。

<!--
## Backing directories {#backing-dir}
 -->
## 支持目录 {#backing-dir}

<!--
Each storage capability must be backed by a corresponding
[directory capability][glossary.directory-capability] to host an isolated
subdirectory for each component. When a component instance attempts to access
the directory provided to it through a storage capability, the framework
generates a unique subdirectory inside the backing directory for that component.
 -->
每项存储能力都必须由相应的[目录能力][glossary.directory-capability]支持（back），以为每个组件托管隔离的子目录。当组件实例尝试通过存储能力访问提供给它的目录时，框架会在该组件的支持目录（backing directory）内生成唯一的子目录。

<!--
Caution: The backing directory capability can also be routed directly to other
components. Providing this capability allows components to access all the
isolated storage directories it contains.
 -->
注意：支持目录能力也可以直接路由到其他组件。提供该能力将允许组件访问其包含的所有隔离存储目录。

<!--
The framework allocates storage subdirectories based on either the component
instance's [moniker][glossary.moniker] or a static
[instance ID][glossary.component-instance-identifier]. Each instance ID is a
256-bit globally unique identifier listed in a component ID index file.
 -->
该框架根据组件实例的[代称][glossary.moniker]或静态[实例 ID][glossary.component-instance-identifier] 分配存储子目录。每个实例 ID 都是一个 256 位的全局唯一标识符，列在组件 ID 索引文件中。

<!--
The following is an example entry in a component ID index file containing a
stable instance ID:
 -->
以下是包含稳定实例 ID 的组件 ID 索引文件中的示例条目：

```json5
{
    instances: [
        {
            instance_id: "47c3bf08f3e560c4dee659c28fa8d863dbdc0b1dbb74065e6cb1f38441ac759c",
            moniker: "/core/my_component",
        },
    ],
}
```

<!--
Instance IDs allow a component's storage to persist across changes to the
component's moniker, such as moving the component instance to a different realm.
Using a moniker is a good secondary option for tests or other use
cases where storage does not need to be durable.
 -->
实例 ID 允许组件的存储在组件名称发生变化时保持不变，例如将组件实例移动到不同的领域时。对于不需要持久存储的测试或其他用例，使用代称是一个很好的辅助选择。

<!--
For more details on instance IDs, see [Component ID index][component-id-index].
 -->
要获取关于实例 ID 的更多细节，请参阅[组件 ID 索引][component-id-index]。

<!--
## Providing storage capabilities {#provide}
 -->
## 提供存储能力 {#provide}

<!--
To provide a storage capability, a component must declare the capability and
[route](#route) it from `self`.
 -->
要提供存储能力，组件必须声明该能力，并从 `self` 对其[路由](#route)。

```json5
{
    capabilities: [
        {
            storage: "tmp",
            from: "self",
            backing_dir: "memfs",
            storage_id: "static_instance_id",
        },
    ],
}
```

<!--
You must specify [`backing_dir`](#backing-dir) with a valid directory capability
name.
 -->
您必须用有效的目录能力名称指定 [`backing_dir`](#backing-dir)（支持目录）。

<!--
The `from` field declares the component providing the backing directory.
You may supply a [component reference][component-reference] if the provider is
another component.
 -->
`from` 字段声明了提供支持目录的组件。如果提供者组件是另一组件，您可以提供[组件引用][component-reference]（component reference）。

<!--
## Routing storage capabilities {#route}
 -->
## 路由存储能力 {#route}

<!--
Storage capabilities cannot be exposed to a parent component. Components should
route the [backing directory](#backing-dir) to an appropriate parent component
where storage can be [declared](#provide) and [offered](#offer) to the necessary
children.
 -->
存储能力不能公开至父级。组件应将[支持目录](#backing-dir)路由至合适的父组件和子组件，它们应当可以[声明](#provide)和[提供](#offer)存储。

<!--
For more details on how the framework routes component capabilities,
see [capability routing][capability-routing].
 -->
要获取关于框架路由组件能力方式的更多细节，请参见[能力路由][capability-routing]。

<!--
### Offering {#offer}
 -->
### 提供 {#offer}

<!--
Offering a storage capability gives a child component access to that
capability:
 -->
提供存储能力会给予子组件访问该能力的权限：

```json5
{
    offer: [
        {
            storage: "data",
            from: "self",
            to: [ "#storage-user" ],
        },
    ],
}
```

<!--
## Consuming storage capabilities {#consume}
 -->
## 使用存储能力 {#consume}

<!--
To consume a storage capability, the component must request the capability and
open the corresponding path in its [namespace][glossary.namespace].
 -->
要使用（consume）存储能力，组件必须请求该能力并在其[命名空间][glossary.namespace]中打开相应路径。

<!--
To request the capability, add a `use` declaration for it:
 -->
要请求该能力，请为其添加 `use`（使用）声明：

```json5
{
    use: [
        {
            storage: "data",
            path: "/example_dir",
        },
    ],
}
```

<!--
This populates the component's namespace with a directory at the provided `path`
containing the isolated storage contents.
 -->
这将会使用位于所提供路径（`path`）处包含隔离存储内容的目录来填充组件的命名空间。

<!--
### Consuming optional storage capabilities
 -->
### 使用可选存储能力

<!--
See [Connect Components: Consuming optional capabilities][consuming-optional-capabilities].
 -->
请参阅[连接组件：使用可选能力][consuming-optional-capabilities]。

<!--
## Storage example {#example}
 -->
## 存储示例 {#example}

<!--
Consider the following example where component `A` requests isolated storage
`tmp` from its parent:
 -->
考虑以下示例，其中组件 `A` 向其父组件请求隔离存储 `tmp`：

```json5
// A.cml
{
    use: [
        {
            storage: "tmp",
            path: "/example_dir",
        },
    ],
}
```

<!--
This provides an isolated storage directory at `/example_dir` in the namespace
of component `A`.
The parent component `B` offers this capability to `A` using a backing directory
provided by the `memfs` component in the same realm:
 -->
这就在组件 `A` 命名空间中的 `/example_dir` 提供了一个隔离的存储目录。父组件 `B` 使用由同一领域中 `memfs` 组件提供的支持目录向 `A` 提供了该能力：

```json5
// B.cml
{
    capabilities: [
        {
            storage: "tmp",
            from: "#memfs",
            backing_dir: "memfs",
        },
    ],
    offer: [
        {
            storage: "tmp",
            from: "self",
            to: [ "#A" ],
        },
    ],
    children: [
        { name: "A", url: "fuchsia-pkg://...", },
        { name: "memfs", url: "fuchsia-pkg://..." },
    ],
}
```

<!--
For more details on implementing directories, see
[directory capabilities][directory-capabilities].
 -->
要获取关于实现目录的更多细节，请参阅[目录能力][directory-capabilities]。

[glossary.directory-capability]: /glossary/README.md#directory-capability
[glossary.component-instance-identifier]: /glossary/README.md#component-instance-identifier
[glossary.moniker]: /glossary/README.md#moniker
[glossary.namespace]: /glossary/README.md#namespace
[glossary.storage-capability]: /glossary/README.md#storage-capability
[capability-routing]: /concepts/components/v2/capabilities/README.md#routing
[consuming-optional-capabilities]: /development/components/connect.md#consuming-optional-capabilities
[component-reference]: https://fuchsia.dev/reference/cml#references
[directory-capabilities]: /concepts/components/v2/capabilities/directory.md
[component-id-index]: /development/components/component_id_index.md
