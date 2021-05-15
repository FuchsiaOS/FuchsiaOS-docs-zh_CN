# C++ 编码风格指南

Fuchsia 项目遵循知名的[谷歌 C++ 编码风格指南][google-guide]，但有一些例外。

使用 [clang-format][clang-format] 是一种很好的做法，因为它可以确保您的代码符合编码风格指南。Gerrit 中的 Tricium 检查还使用 clang-format 作为非选通的 linter。但是，只要代码符合这些准则，您仍然可以手动格式化代码。

#### 提示：待办事项注释

按照谷歌 C++ 编程风格指南，在待办事项注释中输入 bug 编号时，使用 `TODO(fxbug.dev/11111)` 格式将提供一个可用作 URL 的地址。

### 例外情况

#### 行长

Fuchsia 采用的列数为 100，而不是 80。

#### 花括号

当块的内容多于一行时，请始终使用花括号 `{ }`。这是您需要注意的事项，因为 Clang-format 不会添加块的花括号。

```cpp
// 错误写法
while (!done)
  doSomethingWithAReallyLongLine(
       wrapped_arg);

// 正确写法
while (!done) {
  doSomethingWithAReallyLongLine(
       wrapped_arg);
}
```


#### 条件和循环

请勿在圆括号内使用空格（谷歌编码风格指南不鼓励但允许这样做）。

请勿使用简短条件和循环的单行形式（谷歌编码风格指南允许这两种形式）：

```cpp
// 错误写法
if (x == kFoo) return new Foo();

// 正确写法
if (x == kFoo)
  return new Foo;
```

#### 命名空间名称

* 禁止嵌套命名空间，但以下情况除外：
  - `internal` （当需要隐藏模板化代码的实现细节时）
  - FIDL 编译器生成的代码
* 禁止使用以下顶层名称空间：
  - `internal`
  - `fuchsia` （FIDL编译器生成的代码除外）
* IDK 库中的命名空间必须保持尽可能短的列表。稍后的文档将提供允许的命名空间的显式列表；同时，应谨慎地引入新的命名空间。
* 还应考虑非 IDK 库中的命名空间，以减少冲突的风险。最好避免使用非常通用的名词（如 `media`）。

理论依据： [本周技巧 #130：命名空间命名][totw-130]

[clang-format]: https://clang.llvm.org/docs/ClangFormat.html
[google-guide]: https://google.github.io/styleguide/cppguide.html
[totw-130]: https://abseil.io/tips/130

#### 包含头文件

* 如果包含的头文件是系统头文件、全局头文件或库头文件（请参见[命名 C/C++ 对象](naming.md)以获取精确定义），请使用 `<尖括号>`和头文件的完整名称。在谷歌 C++ 编码风格指南中，这些头文件被当作“C 库头文件”：

  ```cpp
  #include <zircon/syscalls.h>           // System header
  #include <fuchsia/io/cpp/fidl.h>       // Global header
  #include <lib/fdio/fd.h>               // Library header
  ```

* 如果包含的头文件是实现头文件，请使用 `"引号"`，并使用从源代码树的根目录到该头文件的完整路径。在谷歌 C++ 编码风格指南中，这些头文件被当作“项目头文件”：

  ```cpp
  #include "src/ui/scenic/bin/app.h"     // Implementation header
  ```

* 使用根相对路径（例如 `#include "third_party/skia/include/core/SkPaint.h"`）或使用其规范头文件名称（例如 `#include <gtest/gtest.h>`）来包含第三方头文件。
