# FIDL lexicon

This document defines general terms that have a specific meaning in a FIDL
context. To learn more about specific FIDL topics, refer to the [FIDL
traihead][trailhead].

## Type, layout, constraint {#type-terms}

A **type** classifies a value in the FIDL type system. Types appear in the
`type-constructor` position defined in the [FIDL grammar][grammar]. For example,
`uint32`, `string:10`, and `vector<bool>` are types.

A **layout** is a parametrizable description of a type. It does not refer to a
single type, but describes a family of types obtained by instantiating the
layout with zero or more **layout parameters**. For example, `vector` is a
layout which takes one layout parameter, a type such as `vector<bool>` itself a
type. As another example, `array` is a layout which takes two layout parameters,
producing types such as `array<bool, 3>`.

The difference between layout and type can be subtle. For example, the statement
above that `uint32` is a type is not strictly correct. Rather, it is a layout
that takes zero parameters, distinct from the type obtained by instantiating it.
The FIDL syntax does not capture this distinction, so when `uint32` is used in
type position (e.g. `alias uint = uint32;`), it is really referencing the layout
`uint32` and implicitly instantiating it with zero layout parameters.

A **constraint** restricts a type to only allow values satisfying a predicate.
For example, the type `string:10` has the constraint `length <= 10` abbreviated
as `10`, meaning the string length cannot exceed 10 bytes.

Layouts and layout parameters affect how bytes are laid out in the [FIDL wire
format][wire-format], while constraints affect validation that restricts what
can be represented (see [RFC-050][rfc-050-layouts-constraints] for more detail).
Syntactically, layout parameters can only be applied to layouts, and constraints
can only be applied to types. For example:

```fidl
alias Bools = vector<bool>;       // ok: layout parameter applied to layout
alias MaxTenBools = Bools:10;     // ok: constraint applied to type
alias MaxTenBytes = Bools<byte>;  // INVALID: layout parameter applied to type
alias MaxTen = vector:10;         // INVALID: constraint applied to layout
```

The general form of a type instantiation is

    L<L_1, L_2, ..., L_n>:<C_1, C_2, ..., C_n>

where `L` is a layout, `L_1` through `L_n` are layout parameters, and `C_1`
through `C_n` are constraints.

## Member, field, variant {#member-terms}

A **member** of a declaration is an individual element belonging to a
declaration, i.e. a declaration is comprised of zero, one, or many members.

For instance, consider the `Mode` bits declaration:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/misc.test.fidl" region_tag="mode" %}
```

Both `READ` and `WRITE` are members.

When referring to members of structs or tables, we can more specifically refer
to these members as **fields**.

When referring to members of a union, we can more specifically refer to these
members as **variants**.

For example, consider the `Command` union declaration:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/misc.test.fidl" region_tag="command" %}
```

The two variants are `create_resource` and `release_resource`.

Furthermore, the **selected variant** of an instance of a union is the current
value held by the union at that moment.

## Tag, and ordinal {#union-terms}

The **tag** is the target language variant discriminator, i.e. the specific
construct in a target language that is used to indicate the selected variant of
a union. For example, consider the following TypeScript representation of the
`Command` union:

```typescript
enum CommandTag {
    Create,
    Release,
}

interface Command {
    tag: CommandTag,
    data: CreateResource | ReleaseResource,
}
```

The tag of `Command` is `Command.tag` and has type `CommandTag`. The actual
values and type representing each variant of `Command` are up to the
implementation.

Note that some languages will not require a tag. For example, some languages use
pattern matching to branch on the variant of a union instead of having an
explicit tag value.

The **ordinal** is the on the wire variant discriminator, i.e. the value used to
indicate the variant of a union in the [FIDL wire format][wire-format]. The
ordinals are explicitly specified in the FIDL definition (in this example, 1 for
`create_resource` and 2 for `release_resource`).

## Encode {#encode}

Encoding refers to the process of serializing values from a target language into
the FIDL wire format.

For the C family of bindings (HLCPP, LLCPP), encode can have a more specific
meaning of taking bytes matching the layout of the FIDL wire format and patching
pointers and handles by replacing them with
`FIDL_ALLOC_PRESENT`/`FIDL_ALLOC_ABSENT` or
`FIDL_HANDLE_PRESENT`/`FIDL_HANDLE_ABSENT` in-place, moving handles into an
out-of-band handle table.

## Decode {#decode}

Decoding refers to the process of deserializing values from raw bytes in the
FIDL wire format into a value in a target language.

For the C family of bindings (HLCPP, LLCPP), decode can have a more specific
meaning of taking bytes matching the layout of the FIDL wire format and patching
pointers and handles by replacing `FIDL_ALLOC_PRESENT`/`FIDL_ALLOC_ABSENT` or
`FIDL_HANDLE_PRESENT`/`FIDL_HANDLE_ABSENT` with the "real" pointer/handle
values in-place, moving handles out of an out-of-band handle table.

## Validate {#validate}

Validation is the process of checking if constraints from the FIDL definition
are satisfied for a given value. Validation occurs both when encoding a value
before being sent, or when decoding a value after receiving it. Example
constraints are vector bounds, handle constraints, and the valid encoding of a
string as UTF-8.

When validation fails, the bindings surface the error to user code, either by
returning it directly or via an error callback.

## Result/error type {#result}

For methods with error types specified:

```fidl
DoWork() -> (struct { result Data; }) error uint32
```

The **result type** refers to the entire message that would be received by a
server for this method, i.e. the union that consists of either a result of
`Data` or an error of `uint32`. The error type in this case is `uint32`, whereas
`Data` can be referred to as either the response type or the success type.

<!-- xrefs -->
[trailhead]: development/languages/fidl/README.md
[wire-format]: reference/fidl/language/wire-format
[grammar]: reference/fidl/language/grammar.md#grammar
[rfc-050-layouts-constraints]: contribute/governance/rfcs/0050_syntax_revamp.md#layouts-constraints
