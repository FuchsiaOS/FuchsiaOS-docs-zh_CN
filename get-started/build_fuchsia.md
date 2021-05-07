# Configure and build Fuchsia {#configure-and-build-fuchsia}

This document describes how to set up and build Fuchsia.

## Prerequisites

Before you can set up and build Fuchsia, you need to follow the steps in
[get the Fuchsia source code](/docs/get-started/get_fuchsia_source.md)
to download Fuchsia source code and set up your environment variables.

## Set build configuration

To set your build configuration for Fuchsia, run the following command:

<pre class="prettyprint">
<code class="devsite-terminal">fx set <var>product</var>.<var>board</var></code>
</pre>

The `fx set` command takes <var>PRODUCT</var> and <var>BOARD</var> arguments,
which define the
[product and board](/docs/concepts/build_system/boards_and_products.md)
configuration of your build. This configuration informs the build system what
packages to build for your Fuchsia device.

For a Fuchsia emulator with the core set of Fuchsia features, the build configuration is:

```posix-terminal
fx set core.qemu-x64
```

In this example:

  * `core` is a product with the minimum feature set for Fuchsia, which includes
     common network capabilities.
  * `qemu-x64` is the board, which refers to the x64 architecture on the
     Fuchsia Emulator (FEMU), which is based on the open source emulator, QEMU.

For a Fuchsia device with the core set of Fuchsia features, the build configuration is

```posix-terminal
fx set core.x64
```

See [Configure a build](/docs/development/build/fx.md#configure-a-build) for
more product and board options.

### Speed up the build {#speed-up-the-build}

Note: This step is optional.

To reduce the time it takes to build Fuchsia, you can do any of the following:

*   [Speed up the build with Goma](#speed-up-the-build-with-goma)
*   [Speed up the build with ccache](#speed-up-the-build-with-ccache)

#### Speed up the build with Goma {#speed-up-the-build-with-goma}

[Goma](https://chromium.googlesource.com/infra/goma/server/){:.external} is a
distributed compiler service for open source projects such as Chrome, Android
and Fuchsia. If you have access to Goma, run the following command to enable a
Goma client on your machine:

```posix-terminal
fx goma
```

#### Speed up the build with ccache {#speed-up-the-build-with-ccache}

If you do not have access to Goma, but want to accelerate the Fuchsia build
locally, use <code>[ccache](https://ccache.dev/){:.external}</code> to cache
artifacts from previous builds.

To use `ccache` on Linux, install the following package:

```posix-terminal
sudo apt-get install ccache
```

For macOS, see
[Using CCache on Mac](https://chromium.googlesource.com/chromium/src.git/+/HEAD/docs/ccache_mac.md){:.external}
for installation instructions.

`ccache` is enabled automatically if your `CCACHE_DIR` environment variable
refers to an existing directory.

To override the default behavior, pass the following flags to `fx set`:

*   Force use of ccache even if other accelerators are available:

    ```posix-terminal
    fx set core.x64 --ccache
    ```

*   Disable use of ccache:

    ```posix-terminal
    fx set core.x64 --no-ccache
    ```

## Build Fuchsia

Note: Building Fuchsia can take up to 90 minutes.

To build Fuchsia, run the following command:

```posix-terminal
fx build
```

The `fx build` command executes the build to transform source code into packages
and other build artifacts.

If you modify source code, re-run the `fx build` command to perform an
incremental build, or run the `fx -i build` command to start a watcher, which
automatically builds whenever you update source code.

See [Execute a build](/docs/development/build/fx.md#execute-a-build) for more
information.

## Next steps

Set up Fuchsia on an emulator or a device:

 * To set up a Fuchsia emulator and experiment with Fuchsia, follow the steps in
   [Set up the Fuchsia emulator (FEMU)](/docs/get-started/set_up_femu.md).
 * To set up a hardware device, follow the steps in 
   [Installing Fuchsia on a device](/docs/development/hardware/paving.md) and the
   [build and pave quickstart](/docs/development/build/build_and_pave_quickstart.md).

Once you have set up the emulator or paved a device with Fuchsia, see:
 
 *  [Explore Fuchsia](/docs/get-started/explore_fuchsia.md) to learn more about how Fuchsia
    is structured and common workflows.
