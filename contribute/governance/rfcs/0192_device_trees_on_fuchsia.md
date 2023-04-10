<!-- Generated with `fx rfc` -->

<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0192" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
{# Fuchsia RFCs use templates to display various fields from _rfcs.yaml. View the #}
{# fully rendered RFCs at https://fuchsia.dev/fuchsia-src/contribute/governance/rfcs #}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Summary

[Flattened Device Trees][dt-specification] ("FDT", or just "devicetrees") are a
format used widely to describe the layout of hardware on a board. They serve a
broadly similar purpose to [ACPI][acpi-rfc], although they operate at a much
lower level of abstraction, pushing more complexity into the operating system.
The FDT specification defines a binary devicetree blob ("DTB") format, and a
source format ("DTS") from which devicetree blobs are compiled.

This RFC proposes a pragmatic approach to introduce support for devicetrees
within Fuchsia without committing to devicetrees (either their layout or binary
format) as an ABI. Rather, the exact ABI of a devicetree is left to board
drivers to define between themselves and the firmware. Also note that this RFC
only concerns the use of devicetrees by board drivers. If or when the kernel
parses devicetrees a separate RFC will describe the format of said devicetrees.

## Motivation

Board drivers alone are not a sustainable way to continue growing Fuchsia's
hardware support, for a number of reasons:

1.  We would like to be able to iterate on hardware configuration without
    reassembling/rebuilding the Fuchsia image.
2.  Devicetree is widely used by developers, and we want to meet developers
    where they are. Devicetree also has a relatively robust ecosystem.
3.  The way they are written today makes them unsuitable for collaboration
    between multiple organisations.
4.  They cause duplicated effort between firmware and the board driver to detect
    available hardware in the system.
5.  They do not scale well when supporting multiple related boards (i.e. sharing
    a SoC or SoC family).
    *   To support multiple boards, one must either add conditionals throughout
        the board driver, or have multiple board drivers.
    *   While it is possible to do this in a clean way, it much easier to end up
        with spaghetti code than it is with devicetrees.
    *   Devicetrees will make it easier to support multiple boards with a single
        Fuchsia image (although note that we are not aiming for a "universal"
        Fuchsia image at this stage).

Devicetrees offer solutions to these issues, by providing a single source of
truth for hardware configuration that is easy to manipulate separately from the
rest of the system (easy to modify both by boot firmware and by groups working
on bringup), and also easy to compose (allowing for easy sharing of
configuration between boards based on the same SoC family).

Note that the goal of this RFC is not to eliminate board drivers entirely. Board
drivers are still a central part of the Fuchsia driver topology -- this RFC
proposes using devicetrees to supplement board drivers in order to increase
their flexibility. Anything that is inexpressible with devicetrees can still be
implemented within a board driver.

## Stakeholders

*Facilitator:*

cpu@google.com

*Reviewers:*

-   surajmalhotra@google.com
-   curtisgalloway@google.com
-   gkalsi@google.com
-   bradenkell@google.com
-   cja@google.com
-   dpursell@google.com

*Consulted:*

-   aaronwood@google.com
-   mcgrathr@google.com

*Socialization:*

A briefer, high-level version of this proposal was shared internally.

## Glossary

*   **board driver** - A driver that tells the driver framework about hardware
    present on the system. There is only a single board driver instance on a
    system.
*   **firmware** - Part of the system that runs when it is first powered on,
    responsible for loading Zircon and booting it.

## Design

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD",
"SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be
interpreted as described in
[IETF RFC 2119](https://tools.ietf.org/html/rfc2119).

### Role of Devicetree

Devicetrees are only intended to describe hardware layout information. Product
configuration MUST be done via [Product Assembly][product-assembly]. A
devicetree should only contain information that is true about a board regardless
of the product it is running, as opposed to configuring behaviour of drivers
that may vary, like pre-allocated memory buffer sizes or partition maps.

### Devicetree vs ACPI

We prefer ACPI to devicetree. If a board supports ACPI, it should be booted
using ACPI. Devicetrees MUST NOT be used on boards where there is ACPI support.
This is because Fuchsia provides a single ACPI board driver for each supported
architecture. The higher level of abstraction offered by ACPI and the degree of
standardisation make it much more tractable to support.

### Compatibility

Our devicetree bindings will not attempt to maintain compatibility with Linux or
any other OS, particularly for device driver bindings. This is simply because
unlike ACPI, devicetrees are not widely standardised and the exact layout and
meaning of the bindings varies between OS versions. We will however adopt the
source and binary formats described in the [spec][dt-specification].

Fuchsia as a platform will not attempt to specify the exact format of device
trees, nor will we seek to support every board with a single board driver -
rather, we will define a set of bindings that provide support for common
functionality (like interrupts, GPIOs, buses), based on the specification. Board
drivers that consume devicetree SHOULD support the bindings defined in this RFC,
but they MAY define their own bindings to support additional functionality as
needed.

We will also provide a way for devicetrees to nominate the board driver they are
written to target (e.g. via a property of the root node like
`fuchsia,board-driver = "vim3-devicetree"`) which board drivers SHOULD use to
ensure they are parsing a compatible devicetree.

Additionally, we will define an interface that board drivers MUST expose to
downstream drivers. This interface will be very similar to the
[ACPI interface][acpi-rfc], and we will seek to bring ACPI and devicetree
in-line with one another so that drivers can be truly agnostic between the two.

### Passing the Devicetree to Fuchsia.

We already have a ZBI item with type `ZBI_TYPE_DEVICETREE`. The content of this
ZBI item is the devicetree blob. The only change we need to make here is to
update the documentation to reflect that it may be consumed by the board driver.

A board MAY choose to use devicetrees. If this is the case it is expected that
the bootloader will know this fact and the bootloader SHOULD pass the devicetree
as a `ZBI_TYPE_DEVICETREE` item. If needed, a board MAY include devicetrees
directly in the ZBI when it is assembled (e.g. during bringup before the
bootloader has built-in support for ZBIs).

Generally bootloaders SHOULD load the device tree from non-volatile storage,
verify its authenticity, and append it to the ZBI at runtime. However, they MAY
use an alternative mechansim (like compiling it into the bootloader) if it is
desired, and MAY skip verification if appropriate (e.g. for developer boards).

### Fuchsia platform implementation

We will provide a library for board drivers wishing to implement this set of
bindings. This library will leverage the existing [devicetree parser][dt-parser]
that is in the tree. Additionally, we will implement a reference board driver
and device tree that makes use of this library to boot a real system. The former
will be part of the SDK and live in the tree, but the latter may eventually move
out of tree.

#### Readability of Devicetrees

One criticism of devicetrees is that the source format can be quite opaque,
since the devicetree compiler has no support for named constants. A common way
of working around this is to use C header files containing `#define` statements
which declare human-readable names for constants, and then including these
header files so they can be used within the devicetree source. We will implement
this approach within fuchsia.git and in the future we will also support
generating header files like this from bind libraries.

Broadly, drivers that support being used with devicetree will have a directory
containing headers for use by devicetree source files. These headers will define
values used when referencing resources exposed by the driver -- a GPIO driver
might have a constant for `PULL_UP`, `PULL_DOWN`, etc. Devicetree source file
build rules will then depend on the drivers they use, which will be used to
validate the devicetree (see [Documentation](#documentation)) and also add to
the include paths provided to the C preprocessor.

#### Reviewability of devicetrees

To aid reviewability of the devicetree, we will also introduce devicetree
"golden" files. These golden files will be the output of compiling the device
tree sources into a binary and then back to source code -- that way the impact
of source changes on a final devicetree will be clear. When the board driver and
devicetree are compiled, we will compile and decompile the devicetree source,
and if the output differs to the golden we will fail the build.

The motivation for this feature is, for instance, in a case where several boards
include a common devicetree source file (e.g. something that's defined by the
SoC). It may not be immediately obvious to a reviewer what the ramifications of
changing the "shared" devicetree file are. Golden files make it obvious what the
final output of each devicetree is when it is used on the device.

### Nodes

Each devicetree node will correspond to a Fuchsia device node. Fuchsia device
nodes will be composite nodes that are children of nodes representing the device
tree node, and of nodes representing the resources consumed by the device (e.g.
an I2C device, a GPIO, etc). This is similar to the approach used by
[ACPI][acpi-rfc]. Note that the below examples are not exhaustive, they merely
aim to illustrate the types of resources that we will support initially. Other
resource types can be added in the future as needed. Particularly, the FIDL
interfaces and bind rules described in this document are not intended to be
final -- rather they are strawman examples designed to indicate what
functionality an interface will support. It is expected that these interfaces
will undergo more thorough review in the future.

### Device metadata node

The device metadata node will expose the following FIDL protocol. This protocol
is intended to be usable as a generic "unstructured metadata" protocol for
device drivers to consume.

```
library fuchsia.hardware.metadata;

using zx;

/// Maximum length of a property key.
const PROP_MAX: uint32 = 128;

/// Reasonable maximum for amount of data in a byte array.
const MAX_BYTE_ARRAY_LENGTH: uint32 = 4096;

/// Reasonable maximum length for a string.
const MAX_STRING_LENGTH: uint32 = 4096;

/// Reasonable maximum number of entries for a string array.
const MAX_STRING_ARRAY_LENGTH: uint32 = 4096;

/// Maximum number of properties in a dictionary.
const MAX_PROPERTIES: uint32 = 128;

type Value = flexible union {
    /// True if property is present but has no value.
    1: present bool;
    /// Little-endian 32-bit value.
    2: u32 uint32;
    /// Little-endian 64-bit value.
    3: u64 uint64;
    /// String or string array.
    4: string vector<string:MAX_STRING_LENGTH>:MAX_STRING_ARRAY_LENGTH;
    /// Byte array
    5: bytes vector<uint8>:MAX_BYTE_ARRAY_LENGTH;
    /// Child node.
    6: node Dictionary;
};

type Property = flexible table {
    /// Name of the property.
    1: name string:PROP_MAX;
    /// Value of the property.
    2: value Value;
};

type Dictionary = struct {
    /// List of properties. There may be duplicate property names, in which case the first one should win.
    1: properties vector<Property>:MAX_PROPERTIES;
};


protocol NodeMetadata {
    /// Get unstructured metadata belonging to this node.
    GetMetadata() -> (Dictionary) error zx.status;

};
```

### Bindings

Each of these sections will explain the device-tree binding and the way the
functionality will be used on Fuchsia.

#### Conventions and standard properties

We will use the conventions described in section 2.2 of the
[specification][dt-specification], version 0.3.

Of the standard properties defined in section 2.3, we will likely support the
following: *compatible*, *phandle*, *status*, *#address-cells*, *#size-cells*,
*reg*, *ranges*. We omit *virtual-reg* because the state of the MMU at the time
the bootloader jumps to the kernel is irrelevant to the board driver (since it's
in userspace). We omit *dma-ranges* and *dma-coherent* because these properties
do not have an immediate equivalent on Fuchsia.

On Fuchsia:

*   *compatible* - the first value in the compatible array will be exposed as
    the bind property `fuchsia.devicetree.first_compatible`. Other values might
    be used in the future when a more sophisticated binding system (with
    priorities) is available.
    *   Additionally, we will allow nodes to define their own bind properties,
        likely through namespaced properties (e.g. `bind,<prop-name> = <value>`
        would correspond to bind property `prop-name` with value `value`).
*   *phandle* - will be used to uniquely identify device nodes within the device
    tree. Generated by the devicetree compiler, and only used at runtime.
*   *status* - may be used within the board driver, to control whether or not
    nodes are published. We will investigate the desirability of omitting
    *status* from our supported bindings during implementation of this RFC, and
    may choose to omit it if it greatly improves clarity of the devicetree
    source files.
    *   Specifically, we may want to encourage composing devicetree source files
        together (splitting a single SoC definition over multiple files and
        including only the necessary ones in the final board file) rather than
        including them in one and disabling/enabling IP blocks via the *status*
        property.
*   *#address-cells*, *#size-cells* - will be used within the board driver to
    determine size of other values.
*   *reg* - for nodes that are addressable from the root of the tree, will be
    used by the board driver to determine physical memory regions. These memory
    regions will be made available to child nodes via a `GetMmio(int index)`
    FIDL call, likely via the platform bus driver.
*   *ranges* - used within the board driver to determine how child address
    ranges map into the parent.

#### Interrupts

Interrupts bindings will be as defined by section 2.4 of the
[specification][dt-specification].

The board driver will provide the following metadata to each interrupt
controller driver:

```
/// Data for an individual interrupt.
type InterruptData = flexible table {
    /// The cells associated with an interrupt. The size of this vector
    /// is determined by the `#interrupt-cells` property.
    1: cells vector<uint32>:MAX;
};

/// Data for an interrupt controller.
type InterruptControllerData = flexible table {
    /// This vector contains all of the interrupts discovered by the board driver.
    1: configuration vector<InterruptData>:MAX;
};
```

Each interrupt driver will then publish a node per interrupt. The nodes will
have the following bind rules:

```
fuchsia.devicetree.node_type == INTERRUPT;
fuchsia.devicetree.phandle == <phandle of interrupt controller devicetree node>;
fuchsia.devicetree.cellN == <nth configuration cell>;
```

Note that `phandle` is a unique value per devicetree and should only be used in
bind rules generated by the board driver.

The nodes will publish the following FIDL service:

```
library fuchsia.hardware.interrupt;

protocol Provider {
    /// Return the interrupt represented by this provider.
    Get(struct {}) -> (resource struct { irq zx.interrupt; }) error zx.status;
};
```

It is expected that in the majority of cases the interrupt will correspond
directly to a physical hardware interrupt, but in some cases (e.g. GPIO) the
interrupts may be multiplexed and an intermediary driver will be involved. We
may want to do work in the future to allow for these interrupts to be handled
from within the kernel, but such work is out of scope for this document.

#### I2C, SPI, UART, and other peripheral buses

Devices on these buses are modelled as children of their controller devices. To
determine the controller's bus type, we will define an extra "compatible" value
for each supported bus type. For example, an i2c controller would have its last
compatible value be `fuchsia,i2c-controller`.

We will define bus-specific devicetree properties that are used to populate the
metadata consumed by Fuchsia bus drivers. These properties will likely align to
their equivalents used by other operating systems, but the exact format will be
documented in //docs.

The nodes representing I2C/SPI/etc buses will be called `i2c000`, `spi000`, etc.
Note that even though devicetree is only capable of representing a single parent
of this type, we still number the parent to remain compatible with ACPI.

#### GPIOs

GPIOs primary difference from other resources is that they have some amount of
client configuration. For example, a client might want to enable an internal
pull-up/down resistor, or the pin might be active high/low.

A GPIO controller node must have a `#gpio-cells` property, defining the number
of cells used to identify a GPIO exposed by the node. Additionally, it must have
the `gpio-controller` property, which is empty.

If the GPIO controller node has no `compatible` string, then the parent of the
GPIO controller node will instead be provided the metadata pertaining to its
child nodes. This is for SoCs where a single logical "controller" exposes
multiple groups of pin banks.

We will define the following metadata so that controller drivers know the
configuration expected of each pin instance. Note that this metadata is not
devicetree-specific. We will define a few conventions for device trees though:

1.  The last configuration cell contains the `GpioConfiguration` flags.
2.  The other cells are put in the `data` vector of each pin's metadata.

The metadata type will likely look like this:

```
library fuchsia.hardware.gpio;

using fuchsia.driver.framework;

/// Maximum number of properties a pin can have.
const uint32 MAX_PIN_PROPERTIES = 32;
/// Maximum number of configuration cells a driver can have.
const uint32 MAX_PIN_CELLS = 8;

/// GPIO configuration flags.
type GpioConfiguration = flexible bits {
    /// If this bit is set, GPIO is active-low. Otherwise, GPIO is active-high.
    GPIO_ACTIVE_LOW = 0x1,
    // more properties here as appropriate.
};

type GpioPinMetadata = flexible table {
    /// Properties the published device should have.
    1: expected_properties vector<fuchsia.driver.framework.NodePropertyValue>:MAX_PIN_PROPERTIES;
    /// Arbitrary, driver-specific data. This will likely encode the pin number.
    2: data vector<uint32>:MAX_PIN_CELLS;
    /// GPIO configuration flags.
    3: flags GpioConfiguration;
};

type GpioMetadata = flexible table {
    /// Standard metadata for GPIO pins.
    /// The GPIO controller driver should publish a GPIO pin node for each of these.
    1: pins vector<GpioPinMetadata>:MAX;
};
```

Additionally, GPIO controllers are frequently part of a larger "pin controller",
where a group of GPIO controllers are managed by a single driver. To support
this, if a driver won't bind to the individual GPIO controller node (i.e. no
compatible or other bind properties are set on it), we will provide this
metadata to the parent node.

Each published pin should have the following properties:

```
fuchsia.devicetree.node_type == GPIO;
fuchsia.devicetree.phandle == <phandle>;
fuchsia.devicetree.cellN == <nth configuration cell>;
```

Devicetree nodes that want to use GPIOs should use properties named like
`<name>-gpios`. These will end up as fragment parents named `gpio-<name>NNN`. If
multiple `gpios` are named, they will be assigned numbers based on their index.
These nodes should expose the `fuchsia.hardware.gpio.Gpio` protocol.

For a driver to bind to a GPIO, it would then need bind rules like:

```
node "gpio" {
    fuchsia.resource.name == "example";
    fuchsia.resource.index == 0; // zeroth gpio in the "example-gpios" list.
    fuchsia.hardware.gpio.Gpio == ZirconTransport;
}
```

#### Voltage Regulators

We will treat voltage regulators similarly to GPIOs, however no metadata will be
provided to regulator drivers. Regulator nodes will be inferred from their uses
in properties labelled `-supply`.

A single regulator is expected to correspond to a single devicetree node, so no
extra configuration is needed.

Regulator drivers are expected to publish nodes with the following properties:

```
fuchsia.devicetree.node_type == REGULATOR;
fuchsia.devicetree.phandle == <phandle>;
```

A supply node can only reference a single regulator. These fragments should
expose the `fuchsia.hardware.vreg.Vreg` protocol.

Regulator consumers will then use bind rules like:

```
node "regulator" {
    fuchsia.resource.name == "vdd"; // equivalent to vdd-supply in devicetree.
    fuchsia.hardware.vreg.Vreg == ZirconTransport;
}
```

#### Clocks and other resources

These devices can have more than one instance assigned to them.

Clocks are quite similar to voltage regulators. No metadata is needed, since the
clock driver should know the number of clocks it exports. Clock controller nodes
should define `#clock-cells`, the number of cells needed to identify a clock
device.

They should have the following properties:

```
fuchsia.devicetree.node_type == CLOCK;
fuchsia.devicetree.phandle == <phandle>;
fuchsia.devicetree.cellN == <nth configuration cell>;
```

Clocks consumers can define clocks by specifying an array of clock identifiers
in the `clocks` property, and an array of optional names in the `clock-names`
property.

Devices which use clocks will have fragment parents named `clk-<name>` (if
`clock-names` is present), or `clk-NNN`, where `NNN` is the index of the clock
in the `clocks` array. Each of these fragments will expose the
`fuchsia.hardware.clock.Device` protocol.

To bind to a clock node, a driver would use bind rules like:

```
// If clock-names is expected:
node "clock-input" {
    fuchsia.resource.name == "input";
    fuchsia.hardware.clock.Device == ZirconTransport;
}

// If clock-names is not expected:
node "clock-input" {
    fuchsia.resource.index == 0;
    fuchsia.hardware.clock.Device == ZirconTransport;
}
```

### End-to-end example

As an example, let's take the [vim3 USB PHY][vim3-usb]. Strictly speaking, the
Fuchsia driver controls two USB PHYs and a special mux that routes one of the
PHYs between host and peripheral mode.

If we focus on the "mux" part of this equation specifically, it has the
following resources:

*   One MMIO region.
*   One interrupt.
*   One clock.

Its devicetree node would look something like this:

```
/ { // Root node of the devicetree.
    // 64 bit addresses.
    #address-cells = <2>;
    #size-cells = <2>;
    #interrupt-parent = <&gic>;

    usb-mux@ffe09000 {
        compatible = "amlogic,g12b-usb-mux";
        reg = <0x0 0xffe09000 0x0 0xa0>; // MMIO region.
        interrupts = <GIC_SPI 16 INTERRUPT_MODE_EDGE_HIGH>; // Interrupt.
        clocks = <&clk CLK_G12B_USB>; // Clocks.
        clock-names = "usb"; // Clock names.
    };

    gic: interrupt-controller@ff000000 {
        compatible = "arm,gic-400";
        interrupt-controller; // This is an interrupt controller.
        #interrupt-cells = <3>; // 3 32-bit values are used to identify interrupt on this controller.
    };
};
```

Given this node layout (looking specifically at the `usb-mux` node), the
devicetree board driver will do the following:

*   See the `reg` node and add the MMIO region from `0xffe09000...0xffe090a0` to
    the platform device's definition.
*   See the interrupt resource, and add an interrupt node to the device group
    with properties described in the [interrupts](#interrupts) section.
*   See the clock resource, and add a clock node to the device group with
    properties described in the [clock](#clocks-and-other-resources) section.

To bind to this touchscreen device, a device driver's composite bind file would
look like:

```
composite g12b_usb_mux;

primary node "pdev" {
    fuchsia.devicetree.first_compatible == "amlogic,g12b-usb-mux";
    fuchsia.hardware.platform.device.PDev == ZirconTransport;
}

node "clock-usb" {
    fuchsia.hardware.clock.Device == ZirconTransport;
    fuchsia.resource.name == "usb";
}

node "interrupt" {
    fuchsia.hardware.interrupt.Provider == ZirconTransport;
    fuchsia.resource.index == 0;
}
```

Note that the bind properties seen by the driver differ from the ones used to
match the composites because we'll leverage the transformation APIs offered by
device groups.

## Implementation

We will likely introduce the code implementing this RFC over a number of CLs.
The implementation process itself is unlikely to be particularly complicated
since at this stage no APIs will be added to the out-of-tree SDK.

Eventually these APIs will be part of the driver SDK, but first we plan to prove
out the design in-tree where it is easier to iterate rapidly.

## Performance

This RFC is unlikely to add any significant runtime performance overhead, as the
majority of logic proposed occurs once at boot.

## Security considerations

The devicetree is a binary blob provided by system firmware or included in the
ZBI and is intrinsically a trusted part of the system. Additionally, drivers can
only access resources that belong to their node. While they can inspect names
and properties of child nodes, they are unable to access any of the resources
their child nodes own by the nature of the system -- resource access is
explicitly granted by the board driver via fragment parents of composite nodes.

However, drivers will be able to parse properties from the devicetree, which may
leave them vulnerable to exploitation if not carefully written. In particular,
we will be providing the ability for drivers to define schemas for the
configuration data they expect to be included in the device tree, but this
parsing code could be vulnerable to exploitation, so we should ensure that it is
easy to test (or fuzz) drivers that consume this kind of configuration data.

The board driver is not part of the Fuchsia platform, so any security fixes to
the devicetree library or to the board driver would need to be ingested by the
owners of said board drivers and a new board driver would need to be re-built.

## Privacy considerations

Access to the devicetree will be restricted, as it could be used to fingerprint
the device.

## Testing

Initially, we will use unit tests to verify individual functionality of the
board driver. Eventually we will do integration testing using arbitrary device
trees to create a device topology.

We will also fuzz the devicetree parser.

We will also provide tooling to validate a devicetree against known
[devicetree schemas][dt-schema].

## Documentation

We will document Fuchsia's use of devicetrees and our approach to supporting
them in a document on fuchsia.dev. Specifically, we want board owners to
understand how they can use devicetree on a board they're trying to bring up -
understand how our approach differs from other operating systems and how to
actually make things work.

Additionally, we will require that drivers specify the
[devicetree schemas][dt-schema] they expect to use and include them in the
build. To ensure documentation matches expectations and devicetree correctness
we will implement schema validation so that production devicetrees can be
validated against these schemas.

## Drawbacks, alternatives, and unknowns

### Drawback: Readability

Devicetrees are comparatively difficult to read. In particular, the pattern of
composing different source files together to produce a final output means that
you need to parse all source files to understand the final result.

We hope to address this with tooling (namely golden files), but this is likely
to be a point of friction when using devicetrees.

### Alternative: Keep using pure board drivers

We could continue to express all hardware configuration programmatically in
board drivers, which requires writing code and rebuilding the driver to express
changes. As discussed above, board drivers are not a sustainable approach to
keep expanding Fuchsia's board ecosystem.

### Alternative: Use a different mechanism to describe hardware

We could use another data-based mechanism to describe hardware layout. The main
benefits of devicetree over a hypothetical alternative are:

*   Widely used and familiar to participants in the hardware ecosystem.
*   Common building blocks (e.g. interrupt support) are well-defined and easily
    adoptable.
*   We are able to leverage existing tooling.

The main drawbacks of devicetrees are:

*   No clear cross-operating-system standardisation efforts. Most activity is
    centred on Linux.
    *   Many existing bindings are written solely for a Linux driver.
*   Easy to abuse for more than hardware configuration (e.g. product-specific
    configuration).
*   Complexity when multiple devicetrees are overlaid and alter each other's
    nodes (e.g. `status = "disabled"`/`status = "okay"` can disable/enable a
    node).

Additionally, no obvious alternative to devicetree has arisen (most operating
systems either use devicetrees or board files to solve this problem). Device
trees have been around since 1994 and is firmly established as the primary way
of describing hardware.

### Alternative: Define a stable devicetree schema, mandated by the Fuchsia platform

We could define a single devicetree schema that is supported by a single
universal "devicetree" board driver, and make that the official way of using
devicetrees.

This would get us closer to the goal of having a single Fuchsia image for all
systems that use devicetree, but the wide variance in devicetrees (compared to
something like ACPI) makes this a very large undertaking and one that is likely
to require a lot of evolution.

This RFC does not rule out adoption of such a schema in the future, but we think
that for now the most pragmatic approach to adopting devicetree is the one
outlined above.

### Alternative: Stabilize devicetree bindings with the broader ecosystem

We could work with other groups that use devicetrees to standardize bindings
across the wider ecosystem.

This would likely be a multi-year effort that is outside the scope of this RFC.
It is unlikely that Fuchsia would be able to lead such an effort simply because
the vast majority of devicetrees are not written for Fuchsia.

### Alternative: Roll our own DSL

Rather than using devicetrees, we could roll our own DSL that integrates with
FIDL and bind rules. This might be something we want to do in the future, but at
the time of writing there are no concrete implementation ideas for what this DSL
would look like. If we do decide a DSL is necessary in the future, it is likely
that lessons from implementing this RFC will be invaluable in influencing its
design.

## Prior art and references

*   [Linux and the Devicetree](https://docs.kernel.org/devicetree/usage-model.html) -
    overview of how Linux uses the devicetree.
*   [Devicetrees for ARM](https://elinux.org/images/d/d8/Vwool-device_trees_arm.pdf) -
    a talk from 2009, when Linux was considering adopting devicetrees for ARM.
*   [Devicetree: Past, Present, Future](https://elinux.org/images/0/06/ELCE_2019_DeviceTree_Past_Present_Future.pdf) -
    a talk from 2019 about the state of devicetrees on Linux.
*   [devicetree.org](https://www.devicetree.org/) - devicetree specification
    organisation.
*   [ACPI vs DT](https://elinux.org/images/f/f8/ACPI_vs_DT.pdf) - a talk about
    unifying ACPI and devicetree interfaces in Linux.

[acpi-rfc]: /docs/contribute/governance/rfcs/0112_acpi_support_on_x86.md
[dt-parser]: https://cs.opensource.google/fuchsia/fuchsia/+/main:zircon/kernel/lib/devicetree/include/lib/devicetree/devicetree.h;drc=4d5a960a88ecfa5b1763111cc88b47838f799e2f
[dt-schema]: https://github.com/devicetree-org/dt-schema
[dt-specification]: https://github.com/devicetree-org/devicetree-specification/releases/tag/v0.3
[product-assembly]: /docs/contribute/roadmap/2021/product_assembly.md
[vim3-usb]: https://cs.opensource.google/fuchsia/fuchsia/+/main:src/devices/board/drivers/vim3/vim3-usb.cc;drc=937a626e8295964db6903e6b5a10c34ef5e10484
