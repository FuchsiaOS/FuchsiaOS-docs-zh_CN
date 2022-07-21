
<!--
# Configure and build Fuchsia {#configure-and-build-fuchsia}
This guide provide instructions on how to configure and build Fuchsia
on a host machine.
-->
# 配置和编译 Fuchsia {#configure-and-build-fuchsia}

这篇文档将引导你在主机上如何配置并编译 Fuchsia。
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
1. [准备](#prerequisites)
1. [配置设置项](#set-your-build-configuration)
1. [编译调优（可选）](#speed-up-the-build)
1. [编译](#build-fuchsia)
<!--
## 1. Prerequisites {#prerequisites}
-->
## 1. 准备 {#prerequisites}
<!--
Before you start, check out the following requirements:
-->
在开始之前，先检查是否满足如下要求：
<!--
* [Source code setup](#source-code-setup)
* [Hardware requirements](#hardware-requirements)
-->
* [准备源码](#source-code-setup)
* [硬件要求](#hardware-requirements)
<!--
### Source code setup {#source-code-setup}
-->
### 准备源码 {#source-code-setup}
<!--
Complete the
[Download the Fuchsia source code](/get-started/get_fuchsia_source.md)
guide to download the Fuchsia source code and set up the Fuchsia development
environment on your machine.
-->
根据文档 [下载 Fuchsia 源码](/get-started/get_fuchsia_source.md) 指示下载 Fuchsia 的源码，然后在你的机器上设置开发环境。
<!--
### Hardware requirements {#hardware-requirements}
-->
### 硬件要求 {#hardware-requirements}
<!--
You can build Fuchsia only on a machine with one of the following
host architectures:
-->
Fuchsia 的编译支持如下架构的机器：
<!--
- x86-64 Linux (Debian-based distributions only)
- x86-64 macOS
-->
- x86-64 Linux (只支持 Debian 系列的)
- x86-64 macOS
<!--
Note: Windows and ARM64 are not supported.
-->
注意：Windows 和 ARM64 是不支持的。
<!--
## 2. Set your build configuration {#set-your-build-configuration}
-->
## 2. 配置设置项 {#set-your-build-configuration}
<!--
Fuchsia's build configuration informs the build system which product to
build and which architecture to build for.
-->
Fuchsia 的编译设置项告诉编译系统如何编译，以及编译什么架构的文件。
<!--
To set your Fuchsia build configuration, run the following
[`fx set`][fx-set-reference] command:
-->
设置你的 Fuchsia 编译选项，可以运行 [`fx set`][fx-set-reference] 命令：
```posix-terminal
fx set {{ '<var>' }}PRODUCT{{ '</var>' }}.{{ '<var>' }}BOARD{{ '</var>' }}
```
<!--
Replace the following:
-->
替换以下选项：
<!--
* `PRODUCT`: The Fuchsia product that you want to build; for example, `core` and
  `workstation_eng`.
* `BOARD`: The architecture of the product; for example, `x64` and `qemu-x64`
-->
* `PRODUCT`: 编译的目标文件类型, 比如可以设置为： `core` 和 `workstation_eng`.
* `BOARD`: 编译的可执行文件架构，比如：`x64` 和 `qemu-x64`
<!--
The example command below sets a build configuration to `core.qemu-x64`:
-->
下面的示例设置了编译选项为 `core.qemu-x64`:

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
  * `core` 就是选择了特性最小化的 Fuchsia， 当然包括了常见的网络相关。
  * `qemu-x64` 这个选项设置了编译架构为在开源的虚拟机 （FEMU）中的 x64 架构 [QEMU][qemu]{:.external}.
<!--
On the other hand, the example below sets the build configuration to
`workstation_eng.x64`, which is commonly used to
[install Fuchsia's Workstation product on a device][build-workstation]:
-->
在看另一个编译选项示例，下面这个命令配置了 [在设备中安装 Fuchsia 工作站][build-workstation] 中常用的一个架构, `workstation_eng.x64`:

```posix-terminal
fx set workstation_eng.x64
```
<!--
For more information on the build configuration,
see [Configure a build](/development/build/fx.md#configure-a-build).
-->
如果想查看更详细的编译选项介绍，参考 [配置编译选项](/development/build/fx.md#configure-a-build)
<!--
## 3. Speed up the build (Optional) {#speed-up-the-build}
-->
## 3. 编译调优（可选） {#speed-up-the-build}
<!--
Note: This step is not required to build Fuchsia, but it's recommended
since it can save you a lot of time when you build Fuchsia.
-->
注意：这一步不是必选项，但是建议您设置，因为设置了这一步能节省很多的编译时间。
<!--
To speed up the Fuchsia build, you can use one of the following services:
-->
编译调优的话，需要下面的服务：
<!--
*   [Enable Goma](#enable-goma)
*   [Install ccache](#install-ccache)
-->
*   [使用 Goma](#enable-goma)
*   [安装 ccache](#install-ccache)
<!--
### Enable Goma {#enable-goma}
-->
### 使用 Goma {#enable-goma}
<!--
[Goma](https://chromium.googlesource.com/infra/goma/server/){:.external} is a
distributed compiler service for open source projects such as Chrome, Android
and Fuchsia.
-->
[Goma](https://chromium.googlesource.com/infra/goma/server/){:.external} 是一个分布式编译服务，常用在一些开源项目中，如 Chrome，Android 以及 Fuchsia。
<!--
If you have access to Goma, enable a Goma client on your machine:
-->
如果你能访问 Goma， 使用下面的命令来开启 Goma 客户端：

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
如果不能访问 Goma，但是想加速本地编译时间，那就使用 <code>[ccache](https://ccache.dev/){:.external}</code> 来缓存来进行增量编译。

* {Linux}
<!--
  To use `ccache` on Linux, install the following package:
-->
  在 Linux 中使用 `ccache`，安装如下安装包：
  ```posix-terminal
  sudo apt install ccache
  ```
* {macOS}
<!--
  For macOS, see
  [Using CCache on Mac](https://chromium.googlesource.com/chromium/src.git/+/HEAD/docs/ccache_mac.md){:.external}
  for installation instructions.
-->
  至于 macOS ，参考 [在 Mac 上使用 CCache](https://chromium.googlesource.com/chromium/src.git/+/HEAD/docs/ccache_mac.md){:.external} 中的安装步骤。
<!--
`ccache` is enabled automatically if your `CCACHE_DIR` environment variable
refers to an existing directory.
-->
如果你的 `CCACHE_DIR` 环境变量指向一个存在的路径，`ccache` 就会自动开启。
<!--
To override this default behavior, specify the following flags to `fx set`:
-->
如果想关闭这个特性，可以指定下面的标志为 `fx set`：
<!--
*   Force the use of `ccache` even when other accelerators are available:
-->
*   强制使用 `ccache`，即使其他的加速项可以使用：

    <pre class="prettyprint">
    <code class="devsite-terminal">fx set <var>PRODUCT</var>.<var>BOARD</var> --ccache</code>
    </pre>
<!--
*   Disable the use of `ccache`:
-->
*   关闭 `ccache`：
    <pre class="prettyprint">
    <code class="devsite-terminal">fx set <var>PRODUCT</var>.<var>BOARD</var> --no-ccache</code>
    </pre>
<!--
## 4. Build Fuchsia {#build-fuchsia}
-->
## 4. 编译 {#build-fuchsia}
<!--
The [`fx build`][fx-build-reference] command executes the build to transform
source code into packages and other build artifacts.
-->
[`fx build`][fx-build-reference] 命令可以把源代码编译打包，或者编译成其他的类型。
<!--
To build Fuchsia, run the following command:
-->
使用如下命令编译 Fuchsia：
<!--
Note: Building Fuchsia can take up to 90 minutes.
-->
注意：编译时间一般为 90 分钟。

```posix-terminal
fx build
```
<!--
When you modify source code, run the `fx build` command again to perform an
incremental build, or run the `fx -i build` command to start a watcher, which
automatically builds whenever you update the source code.
-->
当你修改源代码后，要运行命令 `fx build` 来增量编译，或则运行 `fx -i build` 命令来开启一个监视进程，这个监视进程一旦发现源码有更新就会自动编译。
<!--
For more information on building Fuchsia,
see [Execute a build](/development/build/fx.md#execute-a-build).
-->
想要更多编译 Fuchsia 的相信信息，参考 [开始编译](/development/build/fx.md#execute-a-build)。
<!--
## Next steps
-->
## 下一步
<!--
To launch the Fuchsia emulator (FEMU) on your machine, see
[Start the Fuchsia emulator](/get-started/set_up_femu.md).
-->
关于如何在你的主机上打开 Fuchsia 模拟器 （FEMU），参考 [开启 Fuchsia 模拟器](/get-started/set_up_femu.md)。
<!--
However, if you want to run Fuchsia on a hardware device, see
[Install Fuchsia on a device](/development/hardware/README.md) instead.
-->
如果你无论如何都想要在硬件平台上运行 Fuchsia， 可以参考 [在硬件设备中安装 Fuchsia](/development/hardware/README.md)。


<!-- Reference links -->

[build-workstation]: /development/build/build_workstation.md
[fx-set-reference]: https://fuchsia.dev/reference/tools/fx/cmd/set
[fx-build-reference]: https://fuchsia.dev/reference/tools/fx/cmd/build
[qemu]: https://www.qemu.org/
