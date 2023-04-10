# [DEPRECATED] Hub

Warning: The hub is deprecated and should only be used for manual exploration. Do not use it
in tests, scripts or tools.

## Definition

The hub is a virtual filesystem that represents a realm and contains relevant
directories of all component instances under that realm.

## Structure of the hub

A hub has the following filesystem layout:

```none
hub
├── children
|   ├── foo
|   |   └── ...
|   └── bar
|       └── ...
├── exposed
├── ns
├── out
└── runtime
```

When a component instance is resolved, the following directories are present in its hub:

-   `children`: Hub directories of the children of this component instance.
-   `exposed`: Capabilities exposed by this component instance in its manifest.
-   `ns`: Capabilities used by this component instance in its manifest.

When a component instance is running, the following directories are also present:

-   `out`: Capabilities served by the program of this component instance.
-   `runtime`: Debug information served by the runner of this component instance.

## Accessing the root hub

A hub scoped to the root component is available in the serial shell and `fx shell` under the
directory `/hub-v2`.

```
(host)$ fx shell
$ cd hub-v2
$ ls
children
exposed
ns
```

### Example: List all capabilities exposed by a component

To list all capabilities exposed by the component with moniker `/bootstrap/archivist`, do the
following:

```
(host)$ fx shell
$ cd /hub-v2/children/bootstrap/children/archivist/exposed
$ ls
diagnostics
fuchsia.diagnostics.ArchiveAccessor
fuchsia.diagnostics.FeedbackArchiveAccessor
fuchsia.diagnostics.LegacyMetricsArchiveAccessor
fuchsia.diagnostics.LoWPANArchiveAccessor
fuchsia.diagnostics.LogSettings
fuchsia.logger.Log
fuchsia.logger.LogSink
```

### Example: See the namespace of a component

To see the namespace of the component with moniker `/core/memory_monitor`, do the following:

```
(host)$ fx shell
$ cd /hub-v2/children/core/children/memory_monitor/ns
$ ls
cache
config
dev
pkg
svc
```