# The Fuchsia build system

## Overview

The Fuchsia build system aims at building both boot images and updatable
packages for various devices. To do so, it uses [GN][gn-main], a meta-build
system that generates build files consumed by [Ninja][ninja-main], which
executes the actual build.

Note that Zircon uses a different build system, though still using GN and
Ninja.

## Getting started

If you are unfamiliar with Fuchsia's build system and GN, see [Using GN
build][gn-preso], which outlines the basic principles of the GN build system.

## Boards and Products

The contents of the generated image are controlled by a combination of a
board and a product that are the minimal starting configuration of a Fuchsia
build. Boards and products define dependency sets that define the packages
that are included in images, updates, and package repositories.
[boards and products](boards_and_products.md) documents the structure and
usage of these build configurations.

## Bundles

A bundle is a grouping of related packages within a part of the source tree,
such as all tools or all tests. An overview of bundles is provided in
[bundles](bundles.md). A set of top-level bundles are defined in
[`//bundles`](/bundles/README.md).

## Build targets

Build targets are defined in `BUILD.gn` files scattered all over the source
tree. These files use a Python-like syntax to declare buildable objects:

```py
import("//build/some/template.gni")

my_template("foo") {
  name = "foo"
  extra_options = "//my/foo/options"
  deps = [
    "//some/random/framework",
    "//some/other/random/framework",
  ]
}
```

Available commands (invoked using gn cli tool) and constructs (built-in target
declaration types) are defined in the [GN reference][gn-reference]. There are
also a handful of custom templates in `.gni` files in the
[`//build` project][build-project].

Fuchsia defines many [custom templates](development/components/build.md#gn_templates)
to support defining and building Fuchsia specific artifacts.

## Executing a build

The simplest way to this is through the `fx` tool, as described in
[fx workflows](development/build/fx.md). Read on to see
what `fx` does under the hood.

The rest of this document assumes that `gn` and `ninja` commands are
available in your `PATH`. These commands can be found in
`prebuilt/third_party/gn/<platform>` and
`prebuilt/third_party/ninja/<platform>` respectively. Alternatively, if
you want to avoid modifying your `PATH`, you can prefix all invocations
with `fx`, i.e. `fx gn` or `fx ninja`.

### Gen step

First configure the primary build artifacts by choosing the board and product
to build:

```posix-terminal
fx gn gen $(fx get-build-dir) --args='import("//boards/x64.gni") import("//products/core.gni")'
```

This will create a build directory (usually `out/default`) containing Ninja
files.

The equivalent `fx set` command is:

```posix-terminal
fx set core.x64
```

For a list of all GN build arguments, run:

```posix-terminal
fx gn args $(fx get-build-dir) --list
```

For documentation on the `select_variant` argument, see [Variants](variants.md).

### Build step

The next step is to run the actual build with Ninja:

```posix-terminal
fx ninja -C $(fx get-build-dir)
```

This is what gets run under the hood by `fx build`.

## Rebuilding

In order to rebuild the tree after modifying some sources, just rerun
**Build step**. This holds true even if you modify `BUILD.gn` files as GN adds
Ninja targets to update Ninja targets if build files are changed! The same
holds true for other files used to configure the build. Any change of source
that requires a manual re-invocation of the **Gen step** is a build bug and
should be reported.

## Tips and tricks

### Inspecting the content of a GN target

```posix-terminal
fx gn desc $(fx get-build-dir) //path/to/my:target
```

### Finding references to a GN target

```posix-terminal
fx gn refs $(fx get-build-dir) //path/to/my:target
```

### Referencing targets for the build host

Various host tools (some used in the build itself) need to be built along with
the final image.

To reference a build target for the host toolchain from a module file:

```
//path/to/target(//build/toolchain:host_x64)
```

To reference a build target for the host toolchain from within a `BUILD.gn`
file:

```
//path/to/target($host_toolchain)
```

### Building only a specific target

If a target is defined in a GN build file as `//foo/bar/blah:dash`, that target
(and its dependencies) can be built with:

```posix-terminal
fx ninja -C $(fx get-build-dir) -j64 foo/bar/blah:dash
```

Note that this only works for targets in the default toolchain.

Note: Building package targets does not result in an updated package
repository, because the package repository is updated by the `updates` group
target. In order for updated package changes to be made available via `fx
serve`, users must build the `updates` group.

### Exploring Ninja targets

GN extensively documents which Ninja targets it generates. The documentation is
accessible with:

```posix-terminal
fx gn help ninja_rules
```

You can also browse the set of Ninja targets currently defined in your output
directory with:

```posix-terminal
fx ninja -C $(fx get-build-dir) -t browse
```

Note that the presence of a Ninja target does not mean it will be built - for
that it needs to depend on the “default” target.

### Understanding why Ninja does what it does

Add `-d explain` to your Ninja command to have it explain every step of its
execution.

### Debugging build timing issues

When running a build, Ninja keeps logs that can be used to generate
visualizations of the build process:

1. Delete your output directory - this is to ensure the logs represent only the
   build iteration you’re about to run;
1. Run a build as you would normally do;
1. Get <https://github.com/nico/ninjatracing>;
1. Run `ninjatracing <output directory>/.ninja_log > trace.json`;
1. Load the resulting json file in Chrome in `about:tracing`.


## Troubleshooting

### My GN target is not being built!

Make sure it rolls up to a label defined in a module file, otherwise the build
system will ignore it.

### GN complains about missing `sysroot`.

You likely forgot to run both commands of **Build step**.

> TODO(pylaligand): command showing path to default target


### Internal GN setup

> TODO(pylaligand): .gn, default target, GN labels insertion

[gn-main]: https://gn.googlesource.com/gn/
[gn-preso]: https://docs.google.com/presentation/d/15Zwb53JcncHfEwHpnG_PoIbbzQ3GQi_cpujYwbpcbZo/
[ninja-main]: https://ninja-build.org/
[gn-reference]: https://gn.googlesource.com/gn/+/HEAD/docs/reference.md
[build-project]: /build/
[zircon-getting-started]: zircon/getting_started.md
