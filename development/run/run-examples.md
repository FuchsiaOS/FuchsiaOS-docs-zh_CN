# Run an example component

This guide shows you how to build Fuchsia to include an example package from
Fuchsia's source [`//examples`](/examples/) directory and run a component on
your Fuchsia target.

Note: You can find the source code for the "Hello, World" example at
[`//examples/hello_world`](/examples/hello_world).

## Prerequisites

Before you can run an example component, you must:

*   [Set up the Fuchsia development environment](/get-started/get_fuchsia_source.md)

## Exploring the example {#exploring-the-example}

This example component prints `Hello, world!` to the system log. The example has
three main elements:

*   An [executable binary](#executable-program) written in a supported language.
*   A [component manifest](#component-manifest) (`.cml`) file to declare the
    component and its capabilities.
*   A [`BUILD.gn`](#build-gn) file to define the component build target and
    include it in a Fuchsia package.

### Executable program {#executable-program}

Fuchsia components can execute programs written in any language with a supported
runtime. The most common runtime used for C++ and Rust programs is the
[ELF runner](/concepts/components/v2/elf_runner.md).

* {C++}

  ```cpp
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/hello_world/cpp/hello_world.cc" region_tag="main" adjust_indentation="auto" %}
  ```

* {Rust}

  ```rust
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/hello_world/rust/src/main.rs" region_tag="main" adjust_indentation="auto" %}
  ```

### Component manifest {#component-manifest}

A [component manifest](/glossary/README.md#component-manifest) describes
how to run a Fuchsia program as a [component](/glossary/README.md#component).
This includes declaring program binary, runtime information, and any capabilities
the component requires to execute, such as logging support.

* {C++}

  ```json5
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/hello_world/cpp/meta/hello_world_cpp.cml" adjust_indentation="auto" %}
  ```

* {Rust}

  ```json5
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/hello_world/rust/meta/hello_world_rust.cml" adjust_indentation="auto" %}
  ```

For more details on component manifests and their declaration syntax,
see [component manifests](/concepts/components/v2/component_manifests.md).

### BUILD.gn {#build-gn}

Fuchsia uses the Generate Ninja (GN) meta-build system to generate inputs for
[Ninja](https://ninja-build.org/){:.external}, which executes the actual build.
The `BUILD.gn` file declares the build targets for a `fuchsia_component()` and
`fuchsia_package()`.

Note: If you aren't familiar with GN, see
[Introduction to GN](/development/build/build_system/intro.md).

* {C++}

  ```gn
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/hello_world/cpp/BUILD.gn" region_tag="cpp_bin" adjust_indentation="auto" %}
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/hello_world/cpp/BUILD.gn" region_tag="fuchsia_component" adjust_indentation="auto" %}
  ```

* {Rust}

  ```gn
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/hello_world/rust/BUILD.gn" region_tag="rustc_tests" adjust_indentation="auto" %}
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/hello_world/rust/BUILD.gn" region_tag="fuchsia_component" adjust_indentation="auto" %}
  ```

To learn more about how Fuchsia uses GN to define components and packages,
see: [Building components](/development/components/build.md).

## Include the example package in your Fuchsia image {#include-the-example}

Note: For new build configurations, these commands can take up to 90 minutes.

To include the example package in your build configuration, use the `--with` flag
when setting your product and board environment:

<pre class="prettyprint">
<code class="devsite-terminal">fx set <var>product</var>.<var>board</var> --with //examples/hello_world</code>
</pre>

For a Fuchsia emulator with the minimum build configuration, the command is:

```posix-terminal
fx set core.qemu-x64 --with //examples/hello_world
```

In this example, `core` is a product with a minimal feature set, which includes
common network capabilities, and `x64` refers to the x64 architecture.

For a Fuchsia device with the minimum build configuration, the command is:

```posix-terminal
fx set core.x64 --with //examples/hello_world
```

See [Configure a build](/development/build/fx.md#configure-a-build) for
more options.

Once you have set your build configuration, build Fuchsia with the following
command:

```posix-terminal
fx build
```

You now have a build that includes the example package that can be
[fetched and launched on demand](/development/build/build_system/boards_and_products.md#universe).

### Explore your product configuration {#explore-your-product-configuration}

You can explore the contents of your product configuration using the
`list-packages` command.

List all:

```posix-terminal
fx list-packages
```

There may be many entries, so add the name to find the one you're looking for:

```posix-terminal
fx list-packages hello-world
hello-world-cpp-unittests
hello-world-rust-tests
hello-world-cpp
hello-world-rust
```

## Run the example component {#run-the-example-component}

To run a Fuchsia component, use its
[Fuchsia package URL](/glossary/README.md#fuchsia-pkg-url) as an argument
to the `run` command:

1.  Open a terminal and run `fx serve-updates`:

    ```posix-terminal
    fx serve-updates
    ```

1.  Open another terminal and run the example component:

    * {C++}

      ```posix-terminal
      ffx component run fuchsia-pkg://fuchsia.com/hello-world#meta/hello-world-cpp.cm
      ```

    * {Rust}

      ```posix-terminal
      ffx component run fuchsia-pkg://fuchsia.com/hello-world#meta/hello-world-rust.cm
      ```

1.  Open another terminal and view the system log:

    ```posix-terminal
    ffx log --filter hello-world
    ```

    The component prints the following output to the log:

    ```none {:.devsite-disable-click-to-copy}
    [ffx-laboratory:hello-world] INFO: Hello, World!
    ```

### Troubleshooting {#troubleshooting}

If `fx serve-updates` is not running, the command prints an error message from
the device or emulator.

If `fx serve-updates` is running, but the package is not found,
[repeat these steps](#include-the-example) and rebuild your Fuchsia image to
include the appropriate packages.
