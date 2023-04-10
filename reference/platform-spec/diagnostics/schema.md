# Diagnostics schema

This document describes the schema of the responses returned by the [Archivist][archivist]'s
[`fuchsia.diagnostics/ArchiveAccessor#StreamDiagnostics`][stream-diagnostics].

## Introduction

The Canonical Diagnostics Data schema describes the structure that is shared across all diagnostics
data sources, whether they are accessed by streaming or by snapshot. This canonical schema provides
some requirements for what we can now call diagnostics data; diagnostics data must:

- Be associated with a specific moniker corresponding to a unit of execution on the system.
- Be timestamped.
- Contain arbitrarily structured data payloads, including hierarchical data.

Diagnostics schemas can be thought of as containing three distinct sections: a namespacing section,
a metadata section, and a payload section.

## Schema

### Top-level

The top-level schema contains the following fields:

- [Data source](#namespacing)
- [Moniker](#namespacing)
- [Metadata](#metadata)
- [Payload](#payload)
- Version: version of the schema, currently `1`.

This would look as follows in JSON:

```json
{
    "data_source": "INSPECT",
    "moniker": "core/foo",
    "metadata": { ... },
    "payload": { ... },
    "version": 1,
}
```

### Namespacing

The namespacing section consists of the information that can be applied to the entire
schema, across all data types; this information serves to describe the namespace of the source
that generated this diagnostics data. The namespacing section is made up of two fields:

- A component [`moniker`][moniker] describing the component that the the diagnostics data contained
  in the schema applies to.
- A `data source` describing which diagnostics service generated the data.

### Metadata

The metadata section consists of properties with statically known and documented schemas for which
a single value cannot be shared across all nested schemas. Each of the data sources adds its own
metadata attributes. The metadata section is the location where unique and un-unifiable variations
in diagnostics are expressed in the unified schema.

#### Inspect

Metadata for Inspect responses contains the following fields:

- Errors: Errors that happened while fetching the components Inspect data (optional).
  The error is an object with a single `message` string field.

- Component URL: The URL with which the component was launched.

- Timestamp: The monotonic time when the Inspect data was snapshotted.

- Filename: The name of the file where the Inspect data was located inside the component's
  `out/diagnostics` directory, if present.

This would look as follows in JSON:

```json
{
    ...,
    "metadata": {
        "errors": [
            {
                "message": "...",
            }
        ],
        "component_url": "fuchsia-pkg://...",
        "timestamp": 12345,
        "filename": "fuchsia.inpsect.Tree"
    },
}
```

#### Logs

Metadata for logs responses contains the following fields:

- Errors: errors that happened while fetching the components Inspect data (optional). Each error can
  be one of the following:

  - Dropped logs: The number of logs that the component dropped while writing due to socket writing
    errors.
  - Rolled out logs: The number of logs that the archivist rolled out from a component due to the
    internal log buffer being full.
  - Failed to parse record: The log record that the component wrote to the socket couldn't be
    parsed.
  - Other: A string explaining some other error that happened.

- Component URL: The URL with which the component was launched.

- Timestamp: The monotonic time when the event happened in the framework.

- Severity: The severity of the log. One of: `Trace`, `Debug`, `Info`, `Warn`, `Error`, `Fatal`.

- Tags: List of string tags associated with the log (optional).

- Pid: The process koid that emitted the log (optional).

- Tid: The thread koid that emitted the log (optional).

- File: The name of the file that emitted the log (optional).

- Line: The line number in the file that emitted the log (optional).

This would look as follows in JSON:

```json
{
    ...,
    "metadata": {
        "errors": [
            {
                "dropped_logs": { "count": 3 },
            }
        ],
        "component_url": "fuchsia-pkg://...",
        "timestamp": 12345,
        "severity": "Info",
        "tags": ["foo"],
        "pid": 123,
        "tid": 456,
        "file": "lib.rs",
        "line": 5,
    },
}
```

### Payload

The payload section contains the actual diagnostics data being reported. The payload is structured
hierarchically, and the context or ttopic of the data is encoded by its location within the
hierarchy (eg, the name of the node it sits on).

#### Inspect

The payload of the Inspect response is an object representing the Inspect tree that the component
was exposing at the time of snapshotting.

This would look as follows in JSON:

```json
{
    ...,
    "payload": {
        "root": {
            "foo": { ... },
            "bar": 3,
            ...
        }
    },
}
```

#### Logs

The logs payload is an object with the following fields:

- Message: Contains a log message. This is an object with one field:
  - Value: String representing the log message.
- Keys: Contains an object with the keys and associated values for structured log messages.
- Printf: Contains printf format data. This is an object with two fields:
  - Format: The format passed to printf.
  - Args: A list of arguments passed to printf.

For a regular text log (or a structured log without keys), this would look as follows in JSON:

```json
{
    ...
    "payload": {
        "message": {
            "value": "my log",
        }
    }
}
```

For a structured log with keys, this would look as follows in JSON:

```json
{
    ...
    "payload": {
        "message": {
            "value": "my log",
        },
        "keys": {
            "foo": 3,
            "bar": "baz",
            ...
        }
    }
}
```

For a printf log, this would look as follows in JSON:

```json
{
    ...
    "payload": {
        "printf": {
            "format": "my log %s",
            "args": ["foo"]
        }
    }
}
```

[archivist]: /docs/reference/diagnostics/inspect/tree.md#archivist
[moniker]: /docs/reference/components/moniker.md
[stream-diagnostics]: https://fuchsia.dev/reference/fidl/fuchsia.diagnostics#ArchiveAccessor.StreamDiagnostics)
