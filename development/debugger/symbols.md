# Diagnosing symbol problems in zxdb

## Variable values are unavailable

Usually this is related to the optimization level of the program:

_Optimized out_ indicates that the program symbols declare a variable with the given name, but
that it has no value or location. This means the compiler has entirely optimized out the variable
and the debugger can not show it. If you need to see it, use a less-optimized build setting.

_Unavailable_ indicates that the variable is not valid at the current address, but that its value
is known at other addresses. In optimized code, the compiler will often re-use registers, clobbering
previous values, which become unavailable.

You can see the valid ranges for a variable with the "sym-info" command:

```none {:.devsite-disable-click-to-copy}
[zxdb] sym-info my_variable
Variable: my_variable
  Type: int
  DWARF tag: 0x05
  DWARF location (address range + DWARF expression bytes):
    [0x3e0d0a3e05b, 0x3e0d0a3e0b2): 0x70 0x88 0x78
    [0x3e0d0a3e0b2, 0x3e0d0a3eb11): 0x76 0x48 0x10 0xf8 0x07 0x1c 0x06

```

Under "DWARF location" it will give a list of address ranges where the value of the variable is
known (inclusive at the beginning of the range, non-inclusive at the end). Run to one of these
addresses to see the value of the variable (use "di" to see the current address).

You can ignore the "DWARF expression bytes" which are the internal instructions for finding the
variable.

## Can't find symbols

The `sym-stat` command will tell you status for symbols. With no running process, it will give
information on the different symbol locations you have specified. If your symbols aren't found, make
sure this matches your expectations:

```none {:.devsite-disable-click-to-copy}
[zxdb] sym-stat
Symbol index status

  Indexed  Source path
 (folder)  /home/me/.build-id
 (folder)  /home/me/build/out/x64
        0  my_dir/my_file
```

If you see "0" in the "Indexed" column of the "Symbol index status" that means that the debugger
could not find where your symbols are. See below for how to specify the location of these.

Symbol sources using the ".build-id" hierarchy will list "(folder)" for the indexed symbols since
this type of source does not need to be indexed. To check if your hierarchy includes a given build
ID, go to ".build-id" inside it, then to the folder with the first to characters of the build ID to
see if there is a matching file.

When you have a running program, `sym-stat` will additionally print symbol information for each
binary loaded into the process. If you're not getting symbols, find the entry for the binary or
shared library in this list. If it says:

```none {:.devsite-disable-click-to-copy}
    Symbols loaded: No
```

then that means it couldn't find the symbolized binary on the local computer for the given build ID
in any of the locations listed in "Symbol index status". You may need to add a new location with
`-s`.

If instead it says something like this:

```none {:.devsite-disable-click-to-copy}
    Symbols loaded: Yes
    Symbol file: /home/foo/bar/...
    Source files indexed: 1
    Symbols indexed: 0
```

where "Source files indexed" and "Symbols indexed" is 0 or a very low integer, that means that the
debugger found a symbolized file but there are few or no symbols in it. Normally this means the
binary was not built with symbols enabled or the symbols were stripped. Check your build, you should
be passing the path to the unstripped binary and the original compile line should have a `-g` in it
to get symbols.

## Understanding how Zxdb loads symbols

Symbol settings should normally be set automatically by your environment (see [About symbol
settings](#about_symbol_settings) below) so most users should not have to do any configuration. This
section provides some implementation details to help in diagnosing problems.

#### About build IDs

Zxdb locates the symbols for a binary on the target device using the binary's "build ID". If the
build ID does not match, Zxdb will not load the symbols even if the file name is the same. To see
the build ID for a binary on Linux (Mac users will have to install readelf separately), dump the
"notes" for the ELF binary:

```none {:.devsite-disable-click-to-copy}
$ readelf -n my_binary

  ... (some other notes omitted) ...

Displaying notes found in: .note.gnu.build-id
  Owner                Data size 	Description
  GNU                  0x00000014	NT_GNU_BUILD_ID (unique build ID bitstring)
    Build ID: 18cec080fc47cdc07ec554f946f2e73d38541869
```

The `sym-stat` Zxdb command will show the build IDs for each binary and library loaded in the
currently attached process and the corresponding symbol file if found.

#### Symbol servers

Zxdb can load symbols for prebuilt libraries from Google servers. This is how symbols arrive
for SDK users for anything not built locally. See [Downloading
symbols](attaching.md#downloading_symbols) for more.

The most common problem for symbol servers is not being authenticated: run the debugger `auth`
command to authenticate.

For large binaries, symbols can be several gigabytes so the download process can take many minutes.
The `sym-stat` command will display "Downloading..." during this time.

Downloaded symbols are stored in the symbol cache. The `symbol-cache` setting contains the name of
this directory:

```none {:.devsite-disable-click-to-copy}
[zxdb] get symbol-cache
symbol-cache = /home/me/.fuchsia/debug/symbol-cache
```

#### ".build-id" directory symbol databases

Many build environments, including the main "fuchsia.git" repository, add symbolized binaries in a
standard directory structure called ".build-id". This directory contains subdirectories named
according to the first two characters of the binary's build ID, and inside those directories will be
the symbol files named according to the remaining characters of the build ID.

You can set one or more build ID directories (they do not need to be named ".build-id") on the
command line or interactively using the `build-id-dirs` setting (a list of directory paths):

```none {:.devsite-disable-click-to-copy}
[zxdb] set build-id-dirs += "/home/me/project/out/x64/.build-id"
```

These directories will appear in the output of the `sym-stat` command but will be annotated with
"(folder)" rather than the number of binaries found in it, and the binaries will not appear in the
`sym-stat --dump-index` output. This is because Zxdb searches these directories on demand when
searching for symbols rather than enumerating them in advance.

#### Individual files and directories

If you have a single binary file without one of the other symbol database formats, you can tell Zxdb
about the file individually. You can use a command-line flag or set it interactively using the
`symbol-paths` setting (a list of files or directories):

```none {:.devsite-disable-click-to-copy}
[zxdb] set symbol-paths += /home/me/project/a.out
```

This setting also accepts directory names. In this case, Zxdb will non-recursively enumerate all
files in that directory and look for binaries with build IDs:

```none {:.devsite-disable-click-to-copy}
[zxdb] set symbol-paths += /home/me/project/build/
```

To see the status of the locations you provided:

```none {:.devsite-disable-click-to-copy}
[zxdb] sym-stat
Symbol index status

  This command just refreshed the index.
  Use "sym-stat --dump-index" to see the individual mappings.

   Indexed  Source path
         1  /home/me/a.out
         2  /home/me/project/build/
```

You can also see the build IDs and file names of the binaries added in this way with the
`sym-stat --dump-index` command.

#### "ids.txt" symbol index

Some older internal Google projects generate a file called "ids.txt". This provides a mapping from a
binary's build ID to the symbol path on the local system. If your build produces such a file and it
is not automatically loaded, you can provide it to Zxdb via a command-line flag or interactively
using the `ids-txts` setting (a list of file names):

```none {:.devsite-disable-click-to-copy}
[zxdb] set ids-txts += "/home/me/project/build/ids.txt"
```

The symbol files from ids.txt files will also be reflected in the `sym-stat` and
`sym-stat --dump-index` commands described in the previous section.

## About symbol settings

The settings described in the above [Understanding how Zxdb loads
symbols](#understanding_how_zxdb_loads_symbols) section should get automatically applied by your
environment. This section describes how they get set for help debugging symbol load problems.

The `symbol-index-files` setting contains one or more JSON-format files that should be set by the
development environment:

```none {:.devsite-disable-click-to-copy}
[zxdb] get symbol-index-files
symbol-index-files =
  • /home/me/.fuchsia/debug/symbol-index.json
```

This file can contain some global settings and also refer to other symbol-index files. Typically
each build environment you are actively using will have a similar file that is included by reference
from this global file. If you are switching between build environments and find symbols aren't
loading, please make sure your environment is registered by checking the
`ffx debug symbol-index list` command.

## Mismatched source lines

Sometimes the source file listings may not match the code. The most common reason is that the build
is out-of-date and no longer matches the source. The debugger will check that the symbol file
modification time is newer than the source file, but it will only print the warning the first time
the file is displayed. Check for this warning if you suspect a problem.

Some people have multiple checkouts. If it's finding a file in the wrong one, override the
`build-dirs` option as described above in [the setup guide](running.md).

To display the file name of the file it found from `list`, use the `-f` option:

```none {:.devsite-disable-click-to-copy}
[zxdb] list -f
/home/me/fuchsia/out/x64/../../src/foo/bar.cc
 ... <source code> ...
```

You can also set the `show-file-paths` option. This will increase file path information:

  * It will show the full resolved path in source listings as in `list -f`.
  * It will show the full path instead of just the file name in other places such as backtraces.

```none {:.devsite-disable-click-to-copy}
[zxdb] set show-file-paths true
```

You may notice a mismatch when setting a breakpoint on a specific line where the displayed
breakpoint location doesn't match the line number you typed. In most cases, this is because this
symbols did not identify any code on the specified line so the debugger used the next line. It can
happen even in unoptimized builds, and is most common for variable declarations.

```none {:.devsite-disable-click-to-copy}
[zxdb] b file.cc:138
Breakpoint 1 (Software) @ file.cc:138
   138   int my_value = 0;          <- Breakpoint was requested here.
 ◉ 139   DoSomething(&my_value);    <- But ended up here.
   140   if (my_value > 0) {
```

