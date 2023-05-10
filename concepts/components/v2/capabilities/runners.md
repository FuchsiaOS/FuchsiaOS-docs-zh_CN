<!--
# Component Runners
 -->
# 组件运行器

<!--
Component runners extend the component framework through an
[environment][glossary.environment] to provide a runtime for launching new
component instances.
 -->
组件运行器（component runner）通过[环境][glossary.environment]扩展组件框架，以提供启动新组件实例的运行时。

<!--
Component manager launches components by sending a request containing
[`ComponentStartInfo`][fidl-runner] to the appropriate runner using the
[`fuchsia.component.runner.ComponentRunner`][fidl-runner] protocol.
The `ComponentStartInfo` contains details about the component's executable and
its [namespace][glossary.namespace]. The runner manages the component's
execution within the supported runtime.
 -->
 <!--TODO-->
组件管理器通过使用 [`fuchsia.component.runner.ComponentRunner`][fidl-runner] 协议，向适当的运行器发送包含 [`ComponentStartInfo`][fidl-runner] 的请求来启动组件。`ComponentStartInfo` 包含关于组件的可执行文件及其[命名空间][glossary.namespace]的详细信息。运行器管理组件在支持的运行时内的执行。

<!--
After starting the component, component manager uses the
[`fuchsia.component.runner.ComponentController`][fidl-runner] protocol provided
in the [`Start`][fidl-runner] request to send execution actions to the runner,
such as stopping the component. The runner chooses how to interpret these
commands as appropriate to the component runtime.
 -->
启动组件后，组件管理器使用 [`Start`][fidl-runner] 请求中提供的 [`fuchsia.component.runner.ComponentController`][fidl-runner] 协议向运行器发送执行动作，例如停止组件。运行器选择如何解释这些命令以适合组件运行时。

<!--
## Providing runner capabilities {#provide}
 -->
## 提供运行器能力 {#provide}

<!--
To provide a runner capability, a component must declare a `runner`
capability, whose `path` designates a FIDL protocol implementing
[`fuchsia.component.runner.ComponentRunner`][fidl-runner] served from the
component's [outgoing directory][glossary.outgoing-directory].
 -->
要提供解析器能力，组件必须声明解析器（`runner`）能力，其路径（`path`）指定一个实现了提供自组件[传出目录][glossary.outgoing-directory] 的 [`fuchsia.component.runner.ComponentRunner`][fidl-resolver] 的 FIDL 协议。

```json5
{
    capabilities: [
        {
            runner: "web",
            path: "/svc/fuchsia.component.runner.ComponentRunner",
        },
    ],
}
```

<!--
Component manager sends `ComponentRunner/Start` requests to this protocol.
Each request includes a [`ComponentController`][fidl-controller] channel which
the runner should serve to handle lifecycle events for the component.
 -->
组件管理器向此协议发送 `ComponentRunner/Start` 请求。每个请求都包含一个 [`ComponentController`][fidl-controller] 通道，运行器应当使用该通道处理组件的生命周期事件。

<!--
## Routing runner capabilities {#route}
 -->
## 路由运行器能力 {#route}

<!--
Components route runner capabilities by [exposing](#expose) them to their
parent and [offering](#offer) them to their children.
 -->
组件通过将运行器能力[公开](#expose)（expose）至其父级并[提供](#offer)（offer）至其子级对其进行路由。

<!--
For more details on how the framework routes component capabilities,
see [capability routing][capability-routing].
 -->
要获取关于框架路由组件能力方式的更多细节，请参见[能力路由][capability-routing]。

<!--
### Exposing {#expose}
 -->
### 公开 {#expose}

<!--
Exposing a runner capability gives the component's parent access to that
capability:
 -->
公开运行器能力会给予父组件访问该能力的权限：

```json5
{
    expose: [
        {
            runner: "web",
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

<!--
### Offering {#offer}
 -->
### 提供 {#offer}

<!--
Offering a runner capability gives a child component access to that
capability:
 -->
提供运行器能力会给予子组件访问该能力的权限：

```json5
{
    offer: [
        {
            runner: "web",
            from: "self",
            to: [ "#child-a" ],
        },
    ],
}
```

<!--
You may optionally specify:
 -->
您可以选择性指定：

* [`as`](#renaming)

<!--
## Registering a component runner {#register}
 -->
## 注册组件运行器 {#register}

<!--
Component runners are made available to components through their
[environment][environment]. To register a new runner within an environment,
add a new entry to the `runners` section of the `environments` declaration:
 -->
组件运行器是通过[环境][environment]提供给组件的。要在环境中注册新的运行器，请将新条目添加到 `environments`（环境）声明的 `runners` 部分：

```json5
environments: [
    {
        name: "my-environ",
        extends: "realm",
        runners: [
            {
                runner: "web",
                from: "parent",
            },
        ],
    },
]
```

<!--
You may optionally specify:
 -->
您可以选择性指定：

* [`as`](#renaming)

<!--
For more details on how to apply environments to components, see the
[environments documentation][environment].
 -->
要获取关于将环境应用到组件的更多细节，请参阅[环境文档][environment]。

<!--
## Selecting a runner
 -->
## 选择运行器

<!--
A component specifies the appropriate runner for execution using the `program`
section of its manifest. The `program` section designates the `runner` as well
as any runner-specific options. The runner must be [registered](#register) in
the component's environment.
 -->
组件使用其清单的 `program`（程序）部分指定要执行的适当运行程序。`program` 部分指定了 `runner` 以及任何特定于运行器的选项。运行器必须在组件的环境中[注册](#register)。

<!--
For example, a component which runs as a web page might have a `program` like
the following:
 -->
例如，作为网页运行的组件的 `program` 可能如下：

```json5
program: {
    runner: "web",
    mode: "incognito",
},
```

<!--
When the component manager attempts to launch this component, it will send a
request to the provider of the `web` runner to start it.
 -->
组件管理器在尝试启动该组件时，会向 `web` 运行器的提供者发送启动请求。

<!--
## Renaming runners {#renaming}
 -->
## 重命名运行器 {#renaming}

<!--
You may `expose`, `offer`, or [register](#register) the runner capability under
a different name using the `as` parameter:
 -->
您可以使用 `as` 参数以不同名称公开（`expose`）、提供（`offer`）或[注册](#register)运行器能力：

```json5
{
    expose: [
        {
            runner: "web",
            from: "#chromium",
            as: "web-chromium",
        },
    ],
}
```

<!--
## Framework runners {#framework}
 -->
## 框架运行器 {#frameWork}

<!--
Component framework provides the following built-in component runners:
 -->
组件框架提供以下内置组件运行器：

<!--
-   [ELF runner][elf-runner]: Runs binaries compiled to the ELF file format.
 -->
-   [ELF 运行器][elf-runner]：运行编译为 ELF 文件格式的二进制文件。

```json5
{
    program: {
        runner: "elf",
        binary: "bin/example",
    },
}
```

[glossary.environment]: /glossary/README.md#environment
[glossary.namespace]: /glossary/README.md#namespace
[glossary.outgoing-directory]: /glossary/README.md#outgoing-directory
[capability-routing]: /concepts/components/v2/capabilities/README.md#routing
[elf-runner]: /concepts/components/v2/elf_runner.md
[environment]: /concepts/components/v2/environments.md
[fidl-directory]: /sdk/fidl/fuchsia.io/directory.fidl
[fidl-runner]: https://fuchsia.dev/reference/fidl/fuchsia.component.runner#ComponentRunner
[fidl-controller]: https://fuchsia.dev/reference/fidl/fuchsia.component.runner#ComponentController
