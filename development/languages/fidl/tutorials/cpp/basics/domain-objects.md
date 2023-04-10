# Using natural and wire domain objects

## Prerequisites

This tutorial builds on the [Compiling FIDL][fidl-intro] tutorial.
For more information on other FIDL tutorials, see the [overview][overview].

## Overview

This tutorial details how to use the natural and wire
[domain objects][glossary.domain-object] by creating a unit test exercising
those data types.

This document covers how to complete the following tasks:

* [Add the C++ bindings of a FIDL library as a build dependency](#add-dep).
* [Include the bindings header into your code](#include-cpp-bindings).
* [Using natural domain objects](#using-natural).
* [Using wire domain objects](#using-wire).
* [Convert between natural and wire domain objects](#convert-natural-wire).

## Using the domain objects example code

The example code accompanying this tutorial is located in your Fuchsia checkout
at `//examples/fidl/cpp/domain_objects`. It consists of a unit test component
and its containing package. For more information about building unit test
components, see [Build components][build-components].

You may build and run the example on a running instance of Fuchsia emulator via
the following:

```posix-terminal
# Add the domain objects unit test to the build.
# This only needs to be done once.
fx set core.x64 --with //examples/fidl/cpp/domain_objects

# Run the domain objects unit test.
fx test -vo fidl-examples-domain-objects-cpp-test
```

## Add the C++ bindings of a FIDL library as a build dependency {#add-dep}

* {GN build}

  For each FIDL library declaration, such as the one in
  [Compiling FIDL][fidl-intro], the C++ bindings code for that library is
  generated under the original target name suffixed with `_cpp`:

  ```gn
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/domain_objects/BUILD.gn" region_tag="binding-dep" adjust_indentation="auto" exclude_regexp="^$" %}
  ```

  The `test` target looks like:

  ```gn
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/domain_objects/BUILD.gn" region_tag="test" adjust_indentation="auto" highlight="9" %}
  ```

  Note the line which adds the dependency on the C++ bindings by referencing that
  `_cpp` target.

  (Optional) To view the generated bindings:

  1. Build using `fx build`.
  2. Change to the generated files directory:
     `out/default/fidling/gen/examples/fidl/fuchsia.examples/fuchsia.examples/cpp/fidl/fuchsia.examples/cpp`,
     where the generated files are located. You may need to change `out/default`
     if you have set a different build output directory. You can check your build
     output directory with `cat .fx-build-dir`.

  For more information on how to find generated bindings code, see
  [Viewing generated bindings code][generated-code].

* {Bazel build}

  <!-- TODO(fxbug.dev/111377): Link to real samples once those are ready.-->
  <!-- TODO(fxbug.dev/98989): `llcpp` should be renamed to `cpp` -->

  When depending on the FIDL library from the Bazel build, an extra build rule
  is required if the FIDL library is not from the SDK:

  ```bazel
  # Given a FIDL library declaration like the following
  fuchsia_fidl_library(
      name = "fuchsia.examples",
      srcs = [
          "echo.test.fidl",
          "types.test.fidl",
      ],
      library = "fuchsia.examples",
      visibility = ["//visibility:public"],
  )

  # This rule describes the generated C++ bindings code for that library
  fuchsia_fidl_llcpp_library(
      name = "fuchsia.examples_llcpp_cc",
      library = ":fuchsia.examples",
      visibility = ["//visibility:public"],
      deps = ["@fuchsia_sdk//pkg/fidl_cpp_v2"],
  )
  ```

  If the FIDL library is from the Bazel SDK, the above step is not needed.

  The C++ bindings code for a FIDL library is generated under the original
  target name suffixed with `_llcpp_cc`:

  ```bazel
  deps = [
    # Example when depending on an SDK library, `fuchsia.io`.
    "@fuchsia_sdk//fidl/fuchsia.io:fuchsia.io_llcpp_cc",

    # Example when depending on a local FIDL library, `fuchsia.examples`
    # defined above.
    # Suppose the library lives in the `//path/to/fidl/library` folder.
    "//path/to/fidl/library:fuchsia.examples_llcpp_cc",

    # ... other dependencies ...
  ]
  ```

## Include the bindings header into your code {#include-cpp-bindings}

After adding the build dependency, you may include the bindings header. The
include pattern is `#include <fidl/my.library.name/cpp/fidl.h>`.

The following include statement at the top of `domain_objects/main.cc` includes
the bindings and makes the generated APIs available to the source code:

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/domain_objects/main.cc" region_tag="include" %}
```

## Using natural domain objects {#using-natural}

Natural types are the ergonomics and safety focused flavor of C++ domain
objects. A tree of FIDL values is represented as a tree of C++ objects with
hierarchical ownership. That means if a function receives some object of natural
type, it can assume unique ownership of all child objects in the entire tree.
The tree is torn down when the root object goes out of scope.

At a high level the natural types embrace `std::` containers and concepts. For
example, a [table][table] is represented as a collection of
`std::optional<Field>`s. A [vector][vector] is `std::vector<T>`, etc. They also
implement idiomatic C++ moves, copies, and equality. For example, a
[resource][resource] type is move-only, while a value type will implement both
copy and moves, where moves are designed to optimize the transfer of objects.
Moving a table doesn't make it empty (it just recursively moves the fields),
similar to `std::optional`.

### Natural bits

Using the strict [`fuchsia.examples/FileMode`][fidl-file] FIDL type and the
flexible [`fuchsia.examples/FlexibleFileMode`][fidl-file] FIDL type as examples:

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/domain_objects/main.cc" region_tag="natural-bits" adjust_indentation="auto" exclude_regexp="^TEST|^}" %}
```

### Natural enums

Using the strict [`fuchsia.examples/LocationType`][fidl-file] FIDL type and the
flexible [`fuchsia.examples/FlexibleLocationType`][fidl-file] FIDL type as
examples:

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/domain_objects/main.cc" region_tag="natural-enums" adjust_indentation="auto" exclude_regexp="^TEST|^}" %}
```

### Natural structs

Natural structs are straightforward record objects that expose const and mutable
accessors. Using the [`fuchsia.examples/Color`][fidl-file] FIDL type as an
example:

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/domain_objects/main.cc" region_tag="natural-structs" adjust_indentation="auto" exclude_regexp="^TEST|^}" %}
```

### Natural unions

Natural unions are sum types similar to `std::variant`. Using the strict
[`fuchsia.examples/JsonValue`][fidl-file] FIDL type and the flexible
[`fuchsia.examples/FlexibleJsonValue`][fidl-file] FIDL type as examples:

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/domain_objects/main.cc" region_tag="natural-unions" adjust_indentation="auto" exclude_regexp="^TEST|^}" %}
```

### Natural tables

Natural tables are record types where every field is optional. Using the
[`fuchsia.examples/User`][fidl-file] FIDL type as an example:

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/domain_objects/main.cc" region_tag="natural-tables" adjust_indentation="auto" exclude_regexp="^TEST|^}" %}
```

## Using wire domain objects {#using-wire}

Wire types are the performance oriented flavor of C++ domain objects. Differing
from natural types which maintain hierarchical object ownership, wire objects
never own their out-of-line children. Whether a child object is stored inline or
out-of-line is determined by the [FIDL wire format][fidl-wire-format].

Natural types may implicitly heap allocate the necessary storage. Conversely,
the user has complete control over memory allocation of wire types. For example,
you may allocate the elements of a FIDL vector on the stack, from a memory pool,
or as part of a larger object. The wire vector type, `fidl::VectorView<T>`, is
an unowned view type consisting of a raw pointer and a length. One may send the
vector as part of a FIDL request without extra heap allocations by borrowing the
elements via this type.

To distinguish from the natural types, wire types from a FIDL library are
defined in the `...::wire` nested namespace, e.g. `fuchsia_my_library::wire`.

The prevalence of unowned pointers in wire types makes them flexible but very
unsafe. This tutorial will focus on the safer side of using wire types based on
memory arenas. For more advanced usages involving unsafe memory borrows, refer
to [Memory ownership of wire domain objects][wire-memory-ownership].

### Wire bits and enums

Because bits and enums have a very simple memory layout and do not have any
out-of-line children, the wire types for FIDL bits and enums are the same as
their natural type counterparts. To stay coherent with the overall namespace
naming profiles, bits and enums are aliased into the `fuchsia_my_library::wire`
nested namespace, appearing alongside wire structs, unions, and tables.

Using the [`fuchsia.examples/FileMode`][fidl-file] FIDL bits as an example,
`fuchsia_examples::wire::FileMode` is a type alias of
`fuchsia_examples::FileMode`.

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/domain_objects/main.cc" region_tag="wire-bits" adjust_indentation="auto" exclude_regexp="^TEST|^}" %}
```

Similarly, using the [`fuchsia.examples/LocationType`][fidl-file] FIDL enum as
an example, `fuchsia_examples::wire::LocationType` is a type alias of
`fuchsia_examples::LocationType`.

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/domain_objects/main.cc" region_tag="wire-enums" adjust_indentation="auto" exclude_regexp="^TEST|^}" %}
```

### Wire structs

Wire structs are simple C++ structs that hold public member variables. Using the
[`fuchsia.examples/Color`][fidl-file] FIDL type as an example:

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/domain_objects/main.cc" region_tag="wire-structs" adjust_indentation="auto" exclude_regexp="^TEST|^}" %}
```

### Wire unions

Wire unions are sum types with a memory layout akin to a discriminator tag
followed by a reference to the active member. Using the strict
[`fuchsia.examples/JsonValue`][fidl-file] FIDL type and the flexible
[`fuchsia.examples/FlexibleJsonValue`][fidl-file] FIDL type as examples:

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/domain_objects/main.cc" region_tag="wire-unions" adjust_indentation="auto" exclude_regexp="^TEST|^}" %}
```

### Wire tables

Wire tables are record types where every field is optional. Differing from
[natural tables](#natural-tables), wire tables do not own any member field.
Copying a wire table is akin to aliasing (copying) a pointer. Similar to
pointers, moving a wire table is an anti-pattern because that equates to a copy.

Because of the memory layout constraints of wire tables, one always use an
associated `Builder` type to create new instances. Once a table is built, one
may not add new members or clear existing members.

Using the [`fuchsia.examples/User`][fidl-file] FIDL type as an example:

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/domain_objects/main.cc" region_tag="wire-tables" adjust_indentation="auto" exclude_regexp="^TEST|^}" %}
```

For more information on the bindings, see the
[bindings reference][bindings-ref].

## Convert between natural and wire domain objects {#convert-natural-wire}

To streamline interoperability, you may call `fidl::ToWire` and
`fidl::ToNatural` functions to convert between wire and natural domain objects.
Using the [`fuchsia.examples/User`][fidl-file] FIDL type as an example:

### Convert from natural to wire: `fidl::ToWire`

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/domain_objects/main.cc" region_tag="natural-to-wire" adjust_indentation="auto" exclude_regexp="^TEST|^}" %}
```

### Convert from wire to natural: `fidl::ToNatural`

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/domain_objects/main.cc" region_tag="wire-to-natural" adjust_indentation="auto" exclude_regexp="^TEST|^}" %}
```

## Persist natural and wire domain objects

You may use `fidl::Persist` to serialize a natural or wire domain object into
a byte vector, the primary use case being long term data persistence.

`fidl::Unpersist` deserializes and copies a sequence of bytes into some instance
of natural domain object.

`fidl::InplaceUnpersist` deserializes a sequence of bytes into some instance of
wire domain object, mutating the bytes in the process.

<<../../../widgets/_persistence.md>>

<!-- xrefs -->
[build-components]: /docs/development/components/build.md#unit-tests
[generated-code]: /docs/development/languages/fidl/guides/generated-code.md#c-family
[bindings-ref]: /docs/reference/fidl/bindings/cpp-bindings.md
[fidl-intro]: /docs/development/languages/fidl/tutorials/fidl.md
[fidl-file]: /examples/fidl/fuchsia.examples/types.test.fidl
[fidl-wire-format]: /docs/reference/fidl/language/wire-format/README.md
[glossary.domain-object]: /docs/glossary#domain-object
[overview]: /docs/development/languages/fidl/tutorials/overview.md
[resource]: /docs/reference/fidl/language/language.md#value-vs-resource
[server-tut]: /docs/development/languages/fidl/tutorials/cpp/basics/server.md
[table]: /docs/reference/fidl/language/language.md#tables
[vector]: /docs/reference/fidl/language/language.md#vectors
[wire-memory-ownership]: /docs/development/languages/fidl/tutorials/cpp/topics/wire-memory-ownership.md
