<!--
# Component configuration
 -->
# 组件配置

<!--
It is often useful to tailor the behavior of a
[component instance][glossary.component-instance] to the context it's running
in. In Fuchsia this is known as "component configuration". This page defines
component configuration and discusses some of the characteristics of different
situations that require configuration.
 -->
根据[组件实例][glossary.component-instance]的运行的上下文定制其行为通常很有用。在 Fuchsia 中，这称作“组件配置”（component configuration）。本页对组件配置进行定义，并讨论需要配置的不同情况的一些特征。

<!--
For details on the different mechanisms Fuchsia provides for component
configuration and which mechanism to use to solve each problem, see
[component configuration mechanisms][config_mechanisms].
 -->
要获取关于 Fuchsia 为组件配置提供的不同机制，以及使用何种机制来解决每个问题的详细信息，请参阅[组件配置机制][config_mechanisms]。

<!--
## What is component configuration? {#what-is-component-configuration}
 -->
## 组件配置是什么？ {#what-is-component-configuration}

<!--
"Component configuration data" is data supplied to a component instance that it
can use to understand and adapt to the context in which it was launched (such as
the [product][product], [board][board], [build type][rfc0115],
[omaha channel][channel], regulatory region, or integration test realm).
"Component configuration" is the process of defining, delivering, and using this
data. The ability to configure components is useful because it lets the same
component be used in different contexts - for example the same component could
be used across different products, or on different hardware platforms. Without
component configuration a developer would need to create multiple components,
for example "foo_for_product_a", "foo_for_product_b", and "foo_for_testing".
 -->
“组件配置数据”是提供给组件实例的数据，可以用来理解和适应其启动上下文（例如[产品][product]、[板型][board]、[构建类型] [rfc0115]、[Omaha 通道][channel]、监管区域或集成测试领域）。“组件配置”是定义、交付和使用这些数据的过程。配置组件的能力的有用之处在于，允许在不同的上下文中使用相同的组件——例如，可以在不同的产品或硬件平台上使用相同的组件。而如果没有组件配置，那么开发人员将需要创建多个组件，例如“foo_for_product_a”、“foo_for_product_b”和“foo_for_testing”等。

<!--
Components consume a wide range of different inputs. Most of these inputs
potentially alter the behavior of the component but only some inputs should be
considered "component configuration data" rather than the more general "data",
such as some state of the system or some user input. The line between component
configuration data and other forms of data can be blurry but the distinction is
important because mechanisms designed for component configuration data often do
not work well when applied to other situations. Exceptions exist in every case,
but configuration values are:
 -->
组件使用的输入范围广泛。其大多数输入可能会改变组件的行为，然而只有一部分输入应视为“组件配置数据”，而非更一般的“数据”，例如系统的某些状态或某些用户输入。组件配置数据和其他形式的数据之间的界限可能很模糊，但二者的区分很重要，因为为组件配置数据设计的机制在应用于其他情况时通常效果不佳。每种情况都存在例外，但配置值：

<!--
- Usually constant during the lifecycle of a component instance
- Usually constant across some set of devices
- Usually set by developers, maintainers, or administrators rather than by end
  users.
 -->
- 通常在组件实例的生命周期内为常量
- 通常在某些设备间为常量
- 通常由开发人员、维护人员或管理员设置，而非由最终用户设置。

<!--
The following are typical examples of component configuration:
 -->
以下是组件配置的典型示例：

<!--
- Feature flags - Enabling or disabling some functionality of a component using a
  boolean configuration. This is often useful for new features that may need to
  be disabled rapidly if problems are encountered. For example, a feature flag
  was used in 2022 to safely enable the use of Pinweaver to encrypt account data
  partitions.
- Board tuning - Modifying the behavior of a component to suit the board it is
  running on. For example, supplying the median error and error bound for the
  CPU clock.
- Product tuning - Modifying the behavior of a component to suit the product it
  is running on. For example, specifying which session component Session Manager
  should start.
- Test control - Specify different behavior when a component is used in testing.
  For example, setting a much faster retry timeout when a component is used in
  integration tests to reduce the time required to run the test.
- Debug control - Enabling or disabling additional component diagnostics to aid
  in debugging. For example, enabling an administrative FIDL interface in `eng`
  builds but not in `user` builds.
 -->
- 功能标志（feature flag）——使用布尔配置来启用或禁用组件的某些功能。在遇到问题时，这对于可能需要快速禁用的新功能很有用。例如，2022 年使用了一个功能标志安全地启用 Pinweaver 以加密帐户数据分区。
- 板型调优（board tuning）——修改组件行为，以适应其运行板型。例如，为 CPU 时钟提供中值误差和误差范围。
- 产品调优（product tuning）——修改组件行为，以适应其运行产品。例如，指定会话管理器应当启动哪个会话组件。
- 测试控制（test control）——在测试中使用组件时指定不同行为。例如，当组件用于集成测试时，设置更快的重试超时时间以减少运行测试所需时间。
- 调试控制（debug control）——启用或禁用其他组件诊断以帮助调试。例如，在 `eng` 构建中启用管理 FIDL 接口，但在 `user` 构建中不启用。
- 
<!--
The following are indications that data is not actually component configuration
data
 -->
以下迹象表明数据实际上并非组件配置数据

<!--
- **The data is modified by the component itself**. Components that change some
  configurable state (say in response to FIDL requests) must rationalize those
  changes with changes in the configuration input. In these cases there are
  usually two similar but distinct states which must be defined: a component
  configuration state, say "default_foo" and a system state, say "foo". The
  component initially sets foo equal to default_foo but the two may then change
  independently. The component owns the state of foo but the configuration
  system owns the state of default_foo.
- **A component instance uses different data for each component it interacts with**.
  If a server supports connections from different clients and lets each
  client tailor the interaction, the configuration is "connection configuration"
  not "component configuration". The mechanisms discussed here aren't intended
  to solve this case, but as above there may still be a component configuration
  state to define the "default" that is used for new connections.
- **The data changes frequently and rapidly at runtime**. Component
  configuration data reflects the environment that a component instance launched
  in. Most often these environments are constant but in some cases the
  environment, or the configuration data associated with an environment, might
  change at runtime. For example, a user may fly to a different regulatory
  region or a product may enable a new feature. However, these runtime changes
  are still much less frequent than changes in many system states and the
  mechanisms discussed are designed with this low rate of change in mind.
 -->
- **数据由组件自身修改**。组件更改某些可配置状态（比如 FIDL 请求响应）时，必须合理化地更改配置输入。在这些情况下，通常必须定义两个相似但不同的状态：组件配置状态（如“default_foo”）和系统状态（如“foo”）。组件最初将 foo 设置为 default_foo ，但随后两者可能会独立更改。该组件拥有 foo 的状态，但配置系统拥有 default_foo 的状态。
- **组件实例为与其交互的每个组件使用不同的数据**。如果服务器支持来自不同客户端的连接，并让每个客户端定制交互，那么该配置是“连接配置”，而非“组件配置”。这里讨论的机制并非为了解决这种情况，但如上所述，仍可能有一个组件配置状态来定义用于新连接的“默认”。
- **数据在运行时频繁快速变化**。组件配置数据反映了组件实例在其中启动的环境。大多数情况下，这些环境是不变的，但在某些情况下，环境或与环境关联的配置数据可能在运行时发生变化。例如，用户可能穿梭至不同的监管区域，产品可能启用新功能。然而，这些运行时更改仍然比许多系统状态中的更改频率低得多，并且所讨论的机制在设计时就考虑到了这种低变化率。

<!--
## What are the types of component configuration situation?{#types-of-situation}
 -->
## 组件配置的情况是什么类型？ {#types-of-situation}

<!--
Each situation that requires component configuration is different. This section
walks through some key questions to ask yourself when investigating a
situation that requires configuration. The answers to these questions will help
you select an appropriate configuration mechanism.
 -->
每种需要组件配置的情况都不同。本节介绍了在调查需要配置的情况时需要自问的一些关键问题。这些问题的答案将帮助您选择合适的配置机制。

<!--
### Who sets the data? {#who-sets}
 -->
### 该数据由谁设置？ {#who-sets}

<!--
- **Component developers**. Here the developers of the component supply
  configuration values. For example one set of values for testing and a
  different set for production, or different sets of values for each build type.
- **Product integrators**. Here the developers responsible for integrating a
  component with a particular product or board supply configuration values based
  on that product or board. These might be the same people that developed the
  component.
- **Fleet managers**. Here the team managing a fleet of devices supplies
  configuration values. For example, disabling a feature flag if there are
  problems with a rollout.
- **Device administrators**. Here the person or organization administering a
  device supplies values. For example, enabling a new experimental feature. For
  development devices the administrator is the developer using the device. If a
  product based on Fuchsia supported enterprise use cases, the enterprise that
  owned a device might act as the device administrator.
- **End Users**. Here the end user of the device supplies values, for example
  setting the device's region during a setup flow.
 -->
- **组件开发人员**。此处组件开发人员提供配置值。例如，一组值用于测试，另一组用于生产，或者不同组别用于每种构建类型。
- **产品集成人员**。此处负责将组件与特定产品或板型集成开发人员，根据其产品或板型提供配置值。其可能也是组件开发人员。
- **队列管理人员**。此处管理设备队列的团队提供配置值。例如，如果发布出现问题，则禁用一个功能标志。
- **设备管理员**。此处管理设备的个人或组织提供配置值。例如，启用新的实验性功能。对于开发设备，管理员是使用设备的开发人员。如果基于 Fuchsia 的产品支持企业用例，那么拥有设备的企业可以充当设备管理员。
- **最终用户**。此处设备的最终用户提供配置值。例如，在设置流程中设置设备的区域。

<!--
The same configuration data might need to be set by more than one of these
actors and it might be set by different actors in different situations. For
example, a feature might be disabled by a product integrator in one product but
settable by administrators in a different product.
 -->
相同配置数据可能需要由多个上述角色设置，并且可能由不同角色在不同情况下设置。例如，某项功能可能在一个产品中被产品集成人员禁用，但在另一个产品中可由管理员设置。

<!--
### When is the data fixed? {#when-is}
 -->
### 该数据何时固定？ {#when-is}

<!--
- **Fixed at release**. If configuration data can only be changed by a component
  developer or product integrator (or fleet manager in some circumstances) then
  it will be fixed when the product is released. This means the release process
  can verify the configuration before signing. For example, the Fuchsia team can
  verify that a debug option is always disabled in production releases.
- **Runtime modifiable**. Configuration data that can be changed by a device
  administrator or end user (or fleet manager in some circumstances) must be
  able to be changed while the device is running.
 -->
- **发布时固定**。配置数据如果只能由组件开发人员或产品集成人员（或某些情况下为队列管理人员）更改，那么会在产品发布时固定。这意味着发布过程可以在签名之前验证配置。例如，Fuchsia 团队可以验证某个调试选项在生产版本中始终处于禁用状态。
- **运行时可修改**。可由设备管理员或最终用户（或某些情况下为队列管理人员）更改的配置数据必须能够在设备运行时更改。

<!--
The same configuration data might be fixed at release in some products or build
types but runtime modifiable in others.
 -->
同样的配置数据，可能在某些产品或构建类型中在发布时固定，但在其他产品或构建类型中可在运行时修改。

<!--
### How many components use the data? {#how-components}
 -->
### 有多少组件使用该数据？ {#how-components}

<!--
- **One component**. In most cases only one component will need to consume the
  configuration data. That component's developers can define the data and if
  needed the configuration can be tightly coupled to the component
  implementation.
- **Multiple components**. In some cases multiple components need to share the
  same configuration data, for example several different components may need to
  know the set of approved SSL root keys.
 -->
- **一个组件**。大多数情况下，只有一个组件需要使用配置数据。该组件的开发人员可以定义数据，且如果需要，可将配置与组件紧密耦合。
- **多个组件**。某些情况下，多个组件需要共享相同的配置数据，例如，几个不同的组件可能需要知道一组已批准的 SSL 根密钥。

<!--
### Does configuration vary across instances of a component? {#does-configuration}
 -->
### 配置在组件的不同实例间有差异吗？{#do-configuration}

<!--
- **No**. Here there is only one instance of a component or there are multiple
  component instances that always use the same configuration values. For
  example, all component instances on a device that read the board architecture
  should receive the same value.
- **Yes**. In the more complex case, different configuration values need to be
  provided to different instances of the same component. This often occurs in
  integration tests. For example, a timeout value may need to be lower when a
  component instance runs in an integration test than when it runs in
  production.
 -->
- **无**。此处仅有一个组件实例，或者有多个组件实例始终使用相同的配置值。例如，设备上所有读取板型架构的组件实例都应收到相同的值。
- **有**。在更复杂的情况下，需要向同一组件的不同实例提供不同的配置值。这通常发生在集成测试中。例如，相比生产中，当组件实例在集成测试中运行时，超时时间值可能需要减小。

<!--
### How large is the data? {#how-large}
 -->
### 该数据大小如何？ {#how-large}

<!--
- **Small**. The configuration data for most components is small or moderately
  sized; a few bytes to a few tens of kilobytes. A typical example is a
  component that takes a handful of integers to configure its performance plus a
  few dozen booleans to enable experiments or features.
- **Large**. In some cases configuration data is significantly larger and is
  measured in megabytes. For example, calibration maps for a sensor or
  parameters for a large ML model.
 -->
- **较小**。大多数组件的配置数据较小或为中等大小，几个字节到几十个千字节。一个典型的例子是，一个组件需要一些整型数据来配置其性能，以及几十个布尔数据来启用实验或功能。
- **较大**。某些情况下，配置数据明显较大，需要用兆字节度量。例如，用于大型机器学习模型的传感器或参数的校准图。


[glossary.component-instance]: /glossary/README.md#component-instance
<!-- TODO(fxbug.dev/104819): Update link once better documentation exists -->
[rfc0115]: /contribute/governance/rfcs/0115_build_types.md

[board]: /development/build/build_system/boards_and_products.md#boards
[channel]: /concepts/packages/ota.md#update-omaha
[config_mechanisms]: /development/components/configuration/mechanisms.md
[product]: /development/build/build_system/boards_and_products.md#products
