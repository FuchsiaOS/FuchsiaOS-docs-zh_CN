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

### Clang/LLVM 工具链

我们使用 Clang 的主干快照并经常更新到新的快找。任何支持 `x86_64` 和 `aarch64` 的近期 Clang 都可以有效构建。 你需要工具链并包括运行库。 我们自然也是使用相同的Clang 构建工具链， 查看[详情](/development/build/toolchain.md')。

### 为工具链设置构建参数

如果你使用预构件的工具链，可以跳过这一步，构建的时候会自动发现它们的。

设置构建参数需指向工具链的位置，

Clang

```sh
fx set bringup.x64 --variant clang --args clang_tool_dir = "<absolute path to>/clang-install/bin/"
```

GCC

```sh
fx set bringup.x64 --variant gcc --args gcc_tool_dir = "<absolute path to>/gcc-install/bin/"
```

注意 `*_tool_dir` 应该是尾部斜线，如果你的路径是相对于 Zircon的，使用空前缀即可。

## 从Zircon 来回复制

当你本地使用 IPv6，可以使用 `fx cp` 从驱动中来回复制。

## 包括附加的用户空间文件

Zircon的构建创建一个引导镜像包含了必须的用户空间组件为了系统的启动（驱动管理器，一些设备驱动等）。内核包含第二个引导镜像，它由QEMU 或 引导加载程序作为 ramdisk 镜像提供。

使用 zbi 工具创建一个引导镜像，它是构建的一部分。它可以为源目录(在这种情况下，包含指定目录及其子目录中的每个文件)或通过一个清单文件组装一个引导映像，该清单文件按文件顺序指定要包含哪些文件。

```none
$BUILDDIR/tools/zbi -o extra.bootfs @/path/to/directory

echo "issue.txt=/etc/issue" > manifest
echo "etc/hosts=/etc/hosts" >> manifest
$BUILDDIR/tools/zbi -o extra.bootfs manifest
```

在引导的 Zircon 系统中，引导中的文件将显示在 /boot 下，因此在上述示例中，"host" 文件将在 /boot/etc/hosts 中。

## 网络启动

网络启动是由2个机制支持的：Gigaboot 和 Zirconboot。Gigaboot 是基于 EFI 引导加载器，而 zirconboot 是一种允许最小 zircon 系统 作为 zircon 引导加载器的机制。

在系统上，通过 EFI 启动（例如 Acer 和 NUC）要么是可行的选项。在另外系统上，zirconboot 可能是唯一的网络启动。

### Gigaboot

[GigaBoot20x6](/src/firmware/gigaboot) 引导启动器说是一种简单的网络启动协议（IPV6 UDP上）不需要指定 host 配置 或者权限即可访问使用。

它通过 IPV6 的优势 链接本地地址和多播，允许被引导的设备通告其引导性，并让主机找到并向它发送一个系统镜像。

如果你有设备（比如是 Broadwell 或者 Skylake Intel NUC）运行 GigaBoot20x6, 第一步，
[创建USB设备](/development/hardware/usb_setup.md).

```none
$BUILDDIR/tools/bootserver $BUILDDIR/zircon.bin

# if you have an extra bootfs image (see above):
$BUILDDIR/tools/bootserver $BUILDDIR/zircon.bin /path/to/extra.bootfs
```

默认情况下，引导服务将继续运行，每次它观察到一个netboot信标，它会发送内核（和 bootfs 如果提供）到该设备上。如果你传递 -1 选项，引导服务器会在成功引导后退出。

### Zirconboot

Zirconboot 是允许 zircon 系统作为引导加载器的机制。

对于zircon它自己来说，Zirconboot 是上面Gigaboot 相似的启动协议。

使用 zirconboot，通过内核命令行将 `netsvc.netboot=true` 参数传到zircon。当zirconboot 启动时，它会尝试从所连接的主机上运行的引导服务器获取并引导到 zircon 系统。

## 网络日志查看

Zircon 默认构建包括网络日志服务，通过本地 IPv6 UDP 链路多播系统日志。

请注意，这是一个可快速破解，协议会在某个点改变。

现在，如果你在QEMU 使用 -N 标示运行 Zircon 或者运行在以太网接口的硬件上（ASIX USB Dongle 或者 NUC 上的Intel Ethernet ）日志监听工具会在本地链接上观察日志广播。

```none
$BUILDDIR/tools/loglistener
```

## 调试

有关 zircon 环境中调试的随机提示，请查阅
[调试](/development/debugging/tips.md).

