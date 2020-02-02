<!-- # Analysis -->
# 分析


<!-- Analysis is run as part of the Fuchsia build. -->

分析作为编译 Fuchsia 的一部分运行。

<!--
For each `dart_library` target, an analysis script gets
also generated in the output directory under:
```sh
out/<build-type>/gen/path/to/package/package.analyzer.sh
```
Running this script will perform an analysis of the target's sources.
Note that other templates usually define a Dart library they build upon. For
example, a `flutter_app` `//foo/bar` will yield a `//foo/bar:bar_dart_library`
target which can also be analyzed.
-->

每一个 `dart_library` 对象都有一个对应的分析脚本被生成在输出目录中。
```sh
out/<build-type>/gen/path/to/package/package.analyzer.sh
```
此脚本对该对象的源代码进行分析。
注意其他模板通常声明一个它们编译时所依赖的 Dart 例如一个 `flutter_app` `//foo/bar` 会生成一个可分析的对象 `//foo/bar:bar_dart_library`。

<!--Dart
As with standard Dart packages, analysis options are defined in an
`analysis_options.yaml` file, which must be placed at the package root.
This file may refer to a common set of options by way of an `include` directive:
```
include: relative/path/to/options.file
```
A canonical set is available at `//topaz/tools/analysis_options.yaml`.
It is customary to merely include that set from a local options file:
```
include: path/to/topaz/tools/analysis_options.yaml
```
-->

标准 Dart 包的分析选项在根目录的 `analysis_options.yaml` 定义。
此文件通过 `include` 指令的方式控制一系列常用的选项。
```
include: relative/path/to/options.file
```
在 `//topaz/tools/analysis_options.yaml` 中有一系列典型的选项。
习惯上通过本地文件来定义选项：
```
include: path/to/topaz/tools/analysis_options.yaml
```

<!--
Analysis may be disabled altogether for a given target with:
```
dart_library("foo") {
  disable_analysis = true
}
```
-->

对于一个指定对象，可以这样彻底关闭分析：
```
dart_library("foo") {
  disable_analysis = true
}
```

<!--
The `//scripts/run-dart-action.py` script makes it easy to run the analysis over
multiple targets:
```sh
scripts/run-dart-action.py analyze --out out/<build-type> --tree //topaz/shell/*
```

Regular analyzer flags may also be passed:
```sh
scripts/run-dart-action.py analyze --out out/<build-type> --fatal-warnings --lints
```
This holds true for the individual analysis scripts.
-->

脚本 `//scripts/run-dart-action.py` 简化了对多个对象的分析。
```sh
scripts/run-dart-action.py analyze --out out/<build-type> --tree //topaz/shell/*
```

可以传入常用的分析器参数：
```sh
scripts/run-dart-action.py analyze --out out/<build-type> --fatal-warnings --lints
```
这适用于单独的分析脚本。