{% set rfcid = "RFC-0034" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

Note: Formerly known as [FTP](../deprecated-ftp-process.md)-034.

## Rejection rationale

## Summary

We propose:

1. Strings in FIDL will be null terminated (all the while retaining the size of
   the string as is the case today);

1. Change the C bindings to use `uint8_t*` for string data.


## Motivation

The current FIDL string encoding makes it easy to accidentally write insecure
code. The C and low-level C++ bindings are used in some of the most privileged
code in the Fuchsia system so extra weight should be given to risks exposed at
those layers.

We have accidentally come across [such
bugs](https://fuchsia-review.googlesource.com/c/fuchsia/+/251775), but a
systematic audit of code in the Fuchsia tree is difficult and won't prevent them
reoccurring in our tree or in third-party code.

## Design

This proposes to change the encoding of FIDL strings to include a single
terminator byte with value zero. This does not make FIDL strings compatible with
C strings but is a FIDL string is used as a C string it will be interpreted as
shorter than intended rather than longer than intended, which is safer.

### Wire Format

The new
[definition](reference/fidl/language/wire-format/README.md#Strings) of
strings for wire encoding would be (changes highlighted):


*   Variable-length sequence of UTF-8 encoded characters representing text.
*   Nullable; null strings and empty strings are distinct.
*   Can specify a maximum size, e.g. `string:40` for a maximum 40 byte string
    (not including null-terminator).
*   String content has a null-terminator.
*   Encoders and decoders MUST validate that there is a null byte after the last
    byte of the string, as indicated by the length.
*   Stored as a 16 byte record consisting of:
    *   `size`: 64-bit unsigned number of code units (bytes) excluding
        null-terminator
    *   `data`: 64-bit presence indication or pointer to out-of-line string data
*   When encoded for transfer, `data` indicates presence of content:
    *   `0`: string is null
    *   `UINTPTR_MAX`: string is non-null, data is the next out-of-line object
*   When decoded for consumption, `data` is a pointer to content:
    *   `0`: string is null
    *   `<valid pointer>`: string is non-null, `data` is at indicated memory
        address

Strings are denoted as follows:

*   `string`: non-nullable string (validation error occurs if null data is
    encountered)
*   `string?`: nullable string
*   `string:N`, `string:N?`: string with maximum length of **N** code units

This will constitute a breaking wire format change, specifically a string whose
length is divisible by 8 will be 8 bytes longer (it'll align to 8 bytes, and be
null terminated, causing another 7 bytes to be added as padding to align to 8
again).

Encoders will need to be updated to add null termination to encoded strings and
to validate that there are no null characters within string content. Decoders
will need to be updated to check that there are no null characters within string
content but that there is a null terminator where the length indicates.

### C Bindings

Currently the C bindings represent strings as a `char*` and a `size_t`. If that
`char*` is passed to a function that expects a C string it may be interpreted
incorrectly. The bindings will be changed to use `uint8_t*` instead so that
passing a string data pointer to `strchr()` or `printf("%s")` will fail to
compile.

## Implementation Strategy

Note: this section lists paths to source code that have changed since this
document was written. The references were correct at the time of writing.

This is a breaking wire-format change. Its deployment will need to be carefully
coordinated across all uses of FIDL.

Behind some build-time flag(s) the following code will need to be updated:

*   `//zircon/system/ulib/fidl/walker.h` (to validate strings correctly)
*   `//zircon/system/host/fidl/lib/flat_ast.cpp` (update `StringType::Shape`)
*   `//zircon/system/host/fidl/lib/c_generator.cpp` (update
    `EmitLinerarizeMessage`, `ProduceInterfaceClientImplementation`, etc)
*   `//sdk/lib/fidl/cpp/string.cc`
*   `//garnet/public/lib/fidl/rust/fidl/src/encoding.rs`
*   `//third_party/go/src/syscall/zx/fidl/encoding.go` (update `marshalString`,
    `unmarshalString`)
*   `//third_party/go/src/syscall/zx/fidl/encoding_new.go` (update `mString`)
*   `//sdk/dart/fidl/lib/src/types.dart` (update `StringType`)
*   llcpp bindings
*   out-of-tree bindings for other languages

These should be tested with the `fidl_compatibility_test`, running tests and
confirming expected system stability.

Actually landing the change will need to be coordinated with the release team
and external teams like Chromium.

## Ergonomics

This makes the C and low-level C++ bindings much easier to use correctly and has
no impact on other bindings.

## Documentation and Examples

The wire format documentation should be updated (as outlined above).

## Backwards Compatibility

This change is API compatible but ABI incompatible.

## Performance

This will have no impact on build performance but will have the following minor
performance impacts:

*   Each string will take on average one extra byte to transmit

## Security

This change is specifically intended to fix potential security holes for
memory-unsafe languages using FIDL.

## Testing

This will be tested using the compatibility test suite. That suite should be
extended to ensure that edge cases like 7, 8 and 9 byte long strings are handled
correctly.

## Drawbacks, Alternatives, and Unknowns

This will increase the size of FIDL messages by on average one byte per string.
For strings whose length is not divisible by 8 there will be no change. For
strings whose length is divisible by 8 the length will be increased by 8 bytes.

### Alternatives

#### Do Nothing

We could keep everything as it is and rely on code review and documentation to
ensure people don't write wrong code. The issue will also be somewhat mitigated
by the adoption of the new low-level-C++ bindings. The potential dangers of the
class of bugs caused by this subtle issue too serious to just leave things as
they are.

#### Null terminate but ban embedded null bytes / use [modified UTF-8](https://en.wikipedia.org/wiki/UTF-8#Modified_UTF-8)

(This was the original proposal)

`'\0'` is a valid UTF-8 character and exists in UTF-8 strings that FIDL users
want to exchange. Banning null bytes would make FIDL inappropriate for many uses
and using modified UTF-8 would impose additional overhead to convert normal
UTF-8 with possibly null bytes to modified UTF-8.


#### Just use `uint8_t` instead of `char`

If FIDL string data pointers were `uint8_t` instead of `char` then they couldn't
be passed to standard string functions or `printf` without casting. This would
help reveal this class of bug but wouldn't prevent them.


#### Fill padding with valid ASCII in debug builds

Seven out of eight strings will be followed by zero padding bytes. In debug
builds we could change those zeros to valid ASCII characters so that when
printed or parsed these bugs show up. It'll be very easy for these to fall
through the cracks.


#### Always move strings out of line

The C/C++ bindings could allocate space for strings and null terminators and
copy and terminate all strings that are received. This would impose an
unacceptable performance cost on the C and low-level C++ bindings.


#### Move strings out of line for ASan builds

The address sanitizer will check that code doesn't overrun heap allocations. For
ASan builds we could allocate space for strings on the heap, copy the bytes
there and return that pointer to the caller. This is non-trivial to integrate
into the C bindings, could mean bug-hiding significant differences in behavior
between ASan and release builds and wouldn't help developers working outside the
Fuchsia build system.


#### Stop using C and C++

If Google can't write safe C++ in 2019, C++ is not safe for use. Unfortunately C
and C++ are the languages of choice for many device driver authors and many
vendors may choose to port their existing C/C++ drivers to our platform.


#### Don't expose raw characters to C

We could expose an opaque structure to C and require any string access in C to
copy strings out of the decoded message buffer.


## Prior Art and References

[DBus](https://dbus.freedesktop.org/doc/dbus-specification.html#idm636), [Capt'n
Proto](https://capnproto.org/encoding.html#blobs) and CORBA[^1] send length and
null terminate, protobuf does not null terminate.

<!-- footnotes -->

[^1]: [CORBA Specification 3.3,
    part 2](https://www.omg.org/spec/CORBA/3.3/Interoperability/PDF),
    section 9.3.2.7.
