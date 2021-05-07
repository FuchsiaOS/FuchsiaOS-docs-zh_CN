# Fonts

Displaying text requires a text rendering library and font assets. On Fuchsia, a
text rendering library needs to built into each runtime, but for fonts, runtimes
have the option of using assets provided by a Fuchsia system service.

Read more about:

*   [Build-time font configuration](build.md)
*   [Font provider service](provider.md)
*   [Common font maintenance tasks](tasks.md)

## Glossary

<a name="gloss-character">character</a>
:   A symbol used in writing, as an abstract concept. For example, this refers
    to the _concept_ of the "upper-case Latin letter A", _not_ its
    [graphical representation](#gloss-glyph) as three connected lines, nor its
    assigned [Unicode code point](#gloss-code-point) of `0x41`, nor any possible
    encodings of it in memory or on disk.

<a name="gloss-code-point">code point</a>
:   A value in the Unicode codespace, in the closed range `[0, 0x10FFFF]`.
    Assigned code points generally correspond to specific characters.

<a name="gloss-glyph">glyph</a>
:   A vector or bitmap image that visually represents a character.

<a name="gloss-font-family">font family</a>
:   A group of typefaces, typically in a variety of styles, sharing common
    design features. Examples include _Noto Sans_, _Noto Sans Mono_, _Arial_,
    _Roboto_, _Times New Roman_, and thousands of others.

<a name="gloss-font-file">font file</a> (or asset)
:   A file containing glyphs, a table mapping code points to glyphs, and other
    metadata required for rendering text.

    Font files can have a variety of formats (encodings). On Fuchsia, the most
    common font file format is TrueType (file extension `.ttf`). Fuchsia also
    uses TrueType Collection files (`.ttc`), which contain multiple typefaces in
    a single file.

<a name="gloss-generic-font-family">generic font family</a>
:   A general category of font families. The ones supported by Fuchsia are those
    listed in the
    [CSS spec](https://www.w3.org/TR/css-fonts-4/#generic-font-families):
    _serif_, _sans-serif_, _monospace_, _cursive_, _fantasy_, _emoji_, _math_,
    and _fangsong_. Please see the CSS spec for detailed descriptions and
    examples.

<a name="gloss-style">style</a>
:   A collection of visual properties of the glyphs of a typeface, such as
    width, slant (upright, italic, or oblique), and weight.

<a name="gloss-typeface">typeface</a>
:   A collection of [glyphs](#gloss-glyph) sharing a common appearance, usually
    corresponding to a single font family, a set of [style](#gloss-style)
    properties, and one or more supported scripts.
