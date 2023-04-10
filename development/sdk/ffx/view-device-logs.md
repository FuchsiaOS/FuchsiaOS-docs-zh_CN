# View device logs

The [`ffx log`][ffx-log] commands can monitor and filter log messages from a Fuchsia device.

## Concepts

The `ffx` daemon, which runs in the background on the host machine, proactively discovers
Fuchsia devices and automatically connects to them as they become reachable. With the
[proactive logging][proactive-logging] feature enabled (which is the default setting for
`ffx`), the `ffx` daemon starts reading device logs from the target device and
caches the logs on the host machine, up to a configured space limit.

When the space limit for the stored logs is reached on the host machine, the logs get
rotated, meaning the oldest logs are deleted to make room for the latest ones. Additionally,
logs are [symbolized][symbolize-logs] in the background as they are read from the device,
before they are written to the cache on the host machine.

By default, the `ffx log` command prints all device logs and leaves the connection open
to continuously stream new logs from the target device.

## Monitor device logs {:#monitor-device-logs}

To monitor device logs in real time, run the following command:

```posix-terminal
ffx log
```
This command prints output similar to the following:

```none {:.devsite-disable-click-to-copy}
$ ffx log
[2022-03-29 18:43:40.973][<ffx>]: logger started.
[0.000][klog][klog][I] printing enabled
[0.000][klog][klog][I] INIT: cpu 0, calling hook 0xffffffff00177fc0 (elf_build_id) at level 0x1, flags 0x1
[0.000][klog][klog][I] ACPI LITE: Found valid RSDT table at physical address 0x7ffe21cc
[0.000][klog][klog][I] PMM: boot reserve add [0x100000, 0x488fff]
[0.000][klog][klog][I] PMM: boot reserve add [0x3502000, 0x46d7fff]
[0.000][klog][klog][I] PMM: adding arena 0xffffffff00485128 name 'memory' base 0x100000 size 0x7fede000
[0.000][klog][klog][I] PMM: adding arena 0xffffffff00485128 name 'memory' base 0x100000000 size 0x180000000
[0.000][klog][klog][I] PMM: boot reserve marking WIRED [0x100000, 0x488fff]
[0.000][klog][klog][I] PMM: boot reserve marking WIRED [0x3502000, 0x46d7fff]
[0.000][klog][klog][I] UART: kernel serial enabled: port=0x3f8, irq=0x4
[0.000][klog][klog][I] UART: enabled with FIFO depth 64
[0.000][klog][klog][I] INIT: cpu 0, calling hook 0xffffffff00342380 (intel_rng_init) at level 0x20001, flags 0x1
[0.000][klog][klog][I] INIT: cpu 0, calling hook 0xffffffff0015dee0 (global_prng_seed) at level 0x20002, flags 0x1
[0.000][klog][klog][I]
[0.000][klog][klog][I] welcome to Zircon
[0.000][klog][klog][I]
[0.000][klog][klog][I] KASLR: .text section at 0xffffffff00100000
[0.000][klog][klog][I] initializing arch pre-vm
...
```

Press `CTRL+C` to exit.

## Filter device logs {:#filter-device-logs}

To filter device logs in real time, run the following command:

Note: The `--filter` flag also works with the [`dump`](#dump-device-logs) option.

```posix-terminal
ffx log --filter <STRING>
```

Replace `STRING` with a string you want to use to filter the logs. Use quotation marks
(`""`) if it contains spaces.

The example below monitors device logs and only prints messages that contain
the string `Hello, World!`:

```none {:.devsite-disable-click-to-copy}
$ ffx log --filter "Hello, World!"
[252.071][core/ffx-laboratory:hello_world][][I] Hello, World!

```

Press `CTRL+C` to exit.

A common use case for this feature is to select a component (for example,
`hello_world` below) and monitor logs only from that component:

```none {:.devsite-disable-click-to-copy}
$ ffx log --filter hello_world
[177262.742][core/pkg-resolver][pkg-resolver][I] Fetching blobs for fuchsia-pkg://fuchsiasamples.com/hello_world: []
[177262.762][core/pkg-resolver][pkg-resolver][I] resolved fuchsia-pkg://fuchsiasamples.com/hello_world as fuchsia-pkg://fuchsiasamples.com/hello_world to ff6a0f00bde933c0fb393405ccf2d4c17e6eabeb9bfc82c9bf91db35693933ca with TUF
[177262.788][core/pkg-resolver][pkg-resolver][I] Fetching blobs for fuchsia-pkg://fuchsiasamples.com/hello_world: []
[177262.789][core/pkg-resolver][pkg-resolver][I] resolved fuchsia-pkg://fuchsiasamples.com/hello_world as fuchsia-pkg://fuchsiasamples.com/hello_world to ff6a0f00bde933c0fb393405ccf2d4c17e6eabeb9bfc82c9bf91db35693933ca with TUF
[177262.816][core/ffx-laboratory:hello_world][][I] Hello, World!

```

## Filter device logs using log levels {:#filter-device-logs-using-log-levels}

To filter device logs based on their log level in real time, run the following command:

Note: The `--severity` flag also works with the [`dump`](#dump-device-logs) option.

```posix-terminal
ffx log --severity <LOG_LEVEL>
```

Replace `LOG_LEVEL` with a log level you want to use to filter the logs. Supported log levels are
`error`, `warn`, `info`, `debug`, and `trace`.

The example below monitors device logs and only prints messages with the log level `WARN`:

```none {:.devsite-disable-click-to-copy}
$ ffx log --severity warn
...
[166547.984][core/network/netstack][netstack,DHCP][W] client.go(692): ethp0004: recv timeout waiting for dhcpOFFER; retransmitting dhcpDISCOVER
[166564.788][core/network/netstack][netstack,DHCP][W] client.go(692): ethp0004: recv timeout waiting for dhcpOFFER; retransmitting dhcpDISCOVER
[166567.571][core/wlancfg][wlancfg_lib::client::scan,wlan][W] Failed to get an SME proxy for scan: no client ifaces available
[166567.571][core/session-manager/session:session/workstation_session/login_shell/ermine_shell][ermine][W] Error encountered during scan: MethodException: ScanErrorCode(1)
[166568.746][core/detect][detect][W] Snapshot trigger was missing: ValueError: Division by zero
[166568.749][core/detect][detect][W] Snapshot trigger was missing: ValueError: Division by zero

```

Press `CTRL+C` to exit.

## Dump device logs {:#dump-device-logs}

The `ffx log dump` command prints all device logs in a session and exits immediately.

To dump all device logs, run the following command:

```posix-terminal
ffx log dump
```

This command prints output similar to `ffx log`, but exits after printing the logs.

## Dump device logs from a previous session {:#dump-device-logs-from-a-previous-session}

The `ffx log dump` command can print device logs from the target device’s previous session.

To dump device logs from a selected session, run the following command:

```posix-terminal
ffx log dump <SESSION_NUMBER>
```

Replace `SESSION_NUMBER` with a number with `~` as a prefix, for example, `~1`, `~2`, `~3,`
and so on.

The example below prints the device logs from the second previous session:

```none {:.devsite-disable-click-to-copy}
$ ffx log dump ~2
```

A session starts on each booting of a device. For session numbers, consider `0` is
reserved for the current, active session on the device. From there, `~1` is the most
recent previous session (that is, the session before the last reboot), and `~2` is the
second most recent previous session.

## Dump device logs from some time ago {:#dump-device-logs-from-some-time-ago}

To dump device logs starting from some specific time in the past, run the following command:

Note: The `--since` flag also works without the `dump` option.

```posix-terminal
ffx log --since <TIME> dump
```

Replace `TIME` with a point of time in human readable format (for example, `"30m ago"`
or `"2h ago"`) which you want to see the logs from.

The example below prints the device logs from 5 minutes ago to the latest message:

```none {:.devsite-disable-click-to-copy}
$ ffx log --since "5m ago" dump
```

<!-- Reference links -->

[ffx-log]: https://fuchsia.dev/reference/tools/sdk/ffx#log_2
[proactive-logging]: /development/tools/ffx/commands/log.md#proactive_logging
[symbolize-logs]: ./symbolize-logs.md
