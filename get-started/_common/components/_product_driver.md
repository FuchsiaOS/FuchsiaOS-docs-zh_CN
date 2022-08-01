<!-- ## Driver framework -->
## 驱动框架

<!-- Similar to session, the Fuchsia Driver Framework enables developers to
implement product-specific device drivers as components. Some driver components
represent hardware interface controllers, such as PCI or USB, while others
interact with end devices, such as an ethernet controller or keyboard. -->
与会话类似，Fuchsia 驱动框架使开发人员能够将产品特定的设备驱动程序实现为组件。
一些驱动程序组件代表硬件接口控制器，例如 PCI 或 USB，而其他驱动程序组件则与终端设备交互，
例如以太网控制器或键盘。

<!-- As devices are discovered or attached to the system, the `driver_manager`
platform component starts the necessary driver components, binds them to the
hardware interfaces, and manages their lifecycle. -->
当设备被发现或连接到系统时，`driver_manager` 平台组件启动必要的驱动程序组件，
将它们绑定到硬件接口，并管理它们的生命周期。

<!-- Note: For more details on the driver framework, see
[Fuchsia Driver Framework](/development/drivers/concepts/fdf.md). -->
注：要获取关于驱动框架的更多细节，请参阅 [Fuchsia 驱动框架](/development/drivers/concepts/fdf.md).
