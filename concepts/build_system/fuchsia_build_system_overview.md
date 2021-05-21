<!-- 
# The Fuchsia build system
 -->
# Fuchsia 构建系统

<!-- 
## Overview
 -->
## 概述

<!-- 
The Fuchsia build system aims at building both boot images and updatable
packages for various devices. To do so, it uses [GN][gn-main], a meta-build
system that generates build files consumed by [Ninja][ninja-main], which
executes the actual build.
 -->
Fuchsia 构建系统旨在为各种设备构建引导映像和可更新的软件包。 为此，它使用 [GN][gn-main]，这是一种元构建系统（meta-build system），该系统生成构建文件供 [Ninja][ninja-main] 使用，而 Ninja 执行实际的构建。

<!-- 
Note that Zircon uses a different build system, though still using GN and
Ninja.
 -->
注意，Zircon 使用不同的构建系统，尽管也使用了 GN 和 Ninja。

<!-- 
## Getting started
 -->
## 准备开始

<!-- 
If you are unfamiliar with Fuchsia's build system and GN, see [Using GN
build][gn-preso], which outlines the basic principles of the GN build system.
 -->
如果您对 Fuchsia 的构建系统和 GN 不熟悉，请参阅 [使用 GN 构建][gn-preso]，该文章概述了 GN 构建系统的基本原则。

<!-- 
## Boards and Products
 -->
## 板型和产品

<!-- 
The contents of the generated image are controlled by a combination of a
board and a product that are the minimal starting configuration of a Fuchsia
build. Boards and products define dependency sets that define the packages
that are included in images, updates, and package repositories.
[boards and products](boards_and_products.md) documents the structure and
usage of these build configurations.
 -->
生成镜像的内容由一组板型（board）和产品（product）控制，它们是 Fuchsia 构建的最小起始配置。板型和产品定义了依赖集合，其中定义了镜像中有哪些包（package）被包括在内、更新和包仓库。[板型和产品](boards_and_products.md)记录了这些构建配置的结构和使用方法。

<!-- 
## Bundles
 -->
## 套装

<!-- 
A bundle is a grouping of related packages within a part of the source tree,
such as all tools or all tests. An overview of bundles is provided in
[bundles](bundles.md). A set of top-level bundles are defined in
[`//bundles`](/bundles/README.md).
 -->
套装是源工作区一部分中相关包的分组，例如用于所有测试的所有工具。[套装](bundles.md)中提供了对于套装的概述。一组顶层套装定义在[`//bundles`](/bundles/README.md)中。

<!-- 
## Build targets
 -->
## 构建目标

<!-- 
Build targets are defined in `BUILD.gn` files scattered all over the source
tree. These files use a Python-like syntax to declare buildable objects:
 -->
构建目标定义在分散于源工作区各处的 `BUILD.gn` 文件中。这些文件使用一种类 Python 语法来声明可构建对象：

```py
import("//build/some/template.gni")

my_template("foo") {
  name = "foo"
  extra_options = "//my/foo/options"
  deps = [
    "//some/random/framework",
    "//some/other/random/framework",
  ]
}
```

<!-- 
Available commands (invoked using gn cli tool) and constructs (built-in target
declaration types) are defined in the [GN reference][gn-reference]. There are
also a handful of custom templates in `.gni` files in the
[`//build` project][build-project].
 -->
可用命令（使用 gn cli 工具调用）和结构（内置目标声明类型）定义在 [GN 参考手册]中。[`//build` 项目][build-project]里的 `.gni` 文件中也有许多自定义模板。

<!-- 
These custom templates mostly define custom target declaration types, such as
the package declaration type.
 -->
这些自定义模板大都定义了自定义目标声明类型，例如包声明类型。

> TODO(pylaligand): list available templates

<!-- 
## Executing a build
 -->
## 执行构建

<!-- 
The simplest way to this is through the `fx` tool, as described in
[fx workflows](/docs/development/build/fx.md). Read on to see
what `fx` does under the hood.

The rest of this document assumes that `gn` and `ninja` commands are
available in your `PATH`. These commands can be found in
`prebuilt/third_party/gn/<platform>` and
`prebuilt/third_party/ninja/<platform>` respectively. Alternatively, if
you want to avoid modifying your `PATH`, you can prefix all invocations
with `fx`, i.e. `fx gn` or `fx ninja`.
 -->
执行构建的最简单方式就是通过 `fx` 工具，同 [fx 工作流](/docs/development/build/fx.md)所描述的一样。继续阅读以理解 `fx` 暗中的操作。

文章的剩余部分假设 `gn` 和 `ninja` 命令在您的 `PATH` 中是可用的。这些命令可分别在 `prebuilt/third_party/gn/<platform>` 和
`prebuilt/third_party/ninja/<platform>`中找到。或者，如果您不想修改您的 `PATH`，您可以在所有调用前加上 `fx`，即 `fx gn` 或 `fx ninja`。

<!-- 
### Gen step
 -->
### 生成步骤

<!-- 
First configure the primary build artifacts by choosing the board and product
to build:
 -->
首先通过选择板型和产品配置基本构建产物：

```posix-terminal
fx gn gen $(fx get-build-dir) --args='import("//boards/x64.gni") import("//products/core.gni")'
```

<!-- 
This will create a build directory (usually `out/default`) containing Ninja
files.
 -->
这将会创建一个包含 Ninja 文件的构建目录（通常为 `out/default`）

<!-- 
The equivalent `fx set` command is:
 -->
等价的 `fx set` 命令是：

```posix-terminal
fx set core.x64
```

<!-- 
For a list of all GN build arguments, run:
 -->
对于包含所有 GN 构建参数的列表，运行：

```posix-terminal
fx gn args $(fx get-build-dir) --list
```

<!-- 
For documentation on the `select_variant` argument, see [Variants](variants.md).
 -->
要获取关于 `select_variant` 参数的文档，请参阅[变体](variants.md)。

<!-- 
### Build step
 -->
### 构建步骤

<!-- 
The next step is to run the actual build with Ninja:
 -->
下一步是使用 Ninja 运行实际构建：

```posix-terminal
fx ninja -C $(fx get-build-dir)
```

<!-- 
This is what gets run under the hood by `fx build`.
 -->
这就是 `fx build` 在暗中运行的内容。

<!-- 
## Rebuilding
 -->
## 重构建

<!-- 
In order to rebuild the tree after modifying some sources, just rerun
**Build step**. This holds true even if you modify `BUILD.gn` files as GN adds
Ninja targets to update Ninja targets if build files are changed! The same
holds true for other files used to configure the build. Any change of source
that requires a manual re-invocation of the **Gen step** is a build bug and
should be reported.
 -->
要在修改源之后重构建工作区，只需重新执行**构建步骤**。即使您修改了 `BUILD.gn` 文件，这也是可行的，因为如果构建文件发生改动，那么 GN 会添加 Ninja 目标以更新 Ninja 目标！对于其他用于配置构建的文件也是如此。任何需要手动重新调用**生成步骤**的源的改动都是构建的程序错误，应当报告。

<!-- 
## Tips and tricks
 -->
## 技巧和诀窍

<!-- 
### Inspecting the content of a GN target
 -->
### 检查 GN 目标的内容

```posix-terminal
fx gn desc $(fx get-build-dir) //path/to/my:target
```

<!-- 
### Finding references to a GN target
 -->
### 寻找对 GN 目标的引用

```posix-terminal
fx gn refs $(fx get-build-dir) //path/to/my:target
```

<!-- 
### Referencing targets for the build host
 -->
### 为构建主机引用目标

<!-- 
Various host tools (some used in the build itself) need to be built along with
the final image.

To reference a build target for the host toolchain from a module file:
 -->
许多主机工具（一些用于构建本身）需要随着最终镜像一同构建。

要为主机工具链从模块文件中引用构建目标：

```
//path/to/target(//build/toolchain:host_x64)
```

<!-- 
To reference a build target for the host toolchain from within a `BUILD.gn`
file:
 -->
要为主机工具链从 `BUILD.gn` 文件中引用构建目标：

```
//path/to/target($host_toolchain)
```

<!-- 
### Building only a specific target
 -->
### 仅构建某一特定目标

<!-- 
If a target is defined in a GN build file as `//foo/bar/blah:dash`, that target
(and its dependencies) can be built with:
 -->
如果目标在 GN 构建文件中定义为 `//foo/bar/blah:dash`，该目标（及其依赖）可以使用下面的方式构建：

```posix-terminal
fx ninja -C $(fx get-build-dir) -j64 foo/bar/blah:dash
```

<!-- 
Note that this only works for targets in the default toolchain.

Note: Building package targets does not result in an updated package
repository, because the package repository is updated by the `updates` group
target. In order for updated package changes to be made available via `fx
serve`, users must build the `updates` group.
 -->
注意，这仅对默认工具链中的目标运作。

注意：构建包目标不会导致包仓库更新，因为包仓库由 `updates` 组目标更新。要使更新的包改动通过 `fx serve` 可用，用户必须构建 `updates` 组。

<!-- 
### Exploring Ninja targets
 -->
### 浏览 Ninja 目标

<!-- 
GN extensively documents which Ninja targets it generates. The documentation is
accessible with:
 -->
GN 全面地记录了其生成的 Ninja 目标。文档可通过下面的方式访问：

```posix-terminal
fx gn help ninja_rules
```

<!-- 
You can also browse the set of Ninja targets currently defined in your output
directory with:
 -->
您也可以通过下面的方式浏览当前在您输出目录中定义的 Ninja 目标集合：

```posix-terminal
fx ninja -C $(fx get-build-dir) -t browse
```

<!-- 
Note that the presence of a Ninja target does not mean it will be built - for
that it needs to depend on the “default” target.
 -->
注意，Ninja 目标的存在并不意味着它会被构建——因为它要依赖于“默认”目标。

<!-- 
### Understanding why Ninja does what it does
 -->
### 理解 Ninja 的所作所为和个中缘由

<!-- 
Add `-d explain` to your Ninja command to have it explain every step of its
execution.
 -->
向您的 Ninja 命令添加 `-d explain` 以使其解释执行的每一个步骤。

<!-- 
### Debugging build timing issues
 -->
### 调试构建用时问题

<!-- 
When running a build, Ninja keeps logs that can be used to generate
visualizations of the build process:

1. Delete your output directory - this is to ensure the logs represent only the
   build iteration you’re about to run;
1. Run a build as you would normally do;
1. Get <https://github.com/nico/ninjatracing>;
1. Run `ninjatracing <output directory>/.ninja_log > trace.json`;
1. Load the resulting json file in Chrome in `about:tracing`.
 -->
运行构建时，Ninja 记录日志，并生成可视化构建进程：

1. 删除您的输出目录——这是为了确保日志仅反映您要将运行的构建迭代；
1. 像您平时一样运行构建；
1. 获取 <https://github.com/nico/ninjatracing>；
1. 运行 `ninjatracing <output directory>/.ninja_log > trace.json`；
1. 加载 json 结果文件至 Chrome 的 `about:tracing`。


<!-- 
## Troubleshooting
 -->
## 故障排除

<!-- 
### My GN target is not being built!
 -->
### 我的 GN 目标未构建！

<!-- 
Make sure it rolls up to a label defined in a module file, otherwise the build
system will ignore it.
 -->
确保将其是模块文件中定义的标签，否则构建系统将忽略它。

<!-- 
### GN complains about missing `sysroot`.
 -->
### GN 报告缺失 `sysroot`。

<!-- 
You likely forgot to run both commands of **Build step**.
 -->
您可能忘记了运行**构建步骤**的两条命令。

> TODO(pylaligand): command showing path to default target

<!-- 
### Internal GN setup
 -->
### 内部 GN 设置

> TODO(pylaligand): .gn, default target, GN labels insertion

[gn-main]: https://gn.googlesource.com/gn/
[gn-preso]: https://docs.google.com/presentation/d/15Zwb53JcncHfEwHpnG_PoIbbzQ3GQi_cpujYwbpcbZo/
[ninja-main]: https://ninja-build.org/
[gn-reference]: https://gn.googlesource.com/gn/+/HEAD/docs/reference.md
[build-project]: /build/
[zircon-getting-started]: /docs/zircon/getting_started.md
