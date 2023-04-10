# FIDL binary-compatibility (ABI) and source-compatibility (API) guide

Many changes to the Fuchsia platform involve changing FIDL APIs that have
already been published. Unless managed carefully, these changes risk breaking
existing usages. Failed changes manifest in the following ways:

* **Source incompatibility**: Users can no longer build against the generated
  code.
* **Binary incompatibility**: Consuming programs can no longer understand each
  other at runtime.

The Fuchsia project requires that changes to published FIDL libraries are both
source-compatible and binary-compatible for partners.

Note: Some changes are binary-compatible, yet require a specific transition path
to avoid runtime validation issues. Binary-compatibility indicates that two
peers have the same understanding of how to read or write the data, though these
two peers may disagree on which values are deemed valid. As an example, a
`uint32` is binary-compatible with a `enum : uint32`, even though the enum has
runtime validation that restricts the domain to only the specific values
identified by the enum members.

## Which changes to a FIDL API are safe?

For the purpose of describing interface compatibility, FIDL libraries are made
up of declarations. Each declaration has a name, type, attributes, and members.
Once an API is used outside of
[fuchsia.git](https://fuchsia.googlesource.com/fuchsia/) the safest assumption
is that all changes to it must be both binary-compatible and source-compatible
with current clients. This usually means evolving libraries using [soft
transitions], where the backwards-incompatible portions of a change are left to
the end when they will have no impact because all clients have already been
migrated. See [safely removing members](#safely-removing-members) for more
information on the most common soft transition pattern.

Note: Source-compatibility guarantees are only guaranteed under "normal"
circumstances. It is possible to write code that causes these guarantees to be
violated, e.g. static asserts.

### Safe changes to members

Aside from a declaration's name and attributes, all changes to its contract are
expressed in terms of changes to the declaration's members. This relationship
also means incompatible changes to a declaration become incompatible changes to
all the FIDL libraries that depend on that declaration, not just direct
consumers of the original library's generated bindings.

All operations are safe to perform if you are certain that all consumers can be
migrated atomically, i.e. they are all in the same source repository as the
library definition. Otherwise, these operations must be completed as the final
stage in a soft transition after all clients have been migrated away.

The table below summarizes various member changes and their respective safety
level when some clients cannot be migrated atomically:

| Parent   | Change Target | Reorder Lines | Add | Remove | Rename | Change Type | Change Ordinal | (Default) Value |
|----------|---------------|---------------|-----|--------|--------|-------------|----------------|-----------------|
| library  | declaration   | ✅ | ✅ | [⚠️](#library-declaration-remove) | ❌ | ❌ | -- | -- |
| protocol | method        | ✅ | [⚠️](#protocol-method-add) | [⚠️](#protocol-method-remove) | [⚠️](#protocol-method-rename) | ❌ | ❌ | -- |
| method   | parameter     | ❌ | ❌ | ❌ | [⚠️](#method-parameter-rename) | ❌ | -- | -- |
| struct   | field         | ❌ | ❌ | ❌ | ❌ | ❌ | -- | ✅ |
| table    | field         | ✅ | [✅](#table-field-add) | [✅️](#table-field-remove) | [⚠️](#table-field-rename) | ❌ | ❌ | -- |
| union    | variant       | ✅ | [⚠️](#union-variant-add) | [⚠️](#union-variant-remove) | [⚠️](#union-variant-rename) | ❌ | ❌ | -- |
| enum     | member        | ✅ | [⚠️](#enum-member-add) | [⚠️](#enum-member-remove) | [⚠️](#enum-member-rename) | ❌ | -- | ✅ |
| bits     | member        | ✅ | [⚠️](#bits-member-add) | [⚠️](#bits-member-remove) | [⚠️](#bits-member-rename) | ❌ | -- | ✅ |
| const    | value         | -- | -- | -- | -- | ❌ | -- | [✅](#const-value-default-value) |
| alias    | type          | -- | -- | -- | [⚠️](#type-alias-rename) | [⚠️](#type-alias-change-type) | -- | -- |
| _all_    | attribute     | -- | [⚠️](#attributes) | [⚠️](#attributes) | -- | -- | -- | -- |
| type     | constraint    | -- | [⚠️](#constraints) | [⚠️](#constraints) | -- | -- | -- | -- |
| _decl_   | modifier      | -- | [⚠️](#modifiers) | [⚠️](#modifiers) | -- | -- | -- | -- |

*Legend:*

* ✅ = *Safe*
* ⚠️ = *Careful (follow linked advice)*
* ❌ = *Unsafe*
* -- = *Not Applicable*

## Library {#library}

### Removing a library declaration {#library-declaration-remove}

**ABI**

It is binary-compatible to remove a library declaration.

**API**

Before removing a library declaration, ensure that no uses of this declaration
exists.

## Protocols {#protocol}

### Adding a method to a protocol {#protocol-method-add}

**ABI**

It is binary-compatible to add a method to a protocol.

**API**

To safely add a method to a protocol, mark the new method with
[`[Transitional]`][transitional]. Once all implementations of the new method are
in place, you can remove the [`[Transitional]`][transitional] attribute.

Examples: adding [an event][example-protocol-event-add],
[a method][example-protocol-method-add].

### Removing a method from a protocol {#protocol-method-remove}

**ABI**

It is binary-compatible to remove a method from a protocol.

**API**

To safely remove a method from a protocol, start by marking the method with
[`[Transitional]`][transitional]. Once this has fully propagated, you can remove
all implementations of the method, then remove the method from the FIDL
protocol.

Note: When using the Rust bindings, you need to manually add catch-all cases
(`_`) to all the match statements rather than rely on the
[`[Transitional]`][transitional] attribute. Read more about [how
`[Transitional]` impacts the Rust bindings][transitional-rust].

Examples: removing [an event][example-protocol-event-remove],
[a method][example-protocol-method-remove].

### Renaming a method {#protocol-method-rename}

**ABI**

Method renames can be made safe with use of the [`[Selector = "..."]`
attribute][selector].

**API**

It is not possible to rename a method in a source-compatible way.

## Method {#method}

### Renaming a method parameter {#method-parameter-rename}

**ABI**

It is binary-compatible to rename a method parameter.

**API**

Bindings typically rely on positional arguments, such that renaming a method
parameter is source-compatible.

## Table {#table}

### Adding a table field {#table-field-add}

**ABI**

It is binary-compatible to add a table field.

**API**

It is source-compatible to add a table field.

Example: [adding a table member][example-table-member-add].

### Removing a table field {#table-field-remove}

**ABI**

It is binary-compatible to remove a table field.

**API**

There must not be any use of the field to ensure a source-compatible removal.

Example: [removing a table member][example-table-member-remove].

### Renaming a table field {#table-field-rename}

**ABI**

It is binary-compatible to rename a table field.

**API**

It is not source-compatible to rename a table field.

## Union {#union}

### Adding a union variant {#union-variant-add}

**ABI**

It is binary-compatible to add a union variant. To ensure the added union
variant is not rejected during runtime validation, it must have propagated to
readers ahead of it being used by writers.

**API**

For `strict` unions, care must be taken to transition [switches on the union
tag](#switch-evolvability).

Example: [adding a union variant][example-union-member-add].

### Removing a union variant {#union-variant-remove}

**ABI**

It is binary-compatible to remove a union variant. To ensure the removed union
variant is not rejected during runtime validation, no writer may use the union
variant when it is removed.

**API**

For `strict` unions, care must be taken to transition [switches on the union
tag](#switch-evolvability).

Example: [removing a union variant][example-union-member-remove].

### Renaming a union variant {#union-variant-rename}

**ABI**

It is binary-compatible to rename a union variant.

**API**

It is not source-compatible to rename a union variant.

## Enum {#enum}

### Adding an enum member {#enum-member-add}

**ABI**

It is binary-compatible to add an enum member. To ensure the added enum member
is not rejected during runtime validation, it must have propagated to readers
ahead of it being used by writers.

**API**

Care must be taken to transition [switches on the enum](#switch-evolvability).

Example: [adding an enum member][example-enum-member-add].

### Removing an enum member {#enum-member-remove}

**ABI**

It is binary-compatible to remove an enum member. To ensure the removed enum
member is not rejected during runtime validation, no writer may use the enum
member when it is removed.

**API**

Care must be taken to transition [switches on the enum](#switch-evolvability).
Ensure that no uses of this enum member exists.

Example: [removing an enum member][example-enum-member-remove].

### Renaming an enum member {#enum-member-rename}

**ABI**

It is binary-compatible to rename an enum member.

**API**

It is not source-compatible to rename an enum member.

## Bits {#bits}

### Adding a bits member {#bits-member-add}

**ABI**

It is binary-compatible to add a bits member. For strict bits, to ensure the
added bits member is not rejected during runtime validation, it must have
propagated to readers ahead of it being used by writers.

**API**

It is source-compatible to add a bits member.

Example: [adding a bits member][example-bits-member-add].

### Removing a bits member {#bits-member-remove}

**ABI**

It is binary-compatible to remove a bits member. For strict bits, to ensure the
removed bits member is not rejected during runtime validation, no writer may use
the bits member when it is removed.

**API**

It is source-compatible to remove a bits member. Ensure that no uses of this
bits member exists.

Example: [removing a bits member][example-bits-member-remove].

### Renaming a bits member {#bits-member-rename}

**ABI**

It is binary-compatible to rename a bits member.

**API**

It is not source-compatible to rename a bits member.

## Constant {#const}

### Updating value of constants {#const-value-default-value}

**ABI**

It is sometimes binary-compatible to update the value of a `const` declaration.
If a constant value affects the public interface semantics (for example, by
representing a runtime invariant in the interface), changing the constant value
is binary-incompatible due to mismatched expectations between peers on different
versions.

**API**

It is usually source-compatible to update the value of a `const` declaration. In
rare circumstances, such a change could cause source-compatibility issues if the
constant is used in static asserts that would fail with the updated value.

## Type alias {#type-alias}

### Renaming a type alias {#type-alias-rename}

**ABI**

It is ABI compatible to rename a type alias.

**API**

It is not source-compatible to rename a type alias.

### Changing the underlying type of a type alias {#type-alias-change-type}

**ABI**

Typically, it is not ABI compatible to change the underlying type of a type
alias. However, if the original underlying type and the replacement underlying
type are ABI compatible, then a change is ABI compatible.

**API**

It is not source-compatible to change the underlying type of a type alias.

## Modifiers {#modifiers}

### Strict vs flexible {#strict-flexible}

Changing the strictness modifier of an enum, bits, or union declaration is
binary-compatible. Changing from `flexible` to `strict` may cause runtime
validation errors as unknown data for a previously flexible type will start
being rejected.

Generally, changing the strictness on a declaration is source-incompatible, but
possible to [soft transition][soft transitions].
Details for each declaration and binding are provided below.

#### Bits

**`strict` to `flexible`**

Changing a bits declaration from `strict` to `flexible` is:

* Source-compatible in LLCPP, Rust, Go, and Dart.
* Source-incompatible in HLCPP.
  * Any usages of the bits type as a template parameter must be removed first,
    since strict bits are generated as an `enum class` and flexible bits are
    generated as a `class` (which cannot be used as a non-type template
    parameter).

Example: [changing a bits declaration from `strict` to
`flexible`][example-bits-strict-flexible].

**`flexible` to `strict`**

Changing a bits declaration from `flexible` to `strict` is:

* Source-compatible Go, and Dart
* Source-incompatible in Rust, HLCPP and LLCPP.
  * Transitions from `flexible` to `strict` will require removing usages of
    [`flexible`-only APIs][bindings-ref].
  * In Rust, certain methods are provided for both strict and flexible bits, but
    usages for strict bits cause a deprecation warning during compation, which
    could become errors if using `-Dwarning` or `#![deny(warnings)]`.
  * In HLCPP, the bit mask is a `const` in the top level library namespace for
    strict bits, but a `static const` member of the generated class for flexible
    bits.

Example: [changing a bits declaration from `flexible` to
`strict`][example-bits-flexible-strict].

#### Enums

**`strict` to `flexible`**

Changing an enum declaration from `strict` to `flexible` is:

* Source-compatible in Go and Dart.
* Source-incompatible in Rust, HLCPP, and LLCPP.
  * In Rust, any `match` statements must be updated to handle unknown enum
    values [when using a `match` statement](#switch-evolvability).
  * In HLCPP and LLCPP, any uses of the enum as a template parameter must be
    removed first. This is because strict enums are generated as an `enum class`
    whereas flexible enums are generated as a `class`, which cannot be used as a
    non-type template parameter.

After changing from `strict` to `flexible`, care must be taken to correctly
handle any unknown enums.

`strict` enums that already have a specific member to represent the unknown case
can transition to being `flexible` by using the [`[Unknown]`][unknown-attr]
attribute.

Example: [changing an enum declaration from `strict` to
`flexible`][example-enum-strict-flexible].

**`flexible` to `strict`**

Changing an enum declaration from `flexible` to `strict` is:

* Source-incompatible in all bindings.
 * To make this change, any usages of [`flexible`-only APIs][bindings-ref], such
   as uses of the unknown placeholder, must be removed first.

Example: [changing an enum declaration from `flexible` to
`strict`][example-enum-flexible-strict].

#### Unions

Changing a union declaration from `strict` to `flexible` is source-compatible,
and changing from `flexible` to `strict` is source-incompatible. To perform the
latter, any usages [`flexible`-only APIs][bindings-ref] for the union must be
removed before it can be changed to `strict`.

Example: changing a union declaration from [`strict` to
`flexible`][example-union-strict-flexible], or [`flexible` to
`strict`][example-union-flexible-strict].

#### One-way methods and events

Changing a one-way method or event from `strict` to `flexible` or `flexible` to
`strict` is source compatible and binary compatible. Changing this modifier only
affects how the receiving end reacts if it receives this one-way method call or
event and does not recognize the ordinal.

#### Two-way methods

Whether changing a two-way method from `strict` to `flexible` or `flexible` to
`strict` is binary compatible depends on whether it uses `error` syntax. Whether
it is source compatible depends on the language.

Changing between `strict` and `flexible` is binary compatible if the method also
uses `error` syntax. If the method does not use `error` syntax, changing between
`strict` and `flexible` is binary-incompatible.

Changing a two-way method between `strict` and `flexible` is source compatible
in LLCPP, Rust, and Dart. In HLCPP, it is not source compatible. Go does not
support `open` or `ajar` protocols, so this is not applicable to Go.

### Value vs resource {#value-vs-resource}

Adding or removing the `resource` modifier on a struct, table, or union is
binary-compatible. Removing the `resource` modifier may cause runtime validation
errors: flexible types, such as tables and flexible unions, will now fail to
decode any unknown data (i.e. unknown variants for flexible unions and unknown
fields for tables) that contains handles. Note that this particular scenario
does not apply to LLCPP because LLCPP never stores unknown handles.

Adding or removing the `resource` modifier is not source-compatible.
Furthermore, bindings are encouraged to diverge APIs if they can leverage the
value type versus resource type distinction for specific benefits in the target
language (see [RFC-0057][rfc-0057-motivation] for context).

### Open, ajar, and closed {#open-ajar-closed}

Changing between `open`, `ajar`, and `closed` is binary compatible. Going from a
more closed protocol to a more open one, e.g. `closed` to `ajar` or `ajar` to
`open` is always possible, however going from a more open protocol to a more
closed one, e.g. `open` to `ajar` or `ajar` to `closed` may require changing
methods from `flexible` to `strict`, which is subject to the binary
compatibility restrictions covered in the [strict vs flexible](#strict-flexible)
section.

Changing between `open`, `ajar`, and `closed` is not source compatible.
Soft-transitioning for this change is not currently supported so the source
changes have to be done atomically with the FIDL change.

## General advice

### Safely removing members {#safely-removing-members}

Most [soft transitions] follow this basic shape:

1. Ensure that the element is not used or referenced
2. Remove the element

In a successful soft transition, only the second step is dangerous.

Note: Safely removing methods is more involved, see [removing a method from a
protocol](#protocol-method-remove).

### Renames {#renames}

Renaming declarations themselves is a source-incompatible change. Similarly,
renaming declaration members (e.g. a struct field) is source-incompatible.

Often, a source-compatible rename is possible following the long process of
adding a duplicate member with the desired name, switching all code to shift
from the old member to the new member, then deleting the old member. This
approach can be quite direct with table fields for instance.

Renames are binary-compatible, except in the case of libraries, protocols,
methods and events. See the `@selector` attribute for binary-compatible renames
of these.

### Attributes {#attributes}

Removing `@discoverable` is a source-incompatible change. You first need to
ensure that there are no references to the generated protocol name before
removing this attribute.

Adding or changing `@selector` is a binary-incompatible change on its own, but
can be used in the same change as method renames to preserve
binary-compatibility.

Removing [`@transitional`][transitional] is a source-incompatible change. You
first need to ensure that all implementations of the method are in place.

Adding or changing `@transport` is a source-incompatible and
binary-incompatible change.

Changes to the following attributes have no effect on compatibility, although
they often accompany other incompatible changes:

* `@deprecated` (although it may in the future if/when implemented)
* `@doc`
* `@max_bytes`
* `@max_handles`
* `@unknown`

### Constraints {#constraints}

For more information on what a constraint is in FIDL, see
[Type, layout, constraint][lexicon-type].

**ABI**

Relaxing or tightening constraints is binary-compatible. However, when evolving
constraints, care must be taken to transition readers or writers to avoid
runtime validation issues.

When relaxing a constraint, all readers must transition ahead of writers to
avoid values being rejected at runtime. Conversely, when tightening a
constraint, all writers must transition ahead of readers to avoid emitting
values that would then be rejected at runtime.

For instance:

* Growing a vector's maximum allowable size from `vector<T>:128` to
  `vector<T>:256` relaxes a constraints, i.e. move values will be allowed. As a
  result, readers must be transitioned ahead of writers.
* Restricting an optional handle `handle?` to be required `handle` tightens a
  constraint, optional handles that were accepted before will no longer be. As
  a result, writers must be transitioned ahead of readers.

**API**

Relaxing or tightening constraints is source-compatible.

### Evolving switch on enums, or union tag {#switch-evolvability}

When adding an enum member (or adding a union variant), any switch on the enum
(respectively the union tag) must first evolve to handle the soon to be
added member (resp. variant). This is done by adding a `default` case for
instance, or a catch-all `_` match. Depending on compiler flags, this may
require additional attributes such as
[`#[allow(dead_code)]`](https://doc.rust-lang.org/stable/rust-by-example/attribute/unused.html).

Similarly, when removing an enum member (or removing a union variant), any
switch on the enum (respectively the union tag) must first evolve to replace the
soon to be removed member (resp. variant) by a default case.

Note: A union tag is the discriminator indicating which variant is currently
held by the union (see [lexicon][lexicon-tag]). This is often an enum in
languages that do not support ADTs like C++.

## Testing

We have automated tests that prevent most breaking API changes. For more
information, see [FIDL API Compatibility Testing][fidl-api-compatibility-testing].

<!-- xrefs -->
[bindings-ref]: /docs/reference/fidl/bindings/overview.md
[example-bits-flexible-strict]: /docs/development/languages/fidl/guides/compatibility/bits_flexible_strict.md
[example-bits-member-add]: /docs/development/languages/fidl/guides/compatibility/bits_member_add.md
[example-bits-member-remove]: /docs/development/languages/fidl/guides/compatibility/bits_member_remove.md
[example-bits-strict-flexible]: /docs/development/languages/fidl/guides/compatibility/bits_strict_flexible.md
[example-enum-flexible-strict]: /docs/development/languages/fidl/guides/compatibility/enum_flexible_strict.md
[example-enum-member-add]: /docs/development/languages/fidl/guides/compatibility/enum_member_add.md
[example-enum-member-remove]: /docs/development/languages/fidl/guides/compatibility/enum_member_remove.md
[example-enum-strict-flexible]: /docs/development/languages/fidl/guides/compatibility/enum_strict_flexible.md
[example-protocol-event-add]: /docs/development/languages/fidl/guides/compatibility/protocol_event_add.md
[example-protocol-event-remove]: /docs/development/languages/fidl/guides/compatibility/protocol_event_remove.md
[example-protocol-method-add]: /docs/development/languages/fidl/guides/compatibility/protocol_method_add.md
[example-protocol-method-remove]: /docs/development/languages/fidl/guides/compatibility/protocol_method_remove.md
[example-table-member-add]: /docs/development/languages/fidl/guides/compatibility/table_member_add.md
[example-table-member-remove]: /docs/development/languages/fidl/guides/compatibility/table_member_remove.md
[example-union-flexible-strict]: /docs/development/languages/fidl/guides/compatibility/union_flexible_strict.md
[example-union-member-add]: /docs/development/languages/fidl/guides/compatibility/union_member_add.md
[example-union-member-remove]: /docs/development/languages/fidl/guides/compatibility/union_member_remove.md
[example-union-strict-flexible]: /docs/development/languages/fidl/guides/compatibility/union_strict_flexible.md
[fidl-api-compatibility-testing]: /docs/development/testing/ctf/fidl_api_compatibility_testing.md
[rfc-0057-motivation]: /docs/contribute/governance/rfcs/0057_default_no_handles.md#motivation
[lexicon-tag]: /docs/reference/fidl/language/lexicon.md#union-terms
[lexicon-type]: /docs/reference/fidl/language/lexicon.md#type-terms
[Platform Versioning]: /docs/contribute/governance/rfcs/0002_platform_versioning.md
[rust-bindings-tables]: /docs/reference/fidl/bindings/rust-bindings.md#types-tables
[rust-enum-macro]: /docs/reference/fidl/bindings/rust-bindings.md#types-enums
[selector]: /docs/reference/fidl/language/attributes.md#selector
[soft transitions]: /docs/contribute/governance/rfcs/0002_platform_versioning.md#terminology
[transitional-rust]: /docs/reference/fidl/bindings/rust-bindings.md#transitional
[transitional]: /docs/reference/fidl/language/attributes.md#transitional
[unknown-attr]: /docs/reference/fidl/language/attributes.md#unknown
