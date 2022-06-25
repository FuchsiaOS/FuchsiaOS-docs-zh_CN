# LLCPP FIDL library

## Prerequisites

This tutorial builds on the [Compiling FIDL][fidl-intro] tutorial.
For more information on other FIDL tutorials, see the [Overview][overview].

## Overview

This tutorial details how to use the LLCPP FIDL bindings
by creating a unit test that you can use as a "playground" for
exploring the LLCPP bindings.

This document covers how to complete the following tasks:

* [Write a C++ FIDL LLCPP test](#write-a-cpp-test).
* [Add the LLCPP bindings of a FIDL library as a build
  dependency](#add-dependency).
* [Import the LLCPP bindings into your code](#include-llcpp-bindings).
* [Inspect and use the generated bindings
  code](#inspect-user-generated-bindings).

The example code is located in your Fuchsia checkout in
`//examples/fidl/llcpp/unittests/`. If you want to write all the code
as you follow this tutorial, you can remove the example code:

```
rm -r examples/fidl/llcpp/unittests/*
```

## Write a C++ FIDL LLCPP test {#write-a-cpp-test}

1. Add a gtest stub to `examples/fidl/llcpp/unittests/main.cc`:

   ```c++
   #include <gtest/gtest.h>

   namespace {

   } // namespace
   ```

1. Define a `test` target, and a package containing the test in
   `examples/fidl/llcpp/unittests/BUILD.gn`:

   ```gn
   {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/unittests/BUILD.gn" region_tag="imports" %}

   test("test") {
     testonly = true
     output_name = "fidl_example_llcpp_test"
     sources = [ "main.cc" ]
     deps = [ "//src/lib/fxl/test:gtest_main" ]
   }

   {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/unittests/BUILD.gn" region_tag="package" %}
   ```

1. Build and run the empty test suite on a running instance of Fuchsia:

   ```
   fx set core.x64 --with //examples/fidl/llcpp/unittests:fidl-example-llcpp-test
   fx build
   ```

   In a separate terminal, run

   ```
   fx qemu -kN
   ```

   In a separate terminal, run

   ```
   fx serve
   ```

   Back in the original terminal (with `fx build`), run

   ```
   fx test -vo fidl-example-llcpp-test
   ```

   You should see test output indicating that zero tests have run,
   since no tests have been added yet.

## Add the LLCPP FIDL bindings as a dependency {#add-dependency}

For each FIDL library declaration, including the one in [Compiling FIDL][fidl-intro],
C++ library containing LLCPP bindings code for that library is generated under the original target
name appended with `_llcpp`.

Add a dependency on the LLCPP bindings by referencing this target. The new `test`
target should look like:

```gn
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/unittests/BUILD.gn" region_tag="test" %}
```

(Optional) To view the newly generated bindings:

1. Rebuild using `fx build`.
2. Change to the generated files directory:
   `out/default/fidling/gen/examples/fidl/fuchsia.examples/fuchsia/examples`, where the
   generated files are located.
   You may need to change `out/default` if you have set a different build output
   directory. You can check your build output directory with `cat .fx-build-dir`.

For more information on how to find generated bindings code, see
[Viewing generated bindings code][generated-code].

## Include the LLCPP bindings in your C++ code {#include-llcpp-bindings}

To include the bindings, add the following include statement to the top of
`examples/fidl/llcpp/unittests/main.cc`

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/unittests/main.cc" region_tag="include" %}
```

## Inspect and use the generated bindings code {#inspect-user-generated-bindings}

You can now write some tests by referring to the generated code. For more
information on the bindings, see [LLCPP Bindings Reference][bindings-ref].

To get started, you can also use some example code. You can add this inside the
anonymous namespace in `main.cc`:

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/unittests/main.cc" region_tag="bits" %}

{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/unittests/main.cc" region_tag="enums" %}

{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/unittests/main.cc" region_tag="structs" %}

{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/unittests/main.cc" region_tag="unions" %}

{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/unittests/main.cc" region_tag="tables" %}
```

To rebuild and rerun the tests, run:

```
fx test -vo fidl-example-llcpp-test
```

<!-- xrefs -->
[generated-code]: development/languages/fidl/guides/generated-code.md#rust
[bindings-ref]: reference/fidl/bindings/llcpp-bindings.md
[fidl-intro]: development/languages/fidl/tutorials/fidl.md
[overview]: development/languages/fidl/tutorials/overview.md
[server-tut]: development/languages/fidl/tutorials/llcpp/basics/server.md
