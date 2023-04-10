# Diagnostics capabilities

This section provides guidance on migrating the capabilities for diagnostics
features to Components v2.

## Inspect {#inspect}

{% dynamic if user.is_googler %}

Note: If your component shares Inspect data in product feedback reports, you may
also need to update the approved selectors to reference the new component
moniker. For more details on updating feedback selectors, see
[go/tq-diagnostics-migration](http://go/tq-diagnostics-migration).

{% dynamic endif %}

If your component is using [Inspect][inspect], you'll need to expose additional
information to the framework. You can quickly determine if your component uses
Inspect by looking for one of the following library dependencies in the
component's `BUILD.gn`:

-   `//sdk/lib/sys/inspect/cpp`
-   `//src/lib/diagnostics/inspect/rust`
-   `dart_package_label.fuchsia_inspect`

In Components v1, `appmgr` provides access to the component's `/diagnostics`
directory, which contains Inspect data. Components v2 requires a component to
explicitly expose `/diagnostics` to the framework. This allows the
[Archivist][archivist] to read Inspect data for snapshots,
[`ffx inspect`][ffx-inspect], and more.

Note: For more details on the differences in data collection between Components
v1 and Components v2, see the [Archivist documentation][archivist].

When [migrating the component manifest][migrate-components], you can add
Inspect capabilities to your v2 component by including the following manifest
shard:

```json5
// my_component.cml
{
    // Expose the diagnostics directory capability for Inspect
    include: [ "inspect/client.shard.cml" ],
    ...
}
```

### Component moniker for selectors

If you added your component to `core.cml`, you can infer your component's
[moniker][moniker] to be `/core/component_name` where `component_name` is the
name of the child you added to `core.cml`.

You can see this hierarchy using `ffx component list` as well:

```
/
/bootstrap
/bootstrap/archivist
...
/core
...
/core/appmgr
/core/appmgr/app
/core/appmgr/app/sysmgr.cmx
/core/appmgr/app/sys
/core/appmgr/app/sys/build-info.cmx
/core/appmgr/app/sys/cobalt.cmx
...
/core/battery_manager
/core/font_provider
...
/startup
```

Alternatively you can use `ffx inspect list` to see available components for
querying inspect data.

### Inspect data in tests {#inspect-tests}

If your test components read Inspect diagnostics data, migrate to the
`fuchsia.diagnostics.ArchiveAccessor` service provided by the
[Archivist][archivist]. Consider the following approaches you may be currently
using from Components v1 to accomplish this:

-   Injected services. The test CMX contains
    `fuchsia.diagnostics.ArchiveAccessor` as an `injected-service`, reading
    isolated inspect data from an embedded Archivist limited to test components:

    ```json
    {
        "fuchsia.test": {
            "injected-services": {
                "fuchsia.diagnostics.ArchiveAccessor":
                    "fuchsia-pkg://fuchsia.com/archivist-for-embedding#meta/archivist-for-embedding.cmx",
                ...
            }
        },
        ...
    }
    ```

    It means the test is reading isolated inspect data from an embedded
    Archivist that only sees test components.

-   Directly from the [Hub](#hub).

In v2, there's an Archivist running inside each test. Instead of instantiating
another Archivist in your test, you can use that embedded Archivist Accessor
protocol directly in your test. Therefore you'll need to do the following:

1.  When [migrating tests][migrate-tests], add the protocol capability to your
    test component:

    ```json5
    // my_component_test.cml (test component)
    {
        use: [
            {
                protocol: [ "fuchsia.diagnostics.ArchiveAccessor" ],
            },
        ]
    }
    ```

1.  Update your program to use the `ArchiveReader` library, which is available
    in [C++][archive-cpp], [Rust][archive-rust], and [Dart][archive-dart]. See the
    [Inspect Codelab][archive-codelab] for more details on using these libraries.

    Note: For components in other languages, use the `ArchiveAccessor`
    [FIDL protocol][archive-fidl] directly.

1.  Update any monikers used in your test's diagnostic selectors.

    - If you declare a child component in the test CML, check the
        [Integration Testing docs][integration-test-monikers] to learn more about your moniker.
    - If you are using `RealmBuilder`, check the [Realm Builder docs][realm-builder-monikers] to
        learn more about your moniker.

## Logging {#logging}

If your component requires access to [logging][logs], you'll need to declare the
`fuchsia.logger.LogSink` capability in your manifest. In Components v1, you may
have included `diagnostics/syslog/client.shard.cmx` or referenced the protocol
directly under `services` in your CMX file.

You can add syslog capabilities to your v2 component by including the following
manifest shard:

```json5
// my_component.cml
{
    // Expose the LogSink capability for syslog
    include: [ "syslog/client.shard.cml" ],
    ...
}
```

Additionally, Components v1 redirects `stderr` and `stdout` to `debuglog`, but
in Components v2 they are only redirected if a component has access to the
`LogSink` service. The `debuglog` is typically
used for low-level debugging information from the kernel and device drivers. If
your component writes log data to these streams, consider the following:

-   [Redirect to syslog](#syslog-redirect): Forward print statements to use the
    system's standard `syslog` buffer instead. This buffer is larger and capable
    of attributing logs per-component.
-   [Redirect to debuglog](#debuglog-redirect): If you have integration tests or
    other use cases that require preserving the default Components v1 behavior
    for log messages, direct the streams back to the `debuglog` buffer.

### Redirecting to syslog {#syslog-redirect}

To send `stderr` and `stdout` to [syslog][syslog] in your v2 component, your
component needs access to `LogSink`. This enables forwarding for all print
statements, including those generated by libraries or runtime code.

When [migrating your component manifest][migrate-components], include
the following manifest shard to enable forwarding:

```json5
// my_component.cml
{
    include: [ "syslog/client.shard.cml" ],
    ...
}
```

Note: Logging directly to `syslog` from your code provides additional features
to your component, such as severity levels. To take advantage of these features,
consider migrating your code to use the logging libraries highlighted in the
[`syslog` documentation][syslog].

### Redirecting to debuglog {#debuglog-redirect}

Your component may have external dependencies that rely on log messages in
[debuglog][debug-log]. One common use case is integration tests that directly
parse log messages from the `stdout` of an emulator process using the
[emulatortest][emulatortest] framework. In these cases, you'll need to manually
direct log messages back to the `debuglog` buffer.

1.  When [migrating your component manifest][migrate-components], disable
    the default forwarding and request the `fuchsia.boot.WriteOnlyLog`
    capability.

    ```json5
    // my_component.cml
    {
        program: {
            ...
            forward_stdout_to: "none",
            forward_stderr_to: "none",
        },
        use: [
            ...
            {
                protocol: [
                    "fuchsia.boot.WriteOnlyLog",
                ],
            },
        ],
    }
    ```

1.  When [adding your component][migrate-components-add], add the following
    to offer this capability to your component from `core`:

    ```json5
    // core.cml / component.core_shard.cml
    {
        offer: [
            ...
            {
                protocol: [ "fuchsia.boot.WriteOnlyLog" ],
                from: "parent",
                to: [ "#my_component" ],
            },
        ],
    }
    ```

1.  Direct `stderr` and `stdout` to `debuglog` in your program. You can use
    libraries for the initialization if your component is written in
    [Rust][debug-log-rust] or [C++][debug-log-cpp].

    Note: If the component isn't written in C++ or Rust you can use the existing
    libraries as a template for how to perform the initialization.

## Hub {#hub}

The hub provides access to detailed structural information about component
instances at runtime. In Components v1, `appmgr` provides the v1 Hub
through a specific directory structure populated in your component's namespace
under `/hub`. In Components v2, many v1 Hub use cases have preferred alternative
approaches.

When migrating to Components v2, consider the following alternatives:

-   [Observing lifecycle events][migrate-features-events]: Clients watching the
    filesystem to observe component instance changes should use
    [event capabilities][event-capabilities] instead.
-   [Reading inspect data](#inspect-tests): Clients reading Inspect data from
    `out/diagnostics` should migrate to the
    `fuchsia.diagnostics.ArchiveAccessor` service instead.
-   [Connecting to exposed services][migrate-tests-inject]: Clients connecting to
    services exposed through a component's `out/svc` directory should route
    these services and capability providers into their tests instead, similar to
    `injected-services`.

For other use cases, follow the instructions in this section to migrate to the
[v2 Hub][hub-v2] provided by Component Manager.

Note: Features of the Hub are designed to support test components only. If you
need to access the Hub outside of the test realm, reach out to
[component-framework-dev][cf-dev-list] for assistance.

### Route the hub directory

When [migrating tests][migrate-tests], you'll need to route the `hub`
[directory capability][directory-capabilities] to your test component if any
components in the test realm need to read data from the v2 Hub.

Following the example in [Test uses injected services][migrate-tests-inject],
add the `hub` directory capability to your CML file:

```json5
//my_component_test.cml
{
    use: [
        {
            directory: "hub",
            from: "framework",
            rights: [ "r*" ],
            path: "/hub",
        },
    ]
}
```

### Update hub reference paths

Update your code to reference the content path from the v2 Hub directory
structure. Here are some examples of path differences between the Hub
implementations:

| v1 Hub Path | [v2 Hub][hub-v2] Path |
| --------------------- | --------------------- |
| `/hub/c/{{ '<var>' }}component-name{{ '</var>' }}/{{ '<var>' }}instance-id{{ '</var>' }}/url` | `/hub/url` |
| `/hub/c/{{ '<var>' }}component-name{{ '</var>' }}/{{ '<var>' }}instance-id{{ '</var>' }}/in/{{ '<var>' }}svc-path{{ '</var>' }}` | `/hub/exec/in/{{ '<var>' }}svc-path{{ '</var>' }}` |
| `/hub/c/{{ '<var>' }}component-name{{ '</var>' }}/{{ '<var>' }}instance-id{{ '</var>' }}/process-id` | `/hub/exec/runtime/elf/process-id` |
| `/hub/c/{{ '<var>' }}child-component{{ '</var>' }}` | `/hub/children/{{ '<var>' }}child-component{{ '</var>' }}` |

Note: The `hub` directory routed to your component is scoped to the current
realm. To access hub contents from the parent realm, route the hub from `parent`
instead of `framework`. This feature is not available with the v1 Hub.

## What's next {#next}

Explore the following sections for additional migration guidance on
specific features your components may support:

-   [Component sandbox features](features.md)
-   [Other common situations](common.md)

[archive-codelab]: /development/diagnostics/inspect/codelab/codelab.md
[archive-cpp]: /sdk/lib/inspect/contrib/cpp
[archive-dart]: /sdk/dart/fuchsia_inspect/lib/src/reader
[archive-fidl]: https://fuchsia.dev/reference/fidl/fuchsia.diagnostics#ArchiveAccessor
[archive-rust]: /src/lib/diagnostics/reader/rust
[archivist]: /reference/diagnostics/inspect/tree.md#archivist
[debug-log-cpp]: /src/sys/lib/stdout-to-debuglog/cpp
[debug-log-rust]: /src/sys/lib/stdout-to-debuglog/rust
[debug-log]: /development/diagnostics/logs/recording.md#debuglog_handles
[directory-capabilities]: /concepts/components/v2/capabilities/directory.md
[cf-dev-list]: https://groups.google.com/a/fuchsia.dev/g/component-framework-dev
[emulatortest]: /tools/emulator/emulatortest
[event-capabilities]: /concepts/components/v2/capabilities/event.md
[ffx-inspect]: https://fuchsia.dev/reference/tools/sdk/ffx.md#inspect
[hub-v2]: /concepts/components/v2/hub.md
[inspect]: /development/diagnostics/inspect/README.md
[integration-test-monikers]: /development/testing/components/integration_testing.md#test-component-moniker
[logs]: /development/diagnostics/logs/README.md
[migrate-components]: /development/components/v2/migration/components.md
[migrate-components-add]: /development/components/v2/migration/components.md#add-component-to-topology
[migrate-features-events]: /development/components/v2/migration/features.md#events
[migrate-tests]: /development/components/v2/migration/tests.md
[migrate-tests-inject]: /development/components/v2/migration/tests.md#injected-services
[moniker]: /reference/components/moniker.md
[realm-builder-monikers]: /development/testing/components/realm_builder.md#test-component-moniker
[syslog]: /development/diagnostics/logs/recording.md#logsinksyslog
