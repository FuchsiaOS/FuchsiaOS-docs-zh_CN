<!-- # Logging -->
# 日志


<!--
It is highly recommended that you use `lib.logging` package when you want to add
logging statements to your Dart package.

Include the `lib.logging` package in your BUILD.gn target as a dependency:
```
deps = [
  ...
  "//topaz/public/dart/logging:lib.logging",
  ...
]
```
-->
当需要添加日志语句到 Dart 包中时，强烈推荐使用 `lib.logging` 包。

将包 `lib.logging` 作为依赖在 BUILD.gn 对象中导入：
```
deps = [
  ...
  "//topaz/public/dart/logging:lib.logging",
  ...
]
```

<!--
In the main function of your Dart / Flutter app, call the `setupLogger()`
function to make sure logs appear in the Fuchsia console in the desired format.
```dart
import 'package:lib.logging/logging.dart';

main() {
  setupLogger();
}
```
-->
在 Dart / Flutter 应用的主函数中调用 `setupLogger()` 函数来确保日志在 Fuchsia 终端中以期望的格式出现。
```dart
import 'package:lib.logging/logging.dart';

main() {
  setupLogger();
}
```

<!--
After setting this up, you can call one of the following log methods to add log
statements to your code:
```dart
import 'package:lib.logging/logging.dart';

// add logging statements somewhere in your code as follows:
log.info('hello world!');
```

The `log` object is a `Logger` instance as documented [here][logger-doc].
-->
这样设置好后，可以调用一个下列的日志方法来给代码添加日志语句：
```dart
import 'package:lib.logging/logging.dart';

// add logging statements somewhere in your code as follows:
log.info('hello world!');
```

`log` 对象是一个 `Logger` 实例，详见 [这里][logger-doc]。

<!-- ## Log Levels -->
## 日志级别

<!--
The log methods are named after the supported log levels. To list the log
methods in descending order of severity:
```dart
log.shout()    // maps to LOG_FATAL in FXL.
log.severe()   // maps to LOG_ERROR in FXL.
log.warning()  // maps to LOG_WARNING in FXL.
log.info()     // maps to LOG_INFO in FXL.
log.fine()     // maps to VLOG(1) in FXL.
log.finer()    // maps to VLOG(2) in FXL.
log.finest()   // maps to VLOG(3) in FXL.
```
-->
每个日志方法都以对应的日志级别命名。日志方法按严重程度降序排列如下：
```dart
log.shout()    // maps to LOG_FATAL in FXL.
log.severe()   // maps to LOG_ERROR in FXL.
log.warning()  // maps to LOG_WARNING in FXL.
log.info()     // maps to LOG_INFO in FXL.
log.fine()     // maps to VLOG(1) in FXL.
log.finer()    // maps to VLOG(2) in FXL.
log.finest()   // maps to VLOG(3) in FXL.
```

<!--
By default, all the logs of which level is INFO or higher will be shown in the
console. Because of this, Dart / Flutter app developers are highly encouraged to
use `log.fine()` for their typical logging statements for development purposes.

Currently, the log level should be adjusted in individual Dart apps by providing
the `level` parameter in the `setupLogger()` call. For example:
```dart
setupLogger(level: Level.ALL);
```
will make all log statements appear in the console.
-->
所有级别等于或高于 INFO 的日志默认都会在终端显示。因此，我们强烈推荐 Dart / Flutter 应用开发者在开发时使用 `log.fine()` 设置通常的日志语句。

目前应通过在单个 Dart 应用的 `setupLogger()` 调用中提供 `level` 参数来调整日志级别。例如：
```dart
setupLogger(level: Level.ALL);
```
会使所有级别的日志语句在终端中显示。


[logger-doc]: https://www.dartdocs.org/documentation/logging/0.11.3%2B1/logging/Logger-class.html
