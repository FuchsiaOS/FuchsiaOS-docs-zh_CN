# Fuchsia emulator

The Fuchsia emulator (FEMU) is the default emulator for Fuchsia. FEMU allows you
to test Fuchsia components and applications without a physical Fuchsia device.

FEMU is included in the Fuchsia source tree. FEMU is downloaded (or updated) by
`jiri`, as part of `jiri update` or `jiri run-hooks`, and is fetched into the
`/prebuilt/third_party/aemu` directory of your Fuchsia source tree.

You can launch FEMU using `ffx emu`.

## FEMU, AEMU, and QEMU {#femu-aemu-and-qemu}

FEMU is based on the
[Android Emulator (AEMU)](https://developer.android.com/studio/run/emulator){:.external},
which is a fork of [QEMU](https://www.qemu.org/){:.external} – in some
instances, such as [debugging the Zircon kernel](#debugging-zircon-kernel), you
should use QEMU instead.

Due to legacy issues, there may be references to AEMU in the code and
documentation.

### Debugging Zircon Kernel {#debugging-zircon-kernel}

If you want to debug the Zircon kernel, you should use `fx qemu` instead. Read
[Debugging the Kernel using QEMU](/development/debugging/qemu.md) to learn
more. This is for kernel developers. Most Fuchsia developers do not need to use
this workflow.

## Features

FEMU looks and behaves like a Fuchsia device, except that no paving or flashing
is required with FEMU.

The features of FEMU include:

*   **GUI Support:** You can run Fuchsia with the GUI (by default) or without
    the GUI (using the `--headless` argument).
*   **GPU Support:** You can run with the host's GPU (by default) with full
    [Vulkan](/development/graphics/magma/concepts/vulkan.md){:.exyernal} support, or
    you can choose software rendering using
    [SwiftShader](https://swiftshader.googlesource.com/SwiftShader/){:.external}.
*   **Remote Development:** You can use a remote desktop with FEMU, either with
    Chrome Remote Desktop or from the command line using
    [fx emu-remote](https://fuchsia.dev/reference/tools/fx/cmd/emu-remote)
    command.

To see full list of supported flags:

```posix-terminal
ffx emu start --help
```

## Image and board support {#image-and-board-support}

When setting up FEMU using `fx set`, FEMU supports the following boards:

*   `qemu-x64`
*   `qemu-arm64`

With the Fuchsia SDK, FEMU supports pre-built images, including:

*   `qemu-x64`
*   `workstation_eng.qemu-x64-release`
*   `qemu-arm64`

ARM64 support (`qemu-arm64`) is very limited and not recommended.

Use `ffx product-bundle list ` to see the full set of available products
available from the SDK, and
<code>ffx product-bundle get {{ '<var>' }}product-bundle</var></code>
to download those products.

## Networking

The `--net` flag specifies the networking mode for the emulator. `--net`
requires a value to indicate which kind of networking to implement.

`--net` has the following possible values:

  - `tap`: Attaches a Tun/Tap interface.
  - `user`: Sets up mapped ports through SLiRP.
  - `none`: Disables networking.
  - `auto`: Checks the host system's capabilities and selects `tap` if it is
            available or `user` if a Tap interface is unavailable.
            `auto` is the default.

On Linux, FEMU should generally be run with the `--net tap` flag that
provides networking through an emulated NIC.

Note: Instructions for setting up
networking for FEMU is in the
[Start the Fuchsia Emulator](/get-started/set_up_femu.md) guide.

`--net tap` and `--net user` allow the emulator to be discoverable
when running `ffx target list`. `--net none` disables networking, which causes
the emulator to not be discoverable after running `ffx target list`.

If starting the emulator with `ffx emu start --net user`, an available TCP
port from the host is picked and forwarded to the emulator's SSH port.

You can manually set the SSH address and use `fx` tools to interact
with your emulator by running the following command:

```posix-terminal
ffx emu start --net user --port-map {{ '<var>' }}PORT-NAME{{ '</var>' }}:{{ '<var>' }}PORT-NUMBER{{ '</var>' }}
```

Replace the following:

  * <var>PORT-NAME</var>: The chosen name for the port. An example port name could be
    `ssh`.
  * <var>PORT-NUMBER</var>: The number of the port. An example port number is
  `8022`.

Any named ports can be set the same way. The ports that can be mapped
are named in the virtual device specification.

To verify that your `fx` tool is using the correct port, run the
following command:

```posix-terminal
ffx target get-ssh-address
```

You should see the SSH address printed next to `Device name`.

To SSH into the emulator, run the following command:

```posix-terminal
fx shell
```

## Unsupported CPUs {#unsupported-cpu}

FEMU currently does not run on:

* ARM64 processors, including the Apple M1 processor.
* AMD processors.

## Supported hardware for graphics acceleration {#supported-hardware}

FEMU currently supports a limited set of GPUs on macOS and Linux for
hardware graphics acceleration. FEMU uses a software renderer fallback
for unsupported GPUs.


<table>
  <tbody>
    <tr>
      <th>Operating System</th>
      <th>GPU Manufacturer</th>
      <th>OS / Driver Version</th>
    </tr>
    <tr>
      <td>Linux</td>
      <td>Nvidia Quadro</td>
      <td>Nvidia Linux Drivers <a href="https://www.nvidia.com/download/driverResults.aspx/160175/en-us">440.100</a>+</td>
    </tr>
    <tr>
      <td>macOS</td>
      <td><a href="https://support.apple.com/en-us/HT204349#intelhd">Intel HD Graphics</a></td>
      <td>macOS version 10.15+</td>
    </tr>
    <tr>
      <td>macOS</td>
      <td>AMD Radeon Pro</td>
      <td>macOS version 10.15+</td>
    </tr>
  </tbody>
</table>

## Common usage {#common-usage}

To launch FEMU, complete the [Get started with Fuchsia](/get-started/README.md) guide.

Alternatively, you can use the Fuchsia SDK and use pre-built system images.

Once you're able to launch FEMU, you can perform the following tasks:

*   [Test components](/development/run/run-test-component.md)
*   [Run end-to-end tests](/development/testing/run_an_end_to_end_test.md)
