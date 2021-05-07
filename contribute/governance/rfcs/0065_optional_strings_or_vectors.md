{% set rfcid = "RFC-0065" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

Note: Formerly known as [FTP](../deprecated-ftp-process.md)-016.

## Rejection rationale

* FTP-016 is rejected, i.e. we're keeping optional strings and vectors for now:
    * We want to revisit optionality more holistically, rather than doing a
      point decision.
    * Usage of this feature has grown since our thoughts formed about their
      usefulness, and we ought to have a better sense of the patterns that
      exist, and the patterns we want to encourage (and discourage).
* At bindings level:
    * Move to not use optional containers for non-optional things.
      We should make it easier to deal with non-optional things, than
      dealing with optional things -- i.e. you should explicitly seek
      optionality if that is appropriate for the domain.
    * On C++ ergonomics:
        * `StringPtr` -> `std::string` or `fit::optional<std::string>`
        * `VectorPtr` -> `std::vector<T>` or `fit::optional<std::vector<T>>`

## Summary

Remove optional strings and optional vectors from the FIDL language.

## Motivation

NB: Throughout this document I will refer to "the null string" or
"an empty vector".
Since a string on the wire is essentially a `vector<uint8>` that must be valid
UTF-8, the wire formats of these are highly similar, and what applies to
one generally applies to the other.

Nullable vectors and strings are hard and unergonomic to represent in
several target languages, and are not widely used.
While there are use cases that need to distinguish not-a-string from an
empty string, I think there are few enough to be worth forcing those places
to explicitly represent that state of affairs.
I suspect this is even more true should we implement tables, which may
cover several use cases of non-present strings or vectors.

For example, in C and C++, a null `vector<T>` is represented as a zero
length and a null pointer, while an empty `vector` is represented as a zero
length and a nonnull pointer.
This nonnull pointer must be a valid pointer-to-T per the language rules,
even though it is not valid to dereference.
We construct this pointer currently by acting as though the next bit of
secondary object storage is one-past-the-end of an array of T, which is
dubiously legal at best and too subtle in any case.
We've fixed several bugs in the implementation and in clients resulting
from the subtlety of these rules.

The C++ representation is also inefficient in its goal to match the
standard vector and string interfaces, by using a pointer to a standard
container.
Rust similarly must wrap the container in an `Option`.

## Design

This proposal modifies the FIDL language by removing the optional vector
and optional string constructs.

It affects every binding's implementation by removing the need to provide
representations for these constructs.

## Implementation strategy

1. Deprecate these constructs in the FIDL language.
   Ideally we could emit deprecation warnings from `fidlc`.
2. Migrate all uses of `vector?` and `string?` to other representations.
   In some cases the interfaces in question do not actually use the
   optionality.
   In other cases we can manually describe the optionality.
3. Remove `vector?` and `string?` from FIDL, and from examples and
   documentation.
4. For each target language: adopt the better string or vector
   implementation this proposal now allows.
   For example, a FIDL vector can just be a `std::vector` in C++.

## Documentation and examples

These constructs are occasionally referenced in things like `go/fidl-tut`,
but never in an essential way.
We should update them all to non-optional versions.

## Backwards compatibility

This disallows a single construct in the language.
An old compiler will be able to compile future code.

The cost is minor as we are not yet in a state where we expect a wide
window of source/compiler compatibility.

# Performance

I expect essentially no performance change.

# Security

I believe this feature will have no impact on security.

# Testing

I expect testing to proceed 1 CQ at a time as we pull on individual uses
of these constructs.

I don't believe any new tests need to be written for the fidl pipeline.
Only modifications to remove all optional strings or vectors need to
happen.

## Drawbacks, alternatives, and unknowns

I believe we should not place deprecation warnings on generated code.
There end up being many more references to those, and it is onerous to
work backwards to the true source of the warning.

There are some genuine cases that want to distinguish not-a-vector from an
empty vector. However, this is just as true of `uint32`, and I believe we
should reconsider optionality in general after this proposal and after
tables.

## Prior art and references

While other RPC or IPC systems surely face this sort of question, I didn't
look at any of them.
I believe in this case, the design pressures vary wildly in terms of
compatibility with other systems, performance needs, target language support,
etc., such that we're unlikely to draw useful conclusions just from
looking at whether another system supports optional strings.

