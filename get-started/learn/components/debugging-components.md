# Debugging components

<<../../_common/components/_debugging_intro.md>>

<<../../_common/components/_debugging_analyze.md>>

## Exercise: Using the Fuchsia debugger

In this exercise, you'll use the Fuchsia debugger (`zxdb`) to inspect a running
instance of the `echo-args` component and understand the cause of a crash.

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
`echo-args`. This enables the debugger to attach as soon as the process starts:

<pre class="devsite-click-to-copy">
<span class="no-select">[zxdb] </span>attach echo-args
</pre>

Set a breakpoint on the `greeting()` function:

<pre class="devsite-click-to-copy">
<span class="no-select">[zxdb] </span>break greeting
</pre>

With the debugger ready, start a new `echo-args` component instance:


```posix-terminal
ffx component run /core/ffx-laboratory:echo-args fuchsia-pkg://fuchsia.com/echo-args#meta/echo-args.cm
```

### Explore the debug session

Upon reaching the breakpoint in `greeting()`, execution stops and the debugger
waits for a new command. Use the `list` command to show where execution is
currently paused:

* {Rust}

  ```none {:.devsite-disable-click-to-copy}
  [zxdb] list
    18
    19 // Return a proper greeting for the list
    20 fn greeting(names: &Vec<String>) -> String {
    21     // Join the list of names based on length
  ▶ 22     match names.len() {
    23         0 => String::from("Nobody"),
    24         1 => names.join(""),
    25         2 => names.join(" and "),
    26         _ => names.join(", "),
    27     }
    28 }
    29
  ```

* {C++}

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

The `print` command will output the state of any variables in the current stack
frame. Print the current value of `names`:

* {Rust}

  <pre class="devsite-click-to-copy">
  <span class="no-select">[zxdb] </span>print names
  <span class="no-select">vec!["Alice", "Bob", "Spot"]</span>
  </pre>

* {C++}

  <pre class="devsite-click-to-copy">
  <span class="no-select">[zxdb] </span>print names
  <span class="no-select">{"Alice", "Bob", "Spot"}</span>
  </pre>

Step through the `greeting()` function a few times using the `next` command:

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

Next, you'll add some code to `src/main.rs` to cause the component to crash
(or panic). Simulate this behavior by adding an `assert!(false)` macro just
after the arguments are collected:

* {Rust}

  `echo-args/src/main.rs`:

  ```rust
  #[fuchsia::main(logging = true)]
  async fn main() -> Result<(), anyhow::Error> {
      // ...

      {{ '<strong>' }}// Simulate a crash {{ '</strong>' }}
      {{ '<strong>' }}assert!(false, "fake crash");{{ '</strong>' }}

      // Print a greeting to syslog
      info!("Hello, {}!", greeting(&args));

      Ok(())
  }
  ```

* {C++}

  `echo-args/main.cc`:

  ```cpp
  int main(int argc, const char* argv[], char* envp[]) {
    // ...

    {{ '<strong>' }}// Simulate a crash {{ '</strong>' }}
    {{ '<strong>' }}std::strlen(nullptr);{{ '</strong>' }}

    // Print a greeting to syslog
    FX_LOGS(INFO) << "Hello, " << echo::greeting(arguments) << "!" << std::endl;

    return 0;
  }
  ```

Run `fx build` again to rebuild the component:

```posix-terminal
fx build
```

Start a new debug session with `zxdb`:

```posix-terminal
ffx debug connect
```

### Debug the crashing stack frame

Configure the debugger to attach to the `echo-args` component:

<pre class="devsite-click-to-copy">
<span class="no-select">[zxdb] </span>attach echo-args
</pre>

Start a new instance of the component:

```posix-terminal
ffx component run /core/ffx-laboratory:echo-args fuchsia-pkg://fuchsia.com/echo-args#meta/echo-args.cm
```

This time, the debugger detects that an exception was thrown. Use the `frame`
command to inspect the stack trace at the point of the crash:

* {Rust}

  ```none {:.devsite-disable-click-to-copy}
  [zxdb] frame
  ▶ 0 abort() • abort.c:7
    1 panic_abort::__rust_start_panic::abort() • panic_abort/src/lib.rs:43
    2 panic_abort::__rust_start_panic(…) • panic_abort/src/lib.rs:38
    3 std::panicking::rust_panic(…) • library/std/src/panicking.rs:672
    4 std::panicking::rust_panic_with_hook(…) • library/std/src/panicking.rs:642
    5 std::panicking::begin_panic::$({closure#0}<&str>)() • rust/library/std/src/panicking.rs:544
    6 std::sys_common::backtrace::$(__rust_end_short_backtrace<std::panicking::begin_panic::{closure#0}, !>)(…) • rust/library/std/src/sys_common/backtrace.rs:144
    7 std::panicking::begin_panic<…>(…) • rust/library/std/src/panicking.rs:543
    {{ '<strong>' }}8 echo_args::main::component_entry_point::$({generator#0})(…) • main.rs:18{{ '</strong>' }}
    9 core::future::from_generator::$({impl#1})::$(poll<echo_args::main::component_entry_point::{generator#0}>)(…) • rust/library/core/src/future/mod.rs:80
    10 core::future::future::$({impl#1})::$(poll<&mut core::future::from_generator::GenFuture<echo_args::main::component_entry_point::{generator#0}>>)(…) • future/future.rs:119
    11 futures_util::future::future::FutureExt::$(poll_unpin<core::pin::Pin<&mut core::future::from_generator::GenFuture<echo_args::main::component_entry_point::{generator#0}>>>)(…) • future/future/mod.rs:562
    12 fuchsia_async::runtime::fuchsia::executor::local::MainTask::$(poll<core::pin::Pin<&mut core::future::from_generator::GenFuture<echo_args::main::component_entry_point::{generator#0}>>>)(…) • fuchsia/src/lib/fuchsia-async/src/runtime/fuchsia/executor/local.rs:444
    13 fuchsia_async::runtime::fuchsia::executor::local::LocalExecutor::$(run_singlethreaded<core::future::from_generator::GenFuture<echo_args::main::component_entry_point::{generator#0}>>)(…) • fuchsia/src/lib/fuchsia-async/src/runtime/fuchsia/executor/local.rs:73
    14 fuchsia::$(main_singlethreaded<fuchsia::init_logging_for_component_with_executor::{closure#0}, core::future::from_generator::GenFuture<echo_args::main::component_entry_point::{generator#0}>, core::result::Result<(), anyhow::Error>>)(…) • fuchsia/src/lib/fuchsia/src/lib.rs:152
    15 echo_args::main() • main.rs:7
    16 core::ops::function::FnOnce::call_once<…>(…) • /b/s/w/ir/x/w/rust/library/core/src/ops/function.rs:227
    17 std::sys_common::backtrace::__rust_begin_short_backtrace<…>(…) • rust/library/std/src/sys_common/backtrace.rs:125
    18 std::rt::lang_start::$({closure#0}<core::result::Result<(), anyhow::Error>>)() • rust/library/std/src/rt.rs:63
    19 core::ops::function::impls::$({impl#2})::call_once<…>(…) • /b/s/w/ir/x/w/rust/library/core/src/ops/function.rs:259 (inline)
    20 std::panicking::try::do_call<…>(…) • library/std/src/panicking.rs:403 (inline)
    21 std::panicking::try<…>(…) • library/std/src/panicking.rs:367 (inline)
    22 std::panic::catch_unwind<…>(…) • library/std/src/panic.rs:129 (inline)
    23 std::rt::lang_start_internal::$({closure#2})() • library/std/src/rt.rs:45 (inline)
    24 std::panicking::try::$(do_call<std::rt::lang_start_internal::{closure#2}, isize>)(…) • library/std/src/panicking.rs:403 (inline)
    25 std::panicking::$(try<isize, std::rt::lang_start_internal::{closure#2}>)(…) • library/std/src/panicking.rs:367 (inline)
    26 std::panic::$(catch_unwind<std::rt::lang_start_internal::{closure#2}, isize>)(…) • library/std/src/panic.rs:129 (inline)
    27 std::rt::lang_start_internal(…) • library/std/src/rt.rs:45
    28 std::rt::lang_start<…>(…) • rust/library/std/src/rt.rs:62
    29 $elf(main) + 0x1f
    30 «libc startup» (-r expands)
    31 «libc startup» (-r expands)
    32 $elf(_start) + 0x11
  ```

  Notice line 8 in the stack trace indicates the point in `src/main.rs` where the
  crash happened, corresponding to the `assert!()` macro line of code.

* {C++}

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

* {Rust}

  <pre class="devsite-click-to-copy">
  <span class="no-select">[zxdb] </span>frame 8 print args
  <span class="no-select">vec!["Alice", "Bob", "Spot"]</span>
  </pre>

* {C++}

  <pre class="devsite-click-to-copy">
  <span class="no-select">[zxdb] </span>frame 1 print arguments
  <span class="no-select">{"Alice", "Bob", "Spot"}</span>
  </pre>

Exit the debugging session to return to the terminal:

<pre class="devsite-click-to-copy">
<span class="no-select">[zxdb] </span>exit
</pre>

### Destroy the instance

Clean up the `echo-args` instance using the following command:

```posix-terminal
ffx component destroy /core/ffx-laboratory:echo-args
```
