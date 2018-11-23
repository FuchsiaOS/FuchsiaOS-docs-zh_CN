# Dart


## Overview

Dart artifacts are not built the same way in Fuchsia as they are on other
platforms.

Instead of relying on [`pub`][pub] to manage dependencies, sources of
third-party packages we depend on are checked into the tree under
[`//third_party/dart-pkg`][dart-3p].
This is to ensure we use consistent versions of our dependencies across multiple
builds.

Likewise, no build output is placed in the source tree as everything goes under
`out/`. That includes `.packages` files which are generated as part of the build
based on a target's dependency.


## Targets

There are five gn targets for building Dart:
- [`dart_library`][target-library] defines a library that can be used by other
Dart targets;
- [`dart_app`][target-app] defines a Dart executable for Fuchsia;
- [`dart_tool`][target-tool] defines a Dart tool for the host;
- [`flutter_app`][target-flutter] defines a [Flutter][flutter] application;
- [`dart_test`][target-test] defines a group of test.

See the definitions of each of these targets for how to use them.


## Package layout

We use a layout very similar to the [standard layout][package-layout].

```
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
[target-library]: https://fuchsia.googlesource.com/build/+/master/dart/dart_library.gni "dart_library target"
[target-app]: https://fuchsia.googlesource.com/topaz/+/master/runtime/dart_runner/dart_app.gni "dart_app target"
[target-tool]: https://fuchsia.googlesource.com/build/+/master/dart/dart_tool.gni "dart_tool target"
[target-flutter]: https://fuchsia.googlesource.com/topaz/+/master/runtime/flutter_runner/flutter_app.gni "flutter_app target"
[target-test]: https://fuchsia.googlesource.com/build/+/master/dart/dart_test.gni "dart_test target"
[flutter]: https://flutter.io/ "Flutter"
