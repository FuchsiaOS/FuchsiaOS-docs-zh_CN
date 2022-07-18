<!---

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

--->

# 驱动绑定

在 Fuchsia 系统中，驱动框架维护了在系统中的一个树形驱动和设备。在这个树形结构中，一个设备代表了操作系统中对某些硬件的访问。驱动程序发布和绑定设备。例如，一个 USB 驱动可以绑定一个 PCI 设备（它的父设备）和发布一个网络设备（它的子设备）。为了决定驱动程序可以绑定哪个设备，每一个驱动程序都有一个绑定规则，同时每一个设备都有一套属性。绑定规则定义了想要绑定的设备属性的匹配条件。

绑定规则和他们参考条件使用特定领域语言来定义。绑定编译器使用该语言并生成对应绑定规则的比特码。语言有两种源文件：规则和库。库被用作在驱动和绑定规则间的共享属性定义。编译器同样从绑定库中生成 FIDL 文件，让驱动可以在代码中查询设备属性。

注意：驱动绑定正在开发中，本文档仅描述当前状态。不是所有驱动都使用这套绑定规则，但是目前正在迁移，以便其全部转换。

关于这个阶段的迁移，需要注意的一点是，不支持在绑定库中定义设备属性键（见下文）。相反，旧驱动绑定系统（[lib/ddk/binding.h](/src/lib/ddk/include/lib/ddk/binding.h)）中的键是可以扩展的。
这些键值在绑定编译器中是硬编码，在 `fuchsia` 命名空间下可用。例如， PCI 供应商 ID 键是`fuchsia.BIND_PCI_VID`。最终，这些硬编码键值将会从这个命名空间中移除，所有绑定属性键值都将在绑定库中定义。

<!---


## The compiler

The compiler takes a list of library sources, and one rule source. For example:

<---

## 编译器

编译器有一组库资源和一个规则资源。例如：

```
fx bindc compile \
  --include src/devices/bind/fuchsia.usb/fuchsia.usb.bind \
  --output tools/bindc/examples/gizmo.h \
  tools/bindc/examples/gizmo.bind
```

<!---

Currently, it produces a C header file that may be included by a driver. The header file defines a
macro:

```
ZIRCON_DRIVER(Driver, Ops, VendorName, Version);
```

 - `Driver` is the name of the driver.
 - `Ops` is a `zx_driver_ops`, which are the driver operation hooks
 - `VendorName` is a string representing the name of the driver vendor.
 - `Version` is a string representing the version of the driver.

For more details, see [the driver development documentation](/docs/concepts/drivers/driver-development).

--->

现在，它生成了一个在驱动内包含的 C 头文件。这个头文件定义了宏：

```
ZIRCON_DRIVER(Driver, Ops, VendorName, Version);
```

 - `Driver `是驱动的名字。
 - `Ops `是一个 `zx_driver_ops`，提供驱动操作钩子函数。
 - `VendorName` 是一个代表驱动供应商的字符串。
 - `Version` 是代表驱动版本的字符串。

需要了解更多细节，请参见[the driver development documentation](/docs/concepts/drivers/driver-development).

<!---

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

--->

## 绑定规则

绑定规则定义了调用驱动`bind()`钩子函数的条件。绑定规则中的每一条声明都是一个设备属性的条件，这些条件必须为真，才能让驱动绑定。如果绑定规则结束运行并且所有的条件都为真，那么设备协调器将会调用驱动的`bind()`钩子函数。

绑定规则被认为是哪一个驱动应该绑定的明确条件表达。正因如此，条件表达式的顺序对于它的最终评价是不相关的。把绑定规则看作是一个布尔公式可能会对理解有帮助。

其中有4种类型的声明：

 - **条件声明** 是相等（或不相等）表达式，其形式为 `<key> == <value>`  (或  `<key> != <value>` )。
 - **接受声明**是一组给定键值的允许接受值。
 - **条件声明**提供简单的分支。
 - **真假声明**可以被用作明确评估绑定规则。

<!---

### Example

This example bind rule can be found at [//tools/bindc/examples/gizmo.bind](/tools/bindc/examples/gizmo.bind).

--->

### 示例

这个示例绑定规则可以在 [//tools/bindc/examples/gizmo.bind](/tools/bindc/examples/gizmo.bind)中找到。

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

<!---

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

--->

### 语言限制

在语言中有一些使用限制，这些限制是为了提供可读性，并确保绑定规则是驱动程序应该绑定条件的简单描述。

- **不允许出现空块**
  空块是否意味着驱动程序将被绑定还是终止，这一点是不明确的。作者应该使用明确的`true` 或者 `false` 来声明。
- **if声明必须有else部分，并且是终端**
  这条限制通过明确的执行分支来提高可读性。由于没有声明可以跟在`if`声明后，所以很容易通过绑定规则追踪到路径。
-  **真和假的陈述必须是其范围内的唯一陈述**
  绑定规则不是必要的程序，评估的顺序也不重要。混合布尔型声明（尤其是 `true`）和其他条件可能导致描述情景不清晰。

<!---

### Grammar

--->

### 语法

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

<!---

An identifier matches the regex `[a-zA-Z]([a-zA-Z0-9_]*[a-zA-Z0-9])?` and must not match any
keyword. The list of keywords is:

--->

标识符匹配正则表达式 `[a-zA-Z]([a-zA-Z0-9_]*[a-zA-Z0-9])?` ，且不得与其他关键字匹配。关键词列表如下：

```
accept
as
else
false
if
true
using
```

<!---

A string literal matches the regex `”[^”]*”`, and a numeric literal matches the regex `[0-9]+` or
`0x[0-9A-F]+`.

The bind compiler will ignore (treat as whitespace) any line prefixed by `//`, and any multiple
lines delimited by `/*` and `*/`.

--->

字符串字面上匹配正则表达式`”[^”]*”`，并且数字上匹配正则表达式 `[0-9]+` 或者`0x[0-9A-F]+`。

绑定编译器将忽略（视为空格）任意前缀为 `//`的句子，多行语句则用 `/*` 和`*/`来定界。

<!---

### Build targets

To declare bind rules within the Fuchsia build system, use the following build target:

--->

### 构建目标

使用以下构建目标，来明确 Fuchsia 构建系统中的绑定规则：

```gn
bind_rules("bind") {
  rules = <bind rules filename>
  output = <generated header filename>
  deps = [ <list of bind library targets> ]
}
```

<!---

For more details, refer to [//build/bind/bind.gni](/build/bind/bind.gni).

--->

更多细节，参见[//build/bind/bind.gni](/build/bind/bind.gni)。

<!---

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

--->

## 测试

绑定编译器支持一个数据驱动的绑定规则单元测试框架，允许你在基于驱动隔离的前提下测试你的绑定规则。一个绑定规则的测试实例包含一个特定设备和一个期待结果，例如，绑定或者退出。测试实例以 JSON 规范文件的形式传递给绑定编译器，编译器通过运行debugger来执行每一条测试实例。

特定 JSON 必须是一系列的测试场景对象，每一个对象包含：

 - `name`，测试场景名字的字符串。
 - `expected `期待结果。必须是 `“match” `或者  `“abort” `。
 - `device` 为一系列键值对描述设备属性。这和 debugger 中 [device specifications](/docs/development/drivers/diagnostics/bind-debugger.md#device-specification)相似。

<!---

### Example

This is an example test case, the full set of tests is at `//tools/bindc/examples/test.json`. This
case checks that the bind rules match a device with the listed properties, i.e. an Intel USB audio
device.

--->

### 示例

以下为一个测试用例示例，完整测试组在 `//tools/bindc/examples/test.json` 。当前用例检查绑定规则是否与一个具有所列属性的设备相匹配，例如，一个 Intel USB 音频设备。

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

<!---

### Build

Define a test build target like so

--->

### 构建

定义了一个构建目标如下

```
bind_test("example_bind_test") {
  rules = <bind rules filename>
  tests = <test specification filename>
  deps = [ <list of bind library targets> ]
}
```

<!---

Alternatively, you can simply add a `tests` argument to your existing `bind_rules` to generate a
test target. It’s name will be the original target’s name plus `_test`. For example, the following
would generate `example_bind_test`.

--->

另外，你可以很简单地添加一个 `tests`  变量在你已有的`bind_rules`中来生成测试目标。它的名字会在原来的目标名字加上`_test`组成。例如，下述会生成`example_bind_test`。

```
bind_rules("example_bind") {
  rules = "gizmo.bind"
  output = “gizmo_bind.h”
  tests = "tests.json"
  deps = [ "//src/devices/bind/fuchsia.usb" ]
}
```

<!---

### Run

If you have defined a build target for your test then you can run the tests as usual with fx test.

--->

### 运行

如果你已经定义了你的测试构建目标，那么接下来你可以像往常一样用 fx test 运行这些测试。

```
fx test example_bind_test
```

<!---

Otherwise you can run the bind tool directly. For example:

--->

否则也可以直接运行绑定工具，例如：

```
fx bindc test \
  tools/bindc/examples/gizmo.bind \
  --test-spec tools/bindc/examples/tests.json \
  --include src/devices/bind/fuchsia.usb/fuchsia.usb.bind
```

<!---

## Bind libraries {#bind-libraries}

A bind library defines a set of properties that drivers may assign to their children. Also,
bind rules may refer to bind libraries.

--->

## 绑定库

一个绑定库定义了一组驱动将要分配给它的子驱动的属性。同样，绑定规则可以适用于绑定库。

<!---

### Namespacing

A bind library begins by defining its namespace:

--->

### 命名空间

绑定库首先要定义它的命名空间：

```
library <vendor>.<library>;
```

<!---

Every namespace must begin with a vendor and each vendor should ensure that there are no clashes
within their own namespace. However, the language allows for one vendor to extend the library of
another. Google will use `fuchsia` for public libraries.

Any values introduced by a library are namespaced. For example, the following library defines a
new PCI device ID `GIZMO_VER_1`.

--->

每一个命名空间都必须以供应商作为开始，并且每一个供应商应当保证在它们自己的命名空间内没有冲突。尽管如此，语言中允许一个供应商来扩展另一个库。Google将使用 `fuchsia` 作为公共库。

库引入的任意值都是在命名空间内的。例如，下述库定义了一个新的 PCI 设备 ID 为`GIZMO_VER_1`。

```
library gizmotronics.gizmo;

using fuchsia.pci as pci;

extend uint pci.device_id {
  GIZMO_VER_1 = 0x4242,
};
```

<!---

To refer to this value the driver author should use the fully qualified name, as follows.

--->

为了查询这个值，驱动程序作者应该使用完全合规的名称，如下所示：

```
using fuchsia.pci as pci;
using gizmotronics.gizmo;

pci.device_id == gizmotronics.gizmo.device_id.GIZMO_VER_1
```

<!---

### Keys and values

Device property definitions look similar to variable declarations in other languages.

--->

### 键值和数据

设备属性定义看起来与其他语言中的变量声明类似。

```
<type> <name>;
Or:
<type> <name> {
  <value>,
  <value>,
  …
};
```

<!---

A bind library may also extend properties from other libraries.

--->

绑定库同样也可以扩展其他库的属性。

```
extend <type> <name> {
  <value>,
  …
};
```

<!---

Each key has a type, and all values that correspond to that key must be of that type. The language
supports primitive types: one of `uint`, `string`, or `bool`; and enumerations (`enum`). When
defining keys you should prefer enumerations except when values will be provided by an external
source, such as hardware.

When definining a primitive value use the form `<identifier> = <literal>`, and for enumerations
only an identifier is necessary. It is valid to define multiple primitive values with the same
literal.

--->

每一个键值都有一个类型，并且所有与该键对应的值必须是该类型的。语言支持的元类型：`uint`，`string`或者`bool`之一；和枚举（`enum`）。当定义键时，你应该更优先使用枚举，除非这个值是由外部来源提供，例如硬件提供。

当使用结构 `<identifier> = <literal>` 来定义原始值时，对于枚举来说，只有一个标识符是有必要的。使用相同字面定义多个原始值也是有效的。

<!---

### Grammar

--->

### 语法

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

<!---

An identifier matches the regex `[a-zA-Z]([a-zA-Z0-9_]*[a-zA-Z0-9])?` and must not match any
keyword. The list of keywords is:

--->

标识符匹配正则表达式 `[a-zA-Z]([a-zA-Z0-9_]*[a-zA-Z0-9])?`，且不得与其他关键字相同，关键字列表如下：

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

<!---

A string literal matches the regex `”[^”]*”`, and a numeric literal matches the regex `[0-9]+` or
`0x[0-9A-F]+`.

The bind compiler will ignore (treat as whitespace) any line prefixed by `//`, and any multiple
lines delimited by `/*` and `*/`.

--->

字符串字面上匹配正则表达式`”[^”]*”`，并且数字上匹配正则表达式 `[0-9]+` 或者`0x[0-9A-F]+`。

绑定编译器将忽略（视为空格）任意前缀为 `//`的句子，多行语句则用 `/*` 和`*/`来定界。

<!---

### Build targets

To declare a bind library within the Fuchsia build system, use the following build target:

--->

### 构建目标

使用以下构建目标，来明确 Fuchsia 构建系统中的绑定库：

```gn
bind_library(<library name>) {
  source = <bind library filename>
  public_deps = [ <list of bind library targets> ]
}
```

<!---

For more details, refer to [//build/bind/bind.gni](/build/bind/bind.gni).

--->

更多细节，请参考[//build/bind/bind.gni](/build/bind/bind.gni)。



