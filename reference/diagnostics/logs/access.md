# Accessing Logs

Like other diagnostics, logs are available to read from the Archivist's [ArchiveAccessor] protocol.
See the [ArchiveAccessor reference] for general information on the API and its usage.

## Parameters

[StreamParameters] are passed to the `StreamDiagnostics` method to initialize a [BatchIterator].

### [DataType]

Send the `LOGS` datatype to request logs from the Archivist.

### [StreamMode]

Logs are available over `SNAPSHOT`, `SNAPSHOT_THEN_SUBSCRIBE`, and `SUBSCRIBE` modes.

### [Format]

Only `JSON` is currently supported.

### [ClientSelectorConfiguration]

Only `select_all=true` is [currently supported](https://fxbug.dev/62650).

### `batch_retrieval_timeout_seconds`

This option has no effect when requesting logs, as there is no point during log queries where the
Archivist has to wait for components producing logs.

## Results

Results are returned as a [`vector<FormattedContent>`][FormattedContent] with each entry's variant
matching the requested [Format], although JSON is the only currently supported format.

### Buffer contents

<!-- TODO(fxbug.dev/65241) link to JSON schema when available -->

Each buffer contains a top-level array with all of its elements as objects:

```json
[
    {
        "metadata": { ... },
        "payload": { ... }
    },
    {
        "metadata": { ... },
        "payload": { ... }
    },
    ...
]
```

The second level of nesting allows the Archivist to pack log messages efficiently while also using
the same API to serve Inspect results where there is a 1:1 mapping between parsed VMOs and returned
VMOs.

### JSON object contents

<!-- TODO(fxbug.dev/65241) link to JSON schema when available -->

Each JSON object in the array is one log message or event. Like other data types in ArchiveAccessor,
each object consists of several fields, although the contents of `metadata` and `payload` differ
from other sources:

```json
{
    "version": 1,
    "moniker": "core/network/netstack",
    "metadata": {
        "timestamp": 1234567890,
        "severity": "INFO",
        "component_url": "fuchsia-pkg://fuchsia.com/network#meta/netstack.cm",
        "size_bytes": 123
    },
    "payload": {
        "root": {
            "pid": 78,
            "tid": 78,
            "tag": "netstack",
            "message": "something happened!"
        }
    }
}
```

Note: version 1 of this format is [not yet stable](https://fxbug.dev/63883) and taking a dependency
on it may require some small migrations in the future. These migrations will be transparent to users
of the [Rust](/src/lib/diagnostics/reader/rust) client library.

Caution: messages with multiple tags [currently have the `tag` field repeated](https://fxbug.dev/63883)
in the same object. Most JSON parsers consider this invalid by default, and will be resolved soon.

#### Timestamps

Time in logs is recorded using the kernel's [monotonic clock] and conveyed without
modification as an unsigned integer.

Note: Archivist has limited ability to verify the accuracy of timestamps reported by components in
their log records. Efforts are taken to ensure correctness of common libraries in use, but it is
not possible in today's system to *guarantee* the timestamps in the metadata of logs records.

#### Monikers: v1 vs. v2

Logs from components run by appmgr and Component Manager are identical.

However, there are two systems for [reporting `LogSink` connections][attribution] to Archivist. As a
result, v1 monikers from appmgr come as a realm path starting from appmgr `app/sys` plus the
manifest filename, `netstack.cmx` for example. In v2, an explicit component name replaces the
manifest filename and all realms are included, `core/pkg-resolver` for example.

#### Dropped logs

<!-- TODO(fxbug.dev/65241) link to JSON schema when available -->

Dropped logs are conveyed in the `metadata.errors` field of the results object, which is an array
when present:

```json
{
    "metadata": {
        ...
        "errors": [
            {
                "dropped_logs": {
                    "count": 3
                }
            }
        ]
    }
}
```

Note: version 1 of this format is [not yet stable](https://fxbug.dev/63883) and taking a dependency
on it may require some small migrations in the future. These migrations will be transparent to users
of the [Rust](/src/lib/diagnostics/reader/rust) client library.

A message with dropped logs in the errors may or may not have a `payload` associated, depending on
where the message was dropped.

When a producing component overflows its log socket, it increments a counter that is used on
subsequent successful sends to populate a field in the log metadata. The Archivist tracks this
metadata but isn't able to learn about a component dropping log records except through later log
records from that component. The metadata is passed on to clients in the same form, as an error
field in a populated log record.

If the message was dropped by the Archivist due to buffer limits, the error is sent in a synthesized
message without a payload to ensure clients are notified even if the given producer does not log
again.

## Legacy APIs

Archivist serves the `fuchsia.logger.Log` protocol that allows clients to read logs in a text
format. This API is superceded by `fuchsia.diagnostics.ArchiveAccessor` and will be deleted in the
future.

Note: Prefer `Log.ListenSafe` and `Log.DumpLogsSafe` to avoid channel overflow issues. Deletion of
the unsafe `fuchsia.logger.LogListener` API is [planned](https://fxbug.dev/48758).

[ArchiveAccessor]: https://fuchsia.dev/reference/fidl/fuchsia.diagnostics#ArchiveAccessor
[ArchiveAccessor reference]: /src/diagnostics/docs/reference/access.md
[attribution]: /docs/concepts/components/diagnostics/logs/attribution.md
[BatchIterator]: https://fuchsia.dev/reference/fidl/fuchsia.diagnostics#BatchIterator
[ClientSelectorConfiguration]: https://fuchsia.dev/reference/fidl/fuchsia.diagnostics#ClientSelectorConfiguration
[DataType]: https://fuchsia.dev/reference/fidl/fuchsia.diagnostics#DataType
[Format]: https://fuchsia.dev/reference/fidl/fuchsia.diagnostics#Format
[FormattedContent]: https://fuchsia.dev/reference/fidl/fuchsia.diagnostics#FormattedContent
[monotonic clock]: /docs/reference/syscalls/clock_get_monotonic.md
[StreamMode]: https://fuchsia.dev/reference/fidl/fuchsia.diagnostics#StreamMode
[StreamParameters]: https://fuchsia.dev/reference/fidl/fuchsia.diagnostics#StreamParameters
