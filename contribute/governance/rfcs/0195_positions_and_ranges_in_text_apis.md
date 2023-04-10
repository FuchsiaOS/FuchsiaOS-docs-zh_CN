<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0195" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
{# Fuchsia RFCs use templates to display various fields from _rfcs.yaml. View the #}
{# fully rendered RFCs at https://fuchsia.dev/fuchsia-src/contribute/governance/rfcs #}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Summary

We propose that the `fuchsia.input.text` APIs shall use
[Unicode scalar values][unicode-scalar-value]{:.external} as atomic units for
text editing positions and ranges (e.g. carets and selections).

[unicode-scalar-value]: https://unicode.org/glossary/#unicode_scalar_value

## Motivation

The `fuchsia.input.text` namespace will provide FIDL protocols for text editing
and composition, enabling cross-runtime implementations of text fields, Input
Method Editors (IMEs), copy and paste, autocorrect, and related functionality.
These APIs will include a number of methods for retrieving, selecting, and
modifying text ranges. As a fundamental part of the design of these methods, the
API must standardize on a way to index into a Unicode string. This is needed to
ensure that, for example, when an on-screen keyboard implemented in Flutter
instructs a text box in a Chromium web view to "delete the three characters
before the caret," the keyboard and the browser are in agreement on what "three
characters" means and where the caret currently is.

Fuchsia's in-tree runtimes currently do not have a consensus on what the basic
unit of string manipulation should be (see
[Prior art and references](#prior_art_and_references)). The standard SDKs of
several other, non-Fuchsia platforms that we reviewed are also inconsistent on
this question, and furthermore are influenced by legacy design choices, in many
cases predating modern Unicode standards.

Because internationalized text editing is a critical feature for modern
user-facing operating systems, and because there is not an existing unified
standard that Fuchsia could adopt, Fuchsia has the opportunity to improve upon
the status quo by choosing its own single standard for its cross-runtime API
that makes sense ergonomically and is not hindered by legacy designs.

Furthermore, because Fuchsia's text editing API would serve as an interop
mechanism between multiple independent runtimes that are not necessarily aware
of each other, it must offer a well-defined interface that is practical to
implement consistently, without standardizing the implementation details of any
one runtime.

## Stakeholders

*Facilitator:* abarth@google.com

*Reviewers:*

-   *Fuchsia HCI:* neelsa@google.com, fmil@google.com

-   *Security:* pesk@google.com

-   *Privacy:* enoharemaien@google.com

-   *Chromium:* wez@google.com

-   *Flutter:* jmccandless@google.com, gspencer@google.com

*Consulted:* quiche@google.com

### Socialization

This design was socialized as a Google Doc among the Fuchsia HCI team and some
of the reviewers prior to being posted as an RFC CL.

## Design

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD",
"SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be
interpreted as described in [IETF RFC 2119][ietf-rfc-2119]{:.external}.

[ietf-rfc-2119]: https://tools.ietf.org/html/rfc2119

### Background

See the [FIDL API Readability Rubric > String encoding][fidl-string-encoding]
for a more detailed overview.

A [FIDL string][fidl-string] is a sequence of bytes representing UTF‑8-encoded
Unicode text.

There are several choices of units into which a string can be divided.

*   *Unicode scalar value*: In Unicode, the basic atom of text is a "Unicode
    scalar value", which is an integer in the ranges `[0x0, 0xD7FF], [0xE000,
    0x10FFFF]`, which can be mapped to an "abstract character".

    Unicode scalar values are a subset of
    [Unicode code points][unicode-code-point]{:.external}, which are integers in
    the range `[0x0, 0x10FFFF]`. The code points that are excluded from Unicode
    scalar values, `[0xD800, 0xDFFF]`, are known as the
    [surrogate code points][unicode-surrogate-code-point]{:.external}. They are
    reserved for implementation details of the UTF-16 encoding, and cannot be
    used to represent any assigned characters.

    Note: In less technical writing, *code point* is often used as a synonym for
    *Unicode scalar value* because the excluded surrogate code points are in any
    case illegal in encodings outside of UTF-16.

*   *Byte*: The output of dividing a string into bytes depends on the encoding.
    For example, the UTF‑8 encoding used by FIDL strings is a variable-length
    encoding, with each scalar value represented by a sequence of 1 to 4 bytes.
    (For example, `k` is one byte, `ك` is two bytes, `ከ` is three bytes, and `𐤊`
    is four bytes.) The UTF‑8 standard specifies how to parse a sequence of
    bytes and to determine where a new scalar value begins. Because UTF‑8 is a
    variable-length encoding, it is not possible to determine the number of
    scalar values in a UTF‑8 string, nor to jump to the *n*-th scalar value, in
    constant time.

*   *Grapheme cluster*: Some combinations of Unicode scalar values, when
    rendered graphically, are combined into a single user-perceived "character"
    that is technically called a "grapheme cluster". Examples include letters
    with diacritics (`á̡`), face emoji with gender and skin tone selectors
    (`💂🏽‍♀️`), and two-letter country codes combined into flag emoji (`🇦🇺`). The
    rules for merging scalar values into grapheme clusters are context-specific
    and are dependent on properties read from the Unicode Character Database; as
    a result, they are subject to change from one release to the next of the
    Unicode standard.

Although not directly relevant to FIDL and its UTF‑8 strings, many legacy
runtimes that use UTF‑16 encoding feature an additional division option:

*   *UTF‑16 code unit*: In the UTF‑16 encoding, every Unicode scalar value is
    encoded by one or two 2-byte sequences called UTF‑16 "code units". The
    UTF‑16 standard specifies how to determine, from the bits of a code unit,
    whether it is a single-code-unit scalar value, or part of a two-code-unit
    [*surrogate pair*][unicode-surrogate-pair]{:.external}.

[fidl-string-encoding]: /docs/development/api/fidl.md#string_encoding_string_contents_and_length_bounds
[fidl-string]: /docs/reference/fidl/language/language.md#strings
[unicode-surrogate-code-point]: https://unicode.org/glossary/#surrogate_code_point
[unicode-code-point]: https://unicode.org/glossary/#code_point
[unicode-surrogate-pair]: https://unicode.org/glossary/#surrogate_pair

### Design

In `fuchsia.input.text`, in any method parameter or return value that represents
one or more indices into a string, the base unit shall be a single Unicode
scalar value.

For example, in the following hypothetical method, `Range` is defined in terms
of positions of Unicode scalar values from the beginning of a string or text
field.

```fidl
protocol ReadableTextField {
    /// Retrieves part of the contents of the text field.
    GetText(struct {
        range Range;
    }) -> (struct {
        // Note that FIDL string field sizes are specified in bytes
        // https://fuchsia.dev/fuchsia-src/reference/fidl/language/language#strings
        contents string:MAX_STRING_SIZE;
    }) error TextFieldError;
};

type Range = struct {
    /// The index of the first scalar value in the range.
    start uint32;
    /// The index _after_ the last scalar value in the range.
    end uint32;
};
```

If a text field contains the string `abcd😀ef🇦🇺gh`, requesting the range `[2, 8)`
would return the substring `cd😀ef🇦`. (Note that the grapheme cluster `🇦🇺` would
be split into `🇦`, `🇺`.)

## Implementation

Internally, implementers may use whichever Unicode string encoding and indexing
is best supported or convenient in their chosen programming language or
libraries.

However, all implementations of the protocols in `fuchsia.input.text`

-   MUST correctly interpret text positions and ranges using the Unicode scalar
    value indices specified in the protocols.
-   MUST send their text editing commands to other `fuchsia.input.text`
    implementations in terms of Unicode scalar value indices.

For reference:

-   In Rust, a Unicode scalar value is a single `char`, and the scalar values in
    a `String` or `&str` can be iterated using
    [`String::chars()`][rust-chars]{:.external}.

-   In Dart, this is a `rune`. A string's Unicode scalar values can be iterated
    using the [`String.runes`][dart-runes]{:.external} property.

-   As of C++ 17, the standard library's utilities for manipulating Unicode text
    are incomplete, so the use of `icu::UnicodeString` with
    [`icu::StringCharacterIterator`][icu-stringcharacteriterator]{:.external} is
    recommended instead. For example, the *n*-th scalar value in a string can be
    retrieved using [`setIndex32(n)`][icu-setindex32]{:.external}.

[rust-chars]: https://doc.rust-lang.org/stable/std/string/struct.String.html#method.chars
[dart-runes]: https://api.dart.dev/stable/2.15.1/dart-core/String/runes.html
[icu-stringcharacteriterator]: https://unicode-org.github.io/icu-docs/apidoc/released/icu4c/classStringCharacterIterator.html
[icu-setindex32]: https://unicode-org.github.io/icu-docs/apidoc/released/icu4c/classicu_1_1UCharCharacterIterator.html#a3a42877b3fb105eae0c85db7be9cdf54

## Performance

For runtimes that use variable-length encodings such as UTF‑8 (e.g. Rust) or
UTF‑16 (e.g. Dart) for their strings, accessing a string position or length by
Unicode scalar value is a linear-time operation. (It would only be a
constant-time operation for UTF‑32 and similar fixed-length encodings, which are
space-inefficient and are not commonly used.)

For use cases that frequently access string lengths and anticipate the presence
of long strings, it may be prudent to cache length values or otherwise
preprocess strings in order to achieve *amortized* constant time.

## Ergonomics

Unicode scalar values have the following advantages for text editing and
composition:

-   This granularity prevents the possibility of splitting UTF‑8-encoded
    characters into invalid byte sequences.
-   It allows, if necessary, editing *inside* a grapheme cluster. For example,
    after inputting "á" ("a" followed by "◌́"), it allows backspacing to delete
    the accent but not the base letter.

See
[Drawbacks, alternatives, and unknowns](#drawbacks_alternatives_and_unknowns)
for comparisons to the other options.

## Backwards Compatibility

This RFC concerns new text editing APIs, which are being implemented from
scratch as part of the Fuchsia platform. We do not anticipate backward
compatibility issues, apart from the inherent tasks of converting between the
FIDL API's text position representation and the representation preferred in any
given language runtime.

## Security considerations

Manipulating text by entire Unicode scalar values rather than by bytes or by
UTF‑16 code units makes it less likely that a string will be invalidly
truncated.

Atomizing by Unicode scalar value leads to the possibility of splitting up
grapheme clusters, which is sometimes desirable (see [Ergonomics](#ergonomics)),
but if done haphazardly, might lead to edge cases where the meaning of some text
changes.

However, this drawback must be accepted because grapheme clustering is dependent
on the Unicode version and even subject to implementation-specific tailoring,
and therefore clients that use different Unicode libraries or versions might
disagree on string lengths, leading to data corruption.

## Privacy considerations

There are no new privacy considerations from this RFC in addition to whichever
privacy considerations already exist around handling user-provided text.

## Testing

Implementers of the `fuchsia.input.text` API will be responsible for writing
appropriate unit and integration tests for their implementations. This RFC's
requirements should be covered as part of those tests.

Depending on the functionality available to
[Compatibility Tests for Fuchsia][ctf], the behavior of clients implementing the
text editing APIs may be tested for broader compliance with those APIs. For
example, the tests might send a series of text editing commands to a client
application that hosts a text field, and then verify that the resulting text
field contents are as expected.

Implementers are advised to include non-ASCII strings in their test data,
including:

<!-- mdformat off(mdformat incorrectly wraps "{:.external}") -->

-   scalar values
    [outside of the Basic Multilingual Plane][wikipedia-unicode-plane]{:.external}

<!-- mdformat on -->

-   multi-code-point grapheme clusters, such as
    -   characters with multiple combining diacritics
    -   emoji with skin tone and/or gender modifiers
    -   flag emoji

[ctf]: /docs/development/testing/ctf/compatibility_testing.md
[wikipedia-unicode-plane]: https://en.wikipedia.org/wiki/Plane_\(Unicode\)

## Documentation

The API documentation for `fuchsia.input.text` classes will explicitly highlight
the units used for any data types involving string positions, ranges, and
lengths.

## Drawbacks, alternatives, and unknowns

### Bytes

#### Pros

-   In FIDL field declarations, `string` lengths are
    [explicitly defined in bytes][fidl-string].
-   Using bytes makes it easier to reason about size in memory.
-   Array access into byte arrays is O(1).

#### Cons

-   Additional validation is required to ensure that byte sequences constitute
    valid UTF‑8.

-   It's easy to inadvertently split a UTF‑8 character into incomplete (and
    hence invalid) byte sequences.

-   Shifting a position by one byte is not a useful operation unless it is known
    that the text being edited solely contains ASCII characters.

### Grapheme clusters

#### Pros

-   In text editors, carets are almost always placed on grapheme cluster
    boundaries.

-   Selecting text by entire grapheme cluster ensures that complex emoji are not
    accidentally split in a user-unfriendly manner (for example, with code
    points, `👮🏽‍♀️` (emoji of a female police officer with a medium skin tone)
    could be split into `POLICE OFFICER (U+1F46E)`, `EMOJI MODIFIER FITZPATRICK
    TYPE-4 (U+1F3FD)`, `ZERO WIDTH JOINER (U+200D)`, `FEMALE SIGN (U+2640)`,
    `VARIATION SELECTOR-16 (U+FE0F)`).

#### Cons

-   Grapheme clustering rules can change between versions of the Unicode
    standard and are dependent on character property table lookups from the
    [CLDR][unicode-cldr]{:.external}.

    More critically, the sets of clustering rules are *not fully specified by
    Unicode versions*; the details are allowed to vary between implementations
    and between locales [^1]. Two components communicating over FIDL (e.g. an
    on-screen keyboard and a runtime that is rendering a text box) might be
    using different Unicode implementations, and might therefore make
    conflicting assumptions about which range of text they are manipulating.

    The Unicode specification for grapheme clustering, *UAX #29: Unicode Text
    Segmentation*,
    [explicitly states][unicode-grapheme-cluster-boundaries]{:.external}:

    > This document defines a default specification for grapheme clusters. It
    > may be customized for particular languages, operations, or other
    > situations. For example, arrow key movement could be tailored by language,
    > or could use knowledge specific to particular fonts to move in a more
    > granular manner, in circumstances where it would be useful to edit
    > individual components. This could apply, for example, to the complex
    > editorial requirements for the Northern Thai script Tai Tham (Lanna).
    > Similarly, editing a grapheme cluster element by element may be preferable
    > in some circumstances. For example, on a given system the backspace key
    > might delete by code point, while the delete key may delete an entire
    > cluster.

[unicode-cldr]: https://cldr.unicode.org
[wikipedia-digraph]: https://en.wikipedia.org/wiki/Digraph_\(orthography\)
[unicode-grapheme-cluster-boundaries]: http://www.unicode.org/reports/tr29/#Grapheme_Cluster_Boundaries
[^1]: How many grapheme clusters are in the string `"ch"`? In the `en-US`
    locale, it's two. In `cs-CZ` (Czech), it should just be one, as `'ch'` is
    a [digraph][wikipedia-digraph]{:.external}.

### UTF‑16 code units

#### Pros

-   Many third-party standard libraries and runtimes use UTF‑16 encoding
    internally for their strings.

#### Cons

-   FIDL transports strings in UTF‑8, not in UTF‑16. Introducing a new
    encoding's units into text editing APIs over FIDL would be completely
    unfounded and would cause confusion by forcing implementers to internally
    support at least two different encodings.
-   As with individual bytes, it is easy to inadvertently split scalar values
    into unmatched UTF‑16 surrogates.

## Prior art and references

### Flutter

Flutter appears to have largely [migrated][flutter-characters-pr]{:.external} to
using grapheme clusters in its public APIs, though its documentation remains
inconsistent:

*   Dart's [`String` class documentation][dart-string]{:.external} states that a
    "string is represented by a sequence of Unicode UTF‑16 code units" and "The
    characters of a string are encoded in UTF‑16. Decoding UTF‑16, which
    combines surrogate pairs, yields Unicode code points," implying that
    *character* means *code unit*.

*   Flutter does not explicitly document its
    [`TextPosition`][flutter-textposition]{:.external} or
    [`TextRange`][flutter-textrange]{:.external} units, defining *offset* as
    "index of the character that immediately follows the position in the string
    representation of the text," but not defining *character* here.

<!-- mdformat off(mdformat incorrectly wraps after "{:.external}") -->

*   Flutter's [`TextField.maxLength`][flutter-textfield-maxlength]{:.external}
    property is defined as

    > The maximum number of characters (Unicode grapheme clusters) to allow in
    > the text field.

    This is elaborated upon further down:

    > **Characters** \
    > For a specific definition of what is considered a character, see the
    > [characters][dart-characters]{:.external} package on Pub, which is what
    > Flutter uses to delineate characters. In general, even complex characters
    > like surrogate pairs and extended grapheme clusters are correctly
    > interpreted by Flutter as each being a single user-perceived character.

<!-- mdformat on -->

[flutter-characters-pr]: https://github.com/flutter/flutter/pull/59778/files
[dart-string]: https://api.dart.dev/stable/2.18.0/dart-core/String-class.html
[flutter-textposition]: https://api.flutter.dev/flutter/dart-ui/TextPosition/offset.html
[flutter-textrange]: https://api.flutter.dev/flutter/dart-ui/TextRange/start.html
[flutter-textfield-maxlength]: https://api.flutter.dev/flutter/material/TextField/maxLength.html
[dart-characters]: https://pub.dev/packages/characters

### Web

JavaScript *characters* are UTF‑16 code units. The
[`Range`][mdn-range]{:.external}, [`Selection`][mdn-selection]{:.external}, and
[`CaretPosition`][mdn-caretposition]{:.external} classes all deal with character
offsets.

(However, for integrations with the the Chromium runtime, it is worth noting
that internally, [Chromium uses UTF-8-encoded strings][chromium-string-usage].)

[mdn-range]: https://developer.mozilla.org/en-US/docs/Web/API/Range
[mdn-selection]: https://developer.mozilla.org/en-US/docs/Web/API/Selection
[mdn-caretposition]: https://developer.mozilla.org/en-US/docs/Web/API/CaretPosition
[chromium-string-usage]: https://www.chromium.org/developers/chromium-string-usage/

### Android

Android's IME APIs explicitly use Java `char`s, which are UTF‑16 code units.
See, for example,
[`android.view.inputmethod.BaseInputConnection.commitText`][android-baseinputconnection-committext]{:.external}.

[android-baseinputconnection-committext]: https://developer.android.com/reference/android/view/inputmethod/BaseInputConnection#commitText\(java.lang.CharSequence,%20int\)

### MacOS and iOS

In Objective C, the `NSString` documentation [says][apple-nsstring]{:.external}:

> An `NSString` object encodes a Unicode-compliant text string, represented as a
> sequence of UTF‑16 code units. All lengths, character indexes, and ranges are
> expressed in terms of 16-bit platform-endian values, with index values
> starting at `0`.

However, the Swift class `String` by default uses the *grapheme cluster* as a
unit, with additional properties to expose Unicode code points, UTF‑16 code
units, and bytes.

Classes relating to text editing use different units depending on whether they
originate in Objective C or in Swift. The
[`UITextInput`][apple-uitextinput]{:.external} protocol uses opaque, abstract
classes [`UITextRange`][apple-uitextrange]{:.external} and
[`UITextPosition`][apple-uitextposition]{:.external}, which are
implementation-specific.

[apple-nsstring]: https://developer.apple.com/documentation/foundation/nsstring#1666323
[apple-uitextinput]: https://developer.apple.com/documentation/uikit/uitextinput
[apple-uitextrange]: https://developer.apple.com/documentation/uikit/uitextrange
[apple-uitextposition]: https://developer.apple.com/documentation/uikit/uitextposition

### Windows

Windows Core Text's documentation calls their indices
["Application Caret Positions"][ms-acp]{:.external}, described as

> a zero-based number that indicates the count of characters from the start of
> the text stream immediately before the caret

The "count of characters" implies UTF‑16 code units because that's what .NET's
`System.Char` type represents.

[ms-acp]: https://docs.microsoft.com/en-us/windows/apps/design/input/custom-text-input#application-caret-position
