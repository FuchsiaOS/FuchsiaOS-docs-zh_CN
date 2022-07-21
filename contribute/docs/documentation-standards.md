<!-- 
# Documentation standards overview
 -->
# 文档标准概述

<!-- 
This document outlines standards, structure, tone, and best practices for Fuchsia documentation.
-->
本文档概述了 Fuchsia 文档的标准、结构、语言风格和最佳做法。

<!-- 
## Document locations
 -->
## 文档定位

  <!-- 
  * **Documentation specific to developing a specific Fuchsia feature:**
    Documentation for developers creating or maintaining a specific part of the Fuchsia codebase
    should be kept in the same directory as the source code. These docs are usually in the form of
    `README.md` files embedded throughout the Fuchsia codebase.
  -->
  * **为开发 Fuchsia 具体特性的专门文档：**
    针对开发者创建或维护 Fuchsia 代码库特定部分的文档，应当和源代码保存在同一目录下。这类文档通常以 `README.md` 文件的形式嵌入在 Fuchsia  代码库中。
  <!-- 
  * **General documentation for Fuchsia developers:** Fuchsia documentation should
    be created in <code>/HEAD/docs/</code>.
    In the `/docs/` directory, you should create documentation in one of these sub-directories: 
  -->
  * **面向 Fuchsia 开发者的总体文档：** Fuchsia 文档应当创建在 <code>/HEAD/docs/</code> 中.
    在 `/docs/` 目录下，您应当在以下子目录之一内创建文档：

    * <code>get-started</code>:
       <!-- 
       Specific guidance to download, set up, and start developing on Fuchsia should go in
       `/get-started`. This content should contain opinionated, short tutorials that help new
       users get started on Fuchsia, and link to additional documentation in Fuchsia.dev.
       -->
       有关下载、设置、开始 Fuchsia 开发的具体指南应当放进 `/get-started`。这类内容应当包含观点明确的、简短的教程，以帮助新用户入门 Fuchsia，并在 Fuchsia.dev 中链接至附加文档。
    *  <code>development</code>:
        <!--
        The `/development/` directory (which displays on the site as "Guides") contains
        instructions and tutorials for developers
        working on Fuchsia. This directory includes documentation
        on how to build, run, and test Fuchsia. 
        -->
        `/development/` 目录（在网站上显示为“指南”（Guides））中包含了针对Fuchsia开发者的说明和教程。该目录包含关于如何构建、运行和测试 Fuchsia 的文档。
    *  <code>concepts</code>:
        <!-- 
        The `/concepts` directory contains in-depth explanations of specific features of
        Fuchsia and how they work, including operating system overviews, frameworks, architecture,
        and packages.
        -->
        `/concepts` 目录包含针对 Fuchsia 具体特性及其工作原理的深入解释，包括操作系统概述、框架、架构和软件包（package）。
    *  <code>reference</code>:
        <!--
        The `/reference/` directory contains generated reference docs on Fuchsia tools and APIs,
        including FIDL and kernel reference.
        -->
        `/reference/` 目录包含自动生成的关于 Fuchsia 工具和 API 的参考文档，包括 FIDL 和内核的参考文档。
    *  <code>contribute</code>:
        <!--
        The `/contribute/` directory contains code and documentation contribution processes and
        best practices, including documentation code and style guides, code polcies, and governance.
        -->
        `/contribute/` 目录包含代码和文档的贡献进度以及最佳做法，包含文档准则和风格指南、代码策略以及管理体系。
    *  `images`
        <!--
        The `/images/` directory contains images used in the documentation. You should
        place images in this common directory.
        -->
        `/images/` 目录包含在文档中使用的图像。您应当将图像放在这一公共目录中。

<!-- 
## Document types
 -->
## 文档类型

<!-- 
Most documentation can be divided into the following categories:
 -->
大多数文档可以分为以下几类：

<!-- 
- [Procedural](documentation-types.md#procedural-documentation)
    - Getting started - Initial setup documentation
    - Guides - Task-oriented documentation
 -->
- [程序性的](documentation-types.md#procedural-documentation)
    - 入门 - 初始设置的文档
    - 指南 - 任务导向的文档

<!-- - [Conceptual](documentation-types.md#conceptual-documentation) - Foundational
  documentation focused on teaching more about Fuchsia, Fuchsia architecture, and Fuchsia components -->
- [概念性的](documentation-types.md#conceptual-documentation) - 多侧重于教授 Fuchsia、Fuchsia 架构和 Fuchsia 组件的基础性文档

<!-- 
- [Reference](documentation-types.md#reference-documentation) - Documentation focused on
  detailing the syntax and parameters of Fuchsia APIs and tools. This documentation is usually
  auto-generated.
 -->
- [参考文档](documentation-types.md#reference-documentation) - 侧重于详细说明 Fuchsia API 和 工具的语法和参数的文档。这类文档通常自动生成。

<!-- 
See [Documentation Types](documentation-types.md) for more information.
 -->
要获取更多信息，请参阅[文档类型](documentation-types.md)。

<!-- 
## Documentation and code style guides
 -->
## 文档和代码风格指南

<!-- 
It's important to follow documentation style guidelines to ensure that the documentation
created by a large number of contributors remains consistent. See the
[Documentation style guide](documentation-style-guide.md) for specific documentation guidance and
[Code sample style guide](code-sample-style-guide.md) for code sample guidance. 
-->
为确保由大量贡献者创建的文档都保持一致性，遵循文档风格指南很重要。要获取具体文档指导，请参阅[文档风格指南](documentation-style-guide.md)；要获取代码样例指导，请参阅[代码样例风格指南](code-sample-style-guide.md)。


<!-- 
## Search best practices
 -->
## 搜索的最佳做法
<!-- 
Documentation is only useful when users can find it. Some findability and search best practices
include the following:
 -->
文档只有在用户能查到的时候才算有用。下面是一些关于可查性和搜索的最佳做法：
<!-- 
- Add your document to the table of contents: Add links to documentation in the left sided
  navigation on fuchsia.dev. See [Site navigation and TOC files](documentation-navigation-toc.md)
  for more information.
 -->
 - 将您的文档添加至目录：在 fuchsia.dev 的左侧导航中添加文档链接。要获取更多信息，请参阅[网站导航和目录文件](documentation-navigation-toc.md)  
<!-- 
- Cross-link documentation: Add links to documents on subjects that help readers better understand the
  content of your document. For example, the conceptual document for the [Fuchsia emulator](/development/build/emulator.md)
  links to relevant guides and getting started documents about the Fuchsia emulator.
-->
- 交叉链接文档：添加指向文档主题的链接，以帮助读者更好地理解文档的内容。 例如，[Fuchsia 模拟器] (/development/build/emulator.md) 的概念文档链接到有关 Fuchsia 模拟器的相关指南和入门文档。
<!-- 
- Use consistent terminology: If you're writing about a specific concept in Fuchsia, verify that you are
  using consistent terminology. Use the [glossary](/glossary/README.md) to verify terms.
 -->
- 使用一致的术语：如果您在撰写有关 Fuchsia 中特定概念的文章，请确认您使用的是一致的术语。 使用[术语表](/glossary/README.md)来验证用语。

<!-- 
## Documentation file formats and file names
 -->
## 文档的文件格式和文件名

<!-- 
All documentation for Fuchsia is written in Markdown (`.md`), and Fuchsia.dev
uses the [Hoedown Markdown Parser](https://github.com/hoedown/hoedown).
 -->
Fuchsia 的所有文档均使用 Markdown（`.md`）撰写，Fuchsia.dev 使用 [Hoedown Markdown Parser](https://github.com/hoedown/hoedown)。

<!-- 
The site's navigation is configured by `_toc.yaml` files, which are included in every documentation
directory. Use the guidance in
[Site navigation and TOC files](documentation-navigation-toc.md) to update these files.
 -->
该网站的导航是由 `_toc.yaml` 文件配置的，该文件包含在每个文档目录中。请使用[网站导航和目录文件](documentation-navigation-toc.md)中的指导来更新这些文件。

<!-- 
File and directory names should be lowercase, and separate words with hyphens, not underscores.
Use only standard ASCII alphanumeric characters in file and directory names. If the file name
contains a command with an underscore, then you can include the underscore.
 -->
文件和目录名应当为小写，并使用短横线（hyphen）而非下划线（underscore）来分隔单词。在文件或目录名中请仅使用标准 ASCII 字母数字字符。如果文件名包含带有下划线的命令，那么您可以包含下划线。

<!-- 
## General guidance on style and tone
 -->
## 语言风格总体指导

<!-- 
- **Write in plain U.S. English.** Write in clear, direct U.S. English that makes content
  easy to understand. Use simple words, be concise, and use contractions like _it's_ or _don't_.
 -->
- **使用通俗易懂的美国英语撰写。** 使用清晰、直白的美国英语撰写，以使内容易于理解。请使用简单的词汇，保持简洁，并使用（常见）缩写，如 _it's_ 或 _don't_。
<!-- 
- **Be respectful.** Follow the guidelines set forth in [Respectful Code](/contribute/respectful_code.md).
 -->
- **心怀敬意。** 请遵循[尊重性规范](/contribute/respectful_code.md)中规定的方针。
<!-- 
- **Write in second-person ("you").** Fuchsia documentation is written to the user ("you"). When
  For example, "You can install Fuchsia by doing the following...". Do not refer to the reader in the
  third person ("Fuchsia users can install Fuchsia by...") or use
  "We" ("We can install Fuchsia by...").
 -->
- **使用第二人称（“you”）撰写。** Fuchsia文档是写给用户（“you”）的。例如，“您（you）可以通过以下步骤安装 Fuchsia……”。不要使用第三人称称呼读者（“Fuchsia 用户可以通过……安装 Fuchsia”）或使用“we”（“我们（we）可以通过……安装 Fuchsia”）。
<!-- 
- **Write in present tense.** Always document the system as it is, not as it will be. Words such
  as "will" are very ambiguous. For example "you will see" leads to questions like "when will I see
  this?" In 1 minute or in 20 minutes? In addition, do not refer to future product features unless
  necessary. Mentioning future plans that might not happen becomes a maintenance burden.
 -->
- **使用现在时态撰写。** 请在记录系统时始终立足眼下（it is），而非未来（it will be）。类似“will”之类的词语非常含糊。例如“您将看到”这种说法将引起如“我何时将会看到？”之类的问题。1分钟，还是20分钟呢？另外，若非必要，请不要提及未来的产品特性。提及可能取消的未来计划将导致维护上的困难。
<!-- 
- **Keep sentences short and concrete.** Using punctuation allows your reader to follow
  instructions and understand concepts. Also, short sentences are easier to translate.
 -->
- **保持语句简短具体。** 使用标点符号可以便于您的读者跟进说明、理解概念。而且，短句更易于翻译。
<!-- 
- **Know your audience.** Define your audience before you write a document. Knowing your audience
  allows you to understand what information your audience should be familiar with. When a document
  is meant for a more advanced audience, state that up front and let users know that as a
  prerequisite before reading your document.
 -->
- **了解您的受众群体。** 在撰写文档之前，请确定好您的受众群体。了解受众群体使您能够明确他们应当熟悉的概念。当撰写面向更高级受众群体的文档时，请在文档前声明，让用户了解这一前提后，再进行阅读。
<!-- 
- **Use active voice.** Try to write in active voice since passive voice can
  make sentences ambiguous and hard to understand. Here's an example:
  - Active voice: "The operating system runs a process." In this case, the subject performs the
    action denoted by the verb.
  - Passive voice: "A process is being run." The subject is no longer _active_, but is being acted
    upon by the verb — it's _passive_.
  In most cases, if you use "by" this indicates that your sentence might be still be in passive
  voice.
 -->
- **使用主动语态。** 请尽量使用主动语态写作，因为被动语态会使句子模棱两可且难以理解。 下面是一个例子：
   - 主动语态：“操作系统运行一个进程。”（The operating system runs a process.）在这种情况下，主语执行动词表示的动作。
   - 被动语态：“一个进程正在被运行。”（A process is being run.）主语不再是“主动的”（_active_），而是被动词作用——它是“被动的”（_passive_）。
   大多数情况下，如果您使用了“by”，那么您的句子可能仍然是被动语态。

<!-- 
- **If you use acronyms, define them the first time you write about them.** For
  example, looks good to me (LGTM). Don't assume that everyone will understand all acronyms. You do
  not need to define acronyms that are industry standards such as TCP/IP.
 -->
- **若使用首字母缩写词，请您在第一次书写时进行定义。** 例如，looks good to me（LGTM，我觉得看起来很好）。不要认为每个人都理解所有的首字母缩写词。您不需要定义工业标准首字母缩写词，如 TCP/IP。

<!-- 
- **Define technical terms and avoid jargon.** Fuchsia documentation should be accessible
  to all levels of developers. Avoid overcomplicating documentation with uncommon or highly
  technical words. If you're using Fuchsia-specific terms, define them in
  the [glossary](/glossary/README.md). Avoid invented words.
 -->
- **定义技术术语，回避行话。** Fuchsia 文档应当易于各个层次的开发者理解。请避免使用不常用或高度技术性词语而使得文章过于复杂。如果您使用了 Fuchsia 特定的术语，请在[术语表](/glossary/README.md)中对其进行定义。请回避自造词。

<!-- 
- **Avoid colloquial phrases or regional idioms.** Keep in mind that many Fuchsia users
  may not be native English speakers. Avoid difficult to translate idioms, like
  "that's the way the cookie crumbles." While it might make sense to you, it doesn't translate
  well into other languages.
 -->
- **回避口头表达或地区习语。** 请记住，许多 Fuchsia 用户并非英语母语者。请避免使用难于翻译的习语，例如“that's the way the cookie crumbles.”（生米已成熟饭/覆水难收）虽然对您而言能够理解，但是很难准确地翻译到其他语言中。

<!-- 
- **Avoid referencing proprietary information.** This can refer to any potential terminology or
  product names that may be trademarked or any internal information (API keys, machine names, etc…)
  internal to your company.
 -->
- **避免引用专有信息。** 这里指的是任何可能是已注册商标的任何潜在词语或您公司的任何内部信息（API 密钥、机器名等）。

<!-- 
- **Use gender-neutral pronouns.** Don't use _he, him, his, she,_ or _her,_ and don't use _he/she_ or
  _(s)he_ or other such punctuational approaches. Instead, use the singular _they._
 -->
- **使用性别中立代词。** 请不要使用 _he_，_him_，_his_，_she_ 或 _her_，也不要使用 _he/she_ 或 _(s)he_ 等其他类似的符号表达方式。取而代之，请使用单数的 _they_。

<!-- 
- **Use consistent terminology.** Ensure that terms are consistent in code, UI, and documentation.
  Use common terms when possible, and use the [glossary](/glossary/README.md) to verify terminology.
 -->
- **使用一致的术语。** 确保术语在代码、用户界面和文档中是一致的。尽可能使用常见术语，并使用[术语表](/glossary/README.md)来验证用语。
