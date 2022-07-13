<!-- 
# Documentation style guide
 -->
# 文档风格指南

<!-- 
This document gives writing style guidance for Fuchsia.dev. These
guidelines build on the general guidance in the [Google Developers Style
Guide][google-dev-doc-style-guide].
 -->
本文档给出了针对 Fuchsia.dev 写作风格的指南。这些方针基于[Google 开发者风格指南][google-dev-doc-style-guide]中的通用指导。

<!-- 
Note: This guide highlights some of the best practices for writing
documentation for Fuchsia. Some of the topics may be covered more extensively
in the resources in the following documents:
 -->
注意：本指南重点介绍了为 Fuchsia 编写文档的一些最佳做法。其中的某些主题在以下文档的资源中可能会得到更为广泛的探讨：

<!-- 
* For information on general documentation standards, including file types,
  locations, and general tone, see the [Fuchsia documentation
  standards][doc-standard].
* For specific guidance on word choice, style, and structure, see the
  [Fuchsia documentation style guide][style-guide].
* For the full Markdown reference guide, see the
  [Markdown reference guide][markdown-guide].
 -->
* 要获取关于通用文档标准的信息，包括文件类型、位置和整体语言风格，请参阅 [Fuchsia 文档标准][doc-standard]。
* 要获取关于措辞、风格和结构的具体指导，请参阅 [Fuchsia 文档风格指南][style-guide]。

<!-- 
## Text and links
 -->
## 文字和链接

<!-- 
### Follow the 80 character limit
 -->
### 遵循 80 字符限制

<!-- 
In the Fuchsia project, the maximum line length for code is 100 characters,
while the maximum line length for documentation is 80 characters. A notable
exception to this rule is URLs (i.e. links) which are written on one line,
without wrapping.
 -->
在 Fuchsia 项目中，代码的行最大长度为 100 字符，而文档的行最大长度为 80 字符。该规则一个值得注意的例外情况是 URL（即链接），写在一行中，不换行。

<!-- 
Code tends to be indented (blank space on the left of the page), while English
prose (documentation) tends to form paragraphs of text. This difference leads to
different width specification.
 -->
代码常使用缩进（页面左侧的空格），而英文散文（文档）多形成文字段落。这种差异导致了不同的宽度规格。

<!-- 
### Mark external links
 -->
### 标记外部链接

<!-- 
Use `{:.external}` to mark any links that are not within `fuchsia.dev`,
`fuchsia.googlesource.com`, or `fuchsia-review.googlesource.com`:
 -->
请使用 `{:.external}` 标记任何不属于 `fuchsia.dev`、
`fuchsia.googlesource.com` 或 `fuchsia-review.googlesource.com` 的链接：

<!-- 
```none
This is an [external](http://example.com){:.external} link.
```
 -->
```none
这是一个[外部](http://example.com){:.external}链接。
```

<!-- 
Notice the external link icon: This is an
[external][external-link-example]{:.external} link.
 -->
注意外部链接标志：这是一个[外部][external-link-example]{:.external}链接。

<!-- 
### Use reference-style links
 -->
### 使用参考风格的链接

<!-- 
In general, Fuchsia recommends using reference-style links in Markdown files.
Reference style links use a reference identifier associated with the link, and
then refers to that identifier whenever you use the link in the doc. This makes
links easy to update in the document.
 -->
一般来说，Fuchsia 建议在 Markdown 文件中使用参考风格（reference-style）的链接。 参考风格链接使用与链接相关联的参考标识符，接下来每当您在文档中使用该链接时，即可引用该标识符。这使得文档中的链接易于更新。


<!-- 
<span class="compare-better">Recommended</span>: Create an identifier where you
want the link.
 -->
<span class="compare-better">推荐</span>：请在您需要链接的位置创建标识符。


<!-- 
In this example, the link identifier is called `fuchsia-home`:
 -->
本例中，链接标识符称为 `fuchsia-home`：

<!-- 
```none
Welcome to the [Fuchsia home page][fuchsia-home].
```
 -->
```none
欢迎来到 [Fuchsia 首页][fuchsia-home]。
```

<!-- 
And then define it at the bottom of the document:
 -->
接着在文档底部对其定义：


<pre><code>[fuchsia-home]: https://fuchsia.dev/</code></pre>


<!-- 
<span class="compare-worse">Not recommended</span>: Writing an in-line link
like the following:
 -->
<span class="compare-worse">不推荐</span>：编写如下的行内链接：

<!-- 
```none
Welcome to the [Fuchsia home page](www.fuchsia.dev).
```
 -->
```none
欢迎来到 [Fuchsia 首页](www.fuchsia.dev)。
```
<!-- 
You can read more about reference style links in the external
[Markdown Guide][markdown-reference-links].
 -->
您可以在外部的 [Markdown 指南][markdown-reference-links]中阅读更多与参考风格链接有关的信息。

<!-- 
### Use correct links to different Fuchsia content
 -->
### 对不同的 Fuchsia 内容使用正确链接

<!-- 
In the Fuchsia documentation you can link to three types of contents:
 -->
在 Fuchsia 文档中您可以为三类内容添加链接：

<!-- 
* `/docs/` - Link to documents that are in the `/docs/` directory of the Fuchsia
  source tree. These links must link to a file with an `.md` extension. For
  example, `/concepts/README.md`.
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
    tree. These documents exist in `/reference/` and are published in the
    `https://fuchsia.dev/fuchsia-src/reference/` section. These links must link
    to a file with an `.md` extension. For example,
    `/reference/syscalls/bti_create.md`.
 -->
* `/docs/` - 链接至位于 Fuchsia 源树中 `/docs/` 目录内的文档。这些链接必须关联到具有 `.md` 扩展名的文件。例如，`/concepts/README.md`。
* 源代码 - 链接至位于 Fuchsia 源树中的源代码文件。这些链接可以关联到任何文件扩展名，但这些文件必须存在于源树中。例如，`/src/sys/sysmgr/main.cc`。
* 参考文档 - 链接至自动生成的 Fuchsia 参考文档。
  * 大多数 Fuchsia 参考文档不存在于源树内，但在 [fuchsia.dev][fuchsia-dev] 上发布了。这些链接必须使用完整形式的 URL。例如，`https://fuchsia.dev/reference/fidl/fuchsia.io`。
  * 不过，一些 Fuchsia 参考文档存在于源树内。这些文档位于 `/reference/`，并在 `https://fuchsia.dev/fuchsia-src/reference/` 中发布。这些链接必须关联到具有 `.md` 扩展名的文件。例如，`/reference/syscalls/bti_create.md`。


<!-- 
### Test your links before submitting a change
 -->
### 测试您的链接后再提交更改

<!-- 
Once you have created a valid markdown document, you should run `doc-checker`
to ensure that your document uses valid links. When you try to submit a change
that includes a `.md` file, Gerrit runs `doc-checker` and blocks submission if
you have broken links.

To run `doc-checker` locally, use the `fx format-code` tool:
 -->
一旦创建了有效的 markdown 文件，您应当运行 `doc-checker` 以确保您的文件使用了有效的链接。当您试图提交包含 `.md` 文件的更改时，Gerrit 会运行 `doc-checker`，并会在您的提交中含有损坏链接时进行阻止。

要在本地运行 `doc-checker`，请使用 `fx format-code` 工具：

```posix-terminal
fx format-code
```

<!-- 
## Headers
 -->
## 标头（header）

<!-- 
### Use sentence case for page and section titles
 -->
### 为页面和章节标题使用句首字母大写格式（sentence case）

<!-- 
<span class="compare-better">Recommended</span>: Using sentence case.
 -->
<span class="compare-better">推荐</span>：使用句首字母大写格式（sentence case）：

```none
# This title is an example of sentence case
```

<!-- 
<span class="compare-worse">Not recommended</span>: Using title case:
 -->
<span class="compare-worse">不推荐</span>：使用标题词首大写格式（title case）：

```none
# This Title is an Example of Title Case
```

<!-- 
### Use dashes, not underscores, for anchors
 -->
### 为锚点使用连接号（dash），不要使用下划线

<!-- 
By default, `fuchsia.dev` creates anchors using underscores (`_`) in place of
spaces. When referencing a section in a page, create a custom anchor using
dashes (`-`) instead, using `{#section-title}`. Also, use dashes for file names.
 -->
默认情况下，`fuchsia.dev` 创建锚点时会在在空格处使用下划线（`_`）。不过，当引用一个页面中的章节时，请使用连接号（`-`，dash），使用 `{#section-title}`。同样地，请在文件名中使用连接号。

<!-- 
<span class="compare-better">Recommended</span>: Using dashes for anchors
 -->
<span class="compare-better">推荐</span>：为锚点使用连接号

```none
 ## 这是一个章节标题 {#this-is-a-section-header}
```

<!-- 
## Code samples
 -->
## 代码样例

<!-- 
### Use posix-terminal for shell command examples
 -->
### 为 shell 命令样例使用 posix-terminal 格式化

<!-- 
<span class="compare-better">Recommended</span>: Allow readers to easily copy
the content in a code block by adding `posix-terminal` after <code>```</code>
for a shell command.
 -->
<span class="compare-better">推荐</span>：为 shell 命令在 <code>```</code> 后添加 `posix-terminal` 能让读者更容易复制代码块中的内容。


<pre>
<code>```posix-terminal
fx ota
```</code>
</pre>

<!-- 
This code block is rendered with `$` in the front of the command:
 -->
该代码块在渲染时，命令前会添加 `$`：

```posix-terminal
fx ota
```

<!-- 
<span class="compare-worse">Not recommended</span>: Don't hardcode a `$`
character in the command.
 -->
<span class="compare-worse">不推荐</span>：请勿在命令前硬编码一个 `$` 符号。

```sh
$ fx ota
```

<!-- 
### Use none to disable the copy feature
 -->
### 使用 none 以禁用复制功能

<!-- 
<span class="compare-better">Recommended</span>: Add `none
{:.devsite-disable-click-to-copy}` after <code>```</code> for code or output
examples that do not require readers to copy the content.
 -->
<span class="compare-better">推荐</span>：对于不需要读者复制内容的代码或输出样例，请在 <code>```</code> 后添加 `none {:.devsite-disable-click-to-copy}`。

<!-- 
<pre>
<code>```none {:.devsite-disable-click-to-copy}
$ my_command
It won't be necessary to copy and paste this code block.
```</code>
</pre>
 -->
<pre>
<code>```none {:.devsite-disable-click-to-copy}
$ my_command
不必复制和粘贴该代码块。
```</code>
</pre>

<!-- 
This code block is rendered without the copy icon in the top right corner:
 -->
该代码块在渲染时，右上角没有复制标志：

<!-- 
```none {:.devsite-disable-click-to-copy}
$ my_command
It won't be necessary to copy and paste this code block.
```
 -->
 ```none {:.devsite-disable-click-to-copy}
$ my_command
不必复制和粘贴该代码块。
```

<!-- 
<span class="compare-worse">Not recommended</span>: Enable the copy feature for
view-only content. If you don't specify anything after <code>```</code>, the
copy feature is enabled by default.
 -->
<span class="compare-worse">不推荐</span>：为只需查看的内容启用复制功能。如果您在 <code>```</code> 之后不指定任何内容，那么复制功能默认是启用的。

<!-- 
<pre>
<code>```
$ my_command
It won't be necessary to copy and paste this code block.
```</code>
</pre>
 -->
<pre>
<code>```
$ my_command
不必复制和粘贴该代码块。
```</code>
</pre>

<!-- 
This code block is rendered as below:
 -->
该代码块渲染为如下形式：

<!-- 
```
$ my_command
It won't be necessary to copy and paste this code block.
```
 -->
```
$ my_command
不必复制和粘贴该代码块。
```

<!-- 
### Use paths instead of URLs when referring to source code
 -->
### 引用源代码时使用路径，不要使用 URL

<!-- 
<span class="compare-better">Recommended</span>: Any links that refer to source
code should be referred to by path only. You will get a static error check
otherwise.
 -->
<span class="compare-better">推荐</span>：任何引用源代码的链接应当仅使用路径指代。否则您将收到静态错误检查（static error check）。

<!-- 
<pre>
Update the [state header][sh]
[sh]: /zircon/system/ulib/inspect/include/lib/inspect/cpp/vmo/state.h
</pre>
 -->
<pre>
更新[状态标头][sh]
[sh]: /zircon/system/ulib/inspect/include/lib/inspect/cpp/vmo/state.h
</pre>

<!-- Reference links -->

[doc-standard]: /contribute/docs/documentation-standards.md
[style-guide]: /contribute/docs/documentation-style-guide.md
[markdown-guide]: /contribute/docs/markdown.md
[google-dev-doc-style-guide]: https://developers.google.com/style
[markdown-reference-links]: /contribute/docs/markdown.md
[external-link-example]: http://example.com
[fuchsia-dev]: https://fuchsia.dev