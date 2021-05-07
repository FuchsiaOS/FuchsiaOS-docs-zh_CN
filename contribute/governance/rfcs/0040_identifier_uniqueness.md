{% set rfcid = "RFC-0040" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

Note: Formerly known as [FTP](../deprecated-ftp-process.md)-040.

_"SnowFlake vs SNOW\_FLAKE"_

## Summary

The FIDL specification and front-end compiler currently considers two
identifiers to be distinct based on simple string comparison.
This proposes a new algorithm that takes into account the transformations
that bindings generators make.

## Motivation

Language binding generators transform identifiers to comply with target
language constraints and style that map several FIDL identifiers to a single
target language identifier.
This could cause unexpected conflicts that aren't visible until particular
languages are targeted.

## Design

This proposes introducing a constraint on FIDL identifiers that no existing
libraries violate.
It doesn't change the FIDL language, IR (yet [[1]](#Footnote1)), bindings, style
guide or rubric.

In practice, identifiers consist of a series of words that are joined together.
The common approaches for joining words are `CamelCase`, where a transition
from lower to upper case is a word boundary, and `snake_case`, where one or
many underscores (`_`) are used to separate words.

Identifiers should be transformed to a canonical form for comparison. This will
be a `lower_snake_case` form, preserving the word separation in the original
form. Words are broken (1) where there are underscores, (2) on transitions from
lower-case or digit to upper-case, and (3) before transitions from upper-case to
lower-case.

In FIDL, identifiers must be used in their original form.
So if a type is named `FooBar`, attempting to refer to it as `foo_bar` is an error.

There is a simple algorithm to carry out this transformation, here in Python:
[[2]](#Footnote2)

```python
def canonical(identifier):
    last = '_'
    out = ''
    for i, c in enumerate(identifier):
        is_next_lower = i + 1 < len(identifier) and identifier[i+1].islower()
        if c == '_':
            if last != '_':
                out += '_'
        elif (((last.islower() or last.isdigit()) and c.isupper())
              or (last != '_' and c.isupper() and is_next_lower)):
            out += '_' + c.lower()
        else:
            out += c.lower()
        last = c
    return out
```

Some examples, with their possible translation in various target languages:

FIDL identifier | Canonical form | C++     | Rust    | Go      | Dart
----------------|----------------|---------|---------+---------+---------
foobar          | foobar         | foobar  | foobar  | Foobar  | foobar_
foo_bar         | foo_bar        | foo_bar | foo_bar | FooBar  | fooBar_
foo__bar        | foo_bar        | foo_bar | foo_bar | FooBar  | fooBar_
FooBar          | foo_bar        | foo_bar | foo_bar | FooBar  | fooBar_
fooBar          | foo_bar        | foo_bar | foo_bar | FooBar  | fooBar_
FOOBar          | foo_bar        | foo_bar | foo_bar | FooBar  | fooBar_

## Implementation strategy

The front-end compiler will be updated to check that each new identifier's
canonical form does not conflict with any other identifier's canonical form.

The next version of the FIDL IR should be organized around canonical names
rather than original names, but the original name will be available as a
field on declarations.
If we can eliminate the use of unmodified names in generated bindings then
the original names can be dropped from the IR.

## Ergonomics

This codifies constraints on the FIDL language that exist in practice.

## Documentation and examples

The FIDL language documentation would be updated to describe this constraint.
It would be expanded to include much of what's in the
[Design](#design) section above.

Because this proposal simply encodes existing practice, examples and
tutorials won't be affected.

## Backwards compatibility

Any existing FIDL libraries that would fall afoul of this change violate our
style guides and won't work with many language bindings.
This does not change the form of identifier that is used to calculate ordinals.

## Performance

This imposes a negligible cost to the front-end compiler.

## Security

No impact.

## Testing

There will be extensive tests for the canonicalization algorithm
implementation in `fidlc`.
There will also be `fidlc` tests to ensure that errors are caught when
conflicting identifiers are declared and to make sure that the original names
must be used to refer to declarations.

## Drawbacks, alternatives, and unknowns

One option is to do nothing.
Generally we catch these issues as build failures in non-C++ generated bindings.
As Rust is used more in `fuchsia.git`, the chance of conflicts slipping
through to other petals is lessened.
And these issues are already pretty rare.

The canonicalization algorithm is simple but has one unfortunate failure case
&mdash; mixed alphanumeric words in UPPER_SNAKE_CASE identifiers might be
broken.
For example `H264_ENCODER` → `h264_encoder` but `A2DP_PROFILE` →
`a2_dp_profile`.
This is because the algorithm treats digits as lower-case letters.
We have to break on digit-to-letter transitions because `H264Encoder` should
canonicalize as `h264_encoder`.
Identifiers with no lower-case letters could be special cased &mdash; only
breaking on underscores &mdash; but that adds complexity to the algorithm and
perhaps to the mental model.

The canonical form could be expressed as a list of words rather than a
lower_camel_case string.
They're equivalent and in practice it's simpler to manage them as a string.

We could use identifiers' canonical form when generating ordinals.
That would make this a breaking change for no obvious benefit.
If there is an ordinal-breaking flag day in the future then we could
consider that change then.

### Initial rejection, and second review

_During its first review on 4/18/2019, this FTP was rejected with the following
rationale._

* Two opposing views on solving this class of problems.
* Work to model target languages' constraints to maintain as much
  flexibility in FIDL as possible, even if that is different than the
  recommended style.
  That's the approach taken by this FTP.
  * Pros: Keeps flexibility for eventual uses of FIDL beyond Fuchsia,
    more pure from a programming language standpoint.
  * Cons: Scoping rules are more complex, style is not enforced, but
    encouraged (through linting for instance).
    Could lead to APIs built by partners that do not conform to the Fuchsia
    style guide we want (since they are not required to run, or adhere to
    linting).
* Enforce style constraints directly in the language, which eliminates the
  class of problem.
  * Pros: Style is enforced, developers are told how things ought to be, or
    it doesn't compile.
  * Cons: ingrains stylistic choices in the language definition, higher hill
    to climb for novice developers using FIDL.
* &rarr; We rejected the proposal, and instead prefer an approach that
  directly enforces style in the language.
* &rarr; Next step here is a formal proposal to make this happen, and clarifies
  all aspects of this (e.g., should `uint8` be `Uint8`, `vector<T>` be
  `Vector<T>`?)

_It was decided to overturn this decision based on the following observations:_

* The kernel API is now described FIDL. This pushed us to re-opened the
  identifier uniqueness issue last October, and we essentially overturned the
  decision, allowing both a 'C-style' and 'FIDL-style' to co-exist. This is
  checked by the fidl linter today.

* We have seen other use cases pushing FIDL to generalize the transport, and
  with it possibly also local style rules to better support these domains.

* Identifier clashes continue to be an issue, and modeling target language
  constraints is a well identified gap in the FIDL toolchain.

## Prior art and references

In proto3 similar rules are applied to generate a `lowerCamelCase` name
for JSON encoding.

-------------------------
##### Footnote1
until a new version of the IR schema, which would likely carry names with
additional structure, rather than the fully-qualified name as it exists
today.

##### Footnote2
This algorithm was modified on 2020-06-03, after the FTP was accepted, in order
to more closely match the existing behavior of FIDL backends.
