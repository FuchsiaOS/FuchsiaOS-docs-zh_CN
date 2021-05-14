命名 C/C++ 对象
====================

## 包含路径

有四类头文件：系统、全局、库和实现。

#### 系统头文件

```
#include <zircon/foo/bar.h>
```

###### 定义

此类头文件定义内核和用户空间之间的接口，也称为 vDSO 接口。这类头文件定义了系统调用，包括相关的类型和结构。这类头文件也定义了一些基本的 C 和 C++ 机制，例如，以明确定义的顺序崩溃。

###### 说明

- 系统头文件可能安装在 `zircon/` 下，而不是 `lib/zircon/` 下。
- 系统调用包装器（如 `zx`）不被视为系统头文件。它们是依赖于系统头文件的库文件（见下文）。
- 标准系统头文件（例如，来自 C 和 C++ 标准库的头文件）具有其标准路径。

###### 示例

- `#include <zircon/process.h>`
- `#include <zircon/syscalls/hypervisor.h>`
- `#include <stdint.h>`
- `#include <algorithm>`

#### 全局头文件

```
#include <fuchsia/foo/bar.h>
```

###### 定义

此类头文件定义了用户空间组件之间的系统范围内的约定。该类头文件是根据这些约定的 FIDL 定义生成的。

###### Notes

- 手动编写的代码应该显示在库头文件中，而不是全局头文件中。

###### 示例

- `#include <fuchsia/sys/cpp/fidl.h>`
- `#include <fuchsia/sysmem/llcpp/fidl.h>`

#### 库头文件

```
#include <lib/foo/bar.h>
```

###### 定义

库头文件是用于应用程序而手动编写的代码。库头文件定义的接口是应用程序的本地接口。一些库是 Fuchsia 特有的，并为一些较低级别的系统设施提供人机工程包装器。有些库可能与 Fuchsia 没有直接关系。

###### 说明

- 所有的库头文件都在 `lib/` 目录中，以避免与应用程序使用的其他头文件冲突。
- 库头文件不能直接放在 `lib/` 下，必须存在子目录（`lib/foo/`）。

###### 示例

- `#include <lib/fit/function.h>`
- `#include <lib/sys/cpp/component_context.h>`
- `#include <lib/zx/event.h>`

#### 实现头文件

```
#include "src/foo/bar.h"
```

###### 定义

实现头文件位于 Fuchsia 平台源代码树的内部。它们从未包含在 SDK 中，引用时使用从源代码树根目录开始的绝对路径。

###### 说明

- 包含实现头文件使用 `"`（不是 `<` ）来表示路径相对于源代码树的根目录。

###### 示例

- `#include "src/ui/scenic/bin/app.h"`
- `#include "src/lib/fxl/observer_list.h"`
