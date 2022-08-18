<!-- 
# Start the Fuchsia emulator
 -->
# 启动 Fuchsia 模拟器

<!-- 
This guide provides instructions on how to set up and launch the
Fuchsia emulator (FEMU) on your machine.
 -->
本指南提供关于如何安装和启动 Fuchsia 模拟器（FEMU）在您的设备上的说明。

<!-- 
The steps are:
 -->
步骤如下：

<!-- 
1. [Prerequisites](#prerequisites).
1. [Build Fuchsia for FEMU](#build-fuchsia-for-femu).
1. [Enable VM acceleration (Optional)](#enable-vm-acceleration).
1. [Start FEMU](#start-femu).
1. [Discover FEMU](#discover-femu).
 -->
1. [前提条件](#prerequisites)。
1. [为 FEMU 构建 Fuchsia](#build-fuchsia-for-femu)。
1. [启用 VM 加速（可选）](#enable-vm-acceleration)。
1. [启动 FEMU](#start-femu)。
1. [发现 FEMU](#discover-femu)。

<!-- 
## 1. Prerequisites {#prerequisites}
 -->
## 1. 前提条件 {#prerequisites}

<!-- 
Running FEMU requires that you've completed the following guides:
 -->
运行 FEMU 需要您完成以下向导：

<!-- 
 * [Download the Fuchsia source code][get-fuchsia-source]
 * [Configure and build Fuchsia][build-fuchsia]
 -->
 * [下载 Fuchsia 源代码][get-fuchsia-source]
 * [配置和构建 Fuchsia][build-fuchsia]

<!-- 
## 2. Build Fuchsia for FEMU {#build-fuchsia-for-femu}
 -->
## 2. 为 FEMU 构建 Fuchsia {#build-fuchsia-for-femu}

<!-- 
To run FEMU, you first need to build a Fuchsia system image that supports
the emulator environment. This guide uses `qemu-x64` for the board
and `workstation_eng` for the product as an example.
 -->
要运行 FEMU，您首先要构建一个支持模拟器环境的 Fuchsia 系统镜像。本指南使用 `qemu-x64` 
板型和 `workstation_eng` 产品作为一个例子。

<!-- 
To build a FEMU Fuchsia image, do the following:
 -->
要构建一个 FEMU 的 Fuchsia 镜像，请执行以下操作：

<!-- 
1. Set the Fuchsia build configuration:
 -->
1. 设置 Fuchsia 构建配置：

   ```posix-terminal
   fx set workstation_eng.qemu-x64 --release
   ```

<!-- 
2. Build Fuchsia:
 -->
2. 构建 Fuchsia：

   ```posix-terminal
   fx build
   ```

<!-- 
For more information on supported boards and products, see the
[Fuchsia emulator (FEMU)][femu-overview] overview page.
 -->
要获取关于支持的板型和产品的更多信息，请参阅 [Fuchsia 模拟器（FEMU）][femu-overview] 概述页面。

<!-- 
## 3. Enable VM acceleration (Optional) {#enable-vm-acceleration}
 -->
## 3. 启用 VM 加速（可选） {#enable-vm-acceleration}

<!-- 
(**Linux only**) Most Linux machines support VM acceleration through
KVM, which greatly improves the performance and usability of the emulator.
 -->
（**仅限Linux**）大部分 Linux 设备支持通过 KVM 进行 VM 加速，大大提高了模拟器的性能和可用性。

<!-- 
If KVM is available on your machine, update your group permission to
enable KVM.
 -->
如果您的设备可以使用 KVM，请更新您的组权限来启用 KVM。

* {Linux}

<!-- 
  To enable KVM on your machine, do the following:
 -->
  要在您的设备上启用 KVM，请执行以下操作：

<!-- 
  Note: You only need to do this once per machine.
   -->
  注意：您只需要在每台设备上执行一次。

<!-- 
  1.  Add yourself to the `kvm` group on your machine:
 -->
  1.  在您的设备上添加您自己到 `kvm` 组：

      ```posix-terminal
      sudo usermod -a -G kvm ${USER}
      ```

<!-- 
  1.  Log out of all desktop sessions to your machine and then log in again.
 -->
  1.  注销与您设备的所有桌面会话，然后重新登录。

<!-- 
  1.  To verify that KVM is configured correctly, run the following command:
 -->
  1.  要验证 KVM 是否配置正确，请运行以下命令：

      ```posix-terminal
      if [[ -r /dev/kvm ]] && grep '^flags' /proc/cpuinfo | grep -qE 'vmx|svm'; then echo 'KVM is working'; else echo 'KVM not working'; fi
      ```

<!-- 
      Verify that this command prints the following line:
 -->
      验证此命令是否打印以下行：

      ```none {:.devsite-disable-click-to-copy}
      KVM is working
      ```

<!-- 
      If you see `KVM not working`, you may need to reboot your machine for
      the permission change to take effect.
 -->
      如果您看到 `KVM not working`，您可能要重启您的设备使权限修改生效。

* {macOS}

<!-- 
  No additional setup is required for macOS.
 -->
  macOS 无需额外的设置。

<!-- 
  Instead of KVM, the Fuchsia emulator on macOS uses the
  [Hypervisor framework][hypervisor-framework]{: .external}.
 -->
  在 macOS 上 Fuchsia 模拟器使用的是 [Hypervisor 框架][hypervisor-framework]{: .external}，而不是 KVM。

<!-- 
## 4. Start FEMU {#start-femu}
 -->
## 4. 启动 FEMU {#start-femu}


<!-- 
### Start the package server
 -->
### 启动包服务

<!-- 
Prior to starting the emulator, start the package server.
 -->
在启动模拟器之前，启动包服务。

<!-- 
To start the the package server, run the following command:
 -->
要启动包服务，请运行以下命令：

  ```posix-terminal
  fx serve
  ```
<!-- 
Note: Alternatively you can background the `fx serve` process.
 -->
注意：或者，您可以将 `fx serve` 进程置为后台。

<!-- 
### Start the emulator
 -->
### 启动模拟器

<!-- 
To start the emulator on your Linux machine, do the following:
 -->
要在您的 Linux 设备启动模拟器，请执行以下操作：

* {Linux}

<!-- 
  1. Configure the upscript by running the following command:
 -->
  1. 通过运行以下命令配置 upscript：

<!-- 
      Note: If your machine is behind a firewall, you may need to apply some additional
      configuration to allow the emulator to access the network. This is typically
      accomplished by running an "upscript", which sets up the interfaces and firewall
      access rules for the current process. If you're on a corporate network, check
      with your internal networking team to see if they have an existing upscript
      for you to use.
 -->
      注意：如果您的设备使用了防火墙，您可能需要应用一些额外的配置来允许模拟器访问网络。这通常通过运行“upscript”来完成，该脚本为当前进程设置接口和防火墙访问规则。如果您位于企业网络，请联系您的内部网络团队来查看他们是否有现行的 upscript 供您使用。
<!-- 
      If you're not behind a firewall, there's still some configuration needed to
      enable tun/tap networking. The example upscript
      at <code>{{ '<var>' }}FUCHSIA_ROOT{{ '</var>' }}/scripts/start-unsecure-internet.sh</code>
      should work for the majority of non-corporate users.
 -->
      如果您没有使用防火墙，仍然需要一些配置来启用 tun/tap 网络。upscript 示例 <code>{{ '<var>' }}FUCHSIA_ROOT{{ '</var>' }}/scripts/start-unsecure-internet.sh</code> 应该适用于大多数非企业网络。


      ```posix-terminal
      ffx config set emu.upscript {{ '<var>' }}FUCHSIA_ROOT{{ '</var>' }}/scripts/start-unsecure-internet.sh
      ```
<!-- 
      * `start-unsecure-internet.sh` is an example upscript.
      * `FUCHSIA_ROOT` is the path to your Fuchsia directory.
 -->
      * `start-unsecure-internet.sh` 是一个 upscript 示例。
      * `FUCHSIA_ROOT` 是您的 Fuchsia 目录的路径。

<!-- 
  1. Start FEMU
 -->
  1. 启动 FEMU

<!-- 
      1. To start the emulator with access to external networks, run the
         following command:
 -->
      1. 要启动可以访问外部网络的模拟器，请运行以下命令：

          ```posix-terminal
          ffx emu start --net tap
          ```

<!-- 
          * `--net` specifies the networking mode for the emulator. `--net tap`
          attaches to a Tun/Tap interface.
 -->
          * `--net` 指定模拟器的网络模式。`--net tap` 附加到 Tun/Tap 接口。

<!-- 
      1. To start the emulator without access to external networks, run
         the following command:
 -->
      1. 要启动不需要访问外部网络的模拟器，请运行以下命令：

          ```posix-terminal
          ffx emu start --net none
          ```

<!-- 
    Starting the emulator opens a new window with the
    title **Fuchsia Emulator**. When the emulator is finished booting, you are
    returned to the command prompt, and the emulator runs in the background.
 -->
    启动模拟器会打开一个标题为 **Fuchsia Emulator** 的新窗口，当模拟器完成启动后，您会返回到命令提示符，然后模拟器在后台运行。

* {macOS}

<!-- 
  To start FEMU on macOS, do the following:
 -->
  要在 macOS 上启动 FEMU，请执行以下操作：

<!-- 
  1. Start FEMU:
 -->
  1. 启动 FEMU：

     ```posix-terminal
     ffx emu start
     ```

<!-- 
     If you launch FEMU for the first time on your macOS (including after a reboot),
     a window pops up asking if you want to allow the process `aemu` to run on your
     machine. Click **Allow**.
 -->
     如果在您的 maxOS 上第一次启动 FEMU（包括重启之后），则会弹出一个窗口询问您是否要允许 `aemu` 进程在您的设备上运行。请点击 **Allow**。

<!-- 
     This command opens a new window with the title **Fuchsia Emulator**.
     When the emulator is finished booting, you are returned to the command
     prompt, and the emulator runs in the background.
 -->
     该命令打开一个标题为 **Fuchsia Emulator** 的新窗口。当模拟器启动完成后，您会返回到命令提示符，然后模拟器在后台运行。

<!-- 
  2. (Optional) If you need to specify the launched Fuchsia emulator, you can
     run the `fx set-device` command in the same terminal:
 -->
  2. （可选的）如果您需要明确指定已启动的 Fuchsia 模拟器，您可以运行 `fx set-device` 命令在同一终端。

     ```posix-terminal
     fx set-device {{ '<var>' }}NAME{{ '</var>' }}
     ```

<!-- 
     Replace the following:
 -->
     替换以下内容：

<!-- 
     * `NAME`: Use the desired value from the `ffx emu list` or `ffx target list`
       command's output. `fuchsia-emulator` is the default value.
 -->
     * `NAME`：使用 `ffx emu list` 或 `ffx target list` 命令输出的期望值。`fuchsia-emulator` 是默认值。


<!-- 
## 5. Discover FEMU {#discover-femu}
 -->
## 5. 发现 FEMU {#discover-femu}

<!-- 
To discover the Fuchsia emulator as a running Fuchsia device, run the
following command:
 -->
要发现 Fuchsia 模拟器作为正在运行的 Fuchsia 设备，请运行以下命令：

```posix-terminal
ffx target list
```

<!-- 
This command prints output similar to the following:
 -->
该命令的打印输出类似于以下内容：

```none {:.devsite-disable-click-to-copy}
$ ffx target list
NAME                      SERIAL       TYPE                    STATE      ADDRS/IP                            RCS
fuchsia-emulator    <unknown>    workstation_eng.qemu-x64    Product    [fe80::866a:a5ea:cd9e:69f6%qemu]    N
```

<!-- 
`fuchsia-emulator` is the default node name of the Fuchsia emulator.
 -->
`fuchsia-emulator` 是 Fuchsia 模拟器的默认节点名称。

<!-- 
The output of `ffx target list` is influenced by the `--net` option in the
following ways:
 -->
`ffx target list` 的输出受到以下 `--net` 选项的影响：

<!-- 
   * `--net none` disables networking, which causes the device to not be
   discoverable when running `ffx target list`.
 -->
   * `--net none` 禁用网络，这导致设备在运行 `ffx target list` 时无法被发现。
<!-- 
   * `--net tap` and `--net user` allow the device to be discoverable
   when running `ffx target list`.
 -->
   * `--net tap` 和 `--net user` 允许设备在运行 `ffx target list` 时能被发现。


<!-- 
## Next steps
 -->
## 下一步

<!-- 
To learn more about Fuchsia device commands and Fuchsia workflows, see
[Explore Fuchsia][explore-fuchsia].
 -->
要学习更多关于 Fuchsia 设备命令和 Fuchsia 工作流，请参阅[探索 Fuchsia][explore-fuchsia]。

<!-- 
## Appendices
 -->
## 附录

<!-- 
This section provides additional FEMU options.
 -->
本章节提供额外的 FEMU 选项。

<!-- 
### See all available flags
 -->
### 查看所有可用的标记（flag）

<!-- 
To see a [full list][ffx-emu-reference] of the emulator's supported flags, run the
following command:
 -->
要查看模拟器所支持标记的完整列表，请运行以下命令：

```posix-terminal
ffx emu start --help
```

<!-- 
### Run FEMU without GUI support
 -->
### 在没有 GUI 支持下运行 FEMU

<!-- 
If you don't need graphics or working under the remote workflow,
you can run FEMU in headless mode:
 -->
如果您不需要图形界面或者在远程工作流下工作，您可以在无头模式（headless mode）下运行 FEMU：

```posix-terminal
ffx emu start --headless
```

<!-- 
### Specify GPU used by FEMU
 -->
### 指定 FEMU 使用的 GPU

<!-- 
By default, the FEMU launcher attempts to detect if the host has a GPU that can be used for
graphics rendering. If you need to explicitly set the rendering type, you can use
the `--gpu` flag with the following options:
 -->
默认情况下，FEMU 启动器尝试检测主机是否有可用于图形渲染的 GPU。如果您需要明确设置渲染类型，您可以使用带有以下选项的 `--gpu` 标记:

<!-- 
<table><tbody>
  <tr>
   <th>GPU Emulation method</th>
   <th>Explanation</th>
   <th>Flag</th>
  </tr>
  <tr>
   <td>Hardware (host GPU)</td>
   <td>Uses the host machine's GPU directly to perform GPU processing.</td>
   <td><code>ffx emu start --gpu host</code></td>
  </tr>
  <tr>
   <td>Software (host CPU)</td>
   <td>Uses the host machine's CPU to simulate GPU processing.</td>
   <td><code>ffx emu start --gpu guest</code></td>
  </tr>
  <tr>
   <td>SwiftShader</td>
   <td>Uses SwiftShader libraries to simulate GPU processing.</td>
   <td><code>ffx emu start --gpu swiftshader_indirect</code></td>
  </tr>
  <tr>
   <td>Auto</td>
   <td>Resolves to <code>host</code> if there is a hardware GPU available or
       <code>swiftshader_indirect</code> if there isn't a hardware GPU available.
       <code>auto</code> is the current default.</td>
   <td><code>ffx emu start --gpu auto</code></td>
  </tr>
</tbody></table>
 -->
<table><tbody>
  <tr>
   <th>GPU 仿真方式</th>
   <th>说明</th>
   <th>标记</th>
  </tr>
  <tr>
   <td>硬件（主机 GPU）</td>
   <td>直接使用主机设备的 GPU 进行 GPU 处理。</td>
   <td><code>ffx emu start --gpu host</code></td>
  </tr>
  <tr>
   <td>软件（主机 CPU）</td>
   <td>使用主机设备的 CPU 来模拟 GPU 处理。</td>
   <td><code>ffx emu start --gpu guest</code></td>
  </tr>
  <tr>
   <td>SwiftShader</td>
   <td>使用 SwiftShader 库来模拟 GPU 处理。</td>
   <td><code>ffx emu start --gpu swiftshader_indirect</code></td>
  </tr>
  <tr>
   <td>自动</td>
   <td>如果有可用的 GPU 硬件，则解析为 <code>host</code>，如果没有可用的 GPU 硬件，则解析为 <code>swiftshader_indirect</code>。<code>auto</code> 是当前默认值。</td>
   <td><code>ffx emu start --gpu auto</code></td>
  </tr>
</tbody></table>

<!-- 
### Reboot FEMU {#reboot-femu}
 -->
### 重启 FEMU {#reboot-femu}

<!-- 
To reboot FEMU, run the following `ffx` command:
 -->
要重启 FEMU，请运行以下 `ffx` 命令：

```posix-terminal
ffx target reboot
```

<!-- 
### Stop FEMU {#stop-femu}
 -->
### 停止 FEMU {#stop-femu}

<!-- 
To stop FEMU, run the following `ffx` command:
 -->
要停止 FEMU，请运行以下 `ffx` 命令：

```posix-terminal
ffx emu stop
```

<!-- 
### Configure IPv6 network {#configure-ipv6-network}
 -->
### 配置 IPv6 网络 {#configure-ipv6-network}

<!-- 
This section provides instructions on how to configure an IPv6 network
for FEMU on Linux machine using [TUN/TAP][tuntap]{: .external}.
 -->
本节提供有关如何在 Linux 设备上使用 [TUN/TAP][tuntap]{: .external} 给 FEMU 配置 IPv6 网络的说明。

* {Linux}

<!-- 
  Note: This has to be completed once per machine.
 -->
  注意：这必须在每台设备上完成一次。

<!-- 
  To enable networking in FEMU using
  [tap networking][tap-networking]{: .external}, do the following:
 -->
  要在 FEMU 中使用 [网络分流][tap-networking]{: .external} 启用网络，请执行以下操作：

<!-- 
  1. Set up `tuntap`:
 -->
  1. 设置 `tuntap`：

     ```posix-terminal
     sudo ip tuntap add dev qemu mode tap user $USER
     ```

<!-- 
  1. Enable the network for `qemu`:
 -->
  1. 为 `qemu` 启用网络：

     ```posix-terminal
     sudo ip link set qemu up
     ```

* {macOS}

<!-- 
  No additional IPv6 network setup is required for macOS.
 -->
  macOS 不需要额外的 IPv6 网络设置。

<!-- 
  [User Networking (SLIRP)][slirp]{: .external} is the default network setup
  for FEMU on macOS – while this setup does not support Fuchsia device
  discovery, you can still use `fx` tools (for example,`fx ssh`) to
  interact with your FEMU instance.
 -->
  [用户网络（SLIRP）][slirp]{: .external}是 macOS 上 FEMU 的默认网络设置——虽然此设置不支持 Fuchsia 设备发现，但您仍然可以使用 `fx` 工具（例如，`fx ssh`）与您的 FEMU 实例进行交互。

<!-- Reference links -->

[get-fuchsia-source]: /get-started/get_fuchsia_source.md
[build-fuchsia]: /get-started/build_fuchsia.md
[femu-overview]: /development/build/emulator.md
[ffx-emu-reference]: https://fuchsia.dev/reference/tools/sdk/ffx#emu
[hypervisor-framework]: https://developer.apple.com/documentation/hypervisor
[explore-fuchsia]: /get-started/explore_fuchsia.md
[swiftshader]: https://swiftshader.googlesource.com/SwiftShader/
[tuntap]: https://en.wikipedia.org/wiki/TUN/TAP
[tap-networking]: https://wiki.qemu.org/Documentation/Networking#Tap
[slirp]: https://wiki.qemu.org/Documentation/Networking#User_Networking_.28SLIRP.29
