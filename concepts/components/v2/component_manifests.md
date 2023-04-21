<!--
# Component manifests {#component-manifests}
 -->
# 组件清单 {#component-manifests}

<!--
A [component manifest](#component-manifest) is a file that defines a component
by encoding a [component declaration](#component-declaration). This document
gives an overview of the concepts used by component declarations.
Component declarations contain information about the following:
 -->
[组件清单](#component-manifest)（component manifest）是一个文件，它通过编码[组件声明](#component-declaration)（component declaration）来定义组件。本文档概述了组件声明使用的概念。组件声明包含了关于以下内容的信息：

<!--
-   **Execution:** Details about the runtime and executable binary for
    components that include an [executable program][manifest-program].
-   **Composition:** [Child component instances][doc-children]
    and [dynamic collections][doc-collections] managed by this component.
-   **Capabilities:** [Routing rules][doc-capabilities] that describe how
    capabilities are provided and consumed between components.
-   **Metadata:** [Freeform data][manifest-facet], which is ignored by the
    component framework but can be interpreted by third parties.
 -->
-   **执行**：针对包含[可执行程序][manifest-program]组件，关于运行时和可执行二进制文件的详细信息。
-   **组成**：由该组件管理的[子组件实例][doc-children]和[动态集合][doc-collections]。
-   **能力**：描述组件之间如何提供和使用能力的[路由规则][doc-capabilities]。
-   **元数据**：[自由格式数据][manifest-facet]，组件框架将其忽略，但可由第三方解释。

<!--
Note: For complete details on component manifest attributes and syntax, see the
[CML reference](https://fuchsia.dev/reference/cml).
 -->
注意：要获取关于组件清单属性和语法的完整细节，请参阅[CML 参考](https://fuchsia.dev/reference/cml)。

<!--
![Component manifest stages](images/component-manifest.png){: width="836"}
 -->
![组件清单阶段](images/component-manifest.png){: width="836"}

<!--
## Component manifest source {#component-manifest-source}
 -->
## 组件清单源 {#component-manifest-source}

<!--
A [component manifest source][glossary.component manifest source] is a file that
encodes part of a component manifest. Component manifest sources are written in
component manifest language (CML), which is the developer-facing source format
for component manifests. CML files are [JSON5][json5-external]{: .external}
files that end with a `.cml` extension.
 -->
[组件清单源][glossary.component manifest source]（component manifest source）是编码组件清单一部分的文件。组件清单源以组件清单语言（component manifest language，CML）编写，该语言是用于组件清单的面向开发人员的源格式。CML 文件是以 `.cml` 扩展名结尾的 [JSON5][json5-external]{: .external} 文件。

<!--
The [`cmc`][src-cmc] tool compiles component manifest sources to
[component manifests](#component-manifest) as a part of the build process.
 -->
[`cmc`][src-cmc] 工具将组件清单源编译为[组件清单](#component-manifest)，作为构建过程的一部分。

<!--
## Component manifest {#component-manifest}
 -->
## 组件清单 {#component-manifest}

<!--
A [component manifest][glossary.component manifest] is a binary file that
encodes the [component declaration](#component-declaration), usually distributed
as part of a [package][glossary.package]. The binary format is the
[persistent encoded form][fidl-wire-encoded] of the component declaration FIDL
object.
 -->
[组件清单][glossary.component manifest]是一个二进制文件，该文件编码[组件声明](#component-declaration)，通常作为[包][glossary.package]（package）的一部分分发。二进制格式是组件声明 FIDL 对象的[持久编码形式][fidl-wire-encoded]（persistent encoded form）。

<!--
A [component URL][doc-component-url] identifies a component in a package by its
component manifest resource path, typically ending in a `.cm` extension.
 -->
[组件网址][doc-component-url]（component URL）通过其组件清单资源路径来标识一个包中的组件，通常以 `.cm` 扩展名结尾。

<!--
## Component declaration {#component-declaration}
 -->
## 组件声明 {#component-declaration}

<!--
A [component declaration][glossary.component declaration] describes what a
component can do, the capabilities it uses and exposes, its children, and other
information needed to run the component. Component declarations are represented
using the [`Component`][fidl-component-decl] FIDL table.
 -->
[组件声明][glossary.component declaration] 描述了组件的功能、使用和公开的能力、子组件以及运行该组件所需的其他信息。组件声明通过使用 [`Component`][fidl-component-decl] FIDL 表来表示。

<!--
The framework calls a [component resolver][capability-resolver] to retrieve a
component declaration from a component URL.
 -->
该框架调用[组件解析器][capability-resolver]从组件网址检索组件声明。

[capability-resolver]: /concepts/components/v2/capabilities/resolvers.md
[capability-runner]: /concepts/components/v2/capabilities/runners.md
[doc-children]: /concepts/components/v2/realms.md#child-component-instances
[doc-capabilities]: /concepts/components/v2/capabilities/README.md
[doc-collections]: /concepts/components/v2/realms.md#collections
[doc-component-url]: /concepts/components/v2/identifiers.md#component-urls
[doc-environments]: /concepts/components/v2/environments.md
[fidl-component-decl]: https://fuchsia.dev/reference/fidl/fuchsia.component.decl#Component
[fidl-wire-encoded]: /reference/fidl/language/wire-format/README.md#dual-forms
[glossary.component declaration]: /glossary/README.md#component-declaration
[glossary.component manifest]: /glossary/README.md#component-manifest
[glossary.component manifest source]: /glossary/README.md#component-manifest-source
[glossary.package]: /glossary/README.md#package
[json5-external]: https://json5.org/
[manifest-program]: https://fuchsia.dev/reference/cml#program
[manifest-facet]: https://fuchsia.dev/reference/cml#facets
[src-cmc]: /tools/cmc
