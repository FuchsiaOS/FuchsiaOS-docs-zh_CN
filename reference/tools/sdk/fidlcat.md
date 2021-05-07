# fidlcat

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

fidlcat [ <options> ] [ command [args] ]
  fidlcat will run the specified command until it exits.  It will intercept and
  record all fidl calls invoked by the process.  The command may be of the form
  "run <component URL>", in which case the given component will be launched.
  fidlcat will return the code 1 if its parameters are invalid.
  fidlcat expects a debug agent to be running on the target device.  It will
  return the code 2 if it cannot connect to the debug agent.
```

__Options:__

```none {: style="white-space: break-spaces;" .devsite-disable-click-to-copy}

  --build-id-dir=<path>
      Adds the given directory to the symbol search path. Multiple
      --build-id-dir switches can be passed to add multiple directories.
      The directory must have the same structure as a .build-id directory,
      that is, each symbol file lives at xx/yyyyyyyy.debug where xx is
      the first two characters of the build ID and yyyyyyyy is the rest.
      However, the name of the directory doesn't need to be .build-id.
  --colors=[never|auto|always]
      For pretty print, use colors:
      - never
      - auto: only if running in a terminal (default value)
      - always
  --columns=<size>
      For pretty print, width of the display. By default, on a terminal, use
      the terminal width.
  --compare=<path>
      Compare output with the one stored in the given file
  --connect
      The host and port of the debug agent running on the target Fuchsia
      instance, of the form [<ipv6_addr>]:port.
  --dump-messages
      Always display the message binary dump even if we can decode the message.
      By default the dump is only displayed if we can't decode the message.
  --exclude-messages
      A regular expression which selects the messages to not display.
      If a message method name satisfy the regexp, the message is not displayed
      (even if it satifies --messages).
      This option can be specified multiple times.
      Message filtering works on the method's fully qualified name.
  --exclude-syscalls
      A regular expression which selects the syscalls to not decode and display.
      Can be passed multiple times.
      To be displayed, a syscall must verify --syscalls and not verify
      --exclude-syscalls.
      To display all the syscalls but the zx_handle syscalls, use:
        --syscalls=".*" --exclude-syscalls="zx_handle_.*"
  --extra-name=<regexp>
      Like --remote-name, it monitors some processes. However, for these
      processes, monitoring starts only when one of of the "--remote-name"
      process is launched. Also, fidlcat stops when the last "--remote-name"
      process stops (even if some "--extra-name" processes are still
      monitored). You must specify at least one filter with --remote-name if
      you use this option (without --remote-name, nothing would be displayed).
  --fidl-ir-path=<path>|@argfile
      Adds the given path as a repository for FIDL IR, in the form of .fidl.json
      files.  Passing a file adds the given file.  Passing a directory adds all
      of the .fidl.json files in that directory and any directory transitively
      reachable from there. An argfile contains a newline-separated list of
      .fidl.json files relative to the directory containing the argfile; passing
      an argfile (starting with the '@' character) adds all files listed in that
      argfile.  This switch can be passed multiple times to add multiple
      locations.
  --format=<output>
      This option must be used at most once.
      The output format can be:
      --format=pretty    The session is pretty printed (with colors).
                         This is the default output is --with is not used.
      --format=json      The session is printed using a json format.
      --format=textproto The session is printed using a text protobuf format.
      --format=none      Nothing is displayed on the standard output (this option only makes sense
                         when used with --to=<path> or with --with).
                         When there is no output, fidlcat is much faster (this is better when you
                         want to monitor real time components).
                         This is the default output is --with is used.
  --from=<source>
      This option must be used at most once.
      Source can be:
      --from=device This is the default input. The input comes from the live monitoring of one or
                    several processes.
                    At least one of '--remote-pid', '--remote-name', 'run' must be specified.
      --from=<path> The input comes from a previously recorded session (protobuf format). Path gives
                    the name of the file to read. If path is '-' then the standard input is used.
  --help
  -h
      Prints all command-line switches.
  --ids-txt=<path>
      Adds the given file to the symbol search path. Multiple --ids-txt
      switches can be passed to add multiple files. The file, typically named
      "ids.txt", serves as a mapping from build ID to symbol file path and
      should contain multiple lines in the format of "<build ID> <file path>".
  --log-file=<pathspec>
      The name of a file to which the log should be written.
  --messages
      A regular expression which selects the messages to display.
      To display a message, the method name must satisfy the regexp.
      This option can be specified multiple times.
      Message filtering works on the method's fully qualified name.
  --quiet=<number or log level>
      The log verbosity.  Legal values are "info", "warning", "error", "fatal",
      or a number, starting from 0. Extra verbosity comes with lower levels.
  --quit-agent-on-exit
      Will send a quit message to a connected debug agent in order for it to
      shutdown. This is so that fidlcat doesn't leak unwanted debug agents on
      "on-the-fly" debugging sessions.
  --remote-name=<regexp>
  -f <regexp>
      Adds a filter to the default job that will cause fidlcat to attach
      to existing or future processes whose names match this regexp.
      For example:
          --remote-name echo_client.*.cmx
          --remote-name echo_client
      Multiple filters can be specified to match more than one process.
  --remote-pid
      The koid of the remote process. Can be passed multiple times.
  --stack=<value>
      The amount of stack frame to display:
      - 0: no stack (default value)
      - 1: call site (1 to 4 levels)
      - 2: full stack frame (adds some overhead)
  --stay-alive
      Don't quit fidlcat when all the monitored processes have ended. This allows to keep monitoring
      upcoming process. At the end you have to use control-c to quit fidlcat. This is useful when
      you monitor a process and restart this process.
  --symbol-cache=<path>
      Directory where we can keep a symbol cache. If a symbol server has been
      specified, downloaded symbols will be stored in this directory. The
      directory structure will be the same as a .build-id directory, and
      symbols will be read from this location as though you had specified
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
  --syscalls
      A regular expression which selects the syscalls to decode and display.
      Can be passed multiple times.
      By default, only zx_channel_.* syscalls are displayed.
      To display all the syscalls, use: --syscalls=".*"
  --thread
      Only display the events for the specified thread.
      This option can be specified multiple times to display several threads.
      By default all the events are displayed.
  --to=<path>
      Save the session using protobuf in the specified file. All events are
      saved including the messages which have been filtered out by --messages
      or --exclude-messages.
  --trigger
      Start displaying messages and syscalls only when a message for which the
      method name satisfies the filter is found.
      This option can be specified multiple times.
      Message filtering works on the method's fully qualified name.
  --verbose=<number or log level>
      The log verbosity.  Legal values are "info", "warning", "error", "fatal",
      or a number, starting from 0. Extra verbosity comes with higher levels
  --version
      Prints the version.
  --with-process-info
      Display the process name, process id and thread id on each line.
These options can be used several times.
  --with=summary
      At the end of the session, a summary of the session is displayed on the standard output.
  --with=summary=<path>
      Like --with=summary but the result is stored into the file specified by <path>.
  --with=top
      At the end of the session, generate a view that groups the output by process, protocol, and
      method. The groups are sorted by number of events, so groups with more associated events are
      listed earlier.
  --with=top=<path>
      Like --with=top but the result is stored into the file specified by <path>.
  --with=group-by-thread
      Like For each thread, display a short version of all the events.
  --with=group-by-thread=<path>
      Like --with=group-by-thread but the result is stored into the file specified by <path>.
```

