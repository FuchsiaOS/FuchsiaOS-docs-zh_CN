# Debug the kernel using QEMU

Zircon can [run under emulation](/development/debugging/qemu.md)
using QEMU. QEMU can either be installed via prebuilt binaries, or built
locally.

## Prebuilt QEMU

QEMU is downloaded by `jiri` as part of `jiri update` or `jiri run-hooks`.

QEMU is fetched into `//prebuilt/third_party/qemu`. You can run it most
conveniently using `fx qemu` (see below).

## Build QEMU

### Install Prerequisites

Building QEMU on macOS requires a few packages. As of macOS 10.12.1:

```
# Using http://brew.sh
brew install pkg-config glib automake libtool

# Or use http://macports.org ("port install ...") or build manually
```

### Build

```
cd $SRC
git clone --recursive https://fuchsia.googlesource.com/third_party/qemu
cd qemu
./configure --target-list=aarch64-softmmu,x86_64-softmmu
make -j32
sudo make install
```

If you don't want to install in /usr/local (the default), which will require you
to be root, add --prefix=/path/to/install (perhaps $HOME/qemu). Then you'll
either need to add /path/to/install/bin to your PATH or use -q /path/to/install
when invoking run-zircon-{arch}.

## Run Zircon under QEMU

```
# for aarch64
fx set bringup.arm64
fx build
fx qemu

# for x86
fx set bringup.x64
fx build
fx qemu
```

If QEMU is not on your path, use -q <directory> to specify its location.

The -h flag will list a number of options, including things like -b to rebuild
first if necessary and -g to run with a graphical framebuffer.

To exit qemu, enter Ctrl-a x. Use Ctrl-a h to see other commands.

## Enabling Networking under QEMU

The run-zircon script, when given the -N argument will attempt to create
a network interface using the Linux tun/tap network device named "qemu".  QEMU
does not need to be run with any special privileges for this, but you need to
create a persistent tun/tap device ahead of time (which does require you be root):

On Linux:

```
sudo ip tuntap add dev qemu mode tap user $USER
sudo ip link set qemu up
```

This is sufficient to enable link local IPv6 (as the loglistener tool uses).

On macOS:

macOS does not support tun/tap devices out of the box; however, there is a widely
used set of kernel extensions called tuntaposx, which can be downloaded
[here](http://tuntaposx.sourceforge.net/download.xhtml). Once the installer
completes, the extensions will create up to 16 tun/tap devices. The
run-zircon-x64 script uses /dev/tap0.

```
sudo chown $USER /dev/tap0

# Run zircon in QEMU, which will open /dev/tap0
fx qemu -N

# (In a different window) bring up tap0 with a link local IPv6 address
sudo ip addr add dev tap0 fc00::/7
sudo ip link set tap0 up
```

<aside class="note">
One caveat with tuntaposx is that the network interface will
automatically go down when QEMU exits and closes the network device. So the
network interface needs to be brought back up each time QEMU is restarted. To
automate this, you can use the -u flag to run a script on qemu startup. An
example startup script containing the above command is located in
scripts/qemu-ifup-macos, so QEMU can be started with:

<pre>
fx qemu -Nu ./scripts/qemu-ifup-macos
</pre>
</aside>

## Using Emulated Disk under QEMU

Using builds based on core (really any product above bringup) will
automatically imply a disk that is provided to serve the `fvm` partition that
includes a minfs partition for mutable storage, and a blobfs partition for
package data storage.

You can attach additional images using flags as follows:

```
fx qemu -d [-D <disk_image_path (default: "blk.bin")>]
```


## Debugging the kernel with GDB

### Sample session

Here is a sample session to get you started.

In the shell you're running QEMU in:

```
shell1$ fx qemu -- -s -S
[... some QEMU start up text ...]
```

This will start QEMU but freeze the system at startup,
waiting for you to resume it with "continue" in GDB.
If you want to run QEMU without GDB, but be able to attach with GDB later
then start QEMU without "-S" in the above example:

```
shell1$ fx qemu -- -s
[... some QEMU start up text ...]
```

You will then need to locate `zircon.elf` in the `out` directory.

And then in the shell you're running GDB in:

```
shell2$ gdb path-to-zircon.elf -x ${FUCHSIA_DIR}/zircon/kernel/scripts/zircon.elf-gdb.py
Reading symbols from /fuchsia/out/default/kernel_x64-clang/zircon.elf...
Loading zircon.elf-gdb.py ...
Zircon extensions installed for /fuchsia/out/default/kernel_x64-clang/zircon.elf
(gdb) target extended-remote :1234
Remote debugging using :1234
0x000000000000fff0 in ?? ()

Thread 1 hit Breakpoint -1, 0x0000000000100050 in ?? ()
Watchpoint set on KASLR relocated base variable

Thread 1 hit Hardware read watchpoint -2: *0x767ca0

Value = 0
0x00000000001000ef in ?? ()
Update symbols and breakpoints for KASLR
KASLR: Correctly reloaded kernel at 0xffffffff00000000
(gdb) # Don't try to do too much at this point.
(gdb) # GDB can't handle architecture switching in one session,
(gdb) # and at this point the architecture is 16-bit x86.
(gdb) break lk_main
Breakpoint 1 at 0xfffffffff010cb58: file kernel/top/main.c, line 59.
(gdb) continue
Continuing.

Breakpoint 1, lk_main (arg0=1, arg1=18446744071568293116, arg2=0, arg3=0)
    at kernel/top/main.c:59
59	{
(gdb) continue
```

At this point Zircon boots and back in shell1 you'll be at the Zircon
prompt.

```
mxsh>
```

If you Ctrl-C in shell2 at this point you can get back to GDB.

```
(gdb) # Having just done "continue"
^C
Program received signal SIGINT, Interrupt.
arch_idle () at kernel/arch/x86/64/ops.S:32
32	    ret
(gdb) info threads
  Id   Target Id         Frame
  4    Thread 4 (CPU#3 [halted ]) arch_idle () at kernel/arch/x86/64/ops.S:32
  3    Thread 3 (CPU#2 [halted ]) arch_idle () at kernel/arch/x86/64/ops.S:32
  2    Thread 2 (CPU#1 [halted ]) arch_idle () at kernel/arch/x86/64/ops.S:32
* 1    Thread 1 (CPU#0 [halted ]) arch_idle () at kernel/arch/x86/64/ops.S:32
```

QEMU reports one thread to GDB for each CPU.

### The zircon.elf-gdb.py script

The `zircon/kernel/scripts/zircon.elf-gdb.py` script should be automatically loaded by gdb. If it's
not loaded automatically, you might need to add its path to gdb's auto-load-safe-path.
Alternatively, you can add manually set it in gdb's command line flag:

```
$ gdb path-to-zircon.elf -x ${FUCHSIA_DIR}/zircon/kernel/scripts/zircon.elf-gdb.py
```

It provides several things:

- KASLR relocation for gdb, allowing you to correctly set breakpoints in functions.

- Pretty-printers for zircon objects (alas none at the moment).

- Several zircon specific commands, all with a "zircon" prefix. To see them:

```
(gdb) help info zircon
(gdb) help set zircon
(gdb) help show zircon
```

- Enhanced unwinder support for automagic unwinding through kernel faults.

Heads up: This script isn't always updated as zircon changes.

**NOTE**: due to [bug 67893](https://fxbug.dev/67893), the KASLR part might not work if using qemu with kvm.
  As a workaround, you can execute the following in gdb:

```
(gdb) # before attaching to qemu
(gdb) mem 0 0xffffffffffffffff ro
(gdb) target extended-remote :1234
(gdb) ...
(gdb) # after the script performed the kaslr relocations
(gdb) mem auto
```

### Terminating the session

To terminate QEMU you can send commands to QEMU from GDB:

```
(gdb) monitor quit
(gdb) quit
```

### Interacting with QEMU from Gdb

To see the list of QEMU commands you can execute from GDB:

```
(gdb) monitor help
```

### Saving system state for debugging

If you have a crash that is difficult to debug, or that you need help
with debugging, it's possible to save system state akin to a core dump.

```
bash$ qemu-img create -f qcow2 /tmp/my_snapshots.qcow2 32M
```

will create a "32M" block storage device.  Next launch QEMU and tell it
about the device, but don't tell it to attach the device to the guest system.
This is OK; we don't plan on using it to back up the disk state, we just want
a core dump.  Note: all of this can be skipped if you are already emulating
a block device and it is using the qcow2 format.

```
bash$ qemu <normal_launch_args> -drive if=none,format=qcow2,file=/tmp/my_snapshots.qcow2
```

When you get to a point where you want to save the core state, drop to the QEMU
console using <C-a><C-c>.  You should get the (qemu) prompt at this point.
From here, just say:

```
(qemu) savevm my_backup_tag_name
```

Later on, from an identical machine (one launched with the same args as before),
you can drop to the console and run:

```
(qemu) loadvm my_backup_tag_name
```

to restore the state.  Alternatively, you can do it from the cmd line with:

```
bash$ qemu <normal_launch_args> -drive if=none,format=qcow2,file=/tmp/my_snapshots.qcow2 -loadvm my_backup_tag_name
```

In theory, you could package up the qcow2 image along with your build output
directory and anyone should be able to restore your state and start to poke
at stuff from the QEMU console.


[minfs-create-image]: /zircon/minfs.md#Host-Device-QEMU-Only
