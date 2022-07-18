<!--

# FIDL design history

-->
# FIDL设计历史

<!---

This page provides a starting point for navigating [RFCs] in the "FIDL" area. It
organizes them in various ways and gives an updated status on each, bringing the
reader up to speed with FIDL's history and current direction.

-->

本页面提供了在[RFCs]中“FIDL”部分的开始引导。本章采用不同方式集合来展现其每个阶段的更新状态，带领读者快速了解FIDL的相关历史和当前方向。

<!---

Previously, these design documents were called [FTPs] or FIDL Tuning Proposals.
The FTP process was later [merged into][rfc-0017] the RFC process. Because of
the way FTPs were renumbered as RFCs, RFC numbers do not imply chronology.
However, each individual list on this page shows RFCs in chronological order.

-->

首先，这些设计文件被称为FTPs(FIDL Tuning Proposals)或者FIDL调试方案。FTP流程后续将合入到RFC（request for comments）流程中。因为FTPs被以RFCs方式进行重新编码，所以RFC编号并不代表着时间顺序。但是本页面上的每个列表都按时间顺序显示RFCs。

<!---

## By theme

--->

## 按主题分类

<!---

This section organizes FIDL RFCs by theme. Each RFC is listed once, under its
primary theme, even if it touches on multiple themes.

--->

本章节按照主题组织FIDL RFCs。每个RFC即使涉及多个主题，但在本章内仅列出一次。

<!---

### Governance

--->

### 管理

* [RFC-0018: FTP process: A modest proposal][rfc-0018]
* [RFC-0049: FIDL tuning process evolution][rfc-0049]
* [RFC-0017: The FTP Process is dead, long live the RFC Process!][rfc-0017]

<!---

### Evolvability

--->

### 可进化性

* [RFC-0047: Tables][rfc-0047]
* (Rejected) [RFC-0063: OrdinalRange][rfc-0063]
* [RFC-0058: Introduce a deprecated attribute][rfc-0058]
* [RFC-0061: Extensible unions][rfc-0061]
* [RFC-0020: Interface ordinal hashing][rfc-0020]
* [RFC-0021: Soft transitions for methods add and remove][rfc-0021]
* [RFC-0024: Mandatory source compatibility][rfc-0024]
* [RFC-0029: Increasing method ordinals][rfc-0029]
* [RFC-0033: Handling of unknown fields and strictness][rfc-0033]
* [RFC-0037: Transactional message header v3][rfc-0037]
* [RFC-0040: Identifier uniqueness][rfc-0040]
* [RFC-0048: Explicit union ordinals][rfc-0048]
* [RFC-0083: FIDL versioning][rfc-0083]

<!---

### Expressivity

--->

### 表现性

* [RFC-0019: Type aliases with using][rfc-0019]
* [RFC-0022: Default values for struct members][rfc-0022]
* [RFC-0066: Programmer advisory explicit defaults][rfc-0066]
* (Rejected) [RFC-0065: No optional strings or vectors][rfc-0065]
* (Rejected) [RFC-0064: Box\<Knox\>][rfc-0064]
* [RFC-0023: Compositional model for protocols][rfc-0023]
* [RFC-0041: Support for unifying services and devices][rfc-0041]
* (Rejected) [RFC-0044: Extensible method arguments][rfc-0044]
* [RFC-0052: Type aliasing and new types][rfc-0052]
* [RFC-0054: Parameter attributes][rfc-0054]

<!---

### Syntax

--->

### 语法

* (Rejected) [RFC-0038: Separating layout from constraints][rfc-0038]
* (Rejected) [RFC-0039: Types come second][rfc-0039]
* [RFC-0050: Syntax revamp][rfc-0050]
* [RFC-0086: Updates to RFC-0050: FIDL attributes syntax][rfc-0086]
* [RFC-0087: Updates to RFC-0050: FIDL method parameter syntax][rfc-0087]

<!---

### Ergonomics

--->

### 工效学

* [RFC-0053: Epitaphs][rfc-0053]
* [RFC-0056: Empty structs][rfc-0056]
* [RFC-0060: Error handling][rfc-0060]
* [RFC-0025: Bit flags][rfc-0025]
* (Rejected) [RFC-0031: Typed epitaphs][rfc-0031]
* [RFC-0057: Default no handles][rfc-0057]

<!---

### Performance

--->

### 性能

* (Rejected) [RFC-0026: Envelopes everywhere][rfc-0026]
* [RFC-0027: You only pay for what you use][rfc-0027]
* (Rejected) [RFC-0032: Efficient envelopes][rfc-0032]
* (Rejected) [RFC-0035: Automatic flow tracing][rfc-0035]
* (Rejected) [RFC-0045: Zero-size empty structs][rfc-0045]

<!---

### Safety & security

--->

### 功能安全和信息安全

* [RFC-0051: Safer structs for C++][rfc-0051]
* (Rejected) [RFC-0062: Method impossible][rfc-0062]
* [RFC-0028: Handle rights][rfc-0028]
* (Rejected) [RFC-0034: Null terminate strings][rfc-0034]

<!---

### Tooling

--->

### 工具

* [RFC-0076: FIDL API summaries][rfc-0076]

<!---

### Documentation

--->

### 文档

* [RFC-0055: Documentation comments][rfc-0055]
* [RFC-0043: Documentation comment format][rfc-0043]

<!---

### Simplify design space

--->

### 简化设计空间

* [RFC-0030: FIDL is little endian][rfc-0030]
* [RFC-0059: Reserved bits in vector, string, and array count fields][rfc-0059]

<!---

## By language feature

--->

## 按语言特性分类

<!---

This section lists RFCs that introduced new feature that are easily visible in
the FIDL language. It only includes current features, not obsolete ones.

--->

本章列出在FIDL语言中引入的新特性的RFCs。它只包含当前特性，而不包含已被废弃的。

| Feature              | RFC        | Note
| -------------------- | -----------| ------------------------------------------
| struct defaults      | [RFC-0022] | Default values for struct members
| `table`              | [RFC-0047] | Forward and backward compatible data type
| `///`                | [RFC-0055] | Documentation comments
| `struct Empty {};`   | [RFC-0056] | Empty structs
| `error`              | [RFC-0060] | Method error result syntax
| `union`              | [RFC-0061] | Initially called `xunion`, replaced the old static unions
| `[Selector]`         | [RFC-0020] | Override method hashing with an explicit selector
| `[Transitional]`     | [RFC-0021] | Attribute for soft transitioning method addition/removal
| `compose`            | [RFC-0023] | Renamed `interface` to `protocol`, added the `compose` feature
| `bits`               | [RFC-0025] | Bit flags types
| handle rights        | [RFC-0028] | Annotate required or excluded handle rights
| `strict`, `flexible` | [RFC-0033] | Strict and flexible types
| `service`            | [RFC-0041] | Services are collection of protocols
| `alias`, `type`      | [RFC-0052] | Replaced `using` aliases with `alias` and `type`
| `resource`           | [RFC-0057] | Value and resource types

<!-- TODO(fxbug.dev/67858): Uncomment when implemented. -->
<!-- | `@available`      | [RFC-0083] | Version annotations -->

<!---

## Current status

--->

## 当前状态

<!---

This section gives the current status of all FIDL RFCs.

--->

本章展示了所有FIDL RFCs的当前状态

_Legend:_

| Status                     | Meaning
| -------------------------- | ------------------------------------------------------
| Unpublished                | Never made public nor formally reviewed
| Withdrawn                  | Made public, but never formally reviewed
| Implemented                | Accepted, implemented, still accurate
| Unimplemented              | Accepted but never implemented, no plans to do so
| Partially implemented      | Accepted and partially implemented, no plans to finish
| Implementation in progress | Accepted, implementation in progress
| Amended                    | Accepted, still mostly accurate, but amended by a later RFC
| Superseded                 | Accepted but no longer accurate, superseded by a later RFC
| Rejected                   | Formally rejected
| Rejected; Superseded       | Formally rejected, then superseded by a later RFC

| RFC        | Title                          | Status
| ---------- | ------------------------------ | --------------------------------
| [RFC-0018] | FTP process: A modest proposal | Superseded by [RFC-0017]
| [RFC-0019] | Type aliases with using | Superseded by [RFC-0052]
| [RFC-0022] | Default values for struct members | Partially implemented: only in HLCPP, Dart
| [RFC-0051] | Safer structs for C++ | Unimplemented: [change][safer-structs-impl] was never merged
| [RFC-0062] | Method impossible | Rejected
| [RFC-0066] | Programmer advisory explicit defaults | Implemented
| [RFC-0047] | Tables | Implemented
| [RFC-0053] | Epitaphs | Implemented
| [RFC-0055] | Documentation comments | Amended by [RFC-0043]
| [RFC-0063] | OrdinalRange | Rejected; Superseded by [RFC-0020]
| &ndash;    | FIDL in FIDL | Unpublished; Protobuf [can do this][protobuf-self-describe]
| [RFC-0056] | Empty structs | Implemented
| [RFC-0058] | Introduce a deprecated attribute | Superseded by [RFC-0083]
| [RFC-0060] | Error handling | Implemented
| [RFC-0061] | Extensible unions | Implemented
| [RFC-0065] | No optional strings or vectors | Rejected
| [RFC-0064] | Box\<Knox\> | Rejected; see also "FIDL large message support"
| &ndash;    | Sequences | Unpublished; earlier version of "Streams"
| &ndash;    | Streams | Unpublished; see also [measure-tape] (a pragmatic solution)
| [RFC-0020] | Interface ordinal hashing | Implemented
| [RFC-0021] | Soft transitions for methods add and remove | Implemented
| [RFC-0023] | Compositional model for protocols | Implemented
| [RFC-0024] | Mandatory source compatibility | Implemented; see also [source_compatibility]
| [RFC-0025] | Bit flags | Implemented
| [RFC-0026] | Envelopes everywhere | Rejected
| [RFC-0027] | You only pay for what you use | Implemented
| [RFC-0028] | Handle rights | Implementation in progress
| [RFC-0029] | Increasing method ordinals | Superseded by [RFC-0037]
| [RFC-0030] | FIDL is little endian | Implemented
| [RFC-0031] | Typed epitaphs | Rejected
| [RFC-0032] | Efficient envelopes | Rejected (accepted at first, later rejected)
| [RFC-0033] | Handling of unknown fields and strictness | Implemented
| [RFC-0034] | Null terminate strings | Rejected
| [RFC-0035] | Automatic flow tracing | Rejected
| [RFC-0036] | Update to struct declarations | Rejected
| [RFC-0037] | Transactional message header v3 | Implemented
| [RFC-0038] | Separating layout from constraints | Rejected; Superseded by [RFC-0050]
| [RFC-0039] | Types come second | Rejected; Superseded by [RFC-0050]
| [RFC-0040] | Identifier uniqueness | Implemented (rejected at first, later accepted)
| [RFC-0041] | Support for unifying services and devices | Implemented
| [RFC-0042] | Non nullable types | Rejected (April Fools)
| [RFC-0043] | Documentation comment format | Implemented
| [RFC-0044] | Extensible method arguments | Rejected; Superseded by [RFC-0087]
| [RFC-0045] | Zero-size empty structs | Rejected
| &ndash;    | Versioning | Unpublished; Superseded by [RFC-0083]
| &ndash;    | Required table fields | Unpublished
| [RFC-0048] | Explicit union ordinals | Implemented
| [RFC-0049] | FIDL tuning process evolution | Implemented
| [RFC-0050] | Syntax revamp | Implementation in progress
| &ndash;    | Unified view of optionality | Unpublished
| [RFC-0052] | Type aliasing and new types | Implementation in progress
| &ndash;    | Iterators | Unpublished; see also [measure-tape] (a pragmatic solution)
| [RFC-0054] | Parameter attributes | Implemented
| &ndash;    | Restrict non-numeric floating point values | Unpublished
| &ndash;    | Constant expressions | Unpublished
| [RFC-0057] | Default no handles | Implemented
| &ndash;    | FIDL text format | Unpublished
| [RFC-0059] | Reserved bits in vector, string, and array count fields | Implemented; reserved bits no longer used by LLCPP
| &ndash;    | FIDL Large message support | [Withdrawn][large-message-cl]
| [RFC-0076] | FIDL API summaries | Implementation in progress
| [RFC-0083] | FIDL versioning | Implementation in progress
| [RFC-0086] | Updates to RFC-0050: FIDL attributes syntax | Implementation in progress
| [RFC-0087] | Updates to RFC-0050: FIDL method parameter syntax | Implementation in progress

<!-- link labels -->
[rfcs]: /docs/contribute/governance/rfcs/README.md#proposals
[ftps]: /docs/contribute/governance/deprecated-ftp-process.md
[safer-structs-impl]: https://fuchsia-review.googlesource.com/c/garnet/+/139083
[protobuf-self-describe]: https://developers.google.com/protocol-buffers/docs/techniques#self-description
[measure-tape]: /tools/fidl/measure-tape/README.md
[source_compatibility]: /src/tests/fidl/source_compatibility/README.md
[large-message-cl]: https://fuchsia-review.googlesource.com/c/fuchsia/+/470640
[rfc-0017]: /docs/contribute/governance/rfcs/0017_folding_ftp_into_rfc.md
[rfc-0018]: /docs/contribute/governance/rfcs/0018_ftp_process.md
[rfc-0019]: /docs/contribute/governance/rfcs/0019_using_evolution_uint64.md
[rfc-0020]: /docs/contribute/governance/rfcs/0020_interface_ordinal_hashing.md
[rfc-0021]: /docs/contribute/governance/rfcs/0021_soft_transitions_methods_add_remove.md
[rfc-0022]: /docs/contribute/governance/rfcs/0022_default_values_for_struct.md
[rfc-0023]: /docs/contribute/governance/rfcs/0023_compositional_model_protocols.md
[rfc-0024]: /docs/contribute/governance/rfcs/0024_mandatory_source_compatibility.md
[rfc-0025]: /docs/contribute/governance/rfcs/0025_bit_flags.md
[rfc-0026]: /docs/contribute/governance/rfcs/0026_envelopes_everywhere.md
[rfc-0027]: /docs/contribute/governance/rfcs/0027_you_only_pay_what_you_use.md
[rfc-0028]: /docs/contribute/governance/rfcs/0028_handle_rights.md
[rfc-0029]: /docs/contribute/governance/rfcs/0029_increasing_method_ordinals.md
[rfc-0030]: /docs/contribute/governance/rfcs/0030_fidl_is_little_endian.md
[rfc-0031]: /docs/contribute/governance/rfcs/0031_typed_epitaphs.md
[rfc-0032]: /docs/contribute/governance/rfcs/0032_efficient_envelopes.md
[rfc-0033]: /docs/contribute/governance/rfcs/0033_handling_unknown_fields_strictness.md
[rfc-0034]: /docs/contribute/governance/rfcs/0034_null_terminate_strings.md
[rfc-0035]: /docs/contribute/governance/rfcs/0035_automatic_flow_tracing.md
[rfc-0036]: /docs/contribute/governance/rfcs/0036_update_struct_declarations.md
[rfc-0037]: /docs/contribute/governance/rfcs/0037_transactional_message_header_v3.md
[rfc-0038]: /docs/contribute/governance/rfcs/0038_seperating_layout_from_constraints.md
[rfc-0039]: /docs/contribute/governance/rfcs/0039_types_come_second.md
[rfc-0040]: /docs/contribute/governance/rfcs/0040_identifier_uniqueness.md
[rfc-0041]: /docs/contribute/governance/rfcs/0041_unifying_services_devices.md
[rfc-0042]: /docs/contribute/governance/rfcs/0042_non_nullable_types.md
[rfc-0043]: /docs/contribute/governance/rfcs/0043_documentation_comment_format.md
[rfc-0044]: /docs/contribute/governance/rfcs/0044_extensible_method_arguments.md
[rfc-0045]: /docs/contribute/governance/rfcs/0045_zero_size_empty_structs.md
[rfc-0047]: /docs/contribute/governance/rfcs/0047_tables.md
[rfc-0048]: /docs/contribute/governance/rfcs/0048_explicit_union_ordinals.md
[rfc-0049]: /docs/contribute/governance/rfcs/0049_fidl_tuning_process_evolution.md
[rfc-0050]: /docs/contribute/governance/rfcs/0050_syntax_revamp.md
[rfc-0051]: /docs/contribute/governance/rfcs/0051_safer_structs_for_cpp.md
[rfc-0052]: /docs/contribute/governance/rfcs/0052_type_aliasing_named_types.md
[rfc-0053]: /docs/contribute/governance/rfcs/0053_epitaphs.md
[rfc-0054]: /docs/contribute/governance/rfcs/0054_parameter_attributes.md
[rfc-0055]: /docs/contribute/governance/rfcs/0055_documentation_comments.md
[rfc-0056]: /docs/contribute/governance/rfcs/0056_empty_structs.md
[rfc-0057]: /docs/contribute/governance/rfcs/0057_default_no_handles.md
[rfc-0058]: /docs/contribute/governance/rfcs/0058_deprecated_attribute.md
[rfc-0059]: /docs/contribute/governance/rfcs/0059_reserved_bits_count_fields.md
[rfc-0060]: /docs/contribute/governance/rfcs/0060_error_handling.md
[rfc-0061]: /docs/contribute/governance/rfcs/0061_extensible_unions.md
[rfc-0062]: /docs/contribute/governance/rfcs/0062_method_impossible.md
[rfc-0063]: /docs/contribute/governance/rfcs/0063_OrdinalRange.md
[rfc-0064]: /docs/contribute/governance/rfcs/0064_box_knox.md
[rfc-0065]: /docs/contribute/governance/rfcs/0065_optional_strings_or_vectors.md
[rfc-0066]: /docs/contribute/governance/rfcs/0066_programmer_advisory_explicit.md
[rfc-0076]: /docs/contribute/governance/rfcs/0076_fidl_api_summaries.md
[rfc-0083]: /docs/contribute/governance/rfcs/0083_fidl_versioning.md
[rfc-0086]: /docs/contribute/governance/rfcs/0086_rfc_0050_attributes.md
[rfc-0087]: /docs/contribute/governance/rfcs/0087_fidl_method_syntax.md
