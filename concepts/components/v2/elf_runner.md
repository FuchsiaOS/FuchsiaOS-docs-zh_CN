<!--
# ELF Runner
 -->
# ELF 运行器

<!--
The ELF runner is the runner responsible for launching
[components][glossary.component] based on standard executable files (ELF
format). It is a built-in runner and is available to all components.
 -->
ELF 运行器是负责启动基于标准可执行文件（ELF格式）的[组件][glossary.component]的运行器。它是内置运行器，可用于所有组件。

<!--
## Using the ELF Runner
 -->
## 使用 ELF 运行器

<!--
To use the ELF runner, the component's manifest must include a `program` block,
containing:
 -->
要使用 ELF 运行器，组件的清单必须包括一个 `program`（程序）块，其中包含：

```json5
{
    program: {
        runner: "elf",
        binary: "bin/foo",
    }
}

```

<!--
- The `runner` field with the value set to the string `elf`.
- The `binary` field with the value set to a binary output name in the component's package.
 -->
- `runner`（运行器）字段的值设置为字符串 `elf`。
- `binary`（二进制文件）字段的值设置为组件的包中二进制文件的输出名称。

<!--
## Fields
 -->
## 字段

<!--
Additionally, the ELF runner accepts a set of optional fields to configure
the ELF component's runtime environment.
 -->
此外，ELF 运行器接受一组可选字段来配置 ELF 组件的运行时环境。

<!--
### Arguments
 -->
### 参数

<!--
Arguments (that is, `argv` in most programming languages) can be passed to a
component's process using the `args` field. This field accepts a vector of
strings (see the example below).
The arguments set will be passed in the same order as declared in the manifest.
 -->
参数（即大多数编程语言中的 `argv`）可以使用 `args` 字段传递到组件的进程。该字段接受字符串向量（请参见下面的示例）。参数集将按照清单中声明的顺序传递。

```json5
{
    program: {
        runner: "elf",
        binary: "bin/foo",
        {{ '<strong>' }}args: ["--verbose", "--debug"]{{ '</strong>' }}
    }
}
```

<!--
### Forwarding stdout and stderr streams
 -->
### 转发 stdout 和 stderr 流

<!--
The stdout and stderr streams of ELF components can be routed to the
[LogSink service][logsink]. By default, the ELF runner only forwards these
streams if LogSink is available to the component. If your component prints
diagnostics messages to either of these streams, you should forward the streams
to the [LogSink service][logsink].
 -->
ELF 组件的 stdout 和 stderr 流可以路由到 [LogSink 服务][logsink]。默认情况下，ELF 运行器仅在 LogSink 可用于组件时才会转发这些流。如果您的组件将诊断消息输出到这些流中的任何一个，那么您应将流转发到 [LogSink 服务][logsink]。

<!--
To enable this feature, add the following to your manifest file:
 -->
要启用该功能，请将以下内容添加到您的清单文件中：

```json5
{
    include: [ "syslog/client.shard.cml" ],
}
```

<!--
After including this shard, all writes to stdout are logged as `INFO` messages,
and all writes to stderr are logged as `WARN` messages. Messages are split
by newlines and decoded as UTF-8 strings. Invalid byte sequences are converted
to the U+FFFD replacement character, which usually looks like `�`.
 -->
包括该分片后，所有到 stdout 的写操作都作为 `INFO`（信息）进行记录，而所有到 stderr 的写操作都作为 `WARN`（警告）消息记录。消息按换行分隔，并解码为 UTF-8 字符串。无效的字节序列转换为 U+FFFD 替代字符，通常看起来像 `�`。

<!--
Whether or not the syslog shard is included, this feature can be disabled with
explicit flags:
 -->
无论是否包括 syslog 分片，都可以使用显式标志来禁用该功能：

```json5
    program: {
        runner: "elf",
        binary: "bin/foo",
        {{ '<strong>' }}forward_stdout_to: "none",{{ '</strong>' }}
        {{ '<strong>' }}forward_stderr_to: "none",{{ '</strong>' }}
    }
```

<!--
Note: There are known issues where messages from `ZX_ASSERT_...` in C/C++
components and `Error` objects returned in `main` in Rust components are lost
when stdout/stderr forwarding is disabled.
 -->
注意：存在一些已知问题，当禁用 stdout/stderr 转发时，C/C++ 组件中来自 `ZX_ASSERT_...` 的消息和 Rust 组件中 `main` 中返回的 `Error` 对象会丢失。

<!--
### Lifecycle
 -->
### 生命周期

<!--
Components have a [lifecycle][lifecycle]. Components run by the ELF runner can
integrate with the lifecycle if you add a `lifecycle` attribute to your component
manifest. Currently `stop` is the only method in the Lifecycle protocol.
 -->
组件具有[生命周期][lifecycle]。如果您在组件清单中添加 `lifecycle`（生命周期）属性，那么由 ELF 运行器运行的组件可以与生命周期集成。当前 `stop`（停止）是生命周期协议中的唯一方法。

```json5
{
    program: {
        runner: "elf",
        binary: "bin/foo",
        {{ '<strong>' }}lifecycle: { stop_event: "notify" },{{ '</strong>' }}
    }
}
```

<!--
The program should take the handle to the Lifecycle channel and serve the
[Lifecycle protocol][lc-proto] on that channel. The component should exit after
receiving and processing the `stop` call.
 -->
该程序应获取生命周期通道的句柄并在该通道上提供[生命周期协议][lc-proto]。该组件应在接收并处理 `stop` 调用后退出。

<!--
The ELF Runner monitors the process it started for the program binary of the
component. If this process exits, the ELF runner will terminate the component's
execution context, which includes the component's job and all subprocesses.
 -->
ELF 运行器监控其为组件的程序二进制文件启动的进程。如果该进程退出，ELF 运行器将终止组件的执行上下文，其中包括组件的作业和所有子进程。

<!--
Note: For a complete lifecycle example, see
[//examples/components/lifecycle][lc-example].
 -->
注意：要获取关于完整生命周期的示例，请参阅 [//examples/components/lifecycle][lc-example]。

<!--
### Security
 -->
### 安全

<!--
There are several privileged fields that are gated by an
[allowlist][security-allowlist]. Only components included in this allowlist
are able to use these fields. For all fields, the policy applies to the first
process in the component. The first process is the one created by ELF runner
for the binary declared in `program` block. All of the fields are booleans
that default to `false`.
 -->
有几个特权字段受[允许列表][security-allowlist]（allowlist）控制。只有该允许列表中包含的组件才能使用这些字段。对于所有字段，该策略适用于组件中的首个进程。首个进程是由 ELF 运行器为 `program` 块中声明的二进制文件创建的进程。所有字段都是默认为 `false` 的布尔值。

<!--
#### Main Process Critical
 -->
#### 对主要进程关键

<!--
The `main_process_critical` field may be used to mark the component's first
process as [critical to component manager's job][job-set-critical], which will
cause component manager (and all components) to be terminated if the process
exits with a non-zero code. This will force the system to trigger a hard reboot.
 -->
`main_process_critical`（对主要进程关键）字段可用于将组件的首个进程标记为[对组件管理器作业关键][job-set-critical]，这将导致该进程以非零代码退出时组件管理器（和所有组件）被终止。从而将迫使系统触发硬重新启动。

<!--
#### Ambient VMO Exec
 -->
#### 环境 VMO 执行（Ambient VMO Exec）

<!--
The `ambient_mark_vmo_exec` field may be used to allow the component's first
process to use [`zx_vmo_replace_as_executable`][vmo-replace] with a
`ZX_HANDLE_INVALID` as the second argument rather than a valid
`ZX_RSRC_KIND_SYSTEM` with base `ZX_RSRC_SYSTEM_VMEX_BASE`.
 -->
`ambient_mark_vmo_exec`（环境标记 VMO 执行）字段可用于允许组件的首个进程使用 [`zx_vmo_replace_as_executable`][vmo-replace] 和 `ZX_HANDLE_INVALID` 作为第二个参数，而非使用基址为 `ZX_RSRC_SYSTEM_VMEX_BASE` 的有效 `ZX_RSRC_KIND_SYSTEM`。

<!--
#### Create Raw Processes
 -->
#### 创建原始进程

<!--
The `job_policy_create_raw_processes` field may be used to allow a component to
create processes by using [`zx_process_create`][process-create].
 -->
`job_policy_create_raw_processes`（作业策略创建原始进程）字段可用于允许组件通过使用 [`zx_process_create`][process-create] 创建进程。

```json5
{
    program: {
        runner: "elf",
        binary: "bin/foo",
        {{ '<strong>' }}job_policy_create_raw_processes: "true"{{ '</strong>' }}
    }
}
```

<!--
#### Is Shared Process
 -->
#### 为共享进程

<!--
The `is_shared_process` field may be used to pass the `ZX_PROCESS_SHARED` flag
when calling [`zx_process_create`][process-create]. This flag can only be used
if the component also has `job_policy_create_raw_processes` set to `true`.
 -->
`is_shared_process`（为共享进程）字段可用于调用 [`zx_process_create`][process-create] 时传递 `ZX_PROCESS_SHARED` 标志。该标志仅在组件也将 `job_policy_create_raw_processes` 设置为 `true` 时才能使用。

```json5
{
    program: {
        runner: "elf",
        binary: "bin/foo",
        job_policy_create_raw_processes: "true",
        {{ '<strong>' }}is_shared_process: "true"{{ '</strong>' }}
    }
}
```

<!--
## Further Reading
 -->
## 深入阅读

<!--
For a detailed explanation of how processes are created, please see
[Zircon program loading and dynamic linking][program-loading].
 -->
要获取关于进程是如何创建的详细说明，请参阅[Zircon 程序加载和动态链接] [program-loading]。

<!--
### Environment Variables
 -->
### 环境变量

<!--
Environment variables can be set for ELF components by using the `environ`
attribute. This field must be a vector of strings where each string contains
the variable and value delimited by an equal sign. For example, the following
sample code declares variables `FAVORITE_ANIMAL` and `FAVORITE_COLOR` to `cat`
and `red`.
 -->
可以使用 `environ`（环境）属性为 ELF 组件设置环境变量。该字段必须是字符串的向量，其中每个字符串包含由等符号界定的变量和值。例如，以下示例代码将变量 `FAVORITE_ANIMAL`（最爱动物）和`FAVORITE_COLOR`（最爱颜色）分别声明为 `cat`（猫）和 `red`（红色）。

```json5
{
    program: {
        runner: "elf",
        binary: "bin/echo",
        {{ '<strong>' }}environ: [{{ '</strong>' }}
            {{ '<strong>' }}"FAVORITE_ANIMAL=cat",{{ '</strong>' }}
            {{ '<strong>' }}"FAVORITE_COLOR=red",{{ '</strong>' }}
        {{ '<strong>' }}]{{ '</strong>' }}
    }
}
```

[glossary.component]: /glossary/README.md#component
[capability-routing]: capabilities/README.md#routing
[cml-shards]: https://fuchsia.dev/reference/cml#include
[lc-example]: /examples/components/lifecycle
[lc-proto]: /sdk/fidl/fuchsia.process.lifecycle/lifecycle.fidl
[lifecycle]: lifecycle.md
[program-loading]: /concepts/process/program_loading.md
[job-set-critical]: /reference/syscalls/job_set_critical.md
[job-set-policy]: /reference/syscalls/job_set_policy.md
[process-create]: /reference/syscalls/process_create.md
[vmo-replace]: /reference/syscalls/vmo_replace_as_executable.md
[fxb-72178]: https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=72178
[fxb-72764]: https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=72764
[logsink]: /development/diagnostics/logs/recording.md#logsinksyslog
[security-allowlist]: /src/security/policy/component_manager_policy.json5
[glossary-component-manifests]: /glossary/README.md#component-manifest
