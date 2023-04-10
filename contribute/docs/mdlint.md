# Markdown linter - `mdlint`

## Running mdlint

```sh
fx mdlint -root-dir $FUCHSIA_DIR/docs
```

For more options, see the [full command line reference](https://fuchsia.dev/reference/tools/fx/cmd/mdlint.md).

If it is not part of the current build configuration for some reason, re-run `fx set`
including the option to build mdlint: `--with //tools/mdlint`.


Markdown linter checks markdown:

## bad-headers

* One H1 header per file.
* Correct nesting of subheading levels.
* Duplicate headers.

## bad-lists

* newline before and after lists.

## casing-of-anchors

* Anchors must be lowercase.

## newline-before-fenced-code-block

* newline before and after fenced code blocks.

## no-extra-space-at-start-of-doc

* No blank lines at the beginning of a document.

## no-extra-space-on-right

* No trailing spaces.

## respect-col-length

* 80 character lines.

## respectful-code

Encourge respectful language in the documentation. See
 [Modifying non-inclusive words in mdlint](/docs/contribute/docs/modify-mdlint-inclusive-language.md).

## simple-utf8-chars

Avoids utf-8 characters that can be represented in ASCII.

* `“`
* `”`
* `'`
* `’`
* `…`
