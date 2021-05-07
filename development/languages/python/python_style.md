# Python style guide

The Fuchsia project follows the [Google Python style guide](https://github.com/google/styleguide/blob/gh-pages/pyguide.md){:.external},
with a few [refinements](#refinements).

The Google Python style guide allows more variation (presumably to cover a large
breadth of existing source). This guide has a tighter set of choices. So a
Fuchsia Python file will also comply with the Google style guide, but a Google
Python file might not comply with this guide. See [refinements](#refinements)
below for details.

## Python versions {#python-versions}

### Scripts invoked by the build

Scripts invoked by the build (GN or Ninja) are executed with Python 3.8.

The build system ensures that all python scripts are executed by the
interpreter that is intalled as part of a Fuchsia source checkout.

### Other scripts

Scripts that are invoked directly should use `python` in the shebang and be
compatible with both 2 and 3: `#!/usr/bin/env python`.

Developers working on Fuchsia modules may use various platforms. Some platforms
include Python 2 and not Python 3 and vice versa. Until Python 3 is
included in the prominent development environments we support, we should support
Python 2.

While Python 2 is supported, test scripts on both versions.

Any policy change will be reflected in this document.

## Multiple Inheritance

Multiple inheritance is strongly discouraged. This is for the same reason
listed in the
[Google C++ style guide: risk of "diamond" inheritance](https://google.github.io/styleguide/cppguide.html#Inheritance){:.external}
patterns, which are prone to confusion. If a case is found where avoiding
multiple inheritance is unreasonable, all classes involved must initially
inherit from the base class `object`, which governs which multiple inheritance
scheme is used.

## Use Unicode for Text

In scripts that support Python 2.x (see [Python versions](#python-versions)),
explicitly declare text strings as unicode and binary data as bytes, using
`u""`, `unicode()`, `unichr()` and  `b""`, `bytes()`, `byte()` respectively.
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

## Refinements

The following refinements we make to the Google Python style guide are largely
choices between variations. For example, if the style guide says you may do A,
B, or C we may choose to favor B and avoid the other choices.

### Indentation

Avoid aligning with opening delimiter. Prefer instead to indent using fixed
(4 space) indentation.

(See
[Indentation](https://github.com/google/styleguide/blob/gh-pages/pyguide.md#34-indentation){:.external}
in the Google Python style guide for comparison.)

### Statements

Avoid creating single line statements, even with `if` statements.

```python {.good}
Yes:

    if foo:
        bar(foo)
```

```python {.bad}
No:

    if foo: bar(foo)
```

(See
[Statements](https://github.com/google/styleguide/blob/gh-pages/pyguide.md#314-statements){:.external}
in the Google Python style guide for comparison.)

### Type annotations

In scripts that support Python 2 (see [Python versions](#python-versions)),
type annotations will not be used.

(See
[Type Annotations](https://github.com/google/styleguide/blob/gh-pages/pyguide.md#319-type-annotations){:.external}
in the Google Python style guide for comparison.)

### Strings

Prefer double quotes for strings (`"`). Use single quotes when the declaration is
more readable with single quotes. For example, `'The cat said "Meow"'` is more readable
than `"The cat said \\"Meow\\""`.

(See
[Strings](https://github.com/google/styleguide/blob/gh-pages/pyguide.md#310-strings){:.external}
in the Google Python style guide for comparison.)

### Be consistent

Be consistent within a large scope. Avoid displaying small pockets of consistency
within Fuchsia. Being consistent within only a single file or directory is not
consistency.

Within `third_party`, the intent is to follow the existing style for that project
or library. Look for a style guide within that library as appropriate.

(See
[Parting Words](https://github.com/google/styleguide/blob/gh-pages/pyguide.md#4-parting-words){:.external}
in the Google Python style guide.)
