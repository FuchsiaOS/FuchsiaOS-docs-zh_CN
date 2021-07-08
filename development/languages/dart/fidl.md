# FIDL

<!-- [FIDL targets][fidl] generate implicit Dart bindings targets. To use the
bindings generated for: -->

[FIDL 目标][fidl] 生成隐式 Dart 绑定目标。生成的绑定使用方法：

```
//foo/bar
//foo/bar:blah
```

<!-- add a dependencies in BUILD.gn: -->

在 BUILD.gn 中添加依赖：

```
deps = [
   ...
   "//foo/bar",
   "//foo/bar:blah",
   ...
]
```

<!-- There are 3 files generated for dart from FIDL.  These are found in
`out/default/dartlang/gen/<path-to-target>/<fidl-servicename>_package/lib` -->

FIDL 为 dart 生成了 3 个文件。你可以在 `out/default/dartlang/gen/<path-to-target>/<fidl-servicename>_package/lib` 找到。

<!-- * fidl.dart - the synchronous bindings
* fidl_async.dart - the asynchronous bindings
* fidl_test.dart - the stubbed out implementation of the service. -->

* fidl.dart - 同步绑定
* fidl_async.dart - 异步绑定
* fidl_test.dart - 被剔除的服务实现。

```dart
import "package:fidl_foo_bar/fidl.dart";
import "package:fidl_foo_bar_blah/fidl_async.dart";
```

<!-- ## Known issues -->

## 已知的问题

<!-- ### Multiple FIDL targets in a single BUILD file -->

### 在一个 BUILD 文件里编写多个 FIDL 目标

<!-- If two FIDL targets coexist in a single BUILD file: -->

如果两个 FIDL 目标共存于一个 BUILD 文件中：

<!-- * Their respective, generated files will currently be placed in the same
  subdirectory of the output directory.  This means that files belonging to one
  target will be available to clients of the other target, and this will likely
  confuse the analyzer.  This should not be a build issue now but could become
  one once the generated Dart files are placed in separate directories if
  clients do not correctly set up their dependencies.
* Depending on one of these targets from *another* FIDL target that is used by
  a Dart package leads to a `Unable to read Dart source ...` error. The
  bindings generator for FIDL builds Dart package names based on the directory
  structure containing the included FIDL file, while GN (used to compute
  dependencies for the Dart package) does so using the full GN target name. For
  example: depending on `lib/foo/fidl:bar` generates a package like
  `lib.foo.fidl._bar`. Depending on the top-level target `lib/foo/fidl`
  generates the package `lib.foo.fidl`, which coincides with the Dart FIDL
  binding's assumptions. -->

* 它们各自生成的文件当前将放置在输出目录的同一子目录中。
  这意味着属于一个目标的文件将可供另一目标的客户端使用，这可能会混淆分析器。
  现在这不应该是一个构建问题，但如果客户端没有正确设置它们的依赖关系，一旦生成的 Dart 文件被放置在单独的目录中，就会成为一个问题。
* 根据 Dart 包使用的 *另一个* FIDL 目标中的这些目标之一，会导致“无法读取 Dart 源……”错误。
  FIDL 的绑定生成器基于包含包含的 FIDL 文件的目录结构构建 Dart 包名称，而 GN（用于计算 Dart 包的依赖关系）使用完整的 GN 目标名称来实现。
  例如：依赖于 `lib/foo/fidl:bar` 生成一个像 `lib.foo.fidl._bar` 这样的包。
  根据顶级目标 `lib/foo/fidl` 生成包 `lib.foo.fidl`，这与 Dart FIDL 绑定的假设一致。

<!-- ## Calling a FIDL service -->

## 调用 FIDL 服务

<!-- The generated bindings for Dart require the importing of fuchsia_services. -->

Dart 生成的绑定需要导入 fuchsia_services。

```dart
import 'package:fuchsia_services/services.dart';
```

<!-- In order to use the Launcher service to start services that implement a FIDL interface,
you need to have the `fuchsia.sys.Launcher` service declared in the .cmx -->

为了使用 Launcher 服务启动实现 FIDL 接口的服务，您需要在 .cmx 文件中声明 `fuchsia.sys.Launcher` 服务。

[fidl]: /build/fidl/fidl.gni "FIDL"
