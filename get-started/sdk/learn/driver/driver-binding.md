# Bind to the device node

To provide services for devices in a Fuchsia system, drivers must bind to device
nodes. The [driver manager][concepts-driver-manager] maintains the topology of
nodes, where each node represents access to a hardware or virtual device in the
system. Once bound to a device node, the driver can start providing services for
the device that the node represents.

The framework matches drivers to device nodes by correlating the
[node properties][concepts-node-properties] of each node with the set of bind
rules provided by the driver. Bind rules are a set of logic rules that describe
which node properties the driver supports.

In this section, you'll create a skeleton driver that binds to the `edu` device
and implements the bare driver framework hooks.

Note: For more details on the binding process and bind rules, see
[Driver binding][concepts-driver-binding].

## Create a new driver component

To begin, create a new project directory in your Bazel workspace for a driver
component called `qemu_edu`:

```posix-terminal
mkdir -p fuchsia-codelab/qemu_edu/drivers
```

After you complete this section, the project should have the following directory
structure:

```none {:.devsite-disable-click-to-copy}
//fuchsia-codelab/qemu_edu/drivers
                  |- BUILD.bazel
                  |- meta
                  |   |- qemu_edu.cml
                  |- qemu_edu.bind
                  |- qemu_edu.cc
                  |- qemu_edu.h
```

Create the `qemu_edu/drivers/BUILD.bazel` file and add the following statement to
include the necessary build rules from the Fuchsia SDK:

`qemu_edu/drivers/BUILD.bazel`:

```bazel
{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/BUILD.bazel" region_tag="imports" adjust_indentation="auto" %}
```

## Add the component manifest

The component manifest file defines the attributes of the component's executable,
including binding rules and the component's capabilities. Drivers are loaded as
shared libraries (`.so`) using the `driver` runner.

Create the `qemu_edu/drivers/meta/qemu_edu.cml` file and add the following:

`qemu_edu/drivers/meta/qemu_edu.cml`:

```json5
{
{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/meta/qemu_edu.cml" region_tag="driver" %}
}

```

Add the following build rules to the bottom of your `qemu_edu/drivers/BUILD.bazel`
file to compile the component manifest:

*   `fuchsia_component_manifest()`: Describes the source file and dependencies
    to compile the driver's [component manifest][concepts-component-manifest].

`qemu_edu/drivers/BUILD.bazel`:

```bazel
{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/BUILD.bazel" region_tag="component_manifest" adjust_indentation="auto" %}
```

## Configure bind rules

The bind rules describe which device nodes this driver can support. These are
listed as a series of condition statements that reference the key/value pairs in
the device node's properties. For a driver to be considered a match, all
rules must evaluate to true for the given device node.

Create `qemu_edu/drivers/qemu_edu.bind` and add the following bind rules to
declare the driver supports PCI devices with a VID and DID matching the `edu`
device:

`qemu_edu/drivers/qemu_edu.bind`:

```none
{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/qemu_edu.bind" region_tag="example_snippet" adjust_indentation="auto" exclude_regexp="fuchsia\.googlesource\.com" %}

```

Add the following build rules to the bottom of your `qemu_edu/drivers/BUILD.bazel`
file to compile the bind rules:

*   `fuchsia_driver_bytecode_bind_rules()`: Describes the specifics of the
    [bind rules][concepts-driver-binding] for a driver. The `rules` attribute
    specifies the file that contains the bind rules of this driver. The `deps`
    attribute specifies the [bind libraries][concepts-bind-libraries] to be used
    with the bind rules specified in rules.

`qemu_edu/drivers/BUILD.bazel`:

```bazel
{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/BUILD.bazel" region_tag="bind_rules" adjust_indentation="auto" %}
```

## Implement driver hooks

Once a driver is bound, the framework loads the component binary and constructs
an instance of the driver class registered using the
`FUCHSIA_DRIVER_LIFECYCLE_CPP_V2()` macro. The driver overrides the `Start()`
method to perform any initialization tasks.

Create `qemu_edu/drivers/qemu_edu.h` and `qemu_edu/drivers/qemu_edu.cc` and add
the following boilerplate code to create the driver class and configure the
initial `Start()` hook:

`qemu_edu/drivers/qemu_edu.h`:

```cpp
#ifndef FUCHSIA_CODELAB_CC_QEMU_EDU_H_
#define FUCHSIA_CODELAB_CC_QEMU_EDU_H_

{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/qemu_edu.h" region_tag="imports" adjust_indentation="auto" %}

{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/qemu_edu.h" region_tag="namespace_start" adjust_indentation="auto" %}

{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/qemu_edu.h" region_tag="class_header" adjust_indentation="auto" %}

{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/qemu_edu.h" region_tag="public_main" %}

{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/qemu_edu.h" region_tag="private_main" %}
{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/qemu_edu.h" region_tag="class_footer" adjust_indentation="auto" %}

{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/qemu_edu.h" region_tag="namespace_end" adjust_indentation="auto" %}

#endif  // FUCHSIA_CODELAB_CC_QEMU_EDU_H_

```

`qemu_edu/drivers/qemu_edu.cc`:

```cpp
{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/qemu_edu.cc" region_tag="imports" adjust_indentation="auto" %}

{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/qemu_edu.cc" region_tag="namespace_start" adjust_indentation="auto" %}

{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/qemu_edu.cc" region_tag="start_method_start" adjust_indentation="auto" %}

  FDF_SLOG(INFO, "edu driver loaded successfully");

{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/qemu_edu.cc" region_tag="start_method_end" adjust_indentation="auto" %}

{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/qemu_edu.cc" region_tag="namespace_end" adjust_indentation="auto" %}

{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/qemu_edu.cc" region_tag="driver_hook" adjust_indentation="auto" %}

```

Add the following build rule to the bottom of your `qemu_edu/drivers/BUILD.bazel`
file to compile the driver code into a shared library binary:

*   `cc_binary()`: Specifies the source and header files (for instance,
    `qemu_edu.cc` and `qemu_edu.h`) for building a C++ binary.

`qemu_edu/drivers/BUILD.bazel`:

```bazel
{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/BUILD.bazel" region_tag="binary" adjust_indentation="auto" exclude_regexp="edu_device\.cc|edu_device\.h|edu_server\.cc|edu_server\.h|\/\/src\/qemu_edu|sdk\/\/fidl\/fuchsia" %}
```

## Load the driver

With the initial scaffolding in place, you can publish the driver to a local
package repository and verify that it binds successfully to the `edu` device.

Add the following rules to the bottom of your `qemu_edu/drivers/BUILD.bazel` file
to build the driver component into a Fuchsia package:

*   `fuchsia_driver_component()`: Describes the binaries and artifacts for the
    `qemu_edu` driver component.
*   `fuchsia_package()`: Builds the driver component into a
    [Fuchsia package][concepts-packages].

`qemu_edu/drivers/BUILD.bazel`:

```bazel
{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/BUILD.bazel" region_tag="component" adjust_indentation="auto" %}

{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/drivers/BUILD.bazel" region_tag="package" adjust_indentation="auto" %}
```

Use the `bazel run` command to build and execute the component target:

```posix-terminal
bazel run //fuchsia-codelab/qemu_edu/drivers:pkg.component
```

The `bazel run` command performs the following steps:

1.  Build the component and package.
1.  Publish the package to a local package repository.
1.  Register the package repository with the target device.
1.  Use `ffx driver register` to load the driver component.

You should see the driver framework automatically detect a match a bind the
driver to the `edu` device node:

```none {:.devsite-disable-click-to-copy}
INFO: Build completed successfully, 1 total action
added repository bazel.pkg.component
Registering fuchsia-pkg://bazel.pkg.component/qemu_edu#meta/qemu_edu.cm
Successfully bound:
Node 'root.sys.platform.pt.PCI0.bus.00_06_0_.pci-00_06.0-fidl', Driver 'fuchsia-pkg://bazel.pkg.component/qemu_edu#meta/qemu_edu.cm'.
```

Inspect the system log and verify that you can see `FDF_SLOG()` message from the
driver code after the driver successfully binds:

```posix-terminal
ffx log --filter qemu_edu
```

```none {:.devsite-disable-click-to-copy}
[driver_index][driver_index,driver][I] Registered driver successfully: fuchsia-pkg://bazel.pkg.component/qemu_edu#meta/qemu_edu.cm.
[driver_manager][driver_manager.cm][I]: [driver_runner.cc:959] Binding fuchsia-pkg://bazel.pkg.component/qemu_edu#meta/qemu_edu.cm to  00_06_0_
{{ '<strong>' }}[full-pkg-drivers:root.sys.platform.pt.PCI0.bus.00_06_0_][qemu-edu,driver][I]: [fuchsia-codelab/qemu_edu/qemu_edu.cc:28] edu driver loaded successfully{{ '</strong>' }}
[driver-hosts:driver-host-3][][I]: [../../src/devices/bin/driver_host2/driver_host.cc:349] Started driver url=fuchsia-pkg://bazel.pkg.component/qemu_edu#meta/qemu_edu.cm
```

## Explore the updated device node

Now that the driver has successfully bound to a device node, `ffx driver list`
reports that your driver is loaded:

```posix-terminal
ffx driver list --loaded
```

```none {:.devsite-disable-click-to-copy}
fuchsia-boot:///#meta/block.core.cm
fuchsia-boot:///#meta/bus-pci.cm
fuchsia-boot:///#meta/cpu-trace.cm
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
{{ '<strong>' }}fuchsia-pkg://bazel.pkg.component/qemu_edu#meta/qemu_edu.cm{{ '</strong>' }}
```

Inspect the device node once more using `ffx driver list-devices`, and verify
that your driver is now listed as attached to the `edu` device node:

```posix-terminal
ffx driver list-devices root.sys.platform.pt.PCI0.bus.00_06_0_.pci-00_06.0-fidl --verbose
```

```none {:.devsite-disable-click-to-copy}
Name     : 0-fidl
Moniker  : root.sys.platform.pt.PCI0.bus.00_06_0_.pci-00_06.0-fidl
{{ '<strong>' }}Driver   : fuchsia-pkg://bazel.pkg.component/qemu_edu#meta/qemu_edu.cm{{ '</strong>' }}
11 Properties
[ 1/ 11] : Key fuchsia.BIND_FIDL_PROTOCOL     Value 0x000004
[ 2/ 11] : Key fuchsia.BIND_PCI_VID           Value 0x001234
[ 3/ 11] : Key fuchsia.BIND_PCI_DID           Value 0x0011e8
[ 4/ 11] : Key fuchsia.BIND_PCI_CLASS         Value 0x000000
[ 5/ 11] : Key fuchsia.BIND_PCI_SUBCLASS      Value 0x0000ff
[ 6/ 11] : Key fuchsia.BIND_PCI_INTERFACE     Value 0x000000
[ 7/ 11] : Key fuchsia.BIND_PCI_REVISION      Value 0x000010
[ 8/ 11] : Key fuchsia.BIND_PCI_TOPO          Value 0x000030
[ 9/ 11] : Key "fuchsia.hardware.pci.Device"  Value true
[10/ 11] : Key fuchsia.BIND_PROTOCOL          Value 0x000000
[11/ 11] : Key "fuchsia.driver.framework.dfv2" Value true
```

Congratulations! You have successfully bound a driver component to a device node
on Fuchsia.

<!-- Reference links -->

[concepts-bind-libraries]: /docs/development/drivers/concepts/device_driver_model/driver-binding.md#bind-libraries
[concepts-component-manifest]: /docs/concepts/components/v2/component_manifests.md
[concepts-driver-binding]: /docs/concepts/drivers/driver_binding.md
[concepts-driver-manager]: /docs/concepts/drivers/driver_framework.md#driver_manager
[concepts-node-properties]: /docs/concepts/drivers/drivers_and_nodes.md#node_properties
[concepts-packages]: /docs/concepts/packages/package.md
