# Zircon and LK

Zircon was born as a branch of [LK](https://github.com/littlekernel/lk) and even
now many inner constructs are based on LK while the layers above are new. For
example, Zircon has the concept of a process but LK does not. However, a Zircon
process is made of LK-level constructs such as LK's ``thread_t``.

LK is a Kernel designed for small systems typically used in embedded
applications. It is a good alternative to commercial offerings like
[FreeRTOS](http://www.freertos.org/) or [ThreadX](http://rtos.com/products/threadx/).
Such systems often have a very limited amount of ram, a fixed set of peripherals
and a bounded set of tasks.

On the other hand, Zircon targets modern phones and modern personal computers
with fast processors, non-trivial amounts of ram with arbitrary peripherals
doing open ended computation.

More specifically, some the visible differences are:

+ LK can run in 32-bit systems. Zircon is 64-bit only.
+ Zircon has first class user-mode support. LK does not.
+ Zircon has a capability-based security model. In LK all code is trusted.

Over time, even the low level constructs have changed to accommodate the new
requirements and to better fit the rest of the system.

