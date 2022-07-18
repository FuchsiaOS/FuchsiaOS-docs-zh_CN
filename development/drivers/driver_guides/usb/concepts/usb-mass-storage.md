<!--
    (C) Copyright 2019 The Fuchsia Authors. All rights reserved.
    Use of this source code is governed by a BSD-style license that can be
    found in the LICENSE file.
-->

<!--
# USB mass storage driver
-->

# USB大容量存储设备驱动

<!--
The USB mass storage driver is used to communicate with mass storage devices
such as flash drives, external hard drives, and other types of removable media
connected through USB. The USB mass storage driver is split into two parts.

* [Block device interface](/src/devices/block/drivers/usb-mass-storage/block.cc)
Uses the [block](/sdk/banjo/fuchsia.hardware.block/block.fidl) protocol.
* [Core](/src/devices/block/drivers/usb-mass-storage/usb-mass-storage.cc) device
Interfaces with the USB stack.
-->

USB大容量存储设备驱动是用来与大容量存储设备通信的，例如闪存盘、外置硬盘以及其他类型的
通过USB连接的可移除媒体等。USB大容量存储设备驱动分为两个部分。

* 使用[block](/sdk/banjo/fuchsia.hardware.block/block.fidl)协议的
[块设备接口](/src/devices/block/drivers/usb-mass-storage/block.cc)。
* 带有USB栈的[核心](/src/devices/block/drivers/usb-mass-storage/usb-mass-storage.cc) 设备接口。

<!--
## Block device
-->

## 块设备

<!--
The block device implements
[BlockImplQuery](/sdk/banjo/fuchsia.hardware.block/block.fidl#95), and
[BlockImplQueue](/sdk/banjo/fuchsia.hardware.block/block.fidl#102). It
supports read and write operations, but flush is not implemented. If power is
lost after a write operation, changes written to a USB mass storage device may
not be persisted to the device.The driver has no mechanism to inform drivers
higher up in the stack of when a write has actually been written to physical
media. For the purposes of USB mass storage, a write is considered complete
when the device acknowledges the write.
-->

块设备实现[BlockImplQuery](/sdk/banjo/fuchsia.hardware.block/block.fidl#95)和
[BlockImplQueue](/sdk/banjo/fuchsia.hardware.block/block.fidl#102)。它支持读写操作，
但是刷新（flush）操作没有被实现。如果在写操作后掉电，向USB大容量存储设备写入的内容
可能不会保存。当物理媒体真正完成写操作时，驱动程序没有办法向栈中更高层级的驱动进行
通知。对于USB大容量存储设备而言，当设备认可了此次写入时，这次写入才被认为完成了。

<!--
# Core device
-->

# 核心设备

<!--
The core device serves as the interface between the block device and the USB
stack. The core accepts requests from the block device, and converts them into
USB requests, which are eventually sent to hardware through the USB stack. For
each request, the following steps are performed:

*   Request is added to a queue.
*   Request is picked up by the worker thread.
*   Request is encoded as a SCSI command and sent to the device.
*   Request status is read back from the device.
*   Completion callback is invoked, informing the block device layer that the
    request has been completed.

Some USB mass storage devices may have multiple block devices such as an array
of disks. In this case, the core driver creates one block device per disk.
-->

核心设备在块设备和USB栈之间作为接口。核心设备从块设备接收请求，并将这些请求
转化成USB请求，这些USB请求最终通过USB栈被发送至硬件。对于每个请求，下列步骤
会被执行：

*   请求被添加入队列中。
*   请求被工作线程（worker thread）接手处理。
*   请求被编码成SCSI命令，并发送至设备。
*   请求状态被设备读取。
*   完成回调（Completion callback）被调用，通知块设备层（block device layer）请求已经完成。

一些USB大容量存储设备或许有多个块设备，例如磁盘阵列（an array of disks）。这种情况下，
核心驱动为每个磁盘创造一个块设备。

