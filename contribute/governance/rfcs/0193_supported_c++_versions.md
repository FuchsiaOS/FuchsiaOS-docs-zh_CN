<!-- Generated with `fx rfc` -->
<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0193" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
{# Fuchsia RFCs use templates to display various fields from _rfcs.yaml. View the #}
{# fully rendered RFCs at https://fuchsia.dev/fuchsia-src/contribute/governance/rfcs #}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Summary

Drop C++14 support from the SDK and clarify how support for C++ versions will be
decided and supported in the future.

## Motivation

The C++ SDK currently [claims to support C++14][sdk-cxx-docs] but there aren't
effective tests in the Fuchsia tree to prevent C++14 incompatible code from
making its way into the SDK. This breaks users and results in roll-backs.

The Fuchsia tree is built with C++17. Many libraries and frameworks in the
Fuchsia tree that would be useful for out of tree developers depend on C++17
features. In particular the new FIDL C++ bindings require C++17.

There are no longer any users of the SDK that compile as C++14 so changing the
supported version is just a matter of changing the documentation.

## Stakeholders

_Facilitator:_ cpu@google.com

_Reviewers:_ johngro@google.com, dschuyler@google.com, abarth@google.com

_Consulted:_ yifeit@google.com, sebmarchand@google.com, phosek@google.com,
eieio@google.com, schottm@google.com, jamesr@google.com, gevalentino@google.com,
jyknight@google.com

## Design

### Terminology

A _C++ version_ refers to a revision of the [ISO C++ standard][iso-cpp]. We
refer to it by short-hand like _C++14_, _C++17_, _C++20_, etc.

The _SDK C++ versions_ is a list of _C++ versions_ that are
[documented][sdk-cxx-docs] to be supported by the Fuchsia SDK and are used by
the build systems shipped as part of the SDK. That list currently includes
_C++14_ and _C++17_.

### Dropping C++14

The [SDK documentation][sdk-cxx-docs] and [C++ in Zircon][zircon-cxx-docs] SHALL
be updated to reflect that C++14 is no longer supported.

### Fuchsia Tree C++ Version

The _C++ version_ used to compile most code that runs on Fuchsia in the Fuchsia
tree MUST be an _SDK C++ version_. Other _C++ versions_ MAY be used in the
Fuchsia tree for small, isolated codebases or for host-only code, but this is
discouraged.

### C++ Version Testing

A core set of tests MUST be built and run for every _SDK C++ version_ as part of
the Fuchsia commit queue. This set of tests includes all C++ tests in the CTS
and API tests for C++ FIDL bindings, but may include more.

### Adding C++ Versions

A new _SDK C++ version_ may be added by writing an RFC to propse that addition
and having it approved.

Only _C++ versions_ that are published by the ISO C++ committee may be added to
the C++ versions.

As part of adding an _SDK C++ version_ the Fuchsia build MUST be updated to
build and run the C++ tests identified above with the added _C++ version_.

### Removing C++ Versions

_SDK C++ version_s may be removed by writing an RFC to propose that removal and
having it approved.

When removing an _SDK C++ version_ it MUST be removed from the SDK documentation
and SHOULD be removed from the Fuchsia build system.

## Implementation

### Dropping C++14

This is just a documentation change.

### C++ Version Testing

After _C++14_ is dropped _C++17_ will be the only _SDK C++ version_. Tests are
already built and run with _C++17_.

Before another version can be added the Fuchsia build system will need to be
updated to allow tests to be built and run with multiple _C++ versions_.

## Performance

N/A

## Ergonomics

It will be much easier to update C++ code that is shipped in the SDK because
every change will be tested against every supported C++ version.

## Backwards Compatibility

All current (in October 2022) Fuchsia users build with C++17 so there are no
backwards compatibility issues.

## Security considerations

N/A

## Privacy considerations

N/A

## Testing

See above.

## Documentation

The [SDK documentation][sdk-cxx-docs] and [C++ in Zircon][zircon-cxx-docs] will
be updated to reflect that C++14 is no longer supported.

This RFC serves as the documentation for the policy for adding and removing _C++
versions_ in the future.

## Drawbacks, alternatives, and unknowns


## Prior art and references

[sdk-cxx-docs]: /docs/development/idk/documentation/compilation.md
[zircon-cxx-docs]: /docs/development/languages/c-cpp/cxx.md
[iso-cpp]: https://isocpp.org/std/the-standard
