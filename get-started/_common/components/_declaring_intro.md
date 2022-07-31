<!-- 
Every component has a declaration that describes the component's attributes and
capabilities. For components that are distributed in packages, the declaration
is expressed using a **component manifest file** and loaded with the help of a
**component resolver**.
 -->
每个组件（component）都有一个声明，描述组件属性和功能。对于在软件包中分发的组件，其声明使用**组件清单（component manifest）文件**表示，并在**组件解析器**的帮助下加载。

<!-- 
![Diagram showing how components are declared using a "component manifest." The
manifest is compiled by the developer tools and resolved onto the device at
runtime.](/get-started/images/components/component-manifest.png){: width="836"}
 -->
![该图显示了如何使用“组件清单”来声明组件。该清单由开发者工具编译，并在运行时解析到设备上。](/get-started/images/components/component-manifest.png){: width="836"}

<!-- 
You declare components using component manifest language (CML) files. At build
time, the Component Manifest Compiler (`cmc`) tool validates and compiles the
manifest source into a binary format (`.cm`) and stores it in the component's
package. At runtime, component resolvers load the binary manifest into a
[ComponentDecl](https://fuchsia.dev/reference/fidl/fuchsia.component.decl#Component)
FIDL structure for [Component Manager](/glossary/README.md#Component-Manager).
 -->
请您使用组件清单语言（CML）文件声明组件。在构建时，组件清单编译器（`cmc`）工具会验证清单源并将其编译为二进制格式（`.cm`），并将其存储在组件的软件包中。在运行时，组件解析器将二进制清单文件加载到[组件管理器](/glossary/README.md#Component-Manager)的 [ComponentDecl](https://fuchsia.dev/reference/fidl/fuchsia.component.decl#Component) FIDL 结构中。
