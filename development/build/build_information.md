# Retrieve build information

Metrics and error reports are collected from devices in several ways: Cobalt,
feedback reports, crash reports, manual reports from developers and QA.
Interpreting these signals requires knowing where they are generated from to
varying levels of detail. This document describes the places where version
information about the system are stored for use in these types of reports.

Note that this information only applies to the base system; dynamically or
ephemerally added software will not be included here.

## View build information using CLI {#view-build-information-using-cli}

To view the device's build information using a command line, run the following
`ffx` command:

```posix-terminal
ffx target show
```

## Access build information at runtime {#access-build-information-at-runtime}

To access build information at runtime, use the
[`fuchsia.buildinfo.Provider`][buildinfo-provider]
[protocol capability][protocol-capability] in your
[component manifest][component-manifest].

Typed build information is defined and documented in the
[`BuildInfo` type][buildinfo-type]. In addition, a `jiri snapshot` taken at
build time may be retrieved.

Lastly, the kernel version may be retrieved with
[`zx_system_get_version_string`][zx-system-get-version-string].

[buildinfo-provider]: https://fuchsia.dev/reference/fidl/fuchsia.buildinfo#Provider
[buildinfo-type]: https://fuchsia.dev/reference/fidl/fuchsia.buildinfo#BuildInfo
[component-manifest]: /docs/concepts/components/v2/component_manifests.md
[protocol-capability]: /docs/concepts/components/v2/capabilities/protocol.md
[zx-system-get-version-string]: /docs/reference/syscalls/system_get_version_string.md
