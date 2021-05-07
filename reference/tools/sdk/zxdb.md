# zxdb

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

zxdb [ <options> ]
  For information on using the debugger, type "help" at the interactive prompt.
Options
  --analytics-show
      Show the opt-in/out status for collection of analytics and what we collect when opt-in.
  --analytics=enable|disable
      Enable or disable collection of analytics:
      --analytics=enable           Enable collection of analytics and save the
                                   status in a configuration file.
      --analytics=disable          Disable collection of analytics and save the
                                   status in a configuration file.
  --build-dir=<path>
  -b <path>
      Adds the given directory to the list of build directories. These
      directories are where source file names from the symbols are relative to.
      There can be multiple ones which will be searched in order.
      It populates the "build-dirs" setting (see "get build-dirs").
  --build-id-dir=<path>
      Adds the given directory to the symbol search path. Multiple
      --build-id-dir switches can be passed to add multiple directories.
      The directory must have the same structure as a .build-id directory,
      that is, each symbol file lives at xx/yyyyyyyy.debug where xx is
      the first two characters of the build ID and yyyyyyyy is the rest.
      However, the name of the directory doesn't need to be .build-id.
  --connect=<host>:<port>
  -c <host>:<port>
      Attempts to connect to a debug_agent running on the given host/port.
  --core=<filename>
      Attempts to open a core file for analysis.
  --debug-mode
  -d
      Output debug information about zxdb.
      Should only be useful for people developing zxdb.
  --filter=<regexp>
  -f <regexp>
      Adds a job filter to the default job. This will automatically attach
      to processes matching this regexp that are launched in the job. Multiple
      filters can be specified to match more than one process.
  --help
  -h
      Prints all command-line switches.
  --ids-txt=<path>
      Adds the given file to the symbol search path. Multiple --ids-txt
      switches can be passed to add multiple files. The file, typically named
      "ids.txt", serves as a mapping from build ID to symbol file path and
      should contain multiple lines in the format of "<build ID> <file path>".
  --quit-agent-on-exit
      Will send a quit message to a connected debug agent in order for it to
      shutdown. This is so that zxdb doesn't leak unwanted debug agents on
      "on-the-fly" debugging sessions.
  --run=<program>
  -r <program>
      Attempts to run a binary in the target system. The debugger must be
      already connected to the debug_agent (use with -c).
  --script-file=<file>
  -S <file>
      Reads a script file from a file. The file must contains valid zxdb
      commands as they would be input from the command line. They will be
      executed sequentially.
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
  --unix-connect=<filepath>
  -u <filepath>
      Attempts to connect to a debug_agent through a unix socket.
  --version
  -v
      Prints the version.
 --debug-adapter-port=<port>
      Uses this port number to serve debug adapter protocol. By default 15678 is used.
 --enable-debug-adapter
      Starts the debug adapter that serves debug adapter protocol.
      This is useful for connecting the debugger with an IDE.
```

