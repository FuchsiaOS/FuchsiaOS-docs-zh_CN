<!-- ## Manifest shards -->
## 清单碎片

<!-- 
Some collections of capabilities represent use case requirements that are common
to many components in the system, such as logging. To simplify including these
capabilities in your components, the framework abstracts them into
**manifest shards** that can be included in your CML source file.
 -->
一些能力集合代表了系统中许多组件所共有的用例需求，例如日志记录。为了简化将这些能力纳入组件的过程，本框架将其抽象为**清单碎片**（Manifest shard），可将其纳入 CML 源文件中。

<!-- 
Below is an equivalent CML to the previous example. In this case, the necessary
logging capabilities are provided by including
`diagnostics/syslog/client.shard.cml` instead of declaring
`fuchsia.logger.LogSink` explicitly:
 -->
下面是一个与前面的示例等效的 CML。在这种情况下，通过包含 `diagnostics/syslog/client.shard.cml` 文件，而不是显式地声明 `fuchsia.logger.LogSink` 来提供必要的日志记录功能：

<!-- 
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
 -->
```json5
{
    include: [ "syslog/client.shard.cml" ],

    // 有关要运行的程序的信息。
    program: {
        // 使用内置的 ELF 运行器。
        runner: "elf",
        // 为此组件运行的二进制文件。
        binary: "bin/hello-world",
        // 程序参数
        args: [
            "Hello",
            "World!",
        ],
    },
}
```
