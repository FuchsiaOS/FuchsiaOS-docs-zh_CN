<!--# Documentation Standards

A document about what to document and how to document it for people who create
things that need documentation.-->

# 文档标准

为做了需要文档化的事情的人提供的一份关于文档写什么以及如何写的文档。

<!--## Why document?

Fuchsia is a new operating system.  As it grows and new people join the project
so grows the need to provide effective documentation resources.-->

## 为什么要写文档？

Fuchsia 是一个新的操作系统。随着项目的发展和新成员的加入提供有效文档资源的需求也随之增长。

<!--## Who is the audience?

The documentation described here is intended to address a technical audience,
i.e. those who expect to implement or exercise APIs or understand the internal
dynamics of the operating system.  These standards are not intended for
end-user product documentation.-->

## 受众是谁？

这里记录的文档是面向技术人员的，即那些希望实现或运行 APIs 或了解操作系统内部动态的人。这些标准不适用于为最终用户产品提供的文档。

<!--## What should I document?

In brief, document your interfaces, introduce essential concepts, explain how
everything fits together.

- Conventions: e.g. this document about documentation, code style
- System Design: e.g. network stack, compositor, kernel, assumptions
- APIs: e.g. FIDL interfaces, library functions, syscalls
- Protocols: e.g. schemas, encodings, wire formats, configuration files
- Tools: e.g. `bootserver`, `netcp`, `fx`
- Workflows: e.g. environment set up, test methodologies, where to find various
  parts, how to get work done-->

## 我应该写什么内容？

简而言之，记录你的程序接口，介绍基本概念，解释所有东西是如何结合在一起的。

- 约定：例如，本文档是关于文档、代码样式
- 系统设计：例如，网络堆栈、compositor、内核、assumptions
- APIs：例如，FIDL 接口、库函数、系统调用
- 协议：例如，schemas、编码、wire formats、配置文件
- 工具：例如，“bootserver”、“netcp”、“fx”
- 工作流程：例如，环境设置、测试方法、在何处查找各个部分、如何完成工作

<!--## Where should I put documents?  What goes where?

Keep your documentation assets in the source tree near the things they
describe.  Where this should go depends on the type of document and its topic.
(See following sections for details.)-->

## 文档保存到哪里？不同文档如何存放？

将文档资源保存在源代码中它们所描述的内容附近。文档应该放在哪里取决于它的类型和主题。（详见以下章节）

<!--### Prose Documentation

Prose documentation is especially effective at explaining big ideas and
demonstrating how to perform particular tasks.  It's great for documenting
system design, protocols, tools, conventions, workflows, and tutorials.

Prose documentation should be written in Markdown format and published as a
file with the extension `.md` in the source repository.-->

### 文档

文档在解释大思想和演示如何执行特定任务方面特别有效。它非常适合记录系统设计、协议、工具、约定、工作流和教程。

<!--Preferred locations:

- Documents about a specific project should go into the `docs` folder at the
  root of that project's repository and be arranged by topic.
  e.g. `//apps/my-project/docs/my-feature.md`
- Documents about Fuchsia as a whole should go into the top-level `docs`
  repository itself.  e.g. `//docs/build_packages.md`-->

优先存放位置

- 有关特定项目的文档应放到该项目存储库根目录下的“docs”文件夹，并按主题排列。例如，`//apps/my-project/docs/my-feature.md`
- 关于 Fuchsia 整体的文档应该放到最外层存储库中的“docs”文件夹。例如，`//docs/build_packages.md`

<!--Alternate locations:

- Adding a `README.md` to the root of a project's repository may serve as a
  brief orientation to the project for first time visitors but this is not
  required.-->

备选存放位置

- 在项目存储库的根目录中添加“README.md”可以为第一次访问项目的人做简要介绍，但这不是必需的。

<!--Tips for writing effective prose documentation:

- Write plain English text.
- Optimize the experience of first time readers.
- Give each document a clear title.
- Briefly describe the purpose and underlying assumptions of each part.
- Be sure to define your jargon; refrain from excess abbreviations.
- Include links to other relevant documentation.
- Stay on topic.
- Use section headers to organize ideas.
- Keep the tone informal.
- Don't restate API documentation which is already published elsewhere (e.g. as
  documentation comments)-->

写出有效文档的技巧：

- 写纯英文文本。
- 优化初次读者的体验。
- 给每份文档一个清晰的标题。
- 简要描述每个部分的目的和基本假设。
- 一定要定义标准术语，不要使用过多的缩写。
- 要包含到其他相关文档的链接。
- 不要偏离主题。
- 使用小标题组织思路。
- 使用非正式的口吻。
- 不要赘述已经在别处写过的 API 文档。(例如，文档注释已写)

<!--### Documentation Comments

Documentation comments are especially effective at describing the purpose of
interfaces, structures, methods, data types, and other elements of program
code.

Documentation comments should be applied consistently to all public APIs since
they are a valuable asset for SDK consumers.-->

### 文档注释

文档注释在描述接口、结构、方法、数据类型和程序代码的其他元素的用途时特别有效。

所有开发使用的 APIs 都应该添加文档注释，因为它们对于SDK用户来说是一项很有用的资产。

<!--Tips for writing effective documentation comments:

- Write plain English text.
- Write complete sentences and paragraphs.
- Keep comments clear and brief, no more than a few sentences.
- Follow the approved style guide for your programming language.
- Always add value; don't restate what is already indicated by the type
  signature.
- Describe units of measure and integrity constraints of variables.
- Link to prose documentation for more elaborate descriptions of how APIs fit
  together as a whole.-->

写出有效文档注释的技巧：

- 写纯英文文本。
- 写出完整的句子和段落。
- 保持注释简洁明了，不超过几句话。
- 遵循编程语言指南中的样式。
- 总是增量来写；不要赘述类型签名已经表示过的内容。
- 描述度量单位和变量的完整性约束。
- 链接到文档以更详细地描述 APIs 是如何作为一个整体组合在一起的。

<!--### Breadcrumbs

Documentation is only useful when your audience can find it.  Adding links to
or from existing documentation artifacts greatly improves the chances that
someone will read it.-->

### 面包屑

文档只有在你的读者能够找到它时才有用。添加指向或来自现有文档中的链接会大大提高别人阅读到它的可能性。

<!--Tips for leaving breadcrumbs:

- Top-down linkage: Add links from more broadly scoped documents to more
  detailed documents to help readers learn more about particular topics.  The
  [Fuchsia book](../the-book/README.md) is a good starting point for top-down
  linkage.
- Bottom-up linkage: Add links from more detailed documents to more broadly
  scoped documents to help readers develop more awareness of the overall
  context of the topics being discussed.  Adding links from module, class, or
  interface level documentation comments to higher level prose documentation
  overviews can be particularly effective.
- Sideways linkage: Add links to documents in related subject domains with
  which a reader should familiarize themselves in order to better understand
  the content of your document.-->

写面包屑的技巧

- 自顶向下链接：添加从范围更广的文档到更详细文档的链接，以帮助读者了解有关特定主题的更多信息。这[Fuchsia book](../the-book/README.md)就是一个很好的自顶向下链接的起点。
- 自底向上链接：添加从更详细的文档到范围更广文档的链接，以帮助读者更好地了解所讨论主题的总体上下文。在模块、类或接口级文档注释中添加到更高级别概述文档的链接会特别有效。
- 横向链接：添加读者应该先熟悉的相关主题文档的链接，以便更好地理解你写的文档内容。