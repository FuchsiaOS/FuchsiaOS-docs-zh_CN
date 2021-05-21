<!-- 
# Build system policies
 -->
# 构建系统策略

<!-- 
This document details design principles and specific technical decisions made
that relate to how the Fuchsia build should work.
These principles apply to all modes of using the Fuchsia build, for instance
whether by interactive engineering workflows or via automated systems such as
CI/CQ.
 -->
本文档详细介绍了与 Fuchsia 构建应当如何运作有关的设计原则和特定技术决策。这些原则适用于使用 Fuchsia 构建的所有模式，例如是否通过交互性工程工作流或通过例如 CI/CQ 的自动化系统的方式。

<!-- 
## Goals and priorities of the build
 -->
## 构建的目标和优先级

<!-- 
Like any system, the build is often subject to multiple conflicting
requirements. When there is a conflict, we generally seek to satisfy these
priorities by order of importance:

1. Meet customer requirements, as determined by Fuchsia technical leadership.
2. Ensure correctness: produce the desired outputs.
3. Promote maintainability: documentation, sound engineering processes.
4. Improve performance: perform the same builds at a lower cost.
 -->
与任何系统相同，该构建常常服从于多个冲突的需求。出现冲突时，我们通常追求满足这些优先事项，它们按重要性顺序排列：

1. 满足客户需求，这是由 Fuchsia 技术领导制定的。
2. 确保正确性：产生期望的输出。
3. 促进可维护性：文档、可靠的工程流程。
4. 提升性能：以更小代价执行相同构建。

<!-- 
## Desired properties of the build
 -->
## 构建的期望属性

<!-- 
The following are considered to be good properties for the build:

* Hermeticity - the build is self-contained and neither influences external
  software and configuration or is influenced by external software and
  configuration. Build steps 
* Repeatability and reproducibility - two builds from the same source tree
  produce the same output or the same outcome deterministically.
  Reproducibility promotes security and auditing, and simplifies
  troubleshooting.
* Efficient - builds should only spend time doing work relevant to the build,
  and must aim to minimize the impact on both human and infrastructure costs.
* Portability - builds should produce consistent results across all supported
  host platforms.
 -->
下列内容被认为是该构建的良好属性：

* 封闭性（hermeticity）——构建是独立的，既不会影响外部软件和配置，也不会受到外部软件和配置的影响。<!-- 构建步骤 -->
* 可重复性（repeatability）和可再现性（reproducibility）——来自同一源工作区的两个构建确定性地产生相同的输出或相同的结果。可重复性能够提高安全性和审核效率，简化故障排除。
* 高效（efficient）——构建应仅花费时间进行与构建相关的工作，并且必须力求将对人力和基础设施代价的影响降至最低。
* 可移植性（portability）——构建应在所有受支持的主机平台上产生一致的结果。

<!-- 
These are ideals.
We aim to meet these ideals and measure our progress against these measures.
 -->
这些是理想。我们致力于实现这些理想，并用它们作为量度来度量我们的进度。

<!-- 
## Python scripts as build actions
 -->
## Python 脚本用于构建行为

<!-- 
Python scripts may be used as build actions.
 -->
Python 脚本可用于构建行为（build action）。

<!-- 
Please follow the [Google style guide for Python][python-style].
 -->
请遵从 [Google Python 风格指南][python-style]。

<!-- 
Fuchsia currently uses Python 3.8. All Python sources are to begin with the
following:
 -->
Fuchsia 当前使用 Python 3.8。所有 Python 源文件都以下面的内容开头：

```shell
#!/usr/bin/env python3.8
```

<!-- 
## Shell scripts as build actions
 -->
## Shell 脚本用于构建行为

<!-- 
Shell scripts may be used as build actions.

Shell scripts are encouraged for tasks that can be expressed with a few simple
shell commands. For complex operations, other languages are preferred.

Please follow the [Google style guide for shell scripting][bash-style].
Please use [shellcheck] to find and correct common shell programming errors.
 -->
shell 脚本可以用于构建行为。

我们鼓励使用 shell 脚本执行可以用一些简单的 shell 指令表示的任务。对于复杂操作，最好使用其他语言。

请遵从 [Google shell 脚本风格指南][bash-style]。请使用 [shellcheck] 寻找和更正常见 shell 编程错误。

<!-- 
We prefer POSIX (aka Bourne) shell scripts for portability across wide set of
host platforms.
If you're maintaining an existing Bash script, please restrict the features
used to version 3.2, or consider rewriting the script as POSIX shell script.
To check whether your script is POSIX compliant, you can use:
 -->
我们更倾向于 POSIX（又名 Bourne）shell 脚本，以实现跨众多主机平台的可移植性。如果您正在维护现有 bash 脚本，请减少使用的特性至 3.2 版本，或考虑将其重写为 POSIX shell 脚本。要检查您的脚本是否与 POSIX 兼容，您可以使用：

```posix-terminal
shellcheck --shell=sh
```

<!-- 
Scripts that run on POSIX shell should begin with the following:
 -->
在 POSIX shell 上运行的脚本应当以下列内容开头：

```shell
#!/bin/sh
```

<!-- 
Scripts that specifically require Bash should begin with the following:
 -->
特定需要 bash 的脚本应当以下列内容开头：

```shell
#!/bin/bash
```

<!-- 
## Migrations
 -->
## 迁移

<!-- 
The build system can assist in performing migrations for such things as
compiler features, new tools, or proliferation of various best practices.
A legacy undesired behavior can often be expressed in terms of a dependency
on a `config()` that applies this behavior. The use of a legacy tool or
template to be replaced can be captured by a dependency on a `group()`
target.
 -->
构建系统可以协助执行诸如编译器特性、新工具或各种最佳实践激增的迁移。遗留的不良行为通常可以表示为对于应用此行为的 `config()` 的依赖。对要被替换的旧版工具或模板的使用可以通过对 `group（）` 目标的依赖来捕获。

<!-- 
### Commit to a plan
 -->
### 记录计划

<!-- 
Efforts to improve code health are always welcome, but you should have a clear
plan to finish what you started before you begin. A half-done migration that's
run out of momentum could be worse than no migration at all.
 -->
我们总是欢迎您为提升代码质量作出努力，但是您应当在开始之前对于您发起的工作有清晰的计划。动力不足、半途而废的迁移可能比完全不迁移还要糟糕。

<!-- 
### Establish a regression stop
 -->
### 建立回归终止

<!-- 
Assume that the codebase doubles every 8 months, and work as early as you can
to prevent new instances of the legacy behavior from being introduced. By
establishing a regression stop, you are "passively" cleaning up the codebase as
governed by its doubling rate, i.e. every doubling period you will have
passively cleaned up half of the codebase.

Ensure that allowlists are guarded by `OWNERS` files, and that POCs for the
migration are listed as owners. Since owners are defined by file, it may be
preferable to subdivide allowlists to different `BUILD.gn` files. For instance,
`config()` targets related to Rust were pulled out to `//build/config/rust` to
better manage the `OWNERS` assignments.
 -->
###建立回归停止

假如代码库每 8 个月翻一番，而你尽早工作以防止引入旧行为的新实例。通过建立回归停止（regression stop），您可以按其倍增率的节奏，“被动”清理代码库，也就是说，代码库每增加一倍，您将已被动清理了一半代码库。

确保 `OWNERS` 文件保护了许可列表（allowlist），并且将进行迁移的 POC 列为所有者。由于所有者是由文件定义的，因此最好将许可列表细分为不同的 `BUILD.gn` 文件。例如，与 Rust 相关的 `config()` 目标被拉至 `//build/config/rust` 中以更好地管理`OWNERS` 分配。


<!-- 
### Document migration / cleanup steps
 -->
### 文档迁移/清理步骤

<!-- 
Publish a clear document explaining the nature of the migration, how to
participate, and how to perform related maintenance work. This allows your
migration effort to scale, and keeps any individual from becoming a roadblock to
ongoing migration efforts such as when they're overwhelmed with support requests
or otherwise unavailable to attend to questions.

Review [C++ implicit conversions][wconversion-project] as a positive example.
 -->
发布清晰的文档，解释迁移的性质、如何参与以及如何进行相关维护工作。这使您的迁移工作得以扩展，并防止任何人成为正在进行的迁移工作的障碍，例如当他们受到大量支持请求的压力时或无法处理问题时。

您可以回顾范例 [C++ 隐式转换][wconversion-project]。

<!-- 
### Simplify and automate allowlist maintenance
 -->
### 简化与自动化许可列表的维护

<!-- 
Allowlists are easy to express as `visibility` lists for GN targets. This opens
the door to automated analysis, and makes changes that violate the allowlist
fail their builds quickly.

When allowlisting targets to use the legacy behavior that you're migrating away
from, make it easy for owners of those targets to make simple refactors such as
renaming individual targets within their directories by allowlisting base
directories, not individual targets.

Document the steps to regenerate and trim any allowlist, such that they can be
conducted by anyone.

See the example below:
 -->
许可列表（allowlist）很容易表示为针对 GN 标签的 `visibility`（可见性）列表。这为自动化分析开启了大门，并且使得违反许可列表的改动很快故障而无法构建。

当将目标列入许可列表以使用您要迁移的旧行为时，要使得这些目标的所有者能够轻松进行简单的重构，例如：通过将基目录（base directory）（而非单个目标）列入许可列表，而在单个目标所在的目录中对其进行重命名。

记录步骤以重新生成（regenerate）和修整（trim）任何许可列表，以便任何人都可以处理它们。

请看下面的示例：

<!-- 
```gn
group("foo_allowlist") {
  #  ________  _________  ________  ________
  # |\   ____\|\___   ___\\   __  \|\   __  \
  # \ \  \___|\|___ \  \_\ \  \|\  \ \  \|\  \
  #  \ \_____  \   \ \  \ \ \  \\\  \ \   ____\
  #   \|____|\  \   \ \  \ \ \  \\\  \ \  \___|
  #     ____\_\  \   \ \__\ \ \_______\ \__\
  #    |\_________\   \|__|  \|_______|\|__|
  #    \|_________|
  # This is an allowlist of targets that use the deprecated "foo" tool.
  # As of April 2021 we no longer use "foo". Users should migrate to the new
  # "bar" tool as described in this guide:
  # https://fuchsia.dev/...
  #
  # To regenerate:
  # fx gn refs $(fx get-build-dir) //path/to:foo_allowlist | sed 's|\(.*\):.*|"\1/*",|' | sort | uniq
  #
  # To trim:
  # scripts/gn/trim_visibility.py --target="//path/to:foo_allowlist"
  visibility = [
    "//src/project1/*",
    "//src/project2/*",
    ...
  ]
}
```
 -->
```gn
group("foo_allowlist") {
  #  ________  _________  ________  ________
  # |\   ____\|\___   ___\\   __  \|\   __  \
  # \ \  \___|\|___ \  \_\ \  \|\  \ \  \|\  \
  #  \ \_____  \   \ \  \ \ \  \\\  \ \   ____\
  #   \|____|\  \   \ \  \ \ \  \\\  \ \  \___|
  #     ____\_\  \   \ \__\ \ \_______\ \__\
  #    |\_________\   \|__|  \|_______|\|__|
  #    \|_________|
  # 这是一份使用了不推荐的“foo”工具的目标的允许列表。
  # 截至 2021 年 4 月，我们不再使用“foo”。用户应当
  # 迁移至新工具“bar”，这篇文章中对其进行了描述：
  # https://fuchsia.dev/...
  #
  # 要重新生成：
  # fx gn refs $(fx get-build-dir) //path/to:foo_allowlist | sed 's|\(.*\):.*|"\1/*",|' | sort | uniq
  #
  # 要修整：
  # scripts/gn/trim_visibility.py --target="//path/to:foo_allowlist"
  visibility = [
    "//src/project1/*",
    "//src/project2/*",
    ...
  ]
}
```

<!-- 
Then elsewhere, automatically add a dependency on the allowlisted target.
 -->
接下来在其他地方，自动添加对于该列入许可列表的目标的依赖。

<!-- 
```gn
# Invoke the legacy foo tool.
# For new usage, please consider using the new bar tool instead!
# See:
# https://fuchsia.dev/...
# ...
template("foo") {
  action(target_name) {
    ...
    deps += [ "//build/foo:foo_allowlist" ]
  }
}
```
 -->
```gn
# 调用旧 foo 工具。
# 对于新用法，请考虑转为使用新的 bar 工具！
# 参阅：
# https://fuchsia.dev/...
# ...
template("foo") {
  action(target_name) {
    ...
    deps += [ "//build/foo:foo_allowlist" ]
  }
}
```


[bash-style]: https://google.github.io/styleguide/shellguide.html
[python-style]: https://google.github.io/styleguide/pyguide.html
[shellcheck]: https://www.shellcheck.net/
[wconversion-project]: /docs/contribute/open_projects/cpp/wconversion.md
