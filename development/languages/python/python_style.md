Python style guide
==============================

The Fuchsia project follows the [Google Python style guide](
https://github.com/google/styleguide/blob/gh-pages/pyguide.md), with a few
[refinements](#Refinements).

The Google Python style guide allows more variation (presumably to cover a large
breadth of existing source). This guide has a tighter set of choices. So a
Fuchsia Python file will also comply with the Google style guide, but a Google
Python file might not comply with this guide. See [refinements](#Refinements)
below for details.

## Python 2 vs 3

Lean towards a Python 3 style where the languages differ, but continue to
support both versions.

Developers working on Fuchsia modules may use various platforms. Some platforms
include Python 2.x and not Python 3.x and vice versa. Until Python 3.x is
included in the prominent development environments we support, we should support
Python 2.x.

While Python 2.x is supported, test scripts on both versions. Python 2.7 will be
supported by the Python team until January 1, 2020. When we drop Python 2.7
support will be influenced by, but not dictated by that support pledge from the
Python team.

## Multiple Inheritance

Multiple inheritance is strongly discouraged. This is for the same reason
listed in the Google C++ style guide: risk of "diamond" inheritance patterns,
which are prone to confusion. If a case is found where avoiding multiple
inheritance is unreasonable, all classes involved must initially inherit from
the base class `object`, which governs which multiple inheritance scheme is
used.

## Use Unicode for Text

While Python 2.x is supported, explicitly declare text strings as unicode and
binary data as bytes, using `u""`, `unicode()`, `unichr()` and  `b""`,
`bytes()`, `byte()` respectively.
Python 3.x defaults to using Unicode for strings, so this guideline will be
removed when support for Python 2 is dropped.

```python {.good}
Yes:

  a = u"Hello"  # Unicode constant.
  b = unicode(foo)  # Convert to Unicode.
  c = unichr(c)  # Convert to Unicode.
  d = io.open("bar.txt").read()  # Read text as Unicode.
```

```python {.bad}
No:

  a = "Hello"  # Ambiguous (depends on Python version).
  b = str(foo)  # Convert to ascii.
  c = chr(c)  # Convert to ascii.
  d = open("bar.txt").read()  # Read text as ascii.
```

# Refinements

The following refinements we make to the Google Python style guide are largely
choices between variations. For example, if the style guide says you may do A,
B, or C we may choose to favor B and avoid the other choices.

## [Indentation](https://github.com/google/styleguide/blob/gh-pages/pyguide.md#34-indentation)

Avoid aligning with opening delimiter. Prefer instead to indent using fixed
(4 space) indentation.

## [Statements](https://github.com/google/styleguide/blob/gh-pages/pyguide.md#314-statements)

Avoid creating single line statements, even with `if` statements.

## [Type Annotations](https://github.com/google/styleguide/blob/gh-pages/pyguide.md#319-type-annotations)

While Python 2.x is supported, type annotations will not be used.

## [Strings](https://github.com/google/styleguide/blob/gh-pages/pyguide.md#310-strings)

Prefer double quotes for strings ("). Use single quotes when the declaration is
more readable with single quotes. E.g. 'The cat said "Meow"' is more readable
than "The cat said \\"Meow\\"".

## [Be consistent](https://github.com/google/styleguide/blob/gh-pages/pyguide.md#4-parting-words)

Be consistent within a large scope. Avoid making small pockets of consistency
within Fuchsia. Being consistent within one file or directory is not much
consistency.

Within third_party, the intent is to follow the existing style for that project
or library.
Look for a style guide within that library as appropriate.