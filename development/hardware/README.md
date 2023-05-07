<!-- 
# Install Fuchsia on a device
 -->
# 在设备上安装 Fuchsia

<!-- 
The Fuchsia platform can be installed on the following hardware devices:
 -->
Fuchsia 平台能够安装在以下硬件设备上：

<!-- 
- [Intel NUC (Next Unit of Computing) devices][install-fuchsia-on-nuc]
- [Khadas VIM3 board][install-fuchsia-on-vim3]
 -->
- [Intel NUC 迷你电脑][install-fuchsia-on-nuc]
- [Khadas VIM3 开发板][install-fuchsia-on-vim3]

<!-- 
## Architecture support
 -->
## 架构支持

<!-- 
Fuchsia supports two ISAs (Instruction Set Architectures):
 -->
Fuchsia 支持两种指令集（Instruction Set Architecture，ISA）：

<!-- 
* `arm64` - Fuchsia supports `arm64` (also called AArch64) with no restrictions on
  supported microarchitectures.
 -->
* `arm64`——Fuchsia 在受支持的微架构上，对 `arm64`（也称为 AArch64）的支持没有限制。

<!-- 
* `x86-64` - Fuchsia supports `x86-64` (also called IA32e or AMD64), but with some
  restrictions on supported microarchitectures.
 -->
* `x86-64`——Fuchsia 在受支持的微架构上，对 `x86-64`（也称为 IA32e 或 AMD64）的支持有一定限制。

<!-- 
## CPU support
 -->
## CPU 支持

<!-- 
Fuchsia's support for CPUs:
 -->
Fuchsia 对于 CPU 的支持：

<!-- 
* Intel - For Intel CPUs, only Broadwell and later are actively supported and will
  have new features added for them.  Additionally, we will accept patches to keep
  Nehalem and later booting.
 -->
* Intel——对于 Intel CPU，只有 Broadwell 及更高版本是受到主动支持的，并且会添加新的功能。另外，我们将接受补丁，以保持 Nehalem 及更高版本正常引导。

<!-- 
* AMD - AMD CPUs are **not** actively supported (in particular, we have no active testing
  on them), but we will accept patches to ensure correct booting on them.
 -->
* AMD——AMD CPU 是**不**受主动支持的（具体地，我们对其没有进行主动测试），但是我们将接受补丁，以确保其正确引导。

<!-- 
## Table of contents
 -->
## 目录

<!-- 
- [Install Fuchsia on a NUC][install-fuchsia-on-nuc]
- [Install Fuchsia on a NUC using Zedboot (Legacy)][install-fuchsia-on-nuc-legacy]
- [Install Fuchsia on a Khadas VIM3 board][install-fuchsia-on-vim3]
- Create a bootable Fuchsia image:
  - [Install Fuchsia from a USB flash drive][prepare-usb]
- Set up experimental hardware:
  - [Install Fuchsia on Acer Switch Alpha 12][install-fuchsia-on-acer12]
 -->
- [在 NUC 迷你电脑上安装 Fuchsia][install-fuchsia-on-nuc]
- [在 NUC 迷你电脑上使用 Zedboot 安装 Fuchsia（旧版）][install-fuchsia-on-nuc-legacy]
- [在 Khadas VIM3 开发板上安装 Fuchsia][install-fuchsia-on-vim3]
- 创建 Fuchsia 可引导镜像：
  - [从 USB 闪存驱动器安装 Fuchsia][prepare-usb]
- 设置实验性硬件：
  - [在 Acer Switch Alpha 12 上安装 Fuchsia][install-fuchsia-on-acer12]

<!-- Reference links -->

[install-fuchsia-on-nuc]: /development/hardware/intel_nuc.md
[install-fuchsia-on-nuc-legacy]: /development/hardware/intel_nuc_with_zedboot.md
[install-fuchsia-on-vim3]: /development/hardware/khadas-vim3.md
[prepare-usb]: /development/hardware/usb_setup.md
[install-fuchsia-on-acer12]: /development/hardware/acer12.md
