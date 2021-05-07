# How to write a display driver

So you've decided to bring up a new board. Before you dive into coding, ensure
that you have everything you need by answering these questions:

 * How will I understand how the device and its registers work?

   * This is usually called a "theory of operation". Manufacturers often provide
     datasheets with register definitions, but those references may not explain
     how the device is used in practice.

 * Is there an existing driver for a similar board?

   * Where practical, reuse code for similar boards by refactoring that code and
     amending the bind rules for the driver.

 * Does the device have a fixed display?

   * Some display controllers and panels (output screens) are tightly coupled.
     If this is the case for a new board, you'll need to add support for the
     GPIO, I2C, and other controls as part of the display driver.

## Prerequisites

This guide assumes that you are familiar with driver development for one or more
operating systems. It also assumes that you are familiar with the [Fuchsia
DDK-TL][ddk-tl].

## Programming Languages {#programming-languages}

New drivers must be written in C++. Rust support is planned, but is still highly
experimental.

If an [appropriately licensed][license-policies] driver already exists and is
written in C, it may be acceptable to port it to Fuchsia rather than
implementing a new version in C++. Please contact <graphics-dev@fuchsia.dev>
before making this decision.

## Getting Started {#getting-started}

For platforms without ACPI or a PCI bus, [Modifying board
drivers](board_driver_changes.md) is the first step. This guide assumes that the
board driver is ready and that the display driver is codenamed `fancy`. All code
for the new driver will live in `src/graphics/display/drivers/fancy-display/`.

To begin, create:

 * A minimal implementation of [DisplayControllerImpl][dcimpl]
 * A [bind program][driver-binding]
 * A build recipe for the `DisplayControllerImpl` and the bind program

### Add the driver to the build {#adding-to-build}

1. Create the build recipe in a file named `BUILD.gn`

```python
# Copyright 2021 The Fuchsia Authors. All rights reserved.
# Use of this source code is governed by a BSD-style license that can be
# found in the LICENSE file.

import("//build/bind/bind.gni")
import("//build/config/fuchsia/rules.gni")

bind_rules("fancy-display-bind") {
  rules = "fancy-display.bind"
  output = "fancy-display-bind.h"
  tests = "bind_tests.json"
  deps = [
    "//src/devices/bind/board_maker_company.platform",
  ]
}

# Factored out so that it can be used in tests.
source_set("common") {
  public_deps = [
    ":fancy-display-bind",
  ]
  sources = [
    "fancy-display.cc",
  ]
}

driver_module("fancy-display") {
  sources = []
  deps = [
    ":common",
    "//src/devices/lib/driver",
  ]
}
```

1. Add `//src/graphics/display/drivers/fancy-display` as a dependency for the
board(s) that you are using as test products. For example, if your device is
part of a [Khadas VIM2 board][vim2-board], modify `//boards/vim2.gni` by adding
your driver to the `_common_bootfs_deps` list.

<!-- TODO: describe this in more detail, including how to build the `core` image
for those products. -->

### Choose devices to drive

Now that you have a build recipe, you can move on to creating the [bind
rules][driver-binding], which the driver manager uses to decide whether a driver
can be used with a device.

1. In `src/graphics/display/drivers/fancy-display`, create `fancy-display.bind`:

```
// Copyright 2021 The Fuchsia Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

using fuchsia.pci;

fuchsia.BIND_PROTOCOL == fuchsia.pci.BIND_PROTOCOL.DEVICE;
fuchsia.BIND_PCI_VID == fuchsia.pci.BIND_PCI_VID.PLANK_HW_INC;
accept fuchsia.BIND_PCI_DID {
  // Fancy
  0x0100,
  // Fancy+ series
  0x0120,
  0x0121,
}
```

For PC devices, the [intel-i915 bind rules][intel-bind] are a good example. For
fixed-hardware SoCs, see the [VIM2 rules][vim2-bind].


#### Minimal driver

Finally, add a bare bones driver that simply constructs a new object every
time it successfully binds to a device. Later, you can use the datasheet to get
the device to actually do something.

In `src/graphics/display/drivers/fancy-display`, create `fancy-display.cc`:

```c++
#include <ddktl/device.h>
#include <fuchsia/hardware/display/controller/cpp/banjo.h>

namespace fancy_display {

class Device;
using DeviceType = ddk::Device<Device>

// A Device exposes a single display controller for use by the core display
// driver in src/graphics/display/drivers/display.
//
// This object is constructed once for each device that matches this
// driver's bind rules.
class Device : public DeviceType {
 public:
  explicit Device(zx_device_t* parent) : DeviceType(parent) {}

  // If Bind() returns an error, the driver won't claim the device.
  zx_status_t Bind() { return ZX_OK };

  // Functionality needed by the common display driver core.
  void DisplayControllerImplSetDisplayControllerInterface(
      const display_controller_interface_protocol* interface) {}

  zx_status_t DisplayControllerImplImportVmoImage(
      image_t* image, zx::vmo vmo, size_t offset) {
    return ZX_ERR_NOT_SUPPORTED;
  }

  zx_status_t DisplayControllerImplImportImage(
      image_t* image, zx_unowned_handle_t sysmem_handle, uint32_t index) {
    return ZX_ERR_NOT_SUPPORTED;
  }

  void DisplayControllerImplReleaseImage(image_t* image) {}

  uint32_t DisplayControllerImplCheckConfiguration(
      const display_config_t** display_config, size_t display_count,
      uint32_t** layer_cfg_result, size_t* layer_cfg_result_count) {}

  void DisplayControllerImplApplyConfiguration(
      const display_config_t** display_config, size_t display_count) {}

  void DisplayControllerImplSetEld(
      uint64_t display_id,
      const uint8_t* raw_eld_list,
      size_t raw_eld_count) {}

  zx_status_t DisplayControllerImplGetSysmemConnection(
      zx::channel connection) {
    return ZX_ERR_NOT_SUPPORTED;
  }

  zx_status_t DisplayControllerImplSetBufferCollectionConstraints(
      const image_t* config, uint32_t collection) {
    return ZX_ERR_NOT_SUPPORTED;
  }

  zx_status_t DisplayControllerImplGetSingleBufferFramebuffer(
      zx::vmo* out_vmo, uint32_t* out_stride) {
    return ZX_ERR_NOT_SUPPORTED;
  }

};

}  // namespace fancy_display

// Main bind function called from dev manager.
zx_status_t fancy_display_bind(void* ctx, zx_device_t* parent) {
    fbl::AllocChecker alloc_checker;
    auto dev = fbl::make_unique_checked<fancy_display::Device>(
        &alloc_checker, parent);
    if (!alloc_checker.check()) {
        return ZX_ERR_NO_MEMORY;
    }
    auto status = dev->Bind();
    if (status == ZX_OK) {
      // The driver/device manager now owns this memory.
      __UNUSED auto ptr = dev.release();
    }
    return status;
}

// zx_driver_ops_t is the ABI between driver modules and the device manager.
// This lambda is used so that drivers can be rebuilt without compiler
// warnings if/when new fields are added to the struct.
static zx_driver_ops_t fancy_display_ops = [](){
    zx_driver_ops_t ops;
    ops.version = DRIVER_OPS_VERSION;
    ops.bind = fancy_display_bind;
    return ops;
}();

// ZIRCON_DRIVER marks the compiled driver as compatible with the zircon
// 0.1 driver ABI.
ZIRCON_DRIVER(fancy_display, fancy_display_ops, "zircon", "0.1");
```

Display drivers are required to implement the `DisplayControllerImpl`
[protocol][dcimpl], which exposes hardware layers and implements vsync
notifications. A shared [display-core][display-core] driver wraps all the
device-specific drivers on the system and handles client multiplexing, resource
tracking, fences, etc.

### Implementation tips

The driver decides when and how a configuration passed to `ApplyConfiguration`
takes effect. In order to avoid [tearing][tearing]{:external}, drivers should
apply new settings just after vsync.

Most devices generate interrupts for vsync events. The easiest way to
ensure timely vsync notifications is to spawn a separate thread just for
servicing that interrupt. *Even if no images are displayed*, your driver must
call `OnDisplayVsync` for every vsync.

#### Controllers with bootloader support

If the display is active on boot, e.g. a panel is turned on and an image is
displayed, then you can get basic functionality in your driver quickly. Read
bootloader logs and/or source to find:

 * The physical address of the framebuffer
 * The registers used to program that address
 * The pixel dimensions of the image, e.g. 800x600
 * The pixel format of the image, e.g. RGB888, NV12, or BGRA8888

Then:

 1. Modify the driver to report a display with the format constraints.
 2. Record the physical address of any imported image in `image->handle`.
 3. When `ApplyConfig` is called, re-program the registers.

If you do not yet know how to observe vsyncs, you can fake it with a thread that
calls `OnDisplayVsync` at 60Hz.

#### Controllers that boot "dark"

There is no one right way to bring up a display controller that lacks even a
basic bootloader driver. In most cases, your roadmap will be:

 1. Power up the device.
 2. Initialize clocks.
 3. Discover attached displays.
 4. Program PHYs for a compatible mode.
 5. Program layouts (framebuffer addrs, etc.) on vsync to avoid [tearing][tearing]{:external}.
 6. Integrate with [Sysmem][sysmem].

<!--xrefs-->
[dcimpl]: /sdk/banjo/fuchsia.hardware.display.controller/display-controller.fidl
[ddk-tl]: /docs/concepts/drivers/driver_development/using-ddktl.md
[display-core]: /src/graphics/display/drivers/display/
[driver-binding]: /docs/concepts/drivers/device_driver_model/driver-binding.md
[intel-bind]: /src/graphics/display/drivers/intel-i915/intel-i915.bind
[license-policies]: /docs/contribute/governance/policy/open-source-licensing-policies.md
[sysmem]: https://fuchsia.dev/reference/fidl/fuchsia.sysmem
[tearing]: https://en.wikipedia.org/wiki/Screen_tearing
[vim2-bind]: /src/graphics/display/drivers/vim-display/vim-display.bind
[vim2-board]: /boards/vim2.gni
