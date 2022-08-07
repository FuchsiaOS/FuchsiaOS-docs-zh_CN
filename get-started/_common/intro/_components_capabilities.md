<!-- ## Component capabilities -->
## 组件功能

<!-- Components obtain privileges to access various parts of the wider system
through **capabilities**. Each component can declare new capabilities that
they offer to the system and capabilities provided by other components
(or the framework) that they require to function. -->
组件通过**功能**获得访问更大系统的各个部分的权限。每个组件都可以声明它能提供给系统的新功能以及所需的由其他组件（或框架）提供的功能。

<!-- As you just saw, `runner` is an example of a capability declaring the runtime
used by the component. Other examples of common capability types are
`directory` to access filesystem resources and `protocol` for communicating
with other components. -->
如您所见，`runner` 是一个声明组件运行时所需功能的例子。其它通用功能类型的例子是访问文件系统资源的 `directory` 和与其它组件通信的 `protocol`（协议）。

<!-- Developers declare the capability types required by the component using the
component manifest. Below is an example of a component manifest requesting
two capabilities: read access to an `example-data` directory and a service
described by the `fuchsia.example.Foo` FIDL protocol. -->
开发者在组件清单声明组件所需要的功能类型。以下是一个需要两个功能的组件清单示例：对 `example-data` 目录的读权限和以 FIDL 协议描述的名为 `fuchsia.example.Foo` 的服务。

```json5
use: [
    {
        directory: "example-data",
        rights: [ "r*" ],
        path: "/example/data",
    },
    {
        protocol: "fuchsia.example.Foo",
    },
]
```

<!-- Component manager uses the capability declarations to populate each component's
namespace with the necessary directory handles. For this example, the component
would receive `/example/data` and `/svc/fuchsia.example.Foo` in their namespace. -->
组件管理器依据功能声明将各个组件所需的目录处理句柄填入各自的命名空间。对于本例子而言，组件将在其命名空间中接收 `/example/data` 和 `/svc/fuchsia.example.Foo`。
