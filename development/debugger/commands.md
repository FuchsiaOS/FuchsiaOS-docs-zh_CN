# Zxdb console commands and interaction model

##

Zxdb has a built-in help system:

```none {:.devsite-disable-click-to-copy}
[zxdb] help
```

To get help on a specific command or topic (in this case, the `step` command):

```none {:.devsite-disable-click-to-copy}
[zxdb] help step
```

## Interaction model

Most command-line debuggers use an exclusive model for input: you’re either interacting with the
debugged process’ stdin and stdout, or you’re interacting with the debugger. In contrast, zxdb has
an asynchronous model similar to most GUI debuggers. In this model, the user is exclusively
interacting with the debugger while arbitrary processes or threads are running or stopped.

When the debugger itself launches a program it will print the program's stdout and stderr to the
console. When you attach (either with a filter or with the `attach` command) they will go to the
original place. Currently there is no way to interact with a process’ stdin.

Zxdb has a regular noun/verb model for typed commands. The rest of this section gives an overview of
the syntax that applies to all commands. Specific commands will be covered in the “Task guide”
section below.

## Nouns

The possible nouns (and their abbreviations) are:

  * `process` (`pr`)
  * `thread` (`t`)
  * `frame` (`f`)
  * `breakpoint` (`bp`)

### Listing nouns

If you type a noun by itself, it lists the available objects of that type:

  * List attached processes

    ```none {:.devsite-disable-click-to-copy}
    [zxdb] process
      # State       Koid Name
    ▶ 1 Not running 3471 debug_agent_unit_tests.cm
    ```

  * List threads in the current process:

    ```none {:.devsite-disable-click-to-copy}
    [zxdb] thread
      # State   Koid Name
    ▶ 1 Blocked 1348 initial-thread
      2 Blocked 1356 some-other-thread
    ```

  * List stack frames in the current thread (the thread must be stopped—see
    `pause` below):

    ```none {:.devsite-disable-click-to-copy}
    [zxdb] frame
    ▶ 0 fxl::CommandLineFromIterators<const char *const *>() • command_line.h:203
      1 fxl::CommandLineFromArgcArgv() • command_line.h:224
      2 main() • main.cc:174
    ```

### Selecting defaults

If you type a noun and its index, you select that as the default for subsequent commands. It also
tells you the stats about the new default.

  * Select thread 3 to be the default for future commands:

    ```none {:.devsite-disable-click-to-copy}
    [zxdb] thread 3
    Thread 3 Blocked koid=9940 worker-thread
    ```

  * Select breakpoint 2 to be the default:

    ```none {:.devsite-disable-click-to-copy}
    [zxdb] breakpoint 2
    Breakpoint 2 (Software) on Global, Enabled, stop=All, @ MyFunction
    ```

## Verbs

By default, a verb (`run`, `next`, `print`, etc.) applies to the current defaults. So to evaluate an
expression in the context of the current stack frame, just type `print` by itself:

```none {:.devsite-disable-click-to-copy}
[zxdb] print argv[1]
"--foo=bar"
```

You can override the default context by prefixing the verb with a noun and its index. So to evaluate
an expression in the context of a specific stack frame (in this case, frame 2 of the current
thread):

```none {:.devsite-disable-click-to-copy}
[zxdb] frame 2 print argv[1]
"--foo=bar"
```

You can keep adding different types of context. This specifies the process, thread, and frame for
the print command:

```none {:.devsite-disable-click-to-copy}
[zxdb] process 1 thread 1 frame 2 print argv[1]
"--foo=bar"
```

## Attributes and settings

Debugger objects have settings associated with them. Use the "get" verb to list the settings for
a given object:

```none {:.devsite-disable-click-to-copy}
[zxdb] breakpoint 1 get
  enabled  true
  location main
  one-shot false
  scope    global
  stop     all
  type     software
```

The "get" command with a specific attribute will list the attribute and help associated with it:

```none {:.devsite-disable-click-to-copy}
[zxdb] breakpoint 1 get scope

  ... help text here ...

scope = global
```

The "set" command sets a value:

```none {:.devsite-disable-click-to-copy}
[zxdb] breakpoint 1 set scope="process 1 thread 2"
[zxdb] breakpoint 1 set enabled=false
```

Some settings are hierarchical. A thread inherits settings from its process, which in turn inherits
settings from the global scope. The "get" command with no context or parameters will list the
global settings and the ones for the current process and thread. You can set a global setting to
apply to all threads and processes without specific overrides, or override a specific context:

```none {:.devsite-disable-click-to-copy}
[zxdb] set show-stdout = false            # Applies to all processes with no override.
[zxdb] process 2 set show-stdout = true   # Overrides a specific process.
```

Some settings are lists. You can use += to append, or specify a new value with "=". List elements
are space-separated (quote strings with spaces).

```none {:.devsite-disable-click-to-copy}
[zxdb] set symbol-paths = /foo/bar/baz "/home/Dr. Strangelove/cache"
[zxdb] set symbol-paths += /tmp
[zxdb] get symbol-paths
  ... help text ...

symbol-paths =
  • /foo/bar/baz
  • "/home/Dr. Strangelove/cache"
  • /tmp
```
