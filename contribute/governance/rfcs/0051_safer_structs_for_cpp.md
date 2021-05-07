{% set rfcid = "RFC-0051" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

Note: Formerly known as [FTP](../deprecated-ftp-process.md)-004.

## Summary

Allow C++ developers to write FIDL code that will break at compile
time if structs aren't fully initialized.

## Motivation

In Peridot we have complex FIDL structs that we're changing as we better
understand how to solve the problems we're tackling.
The structs are often deeply nested and sent in code far from where
they're constructed.
When iterating on structs we often make breaking changes to the semantics,
adding required fields or making previously optional fields required.
It's difficult to track down all of the code that needs to be updated.
These do not appear as compile time errors, but as runtime errors, which are
difficult to correlate with the code that has incorrectly initialized
the structs.

The same class of issues was prevalent in Dart code until a change
was made to require all required fields to be passed into the struct
constructor.
This change has made developing Dart code much more efficient and robust.

## Design

This modifies the C++ bindings library and code generator.
It does not remove any existing interfaces but simply adds a new way to
construct instances of FIDL structs.

This adds a builder pattern for FIDL structs.
Using it looks like:

```fidl
FooPtr foo = Foo::Builder()->set_bar("hello")->set_baz("world");
```

The **Builder()** static method on a struct class returns a templated builder
object.
The builder template params capture the type of the struct being built and
classes for each unset field on the struct.
It holds an instance of the struct.

Field classes have two methods: a `set_`*name*`(value)` method that sets the
field value on the instance and returns a builder with the field removed from
the builder's template arguments, and a `Check()` method that is a no-op for
optional fields and a `static_assert` failure for required fields.

The builder class extends all of the field types in its template
parameters so that the developer has access to the setter methods.
As the developer calls setters and receives new builder types the list of
field classes in the builder template arguments shrink.
For example, eliding some of the template shenanigans:


`Foo::Builder()` is a `Builder<Foo, Foo::Field_bar, Foo::Field_baz>`
with `set_bar()` and `set_baz()` methods.

`Foo::Builder()->set_bar(...)` is a `Builder<Foo, Foo::Field_baz>`
with a `set_baz()` method.

`Foo::Builder()->set_bar(...)->set_baz(...)` is a `Builder<Foo>`
without any setter methods.

Builders have implicit conversion operators to the struct type and
struct pointer types.
These call the `Check()` method on the remaining field types and return the
struct instance held by the builder.
The `Check()` methods will either be no-ops (for optional fields) or
`static_assert` failures specifying which required field hasn't been set.

## Documentation and examples

The [FIDL tutorial] and examples will be updated to demonstrate the
traditional and new ways of making a struct instance.

## Backwards compatibility

This proposal is purely additive.
It introduces no backwards incompatibility.

## Performance

This change has no runtime performance cost.
It was [prototyped in Compiler Explorer](https://godbolt.org/g/LXXfZF)
specifically to ensure that no additional code would be generated or
executed.

It adds a new header file to the bindings library and a few extra
lines per struct field in the generated C++ code.
The C++ compiler has to do a little extra work to resolve the templates
but it doesn't add any additional steps to compilation that would have
a significant impact.

## Security

This change allows us to turn programmer mistakes from runtime errors
into build time errors.
This reduces the state space of the program and reduces the number of error
cases that must be correctly handled and tested.
This reduction in unexpected behavior is good for security.

## Testing

The C++ bindings unit tests should be extended to test that builders
are correctly setting different types of field.

It's challenging to test that incorrect use of the builder (i.e.,:
failing to set a required field) is caught by the compiler.
It's unclear how that should be tested.

## Drawbacks, alternatives, and unknowns

This adds some fairly tricky templates to the FIDL C++ bindings library.
This introduces a maintenance burden and potentially some small
build-time overhead.

A previous template approach used a bitmask, which had simpler templates
but imposed limits like 64 required fields and added complexity to
the FIDL compiler.

We could also build a linter that tried to track that the required fields
were all set.
This seems like a pretty complicated dataflow analysis.

## Prior art and references

The Dart bindings were changed last year so that struct constructors take
named arguments for each field.
The required ones are marked as required so that the dartanalyzer can reject
changes that leave some fields uninitialized.

<!-- xrefs -->
[FIDL tutorial]: /docs/development/languages/fidl/tutorials/overview.md
