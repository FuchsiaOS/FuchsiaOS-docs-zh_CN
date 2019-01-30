<!--# Putting Fuchsia on a Device-->

<!--这篇文章应该最先阅读-->

# 在设备上安装 Fuchsia

<!--One of the best ways to experience Fuchsia is by running it on actual hardware.
This guide will help you get Fuchsia installed on your device. Fuchsia has good
support for a few different hardware platforms including the Acer Switch 12,
Intel NUC, and Google Pixelbook (not to be confused with the Chromebook Pixel).
The install process is not currently compatible with ARM-based targets. The
Fuchsia install process, called 'paving', requires two machines, the machine on
which you want to run Fuchsia ("target") and the machine on which you build
Fuchsia ("host"). Host and target must be able to communicate over a local area
network. On your host system you will build Fuchsia, create a piece of install
media, and stream a large portion of the system over the network to the target.-->

体验 Fuchsia 的最佳的方式就是在真实的硬件上运行它。本文将帮助你在设备上安装 Fuchsia。Fuchsia对一些不同的硬件平台有很好的支持：Acer Switch 12、Intel NUC 和 Google Pixelbook（不要与 Chromebook Pixel 弄混）。目前安装进程不支持基于 ARM 的硬件。Fuchsia 的安装进程称为 “paving”，安装 Fuchsia 需要两台设备，一台用来安装 Fuchsia 叫做 “目标设备”，另外一台用来构建 Fuchsia 叫做 “主机设备”。主机设备和目标设备需要可以通过局域网进行通信。在主机设备系统中需要构建 Fuchsia，创建一个安装工具，将系统的大部分通过网络安装到目标设备中。

<!--The `fx` command will be used throughout these instructions. If you have fx
mapped into your command path you can follow the instructions verbatim. If you
don't have fx in your path, it can be found at `//scripts/fx` and you'll need
to use the appropriate relative path in the supplied commands. Many of fx
commands are relatively thin wrappers around build actions in GN coupled with
tool invocations. If your use case isn't quite served by what's currently
available there may a few GN targets you can build or some GN templates you can
extend to allow you to build what you need.-->

 指令中将使用 `fx`。如果你已经将 fx 映射到您的命令路径中，那么可以完全按照说明操作。如果没有将 fx 添加进去，再 “//scripts/fx” 路径下你能找到命令工具，你需要在使用的命令中使用相对路径。许多 fx 都是对 GN 中的构建操作以及工具调用的简单包装。如果用例还没有完全满足当前可用的功能，那么你可以构建一些 GN 目标，或者扩展一些 GN 模板来构建所需的功能。



## TL;DR

<!--Read this all before? See the
[quickstart guide](https://fuchsia.googlesource.com/fuchsia/+/master/docs/development/workflows/build_and_pave_quickstart.md)
for a workflow summary.-->

之前都读过吗?有关工作流摘要，请参阅[快速入门指南](https://github.com/FuchsiaOS/FuchsiaOS-docs-zh_CN/blob/master/development/workflows/build_and_pave_quickstart.md)。

<!--## Building-->
## 构建

<!--Detailed instructions for obtaining and building Fuchsia are available from the
[Getting Started](/getting_started.md) guide, but we'll assume here that the
target system is x86-based and that you want to build a complete system. To
configure our build for this we can run `fx set x64` and then build with
`fx full-build`.-->
获取和构建 Fuchsia 的详细说明可以从[入门指南](/getting_started.md)中获得，我们在这里假设目标设备系统是基于 x86 的，并且您希望构建一个完整的系统。我们可以运行 `fx set x64` 来配置构建类型，然后运行 `fx full-build` 来生成。

<!--## Creating install media-->
## 生成安装镜像

<!--To create your install media we recommend using a USB drive since these are
well-supported as boot media by most systems. Note that the install media
creation process **will wipe everything** from the USB drive being used. Insert the
USB drive and then run `fx mkzedboot <device_path>`, which on Linux is
typically something like /dev/sd&lt;X&gt; where X is a letter and on Mac is typically
something like /dev/disk&lt;N&gt; where 'N' is a number. **Be careful not to select
the wrong device**. Once this is done, remove the USB drive.-->

要创建安装镜像，我们建议使用 USB 驱动器，因为大多数系统都支持 USB 驱动器作为引导介质。注意，安装镜像创建过程会**清除**正在使用的 USB 驱动器中的所有内容。插入 USB 驱动器然后运行 `fx mkzedboot <device_path>` ，在 Linux 上 device_path 通常类似于 /dev/sd&lt;X&gt; X 代表字母，在 Mac 上 device_path 类似于 /dev/disk&lt;N&gt; N 代表数字。**注意，不要选错了设备**。安装完成后，移除 USB 驱动器。

<!--## Paving-->
## Fuchsia 安装过程

<!--Now we'll build the artifacts to transfer over the network during the paving
process. What is transferred is dependent on the target device. For UEFI based
systems (like Intel NUC or Acer Switch 12) our output target type is 'efi'. For
ChromeOS-based systems (like Pixelbook) that use vboot-format images, the target
type is 'vboot'. To start the bootserver with the correct image just run `fx pave`.-->

现在，我们将构建构件，以便在 Fuchsia 安装过程中通过网络进行传输。传输的内容取决于目标设备。对于基于 UEFI 的系统（如 Intel NUC 或 Acer Switch 12）的设备，我们的输出类型为 “efi”。对于使用 vboot 格式镜像的基于 Chromeos 系统（如 PixelBook ）的设备，输出类型应为 “vboot”。运行 `fx pave` 启动正确镜像的 bootserver。

<!--Insert the install media into the target device that you want to pave. The target
device's boot settings may need to be changed to boot from the USB device and
this is typically device-specific. For the guides listed below, **only** go
through the steps to set the boot device, don't continue with any instructions on
creating install media.-->

将安装了镜像的 USB 驱动器插入到你想安装 Fuchsia 系统的目标设备中。目标设备的引导设定需要改为从 USB 启动这通常根据设备的不同设置方法不同。对于下面列出的方法，**只是**列出设置引导设备的方法，不包含安装镜像的内容。

* [Acer Switch Alpha 12](https://fuchsia.googlesource.com/zircon/+/master/docs/targets/acer12.md)
* [Intel NUC](https://fuchsia.googlesource.com/zircon/+/master/docs/targets/nuc.md)
* [Google Pixelbook](/development/hardware/pixelbook.md)

<!--Paving should occur automatically after the device is booted into Zedboot from the
USB drive. After the paving process completes, the system should boot into the
Zircon kernel. After paving, the whole system is installed on internal storage. At
this point the USB key can be removed since the system has everything it needs
stored locally. If you plan to re-pave frequently it may be useful to keep the
USB drive inserted so your system boots into Zedboot by default where paving
will happen automatically. After the initial pave on UEFI systems that use
Gigaboot, another option for re-paving is to press 'z' while in Gigaboot to
select Zedboot. For vboot-based systems using the USB drive is currently the
only option for re-paving. In all cases the bootserver needs to have been
started with `fx pave`-->

设备从 USB 驱动器引导到 Zedboot 后会自动的启动 Fuchsia 安装进程。在安装进程完成后，系统会引导到 Zircon 内核。安装完成后整个系统被安装在内存中。此时系统的所有需要的内容都被存储在本地， USB 驱动器可以移除。如果你想要频繁的安装 Fuchsia 那么你可以将 USB 驱动器一直插在目标设备上，那么每次设备都会默认引导到 Zedboot 并自动安装 Fuchsia。在使用 Gigaboot 的 UEFI 系统上进行初始安装后，重新安装的另一个方法是在 Gigaboot 中按 “Z” 键以选择 Zedboot。对基于 vboot 的系统使用 USB 驱动器是当前唯一的选择。无论哪种方法，都需要使用 `fx pave` 启动 bootserver。

<!--## Troubleshooting-->

## 常见问题

<!--In some cases paving may fail because you have a disk layout that is incompatible.
In these cases you will see a message that asks you to run
'install-disk-image wipe'. If it is incompatible because it contains an older
Fuchsia layout put there by installer (vs the paver) you can fix this by killing
the fx pave process on the host, switching to a different console (Alt+F3) on
the target, and running `install-disk-image wipe`. Then reboot the target,
re-run `fx pave` on the host, and the pave should succeed.-->

在某些情况下，安装可能会失败，因为磁盘格式不兼容。在这种情况下，你会看到一条信息 “install-disk-image wipe”。如果是由于有旧的 Fuchsia 安装（相对于直接安装）导致的不兼容，你可以杀死主机设备上的 fx pave 进程，切换目标设备上的控制台（Alt+F3）然后运行 `install-disk-image wipe`。再重启目标设备，在主机设备上重新运行 `fx pave` ，此时应该会成功。

<!--In some cases paving may fail on an Acer with some error indicating "couldn't
find space in gpt". In these cases (as long as you don't want to keep the other
OS, i.e. Windows, parts) run `lsblk` and identify the partition that isn't your
USB (it shouldn't have RE in the columns). Identify the number in the first
column for your partition (likely to be either 000 or 003). Then run
`gpt init /dev/class/block/N` where N is the number previously identified. This
will clear all Windows partitions from the disk. Once this is done, reboot into
zedboot and paving should work.-->

在某些情况下，在 Acer 上安装 Fuchsia 可能会失败，并出现一些错误提示 “couldn't find space in gpt”。在这种情况下（只要你不想保留其他操作系统，即Windows，部分）运行 `lsblk` 并识别不是你的 USB 分区（它不应该在列中有 RE ）。确认分区第一列中的标识编号（可能是 000 或 003 ）。然后运行  `gpt init /dev/class/block/N` N 是之前确定的编号。这会清除磁盘上的所有 Windows 分区。一旦完成，重新引导进入 Zedboot 安装 Fuchsia 就能成功。

<!--## Changing boot target (localboot, netboot, etc) default-->

## 更改默认启动对象（本地引导、网络引导 等等）

<!--For EFI-based systems, it is possible to change the default boot option of the
system paved on the target between local booting and Zedboot for network
booting. By default the system boots locally with a 1-second delay in Gigaboot
to allow you to select a different mode. To change this default to Zedboot,
supply the `always_zedboot` argument when calling your set command, for example
`fx set <target_type> --args "always_zedboot=true"`.-->

对于基于 EFI 系统的设备，可以更改目标设备的默认的引导选项为本地引导或者网络引导的 Zedboot。默认情况下，系统本地启动，Gigaboot 会延迟1秒，允许您选择其他模式。要将此默认值更改为 Zedboot，请在调用set命令时提供 `always_zedboot` 参数，例如 `fx set <target_type> --args "always_zedboot=true"`。

