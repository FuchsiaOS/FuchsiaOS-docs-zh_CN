# Debugging workflow

## Native code debugging

Fuchsia has a debugger for native code (C++ and Rust) called zxdb.

Please see the [zxdb documentation](/development/debugger/README.md) for
more details.

## Backtraces

### Automagic backtraces

Fuchsia starts a program at boot called "crashsvc" that reports program
crashes and prints a backtrace of the crashing thread.

Example:

```
$ ffx log
...
[10023.913][klog][klog][I] crashsvc: exception received, processing
[10023.913][klog][klog][I] <== CRASH: process crasher[1007318] thread initial-thread[1007320]
[10023.913][klog][klog][I] <== write not-present page fault (error ZX_ERR_NOT_FOUND) at 0, PC at 0x2f3734f90fc
[10023.913][klog][klog][I]  CS:                   0 RIP:      0x2f3734f90fc EFL:            0x10246 CR2:                  0
[10023.913][klog][klog][I]  RAX:                  0 RBX:     0x43cf4e707f58 RCX:                  0 RDX:                  0
[10023.913][klog][klog][I]  RSI:      0x2f3734de1f1 RDI:                  0 RBP:      0x1d7cee8df60 RSP:      0x1d7cee8df60
[10023.913][klog][klog][I]   R8: 0xdc1287fe16b1b597  R9:                  0 R10: 0xdc1287fe16b1b597 R11:       0xc87a4939e8
[10023.913][klog][klog][I]  R12:       0xc87a495fb0 R13:       0xc87a495fa0 R14:     0x4066b869c998 R15:       0xc87a495fc0
[10023.913][klog][klog][I]  fs.base:      0x3bc9af38b18 gs.base:                  0
[10023.913][klog][klog][I]  errc:               0x6
[10023.913][klog][klog][I] bottom of user stack:
[10023.913][klog][klog][I] 0x000001d7cee8df60: cee8df90 000001d7 734fa0ca 000002f3 |..........Os....|
[10023.913][klog][klog][I] 0x000001d7cee8df70: 7353f000 000002f3 734dc1b1 000002f3 |..Ss......Ms....|
[10023.913][klog][klog][I] 0x000001d7cee8df80: 7a495fb0 000000c8 00000001 00000000 |._Iz............|
[10023.913][klog][klog][I] 0x000001d7cee8df90: cee8dfe0 000001d7 b82036f1 00004066 |.........6 .f@..|
[10023.913][klog][klog][I] 0x000001d7cee8dfa0: 00000000 00000000 00000001 00000000 |................|
[10023.913][klog][klog][I] 0x000001d7cee8dfb0: 00000001 00000000 4e707fd0 000043cf |..........pN.C..|
[10023.913][klog][klog][I] 0x000001d7cee8dfc0: 0000000b 00000000 4e707f30 000043cf |........0.pN.C..|
[10023.913][klog][klog][I] 0x000001d7cee8dfd0: 4e707e90 000043cf 4e707ec0 000043cf |.~pN.C...~pN.C..|
[10023.913][klog][klog][I] 0x000001d7cee8dfe0: cee8dff0 000001d7 b820393b 00004066 |........;9 .f@..|
[10023.913][klog][klog][I] 0x000001d7cee8dff0: 00000000 00000000 735146f1 000002f3 |.........FQs....|
[10023.913][klog][klog][I] memory dump near pc:
[10023.913][klog][klog][I] 0x000002f3734f90cc: cc cc cc cc 55 48 89 e5 48 83 ec 10 48 89 7d f8 |....UH..H...H.}.
[10023.913][klog][klog][I] 0x000002f3734f90dc: 48 8b 7d f8 e8 9b 41 04 00 48 83 c4 10 5d c3 cc |H.}...A..H...]..
[10023.913][klog][klog][I] 0x000002f3734f90ec: cc cc cc cc 55 48 89 e5 48 89 7d f8 48 8b 45 f8 |....UH..H.}.H.E.
[10023.913][klog][klog][I] 0x000002f3734f90fc: c7 00 ea 1d ad 0b 31 c0 5d c3 cc cc cc cc cc cc |......1.].......
[10023.913][klog][klog][I] arch: x86_64
[10023.917][klog][klog][I] [[[ELF module #0x0 "<VMO#1007304=/boot/bin/crasher>" BuildID=d2974008fa020a45 0x2f3734d9000]]]
[10023.918][klog][klog][I] [[[ELF module #0x1 "libc.so" BuildID=bda30eefb7cce4af 0x4066b819b000]]]
[10023.918][klog][klog][I]    #0    0x000002f3734f90fc in blind_write(volatile unsigned int*) ../../src/developer/forensics/crasher/cpp/crasher.c:25 <<VMO#1007304=/boot/bin/crasher>>+0x200fc sp 0x1d7cee8df60
[10023.918][klog][klog][I]    #1    0x000002f3734fa0ca in main(int, char**) ../../src/developer/forensics/crasher/cpp/crasher.c:352 <<VMO#1007304=/boot/bin/crasher>>+0x210ca sp 0x1d7cee8df70
[10023.918][klog][klog][I]    #2    0x00004066b82036f1 in start_main(const start_params*) ../../zircon/third_party/ulib/musl/src/env/__libc_start_main.c:161 <libc.so>+0x686f1 sp 0x1d7cee8dfa0
[10023.918][klog][klog][I]    #3    0x00004066b820393b in __libc_start_main(zx_handle_t, int (*)(int, char**, char**)) ../../zircon/third_party/ulib/musl/src/env/__libc_start_main.c:238 <libc.so>+0x6893b sp 0x1d7cee8dff0
[10023.918][klog][klog][I]    #4    0x000002f3735146f1 in _start(zx_handle_t) ../../zircon/system/ulib/c/Scrt1.cc:7 <<VMO#1007304=/boot/bin/crasher>>+0x3b6f1 sp 0x43cf4e707fe0
[10023.918][klog][klog][I]    #5    0x0000000000000000 is not covered by any module sp 0x43cf4e707ff0
```

### Manually requesting backtraces

Akin to printf debugging, one can request crashsvc to print a backtrace at a
particular point in your code.

Include this header from zircon's backtrace-request library, which you must
depend on in your target's BUILD.gn rules:

```
#include <lib/backtrace-request/backtrace-request.h>
```

and then add the following where you want the backtrace printed:

```
void my_function() {
  ...
  backtrace_request();
  ...
}
```

### Inspecting FIDL messages

Fuchsia has a tool for viewing FIDL messages as they are sent and received. See
details at
[the doc page for fidl inspection](/development/monitoring/fidlcat/README.md).
