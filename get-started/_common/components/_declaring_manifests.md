## Component manifests

CML files are [JSON5](https://json5.org/){: .external} files that end with a
`.cml` extension. Below is an example CML manifest file for a simple component
running an ELF binary that prints a "Hello, World" message to the system log:

```json5
{
    // Information about the program to run.
    program: {
        // Use the built-in ELF runner.
        runner: "elf",
        // The binary to run for this component.
        binary: "bin/hello",
        // Program arguments
        args: [
            "Hello",
            "World!",
        ],
    },

    // Capabilities used by this component.
    use: [
        { protocol: "fuchsia.logger.LogSink" },
    ],
}
```

This file declares two main sections of information about the component:

Note: For more details on component manifests, see
[component manifests](concepts/components/v2/component_manifests.md).

* `program`: Describes the executable information such as the binary file,
  program arguments, and the associated runtime. In this example, a binary
  is compiled as an ELF executable and uses the built-in
  [ELF runner](concepts/components/v2/elf_runner.md).
* `use`: Declares the capabilities this component requires to run. In this
  example, the `fuchsia.logger.LogSink` protocol enables the component to write
  messages to the system log (`syslog`).
