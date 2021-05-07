# symbolizer

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

symbolizer [<options>]
  Parses log from stdin and converts symbolizer markups into human readable
  stack traces using local or remote debug symbols.
Options
  --auth
      Starts the authentication process for symbol servers.
  --build-id-dir=<path>
      Adds the given directory to the symbol search path. Multiple
      --build-id-dir switches can be passed to add multiple directories.
      The directory must have the same structure as a .build-id directory,
      that is, each symbol file lives at xx/yyyyyyyy.debug where xx is
      the first two characters of the build ID and yyyyyyyy is the rest.
      However, the name of the directory doesn't need to be .build-id.
  --dumpfile-output=<path>
      Write the dumpfile output to the given file.
  --help
  -h
      Prints this help.
  --ids-txt=<path>
      Adds the given file to the symbol search path. Multiple --ids-txt
      switches can be passed to add multiple files. The file, typically named
      "ids.txt", serves as a mapping from build ID to symbol file path and
      should contain multiple lines in the format of "<build ID> <file path>".
  --omit-module-lines
      Omit the "[[[ELF module ...]]]" lines from the output.
  --symbol-cache=<path>
      Directory where we can keep a symbol cache, which defaults to
      ~/.fuchsia/debug/symbol-cache. If a symbol server has been specified,
      downloaded symbols will be stored in this directory. The directory
      structure will be the same as a .build-id directory, and symbols will
      be read from this location as though you had specified
      "--build-id-dir=<path>".
  --symbol-index=<path>
      Populates --ids-txt and --build-id-dir using the given symbol-index file,
      which defaults to ~/.fuchsia/debug/symbol-index. The file should be
      created and maintained by the "symbol-index" host tool.
  --symbol-path=<path>
  -s <path>
      Adds the given directory or file to the symbol search path. Multiple
      -s switches can be passed to add multiple locations. When a directory
      path is passed, the directory will be enumerated non-recursively to
      index all ELF files. When a file is passed, it will be loaded as an ELF
      file (if possible).
  --symbol-server=<url>
      Adds the given URL to symbol servers. Symbol servers host the debug
      symbols for prebuilt binaries and dynamic libraries.
  --version
  -v
      Prints the version.
```

