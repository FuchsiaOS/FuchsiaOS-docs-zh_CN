# Build system

Fuchsia builds use [Generate Ninja](https://gn.googlesource.com/gn/) (GN),
a meta-build system that generates build files consumed by
[Ninja](https://ninja-build.org/){:.external}, which executes the actual build.
The build system provides the tools to configure the build for a specific
product and templates to build code for Fuchsia targets.

## Build targets

You define individual build targets for GN using `BUILD.gn` files located with
your project source code. The Fuchsia build system provides templates as GN
imports (`.gni`) for you to declare Fuchsia artifacts, such as:

* `fuchsia_component()`: Defines an executable
  [component](concepts/components/v2), containing the manifest, program
  binary, and resources.
* `fuchsia_package()`: Defines a [package](concepts/packages/package.md)
  containing one or more components for distribution in a package repository.
* `fuchsia_test_package()`: Defines a package containing test components.

Note: You can see all the Fuchsia build templates in
[`//build/components.gni`](/build/components.gni).

Below is an example of a `BUILD.gn` file for a simple component package with
tests:

```gn
import("//build/components.gni")

executable("bin") {
  sources = [ "main.cc" ]
}

fuchsia_component("hello-world-component") {
  deps = [ ":bin" ]
  manifest = "meta/hello-world.cml"
}

fuchsia_package("hello-world") {
  deps = [
    ":hello-world-component",
  ]
}

fuchsia_component("hello-world-test-component") {
  testonly = true
  deps = [ ":bin_test" ]
  manifest = "meta/hello-world-bin-test.cml"
}

fuchsia_test_package("hello-world-tests") {
  test_components = [ ":hello-world-test-component" ]
}
```

A unique **label** composed of the target's name and the path to its `BUILD.gn`
file identifies everything that can participate in the build. In the above
example, the `hello-world` target might have a label that looks like
this: `//src/examples/basic:hello-world`.

Note: For more details on the mechanics of building with GN, see
[Introduction to GN](development/build/build_system/intro.md).

## Build configuration

The GN front-end configures the build according to the chosen Fuchsia
**product configuration**, collecting all the necessary packages and components
required by the build. These targets are defined in various `BUILD.gn` files
throughout the source tree. The output of the GN step is an optimized set of
instructions for Ninja in the build directory.

The build system invokes GN when you run the `fx set` command to configure
the build.

```posix-terminal
fx set workstation.qemu-x64
```

<aside class="key-point">
You can also invoke GN directly with <code>fx gn</code> to customize or
troubleshoot the build.
</aside>

You should run the GN configuration step anytime you want to adjust the product
configuration or the packages available to the build. GN is also invoked
automatically during a build anytime one of the `BUILD.gn` files in the current
configuration is changed.

## Boards and products

The Fuchsia build system defines the baseline configuration for a Fuchsia build
as a combination of a **product** and **board**. Together, these elements form
the build configuration you provide to `fx set`.

![Data table showing build configurations separated into "product" and "board"
layers. Each describes a different set of functional elements provided to the
final build.](get-started/images/build/build-configuration.png){: width="570"}

Boards define the architecture that the build targets, which may affect what
drivers are included and influence device specific kernel parameters.

This codelab targets the `qemu-x64` board, which supports the Fuchsia emulator
(FEMU) running on x64 architecture.

<aside class="key-point">
You can discover all the available target boards with
<code>fx list-boards</code>.
</aside>

A product defines the software configuration that a build produces. This
configuration may include what services are available and the user-facing
experience.

This codelab targets the `workstation` product, which provides a general
purpose computing distribution of Fuchsia with a graphical interface and some
built-in user apps like a terminal and browser.

<aside class="key-point">
You can discover all the available target products with
<code>fx list-products</code>.
</aside>

## Build

Once the GN build configuration is complete, Ninja consumes the generated build
files and runs the appropriate compile, link, and packaging commands to generate
the Fuchsia image.

The build system invokes Ninja when you run the `fx build` command to execute
the current build configuration.

```posix-terminal
fx build
```

<aside class="key-point">
You can also invoke Ninja directly with <code>fx ninja</code> to customize or
troubleshoot the build.
</aside>

## Exercise: Build workstation

In this exercise, you'll build the `workstation` product configuration from
source to run on the `qemu-x64` emulator board.

### Configure the build

Set up the build environment for the `workstation` product and `qemu-x64` board:

```posix-terminal
fx set workstation.qemu-x64
```

This command runs GN on the set of targets defined in the product's build
configuration to produce the build instructions. **It does not actually
perform the build**, but instead defines the parameters of what is considered
buildable.


<aside class="key-point">
You can also ask GN to regenerate the existing build configuration using
<code>fx gen</code> without needing to supply the product configuration again.
This can be helpful when you are editing <code>BUILD.gn</code> files and want to
quickly validate without running a full build.
</aside>

### Inspect the build configuration

Once the build is configured, use `fx list-packages` to print the set of
packages the build is aware of:

```posix-terminal
fx list-packages
```

This is a useful tool to determine if a package you need was properly included
in the build configuration.

### Build Fuchsia workstation

Build the workstation target with `fx build`:

<aside class="caution">
A full build on a fresh checkout of the source can take upwards of 60-90 minutes
to complete, depending on the capabilities of the build machine. Subsequent
incremental builds will only take a few minutes.
</aside>

```posix-terminal
fx build
```

<<../_common/_restart_femu.md>>

### Inspect the device

Open another terminal window and run the following command to print the details
of your device target:

```posix-terminal
ffx target show
```

Look for the build configuration of the target output:

```none {:.devsite-disable-click-to-copy}
{{ '<strong>' }}Version: "2000-01-01T12:00:00+00:00"{{ '</strong>' }}
Product: "workstation"
Board: "qemu-x64"
{{ '<strong>' }}Commit: "2000-01-01T12:00:00+00:00"{{ '</strong>' }}
```

Notice that the configuration points to the build you just completed on your
machine.

You are now running your own build of Fuchsia!
