# Hardware watchdog timers

## Overview

A hardware watchdog timer (WDT) is a special piece of hardware responsible for
resetting a system in the event of a hard system-wide lockup. They are
frequently present in system-on-a-chip (SoC) designs, especially in SoCs
targeted towards smaller embedded systems applications. They can be an important
aspect of system design in that they can trigger a reboot of a system that has
locked up so completely that is has become incapable of managing mission
critical tasks, such as active thermal management, before the system suffers
irreparable damage. In addition, they can help to mitigate a poor user
experience in an embedded system by automatically resetting a hard-locked system
without user intervention. No one wants to have to deal with a locked-up device,
but if the worst happens, it is much more preferable that a system automatically
reset itself instead of forcing a user to suffer a non-responsive system until
they decide that it needs to be power cycled, and then have to go and physically
unplug the device in order to recover it.

A watchdog timer (just "watchdog" or "WDT" for short) typically works by being
configured to count at a particular rate up to a threshold count. If the counter
reaches the threshold before being reset by software, the WDT will
automatically, and un-gracefully, reboot the system. The act of resetting the
WDT from software is commonly referred to as "petting" the watchdog. The period
of a hardware watchdog in a system tends to be rather large (one or more
seconds) as this mechanism is an absolute worst case fail-safe. The system
should be well and truly locked-up before the watchdog ever fires.

## Usage in Zircon

A WDT in Zircon, when available, is used to protect the absolute lowest level of
the system. When enabled, it is nominally pet somewhere between 1/4 and 1/2 of
the way through it cycle by a kernel level timer, meaning that it is pet in a
hard IRQ context independent of threads. Petting the watchdog is not subject to
thread weights, deadlines, or any other scheduler behavior. For a hardware
watchdog to fire and reboot the system, the system needs locked up to the point
that timer IRQs cannot be serviced.

As mentioned above, watchdog timers are hardware specific entities. Whether or
not one exists, what is it capable of doing, and specifically how to operate one
(when it exists), are not common to an architecture like x64 or ARM64. Given the
location of the "pet" operation at the absolute lowest level of the Zircon
kernel, it is up to the kernel to pet the dog, not any hardware specific
user-mode drivers.

Because of this, it is up to the bootloader to configure the watchdog properly
and communicate to the kernel (through the ZBI) the important details of whether
or not the WDT exists, whether it is enabled, how frequently it must be pet, and
how to pet, enable, or disable it. A system running zircon only "has" a WDT if
the bootloader tells it that it does and how to operate it. While a bootloader
must tell the kernel how to pet the watchdog when present and enabled, it might
not tell the kernel how to disable it. This could be the result of either a
system design decision, or because the WDT cannot be disabled from the kernel.

Typically, hardware WDTs are configured and enabled by the bootloader just
before control is transferred to the kernel. This way, if the kernel completely
locks up during startup, the WDT will reset the system. On the other side of the
fence, the kernel attempts to recognize and pet the WDT as early as possible in
the boot sequence. Later on, it will settle into a pattern of periodically
petting the dog once boot has progressed to the point where it is possible to
set timers.

## Methods for controlling the watchdog during development

The vast majority of developers should never need to do anything with a watchdog
timer, or even be aware of it existing. For it to fire during normal operation
is an indication of something going rather badly wrong. In some situations,
however, developers may be in a situation where they need to hold off hard interrupt
requests (IRQs) for excessive amounts of time as part of investigating a bug, or
other performance issues. In these situations, it is good to know what options
exist for controlling the watchdog, and not getting bit at inappropriate times.

### Use the kernel shell extension

If you have access to the kernel shell and the system is stable enough to boot
to the point where the kernel shell is accessible, you can use the shell
extension to manipulate the WDT.  Run `k wdt help` to see a list of the
available commands.  Run `k wdt status` to see if the kernel is aware of any
hardware WDTs at all, and if it is, whether the WDT is enabled or not, what the
nominal pet period is, and how long ago the timer was last pet.  If needed, you
can run `k wdt disable` to disable the watchdog. You can only disable the WDT if
the bootloader has told the kernel how to disable the WDT.

### Use the kernel command line

You can pass kernel command line arguments to control the watch dog.  You can
send `kernel.force-watchdog-disabled=true` to tell the kernel to force disable
the watchdog as early as possible during the boot. This can be useful if
problems are causing the watchdog to fire before it gets to the point where the
kernel shell is easily accessible. However, this is only an option if the
bootloader has told the kernel how to disable the watchdog.
