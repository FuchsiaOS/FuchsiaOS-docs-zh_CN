<!--
# Command-line Tools Rubric
-->

# 命令行工具说明

<!--

## Overview

This document is for command line interface (CLI) tools. Graphical User
Interfaces (GUI) are out of scope.

When developing tools for Fuchsia there are specific features and styles that
will be used to create consistency. This document walks through those
requirements.

The goal is to maintain a uniform fit and finish for Fuchsia developer tools so
that developers can know what to expect. They can most easily see how to
accomplish common tasks and there is a well lit path to discover rarer used
tools.
-->

## 概述

本文档适用于命令行界面（CLI）工具。图形用户界面（GUI）不在范围内。

在为Fuchsia开发工具时，将使用特定的功能和样式来创建一致性。本文档将介绍这些要求。

目标是保持Fuchsia开发人员工具的统一适合度和完成度，以便开发人员可以知道会发生什么。他们可以很容易地看到如何完成常见的任务，并有一个很好的路径来发现很少使用的工具。

<!--

## Guide

The experience developers have writing software for Fuchsia will impact their
general feelings toward writing for the platform and our tools are a significant
part of that experience. Providing tools that are inconsistent (with one
another) creates a poor developer experience.

This guide provides a rubric that Fuchsia tools must follow.
-->

## 指南

开发人员为Fuchsia编写软件的经历将影响他们的对平台和我们的工具使用的总体感觉，而我们的工具是该经历的重要组成部分。提供不一致的工具（相互之间）会给开发人员带来糟糕的体验

本指南提供了Fuchsia工具必须遵循的规则。

<!--
> **IDK**
>
> Some sections have an "IDK" call-out, like this one. These detail specific
> rules that apply to tools included with the Fuchsia Integrator Development Kit distribution.
-->

> **IDK**
>
> 某些部分带有“ IDK”标注，例如这一部分。这些详细的特定规则适用于Fuchsia Integrator Development Kit发行版随附的工具。

<!--
## Considerations

Before embarking on the creation of a new tool, consider these factors to
determine if the tool is a good fit for Fuchsia or the Fuchsia SDK.
-->

## 注意事项

在开始创建新工具之前，请考虑以下因素，以确定该工具是否适合Fuchsia或Fuchsia SDK。

<!--
> **IDK**
>
> IDK tools are specific to Fuchsia in some way. Generic tools or tools that are
> widely available should not be part of Fuchsia and will not be included in the
> Fuchsia IDK. For example, a tool that verifies generic JSON files would not be
> a good addition. However a tool that verifies Fuchsia `.cmx` files, which
> happen to use the JSON format, would be okay.
-->

> **IDK**
>
> IDK工具在某种程度上特定于Fuchsia。通用工具或广泛使用的工具不应该是Fuchsia的部分，也不会包含在Fuchsia IDK中。例如，验证通用JSON文件的工具不是很好的选择。但是，可以使用一种验证Fuchsia `.cmx`文件（恰好使用JSON格式）的工具。

<!--
> **ffx**
>
> [ffx](/docs/development/tools/ffx/overview.md) is Fuchsia's unified CLI
> tool platform for host to target interactions. It provides a logical
> subcommand based grouping that maps to high-level Fuchsia workflows.
> It also provides a plugin framework to allow contributors to expand the
> `ffx` command surface. `ffx` is distributed as part of the Fuchsia IDK.
-->

> **ffx**
>
> [ffx](/docs/development/tools/ffx/overview.md) 是Fuchsia的统一CLI 工具平台，用于主机到目标的交互。它提供了基于逻辑子命令的分组，可映射到高级Fuchsia工作流。它还提供了一个插件框架，以允许贡献者扩展`ffx`命令界面。`ffx` 是FuchsiaIDK的一部分。

<!--
### Audience

Tools may be used for different development tasks. On a large team these roles
may be separate people. Some categories are:

- Component development
- Driver development (DDK)
- Fuchsia development (SDK)
- Build integration (GN, etc.)
- Quality assurance (QA)
- System integrators (e.g., on-device network tools)
- Publishing (from dev host to server)
- Deployment (from server to customers)

Consider which users may use a tool and cater the tool to the audience.

Tools may have different integration expectations. For example, a developer
doing mod development may expect tools to integrate with their Integrated
Development Environment (IDE), while a build integration tool may be called from
a script.
-->

### 用户

工具可以用于不同的开发任务。在一个大型团队中，这些角色可能是独立的人。比如

- 组件开发
- 驱动程序开发（DDK）
- Fuchsia开发（SDK）
- 构建集成（GN等）
- 质量保证（QA）
- 系统集成商（例如，设备上的网络工具）
- 发布（来自开发者主机到服务器）
- 部署（从服务器到客户）


考虑哪些用户可以使用该工具并将使该工具可以满足用户。

不同的人对于工具可能具有不同的期望。例如，开发人员进行mod开发可能希望工具能够与其集成开发环境（IDE）集成在一起，而构建集成工具则可以从脚本中调用。

<!--
### Grouping Related Tools

Prefer to put related commands under a common tool, such as `ffx`.
As an example, `git`, `ffx`, or `fx` present many features (or,
"sub-tools") under a single user-facing command. This helps encourage
the team toward a shared workflow and provides a single point of
discovery.

Prefer subcommands to multiple tools. E.g. don't create tools with hyphenated
names like `package-create` and `package-publish`, instead create a `package`
command that accepts create and publish subcommands.

Keep the number of commands under a tool organized and reasonable. I.e. avoid
adding unrelated commands to a tool and provide sensible organization of the
commands in the help and documentation.
-->

### 分组相关工具

将相关命令放在通用工具例如 `ffx`下。例如 `git` ，`ffx`或 `fx` 在单个面向用户的命令下提供了许多功能（或"sub-tools"）。这可以激励团队共享工作流程，不遗漏星星点点的发现。

优先使用子命令而不是多个工具。例如，不要创建带有连字符名称的工具，例如 `package-create` 和 `package-publish`，取而代之的是创建一个 `package`接受创建和发布子命令的命令

保持一个工具下的命令数量有组织和合理。例如，避免向工具中添加不相关的命令，并在帮助和文档中提供合理的命令组织。

<!--
### Scope of Work

Command line tools can be divided into two groups: simple single purpose tools
and larger more featureful tools. Create tools that are ergonomic for their
purpose. Simple tools should be quick to start up, while more complex tools will
lean toward the more featureful.

Larger tools will encompass an entire task at the user (developer) level. Avoid
making a tool that accomplishes one small step of a task; instead make a tool
that will perform a complete task.

For example, when:

- developing a C++ application: run the preprocessor, run the compiler, run the
linker, start the built executable.
- working on a unit test: build the tests and run the tests being worked on
- developing a mod: compile the code, move the code and resources to the device,
start the mod (or hot-reload)

Lean toward a tool that will accomplish all the steps needed by default, but
allow for an advanced user to do a partial step (for example, passing an
argument to ask the C++ compiler to only run the preprocessor).
-->

### 工作范围

命令行工具可以分为两类：简单的单一目的工具和功能更大的更大工具。创建人性化的工具。简单的工具应该能够快速启动，而更复杂的工具将倾向于更多功能。

较大的工具将涵盖用户（开发人员）级别的整个任务。避免制作完成一项任务的工具；而是制作一个可以执行完整任务的工具。

例如，在以下情况下：

- 开发C ++应用程序：运行预处理器，运行编译器，运行链接器，启动生成的可执行文件。
- 进行单元测试：构建测试并运行正在进行的测试
- 开发mod：编译代码，将代码和资源移至设备，启动mod（或热重载）

精益软件，使该工具能完成默认情况下所需的所有步骤，但允许高级用户执行部分步骤（例如，传递参数以要求C ++编译器仅运行预处理器）。

<!--
> **IDK**
>
> For development environment integrators and EngProd teams, separate tools.
> The build integrators will learn each and piece them together to make a
> working system.
-->

> **IDK**
>
> 对于开发环境集成商和测试团队来说,单独的工具。构建集成商将学习每一种方法，并将它们组合在一起形成一个工作系统。

<!--
> **ffx**
>
> `ffx` introduces many subgroupings and related subcommands.
> In general for tools that fall in the categories such as host to target
> interaction, system integration, and publishing, prefer extending the
> existing `ffx` service instead of a new standalone tool. This can be accomplished
> by extending `ffx` via additional flags, options or subcommands to take
> advantage of shared code and functionality. For considerations and
> additional details refer to the `ffx` development
> [overview](/docs/development/tools/ffx/overview.md).
-->

> **ffx**
>
> `ffx` 引入了许多子组和相关的子命令。通常，对于这些属于主机到目标交互，系统集成和发布之类的工具，最好扩展现有的 `ffx` 服务，而不是新的独立工具。这可以通过以下方式来实现：通过附加标志，选项或子命令扩展 `ffx` 以利用共享代码和功能的优势。有关注意事项和其他详细信息，请参见 `ffx` 开发
> [概述](/docs/development/tools/ffx/overview.md).

<!--
#### Sharing common functionality

If a small step of a task will be needed by several tools, it doesn't make sense
to duplicate that code. Consider making a small support tool or create a library
to share the code.

Making a small tool that performs one step of the task can make sense to promote
code reuse. If the user is not expected to run this small tool individually,
place the support tool in a directory that is not added to the `$PATH`. I.e.
avoid polluting the environment path unnecessarily.

Providing a library to share code may be preferable, so that a subprocess isn't
needed.
-->

#### 共享通用功能

如果一个任务的一个小步骤需要多个工具，那么复制该代码是没有意义的。考虑制作一个小型支持工具或创建一个库来共享代码。


制作一个只执行任务的一个步骤的小工具对于促进代码重用是有意义的。如果不希望用户单独运行这个小工具，请将支持工具放在一个没有添加到 `$PATH`的目录中。可以避免不必要地污染环境变量路径。

最好提供一个库来共享代码，这样就不需要子流程了。

<!--
## Implementation

Here is some guidance for the nuts and bolts of creating a tool. We'll cover
which language to write the tool in, what style to use in that language, and so
on.
-->

## 实施

这里是一些有关创建工具的基本指导。我们将介绍用哪种语言编写工具，以哪种语言使用该语言等等。

<!--
> **ffx**
>
> `ffx` follows the rubric and conventions laid out below and provides
> a reference implementation for the outlined recommendations.
-->

> **ffx**
>
> `ffx` 遵循下面列出的规则和约定，并为概述的建议提供了参考实现。

<!--
### Naming

The following applies to names of binaries, tools, sub-commands, and long
parameter flags.

Use well-known US English terms or nouns for names. Well-known nouns includes
those in common use for the subject matter, or the names of whole subsystems.
If a name does not appear in documentation, it is likely not well-known. If
it does not appear in any implementation, it is definitely not well-known.

Only use lower-case letters (`a-z`) in the US-ASCII character set and hyphens.
A single hyphen (`-`) is used to separate words in a name. A Platform
required extension is an exception (such as `.exe`).

Name CLI tools with more than three characters. Keep the short file names
available for user shortcuts (aliases). If you believe a tool should have
a very short name, request approval from the Fuchsia API Council.

Keeping the points above in mind:

- Prefer whole words rather than abbreviations.
- Prefer shorter names where a user is expected type the name frequently. For
  less frequently typed names bias to more explicit names.
- Prefer a single word to multiple words
- Prefer subcommands to multiple tools that are hyphenated (e.g. avoid
  `foo-start`, `foo-stop`, `foo-reset`; instead have `foo` that accepts
  commands `start|stop|reset`).
- Prefer symmetry (particularly in verbs) with other similar commands or
  sub-systems, unless that introduces a broken metaphor.
-->

### 命名

以下内容适用于二进制文件，工具，子命令和长参数标志的名称。

使用著名的美国英语术语或名词作为名称。熟知的名词包括主题通用的名词或整个子系统的名称。如果名称未出现在文档中，则可能不是熟知的名称。如果它没有出现在任何实现中，则绝对不是熟知的。

在US-ASCII字符集和连字符中只能使用小写字母 (`a-z`) 。单个连字符 (`-`) 用于分隔名称中的单词。平台必需的扩展名是一个例外（例如 `.exe`）。

使用三个以上的字符来命名CLI工具。使简短文件名可用于用户快捷方式（别名）。如果您认为工具的名称应该很短，请向请求Fuchsia API 委员会批准。

请牢记以下几点：

- 优先使用整个单词而不是缩写。
- 在需要用户输入的情况下，最好使用较短的名称。对于不太频繁键入的名称，请偏向更明确的名称。
- 优先使用单个单词而不是多个单词
- 优先使用子命令而不使用多个带连字符的工具（例如，避免使用 `foo-start`, `foo-stop`, `foo-reset`；而应使用 `foo` 来接受命令 `start|stop|reset`）.
- 优先选择对称性（尤其是动词形式）与其他类似的命令或子系统，除非这引入一个不完整的隐喻。

<!--
### Programming Languages

Tools may be written in C++, Rust, and Go. For clarity, here are some languages
not approved: Bash, Python, Perl, JavaScript, and Dart (see exceptions below).

No language is preferred between C++, Rust, and Go. The choice between these
languages is up to the author of the tool.
-->

### 编程语言

工具可以用C ++，Rust和Go编写。为了清楚起见，以下是一些未获批准的语言：Bash，Python，Perl，JavaScript和Dart（请参见下面的异常）。

在C ++，Rust和Go之间没有语言是首选。这些语言之间的选择取决于工具的作者。

<!--
> **IDK**
>
> If a SDK that is an integration of the Fuchsia IDK includes a specific language
> (e.g. Dart), that language may be used for tools that are distributed with
> that SDK. In other words, do not include a Dart tool in a SDK that
> wouldn't otherwise include the Dart runtime, but if it's already there,
> that's okay.
-->

> **IDK**
>
> 如果作为Fuchsia IDK集成的SDK包含特定的语言（例如Dart），则该语言可以用于与该SDK一起发布的工具。换句话说，不要在SDK中包含Dart工具：否则运行时将不包含Dart，但如果已经存在，则可以。

<!--
### Style Guides

Follow the corresponding [style guide](/docs/development/languages/README.md)
for the language and area of Fuchsia being developed. For example, if the tool
is included with Zircon and written in C++, use the style guide for C++ in
Zircon. Specifically, avoid creating a separate style guide for tools.
-->

### 样式指南

按照相应的[style guide](/docs/development/languages/README.md)了解正在开发的Fuchsia的语言和区域。例如，如果该工具包含在Zircon中并用C++编写，请使用Zircon中的C++样式指南。特别是，避免为工具创建单独的样式指南。

<!--
### Runtime Link Dependencies

Try to minimize runtime link dependencies (statically link dependencies
instead). On Linux it is acceptable to runtime link against the glibc suite of
libraries (libm, etc.); other runtime link dependencies are not allowed.
-->

### 运行时链接依赖关系

尝试最小化运行时链接依赖关系（改为静态链接依赖关系）。在Linux上，运行时链接到glibc库套件（libm等）是可以接受的；不允许其他运行时链接依赖项。

<!--
### Building from Source

Keep in mind that some developers will want to build the tools from source. Use
the same build and dependency structure as the code in the Platform Source Tree.
Do not make a separate system to build tools.
-->

### 从源代码构建

请记住，一些开发人员将希望从源代码构建工具。使用与平台源代码树中的代码相同的构建和依赖关系结构。
不要建立单独的系统来构建工具。

<!--
## Host Platforms

Keep an eye on how resource heavy a tool becomes and what OSes it will be
expected to operate on.
-->

## 主机平台

密切关注工具的资源消耗量以及预期将在哪些操作系统上运行。

<!--
### Run on a Variety of Hardware

Developer machines may range from a few CPU cores and moderate amount of RAM to
dozens of CPU cores and huge amounts of RAM. Don't assume that host machines are
very powerful or that a server cluster is available to offload work to.
-->

### 在各种硬件上运行

开发人员的机器可能只有几个CPU内核和中等数量的RAM，也可能有几十个CPU内核和大量的RAM。不要假设主机非常强大，或者服务器集群会将工作分担给它们。

<!--
### Supported OSes

This section is for the convenience of the reader. This document is not
authoritative on which platforms are supported.

We currently support

- Linux
- macOS

Tools written for developers must run on those platforms. There are other
platforms to consider, and while these are not required at this time, it's good
to keep the platforms listed below in mind.

Tools should be built in a way that makes them easy to port to the following
platforms:

- Fuchsia (self-hosted)
- Windows

This is not an exhaustive list, we may support others.
-->

### 支持的操作系统

本节是为了方便读者。 本文档对在哪些平台上受支持不具有权威性。

我们目前支持

- Linux
- macOS

为开发人员编写的工具必须运行在这些平台上。还需要考虑其他平台，虽然目前不需要这些平台，但最好记住下面列出的平台。

工具的构建方式应使其易于移植到以下平台：

- Fuchsia (自托管)
- Windows

这不是一个详尽的清单，我们会尽量支持其他的设备。

<!--
### Case Insensitive File Systems

Don't rely on case sensitivity in file paths. E.g. don't expect that `src/BUILD`
and `src/build` are different files. Conversely, don't rely on case
insensitivity since some platforms are case sensitive.
-->

### 不区分大小写的文件系统

不要依赖于文件路径中的区分大小写。例如，不要认为 `src/BUILD`和 `src/build` 是不同的文件。相反，不要依赖于不区分大小写，因为有些平台是区分大小写的。

<!--
### Development Hosts Using a Non-English Locale

There are several aspects to consider for non-English developers:

- Whether the tool itself can be localized
- Whether the documentation for the tool can be localized
- Whether the tool can work with path names and data that include non-ASCII
- Whether the tool works correctly on non-English OSes

Tools are provided in US English. It's not required that a tool be localized.
(This may change in the future.)

The documentation for a tool will support non-ASCII characters. Both HTML and
Markdown can support Unicode (UTF-8) characters, so these are both good choices
for documentation. Doing the translation is not required, merely allow for the
possibility.

Tools will function properly with file paths that contain binary sequences and
white space. Use a library to work with file paths rather than manipulating
paths as strings. (e.g. path.Join in Go.)

Tools will operate correctly on non-English platforms (e.g. Japanese or French).
This means handling binary (e.g. UTF-8) data without corrupting it. E.g. don't
assume a text file is just ASCII characters.
-->

### 使用非英语语言环境的开发主机

对于非英语开发人员，需要考虑以下几个方面：

- 工具本身是否可以本地化
- 该工具的文档是否可以本地化
- 该工具是否可以使用包含非ASCII的路径名和数据
- 该工具在非英语操作系统上是否正常运行

工具以美式英语提供。 不需要将工具本地化。 （将来可能会改变。）

该工具的文档应将支持非ASCII字符。 HTML和Markdown都可以支持Unicode（UTF-8）字符，因此它们都是文档的不错选择。 使翻译不是必须的成为可能。

工具将正确处理包含二进制序列和空格的文件路径。 使用库来处理文件路径，而不是将路径作为字符串来处理。 （例如，path.Join in Go。）

工具可以在非英语平台（例如日语或法语）上正常运行。 这意味着在不破坏数据的情况下处理二进制（例如UTF-8）数据。 例如。 不要假设文本文件只是ASCII字符。

<!--
## Execution

At runtime (or execution time) consider how the tool should behave.
-->

## 执行

在运行时(或执行时)考虑工具应该如何运行。

<!--
### Optimize for No Work Needed

When appropriate, such as with a build tool, have the tool exit quickly if there
is no work to do. If possible, go one step better by providing information to
the caller about the dependencies so that the caller can accurately determine
whether the tool needs to be called at all.
-->

### 优化无需工作

在适当的时候（例如，使用构建工具），如果没有任何工作要做，请使该工具快速退出。 如果可能，通过向调用者提供有关依赖项的信息，使调用者更好地迈出第一步，以使调用者可以准确地确定是否需要调用该工具。

<!--
### Command Line Arguments

There are three types of command line arguments:

- exact text
- arguments
- options (i.e. switches and keys)
-->

### 命令行参数

命令行参数有三种类型

- 详细的描述
- 参数
- 选项（即开关和按键）

<!--
#### Exact text

Exact text is placed as-is on the command line. A piece of exact text may be
required or optional. Parsing exact text arguments should be restricted to cases
where they are needed for disambiguation (i.e. for correctly parsing other
arguments). For example if a `copy` command accepted multiple source and
destination arguments, an exact text argument may be used to clarify which is
which: `copy a b c` may be ambiguous; while `copy a to b c` may indicate that
'`a`' is copied to two destinations.
-->

#### 详细的描述

详细的描述按原样放置在命令行上。 一段准确的文本可以是必需的也可以是可选的。解析详尽的描述自变量应仅限于歧义消除（即正确解析其他自变量）所需的情况。例如，如果 `copy` 命令接受了多个源和目标参数可以使用确切的文本参数来说明是哪个：`copy a b c`可能有歧义； 而 `copy a to b c` 可表示'`a`' 已复制到两个目标地址。

<!--
#### Arguments

Arguments are like function parameters or slots for data to fit into. Often,
their order matters. In the example `copy <from> <destination>`, both `<from>`
and `<destination>` are ordered arguments. In cases where a single logical
argument is repeated the order may not matter, such as remove `<files>...` where
the tool might process the `<files>` in an arbitrary order.
-->

#### 参数

实参类似于函数参数或用于容纳数据的槽。通常，他们的顺序很重要。例如在 `copy <from> <destination>`中 `<from>` 和 `<destination>` 都是有序参数。在重复一个逻辑参数的情况下，顺序可能没有关系，再例如 remove `<files>...` ，工具可能以任意顺序处理 `<files>` 。

<!--
#### Options

Some arguments are known as options. Both switches and keyed (key/value pairs)
are options. Options tend to modify the behavior of the tool or how the tool
processes parameter arguments. Options consist of a dash prefixed letter or
word.

Options must start with either one ('`-`') or two ('`--`') dashes followed by an
alphanumeric label. In the case of a single dash, the length of the label must
be 1. If the length of the label is two or more, then two dashes must be used.
For example: `-v` or `--help` are correct; `-help` is not valid.

For option names with more than one word (for example, "foo bar"),
you must use a single dash ('`-`') between words. For example, "foo bar"
becomes `--foo-bar`.

All choices are required to have a (`--`) option. Providing single character
shorthand (`-`) is optional. E.g. it's okay to provide just `--output`, or both
`-o` and `--output`, but it's not ok to only provide an `-o` option without a
long option as well.

Do not create numeric options, such as `-1` or `-2`. E.g. rather than having
`-1` mean to do something once, add a `--once` option. If a numeric value is
needed, make a keyed option, like `--repeat <number>`.

One (`-`) or two (`--`) dashes on their own are special cases and are not
allowed as a key or switch.
-->

#### 选项

有些参数称为选项。开关和键控(键/值对)都是选项。选项倾向于修改工具的行为或工具处理参数参数的方式。选项由一个以破折号为前缀的字母或单词组成。

选项必须以一个 ('`-`') 或两个 ('`--`') 破折号开头，后跟一个字母数字标签。 如果是单个破折号，则标签的长度必须为1。如果标签的长度为两个或更多，则必须使用两个破折号。 例如： `-v` 或 `--help` 是正确的，而 `-help` 则是无效的。

对于带有多个单词的选项名称（例如， "foo bar"），必须在单词之间使用一个短划线 ('`-`') 。 例如 "foo bar"变为 `--foo-bar`。

所有选项都必须有 (`--`) 选项。提供单字符简写 (`-`) 是可选的。例如，只提供 `--output`或同时提供 `-o` 和 `--output`都可以，但不可以只提供 `-o` 选项而不提供长选项。

不要创建数字选项，例如 `-1` 或 `-2`。例如，与其让 `-1` 表示做某事一次，不如添加一个 `--once` 选项。如果需要一个数字值，创建一个关键选项，如 `--repeat <number>`。

单独的一个 (`-`) 或两个 (`--`) 破折号是特殊情况，不允许用作键或开关。

<!--
#### Switches

The presence of a switch means the feature it represents is 'on' while its
absence means that it is 'off'. Switches default to 'off'. Unlike keyed options,
a switch does not accept a value. E.g. `-v` is a common switch meaning verbose;
it doesn't take a value, making it switch rather than a keyed value.

All switches must be documented (hidden switches are not allowed).

Running switches together is not allowed. E.g. `-xzf` or `-vv`, each must be
separate: "`-x -z -f`" or "`-v -v`".
-->

#### 开关

开关的存在表示其表示的功能处于“打开”状态，而开关的缺失表示其处于“关闭”状态。 将默认设置切换为“关闭”。 与键选项不同，开关不接受值。 例如。 `-v` 是一个普通的开关，意思是冗长； 它不需要值，因此可以切换而不是键值。
所有开关都必须记录（不允许使用隐藏开关）。

不允许同时运行开关。 例如。 `-xzf` 或 `-vv`，每个必须分开 "`-x -z -f`" 或 "`-v -v`"。

<!--
#### Keyed Options

Keyed options consist of a key and a value. Keys are similar in syntax to
switches except that a keyed option expects a value for the key.
E.g. `-o <my_output_file>` has a key '`-o`' and a value of '`my_output_file`'.

Do not use an equals punctuation (or similar) to separate the key and value.
E.g. do not do `-o=<my_output_file>`.

Note about a rare case: Avoid making optional keys (where the value appears
without its key) or optional values (where the key appears without its
value). It's clearer to consider the key/value pair optional, but inseparable.
I.e. if the key is present a value is required and vice versa. Consider making
an argument instead of a keyed option with an optional key. E.g. rather than
"`do-something [--config [<config_file>]]`" where not passing `[<config_file>]`
means don't use a config file; instead do
"`do-something [--config <config_file>|--no-config]`" where passing
`--no-config` means don't load a config file.
-->

#### 键控选项

键控选项由一个键和一个值组成。 键在语法上与开关相似，不同之处在于键控选项需要键的值。
例如。 `-o <my_output_file>` 具有键 '`-o`' 和 '`my_output_file`'值。

请勿使用等号标点（或类似标点）来分隔键和值。
例如。 不要执行 `-o=<my_output_file>`。

请注意以下几种罕见情况：避免使用可选键（其中的值显示时没有键）或可选值（键中的显示时没有值）。 将键/值对视为可选但不可分割的更清楚些。
也就是说，如果键出现，则需要一个值，反之亦然。考虑使用一个参数代替一个带可选键的键控选项。 例如，与其使用"`do-something [--config [<config_file>]]`" 倒不如使用 `[<config_file>]` 来表示不要使用配置文件; 传递 "`do-something [--config <config_file>|--no-config]`" 则表示在不加载配置文件的地方使用 `--no-config` 。

##### Mutually Exclusive Options

Some options don't make sense with other options. We call the options mutually
exclusive.

Passing mutually exclusive options is considered a user error. When this occurs
the tool will do one of the following:

- Write an error message explaining the issue and exit with a non-zero result
  code; doing no work (i.e. there was no data changed as a result of the call).
  This is the expected handling, so no further documentation or notes are
  required.
- Prioritize one option over another. E.g. "`passing -z will override -y`". In
  this case the handling will be documented in the `--help` output.
- Other handling is possible (first takes precedence or last takes precedence or
  something else) though this is discouraged. In this case the handling will
  be documented in the Description, Options, ***and*** Notes; though
  "`See Notes`" may be used in Description and Options with the full write-up in
  `Notes`.

##### Grouping Options

There is no specific syntax to indicate when enabling one option will also
affect another option. When an option implies that another option is enabled or
disabled, specify that in the Options. E.g. "`passing -e implies -f`" means that
if `-e` is enabled, `-f` will be enabled as if it were passed on the command
line (regardless of whether `-f` was explicitly passed). The redundant passing
of the implied value is harmless (not an error).

##### Option Delimiter

Two dashes ('`--`') on their own indicates the end of argument options. All
subsequent values are given to the tool as-is. For example, with
"`Usage: foo [-a] <file>`", the command line "`foo -- -a`" may interpret `-a` as
a file name rather than a switch. Further, "`foo -a -- -a`" enables the switch
`-a` (the first `-a`, before the `--`) and passes the literal text `-a` (the
second `-a`).

##### Repeating Options

Repeating switches may be used to apply more emphasis (what more emphasis means
is up to the tool, the description here is intentionally vague). A common
example is increasing verbosity by passing more `-v` switches.

Repeating keyed options may be used to pass multiple values to the same command.
Often this is done to avoid calling the same command multiple times. Common
commands that accept repeating options are `cp`, `rm`, `cat`. Care must be taken
to ensure that repeating commands are unambiguous and clear. E.g. `cp` always
interprets the last argument as the destination; if `cp` accepted multiple
source and destination arguments the parsing would become ambiguous or unclear.

#### Standard Input Alias

In Fuchsia tools a single dash (`-`) is not interpreted as an alias to stdin. Use
pipes to direct data into stdin or use `/dev/stdin` as an alias for stdin.
(Note: `/dev/stdin` is not available on Fuchsia or Windows).

#### Single Dash

A single dash ('-') on its own is reserved for future use.

#### Subcommands

Tools may contain sub-command that accept independent command line arguments.
(Similar to the `git` tool). Subcommands do not begin with any dashes. E.g. in
`fx build` the `build` argument is a subcommand.

When a tool has many subcommands, it should also have a help subcommand that
display help about other subcommands. E.g. "`fx help build`" will provide help
on the build subcommand.

Subcommands may have their own arguments that are not handled by the main tool.
Arguments between the tool name and the subcommand are handled by the tool and
arguments that follow the subcommand are handled by the subcommand. E.g. in
`fx -a build -b` the `-a` is an argument for the `fx` tool, while the `-b`
argument is handled by the `build` subcommand.

### Common Features

Command line tools are expected to support some common switches:

- `--help`
- `--quiet`
- `--verbose`
- `--version`

#### Interactive Help (--help)

A tool must accept a `--help` switch and provide usage information to the
command line in that case. The layout and syntax of the help text is described
in [CLI tool help requirements](/docs/concepts/api/cli_help.md).

The tool must not do other work (i.e. have side effects) when displaying help.

Use a library that can parse the arguments as well as present help information
from the same source. Doing so keeps the two in sync. I.e. avoid writing command
line help as an independent paragraph of text.

Keep the interactive help reasonably concise. Plan for a skilled reader, i.e.
someone looking for a reminder on how to use the tool or a developer experienced
in reading interactive help. For the novice, provide a note referring them to
the Markdown documentation.

Provide an option to generate machine parsable output.

#### Verbosity (--quiet and --verbose)

The `--quiet` and `--verbose` switches decrease or increase informational output
to the user. Their implementation is optional, but all tools will accept them as
arguments and must not use those terms for other purposes, e.g. don't use
`--quiet` to turn off the audio output (use `--silence` or `--volume 0` or some
other synonym).

#### Interactive Version (--version)

A tool must accept a `--version` switch and provide an indication of the code
used to build the tool in that case. The layout and syntax is not specified, but
the version will include a version number of some kind.

The tool must not do other work (have side effects) when reporting its version.

### Logging

Logging is distinct from normal output. The audience for logging is normally the
tool developer or a power user trying to debug an issue. Logging may go to
stdout in special cases, such as when `--verbose` output is requested.

Logging from multiple threads will not interlace words within a line, i.e. the
minimum unit of output is a full text line. Each line will be prefixed with an
indication of the severity of the line. The severity will be one of: detail,
info, warning, error, fatal.

## Metrics

Every tool must file a Privacy Design Document (PDD) in order to collect usage
metrics.

Metrics are important to drive quality and business decisions. Questions we want
to answer with metrics include:

- Which OS are our users using? - so we know how to prioritize work for various
  platforms
- Which tools are they using? - so we know how to prioritize investments, and to
  learn which workflows are currently being used so we can prioritize
  investments or identify weak spots
- How often do they use a tool? - so we know how to prioritize investments, and
  to learn which workflows are currently being used so we can prioritize
  investments or identify weak spots
- Do our tools crash in the wild? How often? - so we know how to prioritize
  maintenance of tools
- How do they use a tool? - assuming that a tool can do one or more things, we'd
  like to learn how to prioritize investments in particular workflows of a tool

The type and content of the metrics collected must be carefully chosen. We will
go through the Google-standard PDD review process to ensure we are compliant
with Google's practices and policies. Tools must get approval on which metrics
are collected before collection.

## Configuration and Environment

Tools often need to know something about the context they are running. Let's
look at how that context should be gathered or stored.

#### Reading Information

Tools should not attempt to gather or intuit settings or other state directly
from the environment. Information such as an attached target's IP address, the
out directory for build products, or a directory for writing temporary files
will be gathered from a platform independent source. Separating out the code that
performs platform-specific work will allow tools to remain portable between
disparate platforms.

Where practical, configuration information should be stored in a way familiar to
the user of the host machine (e.g. on Windows, use the registry). Tools should
gather information from SDK files or platform-specific tools that encapsulate
the work of reading from the Windows registry, Linux environment, or Mac
settings.

Tools will be unbiased towards any build system or environment as well.
Accessing a common file such as build input dependency file is okay.

#### Writing Information

Tools will not modify configuration or environment settings, except when the
tool is clearly for the purpose of modifying an expected portion of the
environment.

If modifying the environment outside of the tool's normal scope may help the
user, the tool may do so with the express permission of the user.


## Execution Success and Failure

Command line tools return an integer value in the range [0..127] when they exit.
A zero represents success (no error) and 1-127 are various forms of error. The
value 1 is used as a general error. Any values other than 0 and 1 that may be
returned must be documented for the user.

### Succeed with Grace

If there were no errors encountered, return a result code of zero.

Avoid producing unnecessary output on success. Don't print "succeeded" (unless
the user is asking for verbose output).

### If Something is Unclear, Stop

If the tool encounters an ambiguous situation or is in danger of corrupting
data, do not continue. E.g. if the path to the directory you're being asked to
delete comes back as just "`/`", there was likely an error trying to get that
configuration information, avoid 'soldiering on' and removing everything under
"`/`".

### Do Not Fail Silently

Tools must clearly indicate failure by returning a non-zero error code. If
appropriate (if it makes sense for the tool or if the user explicitly asked for
verbose output) print an error message explaining what went wrong.

### Provide Direction on Failure

When a tool execution fails, be clear about whether the error came from bad
inputs, missing dependencies, or bugs within the tool. Make error reports
comprehensible and actionable.

If the error came from bad inputs

1. If the user gave the tool bad data, give context about the error and guide
   the user toward fixing the input, for example, by printing the input file
   (and line number if that's appropriate for the input) where the input error occurred.
   - Prefer output that follows this format (for easy regex use):
     `file_name:line:column:description`. This is a common format used by many
     tools. Other formats are acceptable, but try to use something that is easy
     for both humans and tools to parse.
2. Provide a reference to further information. If documentation is
   available, provide a link to documentation about the tool in general or to
   documentation regarding the specific error. If the tool has the capacity to
   provide more details, describe that (like how `gn` can explain how to run the
   tool to get more help).

If the error came from missing dependencies

1. Be clear that the error is from missing dependencies. Don't leave the
   user trying to debug their input data if that is not the issue.
2. Provide instruction on how to satisfy the dependencies. This can be an
   example command to run (`apt-get install foo`) or a link to further
   instructions (`see: http:example.com/how-to-install-foo`).

If the error came from an unexpected state (i.e. a bug) in the tool

1. Apologize. Explain that the tool got into an unexpected state. Don't leave
   the user trying to guess whether their input data was bad or they were
   missing dependencies.
2. Suggest a mailing list or forum to get help. Help the user find out if the
   bug is fixed in the next tool version; or someone has found a workaround.
3. Invite the user to enter a bug report and make that as easy as possible.
   Provide a link that goes to the bug database with the tool and platform
   information prepopulated.


## Include Tests

Tools must include tests that guarantee its correct behavior. Include both unit
tests and integration tests with each tool. Tests will run in Fuchsia continuous
integration.

> **IDK**
>
> It's especially important that IDK tools imported from the Fuchsia build (pm,
> etc.) have tests that run in Fuchsia continuous integration because the IDK
> bot does not currently prevent breaking changes.

> **ffx**
> The `ffx` platform provides a framework for introducing tests that are
> run automatically in Fuchsia continuous integration. Contributors can
> see examples of plugin tests and end-to-end self-tests in the `ffx`
> [source](/src/developer/ffx).

## Documentation

The Markdown documentation is the right place to put more verbose usage examples
and explanations.

> **IDK**
>
> All tools included in the IDK and intended to be executed directly by an end
> user must have a corresponding Markdown documentation file.

## User vs. Programmatic Interaction

A tool may be run interactively by a human user or programmatically via a script
(or other tool).

While each tool will default to interactive or non-interactive mode if they can
glean which is sensible, they must also accept explicit instruction to run in a
given mode (e.g. allow the user to execute the programmatic interface even if
they are running in an interactive shell).

### Stdin

For tools that are not normally interactive, avoid requesting user input
e.g. readline or linenoise). Don't suddenly put up an unexpected prompt to
ask the user a question.

For interactive tools (e.g. `zxdb`) prompting the user for input is expected.

### Stdout

When sending output to the user on stdout use proper spelling, grammar, and
avoid unusual abbreviations. If an unusual abbreviation is used, be sure it has
an entry in the [glossary.md](/docs/glossary.md).

Try to check for output to terminal, i.e. see if a user is there or whether the
receiver is a program.

#### ANSI Color

Use of color is allowed with the following caveats

- Enabling/disabling color output based on terminal information (i.e. whether it
  supports color) is encouraged, but that's not always possible (so it's not
  required)
  - Always allow the user to override color use (they can disable it)
- When using color, be sure to use colors that are distinct for readers who may
  not be able to see a full range of color (e.g. color-blindness).
- Never rely on color to convey information. Only use color as an enhancement.
  Seeing the color must not be needed for correct interpretation of the output.

### Stderr

Use stderr for reporting invalid operation (diagnostic output) i.e. when the
tool is misbehaving. If the tool's purpose is to report issues (like a linter,
where the tool is not failing) output those results to stdout instead of stderr.

See Success and Failure for more information on reporting errors.

### Full-Screen

Avoid creating full-screen terminal applications. Use a GUI application for such
a tool.

### Non-interactive (Programmatic)

Include a programmatic interface where reasonable to allow for automation.

If there is an existing protocol for that domain, try to follow suit (or have a
good reason not to). Otherwise consider using manifest or JSON files for
machine input.

### IDE (Semi-Programmatic)

Allow for tools to be used by an Integrated Development Environment. This
generally involves accepting a manifest for input and generating a manifest.

### Interactive (User)

Interacting with the user while the tool is running is an uncommon case for many
tools. Some tools may run interactively as an option, e.g. `rm -i` will prompt
the user before each removal.

## State Files

State files encode information for data sharing between tools. PID file and lock
files are examples of state files.

Avoid using a PID file to contain the process ID of a running executable.

Avoid using a lock file to manage mutual exclusion of resource access (i.e. a
mutex).
