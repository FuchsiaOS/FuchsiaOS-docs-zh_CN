<!--
# Component resolvers
 -->
# 组件解析器

<!--
Component resolvers extend the component framework through an
[environment][glossary.environment] to resolve a
[component URL][glossary.component-url] into a component.
 -->
组件解析器（component resolver）通过[环境][glossary.environment]扩展组件框架，将[组件网址][glossary.component-url]解析为组件。

<!--
Component manager resolves component URLs by finding a resolver that supports a
matching URL scheme in the relevant environment and sending a request using the
[`fuchsia.component.resolution.Resolver`][fidl-resolver] protocol.
 -->
组件管理器通过在相关环境中查找支持匹配网址方案的解析器并使用 [`fuchsia.component.resolution.Resolver`][fidl-resolver] 协议发送请求来解析组件网址。

<!--
If resolution succeeds, the component resolver returns a
[`ComponentDecl`][fidl-decl], the FIDL representation of a
[component manifest][component-manifest]. If the component being resolved has
an associated package, the component resolver also returns a
[`fuchsia.io.Directory`][fidl-directory] handle for the package directory.
 -->
如果解析成功，那么组件解析器会返回一个 [`ComponentDecl`][fidl-decl]，它是[组件清单][component-manifest] 的 FIDL 表示。如果正在解析的组件有关联的包，组件解析器还会返回包目录的 [`fuchsia.io.Directory`][fidl-directory] 句柄。

<!--
## Providing resolver capabilities {#provide}
 -->
## 提供解析器能力 {#provide}

<!--
To provide a resolver capability, a component must declare a `resolver`
capability, whose `path` designates a FIDL protocol implementing
[`fuchsia.component.resolution.Resolver`][fidl-resolver] served from the component's
[outgoing directory][glossary.outgoing-directory].
 -->
要提供解析器能力，组件必须声明解析器（`resolver`）能力，其路径（`path`）指定一个实现了提供自组件[传出目录][glossary.outgoing-directory] 的 [`fuchsia.component.resolution.Resolver`][fidl-resolver] 的 FIDL 协议。

```json5
{
    capabilities: [
        {
            resolver: "my_resolver",
            path: "/svc/fuchsia.component.resolution.Resolver",
        },
    ],
}
```

<!--
Component manager submits requests to resolve a component URL to this protocol.
 -->
组件管理器提交请求以将组件网址解析到此协议。

<!--
## Routing resolver capabilities {#route}
 -->
## 路由解析器能力 {#route}

<!--
Components route resolver capabilities by [exposing](#expose) them to their
parent and [offering](#offer) them to their children.
 -->
组件通过将解析器能力[公开](#expose)（expose）至其父级并[提供](#offer)（offer）至其子级对其进行路由。

<!--
For more details on how the framework routes component capabilities,
see [capability routing][capability-routing].
 -->
要获取关于框架路由组件能力方式的更多细节，请参见[能力路由][capability-routing]。

<!--
### Exposing {#expose}
 -->
### 公开 {#expose}

<!--
Exposing a resolver capability gives the component's parent access to that
capability:
 -->
公开（expose）解析器能力会给予父组件访问该能力的权限：

```json5
{
    expose: [
        {
            resolver: "my_resolver",
            from: "self",
        },
    ],
}
```

<!--
### Offering {#offer}
 -->
### 提供 {#offer}

<!--
Offering a resolver capability gives a child component access to that
capability:
 -->
提供（offer）解析器能力会给予子组件访问该能力的权限：

```json5
{
    offer: [
        {
            resolver: "my_resolver",
            from: "self",
            to: [ "#child-a" ],
        },
    ],
}
```

<!--
## Registering a component resolver {#register}
 -->
## 注册组件解析器 {#register}

<!--
Component resolvers are made available to components through their
[environment][environment]. To register a new resolver within an environment,
add a new entry to the `resolvers` section of the `environments` declaration:
 -->
组件解析器是通过[环境][environment]提供给组件的。要在环境中注册新的解析器，请将新条目添加到 `environments`（环境）声明的 `resolvers` 部分：

```json5

environments: [
    {
        name: "my-environ",
        extends: "realm",
        resolvers: [
            {
                resolver: "my_resolver",
                scheme: "my-scheme",
                from: "parent",
            }
        ],
    },
]
```

<!--
The registered resolver will be used to resolve component URLs whose URL scheme
matches the provided `scheme`.
 -->
已注册解析器将用于解析与所提供 `scheme`（方案）相匹配的组件网址。

<!--
For more details on how to apply environments to components, see the
[environments documentation][environment].
 -->
要获取关于将环境应用到组件的更多细节，请参阅[环境文档][environment]。

<!--
## Framework resolvers {#framework}
 -->
## 框架解析器 {#framework}

<!--
Component framework provides the following built-in component resolvers to
support standard Fuchsia URL schemes:
 -->
组件框架提供了以下内置组件解析器来支持标准 Fuchsia 网址方案：

<!--
| Resolver            | URL scheme                 |
| ------------------- | -------------------------- |
| `boot_resolver`     | [`fuchsia-boot`][url-boot] |
| `base_resolver`     | [`fuchsia-pkg`][url-pkg]   |
| `full-resolver`     | [`fuchsia-pkg`][url-pkg]   |
 -->
| 解析器          | 网址方案                   |
| --------------- | -------------------------- |
| `boot_resolver` | [`fuchsia-boot`][url-boot] |
| `base_resolver` | [`fuchsia-pkg`][url-pkg]   |
| `full-resolver` | [`fuchsia-pkg`][url-pkg]   |

[glossary.component-url]: /glossary/README.md#component-url
[glossary.environment]: /glossary/README.md#environment
[glossary.outgoing-directory]: /glossary/README.md#outgoing-directory
[capability-routing]: /concepts/components/v2/capabilities/README.md#routing
[component-manifest]: /concepts/components/v2/component_manifests.md
[environment]: /concepts/components/v2/environments.md
[fidl-resolver]: /sdk/fidl/fuchsia.component.resolution/resolver.fidl
[fidl-decl]: /sdk/fidl/fuchsia.component.decl/component.fidl
[fidl-directory]: /sdk/fidl/fuchsia.io/directory.fidl
[url-boot]: /reference/components/url.md#fuchsia-boot
[url-pkg]: /reference/components/url.md#fuchsia-pkg
