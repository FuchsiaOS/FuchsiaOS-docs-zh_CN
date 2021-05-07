# Documentation style guide

This document gives writing style guidance for Fuchsia.dev, and these
guidelines build on the general guidance in the [Google Developers Style
Guide][google-dev-doc-style-guide].

For information on general documentation standards, including file types,
locations, and general tone, see the [Fuchsia documentation
standards][doc-standard]. For specific guidance on word choice, style, and
structure, see the [Fuchsia documentation style guide][style-guide].

## Text and links

### Follow the 100 character limit

In the Fuchsia project, the maximum line length for documentation and code is
100 characters.

### Mark external links

Use `{:.external}` to mark any links that are not within `fuchsia.dev` or
`fuchsia.googlesource.com`:

```none
This is an [external](http://example.com){:.external} link.
```

Notice the external link icon: This is an
[external][external-link-example]{:.external} link.

### Use reference-style links

In general, Fuchsia recommends using reference-style links in Markdown files.
Reference style links use a reference identifier associated with the link, and
then refers to that identifier whenever you use the link in the doc. This makes
links easy to update in the document.


<span class="compare-better">Recommended</span>: Create an identifer where you
want the link.

In this example, the link identifier is called `fuchsia-home`:

```none
Welcome to the [Fuchsia home page][fuchsia-home].
```

And then define it at the bottom of the document:

<pre><code>[fuchsia-home]: https://fuchsia.dev/</code></pre>

<span class="compare-worse">Not recommended</span>: Writing an in-line link
like the following:

```none
Welcome to the [Fuchsia home page](www.fuchsia.dev).
```

You can read more about reference style links in the external
[Markdown Guide][markdown-reference-links]{:.external}.

## Headers

### Use sentence case for page and section titles

<span class="compare-better">Recommended</span>: Using sentence case.

```none
# This title is an example of sentence case
```

<span class="compare-worse">Not recommended</span>: Using title case:

```none
# This Title is an Example of Title Case
```

### Use dashes, not underscores, for anchors

By default, `fuchsia.dev` creates anchors using underscores (`_`) in place of
spaces. When referencing a section in a page, create a custom anchor using
dashes (`-`) instead, using `{#section-title}`. Also, use dashes for file names.

<span class="compare-better">Recommended</span>: Using dashes for anchors

```none
 ## This is a section header {#this-is-a-section-header}
```

## Code samples

### Use posix-terminal for shell command examples

<span class="compare-better">Recommended</span>: Allow readers to easily copy
the content in a code block by adding `posix-terminal` after <code>```</code>
for a shell command.

<pre>
<code>```posix-terminal
fx ota
```</code>
</pre>

This code block is rendered with `$` in the front of the command:

```posix-terminal
fx ota
```

<span class="compare-worse">Not recommended</span>: Don't hardcode a `$`
character in the command.

```sh
$ fx ota
```

### Use none to disable the copy feature

<span class="compare-better">Recommended</span>: Add `none
{:.devsite-disable-click-to-copy}` after <code>```</code> for code or output
examples that do not require readers to copy the content.

<pre>
<code>```none {:.devsite-disable-click-to-copy}
$ my_command
It won't be necessary to copy and paste this code block.
```</code>
</pre>

This code block is rendered without the copy icon in the top right corner:

```none {:.devsite-disable-click-to-copy}
$ my_command
It won't be necessary to copy and paste this code block.
```

<span class="compare-worse">Not recommended</span>: Enable the copy feature for
view-only content. If you don't specify anything after <code>```</code>, the
copy feature is enabled by default.

<pre>
<code>```
$ my_command
It won't be necessary to copy and paste this code block.
```</code>
</pre>

This code block is rendered as below:

```
$ my_command
It won't be necessary to copy and paste this code block.
```

### Use paths instead of URLs when referring to source code

<span class="compare-better">Recommended</span>: Any links that refer to source
code should be referred to by path only. You will get a static error check
otherwise.

<pre>
Update the [state header][sh]
[sh]: /zircon/system/ulib/inspect/include/lib/inspect/cpp/vmo/state.h
</pre>


<!-- Reference links -->

[doc-standard]: /docs/contribute/docs/documentation-standards.md
[style-guide]: /docs/contribute/docs/documentation-style-guide.md
[google-dev-doc-style-guide]: https://developers.google.com/style
[markdown-reference-links]: https://www.markdownguide.org/basic-syntax/#reference-style-links
[external-link-example]: http://example.com

