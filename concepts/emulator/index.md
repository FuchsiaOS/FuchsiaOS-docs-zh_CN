# Fuchsia emulator (FEMU)

The Fuchsia emulator (FEMU) allows you to test Fuchsia components and applications without needing a Fuchsia device.
FEMU is included in Fuchsia source, and it’s downloaded by `jiri` as part of `jiri update` or `jiri run-hooks`.
It’s fetched into the Fuchsia directory `/prebuilt/third_party/aemu`.

You can call FEMU with `fx` using `fx vdl`. Alternatively,
you can call FEMU from the Fuchsia IDK using `fvdl`.

## FEMU and other emulators {#femu-and-other-emulators}

FEMU is the default emulator for Fuchsia. FEMU is based on the
[Android Emulator (AEMU)](https://developer.android.com/studio/run/emulator), which is a fork of
[QEMU](https://www.qemu.org/). Due to legacy issues, there may be references to AEMU in the code and documentation.

In some instances, such as [emulating Zircon](#emulating-zircon), you must use QEMU instead.


## FEMU Features {#femu-features}

FEMU looks and behaves like a Fuchsia device, with the exception that no paving is required.

FEMU features include:

*   **GUI Support:** You can run Fuchsia with the GUI (by default) or without the GUI
    (using the `--headless` argument).
*   **GPU Support:** You can run with the host’s GPU (by default) with full
    [Vulkan](/docs/concepts/graphics/magma/vulkan.md) support, or you can choose
    software rendering using [SwiftShader](https://swiftshader.googlesource.com/SwiftShader/).
*   **Remote Development:** You can use a remote desktop with FEMU, either with Chrome Remote Desktop
     or from the command line using [fx emu-remote](https://fuchsia.dev/reference/tools/fx/cmd/emu-remote)
     command.

To see full list of supported flags:

```posix-terminal
fx vdl start --help
```

To configure these features, see the [Set up and start FEMU](/docs/get-started/set_up_femu.md)
page.

If you’re using the Fuchsia IDK, `fvdl` supports the same flags as `fx vdl`


## FEMU limitations {#femu-limitations}

### FEMU image and board support {#femu-image-and-board-support}

When setting up FEMU using `fx set`, FEMU only supports the following boards:

*   `qemu-x64`
*   `qemu-arm64`

When using the Fuchsia IDK to set up FEMU, you are limited to the following pre-built images:

*   `qemu-x64`
*   `workstation.qemu-x64-release`
*   `qemu-arm64`

Note: ARM64 support (`qemu-arm64`) is very limited and not recommended.

### FEMU networking  {#femu-networking}

On Linux, Fuchsia Emulator should generally be run with the `-N` flag that provides networking through an
emulated NIC. Instructions for setting up networking for FEMU is in
[Setting up the Fuchsia Emulator](/docs/get-started/set_up_femu.md).

Note: Without `-N`, your emulator won't be discoverable using `fx list-devices`. However, you can manually set the SSH address and use `fx` tools to interact with your emulator.

If starting the emulator without `-N` (i.e `fx vdl start`), an available TCP port from the host will be
picked and forwarded to the emulator's SSH port. When the emulator launches successfully, instruction to set `fx` tools with the correct SSH port are printed in the terminal output. 
Then, you can manually set the SSH device:


```posix-terminal
fx set-device 127.0.0.1:{{ '<var>' }}SSH_PORT{{ '</var>' }}
```

To verify `fx` is using the correct port:

```posix-terminal
fx status
```

You should see the SSH address printed next to `Device name`. To SSH into the emulator:

```posix-terminal
fx ssh
```

### Emulating Zircon {#emulating-zircon}

If you only want to emulate Zircon, you must use `fx qemu` instead. Read
[Debugging the Kernel using QEMU](/docs/development/debugging/qemu.md) to
learn more. This is for kernel developers. Most Fuchsia developers do not need
to use this workflow.


## FEMU common usage  {#femu-common-usage}

To use FEMU, you must first
[download the Fuchsia source](/docs/get-started/get_fuchsia_source.md)
and [build Fuchsia](/docs/get-started/build_fuchsia.md).

Alternatively, you can use the Fuchsia IDK and use pre-built system images.

Then you can use FEMU to do the following:

*   [Set up and start FEMU](/docs/get-started/set_up_femu.md)
*   [Test components](/docs/development/run/run-test-component.md)
*   [Run end-to-end tests](/docs/development/testing/run_an_end_to_end_test.md)
