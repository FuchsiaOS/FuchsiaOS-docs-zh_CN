## Manifest shards

Some collections of capabilities represent use case requirements that are common
to many components in the system, such as logging. To simplify including these
capabilities in your components, the framework abstracts them into
**manifest shards** that can be included in your CML source file.

Below is an equivalent CML to the previous example. In this case, the necessary
logging capabilities are provided by including
`diagnostics/syslog/client.shard.cml` instead of declaring
`fuchsia.logger.LogSink` explicitly:

```json5
{
    include: [ "syslog/client.shard.cml" ],

    // Information about the program to run.
    program: {
        // Use the built-in ELF runner.
        runner: "elf",
        // The binary to run for this component.
        binary: "bin/hello-world",
        // Program arguments
        args: [
            "Hello",
            "World!",
        ],
    },
}
```
