<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0160" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Summary

This RFC proposes removing the ability to specify default values on FIDL
struct fields.

## Motivation

It is currently possible to specify default values on struct fields, as
shown in the following example:

```
type MyStruct = struct{
  x int8 = 123; // 123 is the default value
};
```

The intention is for struct defaults to be a mechanism for all bindings to
apply an identical value to unassigned fields, resulting in consistent
behavior between bindings. However, in practice this is not the case.
Today, only the HLCPP and Dart bindings actually support for struct default
values. In fact, in some bindings such as Go, significant
restructuring of domain objects would be needed to support defaults
as there is no language-level support for defaults on fields.

Further, while default field values on request and response structs can be
specified in the FIDL language, they are ignored and the generated code for
requests and responses lacks default values. This is not only due to lack
of implementation - it is difficult to expose default values in our current
API. For instance, in C++ bindings replies are made through function calls
but C++ only supports trailing default arguments for function call parameters.

Adding to the inconsistency, tables and unions don't have support for field
defaults due to the complication and subtleties in adding support - these
evolvable types have even more differences across bindings and implementation
complexities than structs.

Therefore, there is inconsistent support for defaults with no clear path
to achieving full support. Rather than continuing to provide an inconsistent
experience, removing support altogether is a better option.

Aside from implementation concerns, there are also conceptual motivations
for removing support for struct default values. Defaults are more subtle
than consts in that they aren't explicitly assigned by bindings users and
this subtlety has the potential to surprise users and lead to bugs. A user
needs to know whether a particular type has a default or not and whether a
given binding supports defaults or not to know how to use a given type, which
is rather complex.

## Stakeholders

_Facilitator:_ hjfreyer@google.com

_Reviewers:_

ianloic@google.com, yifeit@google.com


_Consulted:_

azaslavsky@google.com, mkember@google.com


_Socialization:_

An email discussion on eng-council-discuss@fuchsia.dev

## Design

It will not be possible to specify default values on struct fields in FIDL.

## Implementation

There are currently 119 usages of struct defaults in 13 FIDL libraries, 68 of
which use non-zero values.

Default values will be deprecated in the immediate term, then eventually
removed.

During the deprecation phase, an "@allow_deprecated_struct_defaults"
(or similar) annotation will be introduced to enable struct defaults in a given
file.

Over time, default value usages will be removed and the directives enabling
struct defaults will be removed alongside them. At this point, support will be
removed entirely.

## Performance

There is no performance impact.

## Ergonomics

In bindings that support generating default values, this RFC may seem
a step back in terms of ergonomics. However, removing struct default support
will improve consistency between bindings and many use cases can be replaced
with consts.

## Backwards Compatibility

This RFC breaks support for an existing feature, however the implementation
will involve a long deprecation period which will reduce the impact.

Regarding compatibility across FIDL source changes, this RFC removes a hazard
where a change in a struct default value results in a complex behavior change
that is hard to reason about. For instance, code that explicitly sets the
initial default value will now behave differently than code that did not set
a default value at all. Further, in different scenarios it might be desirable
for all values to change to the new default or for all of the values to be
unchanged.

## Security considerations

Removing defaults may appear to present a security risk because in certain
cases defaults may be initializing otherwise uninitialized memory. However,
in practice this is not the case. Both HLCPP and Dart, the two bindings that
currently support defaults, ensure that struct fields are initialized
regardless of whether a default is provided.

Given this, there shouldn't be a meaningful impact on security.

## Privacy considerations

There is no privacy impact.

## Testing

During the deprecation phase, tests will be added to ensure no new struct
defaults can be added.

## Documentation

The documentation for the struct default value feature will be removed.

## Drawbacks, alternatives, and unknowns

### Default Values for Documentation

Some users use struct default values for documentation in lieu of comments.
Default struct field values have the benefit of being a more formalized part
of the language and are type-checked. However, without the default value
feature, it will still be possible to document the value that users should
initialize a field to, but it will instead be done through comments or const
definitions.

### Comparison with consts

Defaults can be superior to consts in situations where the consts would be used
as defaults. Defaults attach a value to a field rather than the value being
treated as a global and the default value is automatically applied instead
of needing to be manually specified by the user.

Despite all of this, defaults can be replaced by consts which are more explicit
and only require a bit more work for the user.

### More expressive consts

It is possible that consts will become more expressive in the future and it
will be possible to define struct consts.

For instance:

```
type MyStruct = struct { x int8; };

const DEFAULT_MY_STRUCT = MyStruct{ x: 123 };
```

Then, in bindings `DEFAULT_MY_STRUCT` can be copied into a variable
(e.g. `myStruct := DEFAULT_MY_STRUCT` in Go) and used as a base value on top
of which fields can be set. This has a similar effect to default values in
structs but should be easier to support more broadly across bindings.

### Alternative: Adding broader support for defaults

One might consider adding broader binding support for defaults as an
alternative to deprecation. However, this is impractical.

Consider go. A generated struct looks as follows:

```go
type MyStruct struct {
	_ struct{}
	X int8
}
```

There is no place to automatically populate a default value through this API.
Additionally, the fields can't be "unset", so there is no way to detect an
unset field and populate a default value during the encode stage. A major
API restructuring would be needed to support defaults.

### Alternative: Zero-value support for FIDL

Some languages such as Go and some encoding formats such as protocol buffers
3 have a concept of zero-value. That is, uninitialized fields take on a
canonical "zero" value, usually literally 0.

It is possible to consider adding support for zero values to FIDL. On one hand,
this would improve memory safety, but on the other hand support for zero values
might conflict with FIDL's principle that you "only pay for what you use".
While extra work might be possible to avoid in some cases, in others there
might be an extra step to ensure unset values are zero.

Today, whether or not fields are initialized is a binding-level concern and
there are no top-level mandates as to what the behavior should be. For
instance, the Go language zeros all unset struct fields but FIDL doesn't
mandate that this happens.

This may be sufficient. Whether or not a zero value takes effect depends on
how users interface with the binding API so it seems reasonable for bindings to
individually make a decision on what happens when a field isn't set. Some
bindings might mandate that all fields must be explicitly set, meaning there
are no defaults.

About half of existing FIDL struct default values are zero and half are not.
This means about half of the existing struct defaults would still need to be
removed or the values would need to be "recalibrated" to center around zero
(for instance by using a zero-based index instead of one-based).

A concept of a shared zero value across bindings would give special meaning to
the value zero in FIDL, which may have significance in some protocols but not
others. In many cases, it might be desirable to be explicit and use a named
constant in places where the zero value has significance. This can be done
regardless of whether the constant value is actually the default.

The main goal of this RFC is to stop the spread of use of a broken feature.
Given this and the unclear benefits of a binding-wide mandate of zero-valued
defaults, the introduction of the concept of a zero-value is not included in
the design of this RFC.

## Prior art and references

Custom default values were removed in Protocol Buffers version 3 (
[see docs](https://developers.google.com/protocol-buffers/docs/proto3#default)
) and were replaced with zero-valued defaults.
There are likely several reasons for this change. One reason is that the
absence of defaults makes it possible to implement protobuf with
"plain old structs" that lack accessors
([link](
https://stackoverflow.com/questions/33222551/why-are-there-no-custom-default-values-in-proto3)).
Another reason is that it was discovered that the majority of usages of
defaults assigned the default to the zero value.

In FTP-047 "Required Table Fields", which was rejected, support for default
values on table fields was proposed. This was just part of a larger proposal
so the rejection does not necessarily imply a conclusion on whether default
values should be supported on table fields.
