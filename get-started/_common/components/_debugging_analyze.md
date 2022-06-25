## Analyzing crashes

Fuchsia starts a program at boot called `crashanalyzer` that reports program
crashes and prints a backtrace of the crashing thread to the system log. While
you can explore these directly by reviewing the logs at runtime, the backtrace
content is encoded using the stack memory address references rather than
pointing to the corresponding lines in the program source files.


```none {:.devsite-disable-click-to-copy}
[klog][I] devmgr: crash_analyzer_listener: analyzing exception type 0x108
[klog][I] <== fatal exception: process crasher[42410] thread initial-thread[42424]
[klog][I] <== fatal page fault, PC at 0x1e1888dbbbd7
[klog][I]  CS:                   0 RIP:     0x1e1888dbbbd7 EFL:            0x10246 CR2:                  0
[klog][I]  RAX:                  0 RBX:                0x1 RCX:     0x721ad98697c6 RDX:     0x77accb36f264
[klog][I]  RSI:                  0 RDI:                  0 RBP:     0x2781c4816f90 RSP:     0x2781c4816f80
[klog][I]   R8:                  0  R9:                  0 R10:                  0 R11:              0x246
[klog][I]  R12:     0x773bf11dcda0 R13:     0x773bf11dcdd0 R14:               0x16 R15:         0x78050d69
[klog][I]  errc:               0x6
[klog][I] bottom of user stack:
[klog][I] 0x00002781c4816f80: f11dcda0 0000773b 9ccd2b38 000039b2 |....;w..8+...9..|
[klog][I] 0x00002781c4816f90: c4816fd0 00002781 88dbbba7 00001e18 |.o...'..........|
[klog][I] 0x00002781c4816fa0: 00000008 00000000 9ccd2b38 000039b2 |........8+...9..|
[klog][I] 0x00002781c4816fb0: f11dcf70 0000773b f11dcf70 0000773b |p...;w..p...;w..|
[klog][I] 0x00002781c4816fc0: cb36f570 000077ac f11dcdd0 0000773b |p.6..w......;w..|
[klog][I] 0x00002781c4816fd0: c4816ff0 00002781 cb2b0d0f 000077ac |.o...'....+..w..|
[klog][I] 0x00002781c4816fe0: 00000054 00000000 f11dcf70 0000773b |T.......p...;w..|
[klog][I] 0x00002781c4816ff0: f11dcfe0 0000773b 00000000 00000000 |....;w..........|
[klog][I] arch: x86_64
[klog][I] dso: id=a94c78564173530d51670b6586b1aa471e004f06 base=0x7d3506a49000 name=libfdio.so
[klog][I] dso: id=a61961ba9776a67a00fb322af9ebbdcfd1ce3f62 base=0x77accb297000 name=libc.so
[klog][I] dso: id=760f1e6e47d3dd8b6a19150aa47241279ec75a9c base=0x721ad9863000 name=<vDSO>
[klog][I] dso: id=b18462140c6784a53736105bbf3021852eeda68c base=0x1e1888dbb000 name=app:crasher
[klog][I] bt#01: pc 0x1e1888dbbbd7 sp 0x2781c4816f80 (app:crasher,0xbd7)
[klog][I] bt#02: pc 0x1e1888dbbba7 sp 0x2781c4816fa0 (app:crasher,0xba7)
[klog][I] bt#03: pc 0x77accb2b0d0f sp 0x2781c4816fe0 (libc.so,0x19d0f)
[klog][I] bt#04: pc 0 sp 0x2781c4817000
[klog][I] bt#05: end
```

This is because the **debug symbols** are stripped out of the core binaries
by default at build time. To properly analyze the crash log, you need to
reapply those symbols to the backtrace to see the call stack in terms of source
code line numbers. When you call the `ffx log` command, the developer tools
process the raw log through an additional binary called `symbolizer` that
reapplies the symbols from your local build configuration to any backtraces in
the log.

```posix-terminal
ffx log
```

The output you see includes the symbols reapplied to the backtrace:

```none {:.devsite-disable-click-to-copy}
[klog][I] devmgr: crash_analyzer_listener: analyzing exception type 0x108
... same output as "raw" backtrace ...
start of symbolized stack:
[klog][I] #01: blind_write at ../../src/developer/forensics/crasher/cpp/crasher.c:21
[klog][I] #02: main at ../../src/developer/forensics/crasher/cpp/crasher.c:137
[klog][I] #03: start_main at ../../zircon/third_party/ulib/musl/src/env/__libc_start_main.c:49
[klog][I] #04: unknown, can't find pc, sp or app/library in line
end of symbolized stack
```

<aside class="key-point">
If you have a manually captured backtrace, you can run the same process by
passing the content through the <code>ffx debug symbolize</code> command.
</aside>

With a properly symbolized backtrace, you can directly discover the site of a
crash in your source code.

### Step-through debugging

Just knowing where a program crashed may not be enough information to fully
diagnose the issue. Sometimes it's necessary to walk through the code
step-by-step and even inspect the state of variables in memory. To support this,
Fuchsia has a debugger for core code called `zxdb`.

The `zxdb` tool is a client that connects to a running `debug_agent` component
on the target device. You can use the `zxdb` commands to configure the
`debug_agent` to attach to specific processes and set breakpoints. Once a debug
session is attached to a running process, `zxdb` allows you to step through the
code and inspect the stack frames.

![Diagram showing how the Fuchsia debugger (zxdb) interacts with the
debug_agent service running on a Fuchsia device to perform interactive
debugging of a process.](get-started/images/components/zxdb.png){: width="591"}

Setting up the debug session requires the following high-level steps:

1.  Run the `debug_agent` component on the target device.
1.  Run the `zxdb` client and connect to the target device.
1.  Set the location for `zxdb` to find debug symbols.

The simplest method to start a debug session is to use the `ffx debug connect`
command, which does all of these in the context of your local Fuchsia build.
However, these steps can also be performed manually if you need to configure
them separately.

Once the debug session is active, you are taken to a `[zxdb]` prompt to issue
debugger commands. You can use `zxdb` to configure the `debug_agent` to attach
to a process using a name filter and set pending breakpoints even if no
matching process is currently running.

The following example sets a pending breakpoint on main to stop at the
beginning of execution, and waits for a process called "hello-world" to start:

```none {:.devsite-disable-click-to-copy}
[zxdb] attach hello-world
Waiting for process matching "hello-world"

[zxdb] break main
Breakpoint 1 (Software) on Global, Enabled, stop=All, @ main
Pending: No matches for location, it will be pending library loads.
```

<aside class="caution">
  <b>Launching components with zxdb</b>
  <p>The Fuchsia debugger supports two main methods of debugging a process:
  <code>run</code> and <code>attach</code>. Avoid using <code>run</code> to
  start and debug components.</p>

  <p>When the debugger launches a component, the component will have the same
  capabilities as the <code>debug_agent</code>. The only way to get the correct
  environment is to launch your component in the way it expects and attach the
  debugger to it.</p>
</aside>

Once the debugger is attached to a process, you can use `zxdb` commands to
control and inspect the state of the process. Here is a short collection of
common commands:

Note: For complete details and reference on `zxdb` commands, see
[The Fuchsia debugger](development/debugger).

<table>
  <tr>
   <td><code>step</code>
   </td>
   <td>Step over the next line of code in the thread
   </td>
  </tr>
  <tr>
   <td><code>next</code>
   </td>
   <td>Step into the next line of code in the thread
   </td>
  </tr>
  <tr>
   <td><code>continue</code>
   </td>
   <td>Continue execution until the next breakpoint, exception, or exit
   </td>
  </tr>
  <tr>
   <td><code>frame</code>
   </td>
   <td>List or select from the current stack frames
   </td>
  </tr>
  <tr>
   <td><code>print</code>
   </td>
   <td>Evaluate an expression and print the result
   </td>
  </tr>
</table>
