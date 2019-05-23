# 测试组件

## 创建一个测试组件

### BUILD.gn

```gn
import("//build/test/test_package.gni")

executable("my_test_bin") {
  testonly = true
  output_name = "my_test"

  sources = [
    "my_test.cc",
  ]
}

test_package("my_test_pkg") {
  deps = [
    ":my_test_bin",
  ]

  tests = [
    {
      name = "my_test"
    },
  ]
}
```

`test_package` 假设在 `meta` 文件夹中有一个对应的 cmx 文件。因此对于上述例子，在 `meta/` 中应该存在一个 `my_test.cmx` 文件。

### meta/my\_text.cmx

```json
{
    "program": {
        "binary": "test/my_test"
    },
    "sandbox": {
        "services": [...]
    }
}
```

## 运行测试

在本地计算机上运行测试有多种方式

```bash
runtests /pkgfs/packages/my_test_pkg/test/
```
这一命令将在你提供的路径下执行所有测试。

```bash
run_test_component my_test
```

这一命令将搜索 `my_text.cmx` 文件，如果找到则运行它。

```bash
run_test_component fuchsia-pkg://fuchsia.com/my_test_pkg#meta/my_test.cmx
```

传递给 `run_test_component` 的 URL 代表了一个测试组件的 url。

## 运行额外服务
所有测试组件都在一个新的沙箱环境中运行。这些环境默认的只包括一些基础的服务，例如
`fuchsia.sys.Environment` 和 `fuchsia.sys.Launcher`。要运行额外的服务，你可以向配置文件中增加 `injected-services` 条目：

```json
"facets": {
  "fuchsia.test": {
    "injected-services": {
        "service_name1": "component_url1",
        "service_name2": "component_url2"
    }
  }
}
```

`run_test_component` will start `component_url1` and `component_url2` and the
test will have access to `service_name1` and `service_name2`.
`run_test_component` 将运行 `component_url1` 和 `component_url2` 。这些测试组件将拥有访问 `service_name1` 和 `service_name2` 服务的权限。

### 网络权限
当前我们不能在沙箱环境下运行一个网络协议栈的实例，因为这会和真正的网络协议栈冲突。如果你的测试组件需要和网络协议栈交互，它只能够与沙箱环境外真正的协议栈进行交互。要进行这种操作，你需要允许一些系统服务：

```json
"facets": {
  "fuchsia.test": {
    "system-services": [
      "fuchsia.netstack.Netstack",
      "fuchsia.net.LegacySocketProvider",
      "fuchsia.net.Connectivity",
      "fuchsia.net.stack.Stack"
    ]
  }
}
```

依赖于实际的使用情况，你可以允许使用上述的一个或多个服务。
然而，我们不允许其他任何服务的使用。

这个选项将在我们修复 CP-144 之后被废弃。