# Trace specification file

A trace specification file is a JSON file that can be passed to `trace record`
to configure parameters of tracing. For those parameters that can be
passed both on the command line and set in the specification file, the command
line value overrides the one from the file.

The file supports the following top level-parameters:

 - `app`: string, url of the application to be run
 - `args`: array of strings, startup arguments to be passed to the application
 - `categories`: array of strings, tracing categories to be enabled
 - `duration`: integer, duration of tracing in seconds
 - `measure`: array of measurement specifications, see Benchmarking

For information about the Fuchsia tracing system,
see [Fuchsia tracing system](/docs/concepts/tracing/README.md).

For information on how to record a trace, see
[Recording a Fuchsia
trace](/docs/development/tracing/tutorial/recording-a-fuchsia-trace.md).

## Configuration

The tracing configuration is a JSON file consisting of a list of known
category names and descriptions.

The format is as follows:

```json
    {
      "categories": {
        "category1": "description1",
        "category2": "description2"
      },
      "providers": {
        "provider-label": "file:///provider-to-start-automatically"
      }
    }
```

