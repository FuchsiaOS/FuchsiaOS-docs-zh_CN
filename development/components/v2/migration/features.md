# Component sandbox features

This section provides guidance on migrating additional CMX
[`sandbox`][cmx-services] features.

Note: If there's a feature in your CMX file that's not in this list,
please reach out to [component-framework-dev][cf-dev-list].

## Storage features {#storage-features}

If your component uses any of the following features, follow the instructions in
this section to migrate storage access:

| Feature                       | Description | Storage Capability | Path     |
| ----------------------------- | ----------- | ------------------ | -------- |
| `isolated-persistent-storage` | Isolated    | `data`             | `/data`  |
:                               : persistent  :                    :          :
:                               : storage     :                    :          :
:                               : directory   :                    :          :
| `isolated-cache-storage`      | Managed     | `cache`            | `/cache` |
:                               : persistent  :                    :          :
:                               : storage     :                    :          :
:                               : directory   :                    :          :
| `isolated-temp`               | Managed     | `tmp`              | `/tmp`   |
:                               : in-memory   :                    :          :
:                               : storage     :                    :          :
:                               : directory   :                    :          :

These features are supported in v2 components using
[storage capabilities][storage-capabilities].

### Declare the required storage capabilities

When [migrating your component manifest][migrate-components], add the
following to your CML file:

```json5
// my_component.cml
{
    use: [
        ...
        {
            storage: "{{ '<var label="storage">data</var>' }}",
            path: "{{ '<var label="storage path">/data</var>' }}",
        },
    ],
}
```

### Route storage from the parent realm

When [adding your component][migrate-components-add], you'll need to offer
the appropriate storage path to your component from its parent realm.

```json5
// core.cml / component.core_shard.cml
{
    children: [
        ...
        {
            name: "my_component",
            url: "fuchsia-pkg://fuchsia.com/my-package#meta/my_component.cm",
        },
    ],
    offer: [
        ...
        {{ '<strong>' }}{
            storage: "{{ '<var label="storage">data</var>' }}",
            from: "self",
            to: [ "#my_component" ],
        },{{ '</strong>' }}
    ]
}
```

Note: If the appropriate storage capability is not currently provided by your
component's parent realm, reach out to [component-framework-dev][cf-dev-list]
for assistance.

### Update component storage index

Components that use storage use a [component ID index][component-id-index] to
preserve access to persistent storage contents across the migration, such as
[`core_component_id_index.json5`][example-component-id-index]. You must update
the component index to map the new component moniker to the same instance within
the component that provides the storage capability.

Find any instances of your current v1 component in component index files:

```json5
// core_component_id_index.json5
{
    instances: [
        ...
        {
            instance_id: "...",
            appmgr_moniker: {
                url: "fuchsia-pkg://fuchsia.com/my-package#meta/my_component.cmx",
                realm_path: [ ... ]
            },
        },
    ],
}
```

Replace the `appmgr_moniker` for your component instance with the new moniker in
the migrated v2 realm, keeping the same `instance_id`:

```json5
// core_component_id_index.json5
{
    instances: [
        ...
        {
            instance_id: "...",
            moniker: "/core/my_component",
        },
    ],
}
```

Note: If you are migrating your component to a realm other than `core`, the
moniker should reflect that.

### (Optional) Enable `persistent_storage` for collection descendants using storage

If the component or any of its ancestors, such as the session component, is part
of a collection, and the component requires storage contents to exist after a
component instance has been destroyed, add the
[`persistent_storage`][collection-persistent-storage] setting to the collection
decl:

```json5
{
  collections: [
    {
      name: "my_collection",
      durability: "{{ '<var label="durability">durability</var>' }}",
      persistent_storage: true,
    }
  ],
}
```

This setting allows collection descendants using the component ID index to
preserve storage content across [dynamic component instances][dynamic-children].

Note: the `persistent_storage` setting will apply to all descendants of the
collection.

### Storage capabilities in tests

When [migrating tests][migrate-tests], you will need to route storage access
to your test component if any of the components in the test realm access a
storage path.

Following the example in [Test uses injected services][migrate-tests-inject],
add the following to route storage access to your test component:

```json5
// my_component_test.cml (test component)
}
    use: [
        ...
        {{ '<strong>' }}{
            storage: "{{ '<var label="storage">data</var>' }}",
            path: "{{ '<var label="storage path">/data</var>' }}",
        },{{ '</strong>' }}
    ],
}
```

Note: Storage capabilities are backed by in-memory storage in tests and contents
will not persist once the test exits.

## Directory features {#directory-features}

If your component uses any of the following features, follow the instructions in
this section to migrate directory access:

Feature                 | Description                                 | Directory Capability    | Path
----------------------- | ------------------------------------------- | ----------------------- | ----
`shell-commands`        | Executable directory of shell binaries      | `bin`                   | `/bin`
`root-ssl-certificates` | Read-only root certificate data             | `root-ssl-certificates` | `/config/ssl`

These features are supported in v2 components using
[directory capabilities][directory-capabilities].

### Declare the required directory capabilities

When [migrating your component manifest][migrate-components], add the
following to your CML file:

```json5
// my_component.cml
{
    use: [
        ...
        {
            directory: "{{ '<var label="directory">root-ssl-certificates</var>' }}",
            rights: [ "r*" ],
            path: "{{ '<var label="directory path">/config/ssl</var>' }}",
        },
    ],
}
```

Note: Unlike storage locations, which are isolated per-component, directories
are a shared resource. You may need to also determine the **subdirectory** your
component needs to access in order to complete this migration.

### Route directory from the parent realm

When [adding your component][migrate-components-add], you'll need to offer
the directory capabilities to your component.

```json5
// core.cml / component.core_shard.cml
{
    children: [
        ...
        {
            name: "my_component",
            url: "fuchsia-pkg://fuchsia.com/my-package#meta/my_component.cm",
        },
    ],
    offer: [
        ...
        {{ '<strong>' }}{
            directory: "{{ '<var label="directory">root-ssl-certificates</var>' }}",
            from: "parent",
            to: [ "#my_component" ],
        },{{ '</strong>' }}
    ],
}
```

Note: If the appropriate directory capability is not currently provided by your
component's parent realm, reach out to [component-framework-dev][cf-dev-list]
for assistance.

### Directory paths in tests

When [migrating tests][migrate-tests], you need to route the directory
capabilities to your test component if any of the components in the test realm
require directory access.

Test Runner Framework only allows the following directory capabilities to be
used by non-hermetic tests:

Capability              | Description                     | Path
----------------------- | ------------------------------- | -------------
`root-ssl-certificates` | Read-only root certificate data | `/config/ssl`

Following the example in [Test uses injected services][migrate-tests-inject],
add the following to route directory access to your test component:

```json5
// my_component_test.cml (test component)
{
    use: [
        ...
        {{ '<strong>' }}{
            directory: "{{ '<var label="directory">root-ssl-certificates</var>' }}",
            rights: [ "r*" ],
            path: "{{ '<var label="directory path">/config/ssl</var>' }}",
        },{{ '</strong>' }}
    ],
}
```

Note: If the appropriate directory capability is not currently provided by the
Test Runner Framework, reach out to [component-framework-dev][cf-dev-list] for
assistance.

## Configuration data {#config-data}

If your component uses any of the following features, follow the instructions in
this section to migrate directory access:

| Feature       | Description        | Directory Capability | Path           |
| ------------- | ------------------ | -------------------- | -------------- |
| `config-data` | Read-only          | `config-data`        | `/config/data` |
:               : configuration data :                      :                :

These features are supported in v2 components using
[directory capabilities][directory-capabilities].

For more details using data files, see
[product-specific configuration with `config_data()`][config-data].

Consider [packaging your data files hermetically with `resource()`][resource-data]
if your component doesn't need to accept data files from arbitrary parts of the
source tree. Using `resource()` is simpler and more efficient.

### Declare the required directory capabilities

When [migrating your component manifest][migrate-components], add the
following to your CML file:

```json5
// my_component.cml
{
    use: [
        ...
        {
            directory: "config-data",
            rights: [ "r*" ],
            path: "/config/data",
        },
    ],
}
```

### Route directory from the parent realm

When [adding your component][migrate-components-add], you'll need to offer
the directory capability with the appropriate subdirectory to your component.

```json5
// core.cml / component.core_shard.cml
{
    children: [
        ...
        {
            name: "my_component",
            url: "fuchsia-pkg://fuchsia.com/my-package#meta/my_component.cm",
        },
    ],
    offer: [
        ...
        {{ '<strong>' }}{
            directory: "config-data",
            from: "parent",
            to: [ "#my_component" ],
            subdir: "{{ '<var label="package name">my-package</var>' }}",
        },{{ '</strong>' }}
    ],
}
```

### Configuration data in tests

When [migrating tests][migrate-tests], you need to route the directory
capability with the appropriate subdirectory to your test component if any of
the components in the test realm require directory access.
The name of the subdirectory should match the name of the package that contains
the component.

Following the example in [Test uses injected services][migrate-tests-inject],
add the following to route directory access to your test component:

```json5
// my_component_test.cml (test component)
{
    use: [
        ...
        {{ '<strong>' }}{
            directory: "config-data",
            rights: [ "r*" ],
            path: "/config/data",
            subdir: "{{ '<var label="package name">my-package</var>' }}",
        },{{ '</strong>' }}
    ],
}
```

## Device directories {#devices}

If your component uses any of the following features, follow the instructions in
this section to migrate device access:

Feature | Description                                      | Path
------- | ------------------------------------------------ | -------------------------
`dev`   | Entries in `devfs`                               | `/dev/*`
`dev`   | [Legacy device entries](#legacy-device-entries)  | `/dev/null`, `/dev/zero`

[Device filesystem][device-model] access is supported in Components v2 using
[directory capabilities][directory-capabilities].

Consider the following example using Components v1 to access
`/dev/class/input-report`:

```json
// my_component.cmx
{
    "program": { ... },
    "sandbox": {
        "dev": [
            "{{ '<var label="device subpath">class/input-report</var>' }}"
        ]
    }
}
```

### Declare the required device capabilities

When [migrating your component manifest][migrate-components], add the
device path as a directory capability to your CML file:

```json5
// my_component.cml
{
    use: [
        ...
        {
            directory: "{{ '<var label="device">dev-input-report</var>' }}",
            rights: [ "r*" ],
            path: "/dev/{{ '<var label="device subpath">class/input-report</var>' }}",
        },
    ],
}
```

### Route device subdirectory from the parent realm

When [adding your component][migrate-components-add], you'll need to offer
the appropriate device path to your component from its parent realm.

```json5
// core.cml / component.core_shard.cml
{
    children: [
        ...
        {
            name: "my_component",
            url: "fuchsia-pkg://fuchsia.com/my-package#meta/my_component.cm",
        },
    ],
    offer: [
        ...
        {{ '<strong>' }}{
            directory: "dev",
            from: "parent",
            as: "{{ '<var label="device">dev-input-report</var>' }}",
            to: [ "#my_component" ],
            subdir: "{{ '<var label="device subpath">class/input-report</var>' }}",
        },{{ '</strong>' }}
    ],
}
```

### Legacy device entries {#legacy-device-entries}

Components v2 does not route the following pseudo-device entries to components:

* `/dev/zero`: Create an equivalent (pseudo-)file in your code if necessary.
  For example, see the Chromium [`ScopedDevZero`][scoped-dev-zero-fuchsia]
  implementation.

* `/dev/null`: Use [fdio_fd_null_create] to get a file descriptor to a file
  that acts like `/dev/null`.

### Device directories in tests

When [migrating tests][migrate-tests], you need to route the directory
capabilities to your test component if any of the components in the test realm
require directory access.

Test Runner Framework only allows the following device directories to be used by
non-hermetic tests:

Capability                   | Description
---------------------------- | -----------------------------
`dev-input`                  | Input
`dev-input-report`           | Input method events
`dev-display-controller`     | Graphical display controller
`dev-goldfish-address-space` | Goldfish address space device
`dev-goldfish-control`       | Goldfish control device
`dev-goldfish-pipe`          | Goldfish pipe device
`dev-gpu`                    | GPU device
`dev-gpu-performance-counters` | GPU performance counters device

Following the example in [Test uses injected services][migrate-tests-inject],
add the following to route directory access to your test component:

```json5
// my_component_test.cml (test component)
{
    use: [
        ...
        {{ '<strong>' }}{
            directory: "{{ '<var label="device">dev-input-report</var>' }}",
            rights: [ "r*" ],
            path: "/dev/{{ '<var label="device subpath">class/input-report</var>' }}",
        },{{ '</strong>' }}
    ],
}
```

Note: If the appropriate device directory is not currently provided by the Test
Runner Framework, reach out to [component-framework-dev][cf-dev-list] for
assistance.

## Event capabilities {#events}

If your component uses any of the following features, follow the instructions in
this section:

Feature | Description                      | Path
------- | -------------------------------- | ----------
`hub`   | Observing component path changes | `/hub/c/*`
`hub`   | Observing realm path changes     | `/hub/r/*`

These features are supported in v2 components using
[event capabilities][event-capabilities].

### Event sources in tests

When [migrating tests][migrate-tests], you'll need to inject any components you
wish to observe into the test realm and route the appropriate lifecycle events
for those components to your test component.

Following the example in [Test uses injected services][migrate-tests-inject],
route the `fuchsia.sys2.EventSource` capability and the appropriate events to
your test component:

```json5
// my_component_test.cml (test component)
{
    children: [
        {
            name: "my_component",
            url: "fuchsia-pkg://fuchsia.com/my-package#meta/my_component.cm",
        },
    ],
    use: [
        {
            protocol: [ "fuchsia.sys2.EventSource" ],
        },
        {
            event: [ "{{ '<var label="event name">started</var>' }}" ],
            from: "framework",
            modes: [ "async" ],
        },
    ],
}
```

Note: The `EventSource` capability comes from the test realm (`parent`), but the
events come from the Component Manager (`framework`). This sets the event scope
to only components in the test realm. For more details on event scope,
see [event-capabilities][event-capabilities].

## Build Info {#build-info}

When migrating the `build-info` feature, instead use the
`fuchsia.buildinfo.Provider` [protocol][build-info-fidl]. This protocol is the
only supported method of retrieving build information moving forward. To use
this protocol, add it [while declaring required services][migrate-components-services].

## Vulkan {#vulkan}

When migrating the `vulkan` feature or code that uses a `//src/lib/vulkan/*.shard.cmx`
shard, instead use the `vulkan/client.shard.cml` [shard][manifests-shard] as
described in the [Vulkan documentation][vulkan].

## What's next {#next}

Explore the following sections for additional migration guidance on
specific features your components may support:

-   [Diagnostics capabilities](diagnostics.md)
-   [Other common situations](common.md)

[build-info-fidl]: https://fuchsia.dev/reference/fidl/fuchsia.buildinfo#Provider
[cf-dev-list]: https://groups.google.com/a/fuchsia.dev/g/component-framework-dev
[cmx-services]: concepts/components/v1/component_manifests.md#sandbox
[collection-persistent-storage]: https://fuchsia.dev/reference/fidl/fuchsia.component.decl#Collection.persistent_storage
[component-id-index]: development/components/component_id_index.md
[config-data]: development/components/data.md#product-specific_configuration_with_config_data
[device-model]: development/drivers/concepts/device_driver_model/device-model.md
[directory-capabilities]: concepts/components/v2/capabilities/directory.md
[dynamic-children]: concepts/components/v2/realms.md#dynamic-children
[event-capabilities]: concepts/components/v2/capabilities/event.md
[example-component-id-index]: /src/sys/appmgr/config/core_component_id_index.json5
[fdio_fd_null_create]: /sdk/lib/fdio/include/lib/fdio/fdio.h#48
[manifests-shard]: development/components/build.md#component-manifest-shards
[migrate-components]: development/components/v2/migration/components.md
[migrate-components-add]: development/components/v2/migration/components.md#add-component-to-topology
[migrate-components-services]: development/components/v2/migration/components.md#required-services
[migrate-tests]: development/components/v2/migration/tests.md
[migrate-tests-inject]: development/components/v2/migration/tests.md#injected-services
[resource-data]: development/components/data.md#hermetic_data_files_with_resource
[scoped-dev-zero-fuchsia]: https://source.chromium.org/chromium/chromium/src/+/main:base/test/scoped_dev_zero_fuchsia.cc
[storage-capabilities]: concepts/components/v2/capabilities/storage.md
[vulkan]: development/graphics/magma/concepts/vulkan.md#components_v2
