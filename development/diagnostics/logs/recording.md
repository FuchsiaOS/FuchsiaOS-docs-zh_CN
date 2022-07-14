# Recording Logs

There are a few ways you might see a Fuchsia program generate log messages:

* the LogSink service
* the kernel's debuglog
* a combination of these, routed through `stdout` or `stderr`

## `LogSink`/syslog

Components that want to generate log messages call [`fuchsia.logger/LogSink.Connect`]. The
[`fuchsia.logger.LogSink`] service must be [allowlisted in the component manifest][logsink-cmx].

`Connect` takes a socket, into which the actual log messages are [written] by the syslog library.

If the socket's buffer is full, the [writing thread will drop logs]. Dropped messages on the writer
side are counted and that count is sent in the next successful message, after which the counter is
reset. `log_listener` [prints a warning] when it's aware of dropped messages.

The `LogSink` service must drain all of the sockets it receives quickly enough to prevent messages
being dropped on the writer-side. `LogSink` is responsible for draining those sockets to fill
[internal buffers]. This can result in high CPU usage in both the writing component and the
`LogSink` when logging heavily.

Different languages use different mechanisms to talk to `LogSink`. See the relevant pages for more
information:

* [C++ logging]
* [Dart logging]
* [Go logging]
* [Rust logging]

## debuglog handles

The kernel allows programs to create debuglog handles from the root resource, each handle allowing
its owner to write messages into the [kernel's shared ring buffer]. Each message has a limit of
`ZX_LOG_RECORD_DATA_MAX` bytes, with content in excess being truncated.

In addition to being [bindable to file descriptors], debuglog handles can be passed to the
[`debuglog_write`] and [`debuglog_read`] syscalls. The read syscall is used to transfer from the
debuglog to the system log.

Components that want to send their standard streams to the debuglog gain access through the
[`fuchsia.boot.WriteOnlyLog`] protocol.

Most logs written to debuglog handles are written through stdio forwarding.

## Standard streams: `stdout` and `stderr`

These concepts are familiar to many from other operating systems but their use in Fuchsia can be
complicated to follow because they are routed differently in different parts of the system.

### Drivers

Drivers log to the `LogSink` sink service, but do so through the use of [`zxlogf`]. This function
provides a wrapper around the syslog library, so that each driver will have its own log message
socket.

In addition, `driver_manager` binds `stdout` and `stderr` to debuglog. This allows `driver_manager`
to output critical information to the debuglog, or to fallback to the debuglog for certain product
configurations where the `LogSink` service is not available.

### Processes

The handles are [populated in procargs] by [appmgr] when creating processes and are pulled from
[`fuchsia.sys/LaunchInfo`] if provided. For example, `run-test-component` provides its own
`stdout`/`stderr` handles for test components so it can prevent that output from reaching the klog.

If no fd's are provided by the caller of `CreateComponent,` then the handles are [cloned] from
appmgr's own `stdout` and `stderr`. appmgr populates its own stdio with debuglog handles, using the
[`stdout-to-debuglog`] library to wire up a handle received from [`fuchsia.boot.WriteOnlyLog`].

### Components

[Components] don't have their `stdout` and `stderr` streams captured by default.
For [ELF] components, there are flags used to tell the ELF runner to redirect
the output of these stream to the `LogSink` service. For more information, see
the ELF runner section on [forwarding stdout and stderr streams].

## Forwarding klog to syslog

The Archivist continually reads from the klog and forwards those messages to the main log. Messages
from the klog can be dropped by the pipeline if they are rolled out of the klog buffer before the
archivist reads them into syslog -- these are not yet tracked.

All kernel log messages are sent to the system log with INFO severity because the debuglog syscalls
lack a way to express the severity of a message.

[`fuchsia.logger/LogSink.Connect`]: https://fuchsia.dev/reference/fidl/fuchsia.logger#Connect
[`fuchsia.logger.LogSink`]: https://fuchsia.dev/reference/fidl/fuchsia.logger#LogSink
[logsink-cmx]: https://fuchsia.googlesource.com/fuchsia/+/1bdbf8a4e6f758c3b1782dee352071cc592ca3ab/src/lib/ui/carnelian/meta/example.cmx#15
[written]: https://fuchsia.googlesource.com/fuchsia/+/1bdbf8a4e6f758c3b1782dee352071cc592ca3ab/zircon/system/ulib/syslog/fx_logger.cc#72
[writing thread will drop logs]: https://fuchsia.googlesource.com/fuchsia/+/1bdbf8a4e6f758c3b1782dee352071cc592ca3ab/zircon/system/ulib/syslog/fx_logger.cc#130
[prints a warning]: https://fuchsia.googlesource.com/fuchsia/+/1bdbf8a4e6f758c3b1782dee352071cc592ca3ab/garnet/bin/log_listener/src/main.rs#708
[internal buffers]: https://fuchsia.googlesource.com/fuchsia/+/1bdbf8a4e6f758c3b1782dee352071cc592ca3ab/src/diagnostics/archivist/src/logs.rs#47
[C++ logging]: /development/languages/c-cpp/logging.md
[Dart logging]: /development/languages/dart/logging.md
[Go logging]: /development/languages/go/logging.md
[Rust logging]: /development/languages/rust/logging.md
[kernel's shared ring buffer]: https://fuchsia.googlesource.com/fuchsia/+/1bdbf8a4e6f758c3b1782dee352071cc592ca3ab/zircon/kernel/lib/debuglog/debuglog.cc#37
[bindable to file descriptors]: https://fuchsia.googlesource.com/fuchsia/+/1bdbf8a4e6f758c3b1782dee352071cc592ca3ab/sdk/lib/fdio/include/lib/fdio/fdio.h#36
[`debuglog_write`]: /reference/syscalls/debuglog_write.md
[`debuglog_read`]: /reference/syscalls/debuglog_read.md
[`zxlogf`]: https://fuchsia.googlesource.com/fuchsia/+/1bdbf8a4e6f758c3b1782dee352071cc592ca3ab/src/lib/ddk/include/ddk/debug.h#103
[kernel params]: /reference/kernel/kernel_cmdline.md#drivernamelogflags
[populated in procargs]: https://fuchsia.googlesource.com/fuchsia/+/1bdbf8a4e6f758c3b1782dee352071cc592ca3ab/src/sys/appmgr/realm.cc#140
[`fuchsia.sys/LaunchInfo`]: https://fuchsia.dev/reference/fidl/fuchsia.sys#LaunchInfo
[cloned]: https://fuchsia.googlesource.com/fuchsia/+/1bdbf8a4e6f758c3b1782dee352071cc592ca3ab/src/sys/appmgr/realm.cc#69
[`stdout-to-debuglog`]: /src/sys/lib/stdout-to-debuglog
[`fuchsia.boot.WriteOnlyLog`]: https://fuchsia.dev/reference/fidl/fuchsia.boot#WriteOnlyLog
[appmgr]: /src/sys/appmgr/README.md
[`ddk/debug.h`]: /src/lib/ddk/include/ddk/debug.h
[Components]: /concepts/components/v2/introduction.md
[ELF]: /concepts/components/v2/elf_runner.md
[forwarding stdout and stderr streams]: /concepts/components/v2/elf_runner.md#forwarding_stdout_and_stderr_streams