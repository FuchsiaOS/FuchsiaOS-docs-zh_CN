# Modifying board drivers

Caution: This page may contain information that is specific to the legacy
version of the driver framework (DFv1). Also the workflows documented on
this page may only be specific to the Fuchsia source checkout
(`fuchsia.git`) environment.

Most system on chip (SoC) board rely on explicitly enumerated hardware
resources, rather than discovery mechanisms like ACPI that are common on
workstations, servers, or laptops. To add support for an SoC's display
controller, you will need to modify the board driver to expose hardware
resources that your display driver can use. See
[amlogic-display.cc][amlogic-display] for an example. Most devices need some
combination of the following:

 * MMIO regions
 * Interrupts for vsync, hotplug events (typically through GPIO), etc. For example,
   the AMLogic display controller has both an RDMA engine and a capture
   interface with their own interrupts.
 * [BTIs][bti], Zircon's primitive for pinning virtual memory to physical addresses for
   hardware interaction.
 * GPIO pins, e.g. to turn on/off hardwired LCD panels.
 * Power resources, to turn on power to the controller and select operating
   voltages.
 * I2C bridges, e.g. for controlling DSI-to-HDMI output converters.
 * DSI access, because DSI support is typically a separate driver offered by the
   kernel.
 * [Sysmem] heaps, so that clients can allocate memory that is accessible to the
   device.

After all these resources have been enumerated, call `AddComposite` to
[expose them][cda] as a unit. The device manager will look for drivers that can
bind to this composite device and load the appropriate display driver.

[This change][vim3-cl] is a complete example of board driver modifications for the VIM3.

<!-- xrefs -->
[bti]: /docs/reference/kernel_objects/bus_transaction_initiator.md
[sysmem]: https://fuchsia.dev/reference/fidl/fuchsia.sysmem
[cda]: https://fuchsia.googlesource.com/fuchsia/+/7a5659579e2340a50cbd5063ee7925b46d4fcf6f/src/devices/board/drivers/astro/astro-display.cc#125
[amlogic-display]: /src/graphics/display/drivers/amlogic-display/amlogic-display.cc
[vim3-cl]: https://fxrev.dev/479034
