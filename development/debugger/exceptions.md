# Working with exceptions in zxdb

## Overview

[Exceptions in Zircon](/docs/concepts/kernel/exceptions.md) are handled in several phases:

  1. The debugger is notified of the exception ("first chance"). The debugger might handle the
  exception at this stage (for example, the debugger might continue continue after a single-step or
  breakpoint exception) in which case exception processing stops.

  2. The debugger can choose to forward the exception to the normal handlers as if a debugger were
  not present. The program itself may resolve the exception at this point.

  3. If still unhandled, the debugger will get the exception again as a "second chance" exception.

## Forwarding exceptions

Continuing execution (via `continue`, `step`, `next`, etc.) after an exception will re-run the
excepting instruction. Normally, this will cause the same exception again and the program will not
make progress. In particular, this will cause problems with gtest "death tests" where an exception is
the expected result of the test. The test harness expects to catch this exception and continue
with the test.

To forward the exception to the program, the exception needs to be explicitly forwarded. This is
done with the `--forward` (`-f` for short) flag to the `continue` command:

For example, upon the expected crash, a death test will report:

```none {:.devsite-disable-click-to-copy}
══════════════════════════
 Invalid opcode exception
══════════════════════════
 Process 2 (koid=57368) thread 7 (koid=57563)
 Faulting instruction: 0x4356104fba24

🛑 Process 2 Thread 7 scudo::die() • fuchsia.cpp:28
   26 uptr getPageSize() { return PAGE_SIZE; }
   27
 ▶ 28 void NORETURN die() { __builtin_trap(); }
   29
   30 // We zero-initialize the Extra parameter of map(), make sure this is consistent
```

And to continue on with the test:

```none {:.devsite-disable-click-to-copy}
[zxdb] continue --forward

[zxdb] c -f             # Alternate Short form.
```

## Automatically forwarding certain types of exceptions

The debugger can automatically forward certain exception types to the program and only handle
them as second-chance exceptions. By default, only page faults are included.

The debugger's `second-chance-exception` setting contains the list of exceptions that will be
handled only as second-chance by default. This setting holds a list of exception type abbreviations:

 * "gen": general
 * "pf": page faults
 * "ui": undefined instruction
 * "ua": unaligned access

See the debugger's `help get` and `help set` for more details on dealing with list settings. Some
examples:

```none {:.devsite-disable-click-to-copy}
[zxdb] get second-chance-exceptions           # List the current values.

[zxdb] set second-chance-exceptions += gen    # Add "general" to the list.

[zxdb] set second-chance-exceptions -= pf     # Remove "page fault" from the list.
```
