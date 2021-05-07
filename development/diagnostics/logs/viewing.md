# Viewing logs

Logs are primarily consumed in either an interactive ("online") context with a live device, or in an
"offline" context with logs collected from past execution of a device.

## Ordering

All logs have a timestamp attached, which is read from the [monotonic clock] when recording the
message. There are many ways that a `LogSink` can receive messages in a different order than
indicated by their timestamps.

The primary [`fuchsia.logger.Log`] implementation sorts messages it is sending via the `LogMany`
method on [`fuchsia.logger.LogListenerSafe`]. This method is called whenever sending
already-collected messages to a newly-connected listener. However, when messages first arrive with
out-of-order timestamps, any active listeners will receive them in an arbitrary ordering. Tools
which display logs accumulated from successive calls to `fuchsia.logger/LogListenerSafe.Log` should
alert their users when messages are received without a strictly linear ordering in timestamps.

## Online

Because there are two buffers that store logs, there are two main ways to view them when you have a
live device. For more information about where logs are stored on-device, see [Concepts: Storage].

### syslog and kernel log

During development, running `fx log` is a good default to see all logs. This script connects to the
target device with SSH and runs the [`log_listener`] program, printing every message from the system
log. This includes those [forwarded from the klog].

[`log_listener`] receives logs through the [`fuchsia.logger.Log`] and
[`fuchsia.logger.LogListenerSafe`] protocols.

Additionally, some logs from syslog are printed to the serial console. By default, this includes the
driver and `driver_manager` logs.

#### Format

[`log_listener`] emits lines in this format by default:

```
[seconds][pid][tid][tags] LEVEL: message
```

The timestamp is from the monotonic clock by default, and it is formatted with microsecond
granularity.

If the message "something happened" is written at WARN level by my-component.cmx from process=1902
and thread=1904 at time=278.14, the default output would be:

```
[278.14][1902][1904][my-component.cmx] WARN: something happened
```

With a running device available, run `fx log -h` to see the options for modifying the output format.

#### `fx test`

Under the hood, `fx test` calls `run-test-component`, which collects isolated `stdout`, `stderr`, and
`LogSink` connections from test components, printing the output inline and preventing them showing
up in the global log buffers.

For tests that are not yet components no interception of logs is performed.

### kernel log only

The klog is [printed over the kernel console] and serial.

It's also [forwarded over UDP by netsvc], which is what's printed when you run `fx klog`. Running
`fx klog` in a background terminal can be a good way to capture logs if your SSH session fails, or
as a backup if there are other issues with running `fx log`.

If neither of the above are options, you can also use [`dlog`] from a device shell directly to dump
the kernel debug logs.

#### Format

The kernel log's dumper emits lines in the format:

```
[timestamp] pid:tid> message
```

The timestamp is from the monotonic clock. It is formatted with 5 digits (leading zeroes) for
seconds and three digits for milliseconds (trailing zeroes).

Process and thread koids are written with 5 digits each (leading zeroes).

If the message "something happened" is written from process=1902 and thread=1904 at time=278.14, the
resulting output would be:

```
[00278.140] 01902:01904> something happened
```

## Offline: CQ/GI

When running tests, a [Swarming] bot invokes [botanist], which collects several output streams to be
presented in the web UI. The `stdout` & `stderr` of botanist are what's presented in the "swarming task
UI".

For individual test executables botanist invokes [testrunner] and collects that output separately.
It is this output that can be seen after a failing test, with a link named `stdio`. Most tests that
testrunner invokes run `run-test-component` via SSH to the target device. This collects the
stdout, stderr, and logs from the test environment and prints them inline.

### syslog.txt

Botanist runs `log_listener` on the target device and saves that output to syslog.txt. This is
comparable to running `fx log` at a development machine.

### infra_and_test_std_and_klog.txt

This log includes the stdout and stderr of the command run by the [Swarming] task.
Normally this includes the following notable items, all interleaved:

* [botanist]'s log messages
* kernel log from netsvc (equivalent to `fx klog`)
* [testrunner]'s log messages
* `stdout` and `stderr` of the tests run by testrunner

This aggregate log is run through the equivalent of `fx symbolize` before upload.

[monotonic clock]: /docs/reference/syscalls/clock_get_monotonic.md
[Concepts: Storage]: /docs/concepts/diagnostics/logs/README.md#storage
[forwarded from the klog]: /docs/development/diagnostics/logs/recording.md#forwarding-klog-to-syslog
[`log_listener`]: /garnet/bin/log_listener/README.md
[`fuchsia.logger.Log`]: https://fuchsia.dev/reference/fidl/fuchsia.logger#Log
[`fuchsia.logger.LogListenerSafe`]: https://fuchsia.dev/reference/fidl/fuchsia.logger#LogListenerSafe
[printed over the kernel console]: /zircon/kernel/lib/debuglog/debuglog.cc
[forwarded over UDP by netsvc]: /src/bringup/bin/netsvc/debuglog.cc
[`dlog`]: /src/bringup/bin/dlog/README.md
[botanist]: /tools/botanist/cmd/main.go
[testrunner]: /tools/testing/testrunner/cmd/main.go
[Swarming]: https://chromium.googlesource.com/infra/luci/luci-py/+/HEAD/appengine/swarming/doc/README.md
