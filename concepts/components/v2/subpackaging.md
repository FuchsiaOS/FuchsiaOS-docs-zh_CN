<!--
# Subpackaging components
 -->
# 子包化组件

<!--
[Packages] can "contain" other packages (referred to as their
[subpackage][glossary.subpackage]s), producing a hierarchy of nested packages.
[Components] can leverage subpackaging to organize a [hierarchy of nested
components][hierarchy-of-nested-components], where each component is
encapsulated in its own package, and brings its own set of dependencies.
 -->
[包][Packages]（package）可以“包含”其他包（称为其[子包][glossary.subpackage]（subpackage）），产生嵌套包的层次结构。[组件][Components]可以利用子包化（subpackaging）来组织[嵌套组件的层次结构][hierarchy-of-nested-components]，其中每个组件都封装在其自身的包中，并带来了其自身的一套依赖项。

<!--
_Subpackages enable:_
 -->
**子包带来了：**

<!--
* Encapsulated dependencies (a packaged component declares its direct
  dependencies only)
* Isolated `/pkg` directories (grouped components don't need to merge their
  files, libraries, and metadata into a single shared namespace)
* Assured dependency resolution (system and build tools ensure subpackages
  always "travel with" their packages)
 -->
* 封装的依赖关系（打包的组件仅声明其直接依赖）
* 隔离的 `/pkg` 目录（分组的组件不需要将其文件、库和元数据合并到单个共享命名空间中）
* 有保证的依赖解析（系统和构建工具确保子包始终“随同”其包装）

<!--
## Relationship to Fuchsia Components
 -->
## 与 Fuchsia 组件的关系

<!--
Fuchsia uses packages to "distribute" its software (for example, to load the
software onto a device). A single component with no included dependencies is
typically contained in a single package. A component that launches other
components can define a package that _includes_ specific versions (determined at
build time) of its child components using subpackaging.
 -->
Fuchsia 使用包来“分发”其软件（例如，将软件加载到设备上）。单个包中通常包含未包括依赖项的单个组件。启动其他组件的组件可以使用子包化来定义包括其子组件特定版本（在构建时确定）的包。

<!--
When organized this way, the subpackage hierarchy mirrors the [component
parent-child relationships][component-parent-child-relationship]. Child
components will then be loaded from a declared subpackage of the parent
component's package. This encapsulates [ABI][glossary.abi] dependencies at
package boundaries.
 -->
以这种方式组织时，子包层次结构反映了[组件父子关系][component-parent-child-relationship]。之后子组件将从父组件包的已声明子包中加载。这就在包边界处封装了 [ABI][glossary.abi] 依赖项。

<!--
Components can also use a subpackage to declare a dependency on a data-only
package. While components cannot resolve a package directly, a package can
expose data to components selectively, by including a component manifest that
defines and exposes directory capabilities to other components. Each directory
capability maps to a specific package subdirectory. (With a data-only package,
the manifest does not include a `program` declaration.) By treating exposed data
as standard [directory capabilities], the components use [capability routing] to
ensure the right information is only made available to the components that need
it.
 -->
组件还可以使用子包来声明对纯数据包的依赖。虽然组件无法直接解析包，但包可以通过包含向其他组件定义和公开目录能力的组件清单的方式，有选择地向组件公开数据。每个目录能力都映射到特定的包子目录。（对于纯数据包，清单不包括 `program`（程序）声明。）通过将公开的数据视为标准[目录能力][directory capabilities]，组件使用[能力路由][capability routing]以确保正确的信息仅可用于需要它的组件。

<!--
### Package dependendencies mirror Component dependencies
 -->
### 包依赖反映组件依赖

<!--
A Fuchsia system is defined by a hierarchy of components. Starting with the
first component (the root of the hierarchy), components add capabilities to the
system by launching `children` (child components) that serve those capabilities.
Each component has the opportunity to launch its own subtree of components.
 -->
Fuchsia 系统由组件层次结构定义。从第一个组件（层次结构的根）开始，组件通过启动为这些能力提供服务的子组件向系统添加能力。每个组件都有机会启动自己的组件子树。

<!--
To instantiate a `child` component, the parent identifies the child's source
(implementation software) by its location in the package system using a
"component URL" (a package URL combined with the intra-package resource location
of the component manifest); for example
`fuchsia-pkg://fuchsia.com/package#meta/component.cm`.
 -->
为了实例化“子”（child）组件，父组件使用“组件网址”（与组件清单的包内资源位置相结合的包网址）通过其在包系统中的位置来识别子组件的来源（实现软件）；例如 `fuchsia-pkg://fuchsia.com/package#meta/component.cm`。

<!--
Importantly, under the Component Framework, the _only_ place a component refers
to a runtime dependency by component URL is when declaring children. Component
URLs are not used to define a dependency on a peer component or any other
component outside of its local subtree.
 -->
重要的是，在组件框架下，组件**唯一**在声明子组件时会通过组件网址引用运行时依赖。组件网址不用于定义对等组件或其本地子树之外的任何其他组件的依赖。

<!--
When a child component is defined by an absolute component URL like
`fuchsia-pkg://fuchsia.com/package#meta/component.cm`, the component developer
cedes control over the implementation of that dependency, to be determined
(potentially) at product assembly time, or at runtime from an ephemeral source
(a package server).
 -->
当子组件由绝对组件网址定义时，如 `fuchsia-pkg://fuchsia.com/package#meta/component.cm`，组件开发人员放弃对该依赖项实现的控制，（可能）待产品汇编时确定，或运行时从临时源（包服务器）确定。

<!--
Subpackaging allows the developer to instead declare package dependencies with
build-time resolution, "baking in" the expected component implementations,
including known ABI and behavior, without compromising the encapsulation and
isolation benefits of package boundaries. This ensures a package with
component dependencies has a hermetic implementation, and the behavior of its
child components will not change without rebuilding the parent component's
package.
 -->
子包化允许开发人员改为使用构建时解析来声明包依赖性，“融入”预期的组件实现，包括已知的 ABI 和行为，而不会损害包边界的封装和隔离优势。这确保了具有组件依赖项的包具有密封实现，且其子组件的行为不会在未重建父组件包的情况下发生变化。

<!--
Subpackaged component URLs also avoid problems inherent with absolute component
URLs: If a parent component is loaded from (for example) an alternate repository
like `fuchsia-pkg://alt-repo.com/package#meta/parent.cm`, its likely that its
children may also be in that `alt-repo`, and there is no way to statically
define an absolute component URL that can resolve from either `fuchsia.com` or
`alt-repo.com` (or another) not known until runtime.
 -->
子包组件网址（subpackaged component URL）也避免了绝对组件网址固有的问题：如果父组件是从（例如）像 `fuchsia-pkg://alt-repo.com/package#meta/parent.cm` 这样的备用仓库加载的，那么其子组件也很可能在该 `alt-repo` 中，且无法静态定义可以从直到运行时才知道的 `fuchsia.com` 或 `alt-repo.com`（或其他）解析的绝对组件网址。

<!--
By using relative package paths, a subpackaged child component's implementation
is identified by a [relative component URL] with subpackage name (a subpackage
URL, with a URI fragment specifying the path to the component manifest), such as
`some-child#meta/default.cm`. The mapping from subpackage name `some-child` is
declared in a build configuration, and resolved at build time, by storing the
subpackage's package hash in the parent component's package metadata, mapped to
the subpackage name.
 -->
通过使用相对包路径，子包子组件（subpackaged child component）的实现由带有子包名称的[相对组件网址][relative component URL]标识（一个带有指定组件清单路径的 URI 片段的子包网址），例如 `some-child#meta/default.cm`。子包名称 `some-child` 的映射在构建配置中声明，并在构建时解析，方法是将子包的包哈希存储在父组件的包元数据中，映射到子包名称。

<!--
<aside class="key-point">
This superpackage <-> subpackage relationship between packages naturally mirrors
the parent <-> child relationship between components.
</aside>
 -->
<aside class="key-point">
这种包间的“超包↔子包”关系自然反映了组件间的“父↔子”关系。
</aside>

<!--
### Dependencies are transitive and encapsulated
 -->
### 依赖是传递和封装的

<!--
Component software implementations _do not `use`_ other components. Components
`use` capabilities. A component's capabilities may come from its parent
(routed directly or indirectly by the parent, without the knowledge of the
component) or from a child. Importantly, a capability exposed by a child can
also be either direct or indirect. The child's implementation is encapsulated,
so a capability it exposes may be implemented by that child, or may be routed
from one of the child's children.
 -->
组件软件实现**不“使用”**（use）其他组件。组件“使用”能力。组件的能力可能来自其父组件（由父组件直接或间接路由，无需该组件知晓）或来自子组件。重要的是，子组件公开的能力也可以是直接或间接的。子组件的实现是封装的，因此其公开的能力可以由其实现，或者可能路由自从该子组件的子组件之一。

<!--
Subpackaging allows a component to completely encapsulate its implementation,
including any dependencies on sub-components.
 -->
子包化允许组件完全封装其实现，包括对子组件的任何依赖。

<!--
When a component declares children using absolute component URLs, the specific
implementation of that child is selected at runtime. This may be desired, for
certain use cases, but the trade-off is that the parent component is not
hermetic: It can be hard to re-use the parent component in new environments.
Distributing and porting non-hermetic code requires keeping track of all of the
external dependencies as well, and then ensuring the dependencies are always
available in each new environment.
 -->
当组件使用绝对组件网址声明子组件时，该子组件的特定实现是在运行时选择的。对于某些用例，这可能是需要的，但代价是父组件不是密封的：很难在新环境中重用父组件。分发和移植非密封代码还需要跟踪所有外部依赖项，并确保依赖项在每个新环境中始终可用。

```json5 {:.devsite-disable-click-to-copy}
    children: [
        {
            name: "intl_property_provider",
            url: "fuchsia-pkg://fuchsia.com/intl_property_manager#meta/intl_property_manager.cm",
        },
        ...
    ]
```

<!--
When runtime resolution is not required, the parent component can update its
children to use relative path URLs, and declare the child components' packages
as subpackage dependencies, resolved at build time. This way, when a component
subpackages a child component, the child's package brings all of its subpackaged
components inherently, without exposing those dependencies to the other
components and runtime environments that may use it.
 -->
当不需要运行时解析时，父组件可以更新其子组件使用相对路径网址，并将子组件的包声明为子包依赖项，在构建时解析。这样，当组件子包化子组件时，子组件会固有地带来所有其子包组件（subpackaged component），而不会将这些依赖项公开给可能使用它的其他组件和运行时环境。

```json5 {:.devsite-disable-click-to-copy}
    children: [
        {
            name: "intl_property_provider",
            url: "intl_property_manager#meta/intl_property_manager.cm",
        },
        ...
    ]
```

<!--
<aside class="key-point">
The subpackaged component can add, remove, or replace child components without
breaking API compatibility with the top component, as long as the child
component continues to serve the same capabilities, regardless of which
components implement which capabilities. Therefore subpackages provide a way to
mirror the encapsulation model of components.
</aside>
 -->
<aside class="key-point">
子包组件可以添加、删除或替换子组件，而只要子组件继续提供相同能力，不论哪些组件实现哪些能力，都不会破坏与顶级组件的 API 兼容性。因此，子包提供了一种镜像组件封装模型的方法。
</aside>

<!--
### No ambient authority through the `/pkg` directory
 -->
### 没有通过 `/pkg` 目录的环境权限

<!--
In order to support the basic runtime requirements of a Fuchsia Component,
a component may access a directory containing the contents of its package, via
the [`/pkg`][ambient-pkg-directory] directory capability.
 -->
为了支持 Fuchsia 组件的基本运行时要求，组件可以通过 [`/pkg`][ambient-pkg-directory] 目录能力访问包含其包内容的目录。

<!--
As described above, subpackaging allows packages to declare their component
dependencies as hierarchical, encapsulated packages of components. This model
does not require a separate package per component, but it does encourage it, and
the Fuchsia runtime and tools are designed to make the process of declaring,
building, and running separately-packaged components natural and performant.
 -->
如前所述，子包化允许包将其组件依赖声明为分层的、封装的组件包。该模型不需要每个组件一个单独的包，但确实鼓励这样做，Fuchsia 运行时和工具旨在使声明、构建和运行单独打包组件的过程自然而高效。

<!--
Conversely, multiple components combined in a single package share a single,
merged `/pkg` directory. Bundling more than one component in a single package
allows each component to access not only the same data, but also the metadata of
the other components in that package as well, without explicit capability
routing.
 -->
相反，组合在单个包中的多个组件共享单个合并的 `/pkg` 目录。将多个组件捆绑在单个包中允许每个组件不仅可以访问相同的数据，还可以访问同一包中其他组件的元数据，而无需显式的能力路由。

<!--
In certain cases, where multiple components share access to the same data, this
may be convenient. However, in cases where components need access to different
sets of data, or one component uses data that should not be exposed to the
other, packaging components together may undermine the [principle of least
privilege], making subpackages a better fit.
 -->
在某些情况下，多个组件共享对相同数据的访问可能很方便。然而，在组件需要访问不同数据集的情况下，或者当一个组件使用不应该公开给另一个组件的数据时，将组件打包在一起可能会破坏[最小权限原则][principle of least privilege]，从而使子包更合适。

<<../../../get-started/_common/components/_no_ambient_authority.md>>

<!--
The fact that a component might not take advantage of this consequential
privilege is more of a concern than a relief because this might not always be
the case, and the privilege opens up an unexpected opportunity for one component
to exploit the data of another component.
 -->
也许有人担心组件可能不会利用这一相应的特权，但情况可能并非总是如此，且该特权为一个组件利用另一组件的数据提供了意想不到的机会。

<!--
<aside class="key-point">
Subpackages ensure each component has its own isolated <code>/pkg</code>
directory while providing the same benefits of relative URL resolution, and
improvements to software hermeticity and software encapsulation benefits through
hierarchical nesting.
</aside>
 -->
<aside class="key-point">
子包确保每个组件都有自己独立的 <code>/pkg</code> 目录，同时继续提供相对网址解析的优势，并通过分层嵌套改进软件密封性和软件封装优势。
</aside>

<!--
## Advantages over using multiple components in a single package
 -->
## 相较于在单个包中使用多个组件的优势

<!--
Today, Fuchsia allows a single package to contain multiple components. This
feature predates the existence of subpackages, and it provides another way
to declare child components by a relative URL; that is, by a URI fragment that
identifies the component by resource path to the component manifest. A
component URL of the form `#meta/some-child.cm` informs the Fuchsia component
resolver to load the component implementation for `some-child` from the same
package that contained the parent component's manifest.
 -->
如今，Fuchsia 允许单个包包含多个组件。该特性早于子包出现，提供了另一种通过相对网址声明子组件的方法；即，通过利用组件清单的资源路径来标识组件的 URI 片段。`#meta/some-child.cm` 形式的组件网址通知 Fuchsia 组件解析器从父组件清单所在包中加载 `some-child` 的组件实现。

<!--
### Built-in access controls to share package resources
 -->
### 用于共享包资源的内置访问控制

<!--
The component framework helps to enforce Fuchsia's capability access control
policies by requiring components to declare their capability needs explicitly,
and by making the parent component responsible for routing any external
capabilities (including resources) from known capability sources (from the
parent's parent, or from another child).
 -->
组件框架通过这样的方式帮助施行 Fuchsia 的能力访问控制策略，即：要求组件显式声明其能力需求，并使父组件负责路由来自已知能力源（来自父级的父级，或来自另一子级）的任何外部能力（包括资源）。

<!--
If one component needs a resource from another component's package, the
Component Framework capability routing declarations allow the source component
to expose the specific subdirectory such that the target component can access
only what is required, and explicitly offered by its parent component.
 -->
如果一个组件需要来自另一个组件的包的资源，组件框架能力路由声明允许源组件公开特定子目录，以使目标组件只能访问所需内容，并由其父组件显式提供。

<!--
This supports any use case that might otherwise have been satisfied by relying
on access to a shared `/pkg` directory from a common package, without exposing
the entire `/pkg` directory.
 -->
这支持了原本可能依靠从通用包对共享 `/pkg` 目录访问的任何用例，而无需公开整个`/pkg`目录。

<!--
Subpackage-isolated `/pkg` directories combined with Component Framework
capability routing provide Fuchsia architecture-consistent way to control access
to and share package resources.
 -->
子包隔离的 `/pkg` 目录与组件框架能力路由相结合，提供了 Fuchsia 架构一致的方式来控制对包资源的访问和共享。

<!--
### Changes to transitive dependencies to not break encapsulation
 -->
### 避免破坏封装的传递性依赖更改

<!--
When combining component dependencies into a single package, all components
share a single, flat namespace, and transitive dependencies must also be
included.
 -->
将组件依赖项组合到单个包中时，所有组件共享单一的扁平（flat）命名空间，且必须包括传递性依赖。

<!-- TODO(fxbug.dev/116980): Add a diagram to help visualize this example. -->

<!--
For example, if single package `SP` bundles component `A` and component `B`, but
`B` also depends on `C` by relative URI fragment (`#meta/C.cm`), package `SP`
must bundle `A`, `B`, and `C`. If `B` is later modified to replace `C` with two
new components `D` and `E`, the definition of package `SP` must change to bundle
`A`, `B`, `D`, and `E`, and drop `C` _unless_ (for the sake of argument) either
`D` or `E` (or both) also depend on `C`.
 -->
例如，如果单个包 `SP` 捆绑了组件 `A` 和组件 `B`，但 `B` 还通过相对 URI 片段（`#meta/C.cm`）依赖 `C`，那么包 `SP` 必须捆绑 `A`、`B` 和 `C`。如果 `B` 稍后被修改，以便用两个新组件 `D` 和 `E` 替换 `C`，那么包 `SP` 的定义必须更改为捆绑 `A`、`B`、`D` 和 `E`，并删除 `C`，**除非**（出于参数原因）`D` 或 `E`（或两者）也依赖 `C`。

<!--
Although some build environments allow a component build target to declare
transitive component dependencies, this practice amplifies the risks of merging
the contents of these components into a single namespace. If a component _or any
of its dependencies_ changes, new files could overwrite files from other
components in any part of the component subtree in that package, breaking
implementations in undefined and potentially catastrophic ways.
 -->
尽管某些构建环境允许组件构建目标声明传递性组件依赖，但这种做法会放大将这些组件的内容合并到单个命名空间中的风险。如果组件**或其任何依赖项**发生变化，新文件可能会覆盖该包中组件子树任何部分中其他组件的文件，从而以未定义和潜在灾难性的方式破坏实现。

<!--
Subpackages greatly simplify transitive dependencies by encapsulating them in
the definition of each subpackage, so package `SP` can be replaced with package
`A` (containing component `A`) having a dependency on _only_ subpackage `B`
(containing component `B`). Package `A` requires no other dependencies, and
does not change, even if `B`'s dependencies change.
 -->
子包通过将它们封装在每个子包的定义中的方式极大地简化了传递性依赖，因此包 `SP` 可以替换为**仅**依赖子包 `B`（包含组件 `B`）包 `A`（包含组件 `A`）。包 `A` 不需要其他依赖项，且即使 `B` 的依赖项发生变化时也不会更改。

<!--
### Subpackaged implementations are build-time guarantees
 -->
### 子包实现是构建时保证

<!--
Using relative URI fragment component URLs (like, `#meta/some-child.cm`), does
not actually guarantee ABI or even API compatibility between parent and child
components "in the same package" because they could in fact be resolved from
different versions of that package.
 -->
使用相对 URI 片段组件网址（例如 `#meta/some-child.cm`），实际上并不能保证“同一包中”父子组件之间的 ABI 甚至 API 兼容性，因为它们实际上可从该包的不同版本解析。

<!--
If the package is resolved ephemerally (from a package server). A new version of
the same package can be re-published between the time the parent component was
resolved and a later time when the child component is required and loaded. The
child implementation might be different from the implementation included in the
original version of the package.
 -->
如果包是（从包服务器）临时解析的。同一包的新版本可以在父组件被解析的时刻和之后子组件被需要并加载的时刻之间重新发布。子实现可能与包原始版本中包含的实现不同。

<!--
This is not a rare or contrived use case: In Component Framework, components are
(by default) resolved only when needed. A component that exposes a single
service `S` will not be loaded until and unless some other component actually
requires service `S`. Depending on the business logic of the program, `S` might
be called upon minutes or hours (or more) after the parent component was
launched.
 -->
这是一个非罕见、非人为的用例：在组件框架中，组件（默认情况下）仅在需要时才解析。公开单个服务 `S` 的组件将不会加载，除非其他组件确实需要服务 `S`。根据程序的业务逻辑，`S` 可能会在父组件启动后几分钟或几小时（或更长时间）被调用。

<!--
## Examples
 -->
## 示例

<!--
### Declaring build dependencies to subpackages
 -->
### 向子包声明构建依赖

<!--
Fuchsia-enabled build frameworks should include a pattern for declaring a
Fuchsia package and its contents. If also enabled to support subpackages, a
package declaration will list the subpackages it depends on, by direct
containment.
 -->
支持 Fuchsia 的构建框架应该包括一个用于声明 Fuchsia 包及其内容的模式。如果还支持子包，则包声明将通过直接包含的方式列出它所依赖的子包。

<!--
For example, in fuchsia.git, the GN templates for declaring Fuchsia packages
support two optional lists, `subpackages` and (less commonly used)
`renameable_subpackages`. One or both can be included. The `renameable_`
version allows the package to assign a package-specific name to the subpackage,
used when referring to the subpackage by package URL or component URL:
 -->
例如，在 fuchsia.git 中，用于声明 Fuchsia 包的 GN 模板支持两个可选列表，`subpackages` 和（不太常用的）`renameable_subpackages`。可以包括其一或两者。`renameable_` 版本允许包为子包分配一个特定于包的名称，在通过包网址或组件网址引用子包时使用：

```gn
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/subpackages/BUILD.gn" region_tag="declare_subpackages" adjust_indentation="auto" %}
```

<!--
The `subpackages` list contains a list of GN `fuchsia_package` build targets. By
default, the subpackage name (the name the containing package will use to refer
to the package) is taken from the defined `package_name` of the subpackage's
`fuchsia_package` target.
 -->
`subpackages` 列表包含 GN `fuchsia_package` 构建目标的列表。默认情况下，子包名称（包含该包的包对其引用时使用的名称）取自子包的 `fuchsia_package` 目标已定义的 `package_name`。

<!--
Subpackage targets can also be declared using the `package` variable in
the `renameable_subpackages` list. `renameable_targets` also include an optional
`name` variable, to override the default name for the subpackage.
 -->
子包目标也可以使用 `renameable_subpackages` 列表中的 `package` 变量声明。`renameable_targets` 还包括一个可选的 `name` 变量，用于为子包覆盖默认名称。

<!--
### Declaring subpackaged children
 -->
### 声明子包子组件

<!--
A subpackage is only visible to its parent package, and the component(s) in that
package. Consequently, subpackage names only need to be unique within that
parent package. If two subpackage targets have the same name (or for any other
reason), the parent is free to assign its own subpackage names (via
`renameable_subpackages` in GN, for instance).
 -->
子包仅对其父包以及该包中的组件可见。因此，子包名称只需要在该父包内是唯一的。如果两个子包目标具有相同名称（或出于任何其他原因），则父包可以自由分配自己的子包名称（例如，通过 GN 中的 `renameable_subpackages`）。

<!--
When declaring subpackaged child components in CML, the `url` should be the
relative subpackaged component URL, as shown in the following example:
 -->
当在 CML 中声明子包子组件时，`url` 应为相对子包组件网址，如下例所示：

```json5
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/subpackages/meta/echo_client_with_subpackaged_server.cml" region_tag="declare_children_statically" adjust_indentation="auto" %}
```

<!--
Subpackaged child components can also be referenced in runtime declarations,
such as when declaring children through [Realm Builder] APIs. For example:
 -->
子包子组件也可在运行时声明中引用，例如通过[领域构建器][Realm Builder]（Realm Builder）API 声明子组件时。例如：

```rust
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/subpackages/src/lib.rs" region_tag="declare_children_dynamically" adjust_indentation="auto" %}
```

[Packages]: /concepts/packages/package.md
[Realm Builder]: /development/testing/components/realm_builder.md
[Components]: /concepts/components/v2/introduction.md
[ambient-pkg-directory]: /concepts/components/v2/capabilities/life_of_a_protocol_open.md#offered-vs-ambient-capabilities
[component-parent-child-relationship]: /concepts/components/v2/topology.md
[capability routing]: /concepts/components/v2/topology.md#capability-routing
[directory capabilities]: /concepts/components/v2/capabilities/directory.md
[hierarchy-of-nested-components]: /concepts/components/v2/components_as_classes.md#component-manifests-as-classes
[principle of least privilege]: /get-started/sdk/learn/intro/sandboxing.md
[relative component URL]: /reference/components/url.md#relative-path-urls-to-subpackaged-components
[glossary.abi]: /glossary/README.md#abi
[glossary.subpackage]: /glossary/README.md#subpackage
