<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0118" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Summary

This RFC proposes a stopgap to allow for better visibility, understanding, and
scalability of Software Delivery (SWD) policy configurations by defining well
understood and scoped policies so that any developer can both understand what
software can and cannot be run on a device running Fuchsia and be sure that
they are not accidentally shipping a less secure configuration.

The current proposal is intended to exist in its current form until
sub-assembly based Product Assembly allows for more clearly defined policies
for different product use-cases. The intermediary work in this proposal will
allow for an easier transition to the system assembly-style configuration, as
the work to centralize the application of SWD policy will already have been
done.

## Motivation

SWD configuration is integral in enforcing verified execution. Currently,
SWD configuration is split across several configuration files and packages that
interact with each other to determine the actual SWD configuration on the
device. This RFC aims to reduce the complexity of understanding the SWD state
for a product. Adding strongly understood and deliberate policies also makes
adding new products and variants simpler and safer, easing the burden of
ensuring that the correct configurations are applied.

## Design

Three SWD policies are currently proposed as part of this RFC. They aim to
formalize the SWD policy configurations with semantic rather than
product-driven naming. These policies are to be applied to product definitions
to define SWD state in a more holistic fashion, and
[product owners][glossary.product-owner] will select one of the available
policies rather than tune each individual configuration setting.

As need arises, we may add more policies, but this set of policies is expected
to remain much smaller than the growth of product and build variants.

### Policy Definitions

#### Base Components Only

All executable code must be directly verified (in base). All configurable SWD
restrictions are set to their default secure state: dynamic configuration of
repositories is not allowed, and executability restrictions are on, meaning
that only base and allowlisted components are resolveable and executable.

#### Local Dynamic Config

Permits dynamic configuration of repositories and manual resolution of
indirectly verified code (universe packages/ephemeral components) if and only
if the user has physical access (e.g. a developer).

#### Unrestricted

There are no limitations or restrictions on the executability or visibility of
code. This is the maximally permissive SWD configuration that allows foreign
code and dynamic configuration, and also disables executability restrictions.

### Table of Policies

|POLICY_NAME         |enable_dynamic_configuration|persisted_repos_dir|disable_executability_restrictions|
|--------------------|----------------------------|-------------------|----------------------------------|
|base_components_only|OFF                         |OFF                |OFF                               |
|local_dynamic_config|ON                          |OFF                |OFF                               |
|unrestricted        |ON                          |ON                 |ON                                |

## Implementation

This RFC proposes that this be implemented at the image assembly level. Since
the SWD configuration requires setting values in both `base_packages` and
`system_image_deps`, the proposal is to modify the respective group targets
to include a new dependency on a GN group defined based on the
`policy_labels` build argument, which is a dictionary that stores a set of
key-value pairs of image assembly-understood key values (in this case `swd`)
and their respective policy labels.

This allows us to ensure that there is always exactly one SWD policy
configuration defined for a given build.

As an example, given that policies are defined in
`//build/security/policies_swd.gni`:

```
policies_swd = [
  {
    name = "local_dynamic_config"
    base_package_deps = [ /* ... */ ]
    system_image_deps = [ /* ... */ ]
    bootfs_deps = [ /* ... */ ]
  },
  {
    name = "foo"
    // ...
  },
]
```

In a product definition file, a product owner can do:

```
import("//build/images/policy.gni")  # defines policy_labels = {} arg.
policy_labels.swd = "local_dynamic_config"
policy_labels.foo = "bar"
```

And in the image assembly step:

```
group("base_packages") {
  testonly = base_cache_packages_testonly
  visibility = [ ":*" ]
  public_deps = [
    // ...
  ]
  // Addition for this proposal
  deps = []
  foreach (policy, policies_swd) {
    if (policy.name == policy_labels.swd) {
      deps += policy.base_package_deps
    }
  }
}
```

Note that this does not completely solve the inheritance of product
definitions from ancestor GNI files. It does however simplify the configuration
and auditability of SWD policy.

## Performance

This change does not affect performance.

## Ergonomics

This change improves the ease of understanding and auditing the state of SWD on
a device by abstracting away configuration settings such that SWD configuration
is controlled by a single build argument.

## Backwards Compatibility

This change does not affect backwards compatibility, the changes should be
structured in such a way that they can be easily reverted.

## Security considerations

This change improves the simplicity, scalability, and readbility of SWD
configuration state, which determines which software can be run on a Fuchsia
device. This makes it less likely that a developer will ship a product with an
insecure configuration, directly improving our security posture. This reduced
configuration space also will make it easier for a product-owner to audit.

## Privacy considerations

This change does not affect privacy or interact with user data.

## Testing

This change will be tested to ensure that builds post this change have the
expected SWD policy configurations. This will be done by manually verifying
the output builds have (or do not have) the expected file (for
`disable_executability_restrictions`) and config-data packages (for
`enable_dynamic_configuration` and `persisted_repos_dir`) before and after the
change.

## Documentation

Additional documentation will be added to define the SWD policy configurations.

## Drawbacks, alternatives, and unknowns

This change is intended to be a short term solution until platform
sub-assemblies exist to allow us to have more fine grained control over the
build process. This work is captured in in-progress RFCS for sub-assemblies
(fxr/553664) and structured configuration (fxr/549661).

The main caveat of this change is that this abstracts package/dependency
inclusion away from the product definitions.

Alternatives that were investigated were:

### Not making any changes

We could continue to live in the current environment where each product can
modify its own configuration, and each product continues to inherit
haphazardly from the union of its ancestors.

But, with the rapidly growing set of Fuchsia products, we made this proposal to
proactively make improvements to the SWD configuration mechanism in order to
head off possible incidents where the incorrect SWD state was applied to the
product, subverting core security assumptions of the Fuchsia platform.

### Modifying product definitions

This is similar to how it is done currently, but instead of having the various
settings defined haphazardly across the inheritance tree of product
definitions, we would define the settings at each level. This was determined to
be infeasible as it both put the onus of knowing what each knob did on the
developer, and would not scale to the ever growing set of products. This also
has the added detriment that any changes to the root/ancestral levels of the
tree would necessarily require changes to all child levels to unset or reset
the associated components.

### Asserting at build verification time

We could offload the problem of determining whether a build was using the
correct configuration to build verification, asserting that the correct
dependencies were associated with each build. This solves the problem of
ensuring that a built image has the correct configuration, but does not solve
the audibility/visibility problem, nor does it improve the ergonomics of
understanding the SWD stack's behavior in a given configuration.

### Modifying/reducing the set of SWD configuration settings

We investigated whether or not it was possible to modify SWD logic in order
to reduce the number of settings to change down to a single value.
Unfortunately, this was deemed to be infeasible as the SWD stack is implemented
in multiple components across multiple packages and the zbi, with configuration
spread across all of them.

[glossary.product-owner]: /glossary/README.md#product-owner
