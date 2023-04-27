<!--
# Environments
 -->
# 环境

<!--
*Environments* provide a way to configure certain choices the framework makes
for components in a [realm][doc-realms].
[Component manifests][doc-component-manifests] may define environments and
assign them to child realms. An environment applies to a component instance's
entire realm, unless some sub-realm overrides it (see
[Propagation](#propagation)).
 -->
**环境**（environment）提供了一种方法来配置框架为[领域][doc-realms]中组件所做的某些选择。[组件清单][doc-component-manifests]可以定义环境并将它们分配给子领域。环境适用于组件实例的整个领域，除非某些子领域进行了覆盖（参见[传播](#propagation)一节）。

<!--
## Properties {#properties}
 -->
## 属性 {#properties}

<!--
Environments let you configure the following behavior of a realm:
 -->
环境允许您配置领域的以下行为：

<!--
-   [Component runners](#runners)
-   [Component resolvers](#resolvers)
 -->
-   [组件运行器](#runners)
-   [组件解析器](#resolvers)

<!--
### Runners {#runners}
 -->
### 运行器 {#runners}

<!--
The component framework is runtime-agnostic and can support new runtime
environments and programming languages without requiring changes to
component manager or to other components. Runners provide the extension point
for components to interact with component manager and add runtime support to
Fuchsia. Some example runners are:
 -->
组件框架是运行时无关的（runtime-agnostic），可以支持新的运行时环境和编程语言，而无需更改组件管理器或其他组件。运行器（runner）为组件提供与组件管理器交互并在 Fuchsia 添加运行时支持的扩展点。一些运行器例如：

<!--
-   The [ELF runner][elf-runner] runs binaries compiled to the ELF file format.
-   The Dart AOT runner provides a runtime for Dart programs, such as a VM.
-   The Chromium web runner provides a runtime for components implemented as web
    pages.
 -->
-   [ELF 运行器][elf-runner]运行编译为 ELF 文件格式的二进制文件。
-   Dart AOT 运行器为 Dart 程序（例如VM）提供运行时。
-   Chromium Web 运行器为实现为网页的组件提供运行时。

<!--
Component manager identifies _what_ to execute and delegates _how_ execution
works to the runner. Runner implementations are free to choose an appropriate
strategy for executing their components, including:
 -->
组件管理器确定要执行的“内容”并委派执行作用于运行器的“方式”。执行器的实现可以自由选择适当的策略来执行其组件，包括：

<!--
-   Start a new process for the component.
-   Isolate the component within a virtual machine.
-   Run the component in the same process as the runner.
-   Execute the component as a job on a remote computer.
 -->
-   为组件启动新进程。
-   将组件隔离至虚拟机中。
-   在与运行器相同的进程中运行组件。
-   在远程计算机上将组件作为作业执行。

<!--
For more details on using and implementing runners, see
[runner capabilities](capabilities/runners.md).
 -->
要获取关于使用和实现运行器的更多细节，请参阅[运行器能力](capabilities/runners.md)。

<!--
### Resolvers {#resolvers}
 -->
### 解析器 {#resolvers}

<!--
Component resolvers interact with component manager on behalf of a component to
resolve its children from a given [component URL][glossary.component-url].
Resolvers are registered with a particular URL scheme (`http`, `fuchsia-pkg`, etc.)
and provide an implementation to fetch the component from the desired URL and
return a [component declaration][glossary.component-declaration].
 -->
组件解析器代表组件与组件管理器进行交互，以从给定的[组件网址][glossary.component-url]中解析其子组件。解析器通过特定的网址方案（`http`、`fuchsia-pkg` 等）注册，并提供实现以从所需网址获取组件并返回[组件声明][glossary.component-declaration]。

<!--
If the component being resolved has an associated package, the resolver also
returns a [`fuchsia.io.Directory`][fidl-directory] handle representing the
package directory.
 -->
如果要解析的组件具有关联的软件包，则解析器还返回一个代表软件包目录的 [`fuchsia.io.Directory`][fidl-directory] 句柄。

<!--
For more details on using and implementing resolvers, see
[resolver capabilities](capabilities/resolvers.md).
 -->
要获取关于使用和实现解析器的更多细节，请参见[解析器能力](capabilities/resolvers.md)。

<!--
## Declaring {#declaring}
 -->
## 声明 {#declaring}

<!--
Define a new environment by adding an [environments][doc-environments]
declaration to a [component manifest][doc-component-manifests].
 -->
通过在[组件清单][doc-component-manifests]中添加[环境][doc-environments]来定义一个新的环境。

<!--
For an environment to be used, you must assign it to a child or collection. See
[Propagation](#propagation).
 -->
要使用环境，您必须将其分配给子组件或集合。请参阅[传播](#propagation)一节。

<!--
Environments support two modes of extension, [`REALM`][fidl-extends] or
[`NONE`][fidl-extends]:
 -->
环境支持两种扩展模式，[`REALM`][fidl-extends] 和 [`NONE`][fidl-extends]：

<!--
-   [`REALM`][fidl-extends]: The environment inherits its properties from the
    environment that was assigned to this component (the "parent environment").
    Any new properties will be added on top of those inherited from the parent
    environment. Any properties that overlap with the parent environment will
    override the parent.
-   [`NONE`][fidl-extends]: The environment starts empty, with no initial
    properties.
 -->
-   [`REALM`][fidl-extends]：环境从分配给该组件的环境（“父环境”）继承其属性。任何新属性都将添加到从父环境继承的属性之上。与父环境重叠的任何属性都将覆盖父环境。
-   [`NONE`][fidl-extends]：环境开始时是空的，没有初始属性。

<!--
## Propagation {#propagation}
 -->
## 传播 {#propagation}

<!--
A component instance is assigned an environment in one of two ways:
 -->
组件实例是通过两种方式之一分配环境的：

<!--
-   Its [child][doc-children] or [collection][doc-collections] does not
    have `environment` set. In this case, it will receive its parent's
    environment. This is the most common case.
-   Its [child][doc-children] or [collection][doc-collections] sets
    `environment`, which refers to one of the [`environments`][doc-environments]
    defined by this component.
 -->
-   其[子组件实例][doc-children]或[集合][doc-collections]没有设置 `environment`（环境）。在这种情况下，其将获得其父组件的环境。这是最常见的情况。
-   其[子组件实例][doc-children]或[集合][doc-collections]设置了 `environment`，其引用该组件定义的 [`environments`][doc-environments] 之一。

<!--
The [root component][doc-root-component] is assigned an environment by
[component manager][doc-component-manager]. This includes a bootstrap resolver,
the [ELF runner][doc-elf-runner], and default configuration options.
 -->
[根组件][doc-root-component]由[组件管理器][doc-component-manager]分配环境。这包括引导程序解析器、ELF 运行器和默认配置选项。

<!--
## Environments vs. capability routing {#cap-routing}
 -->
## 环境与能力路由的比较 {#cap-routing}

<!--
The semantics of environments contrast with
[capability routing][doc-capability-routing]. With capability routing, a
capability must be explicitly [exposed][doc-expose] or [offered][doc-offer] by
every component in the path from the provider to the consumer. The explicit
nature of capability routing makes it easy to guarantee that components don't
receive access to capabilities they shouldn't have, thus maintaining the
[principle of least privilege][wiki-least-privilege].
 -->
环境的语义与[能力路由][doc-capability-routing]形成对比。对于能力路由，从提供者到消费者的路径中的每个组件都必须显式地[公开][doc-expose]或[提供][doc-offer]能力。能力路由的显式性质使得很容易保证组件不会获得其不应拥有的能力的访问权限，从而维护[最小特权原则][wiki-least-privilege]。

<!--
However, there are some configuration choices that don't make sense to configure
on a per-component basis. For example, consider [runners][doc-runners]. Almost
every component needs to use a runner, but defining a new runner is not very
common -- certainly less common than defining a protocol capability, for
instance. Furthermore, access to a runner doesn't inherently grant a component
much privilege, for the component framework mediates access to the runner's
protocol and the component can't use that protocol directly. Therefore, runner
capabilities are registered in an environment, which makes them available to any
component in the realm to which that environment was assigned (unless some
sub-realm decides to set a new environment with the runner absent).
 -->
但是，有一些配置选择在每个组件上都进行一次配置没有意义。例如，考虑[运行器][doc-runners]。几乎每个组件都需要使用一个运行器，但是定义新运行器并不常见——比如肯定不如定义协议能力那么常见。此外，访问运行器本身并不会授予组件太多特权，因为组件框架处理对运行器协议的访问，而组件不能直接使用。因此，运行器能力在环境中注册，这使其可用于该环境所分配到的领域中的任何组件（除非某些子领域决定设置一个没有运行器的新环境）。

[glossary.component-url]: /glossary/README.md#component-url
[glossary.component-declaration]: /glossary/README.md#component-declaration
[doc-capability-routing]: ./capabilities/README.md#routing
[doc-children]: https://fuchsia.dev/reference/cml#children
[doc-collections]: https://fuchsia.dev/reference/cml#collections
[doc-component-manager]: ./component_manager.md
[doc-root-component]: ./component_manager.md#booting-the-system
[doc-component-manifests]: ./component_manifests.md
[doc-elf-runner]: ./elf_runner.md
[doc-environments]: https://fuchsia.dev/reference/cml#environments
[doc-expose]: https://fuchsia.dev/reference/cml#expose
[doc-offer]: https://fuchsia.dev/reference/cml#offer
[doc-realms]: ./realms.md
[doc-runners]: ./capabilities/runners.md
[doc-use]: https://fuchsia.dev/reference/cml#use
[elf-runner]: /concepts/components/v2/elf_runner.md
[fidl-directory]: /sdk/fidl/fuchsia.io/directory.fidl
[fidl-extends]: /sdk/fidl/fuchsia.component.decl/environment.fidl
[wiki-least-privilege]: https://en.wikipedia.org/wiki/Principle_of_least_privilege
