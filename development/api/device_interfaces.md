<!--
# Fuchsia Device Interface Rubric

The Fuchsia device interfaces are expressed as FIDL interfaces.  These FIDL
definitions should conform to the [FIDL Readability Rubric].
-->

# Fuchsia 设备接口规范

Fuchsia 设备接口表示形式为 FIDL 接口。这些 FIDL 定义应该遵守 [FIDL可读性规范][FIDL Readability Rubric]

<!-- 
## Identifiers

Prefer descriptive identifiers.  If you are using domain-specific abbreviations,
document the expansion or provide a reference for further information.

Every identifier that is defined as part of an interface must be documented with
a comment explaining its interpretation (in the case of fields, types, and
parameters) or behavior (in the case of methods).
-->

## 标识符

应首选描述性的标识符。如果使用 `特定领域` 的术语缩写，需要记录其相关扩展内容或提供引用来获取更多信息。

接口中定义的每个标识符都必须有注释，用来解释其含义（针对成员，类型和参数）或行为（针对方法）。

<!-- 
## Interfaces

All device interfaces must use the `[Layout = "Simple"]` attribute.  This
restriction exists to allow ease of implementing interfaces in any of our
supported languages for driver development.
-->

## 接口

所有设备接口必须使用 `[Layout = "Simple"]` 属性。这个限制允许驱动开发中使用任何支持的语言轻松实现接口。

<!-- 
## Method Statuses

Use a `zx.status` return to represent success and failure.  If a method should not be
able to fail, do not provide a `zx.status` return.  If the method returns multiple
values, the `zx.status` should come first.
-->

## 方法状态

使用 `zx.status` 作为方法返回值，表示方法的成功和失败。若某个方法不应失败，则无需提供 `zx.status` 作为返回值。若方法中有多个返回值，`zx.status` 应第一个返回。

<!--
## Arrays, Strings, and Vectors

All arrays, strings, and vectors must be of bounded length.  For arbitrarily
selected bounds, prefer to use a `const` identifier as the length so that
interface consumers can programmatically inspect the length.
-->

## 数组，字符串和向量

全部数组，字符串和向量必须限定长度。建议使用 `const` 修饰的标识符作为长度边界，方便接口使用者能够以编程的方式检查长度。

<!-- 
## Enums

Prefer enums with explicit sizes (e.g. `enum Foo : uint32 { ... }`) to plain
integer types when a field has a constrained set of non-arithmetic values.
-->

## 枚举

当字段表示非算术值约束集中某个值时，优先使用明确尺寸的枚举（例如: `enum Foo : uint32 { ... }`）而不是普通整数类型。

<!-- 
## Bitfields

If your interface has a bitfield, represent its values using `const` values.
They should be grouped together in the FIDL file and have a common prefix.  For
example:

```
// Bit definitions for Info.features field

// If present, this device represents WLAN hardware.
const uint32 INFO_FEATURE_WLAN = 0x00000001;
// If present, this device is synthetic (i.e. not backed by hardware).
const uint32 INFO_FEATURE_SYNTH = 0x00000002;
// If present, this device will receive all messages it sends.
const uint32 INFO_FEATURE_LOOPBACK = 0x00000004;
```

If FIDL gains bitfield support, this guidance will be updated.
-->

## 位域

若接口中存在位域，使用 `const` 来表示其全部值。位域的值应具有相同前缀，并且在 FIDL 被分组放在一起。例如：

```
// Bit definitions for Info.features field

// If present, this device represents WLAN hardware.
const uint32 INFO_FEATURE_WLAN = 0x00000001;
// If present, this device is synthetic (i.e. not backed by hardware).
const uint32 INFO_FEATURE_SYNTH = 0x00000002;
// If present, this device will receive all messages it sends.
const uint32 INFO_FEATURE_LOOPBACK = 0x00000004;
```

如果 FIDL 得到 bitfield 的支持，此指南将会更新。

<!-- 
## Non-channel based protocols

Some interfaces may negotiate a non-channel protocol as a performance
optimization (e.g. the zircon.ethernet.Device's GetFifos/SetIOBuffer methods).
FIDL does not currently support expressing these protocols.  For now, represent
any shared data structures with `struct` definitions and provide detailed
documentation about participation in the protocol.  Packed structures are not
currently supported.

[FIDL Readability Rubric]: fidl.md
-->

## 基于非通道协议

某些接口为了性能优化可能使用非通道协议（例如 `zircon.ethernet.Device` 的 `GetFifos/SetIOBuffer` 方法），FIDL 目前不支持表示这些协议。

现在，任何由 `struct` 定义，代表共享数据的结构，应提供其在协议中参与部分的详细文档。

目前不支持包装结构（`Packed structures`）。

[FIDL Readability Rubric]: fidl.md