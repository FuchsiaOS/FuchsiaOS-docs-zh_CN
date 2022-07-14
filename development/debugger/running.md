# Set up and run zxdb, Fuchsia's debugger for C++ and Rust code

Launching zxdb on Fuchsia is as simple as one command:

```posix-terminal
ffx debug connect
```

When the debugger is launched, it should show

```none
Connecting (use "disconnect" to cancel)...
Connected successfully.
👉 To get started, try "status" or "help".
[zxdb]
```

It'll work 95% of the time, regardless of whether you work in-tree or out-of-tree, using an emulator
or a hardware device. If it doesn't work as expected, please check the troubleshooting below.

## Troubleshooting

### The build type is compatible

Debuggers on Fuchsia depend on privileged syscalls, most notably
[`zx_process_write_memory`](/reference/syscalls/process_write_memory.md).
These syscalls are only enabled when the kernel flag
[`kernel.enable-debugging-syscalls`](/gen/boot-options.md#kernelenable-debugging-syscallsbool)
is set to `true`, which means debuggers are not available for `user` and `userdebug`
[build types](/contribute/governance/rfcs/0115_build_types.md).

If you are building from source, most probably these syscalls are enabled.

### zxdb and debug_agent are built

Zxdb depends on a target-side component called debug\_agent.  If an error message says "The plugin
service selector 'core/debug\_agent:expose:fuchsia.debugger.DebugAgent' did not match any services
on the target", it means that debug\_agent is not built.  You can also check whether there's
`debug_agent` and `host_x64/zxdb` in your build directory.

If you don't have the debugger in your build, add `//bundles:tools` to your "universe", either with:

```posix-terminal
fx <normal_stuff_you_use> --with //bundles:tools
```

Or you can edit your GN args directly by editing `<build_dir>/args.gn` and adding to the bottom:

```none
universe_package_labels += [ "//bundles:tools" ]
```

### ffx can talk with the device

Make sure that ffx can discover the device, either a emulator or a hardware device, and
RCS is started on the device.

```
$ ffx target list
NAME       SERIAL       TYPE             STATE      ADDRS/IP                    RCS
demo-emu   <unknown>    core.qemu-x64    Product    [10.0.2.15,                 Y
                                                    fec0::90e:486e:b6b5:9780,
                                                    fec0::487b:fabd:20fa:43ee,
                                                    127.0.0.1]
```

### Package server is running

For most build configurations, the debug agent will be in the "universe" (i.e. "available to use")
but not in the base build so won't be on the system before boot. You will need to run:

```posix-terminal
fx serve
```

### Debug symbols are registered {#set-symbol-location}

Zxdb will by default obtain the locations of the debug symbols from the
[symbol index](/development/sdk/ffx/register-debug-symbols.md).
The registrations of debug symbols from in-tree and most out-of-tree environments are automated.
In case these doesn't work out, there are three command-line flags in zxdb to provide additional
symbol lookup locations for zxdb: `--build-id-dir`, `--ids-txt`, and a general `--symbol-path`.
They all have the corresponding settings that can be manipulated using `set` or `get`.

For example, to add a ".build-id" directory, either use `--build-id-dir` flag:

```posix-terminal
ffx debug connect -- --build-id-dir some/other_location/.build-id
```

Or add it to the `build-id-dirs` list option in the interactive UI:

```none {:.devsite-disable-click-to-copy}
[zxdb] set build-id-dirs += some/other_location/.build-id
```

For in-tree development, `ffx debug connect` automatically sets up all necessary
flags.

#### `build-id-dir`

Some builds produce a `.build-id` directory. Symbol files in it are already indexed according to
their build IDs. For example, the Fuchsia build itself makes a `.build-id` directory inside the
build directory, e.g., `out/x64/.build-id`. They can be added to zxdb by `--build-id-dir`
command-line flag or `build-id-dirs` setting. This is the best option.

#### `ids-txt`

Instead of a `.build-id` directory, some builds produce a file called `ids.txt` that lists build IDs
and local paths to the corresponding binaries. They can be added to zxdb by `--ids-txt` command-line
flag or `ids-txts` setting. This is the second-best option.

#### `symbol-path`

In addition, `--symbol-path` flag can be used to add arbitrary files or directories to symbol index.
If the path is pointing to a file, it will be treated as an ELF file and added to the symbol index.
If it's a directory, all binaries under the given path are indexed.

### Source code location is correctly set up

The Fuchsia build generates symbols relative to the build directory so relative paths look like
`../../src/my_component/file.cc`). The build directory is usually provided by the symbol index.

If your files are not being found with the default build directories, you will need to provide a
build directory to locate the files. This build directory does not need have been used to build, it
just needs to produce correct absolute paths when concatenated with the relative paths from the
symbol file.

You can add additional build directories on the command line:

```posix-terminal
ffx debug connect -- --build-dir /home/me/fuchsia/out/x64
```

Or interactively from within the debugger:

```none
[zxdb] set build-dirs += /home/me/fuchsia/out/x64
```

If debugger is finding the wrong file, you can replace the entire build directory list by omitting
the `+=`:

```none
[zxdb] set build-dirs /home/me/fuchsia/out/x64
```

If your build produces DWARF symbols with absolute file paths the files must be in that location on
the local system. Absolute file paths in the symbols are not affected by the build search path.
Clang users should use the `-fdebug-prefix-map`, which will also help with build hermeticity.
