<!--
# Fuchsia Device Interface Rubric

The Fuchsia device interfaces are expressed as FIDL protocols.  These FIDL
definitions should conform to the [FIDL Style Rubric][fidl-style] and
[FIDL API Rubric][fidl-api].
-->

# Fuchsia 设备接口规范

Fuchsia 设备接口表示形式为 FIDL 协议。这些 FIDL 定义应符合 [FIDL Style Rubric][fidl-style] 和 [FIDL API Rubric][fidl-api].
<!-- 
## Identifiers

Prefer descriptive identifiers.  If you are using domain-specific abbreviations,
document the expansion or provide a reference for further information.

Every identifier that is defined as part of a protocol must be documented with
a comment explaining its interpretation (in the case of fields, types, and
parameters) or behavior (in the case of methods).
-->

## 标识符

应首选描述性的标识符。如果使用特定领域的术语缩写，需要描述其相关扩展内容或提供引用来获取更多信息。

接口中定义的每个标识符都必须有注释，用来阐释其含义（针对成员，类型和参数）或行为（针对方法）。

<!-- 
## Protocols

All device interface protocols must use the `[Layout = "Simple"]` attribute.  This
restriction exists to allow ease of implementing protocols in any of our
supported languages for driver development.
-->

## 协议

所有设备协议必须使用 `[Layout = "Simple"]` 属性。这个限制允许在驱动开发中使用任何支持的语言轻松实现接口。

<!-- 
## Method Statuses

Use a `zx.status` return to represent success and failure.  If a method should not be
able to fail, do not provide a `zx.status` return.  If the method returns multiple
values, the `zx.status` should come first.
-->

## 方法状态

使用 `zx.status` 作为方法返回值，表示成功和失败。当某个方法不可能失败，则无需返回 `zx.status`。若方法中有多个返回值，`zx.status` 应第一个返回。

<!--
## Arrays, Strings, and Vectors

All arrays, strings, and vectors must be of bounded length.  For arbitrarily
selected bounds, prefer to use a `const` identifier as the length so that
protocol consumers can programmatically inspect the length.
-->

## 数组，字符串和向量

全部数组，字符串和向量必须限定长度。建议使用 `const` 修饰的标识符作为长度边界，方便协议使用者能够以编程的方式检查长度。

<!-- 
## Enums

Prefer enums with explicit sizes (e.g. `enum Foo : uint32 { ... }`) to plain
integer types when a field has a constrained set of non-arithmetic values.
-->

## 枚举

当字段表示非算术值约束集中某个值时，优先使用明确大小的枚举（例如: `enum Foo : uint32 { ... }`）而不是普通整数类型。

<!-- 
## Bitfields

If your protocol has a bitfield, represent its values using `bits` values.
For details, see the ["bits"][bits] topic in the readability rubric.
-->

## 位域

如果您的协议具有位字段，请使用“位”值表示其值。有关详细信息，请参见可读性规则中的["bits"][bits]主题。


<!-- 
## Non-channel based protocols

Some interface protocols may negotiate a non-channel protocol as a performance
optimization (e.g. the zircon.ethernet.Device's GetFifos/SetIOBuffer methods).
FIDL does not currently support expressing these protocols.  For now, represent
any shared data structures with `struct` definitions and provide detailed
documentation about participation in the protocol.  Packed structures are not
currently supported.

[fidl-style]: /docs/development/languages/fidl/guides/style.md
[fidl-api]: /docs/concepts/api/fidl.md
[bits]: /docs/concepts/api/fidl.md#bits
-->

## 基于非通道协议

某些协议为了性能优化可能会使用非通道协议（例如 `zircon.ethernet.Device` 的 `GetFifos/SetIOBuffer` 方法），FIDL 目前不支持表示这些协议。

现在，任何由 `struct` 定义，代表共享数据的结构，应提供其在协议中参与部分的详细文档。

目前不支持包装结构（`Packed structures`）。

[fidl-style]: /docs/development/languages/fidl/guides/style.md
[fidl-api]: /docs/concepts/api/fidl.md
[bits]: /docs/concepts/api/fidl.md#bits