
<!-- # Set up and start the Fuchsia emulator (FEMU)

This document describes how to set up and run the Fuchsia emulator (FEMU), including networking
and GPU support setup. -->
# 配置并运行 Fuchsia 模拟器(FEMU)

本文将向你介绍如何配置并运行 Fuchsia 模拟器(FEMU)，其中包括网络和 GPU 支持的配置。

<!-- ## Prerequisites

To run FEMU, you must have:

 * [Checked out the Fuchsia source and set up some environment variables](/docs/get-started/get_fuchsia_source.md)
 * [Configured and built Fuchsia](/docs/get-started/build_fuchsia.md) -->
## 前提条件

运行 FEMU 之前，请先阅读下述文章：

 * [下载 Fuchsia 源代码并设置环境变量](/docs/get-started/get_fuchsia_source.md)
 * [配置和编译 Fuchsia](/docs/get-started/build_fuchsia.md)

<!-- 
### Building Fuchsia for FEMU

Before you can use FEMU, you need to build Fuchsia using `fx set`, 
specifying a qemu board and supported product. This example uses
`qemu-x64` for the board and `workstation` for the product:
 -->
### 编译 Fuchsia 以适配 FEMU

使用 FEMU 之前，你需要通过设置 `fx set` 命令的参数，来指定适用于 qemu 的主板硬件配置和产品软件配置。
下面的示例中，我们选择
`qemu-x64` 为主板硬件配置， `workstation` 为产品软件配置：

<!-- 
<pre class="prettyprint">
<code class="devsite-terminal">fx set workstation.qemu-x64 --release [--with=...]</code>
<code class="devsite-terminal">fx build</code>
</pre>

Note: More information on supported boards and products is in the
[Fuchsia emulator overview](/docs/concepts/emulator/index.md).
 -->

<pre class="prettyprint">
<code class="devsite-terminal">fx set workstation.qemu-x64 --release [--with=...]</code>
<code class="devsite-terminal">fx build</code>
</pre>

说明：参考
[Fuchsia 模拟器概述](/docs/concepts/emulator/index.md)一文获取更多配置信息。

<!-- 
## Configure network

For Fuchsia's ephemeral software to work with FEMU, you need to configure
an IPv6 network.

  * [Linux configuration](#linux-config)
  * [macOS configuration](#mac-config)

   -->
## 配置网络信息

为了使 Fuchsia 的临时软件能够在 FEMU 中使用，您需要进行 IPv6 网络配置。

  * [Linux 配置](#linux-config)
  * [macOS 配置](#mac-config)


<!-- 
### Linux {#linux-config}

To enable networking in FEMU using [tap networking](https://wiki.qemu.org/Documentation/Networking#Tap), run the following commands:

<pre class="prettyprint">
<code class="devsite-terminal">sudo ip tuntap add dev qemu mode tap user $USER</code>
<code class="devsite-terminal">sudo ip link set qemu up</code>
</pre>
 -->
### Linux {#linux-config}

为了在 FEMU 中启用 [分流网络模式](https://wiki.qemu.org/Documentation/Networking#Tap)，请执行下述命令：

<pre class="prettyprint">
<code class="devsite-terminal">sudo ip tuntap add dev qemu mode tap user $USER</code>
<code class="devsite-terminal">sudo ip link set qemu up</code>
</pre>

<!-- 
### macOS {#mac-config}

[User Networking (SLIRP)](https://wiki.qemu.org/Documentation/Networking#User_Networking_.28SLIRP.29){: .external} is the default networking set up for FEMU on macOS. This networking set up does not support Fuchsia device discovery.
 -->
### macOS {#mac-config}

[User 网络模式(SLIRP)](https://wiki.qemu.org/Documentation/Networking#User_Networking_.28SLIRP.29){: .external} 是 FEMU 在 macOS 上的默认网络模式， 该模式不支持 Fuchsia 设备发现功能。

<!-- 
## Start FEMU

The most common way to run FEMU is with networking enabled, using the following commands.
 -->
## 启动 FEMU

常用的 FEMU 启动方式一般会同时启用网络功能，命令如下： 

<!-- 
### Linux {#linux-start-femu}

To support device discovery without access to external networks.

```posix-terminal
fx vdl start -N
```
 -->
### Linux {#linux-start-femu}

若要启用设备发现功能，但不访问外部网络，请执行：

```posix-terminal
fx vdl start -N
```

<!-- To get access to external networks:

{% dynamic if user.is_googler %}
Note: Command will differ depending on the type of machines you use.

* {Corp}

  To use FEMU on a corp machine, see [go/fuchsia-emulator-corp](http://go/fuchsia-emulator-corp).

* {Non-Corp}

  Note: `FUCHSIA_ROOT` is the path to the Fuchsia checkout on your local machine (ex: `~/fuchsia`).

  ```posix-terminal
  fx vdl start -N -u {{ '<var>' }}FUCHSIA_ROOT{{ '</var>' }}/scripts/start-unsecure-internet.sh
  ```

{% dynamic else %}

Note: `FUCHSIA_ROOT` is the path to the Fuchsia checkout on your local machine (ex: `~/fuchsia`).

```posix-terminal
fx vdl start -N -u {{ '<var>' }}FUCHSIA_ROOT{{ '</var>' }}/scripts/start-unsecure-internet.sh
```
{% dynamic endif %}
 -->

若想访问外部网络：

{% dynamic if user.is_googler %}
说明：根据你的电脑的不同， 命令将会有所不同

* {Corp}

  想要在 corp 机器上运行 FEMU，请参见 [go/fuchsia-emulator-corp](http://go/fuchsia-emulator-corp).

* {Non-Corp}

  说明：参数 `FUCHSIA_ROOT` 指向的是 Fuchsia 源代码在你电脑上的保存位置 (例如: `~/fuchsia`)。

  ```posix-terminal
  fx vdl start -N -u {{ '<var>' }}FUCHSIA_ROOT{{ '</var>' }}/scripts/start-unsecure-internet.sh
  ```

{% dynamic else %}

说明：参数 `FUCHSIA_ROOT` 指向的是 Fuchsia 源代码在你电脑上的保存位置 (例如: `~/fuchsia`)。

```posix-terminal
fx vdl start -N -u {{ '<var>' }}FUCHSIA_ROOT{{ '</var>' }}/scripts/start-unsecure-internet.sh
```
{% dynamic endif %}

<!-- 
Once you run the command, a separate window opens with the title "Fuchsia Emulator". After
the Fuchsia emulator launches successfully, the terminal starts with the SSH console. You
can run shell commands in this window, just like you would on a Fuchsia device.
 -->

命令运行后， 会打开一个名为"Fuchsia Emulator"的新窗口。当模拟器启动成功后，终端会打开一个 SSH 会话。在该窗口中，你可以执行作用于这个 Fuchsia 设备的 Shell 命令。

<!-- 
### macOS {#mac-start-femu}

On macOS, Fuchsia device discovery does not work. However, you can still use `fx` tools such as `fx ssh`.


```posix-terminal
fx vdl start
```

From the output, take note of the instruction on running `fx set-device`, you will need it for the steps below. -->
### macOS {#mac-start-femu}

在 macOS 上， Fuchsia 设备发现功能不可用。 但你仍可以正常使用 `fx` 工具，比如 `fx ssh` 命令。


```posix-terminal
fx vdl start
```

从命令的输出中，记下关于 `fx set-device` 的说明，我们下面会用到它。

<!-- 
Note: When you launch FEMU for the first time on your Mac machine after starting up (ex: after a reboot),
a window pops up asking if you want to allow the process “aemu” to run on your machine.
Click “allow”.

Run `fx set-device` to specify the launched Fuchsia emulator SSH port. For `SSH_PORT`, use the value that the `fx vdl start --host-gpu` command outputted.


```posix-terminal
fx set-device 127.0.0.1:{{ '<var>' }}SSH_PORT{{ '</var>' }}
```
 -->

说明：每次 Mac 电脑启动后，首次运行 FEMU，都会弹出窗口，询问你是否允许“aemu”在你的电脑上运行，点击“允许”即可。

执行下述 `fx set-device` 命令来给已经启动的 Fuchsia 模拟器指定 SSH 端口，命令中，参数`SSH_PORT`由上述 `fx vdl start --host-gpu` 命令的输出中获取。 



```posix-terminal
fx set-device 127.0.0.1:{{ '<var>' }}SSH_PORT{{ '</var>' }}
```

<!-- 
## Additional FEMU options

### Input options

By default FEMU uses multi-touch input. You can add the argument `--pointing-device mouse`
for mouse cursor input instead.

```posix-terminal
fx vdl start --pointing-device mouse
```
 -->
## 其他 FEMU 选项

### 输入 选项

FEMU 默认使用多点触屏输入。你可以使用参数 `--pointing-device mouse` 来指定使用鼠标作为输入源。

```posix-terminal
fx vdl start --pointing-device mouse
```
<!-- 
### Run FEMU without GUI support

If you don't need graphics or working under the remote workflow, you can run FEMU in headless mode:

```posix-terminal
fx vdl start --headless
```
 -->
### 无图形界面运行 FEMU 

如果你不需要图形界面，又或者使用远程工作流来工作，则可以以 headless 模式运行 FEMU：

```posix-terminal
fx vdl start --headless
```

<!-- 
### Specify GPU used by FEMU

By default, FEMU launcher uses software rendering using [SwiftShader](https://swiftshader.googlesource.com/SwiftShader/). 
To force FEMU to use a specific graphics emulation method, use the parameters `--host-gpu` or `--software-gpu` to the `fx vdl start` command.
 -->
### 让 FEMU 使用指定的 GPU

默认情况下， FEMU 启动器会使用 [SwiftShader](https://swiftshader.googlesource.com/SwiftShader/) 来进行软件渲染。
要强制 FEMU 使用指定的图形模拟方法，可以在命令 `fx vdl start` 中使用参数 `--host-gpu` 或者 `--software-gpu` 。

<!-- 
These are the valid commands and options:

<table><tbody>
  <tr>
   <th>GPU Emulation method</th>
   <th>Explanation</th>
   <th><code>fx vdl start</code> flag</th>
  </tr>
  <tr>
   <td>Hardware (host GPU)</td>
   <td>Uses the host machine’s GPU directly to perform GPU processing.</td>
   <td><code>fx vdl start --host-gpu</code></td>
  </tr>
  <tr>
   <td>Software (host CPU)</td>
   <td>Uses the host machine’s CPU to simulate GPU processing.</td>
   <td><code>fx vdl start --software-gpu</code></td>
  </tr>
</tbody></table>
 -->
可用的选项及参数如下：

<table><tbody>
  <tr>
   <th>GPU 模拟方法</th>
   <th>解释</th>
   <th>用法</th>
  </tr>
  <tr>
   <td>硬件模拟 (使用宿主机GPU)</td>
   <td>直接使用宿主机 GPU 处理图形计算</td>
   <td><code>fx vdl start --host-gpu</code></td>
  </tr>
  <tr>
   <td>软件模拟 (使用宿主机 CPU)</td>
   <td>使用宿主机 CPU 来模拟 GPU 图形计算</td>
   <td><code>fx vdl start --software-gpu</code></td>
  </tr>
</tbody></table>


<!-- 
### Supported hardware for graphics acceleration {#supported-hardware}

FEMU currently supports a limited set of GPUs on macOS and Linux for
hardware graphics acceleration. FEMU uses a software renderer fallback for unsupported GPUs.

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
 -->
### 受支持的图形加速硬件列表 {#supported-hardware}

在 macOS 和 Linux 上，FEMU 支持使用部分 GPU 进行硬件图形加速。对于不支持的 GPU，FEMU 则使用软件渲染器来实现加速功能。

<table>
  <tbody>
    <tr>
      <th>操作系统</th>
      <th>GPU 制造商</th>
      <th>系统/驱动版本</th>
    </tr>
    <tr>
      <td>Linux</td>
      <td>Nvidia Quadro</td>
      <td>Nvidia Linux Drivers <a href="https://www.nvidia.com/download/driverResults.aspx/160175/en-us">440.100</a>+</td>
    </tr>
    <tr>
      <td>macOS</td>
      <td><a href="https://support.apple.com/en-us/HT204349#intelhd">Intel HD Graphics</a></td>
      <td>macOS 10.15+</td>
    </tr>
    <tr>
      <td>macOS</td>
      <td>AMD Radeon Pro</td>
      <td>macOS 10.15+</td>
    </tr>
  </tbody>
</table>

<!-- 
## Exit FEMU

To exit FEMU, run `dm poweroff` in the FEMU terminal.

## Next steps

 *  To learn more about how FEMU works, see the
    [Fuchsia emulator (FEMU) overview](/docs/concepts/emulator/index.md).
 *  To learn more about Fuchsia device commands and Fuchsia workflows, see
    [Explore Fuchsia](/docs/get-started/explore_fuchsia.md).

 -->

## 退出 FEMU

想要退出 FEMU, 请在 FEMU 终端执行命令 `dm poweroff` 。

## 下一步

 *  了解 FEMU 工作原理，参见
    [Fuchsia 模拟器(FEMU)概述](/docs/concepts/emulator/index.md).
 *  了解 Fuchsia 设备指令及 Fuchsia 工作流，参见
    [探索 Fuchsia](/docs/get-started/explore_fuchsia.md).

