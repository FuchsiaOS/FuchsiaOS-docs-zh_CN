{% set rfcid = "RFC-0021" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

Note: Formerly known as [FTP](../deprecated-ftp-process.md)-021.

## Summary

We propose declaring a new attribute that allows code to build regardless of
whether the method is implemented or not.

# Motivation

Since moving the Fuchsia tree to the flower model of global integration hard
breaking changes have become ... hard.
This derives from the fact that to implement a FIDL interface in a binding the
developer must implement exactly the set of methods that were defined in the
FIDL interface in their concrete implementation.
This means if a method is added to or removed from an interface in one petal
then global integration will fail.

# Design

We should declare a new attribute `[Transitional="description"]` that instructs
bindings to generate code that will successfully build whether the method is or
is not implemented.

Calling a transitional method is implementation defined &mdash; it may work as
documented, may never complete or even may cause the caller or callee to crash.
It must not interfere with the functioning of other methods and it must be
possible to implement the method.
The FIDL frontend compiler wouldn't need to change at all, only the language
bindings.

## C

* It isn't a build-time error to fail to implement a method in the C bindings so
  there's nothing to do here.

## C++
* Instead of declaring methods as pure virtual functions declare them with a
  concrete base implementation that simply prints an error.

## Dart
* Instead of declaring methods without a body, declare them with a body that
  returns a failed Future or throws an exception (depending on the binding style).

## Go
* Transitional methods can have a default implementation on a newly introduced
  struct 'InterfaceStubBase' which can be embedded in the actual
  implementation struct to provide forward/backwards compatibility.

## Rust
* TBD

# Implementation strategy

Once we have an approach to long-term evolution we will remove this
functionality from FIDL.

# Documentation and examples

# Backwards compatibility

See [Drawbacks](#drawbacks_alternatives_and-unknowns), below.

# Performance

No performance impact when not used.
Potential additional indirection by using dynamic dispatch for `[Transitional]` methods,
instead of a more direct calling strategy.

# Security

No impact, transitional methods fail fast.

# Testing

Code generation on before/during/after libraries to simulate adding or removing methods
and events using `[Transitional]` attribute, and ensuring compilation succeeds.

# Drawbacks, alternatives, and unknowns

This does not offer a way forward for long-term evolution of FIDL interfaces.
It does not provide for renaming methods or changing their signature, though it
could be used as part of a multi-stage process for that.
It does not solve the problem of adding variants to unions.

# Prior art and references

N/A

