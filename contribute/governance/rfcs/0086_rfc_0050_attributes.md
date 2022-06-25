{% set rfcid = "RFC-0086" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}

# {{ rfc.name }}: {{ rfc.title }}

<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

## Summary

This document describes a new syntax for [attributes][fidl-attributes] in the
FIDL language.

**See also:**

* [RFC-0050: FIDL Syntax Revamp](0050_syntax_revamp.md)
* [RFC-0087: Updates to RFC-0050: FIDL Method Parameter Syntax](0087_fidl_method_syntax.md)
* [RFC-0088: Updates to RFC-0050: FIDL Bits, Enum, and Constraints Syntax](0088_rfc_0050_bits_enums_constraints.md)

## Motivation

FIDL attributes provide a clear way to attach compile-time metadata to FIDL
declarations, allowing authors to pass extra information to the FIDL compiler
pertaining to documentation ([Doc][fidl-attributes-doc],
[NoDoc][fidl-attributes-no-doc]) compile-time validation
([ForDeprecatedCBindings][fidl-attributes-for-deprecated-c-bindings],
[MaxBytes][fidl-attributes-max-bytes],
[MaxHandles][fidl-attributes-max-handles]), code generation
([Transitional][fidl-attributes-transitional],
[Transport][fidl-attributes-transport]), desired API availability
([Discoverable][fidl-attributes-discoverable]), and so on. In addition to these
"official" attributes, FIDL authors are allowed to define their own "custom"
attributes, which do not affect compilation, but are still attached to the
resulting [FIDL JSON Intermediate Representation][fidl-json-ir] for potential
downstream use.

Three properties of the existing attributes syntax are lacking:

* Multiple attributes are written as a single, comma-separated declaration,
like so: `[Discoverable, NoDoc, Transport = "Channel"]`. This is inflexible, and
it can be unclear that the attributes are independent of one another.
* Attributes are currently only able to take either zero or one argument, but
not more.
* When attributes do take arguments, the values are always strings. For example,
the MaxBytes attribute takes a stringified number as its argument, like so:
`[MaxBytes="128"]`. This is confusing, especially when the remainder of FIDL
is predictably typed.

The FIDL syntax is currently undergoing a major migration as part of the
[RFC-0050: Syntax Revamp][rfc-0050] effort. This presents a good opportunity to
implement syntax changes for attributes as well.

## Design

### Syntax

Each attribute is a single declaration, with the name of the attribute
(lower_snake_cased by convention) being directly preceded by an `@` symbol. For
declarations that carry multiple attributes, there is no preferred ordering
between attributes. For example, the attribute declaration `[Discoverable,
NoDoc, Transport = "Channel"]` may now be written as:

```fidl
@discoverable
@no_doc
@transport("Channel")
protocol P {};
```

If necessary, the attribute name is optionally followed by a set of parentheses,
containing a comma-separated list of one or more arguments. An argument may be
any valid FIDL constant. For example:

```fidl
const MAX_HANDLES uint64 = 32;

@max_handles(MAX_HANDLES)
type S = struct {};

@transport("Channel")
protocol P {};
```

Arguments must be denoted with a lower_snake_cased "keyword" syntax similar to
that [found in Python][python-decorator-syntax], except for attributes that only
take one argument, which must omit the keyword. Argument typing and the presence
of required arguments are only validated for attributes native to fidlc. For
such attributes, the compiler may default omitted optional arguments to some
predetermined value. Consider the following example usages of a mock `@native`
attribute which takes two required arguments (`req_a` and `req_b`), and two
optional ones(`opt_a` and `opt_b`):

```fidl
const C bool = true;
@native(req_a="Foo",req_b=3)                  // OK: neither opt arg is set
@native(req_a="Foo",req_b=3,opt_c=C)          // OK: only opt_a is set
@native(req_a="Foo",req_b=3,opt_d=-4)         // OK: only opt_b is set
@native(req_a="Foo",req_b=3,opt_c=C,opt_d=-4) // OK: both opt args are set
@native(req_a="Foo",req_b=3,opt_d=-4,opt_c=C) // OK: arg order is irrelevant
@native(opt_d=-4,req_a="Foo",req_b=3)         // OK: arg order is irrelevant
@native(req_b=3)                              // Error: missing req_a
@native(req_a="Foo")                          // Error: missing req_b
type S = struct {};
```

Author-defined custom attributes are schema-less, and are described solely by
their presence in a FIDL file. The compiler has no way of validating the number
of arguments, whether or not they are required, and what their proper types are
(though this may change should we choose to implement "In-FIDL" attribute
schemas, as described in the
[Alternatives](#drawbacks-alternatives-and-unknowns) section below). Thus, only
the grammatical correctness of the attribute signature is enforced. An example
of an author-defined attribute, `@custom`, is shown here:

```fidl
@custom(a="Bar",b=true) // OK: grammatically correct
@custom("Bar",true)     // Error: bad grammar - multiple args require keywords
@custom("Bar")          // OK: correct grammar (though signature now unclear)
@custom(true)           // OK: correct grammar (signature even more unclear)
@custom()               // Error: bad grammar - cannot have empty arguments list
@custom                 // OK: grammatically correct
type S = struct {};
```

Formally, the [modified BNF grammar][fidl-grammar] for the new FIDL attribute
syntax is:

```
attribute = "@", IDENTIFIER , ( "(" , constant | attribute-args, ")" ) ;
attribute-args = attribute-arg | attribute-arg, "," attribute-args;
attribute-arg = IDENTIFIER , "=" , constant;
```

[RFC-0040: Identifier Uniqueness][rfc-0040] applies to attribute names. This
means that different casings and consecutive underscores in attribute names are
reduced to a single common name during canonical name resolution. The following
example thus causes a scoping clash, generating an error at compile time:

```fidl
@foo_bar
@FooBar   // Error: re-used attribute name "foo_bar"
@fooBar   // Error: re-used attribute name "foo_bar"
@Foo_Bar  // Error: re-used attribute name "foo_bar"
@foo__bar // Error: re-used attribute name "foo_bar"
@FOOBar   // Error: re-used attribute name "foo_bar"
type S = struct {};
```

### JSON IR

The [FIDL JSON Intermediate Representation][fidl-json-ir] schema for attributes
reflects the new syntax. The schema's [attribute
definition][fidl-json-schema-attribute] now has a `location` field to track the
in-file position of the attribute declaration. The `value` field has been
replaced by an `arguments` field, which stores the value of each argument as a
`name`/`value` pair, with the latter taking the schema of a
`#/definitions/constant`. For attributes that only accept one argument, and thus
have no keyword `name` in the source, the sole argument's name defaults to
`"value"`.

Additionally, `compose` stanzas and `reserved` table/union members were
previously not able to carry attributes. This RFC rectifies these oversights,
adding a new `#/definitions/compose` definition as a property on
`#/definitions/interface`, and formally attaching a maybe_attributes property to
`#/definitions/table-member`.

In sum, this RFC introduces three new schema definitions
(`#/definitions/attribute-arg`, `#/definitions/attribute-args`, and
`#/definitions/compose`) and modifies three others (`#/definitions/attribute`,
`#/definitions/interface`, and `#/definitions/table-member`). The new attribute
definitions will appear as follows:

```json5
"attribute-arg": {
  {
    "description": "Definition of an attribute argument",
    "type": "object",
    "required": [
        "name",
        "value",
    ],
    "properties": {
      "name": {
        "description": "Name of the attribute argument",
        "type": "string",
      },
      "value": {
        "description": "Value of the attribute argument",
        "$ref": "#/definitions/constant",
      },
      "location": {
        "description": "Source location of the attribute argument",
        "$ref": "#/definitions/location"
      },
    },
  },
},
"attribute-args": {
  {
    "description": "Definition of an attributes argument list",
    "type": "array",
    "items": {
      "description": "List of arguments",
      "$ref": "#/definitions/attribute-arg",
    },
  },
},
"attribute": {
  {
    "description": "Definition of an attribute",
    "type": "object",
    "required": [
        "name",
        "arguments",
        "location",
    ],
    "properties": {
      "name": {
        "description": "Attribute name",
        "type": "string",
      },
      "arguments": {
        "description": "Attribute arguments",
        "$ref": "#/definitions/attribute-args",
      },
      "location": {
        "description": "Source location of the declaration",
        "$ref": "#/definitions/location",
      },
    },
  },
},
"compose": {
  {
    "description": "Compose member of an interface declaration",
    "type": "object",
    "required": [
        "name",
        "location",
    ],
    "properties": {
      "name": {
        "$ref": "#/definitions/compound-identifier",
        "description": "Name of the composed interface"
      },
      "maybe_attributes": {
        "description": "Optional list of attributes of the compose declaration",
        "$ref": "#/definitions/attributes-list",
      },
      "location": {
        "description": "Source location of the compose declaration",
        "$ref": "#/definitions/location",
      },
    },
  },
},
```

Continuing with the `@native` example from the previous section, the
`maybe_attributes` field of its JSON IR output for `type S` would be:

```json5
"maybe_attributes": [
  {
    "name": "native",
    "arguments": [
      // Note: the omitted opt_d is not included in the IR
      {
        "name": "req_a",
        "value": {
          "kind": "literal",
          "value": "Foo",
          "expression": "\"Foo\"",
          "value": {
            "kind": "string",
            "value": "Foo",
            "expression": "\"Foo\"",
          },
        },
      },
      {
        "name": "req_b",
        "value": {
          "kind": "literal",
          "value": "3",
          "expression": "3",
          "literal": {
            "kind": "numeric",
            "value": "3",
            "expression": "3",
          },
        },
      },
      {
        "name": "opt_c",
        "value": {
          "kind": "identifier",
          "value": "true",
          "expression": "C",
          "identifier": "example/C",
        },
      },
    ],
    "location": {
        "filename": "/path/to/src/fidl/file/example.fidl",
        "line": 4,
        "column": 0,
        "length": 36,
    },
  },
],
```

### Case Study: FIDL Versioning

[RFC-0083: FIDL Versioning][rfc-0083] describes an attribute-based syntax for
appending versioning metadata to FIDL declarations and their members. In
particular, it defines a new attribute, `@available`, which takes the optional
arguments `platform`, `since`, `removed`, `deprecated`, and `note`. For example:

```fidl
@available(since=2)
type User = table {
  // Was created with struct at version 2, so no @available attribute is needed.
  1: is_admin bool;
  // Deprecated in favor of, and eventually replaced by, the name field.
  @available(deprecated=3,removed=4,note="use UTF-8 `name` instead")
  2: uid vector<uint8>;
  // The current (unreleased) version constrains this definition.
  // Placing "removed" before "since" is discouraged, but won't fail compilation.
  @available(removed="HEAD",since=3)
  3: name string;
  @available(since="HEAD")
  3: name string:60;
};
```

It is worth noting that arguments that reference numbered platform versions
(namely `since`, `removed`, and `deprecated`) may also take the special string
`"HEAD"`. This means that these arguments do not have a single, easily
resolvable type such as `uint8`. Such constructs are permitted because the
specific type validation rules for an "official" attribute like `@available` are
hard-coded into the compiler itself. Other, more subtle rules, such as the
restriction that only `@available` attributes attached to `library` declarations
may carry the `platform` argument, are handled in this bespoke manner as well.

## Implementation

This proposal will be implemented as part of the broader [RFC-0050][rfc-0050]
FIDL syntax conversion. All FIDL files written in the "new" syntax will be
expected to conform to the changes laid out in this RFC, and the formal FIDL
grammar will be updated to reflect its design at the same time as the rest of
RFC-0050.

In addition, the [schema definition][fidl-json-schema] will be updated to
accommodate the JSON IR changes specified by this document. However, it is
important to keep the JSON IR schema static during the actual migration to the
syntax defined by RFC-0050, as ensuring that the pre- and post-migration
syntaxes produce the same IR will be an important tool for verifying the
accuracy of the new compiler. Thus, the update to the [attribute
definition][fidl-json-schema-attribute] proposed by this document must happen
prior to any RFC-0050 migration, to ensure that the JSON IR does not change
while that process is ongoing.

## Performance

These syntax changes are unlikely to have a performance impact.

## Security Considerations

These syntax changes are unlikely to have a major security impact. They do have
the minor benefit of making potential future security-validating attributes
easier to write and reason about.

## Privacy Considerations

These syntax changes are unlikely to have a major privacy impact. They do have
the minor benefit of making potential future privacy-validating attributes
easier to write and reason about.

## Testing

These syntax changes are unlikely to have a major testing impact. They do have
the minor benefit of making potential future test-instrumenting attributes
easier to write and reason about.

## Documentation

All relevant documentation and examples will be updated to feature the new
syntax as part of the broader [RFC-0050][rfc-0050] documentation update. In
particular, the [reference documentation][fidl-attributes] for official FIDL
attributes will be updated to reflect the rules stipulated by this design, and
to note valid argument types and implied defaults for each attribute.

## Drawbacks, Alternatives, and Unknowns

### In-FIDL Attribute Schemas

There is an almost infinite design space of possible syntaxes, and this section
will not try to address all of them. However, one option that received serious
consideration was allowing the interfaces of annotation functions to be defined
"with FIDL." This alternative syntax and the rationale for its rejection are
described below.

Consider the following FIDL file, with the interface of its custom attribute
defined inline:

```fidl
type MyAttrArgs = table {
  1: foo string;
  2: bar int32;
  3: baz bool;
};
@myAttr(MyAttrArgs{foo: "abc", bar: 1, baz: false})
type MyStruct = struct {...};
```

This design has the benefit of "eating our own dogfood:" FIDL is an interface
definition language, so why not define the interfaces to our built-in,
compiler-aware attribute functions with it as well? Further, this allows for
custom, user-defined attributes to be enabled at some point in the future,
though how the compiler would attach such user-defined meta information to the
generated bindings code remains an open question.

This design path is ultimately rejected for trying to do too much. None of the
use cases for attributes in the foreseeable future require such in-language
definition capabilities, and it is not even clear that enabling user-defined
attributes is a desirable future goal. The syntax proposed by this document is
simpler and more familiar to users of other popular languages such as Rust and
Python.

Nothing in the current design explicitly precludes the implementation of In-FIDL
attribute schemas, so this alternative remains a viable option for future
extensions to the attributes syntax.

### Use String Literals for Attribute Arguments

One possible alteration to the design specified in this document is to refrain
from allowing typed arguments, and to instead retain the old regime of requiring
that all argument values be string literals. Consider the following example
illustrating such a design:

```fidl
const MAX_STR string = "123";
const MAX_NUM uint64 = 123;

@max_handles(123)     // Error: cannot use non-string types
@max_handles(MAX_STR) // Error: cannot use const identifiers
@max_handles(MAX_NUM) // Error: cannot use non-string types or const identifiers
@max_handles("123")   // OK: only string literals are allowed
type S = struct {};
```

The upside of this design is the simplicity of implementation it affords. To
accommodate typed attribute arguments, backends will need a relatively more
complex ingestion logic that properly accounts for the various possible types
for each of the attributes' arguments. Where previously a simple cast from the
string `"123"` to the int `123` would have sufficed, backends now need to handle
the entirety of the `#/definitions/constant` schema. This additional
implementation cost is multiplied by the number of backends being supported.

The benefit of allowing typed attributes is that it centralizes this type
casting logic. For example, consider the attribute declaration
`@my_custom_attr("3.")`. If each backend is expected to do its own type casting,
one may decide that `"3."` is a valid value to cast as an integer, while another
may not. It would be difficult to catch all subtleties of this ilk, leading to
backends inevitably diverging in their attribute implementations. Enshrining one
canonical understanding of how attribute types behave in fidlc eliminates such
inconsistencies.

### Rejected Minor Alterations

The attribute syntax described in this document explicitly states that the
ordering between multiple attributes attached to a single declaration is
irrelevant. An alternative to this would have been to enforce alphabetical
order. This was rejected because author-defined custom attributes, as well as
future fidlc-native attributes, may reference one another in ways that may
benefit from using specific orderings for clarity. Consider the following two
author-defined custom attributes, whose order would need to be confusingly
reversed if alphabetical ordering were required:

```fidl
@this_attr("Foo")
@test_for_this_attr(false)
protocol P {};
```

Additionally, camelCase was considered as the recommended casing convention for
the attribute syntax. This recommendation was ultimately rejected, as none of
the other casing suggestions in the [FIDL Style Guide][fidl-style-guide]
implement camelCase, and it was judged to be overly confusing to add a new
casing style for attributes alone.

## Prior Art and References

This RFC is an evolution of the syntax defined in [RFC-0050: Syntax
Revamp][rfc-0050], and will thus modify the formal [FIDL Grammar][fidl-grammar].

This proposal drew on a number of existing "attribute-like" implementations in
other languages, specifically:

* Python's [decorator syntax][python-decorator-syntax] and
[keyword argument][python-keyword-syntax] design serve as inspiration for
some of the design choices seen in this document, such as the use of the `@`
symbol and referencing arguments by keyword.
* Rust's [attribute specification][rust-attribute-syntax], while superficially
different on some syntactic choices, is also conceptually similar to this
proposal.
* Cap'n Proto's [annotations schemas][capn-proto-annotation-syntax] were
considered as a reference point for a potentially more featureful alternative to
the syntax proposed in this document.

[capn-proto-annotation-syntax]: https://capnproto.org/language.html#annotations
[fidl-attributes]: reference/fidl/language/attributes.md
[fidl-attributes-discoverable]: reference/fidl/language/attributes.md#discoverable
[fidl-attributes-doc]: reference/fidl/language/attributes.md#doc
[fidl-attributes-for-deprecated-c-bindings]: reference/fidl/language/attributes.md#layout
[fidl-attributes-max-bytes]: reference/fidl/language/attributes.md#maxbytes
[fidl-attributes-max-handles]: reference/fidl/language/attributes.md#maxhandles
[fidl-attributes-no-doc]: reference/fidl/language/attributes.md#nodoc
[fidl-attributes-transitional]: reference/fidl/language/attributes.md#transitional
[fidl-attributes-transport]: reference/fidl/language/attributes.md#transport
[fidl-grammar]: reference/fidl/language/grammar.md
[fidl-json-ir]: reference/fidl/language/json-ir.md
[fidl-json-schema]: https://cs.opensource.google/fuchsia/fuchsia/+/main:tools/fidl/fidlc/schema.json;drc=ed2a2cddbd2257595ff9fd8e9c4d151b291edec1
[fidl-json-schema-attribute]: https://cs.opensource.google/fuchsia/fuchsia/+/main:tools/fidl/fidlc/schema.json;l=1183;drc=ed2a2cddbd2257595ff9fd8e9c4d151b291edec1
[fidl-style-guide]: development/languages/fidl/guides/style.md#usage
[python-decorator-syntax]: https://docs.python.org/3/reference/expressions.html#calls
[python-keyword-syntax]: https://www.python.org/dev/peps/pep-0318/
[rfc-0040]: contribute/governance/rfcs/0040_identifier_uniqueness.md
[rfc-0050]: contribute/governance/rfcs/0050_syntax_revamp.md
[rfc-0083]: contribute/governance/rfcs/0083_fidl_versioning.md
[rust-attribute-syntax]: https://doc.rust-lang.org/reference/attributes.html
