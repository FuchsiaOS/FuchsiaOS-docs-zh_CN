<!-- 
# Logging in Go

Go programs on Fuchsia generally use the [syslog package] and its `syslog.Infof()` functions.

See the [language agnostic logging docs](/docs/concepts/diagnostics/logs/README.md) for more information
about recording and viewing logs.
-->

# Go 语言中的 Logging

Fuchsia 上的 Go 程序通常使用 [syslog package] 及其中的 `syslog.Infof()` 函数。

有关记录和查看日志的更多信息，请参阅 [语言无关的日志文档](/docs/concepts/diagnostics/logs/README.md)
<!--
## Requirements

### GN dependencies

The necessary packages can be included with an addition to `deps` in `BUILD.gn`:

```
deps = [
    "//src/lib/component",
    "//src/lib/syslog/go",
]
```

See [Go: Overview][go-dev] for more information about building Go within Fuchsia.
-->

## Requirements

### GN 依赖

必要的 package 包含在 `BUILD.gn` 中的 `deps` 中：

```
deps = [
    "//src/lib/component",
    "//src/lib/syslog/go",
]
```

有关在 Fuchsia 中构建 Go 的更多信息，请参阅 [Go: Overview][go-dev]。

<!--
### Component manifest dependency

Ensure that your component has the required capabilities to log by including the
following in your component manifest:

   * {.cmx}

   ```json
   {
     "include": [
       "sdk/lib/diagnostics/syslog/client.shard.cmx"
     ],
     ...
   }
   ```

   * {.cml}

   ```json5
   {
     include: [
       "sdk/lib/diagnostics/syslog/client.shard.cml"
     ],
     ...
   }
   ```

Note: The above is only available for in-tree development.
This is tracked in [fxbug.dev/64207](http://fxbug.dev/64207).
Out of tree developers should copy the snippets shown below instead.

   * {.cmx}

   ```json
   {
     "sandbox": {
       "services": [
         "fuchsia.logger.LogSink"
       ]
     },
     ...
   }
   ```

   * {.cml}

   ```json5
   {
     use: [
       { protocol: "fuchsia.logger.LogSink" },
     ],
     ...
   }
   ```

The syslog library will fallback to `stderr` if the `LogSink` connection fails.
-->

### 组件依赖清单

通过在组件清单中包含以下内容，确保您的组件具有记录所需的功能：

    * {.cmx}

   ```json
   {
     "include": [
       "sdk/lib/diagnostics/syslog/client.shard.cmx"
     ],
     ...
   }
   ```

    * {.cml}

   ```json5
   {
     include: [
       "sdk/lib/diagnostics/syslog/client.shard.cml"
     ],
     ...
   }
   ```

注意：以上仅适用于 in-tree 开发。
这在 [fxbug.dev/64207](http://fxbug.dev/64207) 中进行了跟踪。
Out of tree 开发人员应该复制下面所述片段。

    * {.cmx}

   ```json
   {
     "sandbox": {
       "services": [
         "fuchsia.logger.LogSink"
       ]
     },
     ...
   }
   ```

    * {.cml}

   ```json5
   {
     use: [
       { protocol: "fuchsia.logger.LogSink" },
     ],
     ...
   }
   ```

如果 `LogSink` 连接失败，syslog 库将回退到 `stderr`。

<!--
## Initialization

```golang
import (
    "go.fuchsia.dev/fuchsia/src/lib/component"
    syslog "go.fuchsia.dev/fuchsia/src/lib/syslog/go"
)

func main() {
  ctx := component.NewContextFromStartupInfo()
  {
    // Global tags, max 4 tags can be passed. Every log message is tagged with these.
    l, err := syslog.NewLoggerWithDefaults(ctx.Connector(), "my_tag")
    if err != nil {
      panic(err)
    }
    syslog.SetDefaultLogger(l)
  }
}
```
-->

## 初始化

初始化时如果没有任何 tag 的话，将默认使用进程名称。

```golang
import (
    "go.fuchsia.dev/fuchsia/src/lib/component"
    syslog "go.fuchsia.dev/fuchsia/src/lib/syslog/go"
)

func main() {
  ctx := component.NewContextFromStartupInfo()
  {
    // 全局 tag, 最多可传递 4 个 tag. 每条日志消息都标有这些 tag.
    l, err := syslog.NewLoggerWithDefaults(ctx.Connector(), "my_tag")
    if err != nil {
      panic(err)
    }
    syslog.SetDefaultLogger(l)
  }
}
```

<!--
## Recording messages

The log methods have two variants: `Levelf` and `LevelTf` (e.g. `Infof` and `InfoTf`). The variant
of each method with a `T` accepts an additional tag for the message.

```golang
syslog.Infof("my msg: %d", 10);          // maps to INFO

// Allow message specific tagging. This message is going to be tagged with
// this local tag and any global tag passed during initialization.
syslog.InfoTf("tag", "my msg: %d", 10);

syslog.Warnf("my msg: %d", 10);          // maps to WARN
syslog.WarnTf("tag", "my msg: %d", 10);

syslog.Errorf("my msg: %d", 10);         // maps to ERROR
syslog.ErrorTf("tag", "my msg: %d", 10);

syslog.Fatalf("my msg: %d", 10);         // maps to FATAL
syslog.FatalTf("tag", "my msg: %d", 10);
```
-->

## 记录 Message

日志方法有两种格式：`Levelf` 和 `LevelTf`（例如`Infof` 和`InfoTf`）。 每个方法种带有 “T” 的格式都接受 Message 的附加标签。

```golang
syslog.Infof("my msg: %d", 10);          // 映射到 INFO

// 允许消息特定标记。
// 此消息将使用此本地标记, 并且初始化期间传递的任何全局标记进行标记。
syslog.InfoTf("tag", "my msg: %d", 10);

syslog.Warnf("my msg: %d", 10);          // 映射到 WARN
syslog.WarnTf("tag", "my msg: %d", 10);

syslog.Errorf("my msg: %d", 10);         // 映射到 ERROR
syslog.ErrorTf("tag", "my msg: %d", 10);

syslog.Fatalf("my msg: %d", 10);         // 映射到 FATAL
syslog.FatalTf("tag", "my msg: %d", 10);
```

<!--
## Standard streams

`fmt.Printf()`, `fmt.Sprintf()` etc. go to standard out (`stdout`) and standard error (`stderr`).

See [`stdout` & `stderr`] in the language-agnostic logging docs for details on the routing of stdio
streams in the system.

[syslog package]: /src/lib/syslog/go
[`.cmx` file]: /docs/concepts/components/v1/component_manifests.md
[go-dev]: /docs/development/languages/go/README.md
[`stdout` & `stderr`]: /docs/development/diagnostics/logs/recording.md#stdout-stderr
-->

## 标准输入输出流

`fmt.Printf()`、`fmt.Sprintf()` 等，标准输出（`stdout`）和标准错误（`stderr`）。

有关系统中 stdio 流路由的详细信息，请参阅与语言无关的日志记录文档中的 [`stdout` 和 `stderr`]。

[syslog package]: /src/lib/syslog/go
[`.cmx` file]: /docs/concepts/components/v1/component_manifests.md
[go-dev]: /docs/development/languages/go/README.md
[`stdout` & `stderr`]: /docs/development/diagnostics/logs/recording.md#stdout-stderr
