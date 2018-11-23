# Logging


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

In the main function of your Dart / Flutter app, call the `setupLogger()`
function to make sure logs appear in the Fuchsia console in the desired format.
```dart
import 'package:lib.logging/logging.dart';

main() {
  setupLogger();
}
```

After setting this up, you can call one of the following log methods to add log
statements to your code:
```dart
import 'package:lib.logging/logging.dart';

// add logging statements somewhere in your code as follows:
log.info('hello world!');
```

The `log` object is a `Logger` instance as documented [here][logger-doc].


## Log Levels

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

By default, all the logs of which level is INFO or higher will be shown in the
console. Because of this, Dart / Flutter app developers are highly encouraged to
use `log.fine()` for their typical logging statements for development purposes.

Currently, the log level should be adjusted in individual Dart apps by providing
the `level` parameter in the `setupLogger()` call. For example:
```dart
setupLogger(level: Level.ALL);
```
will make all log statements appear in the console.


[logger-doc]: https://www.dartdocs.org/documentation/logging/0.11.3%2B1/logging/Logger-class.html
