## Driver framework

Similar to session, the Fuchsia Driver Framework enables developers to
implement product-specific device drivers as components. Some driver components
represent hardware interface controllers, such as PCI or USB, while others
interact with end devices, such as an ethernet controller or keyboard.

As devices are discovered or attached to the system, the `driver_manager`
platform component starts the necessary driver components, binds them to the
hardware interfaces, and manages their lifecycle.

Note: For more details on the driver framework, see
[Fuchsia Driver Framework](development/drivers/concepts/fdf.md).
