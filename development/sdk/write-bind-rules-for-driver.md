# Write bind rules for a driver

This guide walks through the steps for writing bind rules using the
[`i2c_temperature`][i2c-temperature-sample-driver] sample driver.

For a driver to bind to a [node][drivers-and-nodes] (which represents a hardware
or virtual device), the driver’s bind rules must match the node properties of the
node. In this guide, we’ll write bind rules for the `i2c_temperature` sample
driver so that they match the node properties of the `i2c-child` node.

The `i2c_controller` driver creates a child node named `i2c-child` for testing
the `i2c_temperature` sample driver. We can use this `i2c_controller` driver to
identify the node properties of the `i2c-child` node and write the matching
bind rules for `i2c_temperature`.

Before you begin, writing bind rules requires familiarity with the concepts
in [Driver binding][driver-binding].

The steps are:

1.  [Identify the node properties](#identify-bind-properties).
1.  [Write the bind rules](#write-bind-rules).
1.  [Add a Bazel build target for the bind rules](#add-a-bazel-build-target).

## 1. Identify the node properties {:#identify-bind-properties}

You can identify the node properties of your target node in one of the following ways:

*   [Use the ffx driver list-devices command](#use-ffx-driver-list-devices)
*   [Look up the node properties in the driver source code](#look-up-the-driver-source-code)

### Use the ffx driver list-devices command {:#use-ffx-driver-list-devices}

To print the properties of every node in the Fuchsia system, run the following command:

```posix-terminal
ffx driver list-devices -v
```

This command prints the properties of a node in the following format:

```none {:.devsite-disable-click-to-copy}
Name     : i2c-child
Moniker  : root.sys.platform.pt.acpi.acpi-FWCF.i2c-child
Driver   : None
3 Properties
[ 1/  3] : Key "fuchsia.hardware.i2c.Service"  Value Enum(fuchsia.hardware.i2c.Service.ZirconTransport)
[ 2/  3] : Key fuchsia.BIND_I2C_ADDRESS        Value 0x0000ff
[ 3/  3] : Key "fuchsia.driver.framework.dfv2" Value true
```

The output above shows that the `i2c-child` node has the following node properties:

*   Property key `fuchsia.hardware.i2c.Service` with an enum value of
    `fuchsia.hardware.i2c.Service.ZirconTransport`.
*   Property key `fuchsia.BIND_I2C_ADDRESS` with an integer value of `0xFF`.

### Look up the node properties in the driver source code {#look-up-the-driver-source-code}

When adding a child node, drivers can provide node properties to the node.
Reviewing the source code of the driver that creates your target node as a child
node helps you identify the node properties to include in your bind rules.

The `i2c_controller` driver creates a child node named `i2c-child` to which the
`i2c_temperature` sample driver binds. Examine the source code of the
`i2c_controller` driver to identify which node properties are passed to this
child node:

```cpp {:.devsite-disable-click-to-copy}
{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/i2c_temperature/controller/i2c_controller.cc" region_tag="add_child_properties" adjust_indentation="auto" %}
```

This code shows that the `i2c-child` node is created with the following bind
properties:

*   Property key `fuchsia.hardware.i2c.Service` with an enum value of
    `fuchsia.hardware.i2c.Service.ZirconTransport`.
*   Property key `fuchsia.BIND_I2C_ADDRESS` with an integer value of `0xFF`.

Note: For more information on the `NodeAddArgs` struct used to pass the bind
properties to a child node, see
[NodeProperty and NodeAddArgs structs](#nodeproperty-and-nodeaddargs-structs).

## 2. Write the bind rules {:#write-bind-rules}

Once you know the node properties you want to match, you can use the bind
language to write the bind rules for your driver.

In the previous section, we’ve identified that the `i2c-child` node has the
following node properties:

*   Property key `fuchsia.hardware.i2c` with an enum value of
    `fuchsia.hardware.i2c.Service.ZirconTransport`.
*   Property key `fuchsia.BIND_I2C_ADDRESS` with an integer value of `0xFF`.

To match these properties, the `i2c_temperature` driver declares the following
bind rules:

```none {:.devsite-disable-click-to-copy}
{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/i2c_temperature/driver/i2c_temperature.bind" region_tag="bind_rules" adjust_indentation="auto" %}
```

Integer-based node property keys that start with `BIND_` (defined in
[`binding_priv.h`][binding-prev-h] in the Fuchsia source tree) are old property
keys currently hardcoded in the bind compiler. See the following definition for
`BIND_I2C_ADDRESS` from `binding_priv.h`:

```cpp {:.devsite-disable-click-to-copy}
#define BIND_I2C_ADDRESS 0x0A02
```

When these keys are used in bind rules, they are prefixed with `fuchsia.`.

## 3. Add a Bazel build target for the bind rules {:#add-a-bazel-build-target}

Once you have written the bind rules for your driver, you need to update the
`BUILD.bazel` file to add a build target for the bind rules bytecode using the
`fuchsia_driver_bytecode_bind_rules()` template:

```bazel {:.devsite-disable-click-to-copy}
{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/i2c_temperature/driver/BUILD.bazel" region_tag="bind_rules" adjust_indentation="auto" %}
```

For each library used in your bind rules, add the library as a dependency to the
build target. For example, the `i2c_temperature` sample driver's bind rules use
the `fuchsia.hardware.i2c` library, so the build target includes the bind library
as a build dependency.

To determine which bind libraries are used in the bind rules, you can examine
the driver source code. In the node properties of the `i2c-child` node, the
first property key `fuchsia.hardware.i2c.Service` is from a generated bind
library from the FIDL protocol:

```cpp {:.devsite-disable-click-to-copy}
{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/i2c_temperature/controller/i2c_controller.cc" region_tag="add_child_properties" adjust_indentation="auto" highlight="3,4" %}
```

The prefix `fuchsia_hardware_i2c` implies that this node property’s key and
value are defined in the following header:

```cpp {:.devsite-disable-click-to-copy}
{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/i2c_temperature/controller/i2c_controller.cc" region_tag="bind_imports" adjust_indentation="auto" %}
```

These bind libraries will have corresponding dependencies in the driver's build
rules. See the following `fuchsia.hardware.i2c` dependency in the `i2c_controller`
binary target:

```bazel {:.devsite-disable-click-to-copy}
{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/i2c_temperature/controller/BUILD.bazel" region_tag="cc_binary" adjust_indentation="auto" highlight="12" %}
```

Note: For more information on the generated bind library, see
[Create a new bind library for a driver][bind-library-tutorial].

## Appendices

### NodeProperty and NodeAddArgs structs {:#nodeproperty-and-nodeaddargs-structs}

Node properties are represented by the `NodeProperty` struct in the
`fuchsia.driver.framework` FIDL library:

```fidl {:.devsite-disable-click-to-copy}
/// Definition of a property for a node. A property is commonly used to match a
/// node to a driver for driver binding.
type NodeProperty = table {
    /// Key for the property.
    1: key NodePropertyKey;

    /// Value for the property.
    2: value NodePropertyValue;
};
```

Then the node properties are passed to a child node using the `NodeAddArgs`
struct:

```fidl {:.devsite-disable-click-to-copy}
/// Arguments for adding a node.
type NodeAddArgs = table {
    /// Name of the node.
    1: name string:MAX_NODE_NAME_LENGTH;

    /// Capabilities to offer to the driver that is bound to this node.
    2: offers vector<fuchsia.component.decl.Offer>:MAX_OFFER_COUNT;

    /// Functions to provide to the driver that is bound to this node.
    3: symbols vector<NodeSymbol>:MAX_SYMBOL_COUNT;

    /// Properties of the node.
    4: properties vector<NodeProperty>:MAX_PROPERTY_COUNT;
};
```

<!-- Reference links -->

[i2c-temperature-sample-driver]: https://fuchsia.googlesource.com/sdk-samples/drivers/+/refs/heads/main/src/i2c_temperature/
[drivers-and-nodes]: /concepts/drivers/drivers_and_nodes.md
[driver-binding]: /concepts/drivers/driver_binding.md
[binding-prev-h]: /src/lib/ddk/include/lib/ddk/binding_priv.h
[protodefs-h]: /src/lib/ddk/include/lib/ddk/protodefs.h
[fuchsia-i2c-bind-library]: /src/devices/bind/fuchsia.i2c/fuchsia.i2c.bind
[bind-library-tutorial]: /development/sdk/create-new-bind-library-for-driver.md
