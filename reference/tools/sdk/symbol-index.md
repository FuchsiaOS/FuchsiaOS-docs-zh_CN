# symbol-index

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

symbol-index [ <options> ] <verb> [ <arguments> ... ]
  Manipulates a symbol-index file.
Available verbs:
  list
      Lists all paths in symbol-index.
  add <symbol path> [ <build directory> ]
      Adds a new symbol path to symbol-index. A symbol path could be either a
      a text file in "ids.txt" format, or a directory in ".build-id" structure.
      An optional build directory could be supplemented, which is used by zxdb
      to locate the source code. If the symbol path is already in symbol-index,
      no changes will be made regardless of the optional build directory.
  add-all [ <input file> ]
      Reads the input and adds all symbol paths with optional build directories.
      The input file can contain multiple lines, each describing a symbol path.
      An optional build directory could be supplemented and separated from the
      symbol path with whitespaces. Relative paths will be resolved based on
      the input file. Empty lines and lines starting with "#" will be ignored.
      If the input file is not specified, the input will be read from the stdin.
  remove <symbol path>
      Removes a symbol path from symbol-index.
  purge
      Removes all non-existent paths from symbol-index.
Options
  --config=<path>
  -c <path>
      Path to the symbol-index config file, default to
      ~/.fuchsia/debug/symbol-index.
  --help
  -h
      Prints this help.
  --version
  -v
      Prints the version.
```

