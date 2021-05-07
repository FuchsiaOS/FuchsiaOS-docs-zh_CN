# Compiling FIDL

## Prerequisites

This tutorial builds on the [Compiling FIDL][fidl-intro] tutorial.
For more information on other FIDL tutorials, see the [Overview][overview].

## Overview

This tutorial details how to include the HLCPP FIDL bindings into
your code by creating a unit test that you can use  as a "playground" for
exploring the HLCPP bindings.

This document covers how to complete the following tasks:

* [Write a C++ host test](#write-a-cpp-test).
* [Add the HLCPP bindings of a FIDL library as a build
  dependency](#add-dependency).
* [Include the HLCPP bindings into your C++ code](#include-hlcpp-bindings).
* [Inspect and use the generated bindings
  code](#inspect-user-generated-bindings).

The example code is located in your Fuchsia checkout in
`//examples/fidl/hlcpp/unittests/`. If you want to write all the code
as you follow this tutorial, you can remove the example code:

```
rm -r examples/fidl/hlcpp/unittests/*
```

## Write a C++ host test {#write-a-cpp-test}

1. Add a gtest stub to `examples/fidl/hlcpp/unittests/main.cc`:

   ```c++
   #include <gtest/gtest.h>

   namespace {

   } // namespace
   ```

1. Define a `test` and then create a dependency on the test through the `$host_toolchain`.
   To do this, add the following to `examples/fidl/hlcpp/unittests/BUILD.gn`:

   ```gn
   {%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/hlcpp/unittests/BUILD.gn" region_tag="first" %}

   test("example-cpp-host-test") {
     sources = [ "main.cc" ]
     deps = [ "//third_party/googletest:gtest_main" ]
   }
   ```

1. Run the empty test suite:

   ```
   fx set core.x64 --with //examples/fidl/hlcpp/unittests
   fx test -vo example-cpp-host-test
   ```

   You should see test output indicating that zero tests have run,
   since no tests have been added yet.

## Add the HLCPP FIDL bindings as a dependency {#add-dependency}

Add a dependency on the HLCPP bindings by referencing the FIDL target
directly. The new `test` target should look like:

```gn
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/hlcpp/unittests/BUILD.gn" region_tag="test" %}
```

(Optional) To view the newly generated bindings:

1. Rebuild using `fx build`.
2. Change to the generated files directory:
   `out/default/fidling/gen/examples/fidl/fuchsia.examples/fuchsia/examples`, where
   the generated files are located.
   You may need to change `out/default` if you have set a different build output
   directory. You can check your build output directory with `cat .fx-build-dir`.

For more information on how to find generated bindings code, see
[Viewing generated bindings code][generated-code].

## Include the HLCPP bindings in your C++ code {#include-hlcpp-bindings}

To include the bindings, add the following include statement to the top of
`examples/fidl/hlcpp/unittests/main.cc`

```cpp
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/hlcpp/unittests/main.cc" region_tag="include" %}
```

## Inspect and use the generated bindings code {#inspect-user-generated-bindings}

You can now write some tests by referring to the generated code. For more
information on the bindings, see [HLCPP Bindings Reference][bindings-ref].

To get started, you can also use some example code. You can add this inside the
anonymous namespace in `main.cc`:

```cpp
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/hlcpp/unittests/main.cc" region_tag="bits" %}

{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/hlcpp/unittests/main.cc" region_tag="enums" %}

{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/hlcpp/unittests/main.cc" region_tag="structs" %}

{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/hlcpp/unittests/main.cc" region_tag="unions" %}

{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/hlcpp/unittests/main.cc" region_tag="tables" %}
```

To rebuild and rerun the tests, run:

```
fx test -vo example-cpp-host-test
```

<!-- xrefs -->
[generated-code]: /docs/development/languages/fidl/guides/generated-code.md#c-family
[bindings-ref]: /docs/reference/fidl/bindings/hlcpp-bindings.md
[fidl-intro]: /docs/development/languages/fidl/tutorials/fidl.md
[overview]: /docs/development/languages/fidl/tutorials/overview.md
