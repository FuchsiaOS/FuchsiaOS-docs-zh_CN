# Creating a FIDL library

## Prerequisites

This tutorial expects that you have completed the [getting
started][getting-started] guide and are able to build and run Fuchsia.  You
should be familiar with running components on Fuchsia, which is covered in [run
an example component][run-examples].

## Overview

In this tutorial, you will define and build the FIDL library
`examples.keyvaluestore.baseline`. After you're done, you'll know how to author
a FIDL file, set up necessary GN rules, and build FIDL bindings.

The full source for this tutorial can be found at
[//examples/fidl/new/key_value_store/baseline/fidl/][baseline]

Note: This library will be used later in the [key-value store example series].

## Define the FIDL library

First, create a directory for this tutorial by running the following command
from your fuchsia checkout:

```
mkdir -p vendor/fidl-tutorials/building-fidl
```

Create a new file, `key_value_store.test.fidl`:

```
touch vendor/fidl-tutorials/building-fidl/key_value_store.test.fidl
```

FIDL file names use the `.fidl` extension. Similar to C header files, FIDL files
define data types and declare functional interfaces. These declarations are used in
conjunction with FIDL-specific data types to communicate between FIDL endpoints.

Add the following FIDL code to `key_value_store.test.fidl`:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/key_value_store/baseline/fidl/key_value_store.test.fidl" %}
```

This FIDL defines an `Item` type to represent items in the store, a `WriteError`
enum to list known errors for writes, and a `Store` protocol with one method:
`WriteItem`. As defined, this library can only write values into a store, but in
the [key-value store example series], you'll augment this library to support
nested stores, reading values, and other features.

Note: The `test.fidl` extension is used instead of `.fidl` to avoid linting and
API review requirements. Use `.fidl` for production code.

## Create a GN target for the FIDL library

Now that you've defined your FIDL, you need to create a `gn` target that other
code can depend on.

Create a new file, `BUILD.gn`:

```
touch vendor/fidl-tutorials/building-fidl/BUILD.gn
```

and add the following build rules:

```gn
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/key_value_store/baseline/fidl/BUILD.gn" %}
```

The `gn` template [`fidl("examples.keyvaluestore.baseline")`][fidl-template]
creates the necessary targets to use this library from other code.

## Compile the FIDL library

To build your new FIDL library, run the following `fx set` command:

```
fx set core.x64\
  --with //vendor/fidl-tutorials/building-fidl:examples.keyvaluestore.baseline_rust\
  --with //vendor/fidl-tutorials/building-fidl:examples.keyvaluestore.baseline_cpp
```

This command configures `fx build` to generate the rust and cpp bindings.

Next, build the code:

```
fx build
```

## Explore the generated bindings

Now that the code is built, you can explore the generated code for your
bindings. To see the generated code, browse the following directories:

| binding type | directory                                                                                                                                   |
|--------------|---------------------------------------------------------------------------------------------------------------------------------------------|
| rust         | //out/default/fidling/gen/vendor/fidl-tutorials/building-fidl/examples.keyvaluestore.baseline/rust/                                         |
| new c++      | //out/default/fidling/gen/vendor/fidl-tutorials/building-fidl/examples.keyvaluestore.baseline/cpp/fidl/examples.keyvaluestore.baseline/cpp/ |

See [generated code] for more details.


## Next steps

Now that you've finished this tutorial, you're ready to explore the full
[key-value store example series].

<!-- xrefs -->
[fidl-template]: /build/fidl/fidl.gni
[getting-started]: /get-started/README.md
[run-examples]: /development/run/run-examples.md
[example series]: /development/languages/fidl/examples.md
[key-value store example series]: /development/languages/fidl/examples/key_value_store/README.md
[baseline]: https://cs.opensource.google/fuchsia/fuchsia/+/main:examples/fidl/new/key_value_store/baseline/fidl/
[generated code]: /development/languages/fidl/guides/generated-code.md
