# Debugging Tips

For general debugging info see the [Fuchsia Debugging Workflow][fuchsia-debugging-doc].

## Generating debug info

There are several GN build arguments used to control the generation of debug
info.

### `symbol_level`

`symbol_level` specifies level of debug info to generate.
The default is `-g3`.
A useful value for getting less debug info usable in backtraces is
`symbol_level = 1`.

### BOOTFS_DEBUG_MODULES

BOOTFS\_DEBUG\_INFO\_FILES allows one to specify which modules
(apps,libs,tests) have their associated debug info included
in the boot image.

The value is a comma-separated list of "module short names"
which are generally `parent_directory/module_directory`.
E.g., `ulib/launchpad,utest/debugger`
Make-style patterns (%) are allowed, e.g., `ulib/%,utest/debugger`.

The default is empty (meaning none).

## Adding debug info to boot image

By default the boot image does not contain debug info as it
can require a lot of extra space. Adding debug info is useful when
using tools like debuggers natively. Note that this does not apply
to cross debugging where the debugger is running on separate machine.
Adding debug info to the boot image is for when you are running debugging
tools on zircon itself.

Example:

```
$ gn gen build-zircon --args='BOOTFS_DEBUG_MODULES=ulib/%,utest/debugger symbol_level = 1'
```

This example will include in the boot image debug info files for all
shared libraries and for the "debugger" test program. To reduce the amount
of debug info to just that usable in backtraces `symbol_level = 1`
is passed.

## Debugging the kernel with QEMU+GDB.

See "Debugging the kernel with GDB" in [QEMU](/docs/development/debugging/qemu.md) for
documentation on debugging zircon with QEMU+GDB.

[fuchsia-debugging-doc]: /docs/development/debugging/debugging.md

## Symbolizing the backtraces

To automatically symbolize the backtraces when running zircon, pass the logs
through the symbolizer as follows:

```
fx set bringup.x64 --variant asan  # or bringup.arm64
fx build
fx ffx emu start --console | ffx debug symbolize
...
> crasher
...
[00021.715] 01044.01212> devmgr: crash_analyzer_listener: analyzing exception type 0x108
[00021.721] 01102.01116> <== fatal exception: process crasher[2853] thread initial-thread[2867]
[00021.721] 01102.01116> <== fatal page fault, PC at 0x38ed815cdbd7
[00021.721] 01102.01116>  CS:                   0 RIP:     0x38ed815cdbd7 EFL:              0x246 CR2:                  0
[00021.721] 01102.01116>  RAX:                  0 RBX:                0x1 RCX:                0x1 RDX:     0x75dec26db264
[00021.722] 01102.01116>  RSI:                  0 RDI:                  0 RBP:      0x5663cdc3f90 RSP:      0x5663cdc3f80
[00021.722] 01102.01116>   R8:                  0  R9:                  0 R10:                  0 R11:              0x206
[00021.722] 01102.01116>  R12:     0x6a3f40970d70 R13:     0x6a3f40970db0 R14:               0x16 R15:         0x7986f4ef
[00021.722] 01102.01116>  errc:               0x6
[00021.722] 01102.01116> bottom of user stack:
[00021.723] 01102.01116> 0x000005663cdc3f80: 40970d70 00006a3f 9af1eb38 00006fea |p..@?j..8....o..|
[00021.724] 01102.01116> 0x000005663cdc3f90: 3cdc3fd0 00000566 815cdba7 000038ed |.?.<f.....\..8..|
[00021.724] 01102.01116> 0x000005663cdc3fa0: 00000008 00000000 9af1eb38 00006fea |........8....o..|
[00021.724] 01102.01116> 0x000005663cdc3fb0: 40970f70 00006a3f 40970f70 00006a3f |p..@?j..p..@?j..|
[00021.724] 01102.01116> 0x000005663cdc3fc0: c26db570 000075de 40970db0 00006a3f |p.m..u.....@?j..|
[00021.724] 01102.01116> 0x000005663cdc3fd0: 3cdc3ff0 00000566 c261cdef 000075de |.?.<f.....a..u..|
[00021.724] 01102.01116> 0x000005663cdc3fe0: 00000054 00000000 40970f70 00006a3f |T.......p..@?j..|
[00021.724] 01102.01116> 0x000005663cdc3ff0: 40970fe0 00006a3f 00000000 00000000 |...@?j..........|
[00021.724] 01102.01116> arch: x86_64
[00021.728] 01102.01116> dso: id=31c12edecfd596b0be787e782f896efadf23e3da base=0x75dec2603000 name=libc.so
[00021.728] 01102.01116> dso: id=b4f9333e0d1bb7e79370905f90299d1da94e4271 base=0x51d5c67da000 name=<vDSO>
[00021.728] 01102.01116> dso: id=a0106c6ceae6a63d35eb7e8923ebc1a62a8df3e8 base=0x38ed815cd000 name=app:crasher
[00021.728] 01102.01116> dso: id=881704361e6af74805ab9e2a236ccf2962cdecc9 base=0x2ce98f7b2000 name=libfdio.so
[00021.738] 01102.01116> bt#01: pc 0x38ed815cdbd7 sp 0x5663cdc3f80 (app:crasher,0xbd7)
[00021.746] 01102.01116> bt#02: pc 0x38ed815cdba7 sp 0x5663cdc3fa0 (app:crasher,0xba7)
[00021.747] 01102.01116> bt#03: pc 0x75dec261cdef sp 0x5663cdc3fe0 (libc.so,0x19def)
[00021.749] 01102.01116> bt#04: pc 0 sp 0x5663cdc4000
[00021.749] 01102.01116> bt#05: end

start of symbolized stack:
#01: blind_write at ./system/uapp/crasher/crasher.c:21
#02: main at ./system/uapp/crasher/crasher.c:137
#03: start_main at ./third_party/ulib/musl/src/env/__libc_start_main.c:49
#04: unknown, can't find pc, sp or app/library in line
end of symbolized stack
```

## Kernel commands
Zircon has a number of shell commands related to the kernel and debugging accessible via the `k`
command. More information on them can be found through `k help`.
