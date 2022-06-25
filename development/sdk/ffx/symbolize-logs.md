# Symbolize logs

The [`ffx debug symbolize`][ffx-debug-symbolize] command processes logs
into symbolized format (that is, human-readable stack traces).

## Concepts

The example below shows symbolized logs from a Fuchsia device:

```none {:.devsite-disable-click-to-copy}
$ ffx log --kernel
...
[326.124][klog][klog][I]    #0    0x0000428d2319f74b in abort() ../../zircon/third_party/ulib/musl/src/exit/abort.c:7 <libc.so>+0x6474b sp 0x27c83188f70
[326.124][klog][klog][I]    #1    0x000001cb1faef047 in main() src/hello_world/hello_world.cc:9 <<VMO#113249=blob-30a652b1>>+0x2047 sp 0x27c83188f80
[326.124][klog][klog][I]    #2    0x0000428d2319eef2 in start_main(const start_params*) ../../zircon/third_party/ulib/musl/src/env/__libc_start_main.c:140 <libc.so>+0x63ef2 sp 0x27c83188fa0
[326.124][klog][klog][I]    #3    0x0000428d2319f145 in __libc_start_main(zx_handle_t, int (*)(int, char**, char**)) ../../zircon/third_party/ulib/musl/src/env/__libc_start_main.c:215 <libc.so>+0x64145 sp 0x27c83188ff0
[326.124][klog][klog][I]    #4    0x000001cb1faef011 in _start(zx_handle_t) ../../zircon/system/ulib/c/Scrt1.cc:7 <<VMO#113249=blob-30a652b1>>+0x2011 sp 0x4046d0234fe0
[326.124][klog][klog][I]    #5    0x0000000000000000 is not covered by any module sp 0x4046d0234ff0
```

With symbolized logs, you can read the file names and line
numbers of the source code (for example,
`in main() src/hello_world/hello_world.cc:9`) where an error has occurred.

By default, the [`ffx log`][ffx-log] command automatically prints symbolized
logs when debug symbols are registered in your environment. (To turn off this
feature, run `ffx log -–no-symbols`.) The `ffx debug symbolize` command
(which `ffx log` uses in the background) takes logs with symbolization markups
as input and generates symbolized logs as output. This process works as long as
the debug symbols corresponding to such markups are registered in your
environment (see [Register debug symbols][register-debug-symbols]).

## Symbolize logs from an external source

During development, the `ffx log` automatically processes registered debug
symbols and prints symbolized logs, so you may never need to manually process
logs into symbolized format in your environment. But in some cases, you may
obtain a copy of raw logs from an external source (for instance, output from
a Fuchsia builder). If you want to process such logs into symbolized format,
you can pass the logs as input to the `ffx debug symbolize` command.

To manually process logs into symbolize format, do the following:

Note: Symbolizing logs from an external source requires that the corresponding
debug symbols are registered in your environment or available online
(see [Register debug symbols][register-debug-symbols]).

1. Start the symbolize tool:

   ```posix-terminal
   ffx debug symbolize
   ```

   The terminal now waits for input.

2. Copy raw logs containing symbolization markups.
3. Paste it in the `ffx debug symbolize` terminal.

   The tool prints the logs in symbolized format.

<!-- Reference links -->

[ffx-debug-symbolize]: https://fuchsia.dev/reference/tools/sdk/ffx#symbolize
[ffx-log]: https://fuchsia.dev/reference/tools/sdk/ffx#log_2
[register-debug-symbols]: ./register-debug-symbols.md
