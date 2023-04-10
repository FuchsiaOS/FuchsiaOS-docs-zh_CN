# Component configuration mechanisms

The [component configuration](/docs/concepts/components/configuration.md) page
introduces how components can be configured on Fuchsia, and describes different
situations that require
[component configuration][glossary.component-configuration].

This page introduces the different configuration mechanisms that Fuchsia
provides, talks about their strengths and weaknesses and provides guidance on
which mechanism to use in which situation.

## Structured configuration {#structured-configuration}

[Structured configuration][structured_config] is a modern, ergonomic, and
flexible configuration system provided by the Component Framework. Structured
configuration is integrated with many Fuchsia tools including [ffx][ffx],
[inspect][inspect], and product assembly and it should be your first choice in
most cases.

A component developer uses structured configuration by defining configuration
keys in the component's manifest, each with a name and datatype. The system
guarantees that values will be provided for all of these configuration keys when
the component is started. Libraries are auto-generated to easily read
configuration values at runtime. Components need not care what part of the
system supplied a structured configuration value and need not handle missing
configuration.

Structured configuration values can be provided through a range of different
paths, such as:

- **Package build rules**. Configuration values set by build rule are included
  in the same package as the component. The configuration is fixed at build time
  so can be verified during release signing.
- **Realm builder**. [Realm builder][realm_builder] integration lets tests
  easily set configuration values. The two main use cases are to verify
  configurable behavior and to change configuration to aid testability.
- **Product assembly**. Product assembly lets the platform define configuration
  keys whose values are supplied by the product. Product integrators supply
  values in their product configuration and these are built into the component
  package where they can be verified during release signing.

[RFC-0127][rfc0127] described several other ways to specify structured
configuration values, but these have not yet been built pending near-term use
cases. If your use case requires any of these features, please comment on the
tracking bug linked below for the feature:

- **Development overrides ([fxbug.dev/96260][bug.96260]**). This would let
  developers change configuration at runtime on engineering builds, for example
  to use pre-release features.
- **Vbmeta ([fxbug.dev/96261][bug.96261]**). This would let signed configuration
  be modified for a release without building a new image.
- **Parent component ([fxbug.dev/96254][bug.96254]**). This would let a
  component specify configuration for children in a dynamic collection as it
  creates those children. This requires a consistent definition of the
  configuration between the two components so may only be supported between
  components in the same image or package.
- **Fleet and enterprise management ([fxbug.dev/104596][bug.104596])**. This
  would let structured configuration be used for incremental rollouts, running
  A/B experiments and enterprise policy enforcement, similar to the experiment
  systems in other large platforms.

Structured configuration definition for a component is local to that component.
There is no global "configuration version" and the system does not provide
guarantees on forwards or backwards compatibility across versions of a
component — compatibility is left to the component developer. Therefore,
structured configuration is not currently suitable as a means of communication
between components that may have been built against different configuration
versions.

Structured configuration is very flexible and works in most situations, however
there are configuration problems for which structured configuration is not the
best solution:

- **Configuration values must be consistent across a large number of different components**.
  Structured configuration keys are defined locally in a component's manifest.
  Product assembly may be used to set consistent configuration values across a
  few different components but this brings maintainability challenges and does
  not scale well. If configuration values must be consistent across a large
  number of different components,
  [package-based configuration](#package-based-configuration) or
  [service-based configuration](#service-based-configuration) are better
  solutions.
- **Large configuration**. When configuration gets very large, say over several
  kilobytes, the tools that structured configuration provides to set and view
  configuration values become inconvenient. The implementation of structured
  configuration does not prioritize efficient handling of large values.
  [Package-based configuration](#package-based-configuration) is a better
  solution for large configuration data.
- **Configuration that changes frequently at runtime**. A component receives its
  structured configuration values as it launches, meaning a component instance
  must be restarted to pick up new configuration.
  [Service-based configuration](#service-based-configuration) is a better
  solution for configuration that must change frequently at runtime.
- **Configuration that is set by end users**. Unlike developers, end users need
  a user interface to control configuration. This user interface means that the
  UI component, the component being configured, and the localization database
  all need to agree on the definition of the configuration and therefore the
  configuration must be defined centrally in some versioned artifact.
  [Service-based configuration](#service-based-configuration) configuration is a
  better solution for configuration that is controlled by end users.

## Package-based configuration {#package-based-configuration}

Package-based configuration supports configuration data that is fixed at release
time by building data files into a [package][glossary.package] and letting the
component read those files. The same package can be routed to several different
components and the configuration files can be arbitrarily large.

Package based configuration is generally less ergonomic than structured
configuration; the files must be manually opened and parsed, the component may
need to handle missing or malformed configuration, and there is no integration
with standard tooling to test, debug, or introspect configuration.

Package based configuration does not work well for the following problems:

- **Configuration needs to be altered at runtime**. Comment on the tracking bugs
  above for dynamic configuration support in structured configuration and/or use
  [service-based configuration](#service-based-configuration) instead.

There are three different variants of package-based configuration based on which
package is used to store the configuration:

### Global (aka "config_data") package configuration

A single global [`config_data`][config_data] package contains configuration data
for many different components. With the data for each component being added
using the "config_data" build rule. This global package was the only way to use
packaged-based configuration in CFv1 but it is inflexible and cumbersome in CFv2
since the config-data directory must be manually routed through the component
topology.

Users of config_data should migrate to either
[structured configuration](#structured-configuration) or
[domain](#domain-package-configuration)/[in-package](#in-package-configuration)
configuration, both of which provide better ergonomics and more flexibility.

### Domain package configuration {#domain-package-configuration}

The developers of the components in a domain can define their own configuration
data package and route its directories to the components they control. Unlike
global package configuration, this mechanism can be used across different build
systems and petals.

### In-package configuration {#in-package-configuration}

Configuration files may also be placed in the same package as the component they
configure. This ensures that the configuration will be distributed with the
component and does not require complex and brittle routing. However, it also
means that the configuration cannot be changed without repackaging the
component. More details are provided in the guide on
[providing data files to components][providing_data_files].

Packages contents are stored in [blobfs][blobfs], a content addressed file
system which de-duplicates blobs. This means if multiple packages include an
identical configuration blob, only one copy will be stored.

## Service-based configuration {#service-based-configuration}

Fuchsia components may be written to collect, maintain, and distribute
configuration data for a domain over FIDL. A good example is the Fuchsia
platform’s [settings service][settings_service] that maintains various user
settings such as the locale: the settings service maintains the state of these
user settings across power cycles and exposes `fuchsia.settings` FIDL protocols
that other components may use to read the current configuration data or request
a change in configuration data.

Service-based configuration can be very flexible and powerful but introducing a
server component is generally more work than the other options and incurs a
higher runtime cost. Service-based configuration should only be used when this
additional cost adds value.

The component can potentially use several different inputs to determine the
correct configuration. For example: business logic to validate and combine
client requests, settings collected from some server, or default configuration
values provided through some other mechanism such as structured configuration.
Service-based configuration works well when configuration must be shared across
a wide range of clients because the configuration is formally defined and
versioned by the FIDL interface used to deliver it. Service-based configuration
also works well when configuration potentially changes frequently.

Service-based configuration does not work well for the following problems:

- **Large Configuration**. Configuration is delivered by FIDL messages which are
  limited by the size of a zircon channel write. Although a FIDL protocol could
  work around this by splitting configuration into chunks or sending a VMO this
  would harm the client simplicity and explicit definition that are
  service-based configuration's strength.
  [Packaged-based configuration](#package-based-configuration) is a better
  solution when working with large configuration.


[glossary.component-configuration]: /docs/glossary/README.md#component-configuration
[glossary.package]: /docs/glossary/README.md#package

[bug.96260]: https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=96260
[bug.96261]: https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=96261
[bug.96254]: https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=96254
[bug.104596]: https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=104596

[rfc0127]: /docs/contribute/governance/rfcs/0127_structured_configuration.md

[blobfs]: /docs/concepts/filesystems/blobfs.md
[config_data]: config_data.md
[ffx]: https://fuchsia.dev/reference/tools/sdk/ffx
[inspect]: /docs/development/diagnostics/inspect/README.md
[realm_builder]: /docs/development/testing/components/realm_builder.md
[settings_service]: https://cs.opensource.google/fuchsia/fuchsia/+/main:src/settings/service/
[structured_config]: structured_config.md
[providing_data_files]: /docs/development/components/data.md