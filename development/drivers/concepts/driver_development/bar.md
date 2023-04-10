<!--
    (C) Copyright 2018 The Fuchsia Authors. All rights reserved.
    Use of this source code is governed by a BSD-style license that can be
    found in the LICENSE file.
-->

# Configuration

Caution: This page may contain information that is specific to the legacy
version of the driver framework (DFv1).

Hardware peripherals are attached to the CPU through a bus, such as the PCI bus.

During bootup, the BIOS (or equivalent platform startup software)
discovers all of the peripherals attached to the PCI bus.
Each peripheral is assigned resources (notably interrupt vectors,
and address ranges for configuration registers).

The impact of this is that the actual resources assigned to each peripheral may
be different across reboots.
When the operating system software starts up, it enumerates
the bus and starts drivers for all supported devices.
The drivers then call PCI functions in order to obtain configuration information about
their device(s) so that they can map registers and bind to interrupts.

## Base address register

The Base Address Register (**BAR**) is a configuration register that exists on each
PCI device.
It's where the BIOS stores information about the device, such as the assigned interrupt vector
and addresses of control registers.
Other, device specific information, is stored there as well.

Call **Pci::MapMmio()**
to cause the BAR register to be mapped into the driver host's address space:

```cpp
#include <lib/device-protocol/pci.h>

zx_status_t Pci::MapMmio(uint32_t bar_id, uint32_t cache_policy,
                         std::optional<fdf::MmioBuffer>* mmio);
```

The `ddk::Pci` class is the interface drivers use to talk to the PCI bus.

The first parameter, `bar_id`, is the BAR register number, starting with `0`.

The second parameter, `cache_policy`, determines the caching policy for access,
and can take on the following values:

`cache_policy` value                | Meaning
------------------------------------|---------------------
`ZX_CACHE_POLICY_CACHED`            | use hardware caching
`ZX_CACHE_POLICY_UNCACHED`          | disable caching
`ZX_CACHE_POLICY_UNCACHED_DEVICE`   | disable caching, and treat as device memory
`ZX_CACHE_POLICY_WRITE_COMBINING`   | uncached with write combining

Note that `ZX_CACHE_POLICY_UNCACHED_DEVICE` is architecture dependent
and may in fact be equivalent to `ZX_CACHE_POLICY_UNCACHED` on some architectures.

The last argument is an output parameter for the created buffer.

## Reading and writing memory

Once the **Pci::MapMmio()**
function returns with a valid result, you can access the BAR with through the `MmioBuffer` interface, for example:

```cpp
#include <lib/device-protocol/pci.h>
#include <lib/mmio/mmio-buffer.h>

std::optional<fdf::MmioBuffer> mmio;
zx_status_t status = pci.MapMmio(0, ZX_CACHE_POLICY_UNCACHED_DEVICE, &mmio);
if (status == ZX_OK) {
  mmio.Write32(0x1234, REGISTER_X);  // configure register X for deep sleep mode
}
```
