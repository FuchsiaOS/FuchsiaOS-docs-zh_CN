# Debugging a process, component, or crash dump

## Attaching to a process by name filters

Running a process on Fuchsia is more complicated than in other systems because there are different
loader environments (see "A note about launcher environments" below).

The only way to reliably debug all types of processes is to create a filter on the process name via
"attach" and start it the normal way you would start that process. The process name is usually the
name of the build target that generates it. To check what this is, use "ps" (either in the debugger
or from a system shell) with it running.

> Note: only the first 32 bytes of the name are included in the Zircon process description.
> Sometimes the number of path components can cause the name to be truncated. If the filter isn't
> working, check the actual name in "ps". We hope to have a better way to match this in the future.

This example sets a pending breakpoint on `main` to stop at the beginning of execution, and waits
for a process called "my\_app" to start:

```
[zxdb] attach my_app
Waiting for process matching "my_app"

[zxdb] break main
Breakpoint 1 (Software) on Global, Enabled, stop=All, @ main
Pending: No matches for location, it will be pending library loads.
```

Then run the process the way you would in normal use (directly on the command line, via `fx test`,
via the shell's `run fuchsia-pkg://...`, or another way. The debugger should then immediately break
on `main` (it may take some time to load symbols so you may see a delay before showing the source
code):

```none {:.devsite-disable-click-to-copy}
Attached Process 1 [Running] koid=51590 my_app.cmx
🛑 on bp 1 main(…) • main.cc:222
   220 }
   221
 ▶ 222 int main(int argc, const char* argv[]) {
   223   foo::CommandLineOptions options;
   224   cmdline::Status status = ParseCommandLine(argc, argv, &options);
```

You can then do basic commands that are similar to GDB:

```none {:.devsite-disable-click-to-copy}
next
step
print argv[1]
continue
quit
```

#### A note about launcher environments

The following loader environments all have different capabilities (in order
from least capable to most capable):

  * The debugger's `run <file name>` command (base system process stuff).
  * The system console or `fx shell` (adds some libraries).
  * The base component environment via the shell's `run` and the debugger's
    `run -c <package url>` (adds component capabilities).
  * The test environment via `fx test`.
  * The user environment when launched from a "story" (adds high-level
    services like scenic).

This panoply of environments is why the debugger can't have a simple "run"
command that always works.

## Launching simple command-line processes

Minimal console apps including some unit tests can be launched directly from within the debugger
which avoids the above "attach" dance:

```none {:.devsite-disable-click-to-copy}
[zxdb] break main
Breakpoint 1 (Software) on Global, Enabled, stop=All, @ $main
Pending: No matches for location, it will be pending library loads.

[zxdb] run /bin/cowsay
```

If you get a shared library load error or errors about files or services not being found, it means
the app can't be run from within the debugger's launcher environment. This is true even for things
that may seem relatively simple.

## Directly launching components

Components that can be executed with the console command `run fuchsia-pkg://...` can be loaded in
the debugger with the following command, substituting your component's URL:

```none {:.devsite-disable-click-to-copy}
[zxdb] run -c fuchsia-pkg://fuchsia.com/your\_app#meta/your\_app.cmx
```

Not all components can be launched this way since most higher-level services won't be accessible: if
you can't do `run ...` from the system console, it won't work from the debugger either. Note also
that `fx test` is a different environment. According to your test's dependencies, it may or may not
work from the debugger's `run` command.

## Attaching to an existing process

You can attach to most running processes given the process’ koid (the [kernel object
ID](/docs/concepts/kernel/concepts.md) that, when applied to a process, is equivalent to a process
ID on other systems). You can get the koid by running `ps` on the target Fuchsia system or use
zxdb's built-in `ps` command:

```none {:.devsite-disable-click-to-copy}
[zxdb] ps
j: 1030 root
  j: 1079 zircon-drivers
    p: 1926 driver_host:sys
...
```

In this listing, "j:" indicates a [job](/docs/concepts/kernel/concepts.md) (a container for
processes) and "p:" indicates a process. The number following the type prefix is the object's koid.

Then to attach:

```none {:.devsite-disable-click-to-copy}
[zxdb] attach 3517
Process 1 Running koid=1249 pwrbtn-monitor
```

When you’re done, you can choose to `detach` (keep running) or `kill` (terminate) the process.

## Attaching to processes in specific jobs

By default the debugger will attampt to attach to the root job so process launch filters will apply
globally. Normally this will appear as "job 1" in the debugger:

```none {:.devsite-disable-click-to-copy}
[zxdb] job
  # State     Koid Name
  1 Attached  1027 root
```

You can also apply filters for processes launched in a specific job. First attach to the job using
the `attach-job` command, specifying the job's koid:

```none {:.devsite-disable-click-to-copy}
[zxdb] attach-job 30053
Job 2 state=Attached koid=30053 name=""
```

The debugger will now be attached to two jobs, with the new job being the current one:

```none {:.devsite-disable-click-to-copy}
[zxdb] job
  # State     Koid Name
  1 Attached  1027 root
▶ 2 Attached 30053
```

Now you can make a filter (see above) that applies to the process names only to this job by
prefixing the attach command with the job object number (not koid) created above:

```none {:.devsite-disable-click-to-copy}
[zxdb] job 2 attach my_app
Waiting for process matching "my_app".
Type "filter" to see the current filters.
```

You can also attach to all current and future processes in a job using the `*` wildcard as the
filter name:

```none {:.devsite-disable-click-to-copy}
[zxdb] attach-job 30053
Job 2 state=Attached koid=30053 name=""

[zxdb] job 2 attach *
Attached Process 1 state=Running koid=28071 name=sysmem_connector.cmx
```

> **Warning:** Be careful only to use the wildcard `attach *` command with an explicit,
> narrowly-scoped job. Making a global filter or applying it to a job with too many children can
> attach to too many processes that may include drivers necessary for the system to function.

## Debugging drivers

It's not currently possible to set up the debugger early enough in system startup to debug most
driver initialization. And since zxdb itself uses the network and filesystem, no drivers associated
with network communication can be debuged, and neither can many drivers associated with storage or
other critical system functions.

Driver debugging support is tracked in issue
[5456](https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=5456).

You can debug running drivers by attaching like any other process (see “Attaching to an existing
process”). You can delay initialization to allow time to attach by adding a busyloop at the
beginning of your code:

```cpp
volatile bool stop = false;
while (!stop) {}
```

To break out of the loop after attaching, either set the variable to true:

```none {:.devsite-disable-click-to-copy}
[zxdb] print stop = true
true
[zxdb] continue
```

Or jump to the line after the loop:

```none {:.devsite-disable-click-to-copy}
[zxdb] jump <line #>
[zxdb] continue
```

## Debugging crash dumps

Zxdb supports loading a minidump generated by a crash report. Use the "opendump" verb and supply the
local file name of the dump. The debugger must not be attached to another dump or a running system
(use "disconnect" first if so).

```none {:.devsite-disable-click-to-copy}
[zxdb] opendump upload_file_minidump-e71256ba30163a0.dmp
Opening dump file
Dump loaded successfully.
```

Now the thread, stack, and memory commands can be used to inspect the state of the program. Use
`disconnect` to close the dump.

For in-tree users, the `fx debug` command can take the path to a core file as an argument.

```posix-terminal {:.devsite-disable-click-to-copy}
fx debug -c upload_file_minidump-e71256ba30163a0.dmp
```

#### Downloading symbols

To tell zxdb to look for debug symbols for your core dump in a GCS URL and download those symbols
automatically, run the following command, substituting the location of your symbols:

```posix-terminal {:.devsite-disable-click-to-copy}
zxdb --symbol-server gs://my-bucket-name/namespace
```

In-tree users automatically have the option set, with the server pointed to a bucket containing
symbols for all release builds.

The first time you use the symbol server, you will have to authenticate using the `auth` command.
The authentication flow will require you to complete part of the authentication in your browser.

```none {:.devsite-disable-click-to-copy}
[zxdb] auth
To authenticate, please supply an authentication token. You can retrieve a token from:

https://accounts.google.com/o/oauth2/v2/< very long URL omitted >

Once you've retrieved a token, run 'auth <token>'

[zxdb] auth 4/hAF-pASODIFUASDIFUASODIUFSADF329827349872V6
Successfully authenticated with gs://fuchsia-artifacts-release/debug
```

### Debugging multiple processes

You can debug many arbitrary processes at the same time. Attaching or running when a process is
already running in the debugger will just create a new one in parallel.

Recall from the “Interaction model” section you can list the current processes with:

```none {:.devsite-disable-click-to-copy}
[zxdb] process
  # State       Koid Name
▶ 1 Running     1249 pwrbtn-monitor
  2 Not running 7235 pwrbtn-monitor
```

Select one of those as the default by providing its index (not koid):

```none {:.devsite-disable-click-to-copy}
[zxdb] process 2
```

Or apply commands to a specific process (even if it’s not the default) with:

```none {:.devsite-disable-click-to-copy}
[zxdb] process 2 pause
```
