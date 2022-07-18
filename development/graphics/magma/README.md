<!--

Magma: Overview

-->


Magma: 概述
===============

<!--

## Background

Fuchsia is a new open source, micro-kernel-like operating system from Google.  Drivers do not execute inside the Zircon kernel, instead they are privileged user-space processes.  Drivers are built using a stable [FDF](/docs/concepts/drivers/fdf.md).

Magma is the gpu driver architecture for Fuchsia. There are two driver components: a gpu-specific library loaded into each application’s address space; and the magma system driver that manages the hardware.

-->

## 背景

Fuchsia 是来自 Google 的一种新的开源、类似微内核的操作系统。 
驱动程序不在 Zircon 内核内部执行，而是特权用户空间进程。 
驱动程序是使用稳定的 [FDF](/docs/concepts/drivers/fdf.md) 构建的。

Magma 是 Fuchsia 的 gpu 驱动架构。 
有两个驱动组件：一个加载到每个应用程序地址空间的特定于 gpu 的库； 
以及管理硬件的岩浆系统驱动程序。 

<!--

## Hardware requirements

### Vulkan conformant gpu
Magma is designed to support [Vulkan](vulkan.md), though it could be used to implement OpenGL or other graphics APIs.  Fuchsia intends to support OpenGL via translation to Vulkan using ANGLE.

-->

## 硬件要求

### 符合 Vulkan 的 gpu
Magma 旨在支持 [Vulkan](vulkan.md)，
尽管它可以用于实现 OpenGL 或其他图形 API。 
Fuchsia 意于通过使用 ANGLE 转换为 Vulkan 来支持 OpenGL。 

<!--

### MMU
A memory management unit that allows arbitrary mapping of system memory pages into the GPU address space is needed for DMA to/from non-contiguous buffers.

### Per-client independent address space
For system security it’s important to maintain address space isolation in the gpu domain as well as in the cpu domain.

### Unified memory architecture
This may be relaxed in the future.

-->

### MMU
DMA需要一个允许将系统内存页任意映射到GPU地址空间的内存管理单元，用于进出非连续缓冲区。

### 每个客户端独立的地址空间
为了系统安全，保持 gpu 域和 cpu 域中的地址空间隔离非常重要。

### 统一内存架构
未来可能会放宽。 

<!--

## Architecture

Similar to the direct rendering model on Linux, there are two driver components: a gpu-specific library loaded into each application’s address space; and the magma system driver that manages the hardware.

### Magma system driver

Responsibilities:

* Initializing hardware
* Setting up memory spaces
* Setting up hardware contexts
* Mapping buffers
* Scheduling command buffers
* Handling faults
* Managing power

-->

## 架构

与 Linux 上的直接渲染模型类似，有两个驱动组件：
加载到每个应用程序的地址空间中的 gpu 特定库； 
以及管理硬件的岩浆系统驱动程序。

### Magma 系统驱动

职责：

* 初始化硬件
* 设置内存空间
* 设置硬件上下文
* 映射缓冲区
* 调度命令缓冲区
* 处理故障
* 管理权限

<!--

### Client library driver

Responsibilities:

* Implementing Vulkan 1.0/1.1/1.2 entry points
* Implementing Fuchsia extensions for import and export of external memory and semaphores
* Implementing VK_KHR_display and/or VK_KHR_swapchain for direct display access

Whereas a traditional client driver makes ioctl syscalls to communicate with a kernel driver; magma provides an interface for client drivers to communicate over IPC with the Magma system driver.

Details on the Magma interface are given in [Magma: Design](design.md).

-->

### 客户端库驱动

职责：

* 实现 Vulkan 1.0/1.1/1.2 接入点
* 为外部存储器和信号量的导入和导出实现 Fuchsia 扩展
* 为直接显示访问实现 VK_KHR_display 和/或 VK_KHR_swapchain

而传统的客户端驱动程序使 ioctl 系统调用与内核驱动程序进行通信； 
magma 为客户端驱动程序提供了一个接口，
用以通过 IPC 与 Magma 系统驱动程序进行通信。

Magma 界面的详细信息在 [Magma: Design](design.md) 中给出。 