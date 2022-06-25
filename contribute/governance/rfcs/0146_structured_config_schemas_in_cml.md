<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0146" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Summary

This document builds on [RFC-0127][rfc-0127] by capturing the design, implementation strategy and
other decisions made regarding structured config schemas in CML. This proposal will provide the
JSON syntax of config schemas, a set of FIDL types for encoding config schemas in component
manifests, and a process to convert config schemas from JSON to FIDL.

Note that “config schema" is the concrete implementation of the “configuration definition file”
defined in [RFC-0127][rfc-0127].

## Motivation

Developers need a way to declare a configuration schema for their component. The process for
parsing that configuration schema, compiling it and delivering it to Component Manager must also be
built. This proposal attempts to solve these concerns.

Config schemas are a primary user-facing interface for structured config. According to the criteria
set by [RFC-0098][rfc-0098], an RFC proposal is required for this feature.

The proposed manifest syntax and FIDL structures will be included in the Fuchsia SDK. To control
usage of this feature, config schemas in manifest files will be gated by a feature flag in `cmc`.

## Stakeholders

_Facilitator: pascallouis@google.com_

_Reviewers: adamperry@google.com, jsankey@google.com, geb@google.com, pascallouis@google.com,
cpu@google.com_

_Consulted: aaronwood@google.con, yifeit@google.com, surajmalhotra@google.com, shayba@google.com_

_Socialization: This design was shared with the Component Framework team and was the topic of a
Component Framework Design Discussion._

## Design

### Scope

This proposal considers the following to be in-scope:

*   Support for the minimum configuration types specified by [RFC-0127][rfc-0127]:
    booleans, integers, bounded length strings, and bounded length vectors of these data types
*   Alignment with the philosophies of Structured Config, specified by [RFC-0127][rfc-0127]:
    Simple, Reliable, Secure, Testable

This proposal considers the following to be future work and out-of-scope for this RFC:

*   [Support for config schemas in manifest shards](#structured-config-in-manifest-shards)
*   [Support for complex data types](#complex-data-types)
*   [Support for default values in config schemas](#default-values-in-config-schemas)
*   Support for override policies
    *   Per [RFC-0127][rfc-0127], override policies are declared for config value files


### CML Syntax

We will introduce a new top-level key in the component manifest to define the structured
configuration for a component. The corresponding value contains a key for each configuration field.

Structured configuration fields require a type system. The Fuchsia team has extensive experience
developing the type system for FIDL and we leverage this experience by defining a type system that
is compatible with FIDL and that shares the same philosophy; for example, a separation between
layout and constraints. The set of types that can be expressed in a structured configuration key is
currently a subset of the types that can be expressed in FIDL. Both structured configuration and
FIDL will evolve over time, and we imagine that structured configuration types may grow to be more
expressive in some aspects than FIDL types, for instance restricting numbers to specific ranges
via the use of constraints.

The CML style and syntax is used for consistency, and where the same concept exists, we make an
effort to align the naming and decomposition. For instance FIDL types are expressed as layouts,
optionally parameterized, and optionally constrained. This same decomposition exists in the
proposed CML syntax extension (a type has optional properties); primitive layouts such as `int32`,
or more complex layouts like `vector` have identical names; constraints can be applied to types
such as setting a maximum size on vectors or strings.

The change to a component manifest is summarized below:

```
{
    use: {
        ...,
    },
    ...
    config: {
        <key>: {
            type: "<type string>",
            <additional properties based on type>
        },
        ...
    }
}
```

`config` is a top-level key in the manifest whose value is a JSON dictionary. Each member of the
dictionary is a config field that consists of a key and a type.

Config keys have the following properties:

*   They are unique identifiers within a component’s config schema
*   They are used in system assembly and when processing overrides
    *   Config keys in compiled manifests are used by system assembly to create config value files
    *   Config keys are stable identifiers that can be used for defining parent overrides
*   They must match the regex `[a-z]([a-z0-9_]*[a-z0-9])?`
    *   This regex is compatible with identifiers in FIDL, JSON and potential
client libraries.
    *   It leaves room for encoding separators in keys, if necessary.
    *   It can also be expanded upon in the future.
*   They must not be more than 64 characters long
    *   This can be expanded in the future.

The type string in config fields is limited to one of the following values:

*   `bool`
*   `uint8`, `uint16`, `uint32`, `uint64`
*   `int8`, `int16`, `int32`, `int64`
*   `string`
*   `vector`

Support for types like `enum`, `float`, `struct` and `array` is possible with this design,
but is outside the scope of this RFC. The syntax of these complex types is discussed in the
[Future Work](#complex-data-types) section.

`bool` and integers do not have any type constraints:

```
config: {
    enable_advanced_features: { type: "bool" },
    num_threads: { type: "uint64" },
}
```

`string` must have the `max_size` type constraint. `max_size` is parsed as a `uint32`:

```
config: {
    network_id: {
        type: "string",
        max_size: 100,
    }
}
```

`vector` must have the `max_count` type constraint and the `element` type argument. `max_count`
is parsed as a `uint32`. `element` is restricted to a `bool`, integer or `string`:

```
config: {
    tags: {
        type: "vector",
        max_count: 10,
        element: {
            type: "string",
            max_size: 20,
        }
    }
}
```

### Examples

Consider the following component manifests that have been adapted to use structured configuration.
These examples will focus on the `config` section of the manifest explicitly.

#### archivist

Current configuration is split between [command-line arguments][archivist-cli]
and [JSON config files packaged with the component][archivist-json].
This example shows how all these config sources can now be collated into structured config.

```
config: {
    // proxy the kernel logger
    enable_klog: { type: "bool" },

    // initializes syslog library with a log socket to itself
    consume_own_logs: { type: "bool" },

    // connects to the component event provider. This can be set to false when the
    // archivist won't consume events from the Component Framework v1 to remove log spam.
    enable_component_event_provider: { type: "bool" },

    ...

    // initializes logging to debuglog via fuchsia.boot.WriteOnlyLog
    log_to_debuglog: { type: "bool" },

    // number of threads the archivist has available to use.
    num_threads: { type: "uint32" }
}
```

#### detect

Programs like `sampler`, `persistence` and `detect` are compiled into a “launcher” binary for
space-savings. Since each program bundled into the launcher has its own manifest, they can have
different structured configurations.

Consider the `detect` program whose current configuration is done using
[command-line arguments][detect-cli]:

```
program: {
    ...
    args: [
         // The mode is passed over argv because it does not vary for this component manifest.
         // Launcher will use the mode to determine the program to run.
         "detect"
    ]
},
config: {
    // how often to scan Diagnostic data
    // unit: minutes
    //
    // NOTE: in detect's CLI parsing, this is optional with a default specified in code.
    // when using structured config, the default would be provided by a build template or
    // by product assembly.
    check_every: { type: "uint64" },

    // if true, minimum times will be ignored for testing purposes.
    // never check in code with this flag enabled.
    test_only: { type: "bool" },
}
```

#### console

Current configuration is done using [command-line arguments][console-cli]. This example shows the
need for advanced types like string vectors in structured configuration.

```
config: {
    // Add a tag to the allow list. Log entries with matching tags will be output to
    // the console. If no tags are specified, all log entries will be printed.
    allowed_log_tags: {
        type: "vector",
        max_count: 40,
        element: {
            type: "string",
            max_size: 40,
        }
    },

    // Add a tag to the deny list. Log entries with matching tags will be prevented
    // from being output to the console. This takes precedence over the allow list.
    denied_log_tags: {
        type: "vector",
        max_count: 40,
        element: {
            type: "string",
            max_size: 40,
        }
    },
}
```

### FIDL specification

The CML syntax defined above must be compiled into an equivalent FIDL object by `cmc`, processed by
Component Manager and used for override resolution in later stages of implementation.

Using FIDL for the configuration schema is an internal choice that simplifies the implementation
in `cmc` and Component Manager. FIDL is not the interface being used by end-developers for config
schemas.

The `ConfigSchema` object contains:

*   an ordered list of fields (`ConfigField` objects)
*   schema checksum: a hash over all the config fields

A `ConfigField` FIDL object consists of a `key` and a `type`. The meaning of these two fields
is the same as in the CML syntax: `key` uniquely identifies the config field and the `type` is
the type to which config values must adhere.

The FIDL encoding allows a maximum of `2^32 - 1` configuration fields.

`cmc` will sort the configuration fields in a deterministic order but that order is not specified
by this RFC. Using a deterministic order provides consistency for the schema checksum, downstream
tools, and runtime config resolution. Leaving the order unspecified leaves room for
optimization in the future.

The schema checksum is computed by `cmc` using the key and value type of each field. This checksum
will also be present in the config value file. Component Manager will check that the checksum in the
schema and value file are exactly the same to prevent any version skew.

```
library fuchsia.component.decl;

// Config keys can only consist of these many bytes
const CONFIG_KEY_MAX_SIZE uint32 = 64;

// The string identifier for a config field.
alias ConfigKey = string:CONFIG_KEY_MAX_SIZE;

// The checksum produced for a configuration interface.
// Two configuration interfaces are the same if their checksums are the same.
type ConfigChecksum = flexible union {
    // A SHA-256 hash produced over a component's config interface.
    1: sha256 array<uint8>:32
};

/// The schema of a component's configuration interface.
type ConfigSchema = table {
    // Ordered fields of the component's configuration interface.
    1: fields vector<ConfigField>:MAX;

    // Checksum produced over a component's configuration interface.
    2: checksum ConfigChecksum;
};

// Declares a single config field (key + type)
type ConfigField = table {
    // The identifier for this config field.
    // This key will be used to match overrides.
    1: key ConfigKey;

    // The type of config values. Config values are verified
    // against this layout at build time and run time.
    2: type ConfigType;
};

// The type of a config value
type ConfigType = struct {
    layout ConfigTypeLayout;
    parameters vector<LayoutParameter>;
    constraints vector<LayoutConstraint>;
};

// Defines valid type ids for config fields.
type ConfigTypeLayout = flexible enum {
    BOOL = 1;
    UINT8 = 2;
    UINT16 = 3;
    UINT32 = 4;
    UINT64 = 5;
    INT8 = 6;
    INT16 = 7;
    INT32 = 8;
    INT64 = 9;
    STRING = 10;
    VECTOR = 11;
};

// Parameters of a given type layout
type LayoutParameter = table {
    // For vectors, this is the type of the nested element.
    1: nested_type ConfigType;
};

// Constraints on a given type layout
type LayoutConstraint = table {
    // For strings, this is the maximum number of bytes allowed.
    // For vectors, this is the maximum number of elements allowed.
    1: max_size uint32;
};
```


`Component` is the FIDL equivalent of a CML manifest and must now contain the `ConfigSchema` FIDL
object.

```
// *** component.fidl ***

library fuchsia.component.decl;
// NOTE: as long as the two libraries are supported, this change will also be made to
// library fuchsia.sys2;

/// A component declaration.
///
/// This information is typically encoded in the component manifest (.cm file)
/// if it has one or may be generated at runtime by a component resolver for
/// those that don't.
type Component = table {
    /// ... previous fields ...

    /// The schema of a component's configuration interface.
    10: config ConfigSchema;
};
```

### Changes to `cmc`

We will extend `cmc` to deserialize CML with a `config` section, validate its contents, and include
the resulting schema in the compiled manifest.

We will implement this behind a feature flag to `cmc`. Only in-tree components on an explicit
allowlist will be able to make use of `config` stanzas while we are implementing the rest of
structured configuration.

### Changes to Component Manager

We will make changes to Component Manager to present the structured config schema to the hub
under the `resolved/config` directory of each component instance. The exact encoding into the hub's
namespace will be unstable and is out of scope for this RFC.

This change to the hub will give us a way to create integration tests to verify that config
schemas pass through the component resolution pipeline successfully.

### Changes to `ffx component`

`ffx component show` uses the hub to output information about component instances. With the changes
to Component Manager, this plugin can now output the config schema of each component instance.

```
$ ffx component show netstack
Moniker: /core/network/netstack
URL: fuchsia-pkg://fuchsia.com/network#meta/netstack.cm
Type: CML static component
Component State: Resolved
...
Configuration:
  log_packets [bool]
  verbosity [string:10]
  socket_stats_sampling_interval [uint32]
  opaque_iids [bool]
  tags [vector<string:10>:20]
Execution State: Running
...
```

Note that the command output of `ffx component show` shown above is subject to change. The command
will only print the configuration schema until resolution of actual values is implemented.

## Implementation

The implementation of this design will be done in three incremental stages.

1. Add new types to `cm_rust`, parse config stanzas in `cmc` behind a feature flag
([Prototype](https://fxrev.dev/590566))
2. Expose configuration using the hub, integration testing ([Prototype](https://fxrev.dev/590741))
3. `ffx component show` changes


## Performance

*   We will be benchmarking the added performance hit to component start time due to structured
configuration. A [bug](https://fxbug.dev/86321) has been filed for this.
*   This design uses FIDL tables to encode the schema, which means some added overhead when
parsing `ComponentDecl` FIDL objects compared to declaring structs. We anticipate this overhead is
negligible compared with overall component start time.
*   This design stores the config keys as strings in the `ConfigField` FIDL object, which consumes
additional disk space and requires more data be copied when starting a component.
    *  This is a requirement for non-ELF runners some of which will need the string key to encode
       configuration values natively.
    *  This also makes debugging via the hub easier.
*   For N config keys, we expect an O(N) cost to config value matching. Config keys will not be
checked for equality in a case with no overrides.
*   We are not concerned with the performance of host tools like `cmc`.
*   Component Manager will not be responsible for hashing the config schema or the config
values. `cmc` will do the hashing in advance. Component Manager only needs to check the equality of
the hashes in the schema and value files.
*   Component Manager will be storing the configuration schema in the hub filesystem. This may
have an additional non-negligible impact on the memory usage of Component Manager.
    *   Making the hub pull-based rather than push-based would address this concern. A
    [bug](https://fxbug.dev/77190) has been filed for this feature request.

## Security Considerations

Author does not see any potential concerns. Note that this feature is gated in `cmc` using an
allowlist, further reducing security risks.

## Privacy Considerations

Configuration keys are visible inside the component manifest. These keys should not include
proprietary information if the component will be published publicly.

Author does not see any other potential concerns.

## Testing

We will have:

*   unit tests for `cmc` that test validation and compilation of `config` stanzas (including failure
cases)
*   unit tests for Component Manager that test the hub’s directory structure with structured config
*   integration tests for Component Manager that resolve component manifests and verify that the hub
shows their config schema
*   unit tests for `ffx component show` that parse structured config from the hub correctly.

## Documentation

We expect to add the following documentation as structured configuration reaches maturity:

*   CML syntax for a component’s structured config schema
*   FIDL specification for a component’s structured config schema
*   Best practices for declaring config schemas: comments, naming conventions and more
*   Example/Codelab: Adding `config` section to a component’s manifest, building the component,
verifying configuration fields using `ffx component show`

## Drawbacks, alternatives, and unknowns

### Alternatives: `config` section points to a FIDL file

It is possible for the `config` section in the manifest to point to a FIDL source file that
describes the configuration schema. [RFC-0127][rfc-0127-config-fidl] discusses this alternative in
detail and its drawbacks. The conclusions drawn from RFC-0127 apply here as well.

### Alternatives: `config` section under `program`

```
program: {
    config: {
        <key>: {
            type: "<type string>",
            <additional JSON fields based on type>
        },
    }
}
```

*   Pro: creates an association between the `program` and the `config`
*   Con: too much nesting
*   Con: components without `program` sections cannot have structured config. In the future, config
routing could involve components without programs.

### Alternatives: `fields` section

```
config: {
    fields: {
        <key>: {
            type: "<type string>",
        }
        ...
    }
}
```

*   Pro: Leaves room for future extensions to the config schema. It's not clear whether we need
that future extensibility.
*   Con: Less concise

### Alternatives: Verbosity of fields

Option B:

```
config: [
    {
        key: "<key>",
        type: "<type string>",
    }
    ...
]
```

*   Pro: Uses a JSON array rather than a JSON object. More consistent with other parts of the
manifest: `use`, `expose`, etc.
*   Con: This option visually differs from the struct/table/map semantics used for the defined
configuration fields. In JSON, values with string keys pointing to values are usually represented
by maps/objects.
*   Con: More verbose

Option C:

```
config: {
    <key>: "<type string>"
    ...
}
```

*   Pro: More concise
*   Con: Leaves no room for future extensions to the field. This may be required for complex types
and default values.

Option D:

```
config: [
    {
        <type string>: "<key>",
        ...
    },
],
```

*   Pro: More concise. No boilerplate 'type' or 'key' keywords
*   Pro: Consistent with capability routing syntax
*   Con: Duplicate keys require explicit checks
*   Con: It is unclear how this would work with the `element` type argument of vectors

## Future work

### Structured config in manifest shards

We are deferring this work until we can prove it is necessary. We also do not have a good strategy
for handling merge conflicts. If merge conflicts halt compilation then each shard needs to name
config fields defensively. If merge conflicts can be resolved, then different shards may
unintentionally share the same config field.

### Default values in config schema

Defaults can be supported in configuration fields. These default values will be described using
JSON types. [RFC-0127][rfc-0127] assumes that defaults should be a part of the configuration
schema, however it currently makes more sense for build rules or subassemblies to provide defaults
by generating config value files.

```
config: {
    enable_advanced_features: {
        type: "bool",
        default: false,
    }
    tags: {
        type: "vector",
        max_count: 10,
        element: {
            type: "string",
            max_size: 20,
        }
        default: [
            "foo",
            "bar",
            "baz",
        ]
    }
}
```

We are deferring this work until we can prove that the subassembly system cannot be used for
default values.

### Complex data types

In the future, we expect to add support for complex types like `array`, `enum`, `float`, and
`struct`. These types should be supported in `vector` and may go through additional verification
steps when possible.

```
config: {
    fsck: {
        type: "struct",
        fields: {
            check_on_mount: { type: "bool" },
            verify_hard_links: { type: "bool" },
            repair_errors: { type: "bool" },
        }
    },
    compression_type: {
        type: "enum",
        variants: {
            uncompressed: 0,
            zstd_chunked: 1,
        }
    },
    // Vectors can store complex structures
    coordinates: {
        type: "vector",
        max_count: 10,
        element: {
            type: "struct",
            fields: {
                x: { type: "int32" },
                y: { type: "int32" },
            }
        }
    },
}
```

### Tooling for config field comments

The configuration fields in above examples have JSON comments that describe the configuration field
in more detail. These comments can be processed and added to other areas of structured config.
One can imagine having the generated client libraries in Rust and C++ having the same descriptions.

Then if developers write client side code, they will get these descriptions as hints in the editor.
Tools like system assembly can also provide more detailed help and error text.

This would require changes to our JSON parsing libraries which currently do not parse JSON comments.

```
config: {
      /// Add a tag to the allow list. Log entries with matching tags will be output to
      /// the console. If no tags are specified, all log entries will be printed.
      allowed_log_tags: {
          type: "vector",
          max_count: 40,
          element: {
              type: "string",
              max_size: 40,
          }
      },

      /// Add a tag to the deny list. Log entries with matching tags will be prevented
      /// from being output to the console. This takes precedence over the allow list.
      denied_log_tags: {
          type: "vector",
          max_count: 40,
          element: {
              type: "string",
              max_size: 40,
          }
      },
}
```

### Maximum supported value for `max_size` and `max_count`

Manifests may want to use maximum supported value for the `max_size` and `max_count` properties in
vectors and strings. This can be done using a `MAX` string or by simply omitting the property
altogether.

```
config: {
    network_id: {
        type: "string",
        max_size: "MAX",
    }
}
```

```
config: {
    tags: {
        type: "vector",
        element: {
            type: "string"
        }
    }
}
```

## Prior art & references

[Component manifest syntax](concepts/components/v2/component_manifests.md#syntax)

[FIDL language specification](reference/fidl/language/language.md#types_and_type_declarations)

[The JSON5 Data Interchange Format](https://spec.json5.org/)

[rfc-0127]: contribute/governance/rfcs/0127_structured_configuration.md
[rfc-0127-config-fidl]: contribute/governance/rfcs/0127_structured_configuration.md#define_configuration_keys_using_fidl
[rfc-0098]: contribute/governance/rfcs/0098_component_framework_rfc_criteria.md#changes_that_require_an_rfc
[archivist-cli]: https://cs.opensource.google/fuchsia/fuchsia/+/main:src/diagnostics/archivist/src/main.rs;l=32;drc=2f81b79bd11f245e4fedd593b10e7e324d41c294
[archivist-json]: https://cs.opensource.google/fuchsia/fuchsia/+/main:src/diagnostics/archivist/configs/archivist_config.json;drc=734f251e7c7d1673a874be8747aee3940d45e03a
[detect-cli]: https://cs.opensource.google/fuchsia/fuchsia/+/main:src/diagnostics/detect/src/lib.rs;l=43;drc=d079db3c28c5c6851a99ec16f1395ddcaee55582
[console-cli]: https://cs.opensource.google/fuchsia/fuchsia/+/main:src/bringup/bin/console/main.cc;l=26;drc=a1bb768bcc156d4d284aefa7d87b7ffa3ad610b9