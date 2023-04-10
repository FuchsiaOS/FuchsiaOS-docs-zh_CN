# Debug a driver when it fails to load

This guide provides best practices for debugging a new driver when it fails to load in your
Fuchsia system.

## Check if your driver is currently running in the system {:#check-if-your-driver-is-currently-running-in-the-system}

In Fuchsia, a driver gets loaded in the system when the driver is matched to a
[node][drivers-and-nodes] that represents a hardware or virtual device. Once matched and
loaded, the driver starts running and provides services to other components in the system.

To see if a driver is loaded (that is, currently running) in your Fuchsia system,
run the following command:

```posix-terminal
ffx driver list --loaded
```

This command prints the list of drivers loaded in the system, for example:

```none {:.devsite-disable-click-to-copy}
$ ffx driver list --loaded
fuchsia-boot:///#meta/bus-pci.cm
fuchsia-boot:///#meta/fvm.cm
fuchsia-boot:///#meta/hid.cm
fuchsia-boot:///#meta/intel-rtc.cm
fuchsia-boot:///#meta/netdevice-migration.cm
fuchsia-boot:///#meta/network-device.cm
fuchsia-boot:///#meta/pc-ps2.cm
fuchsia-boot:///#meta/platform-bus-x86.cm
fuchsia-boot:///#meta/platform-bus.cm
fuchsia-boot:///#meta/ramdisk.cm
fuchsia-boot:///#meta/sysmem.cm
fuchsia-boot:///#meta/virtio_block.cm
fuchsia-boot:///#meta/virtio_ethernet.cm
fuchsia-pkg://fuchsia.com/virtual_audio#meta/virtual_audio_driver.cm
fuchsia-pkg://bazel.pkg.component/qemu_edu#meta/qemu_edu.cm
fuchsia-boot:///#meta/block.core.cm
fuchsia-boot:///#meta/intel-i2c-dfv2.cm
```

Alternatively, if you know the exact driver component, you can use the
[`ffx component show`][ffx-component-show] command to view the state of the component,
for example:

```none {:.devsite-disable-click-to-copy}
$ ffx component show intel-i2c-dfv2
               Moniker: /bootstrap/boot-drivers:root.sys.platform.pt.pci.00_15_2.composite
                   URL: fuchsia-boot:///#meta/intel-i2c-dfv2.cm
                  Type: CML dynamic component
       Component State: Resolved
 Incoming Capabilities: fuchsia.boot.Items
                        fuchsia.device.fs.Exporter
                        fuchsia.driver.compat.Service
                        fuchsia.logger.LogSink
                        pkg
  Exposed Capabilities: diagnostics
                        fuchsia.driver.compat.Service
       {{ '<strong>' }}Execution State: Running{{ '</strong>' }}
          Start reason: Instance is in a single_run collection
 Outgoing Capabilities: fuchsia.driver.compat.Service
```

If you discover that your driver is not loaded in the system, see the next section for the steps
you can take to debug the issue.

## Best practices for debugging {:#best-practices-for-debugging}

During the initial phase of writing a new Fuchsia driver, you’d first want to make sure that your
new component is recognized as a driver in your Fuchsia system. Once the component is listed as
a driver in your Fuchsia system, you can start working on [writing bind rules][write-bind-rules]
so that your driver can bind to a specific node that represents your target device in the system.
When you have the driver bound to your target node (thus, the driver is successfully loaded in the
system), you can then move on to the next phrase of development, which is to start implementing
features for the driver.

When debugging a driver that fails to load in your Fuchsia system, consider the following steps:

1. [Check the component manifest](#check-the-component-manifest).
1. [Register your driver with the driver framework](#register-your-driver-with-the-driver-framework).
1. [Verify that the bind rules are correct](#verify-that-the-bind-rules-are-correct).

### 1. Check the component manifest {:#check-the-component-manifest}

For a Fuchsia system to view a component as a driver, the component’s `runner` field in the
[component manifest][component-manifests] (`.cml`) must be set to `driver`, for example:

```none {:.devsite-disable-click-to-copy}
    program: {
        runner: 'driver',
        ...
    }
```

### 2. Register your driver with the driver framework {:#register-your-driver-with-the-driver-framework}

Next, confirm that your component is listed as a driver in your Fuchsia system.

For a component to be recognized as a driver, you need to explicitly register the component as
a driver in your Fuchsia system. If a component does not appear as a driver in the system,
it means that the [driver framework][driver-framework] is not aware of the component, therefore
such a component will never be considered for [driver binding][driver-binding]. So having your
new component appear as a driver in your Fuchsia system should be the first milestone when
writing a new driver.

To register a component as a driver in your Fuchsia system, do the following:

1. Upload the Fuchsia package (that contains the driver component) to your
   [Fuchsia package server][fuchsia-package-server].

   Note: The `bazel` commands can execute the building, uploading, and registering
   of a driver package at once. If your Fuchsia development environment uses the
   Bazel build system, you can replace steps 1 and 2 here with the
   [`bazel run`][bazel-run] command.

1. Register the component as a driver in the system:

   ```posix-terminal
   ffx driver register <URL>
   ```

   Replace `URL` with the component URL from your Fuchsia package server, for example:

   ```none {:.devsite-disable-click-to-copy}
   ffx driver register fuchsia-pkg://fuchsia.com/my_example#meta/my_new_driver.cm
   ```

1. View the list of drivers currently registered (but not necessarily running) in the system:

   ```posix-terminal
   ffx driver list
   ```

   This command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   $ ffx driver list
   fuchsia-boot:///#meta/block.core.cm
   fuchsia-boot:///#meta/bus-pci.cm
   fuchsia-boot:///#meta/fvm.cm
   fuchsia-boot:///#meta/hid-input-report.cm
   fuchsia-boot:///#meta/hid.cm
   fuchsia-boot:///#meta/intel-rtc.cm
   fuchsia-boot:///#meta/netdevice-migration.cm
   fuchsia-boot:///#meta/network-device.cm
   fuchsia-boot:///#meta/pc-ps2.cm
   fuchsia-boot:///#meta/platform-bus-x86.cm
   fuchsia-boot:///#meta/platform-bus.cm
   fuchsia-boot:///#meta/ramdisk.cm
   fuchsia-boot:///#meta/sysmem.cm
   fuchsia-boot:///#meta/virtio_block.cm
   fuchsia-boot:///#meta/virtio_ethernet.cm
   fuchsia-boot:///#meta/zxcrypt.cm
   fuchsia-pkg://fuchsia.com/virtual_audio#meta/virtual_audio_driver.cm
   fuchsia-pkg://fuchsia.com/my_example#meta/my_new_driver.cm
   ```

   Verify that your new driver component appears in this list.

### 3. Verify that the bind rules are correct {:#verify-that-the-bind-rules-are-correct}

At last, start examining the bind rules of your driver.

A driver’s bind rules determine which nodes it can bind to in a Fuchsia system. The driver
framework loads drivers only when they match the node properties of specific nodes in the system.
If your driver is registered in the system, but is not loaded (therefore is not running), then
check the driver’s bind rules and verify that they are correctly written to match the bind
properties of the target node in your Fuchsia system.

To view all nodes and their node properties in your Fuchsia system, run the following command:

```posix-terminal
ffx driver list-devices -v
```

This command prints output similar to the following:

```none {:.devsite-disable-click-to-copy}
$ ffx driver list-devices -v
...
Name : acpi-I2C2
Moniker : root.sys.platform.pt.acpi.acpi-I2C2
Driver : None
6 Properties
[ 1/ 6] : Key fuchsia.BIND_ACPI_ID Value 0x000034
[ 2/ 6] : Key fuchsia.BIND_PCI_TOPO Value 0x0000aa
[ 3/ 6] : Key fuchsia.BIND_ACPI_BUS_TYPE Value 0x000001
[ 4/ 6] : Key "fuchsia.hardware.acpi.Device" Value true
[ 5/ 6] : Key fuchsia.BIND_PROTOCOL Value 0x00001e
[ 6/ 6] : Key "fuchsia.driver.framework.dfv2" Value true
...
Name : 00_15_2
Moniker : root.sys.platform.pt.pci.00_15_2
Driver : None
9 Properties
[ 1/ 9] : Key fuchsia.BIND_PROTOCOL Value 0x00001f
[ 2/ 9] : Key fuchsia.BIND_PCI_VID Value 0x008086
[ 3/ 9] : Key fuchsia.BIND_PCI_DID Value 0x009d62
[ 4/ 9] : Key fuchsia.BIND_PCI_CLASS Value 0x000011
[ 5/ 9] : Key fuchsia.BIND_PCI_SUBCLASS Value 0x000080
[ 6/ 9] : Key fuchsia.BIND_PCI_INTERFACE Value 0x000000
[ 7/ 9] : Key fuchsia.BIND_PCI_REVISION Value 0x000021
[ 8/ 9] : Key fuchsia.BIND_PCI_TOPO Value 0x0000aa
[ 9/ 9] : Key "fuchsia.driver.framework.dfv2" Value true
...
```

When debugging bind rules, a recommended practice is to visually examine the output from this
command to ensure that your Fuchsia system contains nodes with the right node properties. Also,
keep in mind that a node is allowed to have only one driver bound to it. So you want to make sure
that the target node in your Fuchsia system does not have a driver already bound to it.

The example output above shows a PCI node and an ACPI node, which the `intel-i2c` driver can
bind to. You may write the driver’s bind rules against these two nodes in the following way:

```none {:.devsite-disable-click-to-copy}
primary node "pci" {
    fuchsia.driver.framework.dfv2 == true;

    fuchsia.BIND_PROTOCOL == fuchsia.pci.BIND_PROTOCOL.DEVICE;
    fuchsia.BIND_PCI_VID == fuchsia.pci.BIND_PCI_VID.INTEL;
    accept fuchsia.BIND_PCI_DID {
        // For now we only add the DID for the touchpad.
        fuchsia.intel.pci.BIND_PCI_DID.SUNRISE_POINT_SERIALIO_I2C2,
    }
}

node "acpi" {
    fuchsia.driver.framework.dfv2 == true;

    fuchsia.BIND_ACPI_ID == 0x000034;
    fuchsia.BIND_PCI_TOPO == 0x0000aa;
    fuchsia.BIND_ACPI_BUS_TYPE == 0x000001;
}
```

For the ACPI node, visually verify that the ACPI values specified in the bind rules match the
values the ACPI node properties (shown in the output of the `ffx` command above). And for
the PCI node, examine the [PCI bind library][fuchsia-pci-bind] directly to check if the values
defined in the library match the values of the PCI node's properties. (For more information
on these two approaches, see [Write bind rules for a driver][write-bind-rules].)

## Appendices

### Error: driver lifecycle not found {:#error-driver-lifecycle-not-found}

After registering a component as a driver in your Fuchsia system, you may see an error message
similar to the following in the device logs ([`ffx log`][ffx-log]):

```none {:.devsite-disable-click-to-copy}
Failed to start driver; driver lifecycle not found url=<DRIVER_URL>
```

If you run into this error, make sure that the `FUCHSIA_DRIVER_LIFECYCLE_CPP_V2()` macro is added
at the end of your driver component’s source code, for example:

```none {:.devsite-disable-click-to-copy}
// Register driver hooks with the framework
FUCHSIA_DRIVER_LIFECYCLE_CPP_V2(fdf::Lifecycle<qemu_edu::QemuEduDriver>);
```

For more information on this macro, see [Implement driver hooks][implement-driver-hooks] in
the Driver codelab.

<!-- Reference links -->

[drivers-and-nodes]: /docs/concepts/drivers/drivers_and_nodes.md
[write-bind-rules]: /docs/development/sdk/write-bind-rules-for-driver.md
[component-manifests]: /docs/concepts/components/v2/component_manifests.md
[driver-framework]: /docs/concepts/drivers/driver_framework.md
[driver-binding]: /docs/concepts/drivers/driver_binding.md
[fuchsia-package-server]: /docs/development/sdk/ffx/create-a-package-repository.md
[bazel-run]: /docs/get-started/sdk/get-started-with-driver.md#build-and-load-the-sample-driver
[fuchsia-pci-bind]: https://cs.opensource.google/fuchsia/fuchsia/+/main:src/devices/bind/fuchsia.pci/fuchsia.pci.bind
[ffx-component-show]: /docs/development/sdk/ffx/view-component-information.md
[ffx-log]: /docs/development/sdk/ffx/view-device-logs.md
[implement-driver-hooks]: /docs/get-started/sdk/learn/driver/driver-binding.md#implement_driver_hooks
