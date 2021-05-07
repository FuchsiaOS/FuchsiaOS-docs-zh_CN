# UTC Time

UTC time is the system’s best estimate of [Coordinated Universal Time][1].

[Architecture](architecture.md) covers the overall architecture of the time
synchronization system including the roles and responsibilities of each
component involved.

On boards with a Real Time Clock (RTC), the system initializes UTC time from the
RTC shortly after power on. On all boards, the system periodically updates UTC
from one or more network time sources once a network is available. In principle,
these time sources could be configured separately for each product, but
currently all products source UTC from google.com using
[HTTPSdate](/src/sys/time/httpsdate_time_source/README.md).

[Algorithms](algorithms.md) covers the time synchronization algorithms
implemented by the central component in the time synchronization system,
[Timekeeper](/src/sys/time/timekeeper).

UTC is distributed to components using a
[kernel clock object](/docs/reference/kernel_objects/clock.md), with each
process holding a handle to this clock object (providing the parent process
supplied this clock). If you are developing components that use UTC, you should
read [UTC behavior](behavior.md) to learn how to acquire UTC time and understand
the behavior the UTC clock may exhibit both before and after time
synchronization.

Previously the kernel maintained an additional internal UTC clock that could be
accessed through [`zx_clock_get`](/docs/reference/syscalls/clock_get.md). This
clock is deprecated and should no longer be used.

[1]: https://en.wikipedia.org/wiki/Coordinated_Universal_Time