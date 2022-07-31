<!-- ## Component manifests -->
## 组件清单

<!-- 
CML files are [JSON5](https://json5.org/){: .external} files that end with a
`.cml` extension. Below is an example CML manifest file for a simple component
running an ELF binary that prints a "Hello, World" message to the system log:
 -->
CML 文件是以 `.cml` 扩展名结尾的 [JSON5](https://json5.org/){: .external} 文件。如下 CML 清单文件示例，描述了一个运行 ELF 二进制文件的简单组件，该文件向系统日志打印一条“Hello, World”信息：

<!-- 
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
 -->
```json5
{
    // 有关要运行的程序的信息。
    program: {
        // 使用内置的 ELF 运行器。
        runner: "elf",
        // 为此组件运行的二进制文件。
        binary: "bin/hello",
        // 程序参数
        args: [
            "Hello",
            "World!",
        ],
    },

    // 此组件使用的能力。
    use: [
        { protocol: "fuchsia.logger.LogSink" },
    ],
}
```

<!-- 
This file declares two main sections of information about the component:
 -->
该文件声明了关于组件的两个主要部分的信息：

<!-- 
Note: For more details on component manifests, see
[component manifests](/concepts/components/v2/component_manifests.md).
 -->
注意：要获取组件清单（component manifest）的更多详细信息，请参阅[组件清单](/concepts/components/v2/component_manifests.md)。

<!-- 
* `program`: Describes the executable information such as the binary file,
  program arguments, and the associated runtime. In this example, a binary
  is compiled as an ELF executable and uses the built-in
  [ELF runner](/concepts/components/v2/elf_runner.md).
 -->
* `program`：描述可执行信息，例如二进制文件、程序参数和相关联的运行时。在此示例中，二进制文件被编译为 ELF 可执行文件并使用内置的 [ELF 运行器](/concepts/components/v2/elf_runner.md)。
<!-- 
* `use`: Declares the capabilities this component requires to run. In this
  example, the `fuchsia.logger.LogSink` protocol enables the component to write
  messages to the system log (`syslog`).
 -->
* `use`：声明此组件运行所需的功能。在此示例中，`fuchsia.logger.LogSink` 协议使该组件能够向系统日志（syslog）写入消息。
