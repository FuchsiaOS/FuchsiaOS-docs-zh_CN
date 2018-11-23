# The build system

## Overview

The Fuchsia build system aims at building both boot images and installable
packages for various devices. To do so, it uses [GN][gn-main], a meta-build
system that generates build files consumed by [Ninja][ninja-main], which
executes the actual build. [Using GN build][gn-preso] is a good intro to GN.

Note that Zircon uses an entirely different build system based on GNU Make.
The rest of the build relies on Zircon being built ahead of time.

## Products

The contents of the generated image are controlled by a set of top level
products. Products define sets of packages that are included in boot and
system update images, preinstalled in paver images, and installable using the
update system. [products](products.md) documents the structure and usage of
fields in product definitions.

## Packages

The contents of products are packages, which may aggregate or reference other
packages and GN labels that are to be built. See [packages](packages.md)
for more information.

## Build targets

Build targets are defined in `BUILD.gn` files scattered all over the source
tree. These files use a Python-like syntax to declare buildable objects:
``` py
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

These custom templates mostly define custom target declaration types, such as
the [package declaration type][packages-source].

> TODO(pylaligand): list available templates

## Executing a build

The simplest way to this is through the `fx` tool, as described in
[Getting Started](/getting_started.md#Setup-Build-Environment). Read on to see
what `fx` does under the hood.

### A

The first step is to build Zircon which uses its own build system:
```bash
$ scripts/build-zircon.sh
```

This is what gets run under the hood by `fx build-zircon`, which is run by `fx
full-build`.

For a list of all options, run `build-zircon.sh -h`. See Zircon's
[Getting started][zircon-getting-started] and
[Makefile options][zircon-makefile-options] for details.

### B

Then configure the content of the generated image by choosing the top level
product to build:
```
# --products and --packages can be omitted to use the defaults, which are
# $layer/products/default.gni and empty, respectively.
$ buildtools/gn gen out/x64 --args='import("//garnet/products/product_name.gni") fuchsia_packages=["garnet/packages/my_stuff"]'
```

This will create an `out/x64` directory containing Ninja files.

The equivalent fx set command is:
```
$ scripts/fx set x64 --products garnet/products/base.gni --packages garnet/packages/my_stuff
```

For a list of all GN build arguments, run `buildtools/gn args out/x64 --list`.
For documentation on the `select_variant` argument, see [Variants](variants.md).

### C

The final step is to run the actual build with Ninja:
```
$ buildtools/ninja -C out/<arch> -j 64
```

This is what gets run under the hood by `fx build`.

## Rebuilding

### After modifying non-Zircon files

In order to rebuild the tree after modifying some sources, just rerun step
**C**. This holds true even if you modify `BUILD.gn` files as GN adds Ninja
targets to update Ninja targets if build files are changed! The same holds true
for package files used to configure the build.

### After modifying Zircon files

You will want to rerun **A** and **C**.

### After syncing sources

You’ll most likely need to run **A** once if anything in the Zircon tree was
changed. After that, run **C** again.


## Tips and tricks

## Inspecting all packages in a product

```bash
$ build/gn/preprocess_products.py --products '["garnet/products/default"]'
```

### Visualizing the hierarchy of build packages

```bash
$ scripts/visualize_module_tree.py > tree.dot
$ dot -Tpng tree.dot -o tree.png
```

### Inspecting the content of a GN target

```bash
$ buildtools/gn desc out/x64 //path/to/my:target
```

### Finding references to a GN target

```bash
$ buildtools/gn refs out/x64 //path/to/my:target
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
```bash
$ buildtools/ninja -C out/x64 -j64 foo/bar/blah:dash
```
Note that this only works for targets in the default toolchain.

### Exploring Ninja targets

GN extensively documents which Ninja targets it generates. The documentation is
accessible with:
```bash
$ buildtools/gn help ninja_rules
```

You can also browse the set of Ninja targets currently defined in your output
directory with:
```bash
$ buildtools/ninja -C out/x64 -t browse
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

### GN complains about a missing `sysroot`.

You likely forgot to run **A** before running **B**.

> TODO(pylaligand): command showing path to default target


### Internal GN setup

> TODO(pylaligand): .gn, default target, mkbootfs, GN labels insertion

[gn-main]: https://chromium.googlesource.com/chromium/src/tools/gn/+/HEAD/README.md
[gn-preso]: https://docs.google.com/presentation/d/15Zwb53JcncHfEwHpnG_PoIbbzQ3GQi_cpujYwbpcbZo/
[ninja-main]: https://ninja-build.org/
[gn-reference]: https://gn.googlesource.com/gn/+/master/docs/reference.md
[build-project]: https://fuchsia.googlesource.com/build/+/master/
[zircon-getting-started]: https://fuchsia.googlesource.com/zircon/+/master/docs/getting_started.md
[zircon-makefile-options]: https://fuchsia.googlesource.com/zircon/+/master/docs/makefile_options.md
