{% set rfcid = "RFC-0070" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!--
*** This should begin with an H2 element (for example, ## Summary).
-->

## Summary

In userspace the PCI Bus driver needs to be able to disable legacy level
triggered interrupts until a device interrupt has been serviced to prevent
the same IRQ from continually waking the Bus Driver's IRQ thread spuriously.
To achieve this we need a way for a device driver to notify the Bus driver
that it is ready to service a new interrupt and re-enable its legacy
interrupt generation.

## Motivation

Most modern PCI devices operate via Messaged Signaled Interrupts (MSIs)
controlled through optional MSI or MSI-X capabilities in the PCI
configuration space. These interrupts are specific to a given device and are
managed by a direct mapping between a kernel handle and an
`MsiInterruptDispatcher`. Each MSI is provided to only a single device and
can be treated as a standard system interrupt from a driver perspective.

However, PCI legacy interrupts operate via a set of interrupt lines shared
across all PCI devices and are detailed in system firmware tables such as
ACPI as defined by the PCI Firmware Specification. These interrupts are level
triggered and active low by specification. When an interrupt is triggered it
is the responsibility of system software to determine which device is
responsible for the interrupt so it can be serviced and release the line. In
the Kernel PCI Bus Driver (kPCI) this is handled by all legacy interrupts
having a shared interrupt handler registered with all PciInterruptDispatchers
in the kernel. This handler then determines which device has generated the
interrupt and signals the appropriate interrupt object. The device's ability
to generate interrupts is then disabled. The next time that driver waits on
the dispatcher the Unmask hook will both unmask and re-enable the device's
legacy interrupt generation capability.

With the Userspace PCI Bus Driver (uPCI) all this machinery has been moved to
userspace. uPCI itself now operates a low overhead IRQ worker which
determines the device responsible and signals a virtual interrupt that the
device's driver interacts with. However, since the interrupts are level based
the interrupt will keep firing if we lack a driver for the device, or if a
given driver fails to service the interrupt properly. But if uPCI disables a
device's interrupt so that a driver can handle it then we have no existing
method for a PCI device driver to re-enable the interrupt. There is currently
no way to notify the uPCI bus driver that a device driver has called
`zx_interrupt_wait` or `zx_port_wait` on the virtual interrupt provided, so the
bus driver doesn't know when a device's interrupt should be re-enabled.

## Design

We need to design for two different usages of interrupts in PCI drivers.

1. Drivers which know they are PCI drivers and directly call the `PciProtocol` methods.
2. Drivers which use the interrupt in a manner which prevents usage of `PciProtocol` methods.

When a legacy interrupt configured using `PCI_IRQ_MODE_LEGACY` is fired for a
shared line the Bus driver is responsible for notifying the correct device.
Similar to the kPCI driver, the Bus will disable a device's legacy interrupt
when signaling to the driver that an interrupt is available to be serviced,
effectively masking it. We will add a new [PCI protocol
method](https://cs.opensource.google/fuchsia/fuchsia/+/main:sdk/banjo/fuchsia.hardware.pci/pci.banjo)
to allow for the device driver to request that its interrupt be re-enabled /
unmasked. This call will be necessary for drivers that may interact with
devices that use a legacy interrupt in some configurations, but will require
no changes to devices that solely operate using MSIs. This results in no
spurious interrupts and will suit the needs of the first usage described.

This is similar to [Linux's handling of Userspace I/O interrupts.](#prior-art-and-references)

For the second usage we will create an alternate legacy IRQ mode
`PCI_IRQ_MODE_LEGACY_ACKLESS`. This mode will not be selected by
`ConfigureIrqMode()` and will be need to be specifically selected by drivers
with this unique requirement. Devices whose interrupt are configured in this
manner will be monitored to see if the number of interrupts per second
exceeds a configured number. If this occurs the device's ability to generate
interrupts will be disabled. This operates similarly to [Linux's handling of
boot interrupts](#prior-art-and-references).

## Implementation

The changes can be made in order without any migrations or CQ concerns.

1. Modify `pci_configure_irq_mode` to add an out parameter to store the IRQ
   mode chosen for drivers that are agnostic to their interrupt mode and update
   existing callers.
2. Add a new protocol method `pci_legacy_interrupt_ack` (or simply
   `pci_interrupt_ack`) which re-enables legacy interrupts for the device
   and returns either `ZX_OK`, or `ZX_ERR_BAD_STATE` if the device is not
   configured to use legacy interrupts.
3. Update existing callers of `pci_configure_irq_mode` and users of
   `PCI_IRQ_MODE_LEGACY` to use the new protocol method in their interrupt
   handling.
4. Update drivers that handle interrupts abstractly and ensure they use
   `PCI_IRQ_MODE_LEGACY_NOACK` instead of `PCI_IRQ_MODE_LEGACY`.
5. Have the uPCI IRQ worker disable a device's legacy interrupt generation
   when signaling the device driver's virtual interrupt once all the drivers
   are migrated.
6. Extensively document the usage of PCI interrupts in the `PciProtocol` banjo
   as well as [Fuchsia.dev](https://www.fuchsia.dev).

## Performance

The majority of PCI devices encountered will operate using MSIs. The types of
devices that still use legacy interrupts are typically limited to older
hardware, integrated SOC devices with low performance requirements, emulated
environments that do not have MSI support, and devices that use an interrupt
only rarely.

Drivers that wish to handle legacy interrupts will add an additional channel
write to their interrupt handling routine by nature of this new PCI protocol
method requiring a write from the driver devhost proxy to uPCI. This can be
profiled by benchmarking the call itself, or checking aggregated cost of a
channel write in Zircon aggregated benchmarks.


## Security considerations

None.

## Privacy considerations

None.

## Testing

Existing integration and end-to-end tests in CQ/CI will verify that
interrupts are still working properly after the change, and a new unit test
will verify the operation of the changes to the `pci_configure_irq_mode`
protocol method.

## Documentation

PCI documentation will need to be expanded to explain the theory of operation
around interrupt modes. Additionally, it may be useful to note the need for
`pci_legacy_interrupt_ack` in `zx_interrupt_wait` and `zx_port_wait`
documentation.

## Drawbacks, alternatives, and unknowns

### Drawbacks
Most drivers will prefer to solely use MSI / MSI-X interrupt modes and will
not need to concern themselves with this api at all, so it is a more narrow
change in the system to require only drivers that may encounter devices using
legacy interrupts to handle this situation, rather than all drivers. However,
this does run the risk that a driver written for a particular device setup
would run into an issue where they only received their first interrupt if
they fail to ack. This may come up for drivers that support a wide range of
devices.

Situations with multiple backends gain some complexity as well. For example,
our xHCI driver has something akin to the following. If its PCI support
involves a legacy interrupt it might have to look something like:

```
// Initialize the proper setup and obtain an interrupt
if (pci_.is_valid()) {
  pci_init();
} else {
  mmio_init();
}

do {
  // Wait loop on the interrupt
  // Handle the interrupt

  if (mode_ == XHCI_MODE_PCI && irq_mode_ == PCI_IRQ_MODE_LEGACY)
    status = pci_.LegacyInterruptAck();
  }
}
```

If the ack code was omitted then this driver would work fine with MSI but
receive no interrupts after the first in legacy mode. Improving the testing
framework around PCI drivers will lead to better solutions for catching these
errors during development.

### Alternatives Considered

#### Mark excessive unhandled interrupts as spurious and disable the interrupt

Similiar to Linux, we could simply set a threshold for sequential spurious
interrupts and if reached we could disable or ignore that interrupt line
until a reboot. One major problem with this approach is that when Linux
handles a shared interrupt it calls the handler chain via the hard IRQ
handler in the kernel before acking the interrupt when all handlers have
finished. This ensures that all of the driver interrupt handlers have run
before the ack, so spurious interrupts only occur if no handler properly
handled the interrupt. With a uPCI driver in Zircon and device drivers being
out of process we can signal them to wake their irq handling threads, but we
have no way of knowing they have run to completion. This will lead to
spurious interrupts in the common case depending on how quickly a driver's
IRQ thread is scheduled and handles a given interrupt condition. However,
this approach still results in more spurious interrupts than the ack proposal
in the common case.

#### Add a PCI protocol method for waiting on interrupts

An option considered was to add a method to handle waiting on any type of
interrupt, effectively `pci_interrupt_wait` to avoid needing an extra
conditional in an interrupt.

Unfortunately, this results in PCI interrupts needing to be handled
differently than other interrupts. To the best of our capabilities I believe
making any interrupt object the same interface as any other interrupt object
has a lot of value and it is important that a driver author continue to be
able to use `zx_interrupt_wait`, `zx_port_bind`, `zx_port_wait`, and
`zx_object_wait_async`. Most drivers in our system have some combination of
an IRQ port with multiple interrupts, or multiple backends (UART, PCI, USB)
to deal with, so it's important not to violate the interface around interrupt
objects.

#### Handle this issue in derived InterruptDispatchers

We could keep kPCI's concept of a `PciInterruptDispatcher` and delegate this
work to it, keeping the interrupt handling between a PCI device driver and
the kernel. Unfortunately, this is a significant amount of coupling between
the userspace PCI driver and Zircon kernel.

1. Each specialized `InterruptDispatcher`would need to be able to write to a
   PCI device's control register to disable the legacy interrupt. This would
   require ideally a VMO, not unlike our approach to MSI, or for an address
   to be provided to whatever syscall creates this object. There is no way
   around this because the Dispatcher must know which device it corresponds
   to.
2. The interrupt disable is a single bit in a control register that the
   userspace Bus driver modifies frequently during bus and device
   initialization, presenting serious risks of race conditions if there are
   any pending interrupts.
3. If we use a derived `InterruptDispatcher` in the devices themselves we still
   need to handle determining whose interrupt fired on a shared line. Since
   the devices are no longer acting with the bus for their interrupt handling
   it means that we need to keep logic similar to kPCI's `SharedIrqHandler` in
   the kernel.
4. We now also need to plumb understanding of the PCI legacy IRQ
   routing table from ACPI / board files through the kernel again. This is
   processed in userspace & ACPI now.

At this time I don't see a reasonable path forward with this approach unless
we are willing to concede that PCI is a special driver that needs some custom
kernel code as well as ~2 additional syscalls to do its job.

## Prior art and references

Handling this entirely in userspace is a uniquely Zircon problem in my research.

1. OSX's DriverKit uses an `IOInterruptDispatchSource` which works in tandem
   with interrupt configuration and handling in the kernel. Additionally,
   PCIDriverKit only supports MSI and MSI-X interrupt modes.

   [PCIDriverKit > IOPCIDevice](https://developer.apple.com/documentation/pcidriverkit/iopcidevice)

2. Most Linux PCI interrupts are handled in the kernel itself. Shared
   interrupts have a chain of handlers registered to them by drivers. When an
   interrupt is fired the kernel calls each handler in sequence until one has
   handled the interrupt. If a shared interrupt line has enough spurious
   interrupts that are unhandled by any handles the kernel disables the
   interrupt.

   [Linux Boot Interrupts](https://www.kernel.org/doc/html/latest/PCI/boot-interrupts.html)

   Linux also supports simple userspace drivers through its Userspace I/O
   (UIO) interface. This allows for a driver to do a blocking `read()` on a
   provided `/dev/uioX` sysfs node to wait for an interrupt. Interrupts are
   disabled when triggered, but a driver can have them re-enabled by making a
   `write()` call to the sysfs node.

   [The Userspace I/O HOWTO](https://www.kernel.org/doc/html/v4.14/driver-api/uio-howto.html)

3. Most Windows PCI drivers are built using the Kernel-Mode Driver Framework
   (KMDF). Their interrupt handlers are called as part of Kernel interrupt
   dispatch and drivers register handlers that run in IRQ context.

   [Introduction to Interrupt Service Routines](https://docs.microsoft.com/en-us/windows-hardware/drivers/kernel/introduction-to-interrupt-service-routines)
