# Controlling thread execution in zxdb

## Threads

To list the current process’ threads (see “Interaction model” above for more):

```none {:.devsite-disable-click-to-copy}
[zxdb] thread
  # State   Koid Name
▶ 1 Blocked 1323 initial-thread
  2 Running 3462 worker-thread
```

Often when you attach to a process the thread will be “blocked”, meaning it is stopped on a system
call. For asynchronous programs this will typically be some kind of wait.

Most thread control and introspection commands only work when a thread is suspended (not blocked or
running). A thread will be suspended when it is stopped at a breakpoint or crashes. You can
explicitly suspend a thread with the `pause` command:

```none {:.devsite-disable-click-to-copy}
[zxdb] thread 2 pause
🛑 syscalls-x86-64.S:67
   65 m_syscall zx_port_create 60 2 1
   66 m_syscall zx_port_queue 61 2 1
 ▶ 67 m_syscall zx_port_wait 62 3 0
   68 m_syscall zx_port_cancel 63 3 1
   69 m_syscall zx_timer_create 64 3 1
```

> When a thread is paused the debugger will show the current source code
> location. Often threads will be in a system call that will resolve to the
> location in the assembly-language macro file that generated the system call
> as shown in the above example.

Running `pause` by itself with no context will pause all threads of all processes currently
attached:

```none {:.devsite-disable-click-to-copy}
[zxdb] pause
```

Unpause a thread with `continue`. As before, `continue` with no context will resume all threads:

```none {:.devsite-disable-click-to-copy}
[zxdb] continue
```

Or continue a specific thread:

```none {:.devsite-disable-click-to-copy}
[zxdb] thread 1 continue
```

## Stack frames

A stack frame is a function call. When a function calls another function, a new nested frame is
created. So listing the frames of a thread tells you the call stack. You can only see the stack
frames when a thread is suspended (see “Working with threads” above).

To list the current thread’s stack frames (the `f` abbreviation will also work).

```none {:.devsite-disable-click-to-copy}
[zxdb] frame
▶ 0 fxl::CommandLineFromIterators<const char *const *>() • command_line.h:203
  1 fxl::CommandLineFromArgcArgv() • command_line.h:224
  2 main() • main.cc:174
```

And to select a given frame as the default:

```none {:.devsite-disable-click-to-copy}
[zxdb] frame 2
```

Frames are numbered with “0” being the top of the stack. Increasing numbers go backwards in time.

You can use the `up` and `down` commands to navigate the frame list:

```none {:.devsite-disable-click-to-copy}
[zxdb] up
  1 fxl::CommandLineFromIterators<const char *const *>() • command_line.h:204

[zxdb] down
  0 fxl::CommandLineFromIteratorsFindFirstPositionalArg<const char *const *>() • command_line.h:185
```

For more context, you can use the `backtrace` command. This is identical to `frame` but gives more
detailed address information as well as function parameters. This command can be abbreviated `bt`:

```none {:.devsite-disable-click-to-copy}
[zxdb] bt
▶ 0 fxl::CommandLineFromIteratorsFindFirstPositionalArg<const char *const *>() • command_line.h:185
      IP = 0x10f982cf2ad0, BP = 0x66b45a01af50, SP = 0x66b45a01af38
      first = (const char* const*) 0x59f4e1268dc0
      last = (const char* const*) 0x59f4e1268dc8
      first_positional_arg = (const char* const**) 0x0
  1 fxl::CommandLineFromIterators<const char *const *>() • command_line.h:204
      IP = 0x10f982cf2ac0, BP = 0x66b45a01af50, SP = 0x66b45a01af40
      first = <'first' is not available at this address. >
      last = <'last' is not available at this address. >
...
```

Each stack frame has a code location. Use the `list` command to look at source code. By itself, it
lists the source code around the current stack frame’s instruction pointer:

```none {:.devsite-disable-click-to-copy}
[zxdb] list
   183 inline CommandLine CommandLineFromIteratorsFindFirstPositionalArg(
   184     InputIterator first, InputIterator last,
 ▶ 185     InputIterator* first_positional_arg) {
   186   if (first_positional_arg)
   187     *first_positional_arg = last;
```

You can list code around the current instruction pointer of other stack frames, too:

```none {:.devsite-disable-click-to-copy}
[zxdb] frame 3 list
```

Or you can list specific things like functions:

```none {:.devsite-disable-click-to-copy}
[zxdb] list MyClass::MyFunc
```

File/line numbers:

```none {:.devsite-disable-click-to-copy}
[zxdb] list foo.cc:43
```

Or whole files:

```none {:.devsite-disable-click-to-copy}
[zxdb] list --all myfile.cc:1
```

## Stepping a thread

When a thread is suspended (see “Threads” above) you can control its execution:

`next` / `n`: Advances to the next line, stepping over function calls.

```none {:.devsite-disable-click-to-copy}
[zxdb] n
```

`step` / `s`: Advances to the next line. If a function call happens before the next line, that
function will be stepped into and execution will stop at the beginning of it. You can also supply an
argument, which is a substring to match of a specific function call. Function names not containing
this substring will be skipped and only matching ones will be stepped into:

```none {:.devsite-disable-click-to-copy}
[zxdb] s
[zxdb] s MyFunction
```

`ss`: List function calls on the current line and step in to the call selected, automatically
completing any of the other calls that happen to occur first.

```none {:.devsite-disable-click-to-copy}
[zxdb] ss
  1 std::string::string
  2 MyClass::MyClass
  3 HelperFunctionCall
  4 MyClass::~MyClass
  5 std::string::~string
  quit
>
```

`finish` / `fi`: Exits the function and stops right after the call.

```none {:.devsite-disable-click-to-copy}
[zxdb] finish
```

`until` / `u`: Given a location (the same as breakpoints, see above), continues the thread until
execution gets there. For example, to run until line 45 of the current file:

```none {:.devsite-disable-click-to-copy}
[zxdb] u 45
```

`jump`: Move the instruction pointer to a new address.

```none {:.devsite-disable-click-to-copy}
[zxdb] jump 22  // Line number
[zxdb] jump 0x87534123  // Address
```

To run until execution gets back to a given stack frame:

```none {:.devsite-disable-click-to-copy}
[zxdb] frame 2 until
```

