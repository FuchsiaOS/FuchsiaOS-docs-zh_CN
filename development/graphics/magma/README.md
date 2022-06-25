Magma: Overview
===============

## Background

Fuchsia is a new open source, microkernel-like operating system from Google.  Drivers do not execute inside the Zircon kernel, instead they are privileged user-space processes.
Drivers are built using a stable [FDF](development/drivers/concepts/fdf.md).

Magma is the gpu driver architecture for Fuchsia. There are two driver components: a gpu-specific library loaded into each application’s address space; and the magma system driver that manages the hardware.

## Hardware requirements

### Vulkan conformant gpu
Magma is designed to support [Vulkan](development/graphics/magma/concepts/vulkan.md), though it could be used to implement OpenGL or other graphics APIs.  Fuchsia intends to support software consuming OpenGL APIs via translation to Vulkan using ANGLE.

### MMU
A memory management unit that allows arbitrary mapping of system memory pages into the GPU address space is needed for DMA to/from non-contiguous buffers.

### Per-client independent address space
For system security it’s important to maintain address space isolation in the gpu domain as well as in the cpu domain.

### Unified memory architecture
This may be relaxed in the future.

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

### Client library driver

Responsibilities:

* Implementing Vulkan 1.0/1.1/1.2 entry points
* Implementing Fuchsia extensions for import and export of external memory and semaphores
* Implementing VK_KHR_display and/or VK_KHR_swapchain for direct display access

Whereas a traditional client driver makes ioctl syscalls to communicate with a kernel driver; magma provides an interface for client drivers to communicate over IPC with the Magma system driver.

Details on the Magma interface are given in [Magma: Design](development/graphics/magma/concepts/design.md).

