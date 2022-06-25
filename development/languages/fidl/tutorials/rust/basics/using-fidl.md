# FIDL Rust crates

## Prerequisites

In this tutorial, you will be using the `fuchsia.examples` FIDL library from the
[Creating a FIDL library][fidl-intro] tutorial. The code for this FIDL library
is available in [//examples/fidl/fuchsia.examples][example-lib]. Take a minute to
review the code before moving on.

## Overview

This tutorial details how to use FIDL from Rust
by creating a unit test that you can use  as a "playground" for
exploring the Rust bindings.

This document covers how to complete the following tasks:

* [Write a "hello world" Rust program](#hello-world).
* [Add the Rust bindings of a FIDL library as a build
  dependency](#add-dependency).
* [Import the Rust bindings crate into your code](#include-rust-bindings).
* [Inspect and use the generated bindings
  code](#inspect-user-generated-bindings).

The example code is located in your Fuchsia checkout in
`//examples/fidl/rust/fidl_crates/`. If you want to write all the code
as you follow this tutorial, you can remove the example code:

```
rm -r examples/fidl/rust/fidl_crates/*
```

## Write a "hello world" program {#hello-world}

1. Add the main function to `examples/fidl/rust/fidl_crates/src/main.rs`:

   ```rust
   {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/rust/fidl_crates/src/main.rs" region_tag="main" adjust_indentation="auto" %}
   ```

1. Define a `rustc_binary` and then create a depencency on the test through the `$host_toolchain`, which will build the binary for the host.
   To do this, add the following to `examples/fidl/rust/fidl_crates/BUILD.gn`:

   ```gn
   {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/rust/fidl_crates/BUILD.gn" region_tag="imports" %}

   rustc_binary("fidl_crates_bin") {
     edition = "2018"
     sources = [ "src/main.rs" ]
   }

   group("fidl_crates") {
      testonly = true
      deps = [ ":fidl_crates_bin($host_toolchain)" ]
   }
   ```

   Note: `rustc_binary` will look for a `src/main.rs` file by default as the crate root. It is possible
   to place the test code in a different file (e.g. `hello_world.rs`) instead, and then specify the
   crate root explicity in the `rustc_binary` declaration (e.g. `source_root = "hello_world.rs"`).

1. Include example in the build

   ```
   fx set core.x64 --with //examples/fidl/rust/fidl_crates
   ```

1. Build the example

   ```
   fx build
   ```

1. Run the binary

   ```
   out/default/host_x64/fidl_crates_bin
   ```

   You should see the hello world message printed.

   Note: the directory inside `out/default` will depend on your machine and
   configuration. For example, if you're running on an ARM machine with ASan,
   the directory will be `out/default/host_arm64-asan` instead.

## Add the Rust FIDL bindings as a dependency {#add-dependency}

For each FIDL library declaration, including the one in [Compiling FIDL][fidl-intro],
a FIDL crate containing Rust bindings code for that library is generated under the original target
name appended with `-rustc`.

Add a dependency on the Rust bindings by referencing this generated crate. The new `rustc_binary`
target should look like:

```gn
rustc_binary("fidl_crates_bin") {
  edition = "2018"
  deps = [ "//examples/fidl/fuchsia.examples:fuchsia.examples-rustc" ]

  sources = [ "src/main.rs" ]
}
```

(Optional) To view the newly generated bindings:

1. Rebuild using `fx build`.
2. Change to the generated files directory:
   `out/default/fidling/gen/examples/fidl/fuchsia.examples/fuchsia.examples`. The generated code is in
   `fidl_fuchsia_examples.rs`.
   You may need to change `out/default` if you have set a different build output
   directory. You can check your build output directory with `cat .fx-build-dir`.

Note: The generated FIDL bindings are part of the build output and are not checked in.

For more information on how to find generated bindings code, see
[Viewing generated bindings code][generated-code].

## Using the FIDL Rust crate in your project {#include-rust-bindings}

Create a place to play around with the generated FIDL crate by adding a test
module and placeholder test:

```rust
#[cfg(test)]
mod test {
   #[test]
   fn fidl_crates_usage() {

   }
}
```

You then need to build with tests by setting the `with_unit_tests` argument:

```gn
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/rust/fidl_crates/BUILD.gn" region_tag="test" %}
```

This will generate a `fidl_crates_bin_test` target, which should then be added
to the build group:

```
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/rust/fidl_crates/BUILD.gn" region_tag="group" %}
```

To import the crate, add the following to the top of the `tests` module.
In the Fuchsia tree, FIDL crates are often aliased to shorter names for brevity:

```rust
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/rust/fidl_crates/src/main.rs" region_tag="import" %}
```

## Use the generated bindings code {#inspect-user-generated-bindings}

You can now write some code using the generated bindings code. For more
information on the bindings, see [Rust Bindings Reference][bindings-ref].

To get started, you can also use the example code below. You can add this inside the
`fidl_crates_usage` test:

```rust
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/rust/fidl_crates/src/main.rs" region_tag="bits" adjust_indentation="auto" %}

{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/rust/fidl_crates/src/main.rs" region_tag="enums_init" adjust_indentation="auto" %}

{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/rust/fidl_crates/src/main.rs" region_tag="structs" adjust_indentation="auto" %}

{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/rust/fidl_crates/src/main.rs" region_tag="unions_init" adjust_indentation="auto" %}

{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/rust/fidl_crates/src/main.rs" region_tag="tables_init" adjust_indentation="auto" %}
```

To rebuild and rerun the tests, run:

```
fx test -vo fidl_crates_bin_test
```

<!-- xrefs -->
[generated-code]: /docs/development/languages/fidl/guides/generated-code.md#rust
[bindings-ref]: /docs/reference/fidl/bindings/rust-bindings.md
[fidl-intro]: /docs/development/languages/fidl/tutorials/fidl.md
[overview]: /docs/development/languages/fidl/tutorials/overview.md
[example-lib]: /examples/fidl/fuchsia.examples/echo.test.fidl
