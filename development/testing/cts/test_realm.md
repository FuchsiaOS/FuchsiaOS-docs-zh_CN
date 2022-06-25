# The CTS Test Realm

[CTS] tests run in a special realm which is meant to isolate any capabilities
that are required only for CTS testing. Tests must opt-in to the realm by
declaring a facet in their component manifests, as shown below.

```json5
// my_test.cml

{
    include: [
        "//src/sys/test_runners/rust/default.shard.cml",
    ],
    program: {
        binary: "bin/my_test_binary",
    },
    {{ '<strong>' }}facets: {
        "fuchsia.test": { type: "cts" },
    },{{ '</strong>' }}
}
```

Below is the list of capabilities provided to CTS tests:
{# Update the list when modifying //sdk/cts/test_realm/meta/cts_test_realm.shard.cml #}

Protocols:

```text
fuchsia.hwinfo.Board
fuchsia.hwinfo.Device
fuchsia.hwinfo.Product
fuchsia.logger.LogSink
fuchsia.process.Launcher
fuchsia.process.Resolver
fuchsia.settings.Privacy
fuchsia.sys2.EventSource
```

Storage:

```text
data
tmp
cache
custom_artifacts
```

[CTS]: development/testing/cts/overview.md
