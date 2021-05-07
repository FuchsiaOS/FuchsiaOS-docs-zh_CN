# Modifying board drivers

Most system on chip (SoC) board rely on explicitly enumerated hardware
resources, rather than discovery mechanisms like ACPI that are common on
workstations, servers, or laptops. To add support for an SoC's display
controller, you will need to modify the board driver to expose hardware
resources that your display driver can use. See
[vim2/vim-display.cc][vim-display] for an example. Most devices need some
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

After all these resources have been enumerated, call `CompositeDeviceAdd` to
[expose them][cda] as a unit. The device manager will look for drivers that can
bind to this composite device and load the appropriate display driver.

[This change][vim3-cl] is a complete example of board driver modifications for the VIM3.

<!-- xrefs -->
[bti]: /docs/reference/kernel_objects/bus_transaction_initiator.md
[sysmem]: https://fuchsia.dev/reference/fidl/fuchsia.sysmem
[cda]: https://fuchsia.googlesource.com/fuchsia/+/a7e9836345caf947df79d72882ac864eceae7eb8/src/devices/board/drivers/vim2/vim-display.cc#126
[vim-display]: /src/devices/board/drivers/vim2/vim-display.cc
[vim3-cl]: https://fxrev.dev/479034
