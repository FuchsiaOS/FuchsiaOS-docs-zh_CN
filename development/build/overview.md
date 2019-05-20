# 构建系统

## 概述

Fuchsia构建系统的设计目标，是同时为多种设备构建引导镜像和可安装软件包。为此，它使用 [GN](gn-main)，一个基于元语的构建系统，来为 [Ninja](ninja-main) 生成构建文件，再由 Ninja 执行实际构建工作。[使用 GN 构建工具](gn-preso) 是一篇对 GN 的介绍。

需要注意的是，Zircon 使用的是基于 GNU Make 的，完全不同的构建系统。
对系统其他部分的构建，都需要建立在先构建 Zircon 的基础上。

## 组件

生成的镜像中具体包含哪些内容，是由一系列顶层组件控制的。组件定义了一系列包，这些包被包含在系统引导（或更新）镜像中、预装在 paver images 中，并可以使用更新系统来安装。[组件](products.md) 解释了组件定义文件中不同域的结构和用法。

## 包

包是组件的内容，包可能包含或引用其他的包（或可用于构建的 GN labels）。详见 [包](packages.md) 页面。

## 构建目标

构建目标在 `BUILD.gn` 文件中定义，这些文件分布在源码树各处。它们使用类似 Python 的语法来声明构建目标：
``` py
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
[GN 定义](gn-reference) 中定义了可用的命令（通过 gn 命令行工具调用）和组件（多种内建目标声明类型）。[`//build` project](build-project) 中提供了大量用户模板 `.gni` 文件。

这些用户模板大多定义了用户目标声明类型，比如 [包声明类型][packages-source]。

> TODO(pylaligand): list available templates

## 执行构建

最简单的方法就是使用 `fx` 工具，详见 [快速开始](/getting_started.md#Setup-Build-Environment) 。下面介绍 `fx` 的幕后工作流程。

### A

构建 Zircon 的第一步就是使用它自带的构建系统：
```bash
$ scripts/build-zircon.sh
```

这就是在执行 `fx build-zircon` 时实际执行的指令，也在执行 `fx full-build` 的时候被执行。

要查看所有的执行选项，可以运行 `build-zircon.sh -h`。查看 Zircon 的
[快速开始][zircon-getting-started] 和
[Makefile 选项][zircon-makefile-options] 了解更多细节。

### B

下一步是通过选择要构建的顶层组件来配置要构建镜像的内容：
```
# --products and --packages can be omitted to use the defaults, which are
# $layer/products/default.gni and empty, respectively.
$ buildtools/gn gen out/x64 --args='import("//garnet/products/product_name.gni") fuchsia_packages=["garnet/packages/my_stuff"]'
```

这将创建一个包含 Ninja 文件的 `out/x64` 目录。

等价的 fx set 指令如下：
```
$ scripts/fx set x64 --products garnet/products/base.gni --packages garnet/packages/my_stuff
```

要查看所有 GN 构建参数的列表，运行 `buildtools/gn args out/x64 --list`。
要查看 `select_variant` 参数的文档，查看 [Variants](variants.md)。

### C

最后一步是使用 Ninja 执行实际构建。
```
$ buildtools/ninja -C out/<arch> -j 64
```

这就是执行 `fx build` 时的幕后工作流程。

## 二次构建

### 在改动了非 Zircon 文件之后

要在更改了一些代码后重新构建代码，只需要重新执行 **C** 步骤，这在你改动了 `BUILD.gn` 文件之后依然有效。在构建文件改动后，GN 将增加 Ninja 目标来更新 Ninja 目标。对于包的构建配置文件来说也是这样的。

### 在改动了 Zircon 文件之后

你需要重新执行 **A** 和 **C** 步骤。

### 在同步了代码之后

如果 Zircon 代码树有变动，你最好执行一次 **A** 步骤，然后再重新执行 **C** 步骤。


## 提示和技巧

###  查看一个组件中所有的包

```bash
$ build/gn/preprocess_products.py --products '["garnet/products/default"]'
```

### 可视化构建包的层级结构

```bash
$ scripts/visualize_module_tree.py > tree.dot
$ dot -Tpng tree.dot -o tree.png
```

### 查看一个 GN 构建目标的内容

```bash
$ buildtools/gn desc out/x64 //path/to/my:target
```

### 查找对一个 GN 目标的引用

```bash
$ buildtools/gn refs out/x64 //path/to/my:target
```

### 为构建主机引用目标

许多构建主机需求的工具（包括一些在构建过程中使用的）需要和最终镜像一起构建。

为主机工具链从一个模块文件设置构建目标引用：
```
//path/to/target(//build/toolchain:host_x64)
```
为主机工具链从一个 `BUILD.gn`文件设置构建目标引用：
```
//path/to/target($host_toolchain)
```

### 只构建一个指定目标

如果一个目标在 GN 构建文件中定义为 `//foo/bar/blah:dash`，这个目标（以及它的依赖）可以用如下指令构建：
```bash
$ buildtools/ninja -C out/x64 -j64 foo/bar/blah:dash
```
需要注意的是，这只对包含在默认工具链中的目标有效。

### 探索 Ninja 构建目标

GN 为它产生的 Ninja 构建目标生成了文档。该文档可以通过以下指令访问：
```bash
$ buildtools/gn help ninja_rules
```

你也可以使用以下命令浏览你的输出目录中目前定义的一系列 Ninja 构建目标：
```bash
$ buildtools/ninja -C out/x64 -t browse
```
需要注意的是，当出现的 Ninja 构建目标需要依赖于 “default” 目标时，它不会被构建。

### 理解 Ninja 的工作步骤

在你的 Ninja 指令中添加 `-d explain` 来使它解释每一步的执行。

### 调试构建时发生的问题

当执行构建时，Ninja 会记录可用于生成构建过程可视化的日志数据：

1. 删除你的输出目录，这是为了保证日志数据记录最新一次的构建过程；
2. 执行正常的构建流程；
3. 安装 <https://github.com/nico/ninjatracing>；
4. 执行 `ninjatracing <output directory>/.ninja_log > trace.json`；
5. 在 Chrome 的 `about:tracing` 中加载生成的 json 文件。


## 常见问题

### 我的 GN 构建目标没有被构建！

确保它使用了模块文件中定义的标签来声明，否则构建系统将忽略它。

### GN 报错：缺少 `sysroot`

在运行 **B** 步骤之前，你需要先运行 **A** 步骤。

> TODO(pylaligand): command showing path to default target


### 系统内的 GN 配置

> TODO(pylaligand): .gn, default target, mkbootfs, GN labels insertion

[gn-main]: https://chromium.googlesource.com/chromium/src/tools/gn/+/HEAD/README.md
[gn-preso]: https://docs.google.com/presentation/d/15Zwb53JcncHfEwHpnG_PoIbbzQ3GQi_cpujYwbpcbZo/
[ninja-main]: https://ninja-build.org/
[gn-reference]: https://gn.googlesource.com/gn/+/master/docs/reference.md
[build-project]: https://fuchsia.googlesource.com/build/+/master/
[zircon-getting-started]: https://fuchsia.googlesource.com/zircon/+/master/docs/getting_started.md
[zircon-makefile-options]: https://fuchsia.googlesource.com/zircon/+/master/docs/makefile_options.md

[packages-source]: 