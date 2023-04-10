{% set rfcid = "RFC-0043" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

Note: Formerly known as [FTP](../deprecated-ftp-process.md)-043.

## Summary

This RFC standardizes on a single format for writing documentation
comments.
This format can be consumed by tools that generate API documentation in
other human readable and machine consumable formats (e.g., HTML, Markdown).

## Motivation

We currently have an [API documentation rubric] that is very clear about
what aspects of FIDL code need to be documented, including parameters,
return values, and errors.
We also encourage developers to provide examples of API usage.
However, we have not given developers a clear way of expressing those
features in FIDL API documentation.

With the advent of the `fidldoc` tool, it becomes more important to provide
developers a way to express formatting in their comments.
Developers writing comments should know how to, for example, format a list
in their output.
They must also know how to indicate that something is a return value or a
parameter, so that it can be surfaced correctly in the output.

## Design

**TL;DR:** We want to use markdown for comments.
The devil, of course, is in the details.

This RFC modifies the API documentation rubric, as well as tooling we use
to process FIDL API documentation.
It does not affect the FIDL language, in that the set of legal FIDL
remains the same.

### Why Markdown?

The solution space for doc comments can be divided into two parts:
"developing our own solution," and "using an existing solution."
We feel that FIDL is not going to be a large enough ecosystem to warrant
development of a separate standard for comment syntax.
By using an existing solution, developers will be able to leverage
external documentation and tooling (and potentially their existing
knowledge).
In addition, by using an existing solution, we may save development time.

If we commit to using an existing solution, we must pick one.
There are several language specific solutions that could be extended
(e.g., javadoc and python doc strings).
There are also general-purpose solutions (e.g., LaTeX, RST, Markdown).

We believe that Markdown is the best choice.
Unlike the language specific solutions, there are a number of tools that
allow markdown integration into new languages.
Markdown is also widely used and understood by developers: it's used, for
example, by Github for its documentation.
Finally, a number of languages (e.g., Rust and Kotlin) are standardizing
on Markdown for their syntax, and it is starting to supplant existing
solutions in other languages, as well (for example, LLVM is going to be
migrating from RST to Markdown).

### What do you mean, Markdown?

Markdown has a variety of implementations that have slightly different
behaviors.
Any number of them are reasonable choices.
We choose CommonMark, because it is the closest we have to a standard.
For developers whose tools need to target both CommonMark and another
markdown implementation, we recommend keeping their docs compatible with
both, to the extent possible.

Markdown is not extensible, and so does not help you express language
elements.
We add special-purpose extensions of markdown that can be processed by
`fidldoc` (and other API doc consuming tools).

A doc comment is written in markdown and precedes the element it is
documenting.
It contains a description, optionally followed by documentation about
parameters, errors, and "see" (which indicates that the reader should look
at the referenced API for more information).

#### Parameters and Errors

Request parameters should be documented:

```
* request `paramname` Description of parameter
```

Response parameters should be documented:

```
* response `paramname` Description of parameter
```

We also considered `param` and `return`, or `in` and `out`, as the
keywords instead of `request` and `response`.

In cases where the method does not use the same identifier as a parameter
for both the request and response, the words `request` and `response` are
optional.

Methods that return with no parameter value (`Foo() -> ()`) can use the
term `response` without a corresponding parameter for documentation.

Error clauses should be documented:

```
* error Description of error values
```

#### Fully Qualified Names {#fully-qualified-names}

Fully qualified names are of the form:

```
<library>/<top level declaration>.<member>
```

This will uniquely identify any member because there is no overloading.

Currently, ordinal hashing is based on names of the form `<library>.<top
level declaration>/<member>` (see [RFC-0020](/docs/contribute/governance/rfcs/0020_interface_ordinal_hashing.md)), and `fidlc`
reports errors using the form `<library>.<top level declaration>/<member>`.
It's our intent to align these on the unambiguous format described above.
We will amend [RFC-0029: Increasing Method Ordinals](/docs/contribute/governance/rfcs/0029_increasing_method_ordinals.md) to use
`<library>/<top level declaration>.<member>` as the name hashed, and modify
`fidlc` to report errors consistently.

Links to other FIDL language elements that have documentation associated
with them (or _documented entities_) can be made by adding \[\`link-target\`\].
For example, \[\`fidl.io/NodeInfo\`\] links to docs on said library.
The resolution rules are as follows:

1. First, nested elements will be checked.
   If you are documenting a `struct Object`, and it contains a member
   `event`, you may refer to it as \[\`event\`\].
2. Next, elements at the same scope level as the documented element will
   be checked.
   For example, if you are documenting a protocol method `foo()`, and the
   same protocol contains a method `bar()`, you may refer to it as \[\`bar\`\].
3. Next, elements of the enclosing scope are checked, if there is an
   enclosing scope.
   For example, if you are documenting a protocol method `foo()`, and
   there is another protocol in the same library called `Info`, you may refer
   to it (and its elements) by saying \[\`Info\`\].
4. 3 is repeated at successively enclosed scopes until you are at the top
   level scope.
   If you are documenting a protocol method `foo()`, and you write
   \[\`fuchsia.io/NodeInfo\`\], it will refer to the union
   `fuchsia.io/NodeInfo`.

Fully qualified names are of the form `<library>/<top level
declaration>.<member>`, see [details above](#fully-qualified-names).

For other link shortcuts, you can specify the link target, e.g.:

<pre>
<code>[fuchsia-concepts]: https://fuchsia.dev/fuchsia-src/concepts</code>
</pre>

That line will not appear in tooling output.

If the given FIDL target type is known to the tool at runtime, the
location does not need to be specified.
For example, it's likely that docs for `fuchsia.sys/ComponentController`
and `fuchsia.sys/EnvironmentController` will be generated as part of the
same tool invocation.
The tool will know about links between them.

Developers may also use the following to indicate there is a related API:

```
* see [`fidl.io`]
```

Where appropriate.

## Implementation strategy

Implementation will include adding this to the FIDL rubric, publicizing
it, and incorporating the special annotations into the `fidldoc` tool.
 We can also add lint checks for `fidldoc` syntax, either to the `fidl-lint`
tool, or to a separate tool.

## Documentation and examples

A complete example is shown below.
Note that this API does not currently look like this; the status has been
changed to an error for illustration.

```fidl
library fuchsia.io;

protocol File {
    /// Acquires a [`fuchsia.mem/Buffer`] representing this file, if
    /// there is one, with the requested access rights.
    ///
    /// ## Rights
    ///
    /// This method requires the following rights:
    ///
    /// * [`OPEN_RIGHT_WRITABLE`] if `flags` includes
    ///   [`VMO_FLAG_WRITE`].
    /// * [`OPEN_RIGHT_READABLE`] if `flags` includes
    ///   [`VMO_FLAG_READ`] or [`VMO_FLAG_EXEC`].
    ///
    /// + request `flags` a bit field composing any of
    ///   `VMO_FLAG_READ`, `VMO_FLAG_WRITE`, or `VMO_FLAG_EXEC`.
    /// - response `buffer` the requested [`fuchsia.mem/Buffer`], or
    ///     null if there was an error, or the buffer does not exist.
    /// * error a `zx.status` value indicating success or failure.
    /// * see [Filesystem architecture][fs-arch] for further details.
    ///
    /// [fs-arch]: https://fuchsia.dev/fuchsia-src/concepts/filesystems/filesystems
    GetBuffer(uint32 flags) ->
        (fuchsia.mem.Buffer? buffer) error zx.status;
};
```

Note that, by convention, you only need to link the first reference to an
element in a given doc comment.
The first reference to `VMO_FLAG_READ` above is linked, and the second one
is not.

## Backwards compatibility

No significant backwards compatibility issues.
Current doc uses the C++ style `|param|` notation to indicate parameters and
return values.
This can be changed relatively easily.

## Performance

This will impact developer velocity by making them type more, but also
understand more.

## Drawbacks, alternatives, and unknowns

The assumption is that having a format is strictly better than not having
a format.
Therefore, there are few drawbacks.

Alternatives might include other API doc formats.
Java uses Javadoc, which is very verbose and relies on inline HTML.
Developers find it painful.
Other languages use [RST](http://docutils.sourceforge.net/rst.html).
However, this is becoming less popular; developers are simply more
familiar with Markdown.
Notably, the LLVM project is migrating from RST to Markdown.

We considered using a different variant of Markdown.
We decided to use CommonMark because it is the best specified and
standardized.
Developers who need their code to work in other Markdown rendering systems
should try to write doc comments that comply both with CommonMark and the
systems they are targeting.

We considered not inventing a new syntax for linking documented entities.
The alternatives that were considered were:

*   Auto-detection.
    Experience with auto-detection mechanisms in other contexts have shown
    that they rarely detect what the developer intended.
    In addition, auto-detection prevents tooling from surfacing the fact
    that a link is wrong.
    We therefore defer work on auto-detection until a future date.
*   Using existing syntax.
    This has the same problems as auto-detection, but the symptoms are
    somewhat less awful.
    If we were to use `fuchsia.io/NodeInfo` as a syntax, then if it were
    misspelled, the link would not be present, and we would simply get code
    font.
    We would like tooling that detects broken links, instead of having a
    fallback behavior.

Items that we should consider in the future, but are out of scope for this
RFC, include:

*   Ways of embedding images or flowcharts into generated docs.
*   Ways of embedding automatically checked examples into docs.

## Prior art and references

This proposal is heavily influenced by the documentation styles for Rust
and Kotlin.

<!-- xrefs -->
[API documentation rubric]: /docs/development/api/documentation.md
