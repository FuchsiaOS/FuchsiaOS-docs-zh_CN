<!-- ## Identifying components -->
## 组件的标记

<!-- 
Components are identified by a URL. The framework resolves component URLs to
component declarations with the help of a **component resolver**. Resolvers are
components themselves that are capable of handling a particular URL scheme and
fetching the component manifest, program, and assets.
 -->
组件由 URL 标识。框架借助**组件解析器**将组件 URL 解析为组件声明。解析器本身就是能够处理特定 URL 格式并获取组件清单、程序和资源的组件。

<!-- 
Most components are published inside a Fuchsia package, so the component URL is
a reference to the component manifest inside that package. See the following
example:
 -->
大多数组件都发布在 Fuchsia 包内，因此组件 URL 是对该包内组件清单的引用。请查看下面的示例：


```none
fuchsia-pkg://fuchsia.com/{{ '<var>' }}foo-package{{ '</var>' }}#meta/{{ '<var>' }}foo-component.cm{{ '</var>' }}
```

<!-- 
Component instances are identified by a topological path reference known as a
**moniker**. A component's moniker indicates its location within the component
instance tree as an absolute or relative path. For example, the moniker path
`/core/system-updater` refers to the instance of `system-updater` that exists
in the `core` realm.
 -->
组件实例由称为 **绰号（moniker）** 的拓扑路径引用来标识。组件的绰号将其在组件实例树中的位置指示为绝对或相对路径。例如，绰号路径 `/core/system-updater` 指的是存在于 `core` 领域中的 `system-updater` 的实例。
