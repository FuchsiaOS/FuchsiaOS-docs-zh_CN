<!-- # Flutter Module Development -->
# Flutter 模块开发

<!--
This directory demonstrates how you create modules with Dart and Flutter. At the
moment this document assumes that every module gets built as part of the core
fuchsia build and included in the bootfs.
-->
此目录演示如何使用 Dart 和 Flutter 创建模块。目前，这篇文档假定每一个模块都作为 Fuchsia 核心编译的一部分被编译，并被包含在 rootfs 中。

<!-- # Example Modules -->
# 示例模块

<!-- ## Hello -->
## Hello

<!--
(located in `//topaz/examples/ui/hello_mod/`)

This example demonstrates how to create a minimal flutter module and implement
the `Module` interface. It shows a simple flutter text widget displaying "hello"
on the screen.
-->
（位于 `//topaz/examples/ui/hello_mod/`）

这个例子演示了如何创建一个最小的 Flutter 模块并实现 `Module` 接口。它通过一个简单的 Flutter text widget 在屏幕上显示“Hello”。

<!-- ## Running the Examples on Fuchsia -->
## 在 Fuchsia 上运行示例

<!--
You can run an example module without going through the full-blown session shell.
The available URLs for for flutter module examples are:

*   `hello_mod`

After a successful build of fuchsia, type the following command from the zx
console to run the basemgr with the dev session shell:

```
killall scenic  # Kills all other mods.
basemgr --session_shell=dev_session_shell --session_shell_args=--root_module=hello_mod
```
-->
运行示例模块无需使用完整的会话 shell。
Flutter 模块示例可用的 URL 有：

*   `hello_mod`

成功编译 Fuchsia 后，在 zx 终端中输入以下命令来用开发会话 shell 运行 basemgr：

```
killall scenic  # Kills all other mods.
basemgr --session_shell=dev_session_shell --session_shell_args=--root_module=hello_mod
```

<!-- # Basics -->
# 基本概念

<!--
A flutter module is a flutter app which use [ModuleDriver](
https://fuchsia.googlesource.com/topaz/+/master/public/lib/app_driver/dart/lib/src/module_driver.dart)
. An example of a minimal
flutter module is available in [//topaz/examples/ui/hello_mod/](
https://fuchsia.googlesource.com/topaz/+/master/examples/ui/hello_mod/).

Below we reproduce the contents of `main()` from that example:

```dart
final ModuleDriver _driver = ModuleDriver();

void main() {
  setupLogger(name: 'Hello mod');

  _driver.start().then((ModuleDriver driver) {
      log.info('Hello mod started');
    });

  runApp(
    MaterialApp(
      title: 'Hello mod',
      home: ScopedModel<_MyModel>(
        model: _MyModel(),
        child: _MyScaffold(),
      ),
    ),
  );
}
```
-->
Flutter 模块是使用 [ModuleDriver](https://fuchsia.googlesource.com/topaz/+/master/public/lib/app_driver/dart/lib/src/module_driver.dart) 的 Flutter 应用。[//topaz/examples/ui/hello_mod/](https://fuchsia.googlesource.com/topaz/+/master/examples/ui/hello_mod/) 提供一个最小的 Flutter 模块示例。

以下是此示例中 `main()` 的内容：

```dart
final ModuleDriver _driver = ModuleDriver();

void main() {
  setupLogger(name: 'Hello mod');

  _driver.start().then((ModuleDriver driver) {
      log.info('Hello mod started');
    });

  runApp(
    MaterialApp(
      title: 'Hello mod',
      home: ScopedModel<_MyModel>(
        model: _MyModel(),
        child: _MyScaffold(),
      ),
    ),
  );
}
```

<!-- # Importing Packages -->
# 导入包

<!-- ## Adding Dependency to BUILD.gn -->
## 在 BUILD.gn 中添加依赖

<!--
To import a dart package written within the fuchsia tree, the dependency should
be added to the project's `BUILD.gn`. The `BUILD.gn` file for the hello_mod
example looks like this:

```gn
import("//topaz/runtime/flutter_runner/flutter_app.gni")

flutter_app("hello_mod") {
  main_dart = "main.dart"
  package_name = "hello_mod"
  fuchsia_package_name = "hello_mod"
  deps = [
    "//topaz/public/dart/widgets:lib.widgets",
    "//topaz/public/lib/app_driver/dart",
  ]
}
```

There are two types of dart packages we can include as `BUILD.gn` dependencies.
-->
导入 Fuchsia 树中的 Dart 包时，应在项目的 `BUILD.gn` 中添加依赖。hello_mod 示例的 `BUILD.gn` 文件如下：

```gn
import("//topaz/runtime/flutter_runner/flutter_app.gni")

flutter_app("hello_mod") {
  main_dart = "main.dart"
  package_name = "hello_mod"
  fuchsia_package_name = "hello_mod"
  deps = [
    "//topaz/public/dart/widgets:lib.widgets",
    "//topaz/public/lib/app_driver/dart",
  ]
}
```

<!-- ### 1. Normal Dart Packages -->
### 1. 普通 Dart 包

<!--
Any third-party dart packages, or regular dart packages manually written in the
fuchsia tree. Import them with their relative paths from the `<fuchsia_root>`
directory followed by two slashes. Third-party dart packages are usually located
at `//third_party/dart-pkg/pub/<package_name>`.
-->
指所有第三方 Dart 包，或在 Fuchsia 树中的一般 Dart 包。以“双斜杠+对于 `<fuchsia_root>` 目录的相对路径”的形式导入它们。第三方 Dart 包通常位于 `//third_party/dart-pkg/pub/<package_name>`。

<!-- ### 2. FIDL-Generated Dart Bindings -->
### 2. FIDL 生成的 Dart 绑定

<!--
To use any FIDL generated dart bindings, you need to first look at the
`BUILD.gn` defining the `fidl` target that contains the desired `.fidl` file.
For example, let's say we want to import and use the `module.fidl` file (located
in `//peridot/public/lib/module/fidl/`) in our dart code. We should first
look at the `BUILD.gn` file, in this case `//peridot/public/lib/BUILD.gn`. In
this file we can see that the `module.fidl` file is included in the
`fidl("fidl")` target.

```
fidl("fidl") {
  sources = [
    ...
    "module/fidl/module.fidl",   # This is the fidl we want to use for now.
    ...
  ]
}
```
-->
使用任何 FIDL 生成的 Dart 绑定之前，请查看定义了 `fidl` 对象的 `BUILD.gn`。前者应包含所需的 `.fidl` 文件。
例如，在 Dart 代码中导入并使用 `module.fidl` 文件（位于`//peridot/public/lib/module/fidl/`）。首先应查看 `BUILD.gn` 文件，这个例子中即 `//peridot/public/lib/BUILD.gn`。其中，可发现 `module.fidl` 文件在
`fidl("fidl")` 对象中被导入。

```
fidl("fidl") {
  sources = [
    ...
    "module/fidl/module.fidl",   # This is the fidl we want to use for now.
    ...
  ]
}
```

<!--
This means that we need to depend on this group of fidl files. In our module's
`BUILD.gn`, we can add the dependency with the following syntax:

`"//<dir>:<fidl_target_name>_dart"`

Once this is done, we can use all the interfaces defined in `.fidl` files
contained in this `story` fidl target from our code.
-->
这意味着我们需要这一组 fidl 文件作依赖。在此模块的 `BUILD.gn` 中，可用以下语法添加依赖：

`"//<dir>:<fidl_target_name>_dart"`

添加后，可在代码中使用所有在 `.fidl` 文件里定义的接口。此 `story` fidl 对象应包含上述 `.fidl` 文件。

<!-- ## Importing in Dart Code -->
## 在 Dart 代码中导入

<!--
Once the desired package is added as a BUILD.gn dependency, the dart files in
those packages can be imported in our dart code. Importing dart packages in
fuchsia looks a bit different than normal dart packages. Let's look at the
import statements in `main.dart` of the hello_world example.

```dart
import 'package:lib.app.dart/app.dart';
import 'package:lib.app.fidl/service_provider.fidl.dart';
import 'package:apps.modular.services.story/link.fidl.dart';
import 'package:apps.modular.services.module/module.fidl.dart';
import 'package:apps.modular.services.module/module_context.fidl.dart';
import 'package:lib.fidl.dart/bindings.dart';

import 'package:flutter/widgets.dart';
```

To import things in the fuchsia tree, we use dots (`.`) instead of slashes (`/`)
as path delimiter. For FIDL-generated dart files, we add `.dart` at the end of
the corresponding fidl file path. (e.g. `module.fidl.dart`)
-->
当所需的包作为 BUILD.gn 依赖被导入后，其中的 Dart 文件便可在 Dart 代码中导入。导入 Fuchsia 中的 Dart 包和普通 Dart 包有些不同。以下是 hello_world 示例中 `main.dart` 里的导入语句。

```dart
import 'package:lib.app.dart/app.dart';
import 'package:lib.app.fidl/service_provider.fidl.dart';
import 'package:apps.modular.services.story/link.fidl.dart';
import 'package:apps.modular.services.module/module.fidl.dart';
import 'package:apps.modular.services.module/module_context.fidl.dart';
import 'package:lib.fidl.dart/bindings.dart';

import 'package:flutter/widgets.dart';
```

导入项来自 Fuchsia 树时，路径分隔符为点（`.`）而非斜杠（`/`）。对于 FIDL 生成的 Dart 文件，在对应的 fidl 文件目录后添加 `.dart`（例如 `module.fidl.dart`）。

<!-- # Using FIDL Dart Bindings -->
# 使用 FIDL 的 Dart 绑定

参见 [FIDL 教程](../fidl/tutorial.md)。

<!-- ## Things to Watch Out For -->
## 注意事项

<!-- ### Handles Can Only Be Used Once -->
### 句柄只能使用一次

<!--
Once an `InterfaceHandle<Foo>` is bound to a proxy, the handle cannot be used in
other places. Often, in case you have to share the same service with multiple
parties (e.g. sharing the same `fuchsia::modular::Link` service across multiple
modules), the service will provide a way to obtain a duplicate handle (e.g.
`fuchsia::modular::Link::Dup()`).

You can also call `unbind()` method on `ProxyController` to get the usable
`InterfaceHandle<Foo>` back, which then can be used by someone else.
-->
一旦 `InterfaceHandle<Foo>` 和一个代理绑定，将不能在其他地方被使用。需要与多方共享同一服务时（例如将 `fuchsia::modular::Link` 服务共享给多个模块），该服务通常会提供获得重复句柄的方式。（例如 `fuchsia::modular::Link::Dup()`）。

<!-- ### Proxies and Bindings Should Be Closed Properly -->
### 应正确地关闭代理和绑定

<!--
You need to explicitly close `FooProxy` and `FooBinding` objects that are bound
to channels, when they are no longer in use. You do not need to explicitly close
`InterfaceRequest<Foo>` or `InterfaceHandle<Foo>` objects, as those objects
represent unbound channels.
-->
当与 channel 绑定的 `FooProxy` 和 `FooBinding` 对象已停用时，请显式地关闭它们。但不需要显式地关闭 `InterfaceRequest<Foo>` 或 `InterfaceHandle<Foo>` 对象，因为它们代表着未绑定的 channel。

<!--
If you don't close or unbind these objects and they get picked up by the garbage
collector, then FIDL will terminate the process and (in debug builds) log the
Dart stack for when the object was bound. The only exception to this rule is for
*static* objects that live as long as the isolate itself. The system is able to
close these objects automatically for you as part of an orderly shutdown of the
isolate.
-->
如果这些对象因未关闭或未解绑而被垃圾回收器处理掉，FIDL 将终止此进程并（在 debug build 中）对 Dart 栈进行关于对象绑定时间的日志记录。此规则的唯一例外是和隔离本身存活时间一样长的*静态*对象。系统能在对隔离进行有序关闭的过程中自动关闭它们。

<!--
If you are writing a Flutter widget, you can override the `dispose()` function
on `State` to get notified when you're no longer part of the tree. That's a
common time to close the proxies used by that object as they are often no longer
needed.
-->
当编写 Flutter widget 时，可以在 `State` 上重写 `dispose()` 函数从而在不再是树的一部分时得到通知。通常应在这时关闭该对象使用的所有代理，因为一般此刻它们不再被需要。

<!-- # Other Useful Tips -->
# 其他有用的提示

<!-- ## Getting the Atom dartlang plugin to work correctly -->
## 获取 Atom 的 dartlang 插件以正常工作

<!--
You need to have the correct `.packages` file generated for the dart packages in
fuchsia tree. After building fuchsia, run this script form the terminal of your
development machine:

```
<fuchsia_root>$ scripts/symlink-dot-packages.py
```

Also, for flutter projects, the following line should be manually added to the
`.packages` file manually (fill in the fuchsia root dir of yours):

```
sky_engine:file:///<abs_fuchsia_root>/third_party/dart-pkg/git/flutter/bin/cache/pkg/sky_engine/lib/
```

You might have to relaunch Atom to get everything working correctly. With this
`.packages` files, you get all dartanalyzer errors/warnings, jump to definition,
auto completion features.
-->
请为 Fuchsia 树中的 Dart 包生成正确的 `.packages` 文件。编译 Fuchsia 后，在开发机的终端中运行此脚本：

```
<fuchsia_root>$ scripts/symlink-dot-packages.py
```

另外对于 Flutter 项目，应在 `.packages` 文件中手动添加以下一行（填写 Fuchsia 根目录）：

```
sky_engine:file:///<abs_fuchsia_root>/third_party/dart-pkg/git/flutter/bin/cache/pkg/sky_engine/lib/
```

可能需要重新启动 Atom 来使一切正常运作。通过这个 `.packages` 文件可获得所有 Dart 分析器的错误/警告、跳转至定义和自动补全功能。