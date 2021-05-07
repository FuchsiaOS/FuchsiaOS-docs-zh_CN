# Diagnosing symbol problems in zxdb

## Variable values are unavailable

Usually this is related to the optimization level of the program:

_Optimized out_ Indicates that the program symbols declare a variable with the given name, but
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
could not find where your symbols are. Try the `-s` flag (see "Running out-of-tree" above) to
specify where your symbols are.

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

