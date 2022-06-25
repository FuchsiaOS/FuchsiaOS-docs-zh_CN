# Driver binding

Caution: This page may contain information that is specific to the legacy
version of the driver framework (DFv1).

In Fuchsia, the driver framework maintains a tree of drivers and devices in the system. In this
tree, a device represents access to some hardware available to the OS. A driver both publishes and
binds to devices. For example, a USB driver might bind to a PCI device (its parent) and publish an
ethernet device (its child). In order to determine which devices a driver can bind to, each driver
has a bind rule and each device has a set of properties. The bind rule defines a condition
that matches the properties of devices that it wants to bind to.

Bind rules and the conditions they refer to are defined by a domain specific language. The bind
compiler consumes this language and produces bytecode for bind rules. The language has two
kinds of source files: rules, and libraries. Libraries are used to share property definitions
between drivers and bind rules. The compiler also produces FIDL files from bind libraries so
that drivers may refer to device properties in code.

Note: Driver binding is under active development and this document describes the current state.
Not all drivers use this form of bind rules but a migration is under way to convert them all.

One thing to note about this stage of the migration is that there is no support for defining device
property keys in bind libraries (see below). Instead, the keys from the old driver binding system
([lib/ddk/binding.h](/src/lib/ddk/include/lib/ddk/binding.h)) are available to be extended.
These keys are hardcoded into the bind compiler and are available under the `fuchsia` namespace.
For example, the PCI vendor ID key is `fuchsia.BIND_PCI_VID`. Eventually the hardcoded keys will be
removed from this namespace and all bind property keys will be defined in bind libraries.


## The compiler

The compiler takes a list of library sources, and one rule source. For example:

```
fx bindc compile \
  --include src/devices/bind/fuchsia.usb/fuchsia.usb.bind \
  --output tools/bindc/examples/gizmo.h \
  tools/bindc/examples/gizmo.bind
```

Currently, it produces a C header file that may be included by a driver. The header file defines a
macro:

```
ZIRCON_DRIVER(Driver, Ops, VendorName, Version);
```

 - `Driver` is the name of the driver.
 - `Ops` is a `zx_driver_ops`, which are the driver operation hooks
 - `VendorName` is a string representing the name of the driver vendor.
 - `Version` is a string representing the version of the driver.

For more details, see [the driver development documentation]
(concepts/drivers/driver-development).

## Bind rules {#bind-rules}

A bind rule defines the conditions to call a driver's `bind()` hook. Each statement in the bind
rule is a condition over the properties of the device that must hold true in order for the
driver to bind. If the bind rules finish executing and all conditions are true, then the device
coordinator will call the driver's `bind()` hook.

A bind rule should be thought of as a declarative expression of the conditions under which a
driver should bind. As such, the order of execution of condition expressions is not relevant to its
final evaluation. It may help to consider the bind rule to be a Boolean formula.

There are four kinds of statements:

 - **Condition statements** are equality (or inequality) expressions of the form
   `<key> == <value>` (or `<key> != <value>`).
 - **Accept statements** are lists of permissible values for a given key.
 - **If statements** provide simple branching.
 - **True and false statements** can be used to explicitly evaluate a bind rule.

### Example

This example bind rule can be found at [//tools/bindc/examples/gizmo.bind](/tools/bindc/examples/gizmo.bind).

```
using fuchsia.usb;

// The device must be a USB device.
fuchsia.BIND_PROTOCOL == fuchsia.usb.BIND_PROTOCOL.INTERFACE;

if fuchsia.BIND_USB_VID == fuchsia.usb.BIND_USB_VID.INTEL {
  // If the device's vendor is Intel, the device class must be audio.
  fuchsia.BIND_USB_CLASS == fuchsia.usb.BIND_USB_CLASS.AUDIO;
} else if fuchsia.BIND_USB_VID == fuchsia.usb.BIND_USB_VID.REALTEK {
  // If the device's vendor is Realtek, the device class must be one of the following values:
  accept fuchsia.BIND_USB_CLASS {
    fuchsia.usb.BIND_USB_CLASS.COMM,
    fuchsia.usb.BIND_USB_CLASS.VIDEO,
  }
} else {
  // If the vendor is neither Intel or Realtek, do not bind.
  false;
}
```

### Language restrictions

There are some restrictions on the language that are imposed to improve readability and ensure that
bind rules are simple representations of the conditions under which a driver should bind.

 - **Empty blocks are not allowed**.
   It's ambiguous whether an empty block should mean that the driver will bind or abort. The
   author should use an explicit `true` or `false` statement.

 - **If statements must have else blocks and are terminal**.
   This restriction increases readability by making explicit the branches of execution. Since no
   statement may follow an `if` statement, it is easy to trace a path through the bind rules.

 - **True and false statements must be the only statement in their scope**.
   Bind rules are not imperative programs and the order of evaluation is not important. Mixing
   boolean statements (particularly `true`) with other conditions may lead to situations where this
   is not clear.

### Grammar

```
rule = using-list , ( statement )+ ;

using-list = ( using , ";" )* ;

using = "using" , compound-identifier , ( "as" , IDENTIFIER ) ;

statement = condition , ";" | accept | if-statement | true | false ;

condition = compound-identifier , condition-op , value ;

condition-op = "==" | "!=" ;

accept = "accept" , compound-identifier , "{" ( value , "," )+ "}" ;

if-statement = "if" , condition , "{" , ( statement )+ , "}" ,
                ( "else if" , "{" , ( statement )+ , "}" )* ,
                "else" , "{" , ( statement )+ , "}" ;

true = "true" , ";" ;

false = "false" , ";" ;

compound-identifier = IDENTIFIER ( "." , IDENTIFIER )* ;

value = compound-identifier | STRING-LITERAL | NUMERIC-LITERAL | "true" | "false" ;
```

An identifier matches the regex `[a-zA-Z]([a-zA-Z0-9_]*[a-zA-Z0-9])?` and must not match any
keyword. The list of keywords is:

```
accept
as
else
false
if
true
using
```

A string literal matches the regex `”[^”]*”`, and a numeric literal matches the regex `[0-9]+` or
`0x[0-9A-F]+`.

The bind compiler will ignore (treat as whitespace) any line prefixed by `//`, and any multiple
lines delimited by `/*` and `*/`.


### Composite Bind

In addition to binding drivers to devices, drivers in Fuchsia can use bind rules
to create a composite device from nodes. The bind rules follow the same language
specification as non-composite bind, but separated into nodes that contain a
name and set of statements.

There must be exactly one primary node in the bind rules. The composite driver will
be started in the same driver host as the primary node.

An example composite bind rule file can be found at
[//tools/bindc/examples/composite-gizmo.bind](/tools/bindc/examples/composite-gizmo.bind).

```
using fuchsia.usb;

composite gizmo_example;

primary node "gizmo_usb" {
  fuchsia.BIND_PROTOCOL == fuchsia.usb.BIND_PROTOCOL.INTERFACE;
}

node "audio" {
  if fuchsia.BIND_USB_VID == fuchsia.usb.BIND_USB_VID.INTEL {
    fuchsia.BIND_USB_CLASS == fuchsia.usb.BIND_USB_CLASS.AUDIO;
  } else if fuchsia.BIND_USB_VID == fuchsia.usb.BIND_USB_VID.REALTEK {
    accept fuchsia.BIND_USB_CLASS {
      fuchsia.usb.BIND_USB_CLASS.COMM,
      fuchsia.usb.BIND_USB_CLASS.VIDEO,
    }
  }
}

```

The grammar for composite bind is:

```
composite-bind-rules = [composite-device], using-list , ( node )+ ;

composite-device = “composite” , IDENTIFIER;

node = [ "primary" ], "node" , STRING-LITERAL , "{" , ( statement )+ , "}"
```

### Build targets

To declare bind rules within the Fuchsia build system, use the following build target:

```gn
driver_bind_rules("bind") {
  rules = <bind rules filename>
  header_output = <generated header filename>
  bind_output = <generated bind binary filename>
  deps = [ <list of bind library targets> ]
}
```

For more details, refer to [//build/bind/bind.gni](/build/bind/bind.gni).

## Testing
The bind compiler supports a data-driven unit test framework for bind rules that allows you to
test your bind rules in isolation from the driver. A test case for a bind rule consists of a
device specification and an expected result, i.e. bind or abort. Test cases are passed to the bind
compiler in the form of JSON specification files and the compiler executes each test case by
running the debugger.

The JSON specification must be a list of test case objects, where each object contains:

 - `name` A string for the name of the test case.
 - `expected` The expected result. Must be `“match”` or `“abort”`.
 - `device` A list of string key value pairs describing the properties of a device. This is
   similar to the debugger's [device specifications](development/drivers/diagnostics/bind-debugger.md#device-specification).

If the test is for a composite device, then each node in the device can have a
list of test case objects. The JSON specification for the unit tests will be a
list of node objects instead. Each node object contains:

-   `node` A string for the node name. It must match a node in the bind rules
    tests.
-   `tests` A list of test case objects.

### Example

This is an example test case, the full set of tests is at `//tools/bindc/examples/test.json`. This
case checks that the bind rules match a device with the listed properties, i.e. an Intel USB audio
device.

```
[
  {
    "name": "Intel",
    "expected": "match",
    "device": {
      "fuchsia.BIND_PROTOCOL": "fuchsia.usb.BIND_PROTOCOL.INTERFACE",
      "fuchsia.BIND_USB_VID": "fuchsia.usb.BIND_USB_VID.INTEL",
      "fuchsia.BIND_USB_CLASS": "fuchsia.usb.BIND_USB_CLASS.AUDIO"
    }
  }
]
```

Here is an example of a composite bind node with test cases. The full set of tests is located in
``//tools/bindc/examples/composite-tests.json`. Each test case checks if the node’s
bind rules match a device with the listed properties.

```
[
    {
        "node": "sysmem",
        "tests": [
            {
                "name": "Match",
                "expected": "match",
                "device": {
                    "fuchsia.BIND_PROTOCOL": "fuchsia.sysmem.BIND_PROTOCOL.DEVICE"
                }
            },
            {
                "name": "Abort sysmem",
                "expected": "abort",
                "device": {
                    "fuchsia.BIND_PROTOCOL": "fuchsia.tee.BIND_PROTOCOL.DEVICE"
                }
            }
        ]
    }
]
```

### Build

Define a test build target like so

```
bind_test("example_bind_test") {
  rules = <bind rules filename>
  tests = <test specification filename>
  deps = [ <list of bind library targets> ]
}
```

Alternatively, you can simply add a `tests` argument to your existing `bind_rules` to generate a
test target. It’s name will be the original target’s name plus `_test`. For example, the following
would generate `example_bind_test`.

```
driver_bind_rules("example_bind") {
  rules = "gizmo.bind"
  header_output = “gizmo_bind.h”
  bind_output = “gizmo.bindbc”
  tests = "tests.json"
  deps = [ "//src/devices/bind/fuchsia.usb" ]
}
```

### Run

If you have defined a build target for your test then you can run the tests as usual with fx test.

```
fx test example_bind_test
```

Otherwise you can run the bind tool directly. For example:

```
fx bindc test \
  tools/bindc/examples/gizmo.bind \
  --test-spec tools/bindc/examples/tests.json \
  --include src/devices/bind/fuchsia.usb/fuchsia.usb.bind
```

## Bind libraries {#bind-libraries}

A bind library defines a set of properties that drivers may assign to their children. Also,
bind rules may refer to bind libraries.

### Namespacing

A bind library begins by defining its namespace:

```
library <vendor>.<library>;
```

Every namespace must begin with a vendor and each vendor should ensure that there are no clashes
within their own namespace. However, the language allows for one vendor to extend the library of
another. Google will use `fuchsia` for public libraries.

Any values introduced by a library are namespaced. For example, the following library defines a
new PCI device ID `GIZMO_VER_1`.

```
library gizmotronics.gizmo;

using fuchsia.pci as pci;

extend uint pci.device_id {
  GIZMO_VER_1 = 0x4242,
};
```

To refer to this value the driver author should use the fully qualified name, as follows.

```
using fuchsia.pci as pci;
using gizmotronics.gizmo;

pci.device_id == gizmotronics.gizmo.device_id.GIZMO_VER_1
```

WARNING: Extending new keys (fxb/95933) and aliasing namespaces (fxb/95939) are currently unsupported.

### Keys and values

Device property definitions look similar to variable declarations in other languages.

```
<type> <name>;
Or:
<type> <name> {
  <value>,
  <value>,
  …
};
```

A bind library may also extend properties from other libraries.

```
extend <type> <name> {
  <value>,
  …
};
```

Each key has a type, and all values that correspond to that key must be of that type. The language
supports primitive types: one of `uint`, `string`, or `bool`; and enumerations (`enum`). When
defining keys you should prefer enumerations except when values will be provided by an external
source, such as hardware.

When definining a primitive value use the form `<identifier> = <literal>`, and for enumerations
only an identifier is necessary. It is valid to define multiple primitive values with the same
literal.

### Grammar

```
library = library-header , using-list , declaration-list ;

library-header = "library" , compound-identifier , ";" ;

using-list = ( using , ";" )* ;

using = "using" , compound-identifier , ( "as" , IDENTIFIER ) ;

compound-identifier = IDENTIFIER ( "." , IDENTIFIER )* ;

declaration-list = ( declaration , ";" )* ;

declaration = primitive-declaration | enum-declaration ;

primitive-declaration = ( "extend" ) , type , compound-identifier ,
                        ( "{" primitive-value-list "}" ) ;

type = "uint" | "string" | "bool";

primitive-value-list = ( IDENTIFIER , "=" , literal , "," )* ;

enum-declaration = ( "extend" ) , "enum" , compound-identifier ,
                   ( "{" , enum-value-list , "}" ) ;

enum-value-list = ( IDENTIFIER , "," )* ;

literal = STRING-LITERAL | NUMERIC-LITERAL | "true" | "false" ;
```

An identifier matches the regex `[a-zA-Z]([a-zA-Z0-9_]*[a-zA-Z0-9])?` and must not match any
keyword. The list of keywords is:

```
as
bool
enum
extend
library
string
uint
using
```

A string literal matches the regex `”[^”]*”`, and a numeric literal matches the regex `[0-9]+` or
`0x[0-9A-F]+`.

The bind compiler will ignore (treat as whitespace) any line prefixed by `//`, and any multiple
lines delimited by `/*` and `*/`.

### Build targets

To declare a bind library within the Fuchsia build system, use the following build target:

```gn
bind_library(<library name>) {
  source = <bind library filename>
  public_deps = [ <list of bind library targets> ]
}
```

For more details, refer to [//build/bind/bind.gni](/build/bind/bind.gni).


### Writing bind rules for device properties

####Current state of device properties
Currently, device properties are defined in bind libraries and C++ header files. In the past, device properties were integer-based key-value pairs described as a C++ struct. All properties were defined in C++ header files and the bind rules were part of the driver source code.

However, the bind system was recently revamped so that bind rules are defined in a separate file using the bind language, and device properties can support string-based keys with boolean, string, integer or enum values.

A migration is now in the process to move all drivers from the old bind system to the new one. Bind properties in the C++ headers are being redefined in bind libraries. For example, all the device protocol ID bind values are hardcoded in [protodefs.h](/src/lib/ddk/include/lib/ddk/protodefs.h). Each device protocol is now defined in their own bind library, which contains a definition of the protocol ID along with other bind properties associated with the protocol. The bind libraries all live in [src/devices/bind](/src/devices/bind).

Until the migration is complete, both the old and new bind systems need to be supported simultaneously.

####Future state of device properties
Once the bind migration is complete, we can stop supporting the old integer-based device properties and remove the C++ definitions, such as [protodefs.h](/src/lib/ddk/include/lib/ddk/protodefs.h) and [binding_priv.h](/src/lib/ddk/include/lib/ddk/binding_priv.h). All properties will be defined in bind libraries and the keys will be entirely string-based.

Existing properties can be updated so they utilize the features of the new system. For example, the `BIND_COMPOSITE` property is a flag that is only set for composite devices. However, since the old system only supports integer values, an integer instead of a boolean represents the property value. With the old bind system removed, the property value can be changed to a boolean.

Other changes that can be made is how VIDs can be represented. Instead of assigning a unique integer number to a VID, we can use the VID name. For instance, the Intel VID is currently represented by the integer value `0x8`:

```
library fuchsia.intel.platform;

extend uint fuchsia.BIND_PLATFORM_DEV_VID {
  INTEL = 0x8,
};
```

With the new bind system, VIDs can be potentially represented by string values or even enums.

####Looking up device properties

#####Using ffx driver list-devices
The command `ffx driver list-devices -v` prints the properties of every device in the tree in the format:

```
Name     : acpi-GFRO
Moniker  : root.sys.platform.platform-passthrough.acpi.acpi-GFRO
Driver   : None
5 Properties
[ 1/  5] : Key fuchsia.BIND_ACPI_ID           Value 0x000024
[ 2/  5] : Key "fuchsia.acpi.hid"             Value "GFSH0008"
[ 3/  5] : Key "fuchsia.hardware.acpi.Device" Value true
[ 4/  5] : Key fuchsia.BIND_PROTOCOL          Value 0x00001e
[ 5/  5] : Key "fuchsia.driver.framework.dfv2" Value true
```

See the [drivers and nodes documentation](concepts/drivers/drivers_and_nodes.md) for more information on the printed topology.

#####Driver properties in the driver source code

When adding a child device, drivers can provide properties that the bind rules match to. As such, you can find the properties to bind to through the driver source code.

In DFv1, the device properties in the source code are represented by “Properties” and “String Properties”. Properties contain integer-based keys and values. String properties however, contain string-based keys and values that can be integer, booleans, strings, or enums.

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

In DFv2, device properties are represented by a `NodeProperty` struct in the [fuchsia.driver.framework FIDL library](/sdk/fidl/fuchsia.driver.framework/topology.fidl):

```
auto properties = fidl::VectorView<fdf::wire::NodeProperty>(arena, 2);
properties[0] = fdf::wire::NodeProperty::Builder(arena)
     .key(fdf::wire::NodePropertyKey::WithIntValue(BIND_PROTOCOL))
     .value(fdf::wire::NodePropertyValue::WithIntValue(ZX_PROTOCOL_PCI))
     .Build();
properties[1] = fdf::wire::NodeProperty::Builder(arena)
    .key(fdf::wire::NodePropertyKey::WithStringValue("ENABLE_TEST"))
    .value(fdf::wire::NodePropertyValue::WithBoolValue(true))
    .Build();

auto args = fdf::wire::NodeAddArgs::Builder(arena)
                  .name(arena, "sample-child")
                  .properties(properties)
                  .Build();
```

####Defining properties in bind libraries

As we migrate from drivers from the old bind system to the new, we redefine the old properties in bind libraries on the new system. These bind libraries are located in the [src/devices/bind](/src/devices/bind) directory.

Any new properties are expected to be defined in the bind libraries.

#####Properties from the old bind system
Most of the old device property keys and values are defined in [binding_priv.h](/src/lib/ddk/include/lib/ddk/binding_priv.h) and [protodefs.h](/src/lib/ddk/include/lib/ddk/protodefs.h).

`binding_priv.h` contains the hardcoded property keys. Each property key is assigned a unique integer value. In the new bind system, these keys are redefined with a `fuchsia.BIND_` prefix. For instance, `BIND_PROTOCOL` becomes `fuchsia.BIND_PROTOCOL` and `BIND_COMPOSITE` becomes `fuchsia.BIND_COMPOSITE`.

`protodefs.h` contains hardcoded ID values for device protocols.

#####Device protocol bind library
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

####Writing bind rules
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

#####Property key

The integer key names are defined in `binding_priv.h` and are prefixed with “fuchsia”. For example, a key 0x03 means `fuchsia.BIND_COMPOSITE` because `binding_priv.h` contains the following:

```
#define BIND_FLAGS 0x0000          // value of the flags register
#define BIND_PROTOCOL 0x0001       // primary protocol of the device
#define BIND_AUTOBIND 0x0002       // if this is an automated bind/load
#define BIND_COMPOSITE 0x003       // Whether this is a composite device
#define BIND_FIDL_PROTOCOL 0x0004  // primary FIDL protocol of the device
```

#####Property values
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

#####Composite bind rules
The same process is used for composite bind rules. For each node you want to write bind rules for, you can print the device properties and write bind rules for them.

Say you want to write composite bind rules that contain a node that binds to the above example and another node that binds to the following:

```
Name     : acpi-I2C2
Moniker  : root.sys.platform.platform-passthrough.acpi.acpi-I2C2
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
