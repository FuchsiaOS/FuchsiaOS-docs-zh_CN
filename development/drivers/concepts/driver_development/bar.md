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

Call **pci_map_bar()**
to cause the BAR register to be mapped into the driver host's address space:

```c
zx_status_t pci_map_bar(const pci_protocol_t* pci, uint32_t bar_id,
                        uint32_t cache_policy, void** vaddr, size_t* size,
                        zx_handle_t* out_handle);
```

The first parameter, `pci`, is a pointer to the PCI protocol.
Typically, you obtain this in your **bind()** function through
**device_get_protocol()**.

The second parameter, `bar_id`, is the BAR register number, starting with `0`.

The third parameter, `cache_policy`, determines the caching policy for access,
and can take on the following values:

`cache_policy` value                | Meaning
------------------------------------|---------------------
`ZX_CACHE_POLICY_CACHED`            | use hardware caching
`ZX_CACHE_POLICY_UNCACHED`          | disable caching
`ZX_CACHE_POLICY_UNCACHED_DEVICE`   | disable caching, and treat as device memory
`ZX_CACHE_POLICY_WRITE_COMBINING`   | uncached with write combining

Note that `ZX_CACHE_POLICY_UNCACHED_DEVICE` is architecture dependent
and may in fact be equivalent to `ZX_CACHE_POLICY_UNCACHED` on some architectures.

The next three arguments are return values.
The `vaddr` and `size` return a pointer (and length) of the register region, while
`out_handle` stores the created handle to the
[VMO](/reference/kernel_objects/vm_object.md).

## Reading and writing memory

Once the **pci_map_bar()**
function returns with a valid result, you can access the BAR with simple pointer
operations, for example:

```c
volatile uint32_t* base;
...
zx_status_t rc;
rc = pci_map_bar(dev->pci, 0, ZX_CACHE_POLICY_UNCACHED_DEVICE, &base, &size, &handle);
if (rc == ZX_OK) {
    base[REGISTER_X] = 0x1234;  // configure register X for deep sleep mode
}
```

It's important to declare `base` as `volatile` &mdash; this tells the compiler not to
make any assumptions about the contents of the data that `base` points to.
For example:

```c
int timeout = 1000;
while (timeout-- > 0 && !(base[REGISTER_READY] & READY_BIT)) ;
```

is a typical (bounded) polling loop, intended for short polling sequences.
Without the `volatile` keyword in the declaration, the compiler would have no reason
to believe that the value at `base[REGISTER_READY]` would ever change, so it would
cause it to be read only once.


