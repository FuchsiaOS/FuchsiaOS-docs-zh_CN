

<!-- 
# Configure and build Fuchsia {#configure-and-build-fuchsia}

This document describes how to set up and build Fuchsia. 
-->
# 配置和编译 Fuchsia {#configure-and-build-fuchsia}

本文将向你介绍如何配置和编译 Fuchsia。


<!-- 
## Prerequisites

Before you can set up and build Fuchsia, you need to follow the steps in
[get the Fuchsia source code](/docs/get-started/get_fuchsia_source.md)
to download Fuchsia source code and set up your environment variables.
 -->

## 前提

在开始之前，请先参照 
[获取 Fuchsia 源代码](/docs/get-started/get_fuchsia_source.md) 
进行源代码获取及环境变量设置。


<!-- 
## Set build configuration

To set your build configuration for Fuchsia, run the following command:

<pre class="prettyprint">
<code class="devsite-terminal">fx set <var>product</var>.<var>board</var></code>
</pre>
 -->

## 设置编译的配置文件

参照下述命令， 设置 Fuchsia 编译的配置文件：

<pre class="prettyprint">
<code class="devsite-terminal">fx set <var>product</var>.<var>board</var></code>
</pre>


<!-- The `fx set` command takes <var>PRODUCT</var> and <var>BOARD</var> arguments,
which define the
[product and board](/docs/concepts/build_system/boards_and_products.md)
configuration of your build. This configuration informs the build system what
packages to build for your Fuchsia device. -->

命令 `fx set` 使用参数 <var>PRODUCT</var> 和 <var>BOARD</var> 来设置
[product 和 board](/docs/concepts/build_system/boards_and_products.md)
，以此设置编译的配置文件。 此配置文件将告知编译系统， 需要为你的 Fuchsia 设备编译哪些软件包。


<!-- 
For a Fuchsia emulator with the core set of Fuchsia features, the build configuration is:

```posix-terminal
fx set core.qemu-x64
```

In this example:

  * `core` is a product with the minimum feature set for Fuchsia, which includes
     common network capabilities.
  * `qemu-x64` is the board, which refers to the x64 architecture on the
     Fuchsia Emulator (FEMU), which is based on the open source emulator, QEMU.
 -->

对于一个包含 Fuchsia 核心功能集的模拟器来说，配置文件设置如下：

```posix-terminal
fx set core.qemu-x64
```

上面命令中：

  * `core` 是一个产品软件配置， 它为 Fuchsia 提供最精简功能集，其中包括 通用的网络配置。
  * `qemu-x64` 是一个主板硬件配置，, 它提供了 x64 架构的 Fuchsia 模拟器(FEMU)配置, 该模拟器基于开源模拟器 QEMU 开发而来。


<!-- 
For a Fuchsia device with the core set of Fuchsia features, the build configuration is

```posix-terminal
fx set core.x64
```

See [Configure a build](/docs/development/build/fx.md#configure-a-build) for
more product and board options. -->

对于一个包含 Fuchsia 核心功能集的硬件设备来说，配置文件设置如下：

```posix-terminal
fx set core.x64
```

参考 [配置 Fuchsia 编译文件](/docs/development/build/fx.md#configure-a-build) 一文，了解更多关于 产品软件配置、主板硬件配置 的选项。


<!-- 
### Speed up the build {#speed-up-the-build}

Note: This step is optional.

To reduce the time it takes to build Fuchsia, you can do any of the following:

*   [Speed up the build with Goma](#speed-up-the-build-with-goma)
*   [Speed up the build with ccache](#speed-up-the-build-with-ccache)

 -->

### 加速编译过程 {#speed-up-the-build}

说明：这一步是可选的。

参考下面的文章，来减少编译时间：

*   [使用 Goma 加速编译过程](#speed-up-the-build-with-goma)
*   [使用 ccache 加速编译过程](#speed-up-the-build-with-ccache)


<!-- 
[Goma](https://chromium.googlesource.com/infra/goma/server/){:.external} is a
distributed compiler service for open source projects such as Chrome, Android
and Fuchsia. If you have access to Goma, run the following command to enable a
Goma client on your machine:

```posix-terminal
fx goma
```
 -->

#### 使用 Goma 加速编译过程 {#speed-up-the-build-with-goma}

[Goma](https://chromium.googlesource.com/infra/goma/server/){:.external} 是一款很多开源项目都在使用的分布式编译服务，例如 Chromium、Android 以及 Fuchsia。 如果你有 Goma 的访问权限，使用如下命令在你的电脑上启动一个 Goma 客户端：

```posix-terminal
fx goma
```


<!-- 
#### Speed up the build with ccache {#speed-up-the-build-with-ccache}

If you do not have access to Goma, but want to accelerate the Fuchsia build
locally, use <code>[ccache](https://ccache.dev/){:.external}</code> to cache
artifacts from previous builds.
 -->

#### 使用 ccache 加速编译过程 {#speed-up-the-build-with-ccache}

如果你没有 Goma 访问权限， 想要在本地加速编译过程，可以使用 <code>[ccache](https://ccache.dev/){:.external}</code> 来缓存上一次编译的
artifacts。


<!-- 
To use `ccache` on Linux, install the following package:

```posix-terminal
sudo apt-get install ccache
```
 -->

在 Linux 上，安装下面的软件包来启用 `ccache` ：

```posix-terminal
sudo apt-get install ccache
```


<!-- 
For macOS, see
[Using CCache on Mac](https://chromium.googlesource.com/chromium/src.git/+/HEAD/docs/ccache_mac.md){:.external}
for installation instructions.

`ccache` is enabled automatically if your `CCACHE_DIR` environment variable
refers to an existing directory.
 -->

在 macOS 上，参考
[Mac 上使用 CCache](https://chromium.googlesource.com/chromium/src.git/+/HEAD/docs/ccache_mac.md){:.external}
 一文完成安装。

当环境变量 `CCACHE_DIR` 指向一个有效的文件夹时，`ccache` 便会自动启用。


<!-- 
To override the default behavior, pass the following flags to `fx set`:

*   Force use of ccache even if other accelerators are available:

    ```posix-terminal
    fx set core.x64 --ccache
    ```

*   Disable use of ccache:

    ```posix-terminal
    fx set core.x64 --no-ccache
    ```
 -->

为了覆盖默认配置，请参考下面的操作，向 `fx set` 传递参数：

*   在其他加速器被启用时，强制开启 ccache 加速：

    ```posix-terminal
    fx set core.x64 --ccache
    ```

*   禁用 ccache 加速：

    ```posix-terminal
    fx set core.x64 --no-ccache
    ```


<!-- 
## Build Fuchsia

Note: Building Fuchsia can take up to 90 minutes.

To build Fuchsia, run the following command:

```posix-terminal
fx build
```

The `fx build` command executes the build to transform source code into packages
and other build artifacts.
 -->

## 编译 Fuchsia

说明：编译 Fuchsia 可能需要耗时长达 90 分钟。

使用下述命令来编译 Fuchsia：

```posix-terminal
fx build
```

`fx build` 命令用来执行编译操作，目的是把源代码转换成软件包及其他编译产物。


<!-- 
If you modify source code, re-run the `fx build` command to perform an
incremental build, or run the `fx -i build` command to start a watcher, which
automatically builds whenever you update source code.

See [Execute a build](/docs/development/build/fx.md#execute-a-build) for more
information.
 -->

如果你修改了源代码，请再次执行 `fx build` 命令进行增量编译，或者执行 `fx -i build` 命令来创建一个编译监听器，它可以在源代码被更新后，自动执行编译操作。

参见 [执行编译操作](/docs/development/build/fx.md#execute-a-build) 一文获取更多信息。


<!-- 
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
 -->

## 下一步

在模拟器或真机上配置并运行 Fuchsia 

 * 关于模拟器上配置并体验 Fuchsia，请参考
   [配置 Fuchsia 模拟器(FEMU)](/docs/get-started/set_up_femu.md)。
 * 关于真机运行 Fuchsia，请参考 
   [真机安装 Fuchsia](/docs/development/hardware/paving.md) 和
   [快速上手 Fuchsia 编译和部署](/docs/development/build/build_and_pave_quickstart.md)。

在模拟器或真机上部署完 Fuchsia 后，请参考：
 
 *  [探索 Fuchsia](/docs/get-started/explore_fuchsia.md) 以了解 Fuchsia 的系统结构和工作流程。
