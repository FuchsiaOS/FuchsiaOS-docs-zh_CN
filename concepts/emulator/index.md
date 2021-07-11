<!-- 
# Fuchsia emulator (FEMU)
 -->
# Fuchsia 模拟器（FEMU）

<!-- 
The Fuchsia emulator (FEMU) allows you to test Fuchsia components and applications without needing a Fuchsia device.
FEMU is included in Fuchsia source, and it’s downloaded by `jiri` as part of `jiri update` or `jiri run-hooks`.
It’s fetched into the Fuchsia directory `/prebuilt/third_party/aemu`.

You can call FEMU with `fx` using `fx vdl`. Alternatively,
you can call FEMU from the Fuchsia IDK using `fvdl`.
 -->
Fuchsia 模拟器（FEMU）使您无需 Fuchsia 设备即可测试 Fuchsia 组件和应用。
FEMU 包含在 Fuchsia 源中，并且由 `jiri` 在执行 `jiri update` 或 `jiri run-hooks` 的同时下载。
FEMU 存放在 Fuchsia 的 `/prebuilt/third_party/aemu` 路径中。

您可以使用 `fx vdl` ，通过 `fx` 来调用 FEMU。或者，您也可以使用 `fvdl`，通过 Fuchsia IDK 来调用 FEMU。

<!-- 
## FEMU and other emulators {#femu-and-other-emulators}
 -->
## FEMU 与其他模拟器 
{#femu-and-other-emulators}

<!-- 
FEMU is the default emulator for Fuchsia. FEMU is based on the
[Android Emulator (AEMU)](https://developer.android.com/studio/run/emulator), which is a fork of
[QEMU](https://www.qemu.org/). Due to legacy issues, there may be references to AEMU in the code and documentation.

In some instances, such as [emulating Zircon](#emulating-zircon), you must use QEMU instead.
 -->
FEMU 是 Fuchsia 的默认模拟器。FEMU 基于[安卓模拟器（AEMU）](https://developer.android.com/studio/run/emulator)，而安卓模拟器是 [QEMU](https://www.qemu.org/) 的一个派生（fork）。由于历史遗留问题，代码和文档中可能会有 AEMU 的参考内容。

在某些情况下，例如[模拟 Zircon](#emulating-zircon)，您反而必须使用 QEMU。


<!-- 
## FEMU Features {#femu-features}
 -->
## FEMU 特性 {#femu-features}

<!-- 
FEMU looks and behaves like a Fuchsia device, with the exception that no paving is required.
 -->
FEMU 的外观和行为如同一台 Fuchsia 设备，除了不需要铺设（paving）以外。

<!-- 
FEMU features include:

*   **GUI Support:** You can run Fuchsia with the GUI (by default) or without the GUI
    (using the `--headless` argument).
*   **GPU Support:** You can run with the host’s GPU (by default) with full
    [Vulkan](/docs/concepts/graphics/magma/vulkan.md) support, or you can choose
    software rendering using [SwiftShader](https://swiftshader.googlesource.com/SwiftShader/).
*   **Remote Development:** You can use a remote desktop with FEMU, either with Chrome Remote Desktop
     or from the command line using [fx emu-remote](https://fuchsia.dev/reference/tools/fx/cmd/emu-remote)
     command.
 -->
FEMU 特性包括：

*   **图形用户界面（GUI）支持：**您能够选择使用（默认） GUI 或不使用 GUI 来运行 Fuchsia（通过使用 `--headless` 参数）。
*   **GPU 支持：**您能够使用完整支持 [Vulkan](/docs/concepts/graphics/magma/vulkan.md) 的主机 GPU 或通过使用 [SwiftShader](https://swiftshader.googlesource.com/SwiftShader/) 来选择软件渲染。
*   **远程开发：**您能够通过 FEMU 使用远程桌面，无论是 Chrome 远程桌面，还是通过命令行使用 [fx emu-remote](https://fuchsia.dev/reference/tools/fx/cmd/emu-remote) 命令。

<!-- 
To see full list of supported flags:
 -->
要查看受支持标记（flag）的完整列表：

```posix-terminal
fx vdl start --help
```

<!-- 
To configure these features, see the [Set up and start FEMU](/docs/get-started/set_up_femu.md)
page.

If you’re using the Fuchsia IDK, `fvdl` supports the same flags as `fx vdl`
 -->
要配置这些特性，请查看[配置并运行 Fuchsia 模拟器](/docs/get-started/set_up_femu.md)页面。

如果您使用的是 Fuchsia IDK，那么 `fvdl` 支持与 `fx vdl` 相同的标记。

<!-- 
## FEMU limitations {#femu-limitations}
 -->
## FEMU 限制 {#femu-limitations}

<!-- 
### FEMU image and board support {#femu-image-and-board-support}
 -->
### FEMU 镜像和板型支持 {#femu-image-and-board-support}

<!-- 
When setting up FEMU using `fx set`, FEMU only supports the following boards:

*   `qemu-x64`
*   `qemu-arm64`
 -->
在通过使用 `fx set` 设置 FEMU 时，FEMU 仅支持下列板型：

*   `qemu-x64`
*   `qemu-arm64`

<!-- 
When using the Fuchsia IDK to set up FEMU, you are limited to the following pre-built images:

*   `qemu-x64`
*   `workstation.qemu-x64-release`
*   `qemu-arm64`
 -->
在通过使用 Fuchsia IDK 设置 FEMU 时，您只能选择下列预构建镜像：

*   `qemu-x64`
*   `workstation.qemu-x64-release`
*   `qemu-arm64`

<!-- 
Note: ARM64 support (`qemu-arm64`) is very limited and not recommended.
 -->
注意：ARM64 的支持非常有限，不推荐。

<!-- 
### FEMU networking  {#femu-networking}
 -->
### FEMU 网络  {#femu-networking}

<!-- 
On Linux, Fuchsia Emulator should generally be run with the `-N` flag that provides networking through an
emulated NIC. Instructions for setting up networking for FEMU is in
[Setting up the Fuchsia Emulator](/docs/get-started/set_up_femu.md).

Note: Without `-N`, your emulator won't be discoverable using `fx list-devices`. However, you can manually set the SSH address and use `fx` tools to interact with your emulator.
 -->
在 Linux 上，Fuchsia 模拟器应当总体上带 `-N` 标记运行，它可以通过模拟的 NIC 提供网络。为 FEMU 设置网络的操作说明位于[设置 Fuchsia 模拟器](/docs/get-started/set_up_femu.md)中。

注意：无 `-N` 时，您的模拟器将无法通过使用 `fx list-devices` 而被发现。不过，您可以手动设置 SSH 地址并使用 `fx` 工具与您的模拟器交互。

<!-- 
If starting the emulator without `-N` (i.e `fx vdl start`), an available TCP port from the host will be
picked and forwarded to the emulator's SSH port. When the emulator launches successfully, instruction to set `fx` tools with the correct SSH port are printed in the terminal output. 
Then, you can manually set the SSH device:
 -->
如果不带 `-N` 启动模拟器（即 `fx vdl start`），那么将会有一个可用的 TCP 端口被选中，并被传送至模拟器的 SSH 端口。当模拟器成功启动时，通过正确 SSH 端口设置 `fx` 工具的指令将被打印在终端输出中。然后，您就能够手动设置 SSH 设备了：

```posix-terminal
fx set-device 127.0.0.1:{{ '<var>' }}SSH_PORT{{ '</var>' }}
```

<!-- 
To verify `fx` is using the correct port:
 -->
要验证 `fx` 使用了正确端口：

```posix-terminal
fx status
```

<!-- 
You should see the SSH address printed next to `Device name`. To SSH into the emulator:
 -->
您应当查看打印在 `Device name`（设备名称）旁边的 SSH 地址。要通过 SSH 进入模拟器：

```posix-terminal
fx ssh
```

<!-- 
### Emulating Zircon {#emulating-zircon}
 -->
### 模拟 Zircon {#emulating-zircon}

<!-- 
If you only want to emulate Zircon, you must use `fx qemu` instead. Read
[Debugging the Kernel using QEMU](/docs/development/debugging/qemu.md) to
learn more. This is for kernel developers. Most Fuchsia developers do not need
to use this workflow.
 -->
如果您只想模拟 Zircon，那么您反而必须使用 `fx qemu`。请阅读[利用 QEMU 调试内核](/docs/development/debugging/qemu.md)以了解更多内容。这是面向内核开发人员的。大多数 Fuchsia 开发者不需要使用这一工作流程。

<!-- 
## FEMU common usage  {#femu-common-usage}
 -->
## FEMU 常见用法  {#femu-common-usage}

<!-- 
To use FEMU, you must first
[download the Fuchsia source](/docs/get-started/get_fuchsia_source.md)
and [build Fuchsia](/docs/get-started/build_fuchsia.md).

Alternatively, you can use the Fuchsia IDK and use pre-built system images.

Then you can use FEMU to do the following:

*   [Set up and start FEMU](/docs/get-started/set_up_femu.md)
*   [Test components](/docs/development/run/run-test-component.md)
*   [Run end-to-end tests](/docs/development/testing/run_an_end_to_end_test.md)
 -->
要使用 FEMU，您必须首先[下载 Fuchsia 源](/docs/get-started/get_fuchsia_source.md)并[构建 Fuchsia](/docs/get-started/build_fuchsia.md)。

或者，您可以使用 Fuchsia IDK 和预构建系统镜像。

然后您就能够使用 FEMU 处理如下事宜：

*   [设置并运行 FEMU](/docs/get-started/set_up_femu.md)
*   [测试组件](/docs/development/run/run-test-component.md)
*   [运行端到端测试](/docs/development/testing/run_an_end_to_end_test.md)
