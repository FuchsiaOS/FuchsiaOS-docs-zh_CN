# FIDL


<!--
[FIDL targets][fidl] generate implicit Dart bindings targets. To use the
bindings generated for:
```
//foo/bar
//foo/bar:blah
```
add a dependency on:
```
//foo/bar:bar_dart
//foo/bar:blah_dart
```
and import the resulting Dart sources with:
```
import "package:foo.bar/baz.dart";
import "package:foo.bar..blah/baz.dart";
```
-->
[FIDL 对象][fidl] 生成间接的 Dart 绑定对象。使用给：
```
//foo/bar
//foo/bar:blah
```
生成的绑定，需要添加依赖：
```
//foo/bar:bar_dart
//foo/bar:blah_dart
```
并导入相关的 Dart 源代码：
```
import "package:foo.bar/baz.dart";
import "package:foo.bar..blah/baz.dart";
```

<!-- ## Known issues -->
## 已知问题

<!-- ### Multiple FIDL targets in a single BUILD file -->
### 单个 BUILD 文件中的多个 FIDL 对象

<!--
If two FIDL targets coexist in a single BUILD file:

* their respective, generated files will currently be placed in the same
  subdirectory of the output directory.  This means that files belonging to one
  target will be available to clients of the other target, and this will likely
  confuse the analyzer.  This should not be a build issue now but could become
  one once the generated Dart files are placed in separate directories if
  clients do not correctly set up their dependencies.
* depending on one of these targets from *another* FIDL target that is used by
  a Dart package leads to a `Unable to read Dart source ...` error. The
  bindings generator for FIDL builds Dart package names based on the directory
  structure containing the included FIDL file, while GN (used to compute
  dependencies for the Dart package) does so using the full GN target name. For
  example: depending on `lib/foo/fidl:bar` generates a package like
  `lib.foo.fidl._bar`. Depending on the top-level target `lib/foo/fidl`
  generates the package `lib.foo.fidl`, which coincides with the Dart FIDL
  binding's assumptions.
  -->
如果两个 FIDL 对象同时存在于一个 BUILD　文件中：

* 目前，它们分别生成出的文件位于输出目录的同一个子目录。这意味着属于一个对象的文件对于另一个对象的客户端也是可见的，使得分析器不理解。当生成的 Dart 文件因依赖关系未被正确设置而出现在不同的目录中时这将成为一个编译问题。
* 这些对象作为另一个被 Dart 包使用的 FIDL 对象的依赖会导致 `Unable to read Dart source ...` 错误。FIDL 的绑定生成器根据含有被导入的 FIDL 文件的目录结构对 Dart 包命名。而 GN（曾被用来处理 Dart 包的依赖关系）使用完整的 GN 对象名解决此问题。 例如：依赖于 `lib/foo/fidl:bar` 会生成一个类似于 `lib.foo.fidl._bar` 的包。依赖于最高级对象 `lib/foo/fidl` 会生成和 Dart 的 FIDL 绑定假设重合的包 `lib.foo.fidl`。


[fidl]: https://fuchsia.googlesource.com/build/+/master/fidl/fidl.gni "FIDL"
