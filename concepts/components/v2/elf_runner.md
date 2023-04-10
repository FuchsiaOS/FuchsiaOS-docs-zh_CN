# ELF Runner

The ELF runner is the runner responsible for launching
[components][glossary.component] based on standard executable files (ELF
format). It is a built-in runner and is available to all components.

## Using the ELF Runner

To use the ELF runner, the component's manifest must include a `program` block,
containing:

```json5
{
    program: {
        runner: "elf",
        binary: "bin/foo",
    }
}

```

- The `runner` field with the value set to the string `elf`.
- The `binary` field with the value set to a binary output name in the component's package.

## Fields

Additionally, the ELF runner accepts a set of optional fields to configure
the ELF component's runtime environment.

### Arguments

Arguments (that is, `argv` in most programming languages) can be passed to a
component's process using the `args` field. This field accepts a vector of
strings (see the example below).
The arguments set will be passed in the same order as declared in the manifest.

```json5
{
    program: {
        runner: "elf",
        binary: "bin/foo",
        {{ '<strong>' }}args: ["--verbose", "--debug"]{{ '</strong>' }}
    }
}
```

### Forwarding stdout and stderr streams

The stdout and stderr streams of ELF components can be routed to the
[LogSink service][logsink]. By default, the ELF runner only forwards these
streams if LogSink is available to the component. If your component prints
diagnostics messages to either of these streams, you should forward the streams
to the [LogSink service][logsink].

To enable this feature, add the following to your manifest file:

```json5
{
    include: [ "syslog/client.shard.cml" ],
}
```

After including this shard, all writes to stdout are logged as `INFO` messages,
and all writes to stderr are logged as `WARN` messages. Messages are split
by newlines and decoded as UTF-8 strings. Invalid byte sequences are converted
to the U+FFFD replacement character, which usually looks like `�`.

Whether or not the syslog shard is included, this feature can be disabled with
explicit flags:

```json5
    program: {
        runner: "elf",
        binary: "bin/foo",
        {{ '<strong>' }}forward_stdout_to: "none",{{ '</strong>' }}
        {{ '<strong>' }}forward_stderr_to: "none",{{ '</strong>' }}
    }
```

Note: There are known issues where messages from `ZX_ASSERT_...` in C/C++
components and `Error` objects returned in `main` in Rust components are lost
when stdout/stderr forwarding is disabled.

### Lifecycle

Components have a [lifecycle][lifecycle]. Components run by the ELF runner can
integrate with the lifecycle if you add a `lifecycle` attribute to your component
manifest. Currently `stop` is the only method in the Lifecycle protocol.

```json5
{
    program: {
        runner: "elf",
        binary: "bin/foo",
        {{ '<strong>' }}lifecycle: { stop_event: "notify" },{{ '</strong>' }}
    }
}
```

The program should take the handle to the Lifecycle channel and serve the
[Lifecycle protocol][lc-proto] on that channel. The component should exit after
receiving and processing the `stop` call.

The ELF Runner monitors the process it started for the program binary of the
component. If this process exits, the ELF runner will terminate the component's
execution context, which includes the component's job and all subprocesses.

Note: For a complete lifecycle example, see
[//examples/components/lifecycle][lc-example].

### Security

There are several privileged fields that are gated by an
[allowlist][security-allowlist]. Only components included in this allowlist
are able to use these fields. For all fields, the policy applies to the first
process in the component. The first process is the one created by ELF runner
for the binary declared in `program` block. All of the fields are booleans
that default to `false`.

#### Main Process Critical

The `main_process_critical` field may be used to mark the component's first
process as [critical to component manager's job][job-set-critical], which will
cause component manager (and all components) to be terminated if the process
exits with a non-zero code. This will force the system to trigger a hard reboot.

#### Ambient VMO Exec

The `ambient_mark_vmo_exec` field may be used to allow the component's first
process to use [`zx_vmo_replace_as_executable`][vmo-replace] with a
`ZX_HANDLE_INVALID` as the second argument rather than a valid
`ZX_RSRC_KIND_SYSTEM` with base `ZX_RSRC_SYSTEM_VMEX_BASE`.

#### Create Raw Processes

The `job_policy_create_raw_processes` field may be used to allow a component to
create processes by using [`zx_process_create`][process-create].

```json5
{
    program: {
        runner: "elf",
        binary: "bin/foo",
        {{ '<strong>' }}job_policy_create_raw_processes: "true"{{ '</strong>' }}
    }
}
```

#### Is Shared Process

The `is_shared_process` field may be used to pass the `ZX_PROCESS_SHARED` flag
when calling [`zx_process_create`][process-create]. This flag can only be used
if the component also has `job_policy_create_raw_processes` set to `true`.

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

## Further Reading

For a detailed explanation of how processes are created, please see
[Zircon program loading and dynamic linking][program-loading].

### Environment Variables

Environment variables can be set for ELF components by using the `environ`
attribute. This field must be a vector of strings where each string contains
the variable and value delimited by an equal sign. For example, the following
sample code declares variables `FAVORITE_ANIMAL` and `FAVORITE_COLOR` to `cat`
and `red`.

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

[glossary.component]: /docs/glossary/README.md#component
[capability-routing]: capabilities/README.md#routing
[cml-shards]: https://fuchsia.dev/reference/cml#include
[lc-example]: /examples/components/lifecycle
[lc-proto]: /sdk/fidl/fuchsia.process.lifecycle/lifecycle.fidl
[lifecycle]: lifecycle.md
[program-loading]: /docs/concepts/process/program_loading.md
[job-set-critical]: /docs/reference/syscalls/job_set_critical.md
[job-set-policy]: /docs/reference/syscalls/job_set_policy.md
[process-create]: /docs/reference/syscalls/process_create.md
[vmo-replace]: /docs/reference/syscalls/vmo_replace_as_executable.md
[fxb-72178]: https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=72178
[fxb-72764]: https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=72764
[logsink]: /docs/development/diagnostics/logs/recording.md#logsinksyslog
[security-allowlist]: /src/security/policy/component_manager_policy.json5
[glossary-component-manifests]: /docs/glossary/README.md#component-manifest
