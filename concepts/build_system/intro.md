<!-- 
# Introduction to GN
 -->
# GN 介绍

<!-- 
This is an introduction to GN's terms and way of thinking. This should be
sufficient background to get your bearings in GN and how it's used in Fuchsia.
GN (and the Fuchsia build) are more complicated than the below will discuss, but
the average developer will not need to understand most of it on a deeper level.

The GN documentation pages [QuickStart] and [Language] give more detailed
background on GN, and [Reference] has the full language documentation.  Use
the `gn help` command to print out the reference interactively for individual
topics.  [Ninja] has its own documentation as well.

In the Fuchsia checkout after running `jiri update`, the commands
`fx gn` and `fx ninja` provide access to the prebuilt binaries.
 -->
本文是 GN 的术语和思维方式的介绍。本文提供的背景知识足以使您了解 GN 以及它是如何在 Fuchsia 中使用的。GN（和 Fuchsia 构建）本身比下面将要讨论的内容更加复杂，但其多数内容普通开发者并不需要在更深层次了解。

GN 文档页面[快速开始][QuickStart]和[语言][Language]给出了 GN 更加详细的背景知识，[参考手册][Reference]拥有完整的语言文档。使用 `gn help` 命令以交互性地打印独立主题的参考资料。[ninja][Ninja] 也拥有其单独的文档。

在运行 `jiri update` 后的 Fuchsia 检查中，命令 `fx gn` 和 `fx ninja` 提供了对预构建二进制文件的访问。

[Ninja]: https://ninja-build.org/manual.html
[QuickStart]: https://gn.googlesource.com/gn/+/HEAD/docs/quick_start.md
[Language]: https://gn.googlesource.com/gn/+/HEAD/docs/language.md
[Reference]: https://gn.googlesource.com/gn/+/HEAD/docs/reference.md

<!-- 
## Two-phase operation: `gn` and `ninja`
 -->
## 两步操作：`gn` 和 `ninja`

<!-- 
Unlike `make`, `gn` is only ever half the story.  It's in the name: GN stands
for Generate [Ninja].  There's a division of responsibilities between the
tools that corresponds to a separation of running the build into two steps:
 -->
不同于 `make`，`gn` 仅完成了工作的一半。从名字里即有所体现：GN 的全称为 Generate [Ninja]（生成 Ninja）。工具间职责的分配与两步运行构建的分割相一致：

<!-- 
1. `gn gen` takes all the configuration choices and makes all the decisions.
   All it really does is generate the `.ninja` files in the build directory.
   This step only has to be done by hand when you change the configuration or
   completely nuke the build directory.  In general, it only needs to be done
   when the GN files change, and in incremental builds it happens
   automatically if the GN files or configuration change.

1. `ninja` runs the commands to compile and link, etc.  It handles incremental
   builds and parallelism.  This is the step you do every time you've changed
   a source file, like running `make`.  GN automatically emits rules to
   re-generate the Ninja files by running `gn gen` again when a relevant
   `BUILD.gn` file (or some other relevant files) has changed, so for most
   changes after the first time you've built, `ninja` does it all.
 -->
1. `gn gen` 接受所有配置选项，并作出所有决定。它是即所做的工作是在构建目录下生成 `.ninja` 文件。除非您更改配置或完全取消构建目录，其他情况下均无需手动进行这一步。总而言之，仅当 GN 文件改动时才需要执行此操作。在增量构建时，如果 GN 文件或配置改动，该操作会自动进行。
1. `ninja` 运行命令进行编译和链接等操作。它处理增量构建和并行性。这一步是您每当改变源文件时都要做的，比如运行了 `make`。当相关的 `BUILD.gn` 文件发生改变后，GN 将通过再次运行 `gn gen` 来自动给出重新生成 ninja 文件的规则，因此对于大多数改动，在您首次构建后，`ninja` 就完成了所有工作。

<!-- 
Ninja is very simple compared to something like GNU `make`.  It just compares
times and runs commands and its input files are written by machines, not
humans.  However, it builds in some useful things that we bend over backward
to accomplish in `make`:
 -->
相比于类似 GNU `make` 的命令，ninja 非常简单。它仅仅比较时间并运行命令，它的输入文件由机器编写，而非人类。然而，它内置的一些功能，我们在 `make` 中要费九牛二虎之力才能实现。

<!-- 
 - Rebuild each file when the command line changes.  Command lines will only
   really change when GN runs again.  But after that, Ninja is smart about
   incremental builds re-doing commands for files that have changed and not
   re-running commands that haven't changed.
 - Handle compiler-generated dependency files.  Ninja knows about the makefile
   subset that compilers emit in `.d` files and consumes them directly when
   directed to by GN.
 - Run with `-j$(getconf _NPROCESSORS_ONLN)` by default.  You can pass `-j1`
   to serialize or `-j1024` when using Goma, but out of the box it does the
   parallelism you usually want.
 - Prevent interleaved `stdout`/`stderr` output from parallel jobs.  Ninja
   buffers the output so that error messages don't get garbled by spew from
   multiple processes.
 - Support terse/verbose command output.  By default, Ninja emits short
   `Kbuild`-style messages for each command it runs, in a wordy-progress-meter
   style.  The -v switch is like V=1 in `Kbuild`, to show each actual command.
 -->
 - 当命令行更改时重建每个文件。仅当 GN 重新运行时，命令行才会真正改变。但接下来，ninja 聪明地针对已更改文件执行增量构建重新执行命令，而不针对未更改的文件。
 - 处理编译器生成的依赖文件。ninja 知道 makefile 子集由编译器在 `.d` 文件中给出，并会在被 GN 定向时直接使用它们。
 - 默认带 `-j$(getconf _NPROCESSORS_ONLN)` 运行。您可以在使用 Goma 时传递（pass）`-j1` 或 `-j1024` 以进行序列化，但它可以开箱即用，实现您所需要的并行性。
 - 防止并行作业中交错的 `stdout`/`stderr` 输出。ninja 对输出进行缓冲，以免错误信息不会从多个进程大量涌出而导致错乱。
 - 支持简洁/啰嗦模式的命令输出。默认情况下，ninja 会以啰嗦进度表样式为其运行的每条命令给出简短的 `Kbuild` 风格消息。-v 开关相当于 `Kbuild`中的 V=1，用来显示每条实际命令。

<!-- 
GN was developed as part of the Chromium project to replace older build
systems.  Fuchsia inherited it from them, and it is now used across the tree as
the primary build system.
 -->
GN 作为 Chromium 项目的一部分，目的是取代旧版系统。Fuchsia 从它们中继承了 GN。现在 GN 在各个项目中被用作主要的构建系统。

<!-- 
## Build directories and `args.gn`
 -->
## 构建目录和 `args.gn`

<!-- 
Ninja always runs in the build directory.  All commands Ninja runs are run from
the root of the build directory.  The common thing is `ninja -C build-dir`.

Neither GN nor Ninja cares what build directory you use.  It's common practice
to use a subdirectory of the source directory, and since file paths are
usually rebased to be relative to the build directory, the file names given to
the compiler will have a whole lot of `../` in them if you put your build
directory elsewhere; but it should work.  It's long been common practice in
Chromium (predating GN itself) to use `out/_something_` in the source
directory, and Fuchsia inherited that default.  But nothing cares what build
directory names you choose, though the `out` subdirectory is in the top-level
`.gitignore` file for Fuchsia.
 -->
ninja 总是在构建目录中运行。ninja 的所有命令都从构建目录的根目录运行。通常的情况是 `ninja -C build-dir`。

GN 和 ninja 都不关注您所使用的构建目录是什么。惯常做法是使用源目录的子目录，并且由于文件路径通常被重新定位（rebase）为构建目录的相对路径，因此如果您将您的构建路径置于别处，传给编译器的文件名中将会包含大量的 `../`；不过它应该依然运作。Chromium（早于 GN 本身）中长期以来的惯例是在源目录使用 `out/_something_`，Fuchsia 继承了这一默认行为。但是您所选择的目录名称不会受到关注，尽管 `out` 子目录列入了 Fuchsia 的顶层 `.gitignore` 文件。

<!-- 
The basic command is `gn gen build-dir`.  This creates `build-dir/` if needed,
and populates it with Ninja files for the current configuration.  If
`build-dir/args.gn` exists, then `gn gen` will read that file to set GN build
arguments (see below).  `args.gn` is a file in GN syntax that can assign values
to GN build arguments that override any hardcoded defaults.  This means just
repeating `gn gen build-dir` preserves what you did last time.
 -->
`gn gen build-dir` 是基本命令。它在必要时创建 `build-dir`，并对当前配置导入 ninja 文件。如果 `build-dir/args.gn` 存在，那么 `gn gen` 将读取该文件以设置 GN 构建参数（见下）。`arg.gn` 是 GN 语法文件，可以向 GN 构建参数分配值，且会覆盖所有硬编码默认值。这意味着仅需重复执行 `gn gen build-dir` 即可保留您上次的操作。

<!-- 
You can also add `--args=...` to gn gen or use the `gn args` command to
configure your build arguments.  The `gn args` command gives you a way to run
your $EDITOR on the `args.gn` file, and upon exiting the editor the command
will re-run `gn gen` for you with the new arguments.  You can also just edit
`args.gn` any time, and the next Ninja run will re-generate the build files.

Args can also be set using the `fx set` command, which invokes `gn gen`. For
example to set `foxtrot` to ' `true` via `fx set`:
 -->
您也可以通过向 `gn gen` 添加 `--args=...` 或使用 `gn args` 命令来培植您的构建参数。`gn args` 命令为您提供了一种方式来运行您的 `$EDITOR` 以编辑 `args.gn`，并在退出编辑器时，为您带着新参数再次运行 `gn gen` 命令。您也可以随时编辑 `args.gn`，下次 ninja 运行将会重新生成构建文件。

args 也可以用 `fx set` 命令设定，这将调用 `gn gen`。例如利用 `fx set` 设置 `foxtrot` 为 `true`：

```sh
$ fx set <your configuration> --args 'foxtrot = true'
```

<!-- 
See [GN Build Arguments](/docs/gen/build_arguments.md), for details.
 -->
参阅 [GN 构建参数](/docs/gen/build_arguments.md)以获取细节。

<!-- 
## GN syntax and formatting
 -->
## GN 语法和格式化

<!-- 
GN syntax is whitespace-insensitive. `x=1 y=2` is the same as:
 -->
GN 语法是空白不敏感的。`x=1 y=2` 与下面的写法相同：

```gn
x = 1
y = 2
```

<!-- 
However, there is *one true indentation and formatting style* for GN code.  The
`gn format` command reformats syntactically valid GN code into the canonical
style.  There is editor syntax support for Emacs and Vim.  Canonical formatting
will be enforced by Tricium and mass reformatting will be done.  If you don't
like the formatting, file bugs or make a change in upstream GN and if it lands
we'll mass reformat everyone to conform to the new one truth.
 -->
然而，GN 代码有*规定的缩进和格式化风格*（one true indentation and formatting style）。`gn format` 命令将有效 GN 代码语法上重新格式化为规范风格。Emacs 和 Vim 具有语法支持。规范格式化将由 Tricium 强制进行，且会完成集中重新格式化工作。如果您不喜欢这种格式化，那么您可以提交错误（file bugs）或对上游 GN 进行改动，如果成功，我们将对所有人进行集中格式化以符合新规定。

<!-- 
## Source paths and GN labels
 -->
## 源路径和 GN 标签

<!-- 
GN uses POSIX-style paths (always in represented as strings) both for files and
to refer to GN-defined entities.  Paths can be relative, which means relative
to the directory containing the `BUILD.gn` file where the path string appears.
They can also be "source-absolute", meaning relative to the root of the source
tree.  Source-absolute paths begin with `//` in GN.
 -->
GN 使用 POSIX 风格路径（path）（总以字符串表示），它们既用于文件，也用于提及 GN 定义的实体。路径可以是相对的，即路径的表示是相对于包含 `BUILD.gn` 文件目录的。他们也可以是“绝对于源的（source-absolute）”，即相对于源根目录。绝对于源的路径在 GN 中以 `//` 开头。

<!-- 
When source paths are eventually used in commands, they are translated into
OS-appropriate paths that are either absolute or relative to the build
directory (where commands run).

Predefined variables are used in source path contexts to locate parts of the
build directory:
 -->
当最终在命令中使用源路径时，它们会转换为对应于操作系统的（OS-appropriate）路径，这些路径是绝对的或相对于构建目录（运行命令的位置）的路径。

预定义的变量用于在源路径上下文中定位构建目录的部分：

<!-- 
 - `$root_build_dir` is the build directory itself
 - `$root_out_dir` is the subdirectory for the current toolchain (see below)
   - This is where all "top-level" targets go.  In many GN builds, all
     executables and libraries go here.
 - `$target_out_dir` is the subdirectory of `$root_out_dir` for files built by
   targets in the current `BUILD.gn` file.  This is where the object files go.
 - `$target_gen_dir` is a corresponding place recommended to put generated code
 - `$root_gen_dir` is a place for generated code needed outside this
   subdirectory
 -->
 - `$root_build_dir` 是构建目录本身
 - `$root_out_dir` 是针对当前工具链的子文件夹（见下）
   - 这是所有“顶层”目标的去向。在许多 GN 构建中，所有可执行文件和库都在这里。
 - `$target_out_dir` 是 `$root_out_dir` 的子文件夹，针对由当前 `BUILD.gn` 内标签构建的文件。这是目标文件的去向。
 - `$target_gen_dir` 是推荐的用于存放生成代码的相应位置。
 - `$root_gen_dir` 是存放该子文件夹外所需生成代码的位置。

<!-- 
GN labels are how we refer to things defined in a `BUILD.gn` file.  They are
based on source paths, and always appear inside GN strings.  The full syntax of
a GN label is `"dir:name"` where the `dir` part is a source path that names the
particular `BUILD.gn` file.  The `name` refers to a target defined in that file
with `target_type("name") { ... }`.  As a shorthand, you can define a target
with the same name as its directory.  The label `"//path/to/dir"` with no `:`
part is a shorthand for `"//path/to/dir:dir"`.  This is the most common case.
 -->
GN 标签是我们引用在 `BUILD.gn` 文件中定义的内容的方式。它们基于源路径，并且总是出现在 GN 字符串之内。GN 标签的完整语法是 `"dir:name"`，其中 `dir` 部分是命名了特定 `BUILD.gn` 文件的源路径。`name` 指在该文件中使用 `target_type("name") { ... }` 定义的目标。简而言之，您可以定义一个名称与其所在目录名称相同的目标。无 `:` 部分的标签 `"//path/to/dir"` 是 `"//path/to/dir:dir"` 的略写。这是最常见的情况。

<!-- 
## Dependency graph and `BUILD.gn` files
 -->
## 依赖图和 `BUILD.gn` 文件

<!-- 
Everything in GN is rooted in the dependency graph.  There is one root
`BUILD.gn` file.  The only way other `BUILD.gn` files are even read is if there
is a dependency on a label in that directory.

There are no wildcards.  Every target must be named as a dependency of some
other target to get built.  You can give individual targets on the `ninja`
command line to explicitly get them built.  Otherwise they must be in the graph
from the `//:default` target (named `default` in the root `BUILD.gn` file).
 -->
GN 的一切内容都根植于其依赖图（dependency graph）中。现有一个根目录文件 `BUILD.gn`。其他 `BUILD.gn` 文件被哪怕是读取的唯一情况，就是有对于该目录下标签的依赖。

没有例外。每个目标必须被命名为其他某个目标的依赖才能被构建。您可以在 `ninja` 命令行中指定单个目标以显式地构建它们。否则它们在图中一定来自于 `//:default` 目标（位于根目录文件 `BUILD.gn` 中，命名为 `default` ）。

<!-- 
There is a generic meta-target type called `group()` that doesn't correspond to
a file produced by the build but is rather a way to structure your dependency
graph nicely.  Top-level targets like `default` are usually groups.  You can
have a group for all the drivers for a piece of hardware, a group for all the
binaries in a use case, etc.

When some code uses something at runtime (a data file, another executable,
etc.)  but doesn't use it as a direct input at build time, that file belongs in
the `data_deps` list of target that uses it.  That will also be enough to get
the thing into the BOOTFS image at its appointed place.
 -->
有一种通用元目标（meta-target）类型称为 `group()`（组），它与构建生成的文件不对应，但却是一种很好地构造您依赖图的方式。顶层目标诸如 `default` 通常都是组（group）。您可以为一款硬件的所有驱动创建一个组，也可以为一个使用场景的所有二进制文件创建一个组，等等。

当某些代码在运行时使用某个文件（一个数据文件、另一个可执行文件等）而不将其作为构建时期的直接输入时，该文件属于使用它的目标的 `data_deps` 列表。这也足以使其被装入 BOOTFS 镜像的指定位置。

<!-- 
Targets can also be labeled with `testonly = true` to indicate that the target
contains tests. GN prevents targets that are not `testonly` from depending on
targets that are, allowing for some level of control over where test binaries
end up.

Building image files is driven from one or more `zbi()` targets.  This will
make a ZBI by building and using the ZBI host tool. Targets can be placed in
this image by existing within its dependency graph, and so you can give it
dependencies on the kernel and any drivers or executables you want in the
image.
 -->
目标也可以被标记 `testonly = true` 以表明目标包含测试。GN 防止非 `testonly` 目标依赖 `testonly` 目标，从而可以对测试二进制文件的最终位置进行一定程度的控制。

镜像文件的构建由一个或多个 `zbi()` 目标驱动。这将通过构建和使用 ZBI 主机工具（ZBI host tool）制作一个 ZBI 文件。目标可以通过存在于镜像的依赖图中的方式而被置于镜像中，这样一来，您就可以为其赋予对于内核和您所希望的镜像中任何驱动或可执行文件的依赖了。

<!-- 
Note that getting targets defined in Ninja files is at the granularity of
`BUILD.gn` files, though the dependency graph from default or any other target
is at the granularity of an individual target.  So having some target in the
`BUILD.gn` file in the graph from default makes all targets in that file (and
toolchain, see below) available as targets on the Ninja command line even
though they are not built by default.
 -->
注意，获取 ninja 文件定义的目标是 `BUILD.gn` 粒度的，尽管来自默认或其他任何目标的依赖图时单个目标粒度的。因此，将一些在 `BUILD.gn` 中的目标置于图的默认值中，使得该文件中的所有目标即使在默认未被构建的情况下，也作为 ninja 命令行的目标而可用。

<!-- 
## More Advanced Concepts
 -->
## 高级概念

<!-- 
### GN expression language and GN scopes
 -->
### GN 表达式语言和 GN 作用域

<!-- 
GN is a simple, dynamically-typed, imperative language whose sole purpose at
the end of the day is to produce declarative Ninja rules.  Everything revolves
around scopes, which is both the lexical binding construct of the language and
a data type.
 -->
GN 是简单的动态类型的命令式语言，其最终目的只是产生声明性的 ninja 规则。一切都围绕作用域决定，它既是该语言的词法绑定（lexical binding，即静态绑定）结构，也是数据类型。

<!-- 
GN values can take any of several types:

 - Boolean, either `true` or `false`
 - Integer, signed with normal decimal syntax; not used much
 - String, always in "double-quotes" (note below about `$` expansion)
 - Scope, in curly braces:  `{ ... }`; see below.
 - List of values, in square brackets: `[ 1, true, "foo", { x=1 y=2 } ]` is a
   list of four elements.
 -->
GN 值可以使用下列几种类型的任何一种：

 - 布尔型（boolean），或 `true` 或 `false`
 - 整型（integer），带符号，使用普通十进制语法；不常用
 - 字符串（string），总是使用"双引号"引住（注意下面关于 `$` 的扩展）
 - 域（scope），使用花括号括住 `{ ... }`；见下。
 - 值列表（list of values），使用方括号括住：`[ 1, true, "foo", { x=1 y=2 } ]`是一个四元素列表。

<!-- 
Values are dynamically-typed and there is no kind of implicit type coercion,
but there is never type-checking as such.  Values of different types never
compare as equal, but it's not an error to compare them.

String literals expand simple `$var` or `${var}` expressions inside the
double-quotes.  This is an immediate expansion: `x${var}y` is the same as `x +
var + y` when var is a string.  In this way, any value can be rendered as a
pretty-printed string.
 -->
值是动态类型的，因而没有隐式类型的强迫，但也就没有这样的类型检查。不同类型的值比较结果永不相等，但是比较它们并不是错误。

字符串字面值在双引号中将简单的 `$var` 或 `${var}` 表达式扩展。这是一种立即扩展（immediate expansion）：当 `var` 为字符串时，`x${var}y` 与 `x +
var + y` 相同。这样，任何值都可以表示为打印美观的字符串。

<!-- 
Identifiers made up of alphanumerics and underscores can populate a scope via
assignment operators.  Imperative assignment with `=` and modification via `+=`
are really all the GN language does (there are also some special ways to have
side effects like `print()`, used for debugging; and `write_file()`, used
sparingly).

Each file is internally represented as a scope, and there is no global scope.
Shared "globals" can be defined in a `.gni` file and imported where they are
used (`import("//path/to/something.gni")`).  Each `.gni file is processed once
per toolchain (see below for information about toolchains), and the resulting
scope is copied into the importing file scope.

Target declarations introduce a sub-scope:
 -->
字母、数字和下划线组成的标志符可以通过赋值运算符填充一个域。GN 语言所做的实际上就是使用 `=` 进行命令式赋值，并通过 `+=` 进行修改（也有一些特殊的方式来产生副作用，如 `print()`，用于调试；又如 `write_file()`，谨慎使用）。

每个文件在内部都表示为一个域，并且没有全局域。共享“全局域”可以定义在 `.gni` 文件中，并在它们被使用的地方导入（`import("//path/to/something.gni")`）。每个 `.gni` 文件在每个工具链（toolchain）中处理一次（见下以获取关于工具链的信息），然后结果域被复制到导入文件的域中。

目标的声明引入了一个子域：

<!-- 
```gn
foo = true
executable("target") {
  foo = 12
}
# Outside the target, foo == true
```
 -->
```gn
foo = true
executable("target") {
  foo = 12
}
# 目标之外，foo == true
```

<!-- 
GN is very strict in diagnosing errors when a variable is defined but never
used within a scope.  The scope inside a target acts like a keyword argument
list for the target with checking that the argument names were spelled
correctly.  The target-defining code can also use `assert()` to diagnose an
error if a required argument was omitted.

A value can also be a scope.  Then it's acting like a struct when you use it:
`value.member`.  But a scope is always a block of GN code that executes to
yield its set of names and values:
 -->
当一个变量在域中被定义而未被使用时，GN 对于错误的诊断非常严格。目标内部的作用域就像目标的关键字参数列表一样，它检查参数名称是否正确拼写。如果必需的参数被忽略，那么目标定义代码也可以使用“ assert（）”来诊断错误。

一个值可以是一个域。那么当你使用它的时候，它就如同一个结构体：`value.member`。但是域总是一个 GN 代码块，它的执行用来产生其名称和值的集合：

```gn
foo = {
  x = global_tuning + 42
  If (some_global && other_thing == "foobar") {
    y = 2
  }
}
```

<!-- 
This always defines `foo.x` but only sometimes defines `foo.y`.
 -->
这总是会定义 `foo.x` ，但仅有时会定义 `foo.y`。

<!-- 
### GN toolchains
 -->
GN 工具链

<!-- 
GN has a concept called a "toolchain".  This will all be happening behind the
scenes and developers shouldn't need to deal with it directly, but it helps
to understand the mechanism.
 -->
GN 有一个称为“工具链（toolchain）”的概念。这一切将在后台发生，开发者不需要直接处理它，但是这有助于理解该机制。

<!-- 
This is what encapsulates the compilers and default compilation switches.  It's
also the only real way to get the same things compiled twice in different
ways. In Fuchsia there will be several toolchains:
 -->
它是对编译器和默认编译选项的概括。它也是真正将相同内容以不同方式变异两次的唯一方法。在 Fuchsia 中将有几种工具链：

<!-- 
 - Host
 - Vanilla userland (compiled with default `-fPIE`)
 - Shared libraries in userland (compiled with `-fPIC`)
 - `userboot`
 - Kernel
 - Kernel physical-address mode for ARM64 (compiled with `-mstrict-align`)
 - Multiboot for x86 (compiled with `-m32`)
 - UEFI for Gigaboot
 - Toolchains are also used in the ["variants"
   scheme](/docs/gen/build_arguments.md#known_variants) that is how we allow selectively
   enabling ASan or the like for parts of userland.
 -->
 - 主机（host）
 - Vanilla 用户区（Vanilla userland）（使用默认值 `-fPIE` 编译）
 - 用户区中的共享库（shared libraries in userland）（使用 `-fPIC` 编译）
 - `userboot`
 - 内核（kernel）
 - 面向 ARM64 的物理地址模式内核（Kernel physical-address mode for ARM64）（使用 `-mstrict-align` 编译）
 - 面向 x86 的多重引导（Multiboot for x86）（使用 `-m32` 编译）
 - 面向 Gigaboot 的 UEFI（UEFI for Gigaboot）
 - 工具链也在[“变体”方案](/docs/gen/build_arguments.md#known_variants)中使用，这是我们允许选择性地为部分用户区启用 ASan 或类似功能的方式。

<!-- 
Each toolchain is identified by a GN label.  The full syntax for target labels
is actually `//path/to/dir:name(//path/to/toolchain/label)`.  Usually the
toolchain is omitted and this is expanded to `label($current_toolchain)`,
i.e. label references are usually within the same toolchain.
 -->
每个工具链由一个 GN 标签标志。完整的目标标签语法实际上是 `//path/to/dir:name(//path/to/toolchain/label)`。通常会忽略工具链，并将其扩展为 `label($ current_toolchain)`，即标签引用通常位于同一工具链中。

<!-- 
All the GN files are instantiated separately in each toolchain.  Each toolchain
can set global variables differently, so GN code can use tests like `if
(is_kernel)` or `if (current_toolchain == some_toolchain)` to behave
differently in different contexts.  This way the GN code stays with the source
it describes, but it can still do different subsets of shared sources for
kernel and user, etc.
 -->
所有 GN 文件在每个工具链中分别实例化。每个工具链可以设置不同的全局变量，因此 GN 代码可以使用诸如 `if (is_kernel)` 或 `if (current_toolchain == some_toolchain)` 的测试以在不同上下文中产生不同行为。这样，GN 代码与其描述的的源保持一致，却又可以为内核和用户等做共享源的不同子集。
