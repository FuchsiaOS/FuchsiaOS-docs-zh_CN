# Documentation style guide

This document gives writing style guidance for Fuchsia.dev. These
guidelines build on the general guidance in the [Google Developers Style
Guide][google-dev-doc-style-guide].

Note: This guide highlights some of the best practices for writing
documentation for Fuchsia. Some of the topics may be covered more extensively
in the resources in the following documents:

* For information on general documentation standards, including file types,
  locations, and general tone, see the [Fuchsia documentation
  standards][doc-standard].
* For specific guidance on word choice, style, and structure, see the
  [Fuchsia documentation style guide][style-guide].
* For the full Markdown reference guide, see the
  [Markdown reference guide][markdown-guide].

## Text and links

### Follow the 80 character limit

In the Fuchsia project, the maximum line length for code is 100 characters,
while the maximum line length for documentation is 80 characters. A notable
exception to this rule is URLs (i.e. links) which are written on one line,
without wrapping.

Code tends to be indented (blank space on the left of the page), while English
prose (documentation) tends to form paragraphs of text. This difference leads to
different width specification.

### Mark external links

Use `{:.external}` to mark any links that are not within `fuchsia.dev`,
`fuchsia.googlesource.com`, or `fuchsia-review.googlesource.com`:

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


<span class="compare-better">Recommended</span>: Create an identifier where you
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
[Markdown Guide][markdown-reference-links].

### Use correct links to different Fuchsia content

In the Fuchsia documentation you can link to three types of contents:

* `/docs/` - Link to documents that are in the `/docs/` directory of the Fuchsia
  source tree. These links must link to a file with an `.md` extension. For
  example, `concepts/README.md`.
* Source code - Link to source code files that exist within the Fuchsia source
  tree. These links can link to any file extension, but these files must exist
  in the source tree. For example, `/src/sys/sysmgr/main.cc`.
* Reference documentation - Links to auto-generated Fuchsia reference
  documentation.
  * Most of the Fuchsia reference documentation doesn't exist in
    the source tree, but is published on [fuchsia.dev][fuchsia-dev]. These links
    must be used as fully qualified URLs. For example,
    `https://fuchsia.dev/reference/fidl/fuchsia.io`.
  * However, some Fuchsia reference documentation exists in the source
    tree. These documents exist in `reference/` and are published in the
    `https://fuchsia.dev/fuchsia-src/reference/` section. These links must link
    to a file with an `.md` extension. For example,
    `reference/syscalls/bti_create.md`.

### Test your links before submitting a change

Once you have created a valid markdown document, you should run `doc-checker`
to ensure that your document uses valid links. When you try to submit a change
that includes a `.md` file, Gerrit runs `doc-checker` and blocks submission if
you have broken links.

To run `doc-checker` locally, use the `fx format-code` tool:

```posix-terminal
fx format-code
```

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

[doc-standard]: contribute/docs/documentation-standards.md
[style-guide]: contribute/docs/documentation-style-guide.md
[markdown-guide]: contribute/docs/markdown.md
[google-dev-doc-style-guide]: https://developers.google.com/style
[markdown-reference-links]: contribute/docs/markdown.md
[external-link-example]: http://example.com
[fuchsia-dev]: https://fuchsia.dev