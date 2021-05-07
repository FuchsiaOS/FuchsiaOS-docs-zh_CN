# Just In Time Debugging

## Overview

Just In Time Debugging (JITD) is a way for Fuchsia to suspend processes that crash so that
interested parties can debug/process them later. This permits interesting flows such as attaching
zxdb to a program that crashed overnight, when the debugger was not attached/running.

This is done by storing process in exceptions in a special place called the "Process Limbo". This
place will keep those processes suspended until some other agent comes and releases them.

See [Implementation](#implementation) for more details about how it works.

## How to enable it

One of the great benefits of the Process Limbo is to be able to catch crashing processes in the
wild, without the need to have already running debuggers. This is specially useful for situations
where the debugger *cannot* be running, such as driver startup. For such cases, having an active
Process Limbo can provide an invaluable source of debugging information.

There are two ways of enabling the Process Limbo:

### Manual activation

The Process Limbo comes with a CLI tool that permits the user to query the current state of the
limbo:

```
$ run run fuchsia-pkg://fuchsia.com/limbo-client#meta/limbo_client.cmx
Usage: limbo [--help] <option>

  The process limbo is a service that permits the system to suspend any processes that throws an
  exception (crash) for later processing/debugging. This CLI tool permits to query and modify the
  state of the limbo.

  Options:
    --help: Prints this message.
    enable: Enable the process limbo. It will now begin to capture crashing processes.
    disable: Disable the process limbo. Will free any pending processes waiting in it.
    list: Lists the processes currently waiting on limbo. The limbo must be active.
    release: Release a process from limbo. The limbo must be active. Usage: limbo release <pid>.
```

### Enable on startup

Manual activation works only if you have a way to send commands to the system. But some development
environments run software earlier that the user can interact with (or run a debugger). Drivers are a
good example of this. For those cases, having the Process Limbo active from the start lets you catch
driver crashes as they occur while the driver is spinning up, which is normally the hardest part to
debug.

In order to do this, there is a configuration that has to be set into the build:

```
fx set <YOUR CONFIG> --with-base //src/developer/forensics:exceptions_enable_jitd_on_startup
```

Or add this label to the `base_package_labels` in your build args. You can still use the Process
Limbo CLI tool to disable and manipulate the limbo afterwards. Then you will need to push an update
to your device for this to take an effect.

NOTE: Driver initialization is finicky and freezing crashing process can leave the system in an
undefined state and "hang" it, so your mileage may vary when using this feature, especially for very
early drivers.

## How to use it

### zxdb

The main user of JITD is zxdb, which is able to attach to a process waiting in the limbo. When
starting zxdb, it will display the processes that are waiting in it:

```
> fx debug
Checking for debug agent on [fe80::2e0:4cff:fe68:8d%3]:2345.
Debug agent not found. Starting one.
Connecting (use "disconnect" to cancel)...
Connected successfully.

👉 To get started, try "status" or "help".

Processes waiting on exception:
272401: crasher
Type "attach <pid>" to reconnect.

[zxdb] attach 272401
Process 1 [Running] koid=272401 crashed
Attached Process 1 [Running] koid=272401 crasher
[Warning] Received thread exception for an unknown thread.

[zxdb] thread
  # State                 Koid Name
▶ 1 Blocked (Exception) 272403 initial-thread

[zxdb] frame
▶ 0 blind_write(volatile unsigned int*) • crasher.c:22 (inline)
  1 main(int, char**) • crasher.c:201
  2 start_main(const start_params*) • __libc_start_main.c:93
  3 __libc_start_main(zx_handle_t, int (*)(int, char**, char**)) • __libc_start_main.c:165
  4 _start + 0x14

[zxdb] list
   17   int (*func)(volatile unsigned int*);
   18   const char* desc;
   19 } command_t;
   20
   21 int blind_write(volatile unsigned int* addr) {
 ▶ 22   *addr = 0xBAD1DEA;
   23   return 0;
   24 }
   25
   26 int blind_read(volatile unsigned int* addr) { return (int)(*addr); }
   27
   28 int blind_execute(volatile unsigned int* addr) {
   29   void (*func)(void) = (void*)addr;
   30   func();
   31   return 0;
```

Within zxdb you can also do `help process-limbo` to get more information about how to use it.

### Process Limbo FIDL Service

The Process Limbo presents itself as a FIDL service, which is what the Process Limbo CLI tool and
zxdb use. The FIDL protocol is defined in `zircon/system/fidl/fuchsia-exception/process_limbo.fidl`.

A good example about how to use the API is the Process Limbo CLI tool itself: `src/developer/forensics/exceptions/limbo_client/limbo_client.cc`.

## Implementation

### Crash Service

When a process throws an exception, Zircon will generate an associated `exception handle`. It will
then look if there are any listeners in any associated exception channels that might be interested
in handling that exception. That is how debuggers such as zxdb get the exceptions from running
processes. See [the exceptions handling](/docs/concepts/kernel/exceptions.md) for more details.

But when there are no more exception handlers left, either because there weren't any or they all
decided to pass on handling it, the root job has an exclusive handler called `crashsvc`. Once an
exception has reached the Crash Service, it is understood that it has "crashed" and that no program
was able to handle it. The Crash Service will then dump the crashing stack trace to the logs and
pass the exception over to the `Exception Broker`.

### Exception Broker

The Exception Broker is in charge of deciding what is to be done with a crashing exception,
depending on the actual system configuration. It might decide to create a minidump file and dump a
crash report, send the exception over to the Process Limbo or kill the process.

The Exception Broker is aware of the Process Limbo and whether it is active or not. When it receives
an exception, it will check whether the Process Limbo is enabled. If so, it will pass the exception
handle over to it. This is the same Process Limbo exposed by the FIDL service.
