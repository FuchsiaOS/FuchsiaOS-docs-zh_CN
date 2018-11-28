<!--
# How to write benchmarks
-->

# 如何写评量基准

<!--
* Updated: 2018 August 9
-->

* 更新于: 2018年8月9日

[TOC]

<!--
## Overview
-->

## 概述

<!--
This guide will walk you through the process of writing a benchmark, running it at every
commit, and automatically tracking the results in the [Performance Dashboard].
-->

这篇指南将会带领你学习编写评量基准的过程，每次代码提交时执行，并且在[Performance Dashboard]中自动回溯查看结果。

<!--
Today we support automating benchmarks for these projects:
* Garnet (Also runs Zircon benchmarks)
* Peridot
* Topaz
-->

现在已经为如下的这些工程支持了自动测量基准性能：
* Garnet（也会执行Zircon的基准性能测量）
* Peridot
* Topaz

<!--
## Writing a benchmark
-->

## 编写评量基准

<!--
Fuchsia benchmarks are command-line executables that produce a JSON results file.  The
executable must meet the following criteria:

1. It accepts the location to the results file as a command line flag.
2. It produces JSON results that match the [benchmark results schema]:
-->

Fuchsia的基准评量是命令行可执行文件，这些程序会生成一个JSON格式的结果文件。执行文件必须满足如下的标准：

1. 程序要能将结果文件的位置作为命令行标记。
2. 程序要能够生成JSON格式结果，并且满足[基准评量结果模式]。

<!--
## Building your benchmark

Your benchmark executable should be built into a Fuchsia package.  For more information
please read the [Fuchsia package documentation].
-->

## 编译评量基准

你编写的基准评量程序应该编译进Fuchsia程序包中，更多信息请阅读[Fuchsia package documentation]

<!--
## Automating your benchmark

We have shell scripts that run all of a layer's benchmarks at every commit to that layer.

* Garnet: [//garnet/tests/benchmarks](https://fuchsia.googlesource.com/garnet/+/master/tests/benchmarks)
* Peridot: [//peridot/tests/benchmarks](https://fuchsia.googlesource.com/peridot/+/master/tests/benchmarks)
* Topaz: [//topaz/tests/benchmarks](https://fuchsia.googlesource.com/topaz/+/master/tests/benchmarks)

These shell scripts are written using a helper library called [Runbenchmarks].  Add a
command to the appropriate script to execute your test.  See the existing commands for
examples.
-->

## 自动执行你的基准评量

* Garnet: [//garnet/tests/benchmarks](https://fuchsia.googlesource.com/garnet/+/master/tests/benchmarks)
* Peridot: [//peridot/tests/benchmarks](https://fuchsia.googlesource.com/peridot/+/master/tests/benchmarks)
* Topaz: [//topaz/tests/benchmarks](https://fuchsia.googlesource.com/topaz/+/master/tests/benchmarks)

这些shell脚本是用一个叫[Runbenchmarks]的辅助库编写。在脚本中添加一行适当的代码，用来执行你的测试程序。可以参考脚本中其他的命令。

<!--
## Testing

At this point, you're ready to build Fuchsia and test that your benchmark runs
successfully. Run the following in a shell:

```sh
fx set-petal $layer
jiri update -gc
# Benchmarks are not included in production packages, so use $layer/packages/kitchen_sink
# or they will not be built.
fx set <arch> --packages $layer/packages/kitchen_sink
fx full-build && fx run
```

Once the Fuchsia shell is loaded:

```sh
# Run just your benchmark
run my_benchmark [options]

# Run all benchmarks for $layer
/pkgfs/packages/${layer}_benchmarks/0/bin/benchmarks.sh /tmp
```

If no errors occurred, you should see your benchmark's output file in `/tmp`, along with
the results files of other benchmarks.
-->

##测试

到这里，你已经准备好编译Fuchsia，并且测试你的基准测量程序能否成功执行。在shell中执行如下命令：

```sh
fx set-petal $layer
jiri update -gc
# 基准评量程序并没有包含在结果程序包中，因此要使用$layer/packages/kitchen_sink
# 否则这些程序可能不会被编译。
fx set <arch> --packages $layer/packages/kitchen_sink
fx full-build && fx run
```

一旦Fuchsia的Shell加载成功：

```sh
# 运行你的基准测试程序
run my_benchmark [options]

# 为$layer运行所有的基准评量程序
/pkgfs/packages/${layer}_benchmarks/0/bin/benchmarks.sh /tmp
```

如果没有错误出现，你就可以在`/tmp`目录查看基准评量程序的输出文件了，同时也包括其他基准评量的结果文件。

<!--
## Tracking in the performance dashboard

Please see the [Performance Dashboard User Guide]

NOTE: We do not yet have a User guide for the [Performance Dashboard Version 2].

[benchmark results schema]: results_schema.md
[Fuchsia package documentation]: /development/build/packages.md
[Performance Dashboard]: https://chromeperf.appspot.com/report
[Performance Dashboard User Guide]: catapult_user_guide.md
[Performance Dashboard Version 2]: https://v2spa-dot-chromeperf.appspot.com/
[Runbenchmarks]: https://fuchsia.googlesource.com/garnet/+/master/testing/runbenchmarks
[//zircon/system/ulib/perftest]: https://fuchsia.googlesource.com/zircon/+/master/system/ulib/perftest/
[//garnet/go/src/benchmarks]: https://fuchsia.googlesource.com/garnet/+/master/go/src/benchmarks
-->

##在性能显示面板中回溯

请参考[Performance Dashboard User Guide]文档。

注意：我们现在还没有[Performance Dashboard Version 2]的使用指南。

[benchmark results schema]: results_schema.md
[Fuchsia package documentation]: /development/build/packages.md
[Performance Dashboard]: https://chromeperf.appspot.com/report
[Performance Dashboard User Guide]: catapult_user_guide.md
[Performance Dashboard Version 2]: https://v2spa-dot-chromeperf.appspot.com/
[Runbenchmarks]: https://fuchsia.googlesource.com/garnet/+/master/testing/runbenchmarks
[//zircon/system/ulib/perftest]: https://fuchsia.googlesource.com/zircon/+/master/system/ulib/perftest/
[//garnet/go/src/benchmarks]: https://fuchsia.googlesource.com/garnet/+/master/go/src/benchmarks

