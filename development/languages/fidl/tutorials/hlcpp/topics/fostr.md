# HLCPP object pretty printing

## Prequisites

This tutorial builds on the [Compiling FIDL][compiling] tutorial. For the
full set of FIDL tutorials, refer to the [overview][overview].

## Overview

This tutorial walks through how to add debug printing to HLCPP types, using the
[`fostr`][fostr-dir] tool.

If you want to write the code yourself, delete the following directories:

```
rm -r examples/fidl/hlcpp/fostr farnet/public/lib/fostr/fidl
```

This tutorial contains the following steps:

* Define a `fostr_fidl` target for a FIDL library. This will allow debug printing for
  types in the library by defining an `operator<<` for each type.
* Use the `fostr_fidl` target.
* Write examples with the formatter code.

## Define fostr_fidl target

1. Create a directory for the BUILD file inside [`//garnet/public/lib/fostr/fidl`][fostr-dir]:

   ```
   mkdir -p garnet/public/lib/fostr/fidl/fuchsia.examples
   ```

2. Create the BUILD file:

   ```
   touch garnet/public/lib/fostr/fidl/fuchsia.examples/BUILD.gn
   ```

3. Add the following definition to the build file:

   ```gn
   {%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="garnet/public/lib/fostr/fidl/fuchsia.examples/BUILD.gn" %}
   ```

This defines a `fostr` target for the `fuchsia.examples` library. Any
dependencies to the library needs to be specified in the build file as well,
which doesn't apply to this tutorial as it has no dependencies. For more complex
examples, refer to the existing definitions in the fostr `fidl` directory.

## Add a dependency on the fostr_fidl target

The target library that the tutorial uses is a simple host test, which is
located in `examples/fidl/hlcpp/fostr`. Notice that the BUILD file for the test
includes the `fostr_fidl` target as a dependency:

```gn
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/hlcpp/fostr/BUILD.gn" region_tag="bin" highlight="7" %}
```

The included library automatically overloads the `<<` operator. The path
is based on the name of the FIDL library it is generated for:

```c++
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/hlcpp/fostr/main.cc" region_tag="includes" %}
```

## Write examples using the formatter code

Write some placeholder tests to show off the output:

```cpp
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/hlcpp/fostr/main.cc" region_tag="tests" %}
```

## Run the example

To run the example:

1. Configure the GN build to include the example:

   ```
   fx set core.x64  --with //examples/fidl/hlcpp/fostr
   ```

2. Run the test:

   ```
   fx test -vo fostr-example-test
   ```

<!-- xrefs -->
[compiling]: /docs/development/languages/fidl/tutorials/fidl.md
[fostr-dir]: /garnet/public/lib/fostr/fidl
[overview]: /docs/development/languages/fidl/tutorials/overview.md
