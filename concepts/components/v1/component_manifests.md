<!--
# Component manifests (Components v1)

<<../_v1_banner.md>>

A component manifest (.cmx) is a JSON file with the file extension `.cmx`.
Component manifests are often located in a package’s `meta/` directory. The
manifest contains information that declares how to run the component and what
resources it receives. In particular, the component manifest describes how
the component is sandboxed.

Here's a simple example of a cmx for an ELF binary component:

```
{
    "include": [
        "src/lib/syslog/client.shard.cmx"
    ],
    "program": {
        "binary": "bin/example_app",
        "args": [ "--example", "args" ]
    },
    "sandbox": {
        "system": [ "data/sysmgr" ],
        "services": [
            "fuchsia.posix.socket.Provider",
            "fuchsia.sys.Launcher"
        ]
    }
}
```

And one for a flutter/dart component:

```
{
    "program": {
        "data": "data/simple_flutter"
    },
    "runner": "flutter_jit_runner"
}
```
-->

# 组件清单（组件 v1）

<<../_v1_banner.md>>

组件清单 (.cmx) 是文件扩展名为“.cmx”的 JSON 文件。
组件清单通常位于包的 `meta/` 目录中,包含声明如何运行组件和
它收到资源的信息。 特别的是，组件清单描述了组件是如何被沙箱化的。

这是 ELF 二进制组件的 cmx 的简单示例：
```
{
    "include": [
        "src/lib/syslog/client.shard.cmx"
    ],
    "program": {
        "binary": "bin/example_app",
        "args": [ "--example", "args" ]
    },
    "sandbox": {
        "system": [ "data/sysmgr" ],
        "services": [
            "fuchsia.posix.socket.Provider",
            "fuchsia.sys.Launcher"
        ]
    }
}
```

还有一个用于 flutter/dart 组件的组件清单：

```
{
    "program": {
        "data": "data/simple_flutter"
    },
    "runner": "flutter_jit_runner"
}
```

<!--

## include

The optional `include` property describes zero or more other component manifest
files (or shards) to be merged into this component manifest.

In the example given above, the component manifest is including contents from a
file provided by the `syslog` library, thus ensuring that the component
functions correctly at runtime if it attempts to write to syslog. By convention
such files end with `.shard.cmx`.

If working in fuchsia.git, include paths are relative to the source root of the
Fuchsia tree.

You can review the outcome of merging any and all includes into a component
manifest file by invoking the following command:

```sh
fx cmc include {{ "<var>" }}cmx_file{{ "</var>" }} --includepath $FUCHSIA_DIR
```

Includes can be recursive, meaning that shards can have their own includes.

-->

## include 属性

可选的 `include` 属性描述了零个或多个（或分片）组件清单要被合并到此组件清单中。

在上面给出的示例中，组件清单包含来自`syslog` 库提供的文件的内容，
从而确保组件在尝试写入 syslog 时在运行时正常运行。 按照惯例，此类文件以`.shard.cmx`结尾。

如果在 fuchsia.git 中工作，包含路径是相对于 Fuchsia 树的源根的。

您可以通过调用以下命令，查看将任何和所有包含合并到组件清单文件的结果：

```sh
fx cmc include {{ "<var>" }}cmx_file{{ "</var>" }} --includepath $FUCHSIA_DIR
```
include 是可以是递归的，这意味着分片可以有自己的include文件。

<!--
## program

The `program` property describes the resources to execute the component.

If [`runner`](#runner) is absent, the `program` property is a JSON object with
the following schema:

```
{
    "type": "object",
    "properties": {
        "binary": {
            "type": "string"
        },
        "args": {
            "type": "array",
            "items": {
                "type": "string"
            },
        },
        "env_vars": {
            "type": "array",
            "items": {
                "type": "string"
            },
        },
    }
}
```

The `binary` property describes where in the package namespace to find the
binary to run the component, and the optional `args` property contains the
string arguments to be provided to the process. The optional `env_vars`
property specifies environment variables to provide to the binary where
each element in the array uses the format `"VAR=VALUE"`, for example 
`"RUST_BACKTRACE=1"`.

If [`runner`](#runner) is present, `program` is a freeform string-string JSON
object interpreted as args to pass to the runner.

For instance, for a flutter/dart component, its format is:

```
{
    "type": "object",
    "properties": {
        "data": {
            "type": "string"
        }
    }
}
```

Where `data` should describe the location of the flutter/dart binaries. By
default, it is under `data/<component-name>`.

-->

## program 属性

`program` 属性描述了执行组件的资源。

如果  [`runner`](#runner)  不存在，则`program`属性是具有以下方案的 JSON 对象：
```
{
    "type": "object",
    "properties": {
        "binary": {
            "type": "string"
        },
        "args": {
            "type": "array",
            "items": {
                "type": "string"
            },
        },
        "env_vars": {
            "type": "array",
            "items": {
                "type": "string"
            },
        },
    }
}
```

`binary` 属性描述了在包命名空间中的哪个位置可以找到运行组件的二进制文件，可选的 `args` 属性包含要提供给进程的字符串参数。
可选的 env_vars 属性指定要提供给二进制文件的环境变量，其中数组中的每个元素都使用`"VAR=VALUE"`格式，例如`"RUST_BACKTRACE=1"`。

如果 [`runner`](#runner)存在，则  `program` 是一个自由格式的字符串 JSON 对象，被解释为传递给 runner 的 args。

例如，对于 flutter/dart 组件，其格式为：

```
{
    "type": "object",
    "properties": {
        "data": {
            "type": "string"
        }
    }
}
```
<!--

## runner

`runner` is an optional property that names another component (or a package
that contains one) to which execution is to be delegated. The target component
must expose the [`Runner`][runner] service.

If `runner` is present, [`program`](#program) is a freeform string-string JSON
object interpreted as args to pass to the runner.

If `runner` is absent, it is assumed that `program.binary` is an ELF binary or
shell script.

The `runner` property is a JSON string.
-->

## runner 属性

`runner` 是一个可选属性，用于命名将被执行委派的另一个组件（或包包含的一个组件) 。
目标组件必须暴露 [`Runner`][runner] 服务。

如果存在 `runner`属性，则 [`program`](#program) 是一个自由格式的字符串-字符串 JSON
对象，被解释为传递给runner的 参数。

如果 `runner` 不存在，则假定 `program.binary` 是一个 ELF 二进制文件或 shell脚本。

<!--

## facets

`facets` is an optional property that contains free-form JSON about the
component. Facets can be consumed by things on the system to acquire additional
metadata about a component.

The schema for `facets` is:

```
{
    "type": "object"
}
```

As an example of a facet, the `fuchsia.test` field is used to convey what
additional services should be
[injected into testing environments][test-components].

-->

## facets 属性

`facets` 是一个可选属性，包含关于组件的自由格式 JSON。
系统上的事物可以使用 Facet 来获取有关组件的其他元数据。

`facets` 的格式是：
```
{
    "type": "object"
}
```

作为 facet 的一个例子，`fuchsia.test` 字段用于传达额外的服务的具体内容：
[注入测试环境][test-components].

<!--

## sandbox

The `sandbox` property controls the environment in which the component
executes. Specifically, the property controls which directories the component
can access during execution.


The `sandbox` property is a JSON object with the following schema:

```
{
    "type": "object",
    "properties": {
        "dev": {
            "type": "array",
            "items": {
                "type": "string"
            }
        },
        "services": {
            "type": "array",
            "items": {
                "type": "string"
            }
        },
        "system": {
            "type": "array",
            "items": {
                "type": "string"
            }
        },
        "pkgfs": {
            "type": "array",
            "items": {
                "type": "string"
            }
        },
        "features": {
            "type": "array",
            "items": {
                "type": "string"
            }
        }
    }
}
```

-->

## sandbox

`sandbox` 属性控制组件执行所处的环境。具体来说，该属性控制组件在执行期间可访问的目录。
`sandbox` 属性是一个具有以下架构的 JSON 对象：

```
{
    "type": "object",
    "properties": {
        "dev": {
            "type": "array",
            "items": {
                "type": "string"
            }
        },
        "services": {
            "type": "array",
            "items": {
                "type": "string"
            }
        },
        "system": {
            "type": "array",
            "items": {
                "type": "string"
            }
        },
        "pkgfs": {
            "type": "array",
            "items": {
                "type": "string"
            }
        },
        "features": {
            "type": "array",
            "items": {
                "type": "string"
            }
        }
    }
}
```

<!--
The `dev` array contains a list of well-known device directories that are
provided to the component. For example, if the string `class/input` appears in
the `dev` array, then `/dev/class/input` will appear in the namespaces of components
loaded from the package. To allow access to a `misc` device, add the string `misc`
to the `dev` array. Allowing access to individual `misc` devices is not possible.

-->

`dev` 数组包含一个众所周知的设备目录列表，这些目录是提供给组件的。例如，如果字符串 `class/input` 出现在
`dev` 数组，然后 `/dev/class/input` 将出现在从包中加载的组件的命名空间中。要允许访问 `misc` 设备，请添加字符串 `misc`
到 `dev` 数组。不允许访问单个“misc”设备。

<!--
The `system` array contains a list of well-known paths within the system package
that are provided to the component. For example, if the string `bin` appears
in the `system` array, then `/system/bin` will appear in the namespaces of
components loaded from the package.

The `pkgfs` array contains a list of well-known paths within the pkgfs tree
that are provided to the component. For example, if the string `versions`
appears in the `pkgfs` array, then `/pkgfs/versions` will appear in the
namespaces of components loaded from the package, providing access to all
packages fully cached on the system.
-->

`system` 数组包含系统包中提供给组件的已知路径的列表。例如，如果出现字符串`bin`
在 `system` 数组中，那么 `/system/bin` 将出现在从包加载的组件。

`pkgfs` 数组包含 pkgfs 树中提供给组件的已知路径的列表。例如，如果字符串 `versions`
出现在 `pkgfs` 数组中，然后 `/pkgfs/versions` 会出现在从包加载的组件的命名空间中，
从而可访问所有完全缓存在系统的包。

<!--
The `services` array defines a list of services from `/svc` that the
component may access. A typical component will require a number services from
`/svc` in order to play some useful role in the system. For example, if
`"services" = [ "fuchsia.posix.socket.Provider", "fuchsia.sys.Launcher" ]`, the
component will have the ability to launch other components and access network
services. A component may declare any list of services in its `services`,
but it will only be able to access services present in its
[environment](/docs/glossary.md#environment). This property should be defined by
all new components, and soon a migration will take place to convert all
components to define `services`.
-->

`services` 数组定义了组件可以访问的来自 `/svc` 的服务列表。一个典型的组件将需要来自
`/svc`的数字服务以便在系统中发挥一些有用的作用。 例如，如果
`"services" = [ "fuchsia.posix.socket.Provider", "fuchsia.sys.Launcher" ]`,，
该组件将具有启动其他组件和访问网络服务的能力。 一个组件可以在其`services` 中声明任何服务列表，
但它只能访问其[环境](/docs/glossary.md#environment) 中存在的服务。
这个属性应该由所有新组件定义，很快就会发生迁移以将所有组件转换为定义“服务”。

<!--
The `features` array contains a list of well-known features that the package
wishes to use. Including a feature in this list is a request for the environment
in which the contents of the package execute to be given the resources required
to use that feature.

The set of currently known features are as follows:
-->

`features` 数组包含软件包希望使用的众所周知的特性列表。
在此列表中包含一个功能是执行环境对包内容的请求，以获得使用该功能所需的资源。

目前已知的一组特征如下：
<!--
- `config-data`, which will provide any configuration data available to the
  package this component is in that was provided in the [config-data](/docs/development/components/data.md)
  package on the system.

- `introspection`, which requests access to introspect the system. The
  introspection namespace will be located at `/info_experimental`.

- `isolated-cache-storage`, which requests access to persistent storage for the
  device, located in `/cache` in the package's namespace. This storage is
  isolated from the storage provided to other components. Unlike
  `isolated-persistent-storage`, items placed in the storage provided by this
  feature will be deleted by the system to reclaim space when disk usage is
  nearing capacity.

-->

- `config-data`，它将提供任何可用的配置数据。该组件所在的包在 [config-data](/docs/development/components/data.md) 中提供
  系统上的包。

- `introspection`, 它请求访问以检查系统。命名空间将位于`/info_experimental`。

- `isolated-cache-storage`，它请求访问设备的持久存储，位于包命名空间的 `/cache` 中。 此存储与提供给其他组件的存储隔离。 不像
  `isolated-persistent-storage`，当磁盘使用量接近容量时，系统将删除放置在此功能提供的存储中的项目以回收空间。

<!--

- `isolated-temp`, which requests that a temp directory be installed into the
  component's namespace at `/tmp`. This is isolated from the system temp and
  the temp directories of other component instances. This directory is backed by
  an in-memory filesystem, and is thus cleared on device reboots.

- `root-ssl-certificates`, which requests access to the root SSL certificates
  for the device. These certificates are provided in the `/config/ssl` directory
  in the package's namespace.

- `hub`, which shows information about the component instance's realm and its
  children in a [directory structure][hub].
-->

- `isolated-temp`, 它要求将临时目录安装到组件的`/tmp`命名空间。 这与系统临时目录和其他组件实例的临时目录隔离。
  该目录由内存文件系统支持，因此在设备重新启动时被清除。

- `root-ssl-certificates`, 它请求访问设备的根 SSL 证书。 这些证书在包命名空间的`/config/ssl` 目录中。

- `hub`, 它在 [目录结构][hub] 中显示有关组件实例的领域及其子项的信息。

<!--

- `deprecated-shell`, which requests access to the resources appropriate for an
  interactive command line. Typically, shells are granted access to all the
  resources available in the current environment. The `deprecated-shell` feature
  also implies the `root-ssl-certificates` and `hub` features.
  As the name suggests, this feature is to be removed. Current uses of this
  feature are explicitly allowlisted, and new uses are discouraged.

- `shell-commands`, which requests access to the currently available shell
  binaries (note: not "installed", but "available"). Binaries are mapped into
  `/bin` in the requesters namespace. Running these commands may require the
  `fuchsia.process.Resolver` and `fuchsia.process.Launcher` services also
  be requested.

- `vulkan`, which requests access to the resources required to use the Vulkan
  graphics interface. This adds layer configuration data in the `/config/vulkan`
  directory in the package's namespace.

-->

- `deprecated-ambient-replace-as-executable`, which provides legacy support for
  using the invalid handle with replace_as_executable.

- `factory-data`, which requests access to the read-only factory partition for
  the device and places it at `/factory` in the component's namespace.

- `durable-data`, which requests access to the read-write durable partition for
  the device and places it at `/durable` in the component's namespace. This
  partition is for storing persistent data that will survive a factory reset,
  and is only to be used for specific, approved use cases.

See [sandboxing](/docs/concepts/process/sandboxing.md) for more information about sandboxing.

[hub]: /docs/concepts/components/v1/hub.md
[runner]: /sdk/fidl/fuchsia.sys/runner.fidl
[test-components]: /docs/concepts/testing/v1_test_component.md
