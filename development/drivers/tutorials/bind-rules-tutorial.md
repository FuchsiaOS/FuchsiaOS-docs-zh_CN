# Bind rules tutorial

This guide explains how to write bind rules for a driver so that it binds to the devices it wants. It explains how to find the node properties and then write bind rules for it using the bind language.

This guide assumes familiarity with [Driver Binding](/docs/development/drivers/concepts/device_driver_model/driver-binding.md).

## Current state of node properties
Currently, node properties are defined in bind libraries and C++ header files. In the past, node properties were integer-based key-value pairs described as a C++ struct. All properties were defined in C++ header files and the bind rules were part of the driver source code.

However, the bind system was recently revamped so that bind rules are defined in a separate file using the bind language, and node properties can support string-based keys with boolean, string, integer or enum values.

A migration is now in the process to move all drivers from the old bind system to the new one. Node properties in the C++ headers are being redefined in bind libraries. For example, all the device protocol ID bind values are hardcoded in [protodefs.h](/src/lib/ddk/include/lib/ddk/protodefs.h). Each device protocol is now defined in their own bind library, which contains a definition of the protocol ID along with other node properties associated with the protocol. The bind libraries all live in [src/devices/bind](/src/devices/bind).

Until the migration is complete, both the old and new bind systems need to be supported simultaneously.

### Future state of node properties
Once the bind migration is complete, we can stop supporting the old integer-based node properties and remove the C++ definitions, such as [protodefs.h](/src/lib/ddk/include/lib/ddk/protodefs.h) and [binding_priv.h](/src/lib/ddk/include/lib/ddk/binding_priv.h). All properties will be defined in bind libraries and the keys will be entirely string-based.

Existing properties can be updated so they utilize the features of the new system. For example, the `BIND_COMPOSITE` property is a flag that is only set for composite devices. However, since the old system only supports integer values, an integer instead of a boolean represents the property value. With the old bind system removed, the property value can be changed to a boolean.

Other changes that can be made is how VIDs can be represented. Instead of assigning a unique integer number to a VID, we can use the VID name. For instance, the Intel VID is currently represented by the integer value `0x8`:

```
library fuchsia.intel.platform;

extend uint fuchsia.BIND_PLATFORM_DEV_VID {
  INTEL = 0x8,
};
```

With the new bind system, VIDs can be potentially represented by string values or even enums.

## Looking up node properties

### Using ffx driver list-devices
The command `ffx driver list-devices -v` prints the properties of every device in the tree in the format:

```
Name     : acpi-_TZ_
Moniker  : root.sys.platform.pt.acpi.acpi-_TZ_
Driver   : None
5 Properties
[ 1/  5] : Key fuchsia.BIND_PROTOCOL Value 0x1F
[ 2/  5] : Key fuchsia.BIND_PCI_VID Value 0x1AF4
[ 3/  5] : Key fuchsia.BIND_PCI_DID Value 0x1052
[ 4/  5] : Key fuchsia.COMPOSITE_BIND Value 1
[ 5/  5] : Key "fuchsia.acpi.hid" Value "GFSH0005"
```

### Driver properties in the driver source code

When adding a child device, drivers can provide properties that the bind rules match to. As such, you can find the properties to bind to through the driver source code.

In DFv1, the node properties in the source code are represented by “Properties” and “String Properties”. Properties contain integer-based keys and values. String properties however, contain string-based keys and values that can be integer, booleans, strings, or enums.

Here is a snippet where a driver adds a device with a `BIND_PROTOCOL` property and “ENABLE_TEST” string property.

```
device_add_args_t args = {};
args.version = DEVICE_ADD_ARGS_VERSION;
args.name = "parent";
args.ops = &dev_ops;

zx_device_prop_t props[] = {
      {BIND_PROTOCOL, 0,  ZX_PROTOCOL_PCI},
}
args.props = props;
args.prop_count = std::size(props);

zx_device_str_prop_t str_props[] = {
      zx_device_str_prop_t{.key = "ENABLE_TEST",
                           .property_value = str_prop_bool_val(true)}
};
args.str_props = str_props;
args.str_prop_count = std::size(str_props);

device_add(parent, &args, &dev);
```

In DFv2, node properties are represented by a `NodeProperty` struct in the [fuchsia.driver.framework FIDL library](/sdk/fidl/fuchsia.driver.framework/topology.fidl):

```
auto properties = fidl::VectorView<fdf::wire::NodeProperty>(arena, 2);
properties[0] = fdf::MakeProperty(arena, BIND_PROTOCOL, ZX_PROTOCOL_PCI);
properties[1] = fdf::MakeProperty(arena, "ENABLED_TEST", true);

auto args = fdf::wire::NodeAddArgs::Builder(arena)
                  .name(arena, "sample-child")
                  .properties(properties)
                  .Build();
```

## Defining properties in bind libraries

As we migrate from drivers from the old bind system to the new, we redefine the old properties in bind libraries on the new system. These bind libraries are located in the [src/devices/bind](/src/devices/bind) directory.

Any new properties are expected to be defined in the bind libraries.

### Properties from the old bind system
Most of the old node property keys and values are defined in [binding_priv.h](/src/lib/ddk/include/lib/ddk/binding_priv.h) and [protodefs.h](/src/lib/ddk/include/lib/ddk/protodefs.h).

`binding_priv.h` contains the hardcoded property keys. Each property key is assigned a unique integer value. In the new bind system, these keys are redefined with a `fuchsia.BIND_` prefix. For instance, `BIND_PROTOCOL` becomes `fuchsia.BIND_PROTOCOL` and `BIND_COMPOSITE` becomes `fuchsia.BIND_COMPOSITE`.

`protodefs.h` contains hardcoded ID values for device protocols.

### Device protocol bind library
Each device protocol in `protodefs.h` is expected to have its own bind library that contains its protocol ID and related properties. One example is the [fuchsia.i2c bind library](/src/devices/bind/fuchsia.i2c/fuchsia.i2c.bind), which not only defines the protocol ID values, but also other i2c related properties:

```
library fuchsia.i2c;

extend uint fuchsia.BIND_PROTOCOL {
  DEVICE = 24,
  IMPL = 25,
};

extend uint fuchsia.BIND_I2C_CLASS {
  HID = 0x01,
};

extend uint fuchsia.BIND_I2C_ADDRESS {
  BACKLIGHT = 0x2C,
  ETH = 0x18,
  FOCALTECH_TOUCH = 0x38,
  AMBIENTLIGHT = 0x39,
};
```

The protocol ID values match the ones defined in `protodefs.h`:

```
DDK_PROTOCOL_DEF(I2C,                     24,   "i2c", 0)
DDK_PROTOCOL_DEF(I2C_IMPL ,               25,   "i2c-impl", 0)
```

The `IMPL` value represents a driver that implements and serves the i2c protocol. `DEVICE` represents a core driver that consumes the `IMPL` protocol and serves the `DEVICE` protocol.

## Writing bind rules
This section describes how to write bind rules for the current state of driver binding. For bind rules that only bind to the above set of properties, you will need to write:

```
using fuchsia.acpi;
using fuchsia.pci;

fuchsia.BIND_PROTOCOL == fuchsia.pci.BIND_PROTOCOL.DEVICE;
fuchsia.BIND_PCI_VID == fuchsia.pci.BIND_PCI_VID.VIRTIO;
fuchsia.BIND_PCI_DID == fuchsia.pci.BIND_PCI_DID.VIRTIO_DEV_TYPE_INPUT;
fuchsia.BIND_COMPOSITE == 1;
fuchsia.acpi.hid == "GFSH0005";
```

### Property key

The integer key names are defined in `binding_priv.h` and are prefixed with “fuchsia”. For example, the key 0x03 in `dm dump` is determined to be `fuchsia.BIND_COMPOSITE` because `binding_priv.h` contains the following:

```
#define BIND_FLAGS 0x0000          // value of the flags register
#define BIND_PROTOCOL 0x0001       // primary protocol of the device
#define BIND_AUTOBIND 0x0002       // if this is an automated bind/load
#define BIND_COMPOSITE 0x003       // Whether this is a composite device
#define BIND_FIDL_PROTOCOL 0x0004  // primary FIDL protocol of the device
```

### Property values
Since the properties are associated with PCI, the matching values are found in the [fuchsia.pci.bind](/src/devices/bind/fuchsia.pci/fuchsia.pci.bind) library. `fuchsia.pci.BIND_PCI_VID.VIRTIO` is found by matching `0x001af4` to a value in `fuchsia.BIND_PCI_VID`.

```
extend uint fuchsia.BIND_PCI_VID {
  TEST = 0x0eff,
  AMD = 0x1002,
  REALTEK = 0x10ec,
  NVIDIA = 0x10de,
  GOOGLE = 0x1ae0,
  VIRTIO = 0x1af4,
  BROADCOM = 0x14e4,
  ATHEROS = 0x168c,
  INTEL = 0x8086,
};
```
If a value is missing from the bind libraries, you will need to add the missing definition in a new or existing bind library. Alternatively, you can just write a bind rule for the value directly:

```
fuchsia.BIND_PCI_VID == 0x1af4;
```

However, it’s preferable to define a value in the bind library.

### Composite bind rules
The same process is used for composite bind rules. For each node you want to write bind rules for, you can print the node properties and write bind rules for them.

Say you want to write composite bind rules that contain a node that binds to the above example and another node that binds to the following:

```
Name     : acpi-I2C2
Moniker  : root.sys.platform.pt.acpi.acpi-I2C2
Driver   : None
6 Properties
[ 1/  6] : Key fuchsia.BIND_ACPI_ID           Value 0x000034
[ 2/  6] : Key fuchsia.BIND_PCI_TOPO          Value 0x0000aa
[ 3/  6] : Key fuchsia.BIND_ACPI_BUS_TYPE     Value 0x000001
[ 4/  6] : Key "fuchsia.hardware.acpi.Device" Value true
[ 5/  6] : Key fuchsia.BIND_PROTOCOL          Value 0x00001e
[ 6/  6] : Key "fuchsia.driver.framework.dfv2" Value true
```

You can then write the following:

```
using fuchsia.acpi;
using fuchsia.pci;

primary node "pci_sample" {
  fuchsia.BIND_PROTOCOL == fuchsia.pci.BIND_PROTOCOL.DEVICE;
  fuchsia.BIND_PCI_VID == fuchsia.pci.BIND_PCI_VID.VIRTIO;
  fuchsia.BIND_PCI_DID == fuchsia.pci.BIND_PCI_DID.VIRTIO_DEV_TYPE_INPUT;
  fuchsia.BIND_COMPOSITE == 1;
  fuchsia.acpi.hid == "GFSH0005";
}

node "acpi" {
  fuchsia.driver.framework.dfv2 == true;
  fuchsia.BIND_ACPI_ID == 0x000034;
  fuchsia.BIND_PCI_TOPO == 0x0000aa;
  fuchsia.BIND_ACPI_BUS_TYPE == 0x000001;
}
```

#### Optional nodes
Optional nodes are nodes that do not have to be present in a device group in order for it
to be matched up to a composite driver.
This is only supported when using device groups, optional nodes will not be matched in
plain old composites where Nodes from the topology are matched to a composite driver.

In the bind rules, putting `optional` before the `node` keyword will mark the node
as being optional. As a convention, all optional nodes should be written after
the regular additional nodes.

Optional nodes are used to write composite driver rules that are more generic. For
example the HID buttons driver supports 9 different button types (volume-up, cam-mute, etc),
but not all hardware devices will have all of these buttons on them. With optional nodes
the bind rules can be written to be platform agnostic so the board drivers can create
composites with an arbitrary set of buttons that the HID buttons driver supports.

## Bind rules examples

### Branching bind rules

The bind language supports branching via if statements with some restrictions.

If statements must have else blocks and are terminal. This restriction increases readability by making explicit the branches of execution. Since no statement may follow an if statement, it is easy to trace a path through the bind rules.

```
if fuchsia.BIND_FIDL_PROTOCOL == fuchsia.hardware.tee.BIND_FIDL_PROTOCOL.DEVICE {
  fuchsia.BIND_PLATFORM_DEV_VID == fuchsia.platform.BIND_PLATFORM_DEV_VID.GENERIC;
} else {
  fuchsia.BIND_PLATFORM_DEV_VID == fuchsia.platform.BIND_PLATFORM_DEV_VID.QEMU;
}
```

Nesting if statements are supported as long they follow the restrictions. For example:

```
if fuchsia.driver.framework.dfv2 == true {
  if fuchsia.acpi.hid == "PNP0303" {
    true;
  } else {
    fuchsia.acpi.first_cid == "PNP0303";
  }
} else {
  fuchsia.BIND_PROTOCOL == fuchsia.acpi.BIND_PROTOCOL.DEVICE;
}
```

### Excluding multiple properties values

To reject multiple property values for a key, you can chain inequality condition statements together. For example, if you need to reject multiple fuchsia.BIND_PCI_DID values, you can do the following:

```
fuchsia.BIND_PCI_DID != 0x191b;
fuchsia.BIND_PCI_DID != 0x1912;
fuchsia.BIND_PCI_DID != 0x191d;
fuchsia.BIND_PCI_DID != 0x1902;
fuchsia.BIND_PCI_DID != 0x1916;
fuchsia.BIND_PCI_DID != 0x191e;
```
