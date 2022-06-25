<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0149" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Summary

No longer require validation during FIDL encode. However, validation still
will take place during decode and padding must be zeroed during encode.

## Motivation

The motivation for this RFC is to loosen design constraints that currently
dictate that bindings must validate during encode. This does not mean that
bindings necessarily should change their behavior, just that they are less
constrained. In fact, in most cases they are unlikely to change as a result
of this RFC.

That said, there are several reasons for no longer mandating validation
during encode:

- **Performance** during encode, validation incurs extra overhead. As an
example, there is a fast path for struct encoding that reduces to a `memcpy`
that in many bindings cannot be taken if there is an enum field within the
struct that needs to be validated.
Performance impacts are particularly significant for the HLCPP bindings,
which walk the object being encoded twice - once to encode and once to
validate (though this might be avoided through redesign of the bindings).

- **Code size** the validation logic increases code size. This code size
increase is most significant for bindings that generate code or use macros
because the validation logic is duplicated multiple times in the output.

## Stakeholders

_Facilitator:_ pascallouis

_Reviewers:_ pascallouis, yifeit, mkember

_Consulted:_

azaslavsky

_Socialization:_

This RFC was distributed on the FEC discuss mailing list.

## Design

* Bindings MAY OR MAY NOT validate FIDL objects during encode.
To be more precise, it is no longer a requirement that objects are encoded
in a way that guarantees that they will always pass validation during decode.

* Bindings MUST ensure padding bytes are zeroed after encoding.
However, the encoder itself need not zero the padding - for instance it could
be zeroed through guarantees of the programming language.

* Bindings MUST validate FIDL objects during decode.

## Implementation

This RFC has no immediate deliverables - the bindings currently validate FIDL
objects and may continue to do so. However, in the future bindings will be able
to loosen validation checks.

## Performance

Performance effects are highly use case and binding specific. Here are some
examples of some of the effects of validation on performance:

- In Rust, encode of a struct with 15 uint8s and 1 boolean is 3.2x slower
(52ns) with boolean validation than without validation. If the structure has
254 uint8s and 1 boolean it is 21x slower (844ns). In LLCPP, however, there
is a negligible performance cost for the same structure. This test case was
chosen because in many bindings a single boolean or enum in a struct, array or
vector body will prevent `memcpy` optimizations.

- Encode of an array of 256 enums is 2.2x slower (2.6us) in LLCPP, 1.2x (192ns)
slower in Rust and 9x (1.1us) slower in Go when there is enum validation.

- Encode of HLCPP objects is 1-5x slower with validation. A 16 element table
is 1.7x slower (400ns) to encode while a message header is 1.3x slower (37ns).

These measurements are from a machine with a Intel Core i5-7300U CPU @ 2.60GHz.
Note that they are microbenchmarks and performance in practice may vary.

CLs used for these benchmarks:

- [Rust](https://fuchsia-review.googlesource.com/c/fuchsia/+/608427)
- [LLCPP](https://fuchsia-review.googlesource.com/c/fuchsia/+/608428)
- [HLCPP](https://fuchsia-review.googlesource.com/c/fuchsia/+/608902)

## Ergonomics

There should be no effect on binding ergonomics.
However, bindings that disable validation during encode would no longer
"fail fast" for certain failure modes which could impact the user.

## Backwards Compatibility

This should have no impact on backwards compatibility.

## Security considerations

Padding bytes have historically been zeroed rather than validated and will
continue to be zeroed. This is important because FIDL objects can be allocated
on top of old allocations, leading to leakages if the memory is copied onto the
wire. The risk of leakages is less significant for most other types of
validation.

There are two main classes of validations being performed.

### Value-restrictions

- **Bool, Enum, Bits** - validation ensures that the integers backing these
types are in the expected ranges.

- **Float** - float validation is not currently part of the specification,
but bindings may still perform some validation, in particular to prevent
NaN values.

- **UTF-8** - FIDL strings are vectors with UTF-8 data in the payload.
Validation ensures they are UTF-8.

Fields of these types are generally assigned by the user through the binding
API. Validation ensures that the user provided valid input through the APIs
and that no other form of unexpected bug changed the values, such as
memory corruption. Note that memory corruption bugs can happen at any stage
of processing the object and it is somewhat arbitrary to expect it to be
caught by, for instance, bool validation.

- **Size limits** - There are limits to the size of certain objects such as
tables.

There exist certain size limits that always must be followed for transports
such as the `64k` message size limit on the channel transport which prevent
infinite blow up of message size. Because of this, it generally isn't an
immediate concern that needs to be addressed before reaching the decoder.

### Invalid state

- **Non-optional type treated as optional** - essentially a non-optional type
with a missing payload.

- **Envelope inline bit** - envelopes have fields indicating whether data
should be stored inline. It is possible to have an envelope with a size
<= 4 bytes that should be stored inline but is missing an inline bit marker.

- **Vector absent but count non-zero** - this should never happen during
encode.

None of these have significant negative consequences if they aren't caught
before hitting the decoder. They also tend to be internal issues in the FIDL
implementation that can be guaranteed to be correct through other means.

### Conclusion

While there are security concerns that need to be considered, they generally
can be caught on the decoding side rather than the encoding side because they
have low risk of information leakage.

## Privacy considerations

There is no impact on privacy.

## Testing

Testing requirements will generally reduce as a result of this RFC.

## Documentation

The FIDL binding spec will need to be updated to address this.

## Drawbacks, alternatives, and unknowns

There is a natural tradeoff between more restrictions and more flexibility.
In the case of validation, more restrictions would bring proported
security benefits while flexibility would make performance and code size
improvements possible. This RFC argues that the security benefits aren't
so significant and FIDL should favor removing the requirement.

After the requirement is removed, bindings will have a choice between
keeping or removing existing encode-side validation.
In practice, it is expected that bindings will eventually remove this
validation as much as possible, but some bindings may continue to
validate in debug mode in order to catch issues earlier.

## Prior art and references

None
