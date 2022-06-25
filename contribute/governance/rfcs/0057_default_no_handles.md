{% set rfcid = "RFC-0057" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

Note: Formerly known as [FTP](../deprecated-ftp-process.md)-057.

_"Look Ma, no hand(le)s!"_

## Summary

We propose to disallow handles in FIDL type declarations by default, and add a
new keyword `resource`[^1] to mark types that are allowed to contain handles or
other resource types. Adding or removing a `resource` modifier MAY be a
source-breaking change.

## Motivation {#motivation}

A distinctive feature of FIDL is its support for [Zircon
handles](concepts/kernel/handles.md). Handles are 32-bit integers in
memory, but they are treated specially: they must be moved rather than copied,
and they must be closed to avoid leaking resources. This special treatment leads
to problems when considering features that only make sense for plain data
without handles. Although FIDL bindings _can_ conditionally enable code based on
the presence of handles, doing so is undesirable because it breaks evolvability
guarantees. For example, adding a field to a table is normally safe, but adding
a handle field would become source-breaking—not only for that table, but for all
types transitively containing it. This pushes bindings to be conservative,
always assuming that types might contain handles even if the library author
never intends to add them.

The need to accommodate handles has led to compromise:

* In Dart, an effort to implement FIDL to JSON encoding received pushback
  because it would only work on types without handles, harming evolvability. It
  was ultimately built using the `MaxHandles` attribute, but this was a
  temporary solution because the attribute only applies to the outermost type,
  not to all types transitively reachable from it.
* In Rust, adding a handle to a type for the first time is source-breaking
  because the type will no longer derive the `Clone` trait. (Properly cloning a
  handle requires invoking the
  [zx_handle_duplicate](reference/syscalls/handle_duplicate.md) syscall,
  which can fail.)
* The Rust bindings for protocols take FIDL objects by mutable reference and
  zero out handles, rather than explicitly taking ownership, so that objects
  without handles can be reused afterwards.

All these cases can be handled in a safer, more ergonomic way if we require
library authors to indicate whether a type may contain handles, and if changing
the indication is expected to be source-breaking.

## Design

### Terminology

A FIDL type is either a **value** type or a **resource** type. Resource types
include:

*   `handle` and <code>handle<<em>H</em>></code> where <code><em>H</em></code>
    is a handle subtype
*   <code><em>P</em></code> and <code>request<<em>P</em>></code> where
    <code><em>P</em></code> is the name of a protocol
*   a struct, table, or union declared with the <code>resource</code> modifier
*   a type alias that refers to a resource type
*   a newtype [RFC-0052][rfc-0052] that wraps a resource type
*   <code><em>T</em>?</code> where <code><em>T</em></code> is a non-nullable
    resource type
*   <code>array<<em>T</em>></code> and <code>vector<<em>T</em>></code> where
    <code><em>T</em></code> is a resource type

All other types are value types.

When the `resource` modifier is used correctly, value types never contain
handles, whereas resource types may (now, or in the future) contain handles.

### Language

A new modifier `resource` can be applied to struct, table, and union
declarations.

Without `resource`, the declaration is not allowed to contain resource types.
The FIDL compiler must verify this. It only needs to check direct fields: if _A_
contains _B_, neither are marked as resources, and _B_ contains a handle, then
compilation will fail due to _B_ and there is no need for a separate error
message about _A_ transitively containing a handle.

With `resource`, the declaration is allowed to contain resource types. The new
type it declares is also considered a resource type, even if it does not contain
resources.

In principle the language could allow `resource` on newtype declarations
[RFC-0052][rfc-0052]. However, there is no practical use for a resource newtype
wrapping a value type, so instead newtypes implicitly inherit value/resource
status from the type they wrap.

### Grammar

This proposal modifies one rule in the [FIDL grammar](reference/fidl/language/grammar.md):

```
declaration-modifiers = "strict" | "resource" ;
```

### JSON IR

This proposal adds a key `"resource"` with a boolean value to all objects in the
`"struct_declarations"`, `"table_declarations"`, and `"union_declarations"`
arrays.

Note that this key is not redundant with `"max_handles"`. Value types must have
`max_handles` set to zero, but resource types can have any number of
`max_handles`, as it reflects the actual contents of the declaration (as opposed
to the library author's intent to allow handles).

### Bindings {#bindings}

This proposal does not include specific changes to the bindings. However, it
enables FIDL bindings authors (including the FIDL team) to address the issues
discussed in  [Motivation](#motivation). Here are some examples made possible by
this FTP, but not required in accepting it:

*   Implement JSON serialization and serialization on value types (or more
    likely the FIDL Text format rather than JSON, as proposed in
    [RFC-0058][rfc-0058].
*   Use different type signatures for the C++ `Clone()` method on value/resource
    types, to emphasize that only resource cloning can fail.
*   Make Rust protocols take value-type arguments as `&T` and resource-type
    arguments as `T`, instead of using `&mut T` for both and only mutating the
    resource types.

### API Rubric

The API rubric should provide guidance on when to use `resource`. Some simple
cases:

*   A struct with no resource types SHOULD NOT be marked `resource`, since
    structs are not designed to be extended (adding a handle later would in most
    cases break ABI).
*   A strict table or union with no resource types SHOULD NOT be marked
    `resource`, since the strictness already signals that modifying its fields
    is a source-breaking change.

It should also address the case of flexible tables and unions that do not
initially have handles. For instance, we might want to recommend erring on one
side or the other depending on the purpose of the library, how widely it will be
used, the anticipated cost of source-breaking changes in the languages being
used, and other factors.

## Implementation Strategy

The high-level implementation steps include:

*   Parse the `resource` keyword in fidlc.
*   Migrate existing FIDL libraries to use `resource` (more on this in
    [Unknowns](#unknowns).
*   Verify value/resource type rules in fidlc, with tests.
*   Store the `resource` flag in the JSON IR and expose it in fidlgen.

## Ergonomics {#ergonomics}

This proposal makes FIDL more complex since it introduces a new concept. Unlike
other FIDL constructs such as "struct" and "protocol," new users are unlikely to
guess what "resource" means, so they will need to learn from documentation.

It is arguable whether this proposal makes the FIDL language more or less
ergonomic. It helps by drawing attention to declarations that contain handles,
especially if the actual handle values are hidden in nested structures. Anyone
skimming a library will immediately see that a structure carries handles, not
just data. On the other hand, it might feel less ergonomic to worry about
whether to use `resource` and to type the keyword. Changing one declaration from
value to resource could have a painful cascading effect where many types must
become resources (though this can be seen as a good thing, since otherwise it
would show up as source breakage).

The increased complexity is justified by improvements in FIDL bindings. With the
freedom to offer different APIs for value types and resource types, the bindings
can be made safer and more ergonomic. For examples of these improvements, see
[Bindings](#bindings).

## Documentation and Examples

The following tasks need to be done:

*   Update all documentation involving handles to use `resource` as appropriate.
*   Update the FIDL Language Specification to explain the `resource` modifier.
*   Mention `resource` in the FIDL Tutorial. There should be a short note
    explaining all the modifiers (so, `strict` and `resource`).
*   Provide guidance on whether a new type without handles should be a resource.
*   Once bindings take advantage of the value/resource distinction, update their
    documentation to note the differences between APIs offered by value types
    and resource types, and provide instructions for transitioning between them
    (if possible).

## Backwards Compatibility {#backwards-compatibility}

This proposal has no impact on ABI compatibility.

> *Amendment (Jul 2021).* During implementation, we discovered an edge case in
> this proposal's interaction with [RFC-0033: Handling of unknown fields and
> strictness][rfc-0033]. Some bindings store unknown members when decoding
> tables and flexible unions; this is not possible for value types if the
> unknown member contains handles, so decoding must fail in this case. See the
> [compatibility guide][compat-resource] for more details.
>
> *Amendment (Oct 2021).* After [RFC-0137: Discard unknown data in
> FIDL][rfc-0137], bindings no longer store unknown data, so there is no more
> edge case. The value/resource distinction thus has no impact on ABI
> compatibility.

Adding or removing a `resource` modifier is **neither source-compatible nor
transitionable**,[^2] in the sense of
[RFC-0024](contribute/governance/rfcs/0024_mandatory_source_compatibility.md).
The bindings are explicitly allowed to generate incompatible APIs for two types
that differ only in the presence of the modifier, and it may in fact be
impossible to write code that compiles before and after adding/removing the
modifier. Library authors wishing to transition to/from `resource` in a
source-compatible manner must create new types and methods instead of changing
existing ones.

Once bindings authors start taking advantage of the value/resource distinction,
we will revisit this decision. It might be worthwhile to require a
transitionable path (perhaps using an intermediate stage with the
`[Transitional]` attribute). At the outset, this is unclear: it might be too
restrictive, undermining potential API improvements this proposal is meant to
enable.

## Performance

This proposal has negligible impact on build performance: the FIDL compiler will
do slightly more work to parse the new keyword and validate its use. It has no
direct impact on IPC performance. It might enable a small improvement in some
languages if bindings use the value/resource distinction to create APIs that
discourage unnecessary copies. For example, there should be no need to clone a
value-type object in order to send it multiple times.

## Security

This proposal does not directly affect security, but it enables bindings to
provide safer APIs. For example, C++ could force error handling on `Clone()` for
resource types with `[[nodiscard]]`, or Rust could take resource-type method
arguments by move to prevent accidental use of the mutated object afterwards.
These kinds of changes could prevent bugs, including security bugs.

## Testing

This feature will be tested in the following ways:

*   Add tests in fidlc for the parsing and validation code paths. These tests
    should exercise a variety of cases where a declaration marked `resource` (or
    not) fails to conform to the definition of a resource type (or value type).
*   Add some resource type declarations to the goldens, in addition to fixing
    existing declarations that need `resource`.
*   Update the [fidl-changes test suite](/src/tests/fidl/source_compatibility/) to
    demonstrate the steps for a transition from value type to resource type and
    vice versa.

## Drawbacks, Alternatives, and Unknowns {#unknowns}

This proposal introduces a new keyword, which makes the language more complex.
Having too many keywords could be a problem; "strict resource union" is a bit of
a mouthful.

This proposal weakens FIDL evolvability guarantees in two ways:

*   Before, adding a handle to a type was not expected to be a source-breaking
    change. Now, this is explicitly allowed and expected (unless the type was
    marked `resource` in anticipation of needing to add a handle).
*   Before, a type could be declared with the future expectation of (1) adding
    handles to it, and (2) being able to include it as a field in any other
    type. Now, library authors must choose between (1) and (2) at the outset.

There are two main alternatives to this proposal:

*   **Do nothing**. Allow handles to be used anywhere, and live with the fact
    that bindings must preserve source compatibility when adding or removing
    handles.
*   **Default allow handles**. Like this proposal, but assume declarations are
    resource types by default, and require a `value` keyword to disallow
    resource types in a declaration.

The [Motivation](#motivation) and [Ergonomics](#ergonomics) sections argue
against doing nothing. For the other alternative, experience has shown that most
messages do not contain handles, and that passing handles in protocols requires
care and upfront planning. In other words, value types are the common case, and
the ability to add handles as an afterthought might not be as useful as it
seems. This suggests that not allowing handles is the better default.

This proposal mainly benefits end-developers using FIDL bindings, whereas its
drawbacks apply to library authors who design the APIs. This tradeoff is in
keeping with the [Fuchsia API Council Charter][api-council-values], which
prioritizes end-developers over API designers and implementers.

One more alternative has been suggested: **handles as references**. Instead of
banning handles from value types, it would resolve the value/resource issues by
representing handles as references. Cloning a structure containing a handle
would just make another reference to the same handle. This could be accomplished
using `shared_ptr` in C++, and it could greatly simplify things without needing
to add the `resource` keyword. However, it has its challenges:

*   All bindings would need a bookkeeping mechanism to ensure that a handle is
    only closed once its last reference is gone. This could be difficult in some
    languages.
*   After sending a handle to another process, all other references to it would
    become invalid, like dangling pointers. The convenience of treating handles
    more like ordinary values means we have less compile-time safety in these
    situations.
*   As this involves changing types for all handles, it would likely be a
    breaking change in all languages. A smooth transition would require a lot of
    work.

Several open questions remain for this proposal:

*   How should we migrate existing FIDL libraries? Marking _all_ existing
    declarations with `resource` would be safe, but not reflect library authors'
    intentions. Marking only the bare minimum (i.e., types that contain handles)
    would work, but might be too aggressive in assuming that anything without
    handles was intended to never contain any.
*   How will this feature interact with generic data types, if they are adopted?
    For example, if we define a `Pair<A, B>` type, it should logically be a
    resource type if `A` or `B` is a resource type, rather than having to
    annotate `Pair` itself. Are there other cases where it is preferable to
    derive whether a type is a resource?

## Prior Art and References

The goal of this proposal is to allow source-breaking changes to occur when
changing a type's value/resource status.
[RFC-0024](contribute/governance/rfcs/0024_mandatory_source_compatibility.md) is relevant to this
goal, since it established the source-compatibility standard for FIDL. It also
touched on the issue of handles making it difficult to use the `Clone` trait in
Rust, which this proposal solves.

We are not aware of other IPC systems addressing this exact issue
(distinguishing types that may contain handles, or system resources). However,
the concept of annotating types in a way that "infects" all use-sites is common
in programming languages. For example, async functions in JavaScript, Python,
and Rust have this behaviour, as well as the IO monad in Haskell.

[^1]: An earlier version of this proposal instead called the keyword `entity`.

[^2]: An earlier version of this proposal required the change to be
    transitionable.

<!-- link labels -->
[api-council-values]: contribute/governance/api_council.md#values
[compat-resource]: development/languages/fidl/guides/compatibility/README.md#value-vs-resource
[rfc-0033]: contribute/governance/rfcs/0033_handling_unknown_fields_strictness.md
[rfc-0052]: contribute/governance/rfcs/0052_type_aliasing_named_types.md
[rfc-0058]: contribute/governance/rfcs/0058_deprecated_attribute.md
[rfc-0137]: contribute/governance/rfcs/0137_discard_unknown_data_in_fidl.md
