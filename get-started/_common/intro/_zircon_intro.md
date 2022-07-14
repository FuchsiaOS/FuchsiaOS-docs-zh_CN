[Zircon][glossary.zircon] is the core that powers Fuchsia.
It is composed of a kernel and a small set of userspace services, drivers,
and libraries necessary for core system functions such as booting.

Although [Zircon][glossary.zircon] applies many of the concepts popularized by
microkernels, it does not strive to be minimal. Instead, the microkernel-like
architecture of Zircon enables Fuchsia to reduce the amount of trusted code
running in the system to a few core functions:

* Memory management
* Scheduling
* Inter-process communication

![Data table showing a comparison between kernel services in Fuchsia and a
typical operating system, indicating Fuchsia includes fewer services in its
kernel.]
(/get-started/images/intro/kernel-services.png){: width="799"}


[glossary.zircon]: /glossary/README.md#zircon
