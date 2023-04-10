# Component configuration

It is often useful to tailor the behavior of a
[component instance][glossary.component-instance] to the context it's running
in. In Fuchsia this is known as "component configuration". This page defines
component configuration and discusses some of the characteristics of different
situations that require configuration.

For details on the different mechanisms Fuchsia provides for component
configuration and which mechanism to use to solve each problem, see
[component configuration mechanisms][config_mechanisms].

## What is component configuration? {#what-is-component-configuration}

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

Components consume a wide range of different inputs. Most of these inputs
potentially alter the behavior of the component but only some inputs should be
considered "component configuration data" rather than the more general "data",
such as some state of the system or some user input. The line between component
configuration data and other forms of data can be blurry but the distinction is
important because mechanisms designed for component configuration data often do
not work well when applied to other situations. Exceptions exist in every case,
but configuration values are:

- Usually constant during the lifecycle of a component instance
- Usually constant across some set of devices
- Usually set by developers, maintainers, or administrators rather than by end
  users.

The following are typical examples of component configuration:

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

The following are indications that data is not actually component configuration
data

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

## What are the types of component configuration situation?{#types-of-situation}

Each situation that requires component configuration is different. This section
walks through some key questions to ask yourself when investigating a
situation that requires configuration. The answers to these questions will help
you select an appropriate configuration mechanism.

### Who sets the data? {#who-sets}

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

The same configuration data might need to be set by more than one of these
actors and it might be set by different actors in different situations. For
example, a feature might be disabled by a product integrator in one product but
settable by administrators in a different product.

### When is the data fixed? {#when-is}

- **Fixed at release**. If configuration data can only be changed by a component
  developer or product integrator (or fleet manager in some circumstances) then
  it will be fixed when the product is released. This means the release process
  can verify the configuration before signing. For example, the Fuchsia team can
  verify that a debug option is always disabled in production releases.
- **Runtime modifiable**. Configuration data that can be changed by a device
  administrator or end user (or fleet manager in some circumstances) must be
  able to be changed while the device is running.

The same configuration data might be fixed at release in some products or build
types but runtime modifiable in others.

### How many components use the data? {#how-components}

- **One component**. In most cases only one component will need to consume the
  configuration data. That component's developers can define the data and if
  needed the configuration can be tightly coupled to the component
  implementation.
- **Multiple components**. In some cases multiple components need to share the
  same configuration data, for example several different components may need to
  know the set of approved SSL root keys.

### Does configuration vary across instances of a component? {#does-configuration}

- **No**. Here there is only one instance of a component or there are multiple
  component instances that always use the same configuration values. For
  example, all component instances on a device that read the board architecture
  should receive the same value.
- **Yes**. In the more complex case, different configuration values need to be
  provided to different instances of the same component. This often occurs in
  integration tests. For example, a timeout value may need to be lower when a
  component instance runs in an integration test than when it runs in
  production.

### How large is the data? {#how-large}

- **Small**. The configuration data for most components is small or moderately
  sized; a few bytes to a few tens of kilobytes. A typical example is a
  component that takes a handful of integers to configure its performance plus a
  few dozen booleans to enable experiments or features.
- **Large**. In some cases configuration data is significantly larger and is
  measured in megabytes. For example, calibration maps for a sensor or
  parameters for a large ML model.


[glossary.component-instance]: /docs/glossary/README.md#component-instance
<!-- TODO(fxbug.dev/104819): Update link once better documentation exists -->
[rfc0115]: /docs/contribute/governance/rfcs/0115_build_types.md

[board]: /docs/development/build/build_system/boards_and_products.md#boards
[channel]: /docs/concepts/packages/ota.md#update-omaha
[config_mechanisms]: /docs/development/components/configuration/mechanisms.md
[product]: /docs/development/build/build_system/boards_and_products.md#products
