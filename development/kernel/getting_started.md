# 开始 Zircon 吧

## 拉取Zircon 代码

注意：Fuchsia 代码包含了Zircon，请查看 Fuchsia的 [开始](/docs/get-started/README.md) 文档

该指南已假定Fuchsia项目已经拉取并设置好 $FUCHSIA_DIR` 和 `fx 的配置

## 使用默认的工具链构建 Zircon

`fx ` 命令已使用配置封装好几个工具，用于构建和使用 Fuchsia。`fx set` 命令常用语制定版本（product）和系统架构（board）。比如：要设置 Zircon 到 `arm64` 上。

```sh
fx set bringup.arm64
```

上面 Fuchsia 使用的 [版本](/concepts/build_system/boards_and_products.md#products) 概念是构建目标的一个集合。[bringup 版本](/concepts/build_system/boards_and_products.md#bringup-product) 是最简的特性集合而成的最小版本。

下面这个命令就是打印出全部的版本的定义：

```sh
fx list-products
```

下面这个命令是打印出全部的系统架构的的定义：

```sh
fx list-boards
```

然后运行下面命令，执行构建：

```sh
fx build
```

该构建的结果会保存在 `$FUCHSIA_DIR/out/default`

## 指定目标机器的工具链

Fuchsia 默认是使用 `clang` 作为工具链，可以通过 `fx set` 中 `variants` 参数来设置 `gcc`：

```sh
fx set bringup.x64 --variant gcc
```

你可以通过 variant 来开启 asan。

## 构建 Zircon 全部类型

你可以通过 `fx multi` 和一个说明构建文件来构建全部类型。每个类型的输出都可以在`$FUCHSIA_DIR/out/<product>.<board>.variant` 找到。 多个构建例子在 <code>[bringup-cq](/tools/devshell/lib/multi-specs/bringup-cq)</code>, 它近似于 CQ测试的构建。

请在构建前自己得明确全部系统架构的全部目的。

## QEMU

你可以选择跳过这段，如果你仅在物理机上测试的话，但模拟器可以便捷的快速本地测试是非常值得拥有的。

查看 [QEMU](/development/debugging/qemu.md) 获取更多的QEMU信息用于构建和使用 zircon。

## 构建工具链（可选）

如果预设的工具链不能使用，你可以通过上层的来源来构建自己的工具链。

- Clang 工具链默认是用于构建 Zircon 的 或者通过 `variants = [ "clang" ]` or `variants = [ "asan" ]` 来指定。
- Clang 是默认的构建工具链，但 C++14 以上都可以胜任。
- GCC 工具链也是可用的。

无论构建工具链哪一种都需要你知道如何构建 Zircon。

### GCC 工具链

我们使用 GNU `binutils` 2.30[^1] 和 GCC 8.2, 配置使用
`--enable-initfini-array --enable-gold`, 和 `--target=x86_64-elf
--enable-targets=x86_64-pep` for x86-64 或者对于 arm64 `--target=aarch64-elf` 

对于 `binutils`, 我们推荐 `--enable-deterministic-archives`但切换不是用于构建的必要条件。

GCC 在 `make` 命令行中带上 `MAKEOVERRIDES=USE_GCC_STDINT=provide` ，我们应该明确 `stdint.h` 只有一个，而不是使用 `#include_next` 包含其他的 `stdint.h` 在其他地方。

C 和 C++ 语言和 `libgcc` 是支持，所以你可以使用各种配置切换取消不同的事情并GCC构建，它本身运行得快使用更少的内存，例如： `--enable-languages=c,c++ --disable-libstdcxx
--disable-libssp --disable-libquadmath`。 查看GCC文档了解更多。

你可能需要不同的配置切换到不同的预设置去构建你指定的系统。查看 GNU 文档

[^1]: The `binutils` 2.30 发布版验证，使用`make check` 检测故障
    `aarch64-elf` and `x86_64-elf` 配置。这是在 `binutils-2_30-branch` 的分支做了更完善的修复,这就是我真正要构建的。但 2.30 版本本身就可以在 Zircon 的构建上运行良好，它只是在测试案例中有些虚报故障。

### Clang/LLVM Toolchain

We use a trunk snapshot of Clang and update to new snapshots frequently. Any
build of recent-enough Clang with support for `x86_64` and `aarch64` compiled in
should work. You'll need a toolchain that also includes the runtime libraries.
We normally also use the same build of Clang for the host as well as for the
`*-fuchsia` targets. See [here](/docs/development/build/toolchain.md) for
details on how we build Clang.

### Set up build arguments for toolchains

If you're using the prebuilt toolchains, you can skip this step, since the build
will find them automatically.

Set the build argument that points to where you installed the toolchains:

```sh
fx set bringup.x64 --variant clang --args clang_tool_dir = "<absolute path to>/clang-install/bin/"
```

or for GCC:

```sh
fx set bringup.x64 --variant gcc --args gcc_tool_dir = "<absolute path to>/gcc-install/bin/"
```

Note that `*_tool_dir` should have a trailing slash. If the `clang` or `gcc` in
your `PATH` works for Zircon, you can just use empty prefixes.

## Copying files to and from Zircon

With local link IPv6 configured you can use `fx cp` to copy files to and from
the device.

## Including Additional Userspace Files

The Zircon build creates a bootfs image containing necessary userspace
components for the system to boot (the device manager, some device drivers,
etc). The kernel is capable of including a second bootfs image, which is provided
by QEMU or the bootloader as a ramdisk image.

To create such a bootfs image, use the zbi tool that's generated as part of the
build. It can assemble a bootfs image for either source directories (in which
case every file in the specified directory and its subdirectories are included)
or via a manifest file that specifies, on a file-by-file basis, which files to
include.

```none
$BUILDDIR/tools/zbi -o extra.bootfs @/path/to/directory

echo "issue.txt=/etc/issue" > manifest
echo "etc/hosts=/etc/hosts" >> manifest
$BUILDDIR/tools/zbi -o extra.bootfs manifest
```

On the booted Zircon system, the files in the bootfs will appear under /boot, so
in the above manifest example, the "hosts" file would appear at /boot/etc/hosts.

## Network Booting

Network booting is supported via two mechanisms: Gigaboot and Zirconboot.
Gigaboot is an EFI based bootloader whereas zirconboot is a mechanism that
allows a minimal zircon system to serve as a bootloader for zircon.

On systems that boot via EFI (such as Acer and NUC), either option is viable. On
other systems, zirconboot may be the only option for network booting.

### Via Gigaboot

The [GigaBoot20x6](/src/firmware/gigaboot) bootloader speaks a simple network
boot protocol (over IPV6 UDP), which does not require any special host
configuration or privileged access to use.

It does this by taking advantage of IPV6 Link Local Addressing and Multicast,
allowing the device being booted to advertise its bootability and the host to
find it and send a system image to it.

If you have a device (for example a Broadwell or Skylake Intel NUC) running
GigaBoot20x6, first
[create a USB drive](/docs/development/hardware/usb_setup.md).

```none
$BUILDDIR/tools/bootserver $BUILDDIR/zircon.bin

# if you have an extra bootfs image (see above):
$BUILDDIR/tools/bootserver $BUILDDIR/zircon.bin /path/to/extra.bootfs
```

By default bootserver will continue to run and every time it observes a netboot
beacon it will send the kernel (and bootfs if provided) to that device. If you
pass the -1 option, bootserver will exit after a successful boot instead.

### Via Zirconboot

Zirconboot is a mechanism that allows a zircon system to serve as the bootloader
for zircon itself. Zirconboot speaks the same boot protocol as Gigaboot
described above.

To use zirconboot, pass the `netsvc.netboot=true` argument to zircon via the
kernel command line. When zirconboot starts, it will attempt to fetch and boot
into a zircon system from a bootserver running on the attached host.

## Network Log Viewing

The default build of Zircon includes a network log service that multicasts the
system log over the link local IPv6 UDP. Please note that this is a quick hack
and the protocol will certainly change at some point.

For now, if you're running Zircon on QEMU with the -N flag or running on
hardware with a supported ethernet interface (ASIX USB Dongle or Intel Ethernet
on NUC), the loglistener tool will observe logs broadcast over the local link:

```none
$BUILDDIR/tools/loglistener
```

## Debugging

For random tips on debugging in the zircon environment see
[debugging](/docs/development/debugging/tips.md).

## Contribute changes

*   See [CONTRIBUTING.md](/CONTRIBUTING.md#contributing-patches-to-zircon)
