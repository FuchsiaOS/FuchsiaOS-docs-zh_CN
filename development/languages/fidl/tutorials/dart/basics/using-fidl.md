# FIDL Dart packages

## Prerequisites

This tutorial builds on the [Compiling FIDL][fidl-intro] tutorial.
For more information on other FIDL tutorials, see the [Overview][overview].

## Overview

This tutorial details how to use FIDL from Dart
by creating a unit test that you can use  as a "playground" for
exploring the Dart bindings.

This document covers how to complete the following tasks:

* [Write a Dart host test](#write-a-dart-test), following the Fuchsia Dart
  [package layout][package-layout]
* [Add the Dart bindings of a FIDL library as a build
  dependency](#add-dependency).
* [Import the Dart bindings crate into your code](#include-dart-bindings).
* [Inspect and use the generated bindings
  code](#inspect-user-generated-bindings).

The example code is located in your Fuchsia checkout in
`//examples/fidl/dart/fidl_packages/`. If you want to write all the code
as you follow this tutorial, you can remove the example code:

```
rm -r examples/fidl/dart/fidl_packages/*
```

Relative paths in the rest of the tutorial will be relative to this directory.

## Write a Dart host test {#write-a-dart-test}

1. Add a dummy test to `test/types_test.dart`:

   ```dart
   import 'package:test/test.dart';

   void main() {
     test('dummy', () {
       expect(1 + 1, equals(2));
     });
   }
   ```

1. Define a `dart_test` and then create a depencency on the test through the `$host_toolchain`.
   To do this, add the following to `BUILD.gn`:

   ```gn
   {%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/dart/fidl_packages/BUILD.gn" region_tag="imports" %}

    dart_test("fidl-example-dart-test") {
      sources = [ "types_test.dart" ]
      deps = [ "//third_party/dart-pkg/pub/test" ]
    }

   {%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/dart/fidl_packages/BUILD.gn" region_tag="group" %}
   ```

   Note: `dart_test` will look for source files in the `test` dir by default, so `types_test.dart`
   is specified relative to that directory. A different directory can be used by specifying a
   value for `source_dir`.

1. Create an empty pubspec file at `pubspec.yaml`:

   ```yaml
   {%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/dart/fidl_packages/pubspec.yaml" %}
   ```

1. (Optional) Create an analysis configuration file at `analysis_options.yaml`. You can reuse the common Fuchsia analysis
   configuration:

   ```yaml
   {%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/dart/fidl_packages/analysis_options.yaml" %}
   ```

1. Run the empty test suite:

   ```
   fx set core.x64 --with //examples/fidl/dart/fidl_packages
   fx test -vo fidl-example-dart-test
   ```

   You should see test output for the dummy test that was added.

## Add the Dart FIDL bindings as a dependency {#add-dependency}

For each FIDL library declaration, including the one in [Compiling FIDL][fidl-intro],
a Dart package containing bindings code for that library is generated under the original target
name.

Add a dependency on the Dart bindings by referencing this generated crate. The new `dart_test`
target should look like:

```gn
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/dart/fidl_packages/BUILD.gn" region_tag="test" %}
```

(Optional) To view the newly generated bindings:

1. Rebuild using `fx build`.
2. Change to the generated files directory:
   `out/default/dartlang/gen/examples/fidl/fuchsia.examples/fuchsia.examples_package`.
   The code is generated into `lib/fidl_async.dart` and `lib/fidl_test.dart`.
   You may need to change `out/default` if you have set a different build output
   directory. You can check your build output directory with `cat .fx-build-dir`.

For more information on how to find generated bindings code, see
[Viewing generated bindings code][generated-code].

## Import the FIDL Dart package into your project {#include-dart-bindings}

To import the package, add the following import statement to the top of the
`types_test.dart` file:

```dart
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/dart/fidl_packages/test/types_test.dart" region_tag="import" adjust_indentation="auto" %}
```

In the Fuchsia tree, FIDL package imports are often aliased to `fidl_[library]` for readability.

## Inspect and use the generated bindings code {#inspect-user-generated-bindings}

You can now write some tests by referring to the generated code. For more
information on the bindings, see [Dart Bindings Reference][bindings-ref].

To get started, you can also use some example code. You can add the following tests inside `main()`:

```dart
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/dart/fidl_packages/test/types_test.dart" region_tag="bits" adjust_indentation="auto" %}

{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/dart/fidl_packages/test/types_test.dart" region_tag="enums" adjust_indentation="auto" %}

{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/dart/fidl_packages/test/types_test.dart" region_tag="structs" adjust_indentation="auto" %}

{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/dart/fidl_packages/test/types_test.dart" region_tag="unions" adjust_indentation="auto" %}

{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/dart/fidl_packages/test/types_test.dart" region_tag="tables" adjust_indentation="auto" %}
```

To rebuild and rerun the tests, run:

```
fx test -vo fidl-example-dart-test
```

<!-- xrefs -->
[package-layout]: /docs/development/languages/dart/README.md#layout
[generated-code]: /docs/development/languages/fidl/guides/generated-code.md#dart
[bindings-ref]: /docs/reference/fidl/bindings/dart-bindings.md
[fidl-intro]: /docs/development/languages/fidl/tutorials/fidl.md
[overview]: /docs/development/languages/fidl/tutorials/overview.md
