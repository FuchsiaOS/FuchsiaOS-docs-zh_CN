{% set rfcid = "RFC-0066" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

Note: Formerly known as [FTP](../deprecated-ftp-process.md)-006.

## Summary

The FIDL specification doesn't state whether primitive and enum struct
fields have default values.
This tuning proposes that we document explicitly that they do not.

# Motivation

Requiring initialization is challenging in some languages and impossible
in others.
This proposal leaves the door open to the lowest common denominator but
provides a policy for higher-level languages to follow.

The lack of clarity about default values in structs for some types results
in some disagreement.
Language bindings are inconsistent in their  handling of implicit and explicit
defaults.
It's clear that nullable types default to null and arrays and vectors default
to empty but not others.
The C++ bindings default primitive types to false, 0 or 0.0 but the Dart
bindings require values to be specified when a struct is constructed if no
default is supplied in the FIDL definition.

Often zero values are great defaults but they should be explicitly declared.
For example, if a `uint32_t` is representing an FTP number then 0
isn't a valid value but FIDL has no way to express that a caller should
specify a number.

# Design

This is primarily a documentation clarification.
It merely clarifies the semantics expressed in FIDL interfaces.
It opens up opportunities for bindings improvements but does not mandate them.

The [FIDL language specification][fidl-language] should include the following
information,  possibly in a different form:


> Primitive and enum fields in structs that don't have defaults values
> declared in the FIDL file SHOULD be specified when instantiated in
> bindings.
> Bindings authors MAY respect default values, if the host language makes
> that possible, and if that behavior is common and expected by programmers.
> For instance, in Dart or C++ it is common to have default values.
> In Go however, structs are initialized by default, and the idiomatic pattern
> to provide standard initialization is to offer a NewMyStruct() function.
> In C, no initialization is expected, instead programmers must explicitly
> define all fields. Resorting to a MACRO may be appropriate.
> If bindings respect default values, then they: MUST respect all default values
> provided, and MUST report an error if a programmer fails to initialize
> non-defaulted fields.

# Documentation and examples

The language specification and tutorial should be updated to reflect this
change.

# Backwards compatibility

Existing behavior varies between different language bindings.
This change  allows all existing behavior and encourages better future behavior.

# Performance

No impact.

# Security

This clarifies the specification and makes accidental misuse of interfaces
more difficult.
These are good for security.

# Testing

No testing is required.

# Drawbacks, alternatives, and unknowns

An alternative would be to formally define that:

```fidl
struct Foo {
    int32 bar;
    bool baz;
    float32 wux;
};
```

is semantically equivalent to:

```
struct Foo {
    int32 bar = 0;
    bool baz = false;
    float32 wux = 0.0;
};
```

but as outlined [above](#motivation) this may fail to capture important semantics.

A previous iteration of this proposal included strings but at the time of
writing they're still nullable so have a way of indicating that they're
required or optional.

# Prior art and references

n/a

<!-- xrefs -->
[fidl-language]: /reference/fidl/language/language.md
