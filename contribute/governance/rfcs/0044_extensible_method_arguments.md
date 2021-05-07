{% set rfcid = "RFC-0044" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

Note: Formerly known as [FTP](../deprecated-ftp-process.md)-044.

## Rejection rationale

Obsoleted by [RFC-0050](/docs/contribute/governance/rfcs/0050_syntax_revamp.md).

## Summary

We encourage FIDL library authors to use tables rather than structs when
extensibility is required however method arguments are encoded as structs. This
proposes a way to have extensible method arguments built on top of tables.

## Motivation

The lack of syntax for extensible method arguments discourages library authors
from using tables to make method arguments extensible. By including this in the
syntax it will put extensibility considerations front-and-center when designing
protocols.

The Modular team is designing protocols that need to both maintain ABI
compatibility and be extensible as they discover new requirements and evolve
their designs. They're considering defining their methods in terms of tables
that they're declaring out of line.

## Design

This proposal extends the FIDL source language and affects the language
bindings.

### FIDL Syntax

It proposes an extension to the syntax for method and event request and response
arguments to add a table to the argument structs.

For example this protocol:

```fidl
protocol Example {
    Foo(int32 arg1, { 1: string arg2, 2: bool arg3 }) -> ({});
};
```

declares a method with one required argument, two optional arguments, an
extensible request and a response that is extensible. It's equivalent to
declaring:

```fidl
table ExampleFooRequestExtension {
    1: string arg2;
    2: bool arg3;
};
table ExampleFooResponseExtension {
};
protocol Example {
    Foo(int32 arg1, ExampleFooRequestExtension extension)
        -> (ExampleFooResponseExtension extension);
}
```

### IR

The current version of the IR can be extended in a backward-compatible way.
Method arguments that are extensible with have tables named:
`{Protocol}{Method}RequestExtension`, `{Protocol}{Method}ResponseExtension` or
`{Protocol}{Method}EventExtension` included. The `[ExtensionArgument]` attribute
will be set on these generated tables so that bindings generators can handle
them specially if they wish.

A future IR may elevate this idea to a more first-class structure. A future IR
should allow a more flexible approach to naming of declaration so that languages
could make better choices in naming the extension tables, for example C++ could
nest them inside the protocol definition class.

### Bindings

Bindings don't need special support for extensible method arguments. Existing
bindings generators will simply include the generated tables as the last
argument to methods.

#### C++

For example the C++ bindings, without any changes for the protocol above would
roughly look like:

```cpp
class ExampleFooRequestExtension;
class ExampleFooResponseExtension;
class Example {
  using FooCallback = fit::function<void(ExampleFooResponseExtension)>;
  virtual void Foo(int32_t arg1,
                   ExampleFooRequestExtension extension,
                   FooCallback callback) = 0;
};
```

An alternative way to bind this protocol would be:

```cpp
class Example {
  using FooCallback = fit::function<void()>;
  virtual void Foo(int32_t arg1,
                   FooCallback callback,
                   std::optional<std::string> arg2 = std::optional<std::string>(),
                   std::optional<bool> arg3 = std::optional<bool>()) = 0;
};
```

This would more closely match how the method was declared but would put the
callback argument in between the static and extensible arguments.

#### Dart

Dart's support for optional, named arguments allows a nice mapping of the FIDL
concept to its syntax. Dart's lack of support for tuples or variadic futures
remains limiting. The binding interface might look like:

```
abstract class Example {
  Future<ExampleFooResponseExtension> foo(int arg1, {String arg2, bool arg3});
}
```


With this syntax adding additional extension arguments preserves source
compatibility.


#### Rust

TBD


#### Go

TBD


#### Simple C

The simple C bindings don't support tables so this feature would be incompatible
with them.

## Implementation strategy

The first step would be to add support for the new syntax into `fidlc` and
update the reference and tutorial documentation.

Next we would add support to the Dart bindings because they're where the most
obvious ergonomic benefits exist.

## Ergonomics

Allowing the evolution of FIDL protocols is an important feature that users are
seeking. Currently changing the arguments of a method is an ABI and API breaking
change. The approaches to softening that are either to introduce new method
names for each change and continuing to support the old method as long as there
are still callers, or including a table as an argument and adding new arguments
to that table. This proposal allows the expression of the latter approach in a
more ergonomic way. It keeps arguments defined in the method definition so that
they can be easily referred to in documentation comments.

The idea of optional arguments is common in many programming languages. It won't
be a surprising concept to library authors.

The need for explicit ordinals in tables makes extension arguments inconsistent
with required arguments but it's better to keep them consistent with tables
rather than introduce a new table-like structure with hashed ordinals.

## Documentation and examples

As an extension to the FIDL language the FIDL reference and tutorial
documentation should be updated.


## Backwards compatibility

Existing FIDL libraries aren't affected by this change.

This proposal significantly improves the ability library authors to maintain ABI
compatible interfaces over the long-term.

Constraints around source compatibility are still TBD and will be informed by
how we plan to bind to our supported languages.

## Performance

Tables are more expensive to encode and decode than structs so performance
critical protocols should use this feature sparingly

## Security

No impact

## Testing

Tests should be added to `fidlc`. The dangerous identifiers test should test the
use of dangerous identifiers as optional arguments.

## Drawbacks, alternatives, and unknowns

### Alternatives

We could switch from structs to tables on a per-method or per-protocol basis. We
could even switch the default from being structs to tables. This approach is
less flexible. Often only the request or the response is expected to be
extended. Often some of the arguments (for example in Modular's case a module
id) is expected to remain stable in the long term while others are not.

Instead of using tables and requiring ordinals for each extended argument we
could define a table-like data structure that hashes names to calculate
ordinals. This would simplify the syntax of the source language but add
complexity to the encoders and decoders and language bindings.

We could have versioned methods. This option wasn't explored in depth.

### Open questions

We should decide how this will be bound in C++, Rust and Go.

Should adding arguments be allowed to break source compatibility?

## Prior art and references

Protobuf declares messages out of line from protocol methods.

Flatbuffers and cap'n proto use versioning.
