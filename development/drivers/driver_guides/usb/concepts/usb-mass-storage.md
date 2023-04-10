<!--
    (C) Copyright 2019 The Fuchsia Authors. All rights reserved.
    Use of this source code is governed by a BSD-style license that can be
    found in the LICENSE file.
-->

# USB mass storage driver

Caution: This page may contain information that is specific to the legacy
version of the driver framework (DFv1).

The USB mass storage driver is used to communicate with mass storage devices
such as flash drives, external hard drives, and other types of removable media
connected through USB. The USB mass storage driver is split into two parts.

* [SCSI block device](/src/devices/block/lib/scsi/disk.cc)
Uses the [block](/sdk/fidl/fuchsia.hardware.block.driver/block.fidl) protocol.
* [Core](/src/devices/block/drivers/usb-mass-storage/usb-mass-storage.cc) device
Interfaces with the USB stack.

## SCSI block device

The block device implements
[BlockImplQuery](/sdk/fidl/fuchsia.hardware.block.driver/block.fidl#95), and
[BlockImplQueue](/sdk/fidl/fuchsia.hardware.block.driver/block.fidl#102). It
supports read, write, and flush operations. If power is lost between a write
operation and a flush operation, changes written to a USB mass storage device may
not be persisted to the device. The driver has no mechanism to inform drivers
higher up in the stack of when a write has actually been written to physical
media. For the purposes of USB mass storage, a write is considered complete
when the device acknowledges the write.

# Core device

The core device serves as the interface between the block device and the USB
stack. The core accepts requests from the block device, and converts them into
USB requests, which are eventually sent to hardware through the USB stack. For
each request, the following steps are performed:

*   Request is added to a queue.
*   Request is picked up by the worker thread.
*   SCSI command stored in the request is sent to the device.
*   Request status is read back from the device.
*   Completion callback is invoked, informing the block device layer that the
    request has been completed.

Some USB mass storage devices may have multiple block devices such as an array
of disks. In this case, the core driver creates one block device per disk.
