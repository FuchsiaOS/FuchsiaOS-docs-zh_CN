# Structured Configuration

 * Project lead: jsankey@google.com
 * Area(s): Components

## Problem statement

Software is more flexible and reusable if it can be "configured"; that is, if
aspects of its behavior can be controlled externally rather than being fixed by
its source code.

Fuchsia is intended for use in large scale production environments across a
wide range of products so there are many reasons configuration might be
necessary. For example:

 * A product engineer tailoring operation of a platform component to meet the
   needs of the product.
 * A software engineer enabling additional diagnostics or faster timeouts
   during an automated platform test.
 * A release manager enabling a new feature on a product's beta channel before
   the production channel.
 * A user researcher A/B testing a proposed UI change on a small fraction of a
   product's production fleet.
 * An enterprise administrator constraining the functionality of devices owned
   and managed by the enterprise.
 * A software developer changing system parameters while developing an
   application in order to explore failure modes.
 * An end user customizing their device to enable a pre-release feature.

Note that these examples have very different properties. Some are "static",
applying the configuration when a package is built or a system is assembled,
others are "dynamic", applying the configuration without any change in the
system image. Some imply a change that should persist across power cycles while
in others change should be only temporary. Some should be possible across all
products while others are only meaningful in the context of a single product.

Other platforms offer one or more flexible configuration mechanisms to meet
this wide range of needs. For example, Chrome has Prefs, Settings, Features,
Switches & Flags

On Fuchsia today, configuration of platform components is performed primarily
through two mechanisms:

 * `config_data`. Build creates a single `config_data` package containing a
   directory for each package containing a configurable component. These
   components can read their own configuration files from this package at
   runtime.
 * `#defines`. Some software is conditionally compiled based on build system
   arguments (e.g. `auto_update_packages`). These arguments can be varied
   across products.

Some platform services also expose FIDL interfaces to control their
configuration and may persist this data across power cycles using isolated
persistent storage or stash.

These mechanisms have been adequate for handling simple configuration cases
across a small number of products but are not amenable to dynamic or
multi-layered configuration and are already causing a number of pain points.
For example:

 * Configuration is set at the system image level and so even minor differences
   in configuration have required the addition of new products (e.g.,
   `_eng_arrested` products)
 * Configuration is set at the system image level and so (given the current
   lack of out of tree assembly) can't be managed out-of-tree
 * Configuration is fixed for a system image so production images cannot be
   directly debugged. For example, a "user" image cannot be combined with
   different flags and re-signed with developer keys to enable ssh on a
   dev-keyed device.
 * Some configuration occurs during compilation which limits the reuse of
   precompiled artifacts across products.
 * Some areas have introduced domain-specific workarounds for the lack of
   platform-level configuration (e.g. modular config)
 * Non-trivial migrations are dangerous since all devices in a channel will
   migrate at the same time. Safely launching places significant burden on the
   component developer to support parallel operation and conduct a dark launch
   (e.g. roughtime to HTTPSDate migration)
 * The `config_data` system was created for CFv1 and a model of one component
   per package. Support in CFv2 is possible using the "directory subdir"
   feature but this is labor intensive and error prone.

## Solution statement

This project is still in its early stages and further work is required to
finalize the requirements and the solution.

Files delivered to components through the `config_data` package are opaque to
the platform - different components use different file formats and the data can
be arbitrarily complex. The Fuchsia platform has no way of knowing what data a
component expects or accepts so the platform cannot validate the data or
combine elements from different sources. Accepting opaque configuration data
from untrusted sources would raise a number of security concerns.

We believe a new type of "structured configuration data" is necessary to
complement the existing "opaque configuration data".  This structured
configuration data should have the following properties:

 * Each component clearly defines the data elements it expects
 * The permitted values for each data element are clearly defined: Booleans,
   enumerations, and bounded integers would be sufficient in many cases.
 * The platform assembles structured configuration data from a variety of
   sources supporting both static and dynamic configuration.
 * Sources are able to prohibit certain changes at "higher" layers, for example
   a product configuration should be able to prevent dynamic rollout of an
   incompatible feature. 
 * The platform delivers structured configuration data to components (e.g.
   through a runner) independently of the source that set the data, i.e.
   components do not know whether a particular data element was configured
   dynamically or statically.
 * Dynamic configuration is available to diagnostics so bugs and system metric
   changes can be attributed to the experiments and partial rollouts that
   caused them.

We expect work over the next quarter will involve the following phases.

### Phase 1

Clearly define the potential sources of configuration, the relationships
between these, and the key attributes used to describe configuration changes.
Agree which combinations are supported in the short and long term and which are
the recommended mechanisms for implementing each. This includes describing the
boundaries between the different forms of configuration, for example when
should a configuration be handled by the system settings service.

The exercise will better define the gap that structured configuration must
fill. The end result should feel similar to the Chrome reference page for
configuration and should be published on fuchsia.dev.   

### Phase 2

Define and prioritize the requirements that structured configuration must
fulfil both in the short term and the eventual future. Work with 1-2 launch
customers to agree schedules and needs for 2021. Likely candidates are
supporting the migration of existing customers away from modular (and hence
away from modular config) or expanding the types of configuration possible in
scalable product assembly.

Open questions for this phase are largely related to scope. For example:

 * Do v1 components or non-componentized drivers need to be supported?
 * Should component manager be configured using the same system as the
   components it manages?
 * Should it be possible to configure different instances of the same component
   differently?
 * Is persistence of dynamic configuration necessary in the short term?

### Phase 3

Propose and agree on a technical solution meeting these requirements through
the RFC process.

## Dependencies

At this stage the structured configuration project does not depend on any other
ongoing projects.

It is likely that the other projects will choose to depend on structured
configuration to enable some new capability or migration. For example,
migrating an existing product to session framework might depend on structured
configuration to replace modular configuration.

Depending on scope and schedule, structured configuration might only be
supported for component framework v2 components. Potentially this would create
a transitive dependency from projects wishing to use structured configuration
to component v2 migration or drivers as components.

## Risks and mitigations

The solution is not yet fully defined and additional risks may emerge during
this process.

Currently the primary risk appears to be around schedule; will structured
configuration be ready in time to meet the needs of the first customers? This
risk could be partially mitigated by adding more people to the project once a
technical solution has been agreed.
