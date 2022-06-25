# Monitor FIDL messages on a device

The [`ffx debug fidl`][ffx-debug-fidl] starts [`fidlcat`][fidlcat] (a
Fuchsia program) to monitor the [FIDL][fidl] traffic on a Fuchsia device.

## Concepts

`fidlcat` is a tool that allows you to monitor and debug FIDL messages
on a Fuchsia device. `fidlcat` establishes a connection to the device and
prints all FIDL messages that a Fuchsia component is sending and receiving
in real time.

Similar to other Fuchsia debugging tools, for
`fidlcat` to understand and unpack FIDL messages of a Fuchsia component,
the component's debug symbols must be available in your development
environment. (For more information on debug symbols, see
[Register debug symbols][register-debug-symbols].)

However, in addition to the debug symbols,`fidlcat` needs to obtain a list
of [FIDL intermediate representation][fidl-ir] (IR) files for the component.
This list is usually stored as a global file labeled `all_fidl_json.txt` in
your Fuchsia project. If you're using Fuchsia's Bazel rules, this list file
gets generated automatically as part of the build.

## Start fidlcat

Note: `fidl` cannot run together with [`zxdb`][zxdb] (the Fuchsia debugger).
They use the same `debug_agent` to connect to the device.

To monitor the FIDL traffic of a Fuchsia component on a device, run the
following command:

```posix-terminal
ffx debug fidl -f <COMPONENT> --fidl-ir-path <LIST_OF_IR_FILES>
```

Replace the following:

*   `COMPONENT` - The name of a Fuchsia component you want to monitor.
*   `LIST_OF_IR_FILES` - The path to a file that contains the list of
     IR files, with `@` as a prefix (for example,
     `@/usr/home/alice/my-fuchsia-project/all_fidl_json.txt`).

The example command below starts `fidlcat` that monitors the
`memory_monitor` component:

```none {:.devsite-disable-click-to-copy}
$ ffx debug fidl -f memory_monitor.cm --fidl-ir-path @/usr/home/alice/my-fuchsia-project/all_fidl_json.txt
```

When successfully connected to the device, this command prints output similar
to the following:

```none {:.devsite-disable-click-to-copy}
$ ffx debug fidl -f memory_monitor.cm --fidl-ir-path @/usr/home/alice/my-fuchsia-project/all_fidl_json.txt
INFO: [main.cc(242)] Connected to symbol server gs://our-fuchsia-artifacts/debug
INFO: [main.cc(125)] Connecting to /tmp/debug_agent_lpaMdL.socket...
INFO: [main.cc(92)] Connected!

```

From this point, the command prints the target component's FIDL messages
in real time, for example:

```none {:.devsite-disable-click-to-copy}
2701.44 Monitoring memory_monitor.cm koid=47467

2703.286866 memory_monitor.cm 47467:47469 zx_channel_call_etc(handle: handle = 7dd2c40b, options: uint32 = ZX_CHANNEL_WRITE_USE_IOVEC, deadline: zx.time = ZX_TIME_INFINITE, rd_num_bytes: uint32 = 65536, rd_num_handles: uint32 = 64)
  sent request fuchsia.kernel/Stats.GetMemoryStats = {}
2703.286866   -> ZX_OK
    received response fuchsia.kernel/Stats.GetMemoryStats = {
      stats: fuchsia.kernel/MemoryStats = {
        total_bytes: uint64 = 8455393280
        free_bytes: uint64 = 6727020544
        wired_bytes: uint64 = 102539264
        total_heap_bytes: uint64 = 32464896
        free_heap_bytes: uint64 = 1672120
        vmo_bytes: uint64 = 1540259840
        mmu_overhead_bytes: uint64 = 50339840
        ipc_bytes: uint64 = 278528
        other_bytes: uint64 = 0
      }
    }

2703.815113 memory_monitor.cm 47467:47469 zx_channel_write_etc(handle: handle = 7df30f83, options: uint32 = 0)
  sent request fuchsia.memory/Watcher.OnChange = {
    stats: fuchsia.memory/Stats = {
      total_bytes: uint64 = 8455393280
      free_bytes: uint64 = 6727020544
      wired_bytes: uint64 = 102539264
      total_heap_bytes: uint64 = 32464896
      free_heap_bytes: uint64 = 1672120
      vmo_bytes: uint64 = 1540259840
      mmu_overhead_bytes: uint64 = 50339840
      ipc_bytes: uint64 = 278528
      other_bytes: uint64 = 0
    }
  }
2703.815113   -> ZX_OK

...
```

To exit `fidlcat`, press `Ctrl-C`.

<!-- Reference links -->

[fidl]: development/languages/fidl/README.md
[fidlcat]: development/monitoring/fidlcat/fidlcat_usage.md
[zxdb]: ./start-the-fuchsia-debugger.md
[register-debug-symbols]: ./register-debug-symbols.md
[ffx-debug-fidl]: https://fuchsia.dev/reference/tools/sdk/ffx#fidl
[fidl-ir]: reference/fidl/language/json-ir.md
