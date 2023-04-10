# Structured Configuration

Structured configuration allows C++/Rust components to declare configuration schemas directly in
their manifest. Benefits of using structured configuration include:

* Errors in configuration are detected at build and assembly time.
* Multiple packages can be created using the same component and different configuration values.
* Components read their configuration with statically-typed libraries.
* Component Framework only starts components with valid configuration.
* Configuration can be viewed at runtime with `ffx` tooling.
* Values can be set at runtime in tests with RealmBuilder.

To use structured configuration in your component, you must update build rules, declare a schema,
define values, and generate a client library.

## Update build rules

To prevent cyclic dependencies when generating client libraries, define a
`fuchsia_component_manifest` rule that compiles the component manifest. Pass this compiled manifest
GN label into the `fuchsia_component` rule.

```gn
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/config/cpp/BUILD.gn" region_tag="component" adjust_indentation="auto" %}
```

## Declare configuration schema

You must declare a configuration schema in a component's manifest:

```json5
{
    ...
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/config/rust/meta/config_example.cml" region_tag="config" %}
}
```

Structured config supports the following types:

* `bool`
* `uint8`, `int8`
* `uint16`, `int16`
* `uint32`, `int32`
* `uint64`, `int64`
* `string` (requires `max_size` property)
* `vector` (requires `element` and `max_count` properties)

See the [CML reference doc][cml-ref-doc] for the complete syntax for a config
schema.

Once your component has a configuration schema, you must define values for the
declared fields, either using Software Assembly or GN.

**Googlers only**: if your component is configured differently for eng and non-eng
[build types][build-types], you must
[add verification of your structured configuration][sc-verification] before you check it in.

There are two ways to define config values: in a JSON5 file or inline in GN.

## Define configuration values using Software Assembly

If your component's configuration varies between products, see the documentation
for [Assembling Structured Configuration][sa-sc-docs]. For components whose
configuration only varies between e.g. tests and production, see the next
section.

## Define & package configuration values using GN

The `fuchsia_structured_config_values` GN template validates the defined values
against the config schema and compiles them into a `.cvf` file that must be
packaged with your component.

There are two ways to define config values in GN: in a JSON5 file or inline.

### JSON5 file

You can write a component's configuration values in a JSON5 file. Because JSON5 is a strict
superset of JSON, existing JSON configuration files can also be reused for structured config.

Each key in the JSON object must correspond to a config key in the schema and the value must be of
a compatible JSON type:

```json5
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/config/config_example_default_values.json5" adjust_indentation="auto" %}
```

Provide the path to the JSON5 file in a `fuchsia_structured_config_values` rule.

```gn
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/config/rust/BUILD.gn" region_tag="config_values_json" adjust_indentation="auto" %}
```

### Inline values

The `fuchsia_structured_config_values` template also supports defining configuration values inline:

* {C++}

  ```gn
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/config/cpp/BUILD.gn" region_tag="args_declare" adjust_indentation="auto" %}
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/config/cpp/BUILD.gn" region_tag="config_values_gn" adjust_indentation="auto" %}
  ```

* {Rust}

  ```gn
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/config/rust/BUILD.gn" region_tag="args_declare" adjust_indentation="auto" %}
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/config/rust/BUILD.gn" region_tag="config_values_gn" adjust_indentation="auto" %}
  ```

By using `declare_args`, you can change configuration values on the command line at build time:

* {C++}

  ```
  $ fx set core.qemu-x64 \
    --with //examples/components/config \
    --args='config_example_cpp_greeting="C++ CLI Override"'
  ```

* {Rust}

  ```
  $ fx set core.qemu-x64 \
    --with //examples/components/config \
    --args='config_example_rust_greeting="Rust CLI Override"'
  ```

### Package the component and values

To package a component and a set of values together, add the `fuchsia_component` and `fuchsia_structured_config_values`
rules as dependencies of a `fuchsia_package`.

* {C++}

  ```gn
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/config/cpp/BUILD.gn" region_tag="package" adjust_indentation="auto" %}
  ```

* {Rust}

  ```gn
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/config/rust/BUILD.gn" region_tag="package" adjust_indentation="auto" %}
  ```

The build system verifies your component's configuration schema and value file. A component
with a faulty configuration (for example: field mismatch, bad constraints, missing value file) will
fail to build.

## Checking the configuration

Component manager validates a component's configuration when the component is resolved.

Use `ffx component show` to print out a components configuration key-value pairs. The component
does not have to be running for this to work.

```
$ ffx component show config_example
                Moniker: /core/ffx-laboratory:config_example
        Component State: Resolved
                      ...
          Configuration: greeting -> "World"
                      ...
```

## Reading the configuration

Components read their resolved configuration values with a generated library. Generate a library
using the following build templates:

* {C++}

  ```gn
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/config/cpp/BUILD.gn" region_tag="binary" adjust_indentation="auto" %}
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/config/cpp/BUILD.gn" region_tag="library" adjust_indentation="auto" %}
  ```

* {Rust}

  ```gn
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/config/rust/BUILD.gn" region_tag="binary" adjust_indentation="auto" %}
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/config/rust/BUILD.gn" region_tag="library" adjust_indentation="auto" %}
  ```

Use the following functions from the library to read configuration values:

* {C++}

  ```cpp
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/config/cpp/main.cc" region_tag="imports" adjust_indentation="auto" %}

  ...

  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/config/cpp/main.cc" region_tag="get_config" adjust_indentation="auto" %}
  ```

* {Rust}

  ```rust
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/config/rust/src/main.rs" region_tag="imports" adjust_indentation="auto" %}

  ...

  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/config/rust/src/main.rs" region_tag="get_config" adjust_indentation="auto" %}
  ```
  ```

## Export configuration to Inspect

You can export a component's configuration to Inspect so that it is available in
crash reports. The client libraries have functions to export a component's configuration to an
Inspect tree:

* {C++}

  ```cpp
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/config/cpp/main.cc" region_tag="inspect" adjust_indentation="auto" %}
  ```

* {Rust}

  ```rust
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/config/rust/src/main.rs" region_tag="inspect" adjust_indentation="auto" %}

Use `ffx inspect show` to print out the component's exported configuration:

```
$ ffx inspect show core/ffx-laboratory\*config_example
core/ffx-laboratory\:config_example:
  ...
  payload:
    root:
      config:
        greeting = World
```

## Testing with Realm Builder

You can use [Realm Builder][rb-feature-matrix] to dynamically replace the configuration values of
a component.

* {C++}

  ```cpp
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/config/integration_test/cpp/test.cc" region_tag="config_replace" adjust_indentation="auto" %}
  ```

* {Rust}

  ```rust
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/config/integration_test/rust/lib.rs" region_tag="config_replace" adjust_indentation="auto" %}
  ```

Realm Builder validates the replaced value against the component's configuration schema.

When setting values dynamically, Realm Builder requires users to choose whether
or not to load packaged configuration values for the launched component.

To load a component's packaged values when providing a subset of values in code:

* {C++}

  ```cpp
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/config/integration_test/cpp/test.cc" region_tag="config_load" adjust_indentation="auto" %}
  ```

* {Rust}

  ```rust
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/config/integration_test/rust/lib.rs" region_tag="config_load" adjust_indentation="auto" %}
  ```

To set all of a component's values in code without using packaged values:

* {C++}

  ```cpp
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/config/integration_test/cpp/test.cc" region_tag="config_empty" adjust_indentation="auto" %}
  ```

* {Rust}

  ```rust
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/config/integration_test/rust/lib.rs" region_tag="config_empty" adjust_indentation="auto" %}
  ```

<!-- TODO(fxbug.dev/104819): Link to fxbug.dev page when better documentation is available.  -->
[build-types]: /contribute/governance/rfcs/0115_build_types.md
[cml-ref-doc]: https://fuchsia.dev/reference/cml#config
[sa-sc-docs]: assembling_structured_config.md
[rb-feature-matrix]: /development/testing/components/realm_builder.md#language-feature-matrix
[sc-verification]: /development/verification/build_integration.md#verifying-structured-configuration-files
