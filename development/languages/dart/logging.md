<!-- # Logging in Dart -->

# Dart 中的日志

<!-- Dart programs on Fuchsia generally write log messages with the `lib.logging` package, consuming and
initializing it through the `fuchsia_logger` package. -->

在 Fuchsia 上的 Dart 程序通常使用 `lib.logging` 包来写入日志消息，通过 `fuchsia_logger` 包来获取和初始化。

<!-- See the [language agnostic logging docs] for more information
about recording and viewing logs. -->

请参阅[语言无关日志文档]来获取更多关于记录和查看日志的信息。

<!-- ## Requirements -->

## 需求

<!-- ### GN dependency -->

### GN 依赖

<!-- The necessary packages can be included with an addtion to `deps` in `BUILD.gn`: -->

必要的包可以包含在`BUILD.gn`中的`deps`中：

```
deps = [
  "//topaz/public/dart/fuchsia_logger",
]
```

<!-- The `fuchsia_logger` package also provides Dart's `lib.logging`.

See [Dart: Overview][dart-dev] for more information about building Dart within Fuchsia. -->

`fuchsia_logger` 包也提供了 Dart 的 `lib.logging`。

请参阅 [Dart: 概述][dart-dev]以了解关于在 Fuchsia 中构建 Dart 的更多信息。

<!-- ### Component manifest dependency -->

### 组件清单依赖

<!-- Ensure that your component has the required capabilities to log by including the
following in your component manifest: -->

请在组件清单中包含以下内容，以确保您的组件具有记录日志所需的功能：

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

<!-- Note: The above is only available for in-tree development.
This is tracked in [fxbug.dev/64207](http://fxbug.dev/64207).
Out of tree developers should copy the snippets shown below instead. -->

Note: 以上仅适用于树内开发。
这在 [fxbug.dev/64207](http://fxbug.dev/64207) 中进行了跟踪。
树外开发人员应该复制下面展示的片段。

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

<!-- The syslog library will fallback to `stderr` if the `LogSink` connection fails. -->

如果 `LogSink` 连接失败，系统日志库将回退到 `stderr`。

<!-- ## Initialization -->

## 初始化

<!-- In your main function, call the `setupLogger()` function to initialize logging: -->

在您的主函数中，调用 `setupLogger()` 函数来初始化日志。

<!-- ```dart
import 'package:fuchsia_logger/logger.dart';

main() {
  // process name will be used if no name is provided here
  setupLogger(name: 'my-component');
}
``` -->

```dart
import 'package:fuchsia_logger/logger.dart';

main() {
  // 如果此处未提供名称，将使用进程名称
  setupLogger(name: 'my-component');
}
```

<!-- ### Configure severity -->

### 配置等级

<!-- By default only messages with `INFO` severity or higher are printed. Severity level can be adjusted
by providing the `level` parameter in the `setupLogger()` call. -->

默认情况下，仅打印具有“INFO”严重性或更高级别的消息。
可以通过在 `setupLogger()` 调用中提供 `level` 参数来调整严重性级别。

<!-- For example, to make all log messages appear in [`fx log`]: -->

例如：使所有日志消息出现在 [`fx log`] 中：

```dart
setupLogger(name: 'noisy-component', level: Level.ALL);
```

<!-- ## Recording messages -->

## 记录消息

<!-- The `log` object is a [Logger] instance. -->

`log` 对象是一个 [Logger] 的实例。

<!-- ```dart
import 'package:fuchsia_logger/logger.dart';

log.finest('quietest');      // maps to TRACE
log.finer('also quietest');  // maps to TRACE also
log.fine('quiet');           // maps to DEBUG
log.info('hello world!');    // maps to INFO
log.warning('uhhh');         // maps to WARN
log.severe('oh no!');        // maps to ERROR
log.shout('this is fatal.'); // maps to FATAL
``` -->

```dart
import 'package:fuchsia_logger/logger.dart';

log.finest('quietest');      // 映射到 TRACE
log.finer('also quietest');  // 也映射到 TRACE
log.fine('quiet');           // 映射到 DEBUG
log.info('hello world!');    // 映射到 INFO
log.warning('uhhh');         // 映射到 WARN
log.severe('oh no!');        // 映射到 ERROR
log.shout('this is fatal.'); // 映射到 FATAL（译：致命的错误）
```

<!-- ## Standard streams -->

## 标准流

<!-- `print` goes to standard out (`stdout`). -->

`print` 进入标准输出 (`stdout`)。

<!-- See [`stdout` & `stderr`] in the language-agnostic logging docs for details on the routing of stdio
streams in the system. -->

有关系统中输入输出流路由的详细信息，请参阅与语言无关的日志记录文档中的 [`stdout` 和 `stderr`]。

[Logger]: https://pub.dev/documentation/logging/latest/logging/Logger-class.html
[`fx log`]: /docs/development/diagnostics/logs/viewing.md
[dart-dev]: /docs/development/languages/dart/README.md
[`.cmx` file]: /docs/concepts/components/v1/component_manifests.md
[`stdout` & `stderr`]: /docs/development/diagnostics/logs/recording.md#stdout-stderr
[语言无关日志文档]: /docs/concepts/diagnostics/logs/README.md
