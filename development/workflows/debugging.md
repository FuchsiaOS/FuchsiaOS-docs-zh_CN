# Debugging

This document is a work-in-progress and provides various suggestions
for debugging Fuchsia programs.

## The ZX debugger

For C/C++ code, try zxdb. See the [zxdb docs](https://fuchsia.googlesource.com/garnet/+/master/docs/debugger.md) for more details.

## Backtraces

### Automagic backtraces

Fuchsia starts a program at boot called "crashanalyzer" that reports
program crashes and prints a backtrace of the crashing thread.

Example:

```
$ crasher
[11156.652165][1048][1187][klog] INFO: devmgr: crash_analyzer_listener: analyzing exception type 0x108
[11156.652215][1107][1121][klog] INFO: <== fatal exception: process crasher[42410] thread initial-thread[42424]
[11156.652218][1107][1121][klog] INFO: <== fatal page fault, PC at 0x1e1888dbbbd7
[11156.652223][1107][1121][klog] INFO:  CS:                   0 RIP:     0x1e1888dbbbd7 EFL:            0x10246 CR2:                  0
[11156.652226][1107][1121][klog] INFO:  RAX:                  0 RBX:                0x1 RCX:     0x721ad98697c6 RDX:     0x77accb36f264
[11156.652229][1107][1121][klog] INFO:  RSI:                  0 RDI:                  0 RBP:     0x2781c4816f90 RSP:     0x2781c4816f80
[11156.652232][1107][1121][klog] INFO:   R8:                  0  R9:                  0 R10:                  0 R11:              0x246
[11156.652235][1107][1121][klog] INFO:  R12:     0x773bf11dcda0 R13:     0x773bf11dcdd0 R14:               0x16 R15:         0x78050d69
[11156.652236][1107][1121][klog] INFO:  errc:               0x6
[11156.652237][1107][1121][klog] INFO: bottom of user stack:
[11156.652244][1107][1121][klog] INFO: 0x00002781c4816f80: f11dcda0 0000773b 9ccd2b38 000039b2 |....;w..8+...9..|
[11156.652248][1107][1121][klog] INFO: 0x00002781c4816f90: c4816fd0 00002781 88dbbba7 00001e18 |.o...'..........|
[11156.652252][1107][1121][klog] INFO: 0x00002781c4816fa0: 00000008 00000000 9ccd2b38 000039b2 |........8+...9..|
[11156.652255][1107][1121][klog] INFO: 0x00002781c4816fb0: f11dcf70 0000773b f11dcf70 0000773b |p...;w..p...;w..|
[11156.652260][1107][1121][klog] INFO: 0x00002781c4816fc0: cb36f570 000077ac f11dcdd0 0000773b |p.6..w......;w..|
[11156.652270][1107][1121][klog] INFO: 0x00002781c4816fd0: c4816ff0 00002781 cb2b0d0f 000077ac |.o...'....+..w..|
[11156.652277][1107][1121][klog] INFO: 0x00002781c4816fe0: 00000054 00000000 f11dcf70 0000773b |T.......p...;w..|
[11156.652281][1107][1121][klog] INFO: 0x00002781c4816ff0: f11dcfe0 0000773b 00000000 00000000 |....;w..........|
[11156.652282][1107][1121][klog] INFO: arch: x86_64
[11156.652550][1107][1121][klog] INFO: dso: id=a94c78564173530d51670b6586b1aa471e004f06 base=0x7d3506a49000 name=libfdio.so
[11156.652553][1107][1121][klog] INFO: dso: id=a61961ba9776a67a00fb322af9ebbdcfd1ce3f62 base=0x77accb297000 name=libc.so
[11156.652554][1107][1121][klog] INFO: dso: id=760f1e6e47d3dd8b6a19150aa47241279ec75a9c base=0x721ad9863000 name=<vDSO>
[11156.652558][1107][1121][klog] INFO: dso: id=b18462140c6784a53736105bbf3021852eeda68c base=0x1e1888dbb000 name=app:crasher
[11156.652637][1107][1121][klog] INFO: bt#01: pc 0x1e1888dbbbd7 sp 0x2781c4816f80 (app:crasher,0xbd7)
[11156.652750][1107][1121][klog] INFO: bt#02: pc 0x1e1888dbbba7 sp 0x2781c4816fa0 (app:crasher,0xba7)
[11156.652847][1107][1121][klog] INFO: bt#03: pc 0x77accb2b0d0f sp 0x2781c4816fe0 (libc.so,0x19d0f)
[11156.652978][1107][1121][klog] INFO: bt#04: pc 0 sp 0x2781c4817000
[11156.653027][1107][1121][klog] INFO: bt#05: end
```

Since debug information is currently not available on the target,
a program (`symbolize`) must be run on the development host to
translate the raw addresses in the backtrace to symbolic form.
Any easy way to capture this output from the target is by running
the `loglistener` program on your development host.

```
$ fx syslog | fx symbolize
[11156.652165][1048][1187][klog] INFO: devmgr: crash_analyzer_listener: analyzing exception type 0x108
... same output as "raw" backtrace ...
start of symbolized stack:
#01: blind_write at ../../zircon/system/uapp/crasher/crasher.c:21
#02: main at ../../zircon/system/uapp/crasher/crasher.c:137
#03: start_main at ../../zircon/third_party/ulib/musl/src/env/__libc_start_main.c:49
#04: unknown, can't find pc, sp or app/library in line
end of symbolized stack
```


### Manually requesting backtraces

Akin to printf debugging, one can request crashlogger print a
backtrace at a particular point in your code.

Include this header:

```
#include <zircon/crashlogger.h>
```

and then add the following where you want the backtrace printed:

```
void my_function() {
  ...
  crashlogger_request_backtrace();
  ...
}
```
