<!-- # Analysis -->

# 分析

<!-- Analysis is run as part of the Fuchsia build. -->

分析的运行是 Fuchsia 构建的一部分。

<!-- For each `dart_library` target, an analysis script gets
also generated in the output directory under: -->

对于每一个 `dart_library` 目标，分析脚本也会生成到位于下面路径中的输出路径下：

```sh
out/<build-type>/gen/path/to/package/package.analyzer.sh
```

<!-- Running this script will perform an analysis of the target's sources.
Note that other templates usually define a Dart library they build upon. For
example, a _flutter_app_ `//foo/bar` will yield a `//foo/bar:bar_dart_library`
target that can also be analyzed. -->

运行此脚本将对目标的来源进行分析。
请注意，其他模板通常定义它们构建的 Dart 库。
例如，_flutter_app_ `//foo/bar` 将产生一个 `//foo/bar:bar_dart_library` 目标，也可以对其进行分析。(that can also be analyzed.)

<!-- As with standard Dart packages, analysis options are defined in an
`analysis_options.yaml` file, which must be placed at the package root.
This file may refer to a common set of options by way of an `include` directive: -->

与标准 Dart Packages 一样，分析选项在 `analysis_options.yaml` 文件中定义，该文件必须放在 package 根目录下。
该文件可以通过 `include` 指令引用一组通用选项：

```
include: relative/path/to/options.file
```

<!-- A canonical set is available at [//topaz/tools/analysis_options.yaml](https://fuchsia.googlesource.com/topaz/+/HEAD/tools/analysis_options.yaml)
It is customary to merely include that set from a local options file: -->

这是一个规范集 [//topaz/tools/analysis_options.yaml](https://fuchsia.googlesource.com/topaz/+/HEAD/tools/analysis_options.yaml)
通常只包含本地选项文件中的设置：

```
include: path/to/topaz/tools/analysis_options.yaml
```

<!-- Analysis may be disabled altogether for a given target with: -->

对于给定的目标，可以完全禁用分析：

```
dart_library("foo") {
  disable_analysis = true
}
```

<!-- The `//scripts/run-dart-action.py` script makes it easy to run the analysis over
multiple targets: -->

`//scripts/run-dart-action.py` 脚本可以轻松地在多个目标上运行分析：

```sh
scripts/run-dart-action.py analyze --out out/<build-type> --tree //topaz/shell/*
```

<!-- Regular analyzer flags may also be passed: -->

也可以通过传递常规分析器标志：

```sh
scripts/run-dart-action.py analyze --out out/<build-type> --fatal-warnings --lints
```

<!-- This holds true for the individual analysis scripts. -->

这适用于各个分析脚本。
