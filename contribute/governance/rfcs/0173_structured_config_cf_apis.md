<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0173" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
{# Fuchsia RFCs use templates to display various fields from _rfcs.yaml. View the #}
{# fully rendered RFCs at https://fuchsia.dev/fuchsia-src/contribute/governance/rfcs #}

<!-- mdformat on -->

## Summary

Changes to Component Framework APIs and implementation for structured
configuration.

## Motivation

Structured configuration was approved in [RFC-0127] which described an overall
architecture and roadmap for the feature, but did not specify a number of
implementation details that are necessary to ratify under the
[Component Framework RFC criteria][cf-criteria]. [RFC-0146] describes the CML
syntax used to declare a configuration schema and [RFC-0158] describes the
client libraries users will generate for accessing their configuration.

The initial prototype of structured configuration is available in fuchsia.git.
The Component Framework team is waiting to make the feature generally available
out-of-tree until the APIs and behaviors in this RFC are ratified.

[RFC-0127]: /contribute/governance/rfcs/0127_structured_configuration.md
[cf-criteria]: /contribute/governance/rfcs/0098_component_framework_rfc_criteria.md
[RFC-0146]: /contribute/governance/rfcs/0146_structured_config_schemas_in_cml.md
[RFC-0158]: /contribute/governance/rfcs/0158_structured_config_accessors.md

## Stakeholders

_Facilitator:_ leannogasawara@google.com

_Reviewers:_

* geb@google.com (Component Framework)
* jsankey@google.com (RFC-0127 author)

_Consulted:_ hjfreyer@google.com, xbhatnag@google.com, aaronwood@google.com,
mcgrathr@google.com, shayba@google.com

_Socialization:_ This RFC is the product of design reviews with the Component
Framework team, subsequent code reviews during the prototyping process, and
feedback from early adopters of the prototype.

## Scope

This RFC describes changes to Component Framework to implement "Phase 1" and
parts of "Phase 2" from RFC-0127's [implementation plan][127-impl], covering
only the "static values" functionality and a limited version of "values from
parents" in test environments using [RealmBuilder].

[127-impl]: /contribute/governance/rfcs/0127_structured_configuration.md#implementation
[RealmBuilder]: /development/testing/components/realm_builder.md

## Design

Component Framework has a number of new responsibilities:

* Compiled component manifests include [config value sources](#component-value-sources)
* Component resolvers fetch a component's config values
* Component Manager encodes a component's config as a [persistent FIDL message]
* Component runners deliver the encoded configuration to components
* RealmBuilder allows setting configuration values of launched children in tests

[persistent FIDL message]: /contribute/governance/rfcs/0120_standalone_use_of_fidl_wire_format.md

### Configuration Values

Every component with a [configuration schema][RFC-0146] must have configuration
values defined. These are stored and transferred between parts of the Component
Framework as `fuchsia.component.config.ValuesData`:

```fidl
library fuchsia.component.config;

type ValuesData = table {
    1: values vector<ValueSpec>:MAX;
    2: checksum fuchsia.component.decl.ConfigChecksum;
};

// NOTE: table to allow defining mutability in future revisions
type ValueSpec = table {
    1: value Value;
};

type Value = flexible union {
    1: single SingleValue;
    2: vector VectorValue;
};

type SingleValue = flexible union {
    1: bool bool;
    2: uint8 uint8;
    3: uint16 uint16;
    4: uint32 uint32;
    5: uint64 uint64;
    6: int8 int8;
    7: int16 int16;
    8: int32 int32;
    9: int64 int64;
   10: string string:MAX;
};

type VectorValue = flexible union {
    1: bool_vector vector<bool>:MAX;
    2: uint8_vector vector<uint8>:MAX;
    3: uint16_vector vector<uint16>:MAX;
    4: uint32_vector vector<uint32>:MAX;
    5: uint64_vector vector<uint64>:MAX;
    6: int8_vector vector<int8>:MAX;
    7: int16_vector vector<int16>:MAX;
    8: int32_vector vector<int32>:MAX;
    9: int64_vector vector<int64>:MAX;
   10: string_vector vector<string:MAX>:MAX;
};
```

Values are stored in the same order as the corresponding fields in the compiled
manifest.

Each [config schema][RFC-0146] includes a checksum which is a hash of all field
names and types. The checksum in `ValuesData` must match the checksum in the
compiled component manifest.

We have considered and rejected an [alternative](#typed-values-data) to store
defined values using the same encoding as is sent to the component itself.

### Component value sources

For packaged components, config values are stored in a "configuration value
file". A configuration value file is a persistent FIDL encoding of
`fuchsia.component.config.ValuesData`.

Because the configuration values are not stored in the component manifest,
the Component Framework needs to know where to find the values when resolving
the component. A new field is added to the compiled representation of config
schemas:

```fidl
library fuchsia.component.decl;

type ConfigSchema = table {
    // ...existing fields...

    3: value_source ConfigValueSource;
};

type ConfigValueSource = flexible union {
    /// The path within the component's package at which to find config value files.
    1: package_path string:MAX;
};
```

A flexible union is used to allow for the addition of new config value sources
over time.

By convention, value files are packaged at `meta/${manifest_basename}.cvf`. For
example, if a component's manifest is packaged at `meta/foo.cm`, its value file
will be packaged at `meta/foo.cvf`.

Build rules for producing structured configuration need to ensure that component
manifests include the path at which the component's config will be packaged.
For example, the in-tree GN build achieves this by requiring the config values
target to reference the component target so they can agree about the packaged
location:

```gn
import("//build/components.gni")

# NOTE: results in a package path of `meta/my_component.cm`
fuchsia_component("my_component") {
  manifest = "meta/my_component.cml"
  deps = [ "..." ]
}

# NOTE: internally calls `get_target_outputs(":my_component")` to determine
# the correct packaging path
fuchsia_structured_config_values("my_config_values") {
  component = ":my_component"
  values_source = "config/my_component.json5"
}

fuchsia_package("my_package") {
  deps = [
    ":my_component",
    ":my_config_values",
  ]
}
```

An [alternative](#cm-inline-values) has been considered and rejected to store
the values in the manifest itself.

An [alternative](#ext-inference) has been considered and rejected to infer the
location of the value file within the package based on the packaged path of the
manifest.

An [alternative](#pkg-index) has been considered and rejected to add an index
blob to packages which points to both the component manifest and the value file.

### Component resolution

Component resolvers are responsible for retrieving a component's packaged values
and returning them to Component Manager as a new field in the
`fuchsia.sys2.Component` table:

```fidl
library fuchsia.sys2;

// NOTE: This type is returned by fuchsia.component.resolution.Resolver/Resolve.
type Component = resource table {
    // ... existing fields ...

    /// Binary representation of the component's configuration values
    /// (`fuchsia.component.config.ValuesData`).
    4: config_values fuchsia.mem.Data;
};
```

`ValuesData` is returned as a `fuchsia.mem.Data` to match how component
manifests are returned from resolvers. This allows for the combined size of a
compiled manifest and configuration values to exceed the size of a single
zircon channel message.

This requires component resolvers to parse the component manifest to interpret
the value source in the manifest, while previously resolvers were able to return
the raw bytes directly to Component Manager without understanding the compiled
manifest representation.

### Encoding configuration values

Once Component Manager has a configuration schema and values, it must pass these
values and the component's [config schema][RFC-0146]'s checksum to the
component's runner for provisioning to the component over a runtime-specific
interface.

#### VMO contents

Configuration values are encoded for the component as a persistent FIDL struct
with the fields in the same order as the compiled manifest. A [rejected
alternative](#encode-as-table) would be to encode the final configuration as a
FIDL table rather than a struct. We have also [considered and
rejected](#non-fidl) non-FIDL encodings for the resolved configuration.

This requires Component Manager to understand the FIDL wire format to perform
runtime encoding of messages that can be successfully parsed by generated FIDL
bindings.

Component Manager writes the encoded configuration into a VMO with the following
contents:

* Bytes `0..1` contain the length `N` of the checksum as a little-endian
  integer
* Bytes `2..2+N` contain the checksum
* Bytes `3+N..ZX_PROP_VMO_CONTENT_SIZE` contain a persistent FIDL message with
  the component's configuration values encoded as a struct

The checksum is stored as a variable-length portion of the header to decouple
the VMO encoding from the size of any particular hash function's output.
Changing the hash algorithm used to derive checksums should not require a change
to the VMO encoding.

#### Passing VMO to runners

Once the config VMO has been created, it is passed to the runner as a field of
`fuchsia.component.runner.ComponentStartInfo`:

```fidl
library fuchsia.component.runner;

// NOTE: Passed to fuchsia.component.runner.ComponentRunner/Start.
type ComponentStartInfo = resource table {
    // ... existing fields ...

    /// Binary representation of the component's configuration.
    ///
    /// # Layout
    ///
    /// The first 2 bytes of the data should be interpreted as an unsigned 16-bit
    /// little-endian integer which denotes the number of bytes following it that
    /// contain the configuration checksum. After the checksum, all the remaining
    /// bytes are a persistent FIDL message of a top-level struct. The struct's
    /// fields match the configuration fields of the component's compiled manifest
    /// in the same order.
    7: encoded_config fuchsia.mem.Data;
};
```

We've also considered and rejected alternatives to [defer encoding to the
runner](#runner-encoded) and to [use FIDL to describe the checksum layout in the
VMO](#double-fidl).

### Running components with encoded configuration

Each runner is responsible for establishing a contract with components it runs
to provide them access to the encoded configuration. This contract is exercised
by [accessor libraries][accessors].

[accessors]: /contribute/governance/rfcs/0158_structured_config_accessors.md

#### ELF runner

The ELF runner provides configuration VMOs as a startup handle:

```c++
// in //zircon/system/public/zircon/processargs.h:
#define PA_VMO_COMPONENT_CONFIG 0x1Du
```

#### Driver runner

The driver runner provides configuration VMOs as a field in the
`fuchsia.driver.framework.DriverStartArgs` table:

```fidl
library fuchsia.driver.framework;

// NOTE: Passed directly to a driver upon starting.
type DriverStartArgs = resource table {
    // ... existing fields ...
    7: config zx.handle:VMO;
};
```

#### Test runners

We are not yet implementing structured config support in test runners, as we
have not yet discovered sufficiently-motivated use cases to constrain and guide
a design.

### Overriding configuration values with RealmBuilder

RealmBuilder supports controlling the configuration values for a component it
launches. Users can provide values for individual fields, and by default they
must specify values for all fields in the component's schema. This will
encourage a component's own integration tests to fully enumerate the matrix of
configuration options to test.

RealmBuilder will also allow test authors to load the packaged values for a
component, either using them wholesale or overriding individual fields. This
will allow tests to, for example, enable a particular testing feature that is
not useful outside of a subset of tests, while still using the rest of the
"production" config for that component.

These methods will be added to RealmBuilder:

```fidl
    LoadPackagedConfigValues(struct {
        name fuchsia.component.name;
    }) -> (struct {}) error RealmBuilderError2;

    SetConfigValue(struct {
        name fuchsia.component.name;
        key fuchsia.component.decl.ConfigKey;
        value fuchsia.component.config.ValueSpec;
    }) -> (struct {}) error RealmBuilderError2;
```

RealmBuilder client libraries will be extended to expose this
functionality.

## Implementation

The contents of this RFC as initially proposed have been experimentally
implemented and made available in fuchsia.git, initially under an allowlist with
the understanding from users that the Component Framework may make breaking
changes while iterating. Changes to match this RFC's final design will be landed
before making structured configuration available to out-of-tree customers.

## Performance

The designs in this RFC could impact system runtime performance in terms of time
needed to resolve and start components, which is not a metric that to the
author's knowledge has thus far been a bottleneck on the quality of products
built with Fuchsia. We do have continuous benchmarking of the start time for
some components within the topology, and we will continue monitoring those for
regressions but none have yet been identified in the process of prototyping this
feature.

Structured config may increase system memory usage if copies of config are
held in memory, which this design minimizes by storing configuration in a
VMO which accessor libraries may close after parsing the contents into the
domain-specific types used by each component's implementation.

## Backwards Compatibility

This RFC assumes that [platform versioning][RFC-0002] will have sufficient
runtime support within the Component Framework before we discover any need to
evolve how configuration is encoded into the VMOs passed to components.

RealmBuilder's overrides implementation allows tests in the same package as the
launched component to take a runtime dependency on a component's configuration
schema, and authors of components used in others' tests will need to exercise
caution when changing the types of a configuration field or when removing a
configuration field.

[RFC-0002]: /contribute/governance/rfcs/0002_platform_versioning.md

## Security considerations

Structured configuration may be used to control security-critical features in
components so it is essential that the implementation delivers the right values.

A component's configuration comes from the same source as its manifest, but in
the future we may extend the existing component resolvers or build new resolvers
which allow a looser relationship between the two. We will need to exercise
caution to ensure that a component's configuration is always unambiguous and
auditable.

## Privacy considerations

Per [RFC-0127], structured configuration is not designed to store user-
generated data.

## Testing

The most sensitive part of this design is the dynamic FIDL encoder used by
Component Manager. Its prototype has been integrated as a "language backend" in
FIDL's GIDL conformance suite to ensure that for the subset of types it supports
it produces identically laid-out messages as the statically-typed FIDL bindings.

## Documentation

[Documentation][sc-docs] for the prototype is currently available for in-tree
developers.

[sc-docs]: /development/components/configuration/structured_config.md

## Drawbacks, alternatives, and unknowns

### Alternative: storing values in a typed format {#typed-values-data}

Instead of the untyped `fuchsia.component.config.ValuesData`, we could store
and transfer a component's configuration values between portions of the
Component Framework using a FIDL-encoded payload with a type that matches the
component's schema, similar to the encoding used for the VMO passed directly to
the component.

This would likely result in a slightly more compact on-disk representation, as
well as being aesthetically aligned with the typed payloads that are eventually
delivered to the component.

The design proposed in this RFC allows various tools to understand configuration
without having to reimplement a dynamically-typed FIDL parser. In the future, we
might reconsider this decision if the FIDL toolchain offers more general tools
for "reflection" over messages.

### Alternative: storing values in the manifest {#cm-inline-values}

Storing values inline in the manifest would simplify several elements of the
implementation (removing responsibilities from component resolvers) at the cost
of changed blob hashes for component manifests between products.

### Alternative: finding value files by file extension {#ext-inference}

Instead of adding a value source to component manifests, we could infer from our
packaging convention where to find the value file. This would create an
undesirable runtime dependency on what are currently only conventions.

### Alternative: finding value files by a package-component index {#pkg-index}

Adding a third blob to the package would allow us to pipeline reads of the
manifest and value file, which would result in a cleaner build graph,
simplify some elements of the manifest compilation process, and allow component
resolvers to understand a simpler file format than having to parse the entire
component manifest.

However, this would increase the system image size and would be a significant
user-visible change to how components are resolved.

### Alternative: encode configuration as a FIDL table {#encode-as-table}

Instead of a FIDL struct, configuration fields could be encoded as a table, as
was suggested as an option in [RFC-0127]. Tables have a number of useful
properties for evolution, but they also require a parser to assume that any of
the fields could be omitted. With structured configuration, Component Manager is
always able to supply all fields and there is no need for the caller to handle
the case where no value has been provided.

FIDL structs occupy fewer bytes to carry the same data and are slightly faster
to parse.

### Alternative: use a non-FIDL encoding for configuration values {#non-fidl}

An encoding other than FIDL would be viable, but the FIDL wire format works
well for this use case and choosing a different encoding would be a net increase
in the number of concepts needed to understand Fuchsia. Using the FIDL encoding
aligns with Component Framework's desire to integrate more closely with FIDL
going forward and enables using FIDL's generated bindings as the implementation
for [accessors], which allows structured config to reap the benefits of
performance and binary size optimizations which have already been achieved
there.

### Alternative: runners encode configuration {#runner-encoded}

Instead of encoding the configuration VMO in Component Manager, we could encode
it in each runner. This would allow runners that operate at different levels of
abstraction to deliver configuration in different formats. For example, a runner
for Javascript code could deliver the configuration as objects in that language
runtime rather than requiring each Javascript component to parse a VMO.

However, as noted in [Testing](#testing), FIDL encoding is the most sensitive
part of this design in terms of correctness. There are multiple runners today
which would use a FIDL-based encoding for config, which means that there would
be multiple binaries (and implementing programming languages) responsible for
this sensitive task. Centralizing the responsibility for config encoding in
Component Manager allows us to constrain the implementation and achieve greater
confidence in the overall feature with extensive testing.

The proposed design would make it difficult for a runner to configure its
behavior based on a component's structured configuration values, as each runner
would need to parse the encoded FIDL based on the component's declared schema.
This would be comparable complexity to making each runner responsible for
encoding which is also undesirable. When it comes time to configure runners
from a component's config, we will design a separate interface for passing
values from the component's configuration to the runner. Defining an explicit
interface for runner configuration also has the advantage of insulating a
component's config namespace from Hyrum's Law-style effects.

### Alternative: describe VMO/checksum layout with FIDL {#double-fidl}

Instead of encoding the config checksum as header bytes in the VMO, we could
describe the VMO's ABI in terms of FIDL:

```fidl
type ConfigVmo = struct {
    checksum bytes:MAX;
    contents bytes:MAX;
};
```

This would occupy slightly more space in each config VMO, and would also require
traversing the bytes of `contents` twice.
