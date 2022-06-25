# Product packages

A product configuration encapsulates a set of packages and components required
to support the target user experience on the underlying hardware. Each product
configuration declares its default list of packages using their GN labels in a
GN import (`.gni`) file. You can augment the contents of the current build
configuration in a few different ways.


<aside class="key-point">
You can see the board and product definitions for the default Fuchsia
configurations at
<a href="https://cs.opensource.google/fuchsia/fuchsia/+/main:boards/"><code>//boards</code></a> and
<a href="https://cs.opensource.google/fuchsia/fuchsia/+/main:products/"><code>//products</code></a>.
</aside>

## Dependency sets

Packages in a build configuration are assigned to one of three dependency sets:

* **Base:** System-critical packages bundled into the system image. Base
  packages always resolve to the version in the system image, even if a
  different version is available from a package repository. Updating a base
  package requires a system (OTA) update.
* **Cache:** Additional packages bundled into the system image. Cached packages
  are updated from a package repository if a different version is available.
* **Universe:** Everything else. This set of packages is only available through
  a package repository and delivered on demand.

![Diagram showing how packages live in the base, cache, or universe dependency
sets of the build configuration. This set determines how new versions of the
package get resolved.](/docs/get-started/images/build/packages.png){: width="645"}

<aside class="key-point">
Fuchsia devices revert to the packages built into the system image on reboot.
Cached and universe packages resolve again once the device connects to a
package repository.
</aside>

Below is an example snippet of a product configuration file. This product
inherits all the packages defined in the `core` product, and then adds a
few additional packages to each dependency set:

```gn
import("//products/core.gni")

legacy_base_package_labels += [
  "//src/chromium:web_engine",
  "//src/flutter",
  "//src/fonts",
]

legacy_cache_package_labels += [
  "//src/media/playback/bundles:services",
]

legacy_universe_package_labels += [
  "//src/ui/examples:bouncing_ball",
]
```

In this example the build system would package the runtimes, resources, and
services into the disk image for the target device. The examples would only be
available on demand from a package server.

Note: For more details on product configurations and dependencies, see
[Products and Boards](/docs/development/build/build_system/boards_and_products.md).

## Manual build customization

Creating a new complete product configuration is not the only way to customize
the build. You can also provide additional labels to the `fx set` command using
these flags:

* `--with`: Add a target label to the universe dependency set.
* `--with-base`: Add a target label to the base dependency set.
* `--with-cache`: Add a target label to the cache dependency set.

This is a good way to temporarily enable packages for development that you don't
want to include in the final build target, such as tests. For example, the
following command adds all the packages in the Fuchsia `tests` bundle to a
standard `workstation` build.

```posix-terminal
fx set workstation.qemu-x64 --with //bundles:tests
```

<aside class="key-point">
<b>Build Arguments</b>
<p>The <code>fx set</code> command and these configuration options ultimately
populate the build arguments that GN uses to generate the build configuration.
Package labels are a common example of build arguments in action.</p>
<p>The current build arguments are stored in the <code>args.gn</code> file in
the build output directory. For more advanced use cases, you can use the
<code>fx args</code> command to interactively edit <code>args.gn</code> before
generating the build configuration.</p>
<p>For a complete list of support build arguments, see
<a href="/docs/gen/build_arguments.md">GN Build Arguments</a>.
</aside>

### Developing with packages

Recall from the introduction that Fuchsia packages do not "install" to a device,
they **resolve** from the local package cache or a package repository. When
developing Fuchsia software, this means that testing your code involves the
following steps:

1.  Build an updated package with any code changes.
1.  Publish updated packages to a package repository.
1.  Trigger an update on the target device.

![Diagram showing how during development, the developer tools publish packages
to the TUF repository, which notifies the Fuchsia device to resolve the latest
version.](/docs/get-started/images/build/package-resolution.png){: width="644"}

Developer tools such as `fx build` publish package updates to a local package
repository as part of the build process. The **dependency set** where your
product declares the package determines how to trigger the update on the target
device:


Note: For more details on the package development workflow, see
[Developing with Fuchsia packages](/docs/concepts/packages/package_update.md).

* Base packages can only be updated by performing a whole system update (OTA).
  Trigger a system update using `fx ota` or flash a new system image on the
  device to update base packages.
* Cached and universe packages update automatically the next time the package
  is resolved.

## Exercise: Customize the build

In this exercise, you'll customize the `workstation` build by temporarily
including additional packages in the universe package set — making them
available to the target device.

### Add packages to the build

You can bundle additional targets with your build configuration using the
`--with` flag of `fx set`. Reconfigure your `workstation` build to include all
the Fuchsia examples:

```posix-terminal
fx set workstation.qemu-x64 --with //examples
```

This is commonly used to include test packages you need to run on the device or
a new package you may be working on that isn't yet included in a product
configuration.

Verify that the example packages were added to the build:

```posix-terminal
fx list-packages example
```

<aside class="key-point">
  <b>Extra credit</b>
  <p>Which package set was used to include the examples? Can you confirm this
  using the filter flags that <code>list-packages</code> supports?</p>
  <p>What do the build arguments look like when you run <code>fx args</code>?</p>
</aside>

### Explore build targets using GN

GN comes with a powerful set of diagnostic tools that allow you to examine the
targets configured in your build. Follow along to discover how to use `fx gn`
subcommands to explore the build targets.

<aside class="key-point">
  Since these are GN commands, you can perform these after the build configuration
  step without the need for a full build. These commands operate after GN
  <code>template()</code> expansion, so the output is expressed in terms of
  <a href="https://gn.googlesource.com/gn/+/main/docs/reference.md#targets">
  built-in GN target types</a>.
</aside>

Begin by using the `desc` command to print details about the `//examples`
bundle you just added to the build.

```posix-terminal
fx gn desc out/default //examples
```

This command prints details about the target type and all its dependencies. For
the `//examples` bundle, the dependencies list represents the individual
example packages added to the build.

Explore the details of the `hello-world` target using the same command.

```posix-terminal
fx gn desc out/default //examples/hello_world:hello-world
```

This target represents a **package** containing multiple **components**, so you
will see additional dependencies such as component manifests and package
metadata.

Go down one more level to explore the `hello-world-rust` component.

```posix-terminal
fx gn desc out/default //examples/hello_world/rust:hello-world-rust-component
```

From the perspective of a specific target, such as the `hello-world-rust`
component, you can also look upwards in the build graph using the `refs` command.
This reports the incoming references to the given target.

```posix-terminal
fx gn refs out/default //examples/hello_world/rust:hello-world-rust-component
```

Finally, use the `path` command to report the reference chain between any two
targets. This can be useful to determine where and how your target is included
in the build at all by checking the reference path between your target and
`//:default`.

```posix-terminal
fx gn path out/default //:default //examples/hello_world:hello-world
```

### Run the example in the emulator

Run `fx build` again to build the updated packages:

```posix-terminal
fx build
```

Use the `ffx component` command to run a Hello World component example:

```posix-terminal
ffx component run fuchsia-pkg://fuchsia.com/hello-world#meta/hello-world-cpp.cm
```

Open a new terminal window and filter the device logs for messages from the
example:

```posix-terminal
ffx log --filter hello
```

You should see the following output in the device logs:

```none {:.devsite-disable-click-to-copy}
[hello-world-cpp][I] Hello, World!
```

## Exercise: Create a new build product

In this next exercise, you'll encapsulate these additional packages into a new
product configuration that extends `workstation`.

### Declare the product configuration

Create a new directory under `//vendor` called `fuchsia-codelab`:

```posix-terminal
mkdir -p vendor/fuchsia-codelab
```

Create a new file `fuchsialab.gni` under `//vendor/fuchsia-codelab/products`
with the following contents:

```gn
# Extend the workstation product
import("//products/workstation.gni")
```

This creates a new product called `fuchsialab` that extends `workstation`,
inheriting all the package labels it defines.

Verify that the build system recognizes your new product with the
`fx list-products` command. You should see `fuchsialab` in the product list.

```posix-terminal
fx list-products
```

### Add packages to your product

To customize the specific packages included with your product, add them to the
labels for the appropriate package set in your product configuration.

Add the following lines to `//vendor/fuchsia-codelab/products/fuchsialab.gni`
to include the Hello World example in your custom product:

```gn
legacy_universe_package_labels += [
    "//examples/hello_world",
]
```

<aside class="key-point">
<b>Tip:</b> You can also remove packages added by a parent product by
"subtracting" them from the appropriate label set.
</aside>

### Build your new product

Reconfigure the build for the `fuchsialab` product, running on the FEMU board:

```posix-terminal
fx set fuchsialab.qemu-x64
```

Verify that the Hello World example package is now part of the build:

```posix-terminal
fx list-packages hello
```

Run `fx build` to generate a new image for your custom product:

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
Version: "2000-01-01T12:00:00+00:00"
{{ '<strong>' }}Product: "fuchsialab"{{ '</strong>' }}
{{ '<strong>' }}Board: "qemu-x64"{{ '</strong>' }}
Commit: "2000-01-01T12:00:00+00:00"
```

Congratulations! You just built your own custom product based on Fuchsia!
