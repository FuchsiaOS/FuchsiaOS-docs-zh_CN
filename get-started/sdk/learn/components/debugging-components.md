# Debugging components

<<../../../_common/components/_debugging_intro.md>>

<<../../../_common/components/_debugging_analyze.md>>

## Exercise: Using the Fuchsia debugger

In this exercise, you'll use the Fuchsia debugger (`zxdb`) to inspect a running
instance of the `echo` component and understand the cause of a crash.

<<../_common/_start_femu_with_packages.md>>

### Start a debug session

Once the emulator has started up, start a `zxdb` debugging session with the
`ffx debug connect` command:

```posix-terminal
ffx debug connect
```

```none {:.devsite-disable-click-to-copy}
Connecting (use "disconnect" to cancel)...
Connected successfully.
👉 To get started, try "status" or "help".
[zxdb]
```

After successfully connecting, the `zxdb` prompt is ready to accept commands.

### Attach to the component

Before launching the component, configure `zxdb` to attach to an instance of
`echo`. This enables the debugger to attach as soon as the process starts:

<pre class="devsite-click-to-copy">
<span class="no-select">[zxdb] </span>attach echo
</pre>

Set a breakpoint on the `greeting()` function:

<pre class="devsite-click-to-copy">
<span class="no-select">[zxdb] </span>break greeting
</pre>

With the debugger ready, start a new `echo` component instance:


```posix-terminal
ffx component run /core/ffx-laboratory:echo fuchsia-pkg://fuchsiasamples.com/echo-example#meta/echo.cm
```

### Explore the debug session

Upon reaching the breakpoint in `greeting()`, execution stops and the debugger
waits for a new command. Use the `list` command to show where execution is
currently paused:

```none {:.devsite-disable-click-to-copy}
[zxdb] list
  17
  18 // Return a proper greeting for the list
▶ 19 std::string greeting(std::vector<std::string>& names) {
  20   // Join the list of names based on length
  21   auto number_of_names = names.size();
  22   switch (number_of_names) {
  23     case 0:
  24       return "Nobody!";
  25     case 1:
  26       return join(names, "");
  27     case 2:
  28       return join(names, " and ");
  29     default:
```

Step into the `greeting()` function using the `next` command:

<pre class="devsite-click-to-copy">
<span class="no-select">[zxdb] </span>next
</pre>

The `print` command will output the state of any variables in the current stack
frame. Print the current value of `names`:

<pre class="devsite-click-to-copy">
<span class="no-select">[zxdb] </span>print names
<span class="no-select">{"Alice", "Bob", "Spot"}</span>
</pre>

Step through the `greeting()` function a more few times using `next`:

<pre class="devsite-click-to-copy">
<span class="no-select">[zxdb] </span>next
</pre>

To let the program continue to completion, use the `continue` command:

<pre class="devsite-click-to-copy">
<span class="no-select">[zxdb] </span>continue
</pre>

Exit the debugging session to return to the terminal:

<pre class="devsite-click-to-copy">
<span class="no-select">[zxdb] </span>exit
</pre>

### Introduce some crashing code

Next, you'll add some code to `main()` to cause the component to crash
(or panic). Simulate this behavior by adding an `strlen(nullptr)` reference just
after the arguments are collected:

`echo/main.cc`:

```cpp
int main(int argc, const char* argv[], char* envp[]) {
  // ...

  {{ '<strong>' }}// Simulate a crash {{ '</strong>' }}
  {{ '<strong>' }}std::strlen(nullptr);{{ '</strong>' }}

  // Print a greeting to syslog
  std::cout << "Hello, " << echo::greeting(arguments) << "!" << std::endl;

  return 0;
}
```

Build and publish the updated package to the `fuchsiasamples.com` repository:

```posix-terminal
bazel run //fuchsia-codelab/echo:pkg.publish -- \
    --repo_name fuchsiasamples.com
```

Start a new debug session with `zxdb`:

```posix-terminal
ffx debug connect
```

### Debug the crashing stack frame

Configure the debugger to attach to the `echo` component:

<pre class="devsite-click-to-copy">
<span class="no-select">[zxdb] </span>attach echo
</pre>

Start a new instance of the component:

```posix-terminal
ffx component run /core/ffx-laboratory:echo fuchsia-pkg://fuchsiasamples.com/echo-example#meta/echo.cm --recreate
```

This time, the debugger detects that an exception was thrown and halts execution:

```{:.devsite-disable-click-to-copy}
Attached Process 1 state=Running koid=1164808 name=echo.cm
════════════════════════════════════════════════
 Page fault reading address 0x0 (second chance)
════════════════════════════════════════════════
 Process 1 (koid=1164808) thread 1 (koid=1164810)
 Faulting instruction: 0x43e0fd349210

🛑 strlen(const char*) • strlen.c:21
[zxdb]
```

Use the `frame` command to inspect the stack trace at the point of the crash:

```none {:.devsite-disable-click-to-copy}
[zxdb] frame
▶ 0 strlen(…) • strlen.c:21
  {{ '<strong>' }}1 main(…) • main.cc:27{{ '</strong>' }}
  2 «libc startup» (-r expands)
  3 «libc startup» (-r expands)
  4 $elf(_start) + 0x11
```

Notice line 1 in the stack trace indicates the point in `main.cc` where the
crash happened, corresponding to the `nullptr` reference.

The current stack frame (frame 0) is deep within the system library, but you
can inspect any stack frame by prefixing the command with the frame number from
the stack trace.

Print the value of the arguments at the point of the crash by passing the
frame number as follows:

<pre class="devsite-click-to-copy">
<span class="no-select">[zxdb] </span>frame 1 print arguments
<span class="no-select">{"Alice", "Bob", "Spot"}</span>
</pre>

Exit the debugging session to return to the terminal:

<pre class="devsite-click-to-copy">
<span class="no-select">[zxdb] </span>exit
</pre>

### Destroy the instance

Clean up the `echo` instance using the following command:

```posix-terminal
ffx component destroy /core/ffx-laboratory:echo
```
