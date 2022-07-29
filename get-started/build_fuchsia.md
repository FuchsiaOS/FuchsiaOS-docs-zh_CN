<!--
# Configure and build Fuchsia {#configure-and-build-fuchsia}
This guide provide instructions on how to configure and build Fuchsia
on a host machine.
-->
# 配置和构建 Fuchsia {#configure-and-build-fuchsia}

这篇文档将引导您在主机上配置并构建 Fuchsia。
<!--
The steps are:
-->
步骤如下：
<!--
1. [Prerequisites](#prerequisites).
1. [Set your build configuration](#set-your-build-configuration).
1. [Speed up the build (Optional)](#speed-up-the-build).
1. [Build Fuchsia](#build-fuchsia).
-->
1. [前提条件](#prerequisites)
1. [配置设置项](#set-your-build-configuration)
1. [提升构建速度（可选）](#speed-up-the-build)
1. [构建](#build-fuchsia)
<!--
## 1. Prerequisites {#prerequisites}
-->
## 1. 准备 {#prerequisites}
<!--
Before you start, check out the following requirements:
-->
在开始之前，请先检查是否满足如下要求：
<!--
* [Source code setup](#source-code-setup)
* [Hardware requirements](#hardware-requirements)
-->
* [源代码设置](#source-code-setup)
* [硬件要求](#hardware-requirements)
<!--
### Source code setup {#source-code-setup}
-->
### 源代码设置 {#source-code-setup}
<!--
Complete the
[Download the Fuchsia source code](/get-started/get_fuchsia_source.md)
guide to download the Fuchsia source code and set up the Fuchsia development
environment on your machine.
-->
根据文档[下载 Fuchsia 源代码](/get-started/get_fuchsia_source.md)指示下载 Fuchsia 的源代码，然后在您的计算机上设置开发环境。
<!--
### Hardware requirements {#hardware-requirements}
-->
### 硬件要求 {#hardware-requirements}
<!--
You can build Fuchsia only on a machine with one of the following
host architectures:
-->
您只能在具有下列主机架构之一的计算机上构建 Fuchsia：
<!--
- x86-64 Linux (Debian-based distributions only)
- x86-64 macOS
-->
- x86-64 Linux (只支持基于 Debian 系列的发行版)
- x86-64 macOS
<!--
Note: Windows and ARM64 are not supported.
-->
注意：不支持 Windows 和 ARM64。
<!--
## 2. Set your build configuration {#set-your-build-configuration}
-->
## 2. 设置构建配置 {#set-your-build-configuration}
<!--
Fuchsia's build configuration informs the build system which product to
build and which architecture to build for.
-->
Fuchsia 的构建配置告诉构建系统进行构建的产品，以及构建面向的平台。
<!--
To set your Fuchsia build configuration, run the following
[`fx set`][fx-set-reference] command:
-->
要设置您的 Fuchsia 构建配置，请运行 [`fx set`][fx-set-reference] 命令：
```posix-terminal
fx set {{ '<var>' }}PRODUCT{{ '</var>' }}.{{ '<var>' }}BOARD{{ '</var>' }}
```
<!--
Replace the following:
-->
请替换以下选项：
<!--
* `PRODUCT`: The Fuchsia product that you want to build; for example, `core` and
  `workstation_eng`.
* `BOARD`: The architecture of the product; for example, `x64` and `qemu-x64`
-->
* `PRODUCT`（产品）：您想要构建的 Fuchsia 产品，比如：`core` 和 `workstation_eng`。
* `BOARD`（板型）：编译的可执行文件架构，比如：`x64` 和 `qemu-x64`。
<!--
The example command below sets a build configuration to `core.qemu-x64`:
-->
下面的示例将一项构建配置设置为 `core.qemu-x64`：

```posix-terminal
fx set core.qemu-x64
```
<!--
In this example:
-->
在这个示例中：
<!--
  * `core` is a product with the minimum feature set of Fuchsia, including
     common network capabilities.
  * `qemu-x64` is a board that refers to the x64 architecture of the Fuchsia
    emulator (FEMU), which is based on the open source emulator
    [QEMU][qemu]{:.external}.
-->
  * `core` 是 Fuchsia 具备最小功能集的产品，包括常用的网络功能。
  * `qemu-x64` 是 Fuchsia 模拟器（FEMU）的 x64 架构这一板型，FEMU 基于开源模拟器 [QEMU][qemu]{:.external}。
<!--
On the other hand, the example below sets the build configuration to
`workstation_eng.x64`, which is commonly used to
[install Fuchsia's Workstation product on a device][build-workstation]:
-->
另外，下面的例子将一项编译配置设置为 `workstation_eng.x64`，这常用于[在设备上安装 Fuchsia 工作站][build-workstation]：

```posix-terminal
fx set workstation_eng.x64
```
<!--
For more information on the build configuration,
see [Configure a build](/development/build/fx.md#configure-a-build).
-->
要获取关于构建配置的更多信息，请参阅[配置构建](/development/build/fx.md#configure-a-build)。
<!--
## 3. Speed up the build (Optional) {#speed-up-the-build}
-->
## 3. 提升构建速度（可选） {#speed-up-the-build}
<!--
Note: This step is not required to build Fuchsia, but it's recommended
since it can save you a lot of time when you build Fuchsia.
-->
注意：这一步对于构建 Fuchsia 而言并不是必需的，但是可以在您构建 Fuchsia 时节省很多时间，因此建议您完成该步骤。
<!--
To speed up the Fuchsia build, you can use one of the following services:
-->
要提升 Fuchsia 构建速度，您可以使用下列服务之一：
<!--
*   [Enable Goma](#enable-goma)
*   [Install ccache](#install-ccache)
-->
*   [启用 Goma](#enable-goma)
*   [安装 ccache](#install-ccache)
<!--
### Enable Goma {#enable-goma}
-->
### 启用 Goma {#enable-goma}
<!--
[Goma](https://chromium.googlesource.com/infra/goma/server/){:.external} is a
distributed compiler service for open source projects such as Chrome, Android
and Fuchsia.
-->
[Goma](https://chromium.googlesource.com/infra/goma/server/){:.external} 是一个分布式编译器服务，适用于 Chrome、Android 和 Fuchsia 等开源项目。
<!--
If you have access to Goma, enable a Goma client on your machine:
-->
如果您能访问 Goma， 请在您的计算机上 Goma 客户端：

```posix-terminal
fx goma
```
<!--
### Install ccache {#install-ccache}
-->
### 安装 ccache {#install-ccache}
<!--
If you do not have access to Goma, but want to accelerate the Fuchsia build
locally, use <code>[ccache](https://ccache.dev/){:.external}</code> to cache
artifacts from previous builds.
-->
如果您无法访问 Goma，但想在本地加速构建，则可以使用 <code>[ccache](https://ccache.dev/){:.external}</code> 缓存之前构建的产物。

* {Linux}
<!--
  To use `ccache` on Linux, install the following package:
-->
  要在 Linux 上使用 `ccache`，请安装如下安装包：
  ```posix-terminal
  sudo apt install ccache
  ```
* {macOS}
<!--
  For macOS, see
  [Using CCache on Mac](https://chromium.googlesource.com/chromium/src.git/+/HEAD/docs/ccache_mac.md){:.external}
  for installation instructions.
-->
  对于 macOS ，请参阅[在 Mac 上使用 CCache](https://chromium.googlesource.com/chromium/src.git/+/HEAD/docs/ccache_mac.md){:.external} 中的安装步骤。
<!--
`ccache` is enabled automatically if your `CCACHE_DIR` environment variable
refers to an existing directory.
-->
如果您的 `CCACHE_DIR` 环境变量指向已有路径，那么 `ccache` 就会自动开启。
<!--
To override this default behavior, specify the following flags to `fx set`:
-->
要覆盖此默认行为，请为 `fx set` 指定以下标志：
<!--
*   Force the use of `ccache` even when other accelerators are available:
-->
*   即使其他的加速项可用，也要强制使用 `ccache`，则：

    <pre class="prettyprint">
    <code class="devsite-terminal">fx set <var>PRODUCT</var>.<var>BOARD</var> --ccache</code>
    </pre>
<!--
*   Disable the use of `ccache`:
-->
*   要禁用 `ccache`：
    <pre class="prettyprint">
    <code class="devsite-terminal">fx set <var>PRODUCT</var>.<var>BOARD</var> --no-ccache</code>
    </pre>
<!--
## 4. Build Fuchsia {#build-fuchsia}
-->
## 4. 构建 {#build-fuchsia}
<!--
The [`fx build`][fx-build-reference] command executes the build to transform
source code into packages and other build artifacts.
-->
[`fx build`][fx-build-reference] 命令可以把源代码构建打包，或者构建成其他的类型。
<!--
To build Fuchsia, run the following command:
-->
要构建 Fuchsia，请运行以下命令：
<!--
Note: Building Fuchsia can take up to 90 minutes.
-->
注意：构建时间一般为 90 分钟。

```posix-terminal
fx build
```
<!--
When you modify source code, run the `fx build` command again to perform an
incremental build, or run the `fx -i build` command to start a watcher, which
automatically builds whenever you update the source code.
-->
当您修改源代码后，请再次运行命令 `fx build` 进行增量构建，或者运行 `fx -i build` 命令来开启一个监视进程，这个监视进程会在发现源代码更新时自动构建。
<!--
For more information on building Fuchsia,
see [Execute a build](/development/build/fx.md#execute-a-build).
-->
要获取关于构建 Fuchsia 的更多信息，请参阅[执行构建](/development/build/fx.md#execute-a-build)。
<!--
## Next steps
-->
## 后续步骤
<!--
To launch the Fuchsia emulator (FEMU) on your machine, see
[Start the Fuchsia emulator](/get-started/set_up_femu.md).
-->
要在您的计算机上启动 Fuchsia 模拟器 （FEMU），请参阅[开启 Fuchsia 模拟器](/get-started/set_up_femu.md)。
<!--
However, if you want to run Fuchsia on a hardware device, see
[Install Fuchsia on a device](/development/hardware/README.md) instead.
-->
不过，如果您想在硬件设备上运行 Fuchsia，请参阅[在设备上安装 Fuchsia](/development/hardware/README.md)。


<!-- Reference links -->

[build-workstation]: /development/build/build_workstation.md
[fx-set-reference]: https://fuchsia.dev/reference/tools/fx/cmd/set
[fx-build-reference]: https://fuchsia.dev/reference/tools/fx/cmd/build
[qemu]: https://www.qemu.org/
