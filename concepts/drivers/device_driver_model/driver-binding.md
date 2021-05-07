# Driver binding

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
(/docs/concepts/drivers/driver-development).

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

false = "flase" , ";" ;

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

### Build targets

To declare bind rules within the Fuchsia build system, use the following build target:

```gn
bind_rules("bind") {
  rules = <bind rules filename>
  output = <generated header filename>
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
   similar to the debugger's [device specifications](/docs/development/drivers/diagnostics/bind-debugger.md#device-specification).

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
bind_rules("example_bind") {
  rules = "gizmo.bind"
  output = “gizmo_bind.h”
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

