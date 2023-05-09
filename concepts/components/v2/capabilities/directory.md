<!--
# Directory capabilities
 -->
# 目录能力

<!--
[Directory capabilities][glossary.directory-capability] allow components
to connect to a directory provided by another component.
 -->
[目录能力][glossary.directory-capability]（directory capability）允许组件连接到另一组件提供的目录。

<!--
For information on directories that are isolated per-component, see
[storage capabilities][storage-capabilities].
 -->
要获取关于按组件隔离的目录的信息，请参阅[存储能力][storage-capabilities]。

<!--
## Providing directory capabilities {#provide}
 -->
## 提供目录能力 {#provide}

<!--
To provide a directory capability, a component must declare the capability and
[route](#route) it from `self`. The component hosts the directory capability in
its [outgoing directory][glossary.outgoing-directory].
 -->
要提供目录能力，组件必须声明该能力，并从 `self` 对其[路由](#route)。组件在其[传出目录][glossary.outgoing-directory]（outgoing directory）中托管目录能力。

<!--
To define the capability, add a `capabilities` declaration for it:
 -->
要定义该能力，请为其添加 `capabilities` 声明：

```json5
{
    capabilities: [
        {
            directory: "data",
            rights: ["r*"],
            path: "/published-data",
        },
    ],
}
```

<!--
This defines a capability hosted by this component whose outgoing directory path
is `/published-data`, and whose maximum usable
[rights](#directory-capability-rights) are "read-only".
 -->
这段声明定义了该传出目录路径为 `/published-data` 的组件托管的能力，其可用的最大[权利](#directory-capability-rights)（right）是“只读的”。

<!--
## Routing directory capabilities {#route}
 -->
## 路由目录能力 {#route}

<!--
Components route directory capabilities by [exposing](#expose) them to their
parent and [offering](#offer) them to their children.
 -->
组件通过将目录能力[公开](#expose)（expose）至其父级并[提供](#offer)（offer）至其子级来对其进行路由。

<!--
For more details on how the framework routes component capabilities,
see [capability routing][capability-routing].
 -->
要获取关于框架路由组件能力方式的更多细节，请参阅[能力路由][capability-routing]。

<!--
### Exposing {#expose}
 -->
### 公开 {#expose}

<!--
Exposing a directory capability gives the component's parent access to that
capability:
 -->
公开目录能力会给予父组件访问该能力的权限：

```json5
{
    expose: [
        {
            directory: "data",
            from: "self",
        },
    ],
}
```

<!--
You may optionally specify:
 -->
您可以选择性指定：

* [`as`](#renaming)
* [`rights`](#directory-capability-rights)
* [`subdir`](#subdirectories)

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
            directory: "data",
            from: "parent",
            to: [ "#child-a", "#child-b" ],
        },
    ],
}
```

<!--
You may optionally specify:
 -->
您可以选择性指定：

* [`as`](#renaming)
* [`rights`](#directory-capability-rights)
* [`subdir`](#subdirectories)

<!--
## Consuming directory capabilities {#consume}
 -->
## 使用目录能力 {#consume}

<!--
To consume a storage capability, the component must request the capability and
open the corresponding path in its [namespace][glossary.namespace].
 -->
要使用（consume）目录能力，组件必须请求该能力并在其[命名空间][glossary.namespace]中打开相应路径。

<!--
To request the capability, add a `use` declaration for it:
 -->
要请求该能力，请为其添加 `use`（使用）声明：

```json5
{
    use: [
        {
            directory: "data",
            rights: ["r*"],
            path: "/data",
        },
    ],
}
```

<!--
This populates the component's namespace with a directory at the provided `path`
containing the shared directory contents.
 -->
这将会使用位于所提供路径（`path`）处包含共享目录内容的目录来填充组件的命名空间。

<!--
You must specify [`rights`](#directory-capability-rights).
You may optionally specify [`subdir`](#subdirectories).
 -->
您必须指定 [`rights`](#directory-capability-rights)。您可以选择性指定 [`subdir`](#subdirectories)。

<!--
### Consuming optional directory capabilities
 -->
### 使用可选目录能力

<!--
See [Connect Components: Consuming optional capabilities][consuming-optional-capabilities].
 -->
请参阅[连接组件：使用可选能力][consuming-optional-capabilities]。

<!--
## Directory capability rights {#directory-capability-rights}
 -->
## 目录能力权利 {#directory-capability-rights}

<!--
Directory rights enable components to control access to directories as they are
routed throughout the system. Directory rights are applied as follows:
 -->
目录权利使组件能够在目录于整个系统中路由时对其进行访问控制。目录权利的应用如下：

<!--
* [`capabilities`][manifest-capabilities]: *Required*.
  Provides the base set of rights available for the directory. Any rights
  specified in a `use`, `offer`, or `expose` must be a subset of what is
  declared here.
* [`use`][manifest-use]: *Required*.
  Describes the access rights requested by the consuming component.
* [`offer`][manifest-offer]: *Optional*.
  Modified rights available to the destination component. Rights are inherited
  from the `offer` source if not present.
* [`expose`][manifest-expose]: *Optional*.
  Modified rights available to the destination component. Rights are inherited
  from the `expose` source if not present.
 -->
* [`capabilities`][manifest-capabilities]：**必需**。提供可用于目录的基本权利集。`use`、`offer` 或 `expose` 中指定的任何权利必须是此处声明内容的子集。
* [`use`][manifest-use]：**必需**。描述消费者组件请求的访问权限。
* [`offer`][manifest-offer]：**可选**。目标组件可用的已修改权限。如果不出现，权利将从 `offer` 来源继承。
* [`expose`][manifest-expose]：**可选**。目标组件可用的已修改权限。如果不出现，权利将从 `expose` 来源继承。

<!--
The `rights` field can contain any combination of the following
[`fuchsia.io.Rights`][fidl-io-rights] tokens:
 -->
`rights` 字段可包含以下 [`fuchsia.io.Rights`][fidl-io-rights] 令牌的任何组合：

```json5
rights: [
  "connect",
  "enumerate",
  "traverse",
  "read_bytes",
  "write_bytes",
  "execute_bytes",
  "update_attributes",
  "get_attributes",
  "modify_directory",
]
```

<!--
The framework provides a simplified form for declaring `rights` using *aliases*.
Each alias represents the combination of FIDL rights tokens to provide common
read, write, or execute access:
 -->
该框架使用“别名”（*alias*）为声明权利（`rights`）提供了简化的方式。每个别名代表 FIDL 权利令牌的组合，以提供通用的读、写或执行权限：

<!--
| Alias | FIDL rights                                                |
| :---: | ---------------------------------------------------------- |
| `r*`  | `connect, enumerate, traverse, read_bytes,`                |
:       : `get_attributes`                                           :
| `w*`  | `connect, enumerate, traverse, write_bytes,`               |
:       : `update_attributes, modify_directory`                      :
| `x*`  | `connect, enumerate, traverse, execute_bytes`              |
| `rw*` | `connect, enumerate, traverse, read_bytes, write_bytes,`   |
:       : `get_attributes, update_attributes, modify_directory`      :
| `rx*` | `connect, enumerate, traverse, read_bytes, execute_bytes,` |
:       : `get_attributes`                                           :
 -->
| 别名  | FIDL 权利                                                                                                    |
| ----- | ------------------------------------------------------------------------------------------------------------ |
| `r*`  | `connect, enumerate, traverse, read_bytes, get_attributes`                                                   |
| `w*`  | `connect, enumerate, traverse, write_bytes, update_attributes, modify_directory`                             |
| `x*`  | `connect, enumerate, traverse, execute_bytes`                                                                |
| `rw*` | `connect, enumerate, traverse, read_bytes, write_bytes, get_attributes, update_attributes, modify_directory` |
| `rx*` | `connect, enumerate, traverse, read_bytes, execute_bytes, get_attributes`                                    |

<!--
The `rights` field may only contain one alias. Additional FIDL rights may be
appended as long as they do not duplicate rights expressed by the alias.
 -->
`rights` 字段可以只包含一个别名。只要不与这些别名表示的权利重复，就可以附加额外的 FIDL 权利。

<!--
### Example
 -->
### 示例

<!--
Consider the following example where component `A` requests *read-write* access
to the `data` directory:
 -->
考虑以下示例，其中组件 `A` 请求访问 `data`（数据）目录的“读-写”（*read-write*）权限：

```json5
// A.cml
{
    use: [
        {
            directory: "data",
            rights: ["rw*"],
            path: "/data",
        },
    ],
}
```

<!--
However, the parent component `B` offers the directory `data` to component `A`
with only *read-only* rights. In this case the routing fails and `data` wouldn't
be present in A's namespace.
 -->
但是，父组件 `B` 仅向组件 `A` 提供了目录 `data` 的“只读”（*read-only*）权利。在这种情况下，路由失败，`data`不会存在于 `A` 的命名空间中。

```json5
// B.cml
{
    capabilities: [
        {
            directory: "data",
            rights: ["r*"],
            path: "/published-data",
        },
    ],
    offer: [
        {
            directory: "data",
            from: "self",
            to: [ "#A" ],
        },
    ],
}
```

<!--
## Subdirectories {#subdirectories}
 -->
## 子目录 {#subdirectories}

<!--
You may `expose`, `offer`, or `use` a subdirectory of a directory capability:
 -->
您可以公开（`expose`）、提供（`offer`）或使用（`use`）目录能力的子目录（subdirectory）：

```json5
{
    offer: [
        {
            directory: "data",
            from: "parent",
            to: [ "#child-a", "#child-b" ],
            subdir: "children",
        },
    ],
}
```

<!--
## Renaming directories {#renaming}
 -->
## 重命名目录 {#renaming}

<!--
You may `expose` or `offer` a directory capability by a different name:
 -->
您可用不同的名称公开（`expose`）或提供（`offer`）目录能力：

```json5
{
    offer: [
        {
            directory: "data",
            from: "#child-a",
            to: [ "#child-b" ],
            as: "a-data",
        },
    ],
}
```

[glossary.directory-capability]: /glossary/README.md#directory-capability
[glossary.outgoing-directory]: /glossary/README.md#outgoing-directory
[capability-routing]: /concepts/components/v2/capabilities/README.md#routing
[fidl-io-rights]: /sdk/fidl/fuchsia.io/rights-abilities.fidl
[manifest-capabilities]: https://fuchsia.dev/reference/cml#capabilities
[manifest-expose]: https://fuchsia.dev/reference/cml#expose
[manifest-offer]: https://fuchsia.dev/reference/cml#offer
[manifest-use]: https://fuchsia.dev/reference/cml#use
[consuming-optional-capabilities]: /development/components/connect.md#consuming-optional-capabilities
[storage-capabilities]: /concepts/components/v2/capabilities/storage.md
