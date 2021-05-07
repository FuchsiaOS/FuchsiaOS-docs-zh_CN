# Set up and run zxdb, Fuchsia's native debugger

## Preparation: Boot with networking

Boot the target system with networking support:

  * Hardware devices: use the device instructions.
  * AEMU: `fx emu -N`
  * QEMU: `fx qemu -N`

(If using x64 with an emulator on a Linux host, we also recommend the "-k" flag, which will make it
run faster).

To manually validate network connectivity run `fx shell` or `fx get-device-addr`.

## Finding the debugger

## Binary location (for SDK users)

The binary is `tools/zxdb` in the Fuchsia SDK. SDK users will have to do an extra step to set up
your symbols. See "Running out-of-tree" below for more.

## Compiling the debugger from source (for Fuchsia team members)

A Fuchsia "core" build includes (as of this writing) the necessary targets for the debugger. So this
build configuration is sufficient:

```posix-terminal
fx --dir=out/x64 set core.x64
```

If you're compiling with another product, you may not get it by default. If you don't have the
debugger in your build, add `//bundles:tools` to your "universe", either with:

```posix-terminal
fx <normal_stuff_you_use> --with //bundles:tools
```

Or you can edit your GN args directly by editing `<build_dir>/args.gn` and adding to the bottom:

```none
universe_package_labels += [ "//bundles:tools" ]
```

## Simple method

You can use the fx utility to start the debug agent and connect automatically.

For most build configurations, the debug agent will be in the "universe" (i.e. "available to use")
but not in the base build so won't be on the system before boot. You will need to run:

```posix-terminal
fx serve
```

to make the debug agent's package available for serving to the system. Otherwise you will get the
message "Timed out trying to find the Debug Agent".

Once the server is running, launch the debugger in another terminal window:

```posix-terminal
fx debug
```

To manually validate packages can be loaded, run `ls` from within the Fuchsia shell (for most setups
this requires `fx serve` to be successfully serving packages).

## Manual method

In some cases you may want to run the debug agent and connect manually. To do so, follow these
steps:

### 1. Run the debug agent on the target

On the target system pick a port and run the debug agent:

```posix-terminal
run fuchsia-pkg://fuchsia.com/debug_agent#meta/debug_agent.cmx --port=2345
```

If you get an error "Cannot create child process: ... failed to resolve ..." it means the debug
agent can't be loaded. You may need to run `fx serve` or its equivalent in your environment to make
it available.

You will want to note the target's IP address. Run `ifconfig` _on the target_ to see this, or run
`fx get-device-addr` on the host.

### 2. Run the client and connect

On the host system (where you do the build), run the client. Use the IP address of the target and
the port you picked above in the `connect` command. If running in-tree, `fx get-device-addr` will
tell you this address.

For QEMU, we recommend using IPv6 and link local addresses. These addresses have to be annotated
with the interface they apply to, so make sure the address you use includes the appropriate
interface (should be the name of the bridge device).

The address should look like `fe80::5054:ff:fe63:5e7a%br0`

```none {:.devsite-disable-click-to-copy}
$ fx zxdb

or

out/<out_dir>/host_x64/zxdb

[zxdb] connect [fe80::5054:ff:fe63:5e7a%br0]:2345
```

(Substitute your build directory as-needed).

If you're connecting or running many times, there are command-line switches:

```posix-terminal
zxdb -c [fe80::5054:ff:fe63:5e7a%br0]:2345
```

  * The `status` command will give you a summary of the current state of the
    debugger.

  * See `help connect` for more examples, including IPv6 syntax.

## Running out-of-tree

You can run with kernels or user programs compiled elsewhere with some extra steps. We hope this
will become easier over time.

Be aware that we aren't yet treating the protocol as frozen. Ideally the debugger will be from the
same build as the operating system itself (more precisely, it needs to match the debug\_agent). But
the protocol does not change very often so there is some flexibility.

When you run out-of-tree, you will need to tell zxdb where your symbols and source code are on the
local development box (Linux or Mac). Zxdb can not use symbols in the binary that you pushed to the
Fuchsia target device.

See [Diagnosing symbol problems](#diagnosing-symbol-problems).

### Set the symbol location {#set-symbol-location}

There are three command-line flags to control the symbol lookup locations for zxdb:
`--build-id-dir`, `--ids-txt`, and a general `--symbol-path`. They all have the corresponding
settings that can be manipulated using `set` or `get`.

For example, to add a ".build-id" directory, either use `--build-id-dir` flag:

```posix-terminal
zxdb --build-id-dir some/other_location/.build-id
```

Or add it to the `build-id-dirs` list option in the interactive UI:

```none {:.devsite-disable-click-to-copy}
[zxdb] set build-id-dirs += some/other_location/.build-id
```

For in-tree development, `fx debug` automatically sets up all necessary
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

### Set the source code location {#set-source-code-location}

The Fuchsia build generates symbols relative to the build directory so relative paths look like
`../../src/my_component/file.cc`).

If your files are not being found with the default build directories, you will need to provide a
build directory to locate the files. This build directory does not need have been used to build, it
just needs to produce correct absolute paths when concatenated with the relative paths from the
symbol file.

You can add additional build directories on the command line:

```posix-terminal
zxdb -b /home/me/fuchsia/out/x64
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

