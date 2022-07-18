<!--
# Logging in Rust

Rust programs on Fuchsia generally use the [log crate] and its `info!/warn!/error!` macros. The
[`fuchsia_syslog`] crate is used for initialization.

See the [language agnostic logging docs](/docs/concepts/diagnostics/logs/README.md) for more information
about recording and viewing logs.

## Requirements

### GN dependencies

The necessary crates can be included with two additions to `deps` in `BUILD.gn`:

```gn
deps = [
  "//src/lib/syslog/rust:syslog",   # for initialization
  "//third_party/rust_crates:log",  # for recording messages
]
```

See [Rust: Overview][rust-dev] for more information about building Rust within Fuchsia.
-->

# Rust 日志

在 Fuchsia 系统中的 Rust 程序一般使用 [log crate] 和 它的 `info!/warn!/error!` 宏。[`fuchsia_syslog`] crate 被用来做初始化。

查看 [语言无关日志文档](/concepts/diagnostics/logs/README.md) 获取更多关于记录和查看日志的信息。

## 要求

### GN 依赖

必须的 crate 可以在 `BUILD.gn` 中通过添加两个添加项到 `deps` 而包含进来：

```gn
deps = [
  "//src/lib/syslog/rust:syslog",   # 用于初始化
  "//third_party/rust_crates:log",  # 用于记录消息
]
```

查看 [Rust: 概述][rust-dev] 获取更多关于在 Fuchsia 上构建 Rust 的信息。

<!--
### Component manifest dependency {#manifest}

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
   }
   ```

The syslog library will fallback to `stderr` if the `LogSink` connection fails.
-->

### 组件清单依赖 {#menifest}

把下边的内容包容到你的组件清单中来确保你的组件拥有要求的能力以打开日志：

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

注意：上边的配置只可用于 Fuchsia 树中的开发。关于这个问题可以在 [fxbug.dev/64207](http://fxbug.dev/64207) 进行追踪。而在树外，开发者应该拷贝下边的片段来代替：

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
   }
   ```
如果 `LogSink` 连接失败，系统日志库会回退至 `stderr` 。

<!--
## Initialization

The Rust `log` crate must be connected to the Fuchsia backend before it can emit messages. Failure
to initialize the library will result in dropped messages.

### Basic

In your component's `main.rs`:

```rust
use fuchsia_syslog as syslog;

fn main() {
    // configures a single tag with the process name
    syslog::init().unwrap();
}
```
-->

## 初始化

Rust 的 `log` crate 在可以发送消息之前必须连接到 Fuchsia 后端。库初始化失败会导致消息丢失。

### 基本配置

在你的组件的 `main.rs` 中：

```rust
use fuchsia_syslog as syslog;

fn main() {
    // 使用进程名来配置一个单一标签
    syslog::init().unwrap();
}
```

<!--
### With tags

By default the process name is used as the tag for all log messages. Tags can be used to further
categorize log messages from the program, and they can also be globally overridden.

```rust
use fuchsia_syslog as syslog;

fn main() {
    // overrides the use of process name for tag
    syslog::init_with_tags(&["my_tags"]).unwrap();

    // additional tag for just this message
    syslog::fx_log_info!(tag: "init", "an update on program initialization");
}
```
-->

### 标签

默认的，所有的日志消息使用进程名来做为标签。标签可以在程序的更多分类日志消息里使用，它们也可以被全局重载。

```rust
use fuchsia_syslog as syslog;

fn main() {
    // 为标签重载进程名的使用
    syslog::init_with_tags(&["my_tags"]).unwrap();

    // 仅为这个消息使用额外的标签
    syslog::fx_log_info!(tag: "init", "an update on program initialization");
}
```

<!--
### Configure severity

The syslog crate starts at `INFO` severity but can be overridden.

```rust
use fuchsia_syslog as syslog;

fn main() {
    syslog::init().unwrap();

    // suppress INFO and below
    syslog::set_severity(syslog::levels::WARN);
}
```
-->

### 配置日志等级

系统日志 crate 等级始于 `INFO` ，但是可以重载。

```rust
use fuchsia_syslog as syslog;

fn main() {
    syslog::init().unwrap();

    // 替换日志等级为 WARN
    syslog::set_severity(syslog::levels::WARN);
}
```


<!--
## Recording messages

Most uses of logging are with the `log` crate's macros:

```rust
trace!("something happened: {}", 5); // maps to TRACE
debug!("something happened: {}", 4); // maps to DEBUG
info!("something happened: {}", 3);  // maps to INFO
warn!("something happened: {}", 2);  // maps to WARN
error!("something happened: {}", 1); // maps to ERROR
```

The `fuchsia_syslog` crate also offers macros like `fx_log_info!`, which allow manually specifying
the tag of a message.
-->

## 记录消息

大多数日志使用的 `log` crate 的宏：

```rust
trace!("something happened: {}", 5); // 映射到 TRACE
debug!("something happened: {}", 4); // 映射到 DEBUG
info!("something happened: {}", 3);  // 映射到 INFO
warn!("something happened: {}", 2);  // 映射到 WARN
error!("something happened: {}", 1); // 映射到 ERROR
```

`fuchsia_syslog` crate 也提供了类似 `fx_log_info!` 的宏，允许手动指定消息的标签。

<!--
## Standard streams

`println!`, `eprintln!` etc. go to standard out (`stdout`) and standard error (`stderr`).

See [`stdout` & `stderr`] in the language-agnostic logging docs for details on the routing of stdio
streams in the system.
-->

## 标准流

`println!`, `eprintln!` 等，会去到标准输出 (`stdout`) 和标准错误 (`stderr`)。
在语言无关日志文档中查看 [`stdout` & `stderr`] 的细节，可以看到标准输入输出流在系统中的路由。 

[log crate]: https://fuchsia-docs.firebaseapp.com/rust/log/
[`fuchsia_syslog`]: https://fuchsia-docs.firebaseapp.com/rust/fuchsia_syslog/
[initialized in main]: /docs/development/languages/rust/add-logging.md
[rust-dev]: /docs/development/languages/rust/README.md
[`.cmx` file]: /docs/concepts/components/v1/component_manifests.md
[`stdout` & `stderr`]: /docs/development/diagnostics/logs/recording.md#stdout-stderr
