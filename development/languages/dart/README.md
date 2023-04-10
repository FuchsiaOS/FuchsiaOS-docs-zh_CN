# Dart

## Overview

Dart artifacts are not built the same way in Fuchsia as they are on other
platforms.

Instead of relying on [`pub`][pub] to manage dependencies, sources of
third-party packages we depend on are checked into the tree under
`//third_party/dart-pkg`.
This is to ensure we use consistent versions of our dependencies across multiple
builds.

Likewise, no build output is placed in the source tree as everything goes under
`out/`. That includes `package_config.json` files, which are generated as part of the build
based on a target's dependency.

## Exiting Dart programs

The Dart runner for Fuchsia does not
monitor the FIDL channels opened by Dart programs and as a result does not end
the program normally, but rather waits for the explicit call to `fuchsia.exit()`
to indicate the program should be ended.

Note: Calling exit() from dart:io will result in an exception since components
are not allowed to call this method since it would shutdown the dart_runner process.

```dart
import 'package:fuchsia/fuchsia.dart' as fuchsia;

void main(List<String> args) {
  print('Hello Dart!');
  fuchsia.exit(23);
}
```


## Targets

There are five gn targets for building Dart:

- [`dart_library`][target-library] defines a library that can be used by other
Dart targets;
- [`dart_component`][target-dart-component] defines a Fuchsia component based on a Dart
  library, that can be packaged using GN targets `fuchsia_package` or
  `fuchsia_test_package`;
- [`dart_tool`][target-tool] defines a Dart tool for the host;
- [`flutter_component`][target-flutter] defines a [Flutter][flutter] component;
- [`dart_test`][target-test] defines a group of test.

See the definitions of each of these targets for how to use them.


## Package layout {#layout}

We use a layout very similar to the [standard layout][package-layout].

```none
my_package/
  |
  |-- pubspec.yaml           # Empty, used as a marker [mandatory]
  |-- BUILD.gn               # Contains all targets
  |-- analysis_options.yaml  # Analysis configuration [mandatory]
  |-- lib/                   # dart_library contents
  |-- bin/                   # dart_binary's (target) or dart_tool's (host)
  |-- test/                  # dart_test contents
```

## Going further

- [Running analysis](analysis.md)
- [Style](style.md)
- [Testing](testing.md)
- [Logging](logging.md)
- [Using FIDL](fidl.md)
- [Managing third_party dependencies](third_party.md)
- [IDEs](ides.md)

[pub]: https://www.dartlang.org/tools/pub/get-started "Pub"
[package-layout]: https://www.dartlang.org/tools/pub/package-layout "Package layout"
[target-library]: /build/dart/dart_library.gni "dart_library target"
[target-dart-component]: /build/dart/dart_component.gni "dart_component target"
[target-tool]: /build/dart/dart_tool.gni "dart_tool target"
[target-flutter]: https://fuchsia.googlesource.com/fuchsia/+/HEAD/build/flutter/flutter_component.gni "flutter_component target"
[target-test]: /build/dart/dart.gni "dart_test target"
[flutter]: https://flutter.io/ "Flutter"
