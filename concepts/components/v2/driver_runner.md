<!--
# Driver runner
 -->
# 驱动程序运行器

<!--
The driver runner is the runner responsible for launching
[components][glossary.component] that run in the driver host environment.
 -->
驱动程序运行器是负责启动在驱动程序主机环境中运行的[组件][glosossary.component]的运行器。

<!--
## Using the driver runner
 -->
## 使用驱动程序运行器

<!--
To use the driver runner, the component's manifest must include a `program`
block similar to the following:
 -->
要使用驱动程序运行器，组件的清单必须包括一个类似于以下内容的 `program`（程序）块：

```json5 {:.devsite-disable-click-to-copy}
{
    program: {
        runner: "driver",
        binary: "driver/example.so",
        bind: "meta/bind/example.bindbc",
    }
}
```

<!--
A driver component's `program` block requires the following fields at a minimum:
 -->
驱动程序组件的 `program` 块最少需要以下字段：

<!--
-   `runner` – This field must be set to the string `driver`.
-   `binary` – The path to the driver's binary output in the component's
    package.
-   `bind` – The path to the compiled bind program in the component's package.
 -->
-   `runner`（运行器）——必须将该字段设置为字符串 `driver`。
-   `binary`（二进制文件）——组件的包中驱动程序二进制文件的输出路径。
-   `bind`（绑定）——组件包中已编译绑定程序的路径。

<!--
## Optional fields
 -->
## 可选字段

<!--
In additional to the required fields, the driver runner accepts a set of
optional fields, which are used to specify metadata or configure the runtime
environment of the driver component.
 -->
除了必填字段之外，驱动程序运行器还接受一组可选字段，用于指定元数据或配置驱动程序组件的运行时环境。

<!--
### Colocation
 -->
### 并置

<!--
If the `colocate` field is set to the string `true`, the driver will be put in
the same [driver host][driver-host] as its parent driver if possible. However
this is advisory. The [driver manager][driver-manager] may still put the driver
in a separate driver host, for instance, if the parent device has `MUST_ISOLATE`
set. In DFv1, a driver is always colocated if the parent device is a composite –
isolation may still be enforced by setting `MUST_ISOLATE` on the primary
fragment of the composite.
 -->
如果 `colocate`（并置）字段设置为字符串 `true`（真），驱动程序将尽可能放在与其父驱动程序相同的[驱动程序主机][driver-host]（driver host）中。不过，这是建议性的。[驱动程序管理器][driver-manager]仍然可能将驱动程序放在单独的驱动程序主机中，例如，如果父设备设置了 `MUST_ISOLATE`。在 DFv1 中，如果父设备是复合设备，则驱动程序总是并置———仍然可以通过在复合设备的主要部分上设置 `MUST_ISOLATE` 来强制执行隔离。

```json5 {:.devsite-disable-click-to-copy}
{
    program: {
        runner: "driver",
        binary: "driver/example.so",
        bind: "meta/bind/example.bindbc",
        {{ '<strong>' }}colocate: "true"{{ '</strong>' }}
    }
}
```

<!--
If the `colocate` field is not specified, its value defaults to the string
`false`.
 -->
如果未指定 `colocate` 字段，则其值默认为字符串 `false`（假）。

<!--
### Default dispatcher options
 -->
### 默认调度器选项

<!--
The `default_dispatcher_opts` field provides the options which are used when
creating the driver's [default dispatcher][driver-dispatcher], for example:
 -->
`default_dispatcher_opts` 字段提供了创建驱动程序[默认调度器][driver-dispatcher]时使用的选项，例如：

```json5 {:.devsite-disable-click-to-copy}
{
    program: {
        runner: "driver",
        binary: "driver/example.so",
        bind: "meta/bind/example.bindbc",
        {{ '<strong>' }}default_dispatcher_opts: [ "allow_sync_calls" ]{{ '</strong>' }}
    }
}
```

<!--
The options in this field correspond to the flags defined in this
[`types.h`][dispatcher-flags] file. Today, the supported options are:
 -->
该字段中的选项对应于该 [`types.h`][dispatcher-flags] 文件中定义的标志。如今，支持的选项是：

<!--
-   `allow_sync_calls`: This option indicates that the dispatcher may not
    share Zircon threads with other drivers. This setting allows the driver
    to make synchronous Banjo or FIDL calls on the dispatcher without
    deadlocking.
 -->
-   `allow_sync_calls`：该选项表示调度程序可能不会与其他驱动程序共享 Zircon 线程。该设置允许驱动程序在调度程序上进行同步 Banjo 或 FIDL 调用而不会出现死锁。

<!--
### Fallback
 -->
### 回退

<!--
If the `fallback` field is set to the string `true`, this fallback driver will
only attempt to bind once all the base driver packages are indexed. Furthermore,
if this driver matches to a node and a non-fallback driver matches to the same
node, the non-fallback driver will bind to the node instead.
 -->
如果将 `fallback`（回退）字段设置为字符串 `true`，则该回退驱动程序只会在所有基本驱动程序包被索引后才会尝试绑定。此外，如果该驱动程序与节点匹配，且非回退驱动程序匹配同一节点，则非回退驱动程序将绑定到节点。

```json5 {:.devsite-disable-click-to-copy}
{
    program: {
        runner: "driver",
        binary: "driver/example.so",
        bind: "meta/bind/example.bindbc",
        {{ '<strong>' }}fallback: "true"{{ '</strong>' }}
    }
}
```

<!--
If the `fallback` field is not specified, its value defaults to the string
`false`.
 -->
如果未指定 `fallback` 字段，则其值默认为字符串 `false`。

<!--
### Device categories
 -->
### 设备类别

<!--
The `device_categories` field provides metadata indicating the device categories
that the driver controls, for example:
 -->
`device_categories`（设备类别）字段提供元数据，指示驱动程序控制的设备类别，例如：

```json5 {:.devsite-disable-click-to-copy}
{
    program: {
        runner: "driver",
        binary: "driver/example.so",
        bind: "meta/bind/example.bindbc",
        {{ '<strong>' }}device_categories: [
            { category: "board", subcategory: "i2c" },
            { category: "sensor", subcategory: "temperature" },
        ]{{ '</strong>' }}
    }
}
```

<!--
This metadata is used to determine the tests that the driver will undergo during
its certification process. See the full list of device categories and
subcategories in the [FHCP schema][fhcp-schema].
 -->
该元数据用于确定驱动程序在认证过程中将进行的测试。请参阅 [FHCP 方案][fhcp-schema]中设备类别和子类别的完整列表。

<!--
## Further reading
 -->
## 深入阅读

<!--
For more detailed explanation of how drivers are bound, see
[Driver binding][driver-binding].
 -->
要获取更多关于驱动程序绑定方式的详细说明，请参阅[驱动程序绑定][driver-binding]。

<!-- Reference links -->

[glossary.component]: /glossary/README.md#component
[driver-host]: /concepts/drivers/driver_framework.md#driver_host
[driver-manager]: /concepts/drivers/driver_framework.md#driver_manager
[driver-dispatcher]: /concepts/drivers/driver-dispatcher-and-threads.md
[dispatcher-flags]: /sdk/lib/driver/runtime/include/lib/fdf/types.h
[fhcp-schema]: /build/drivers/FHCP.json
[driver-binding]: /concepts/drivers/driver_binding.md
