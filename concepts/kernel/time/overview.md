# Time Overview

Components on Fuchsia can read current time using three different
[time standards][1]:

* **[Monotonic](monotonic.md)**: Monotonic time is a measurement of the time
  since the system was powered on. Monotonic time always moves forwards and is
  always available to all applications but is only meaningful with the context
  of a single power cycle on a single Fuchsia device.
* **[UTC](utc/overview.md)**: UTC time is the system’s best estimate of
  [Coordinated Universal Time][2]. UTC is usually acquired over a network
  so there are conditions under which the system may not know UTC. Developers
  using UTC should pay particular attention to [UTC behavior](utc/behavior.md)
  to understand the unique properties of UTC on Fuchsia.
* **[Local](local.md)**: Local time is the system’s best estimate of the
  [standard time][3] at the device’s location, aka "wall clock time". Local time
  is derived from UTC and time zone, so inherits much of the
  [UTC behavior](utc/behavior.md). There are conditions under which local time
  is not available. The local time will jump substantially if the user changes
  time zone.

These time standards are frequently available through the
[time functions in supported languages](language_support.md) in addition to
[time syscalls](/docs/reference/syscalls/clock_create.md).

As a developer, you must select the most appropriate time standard to address
each problem. Monotonic time has the fewest failure modes and the most stable
behavior so should generally be the default choice unless there is a reason that
monotonic time will not work. UTC has fewer failure modes and more stable
behavior than local time so should be preferred over local time unless there is
some reason that local time is necessary.

For example:

1. Use monotonic time to implement a ten second delay between retries.
   Monotonic time will be available in all cases so provides the simplest and
   most reliable solution.
1. Use UTC time to expire and delete a file stored on disk after seven days.
   Here monotonic time would not allow the expiry time to be
   preserved across power cycles and local time would have coupled the
   correctness of the expiry to the timezone setting.
1. Use UTC time to timestamp an on-device event that will be read by some
   server. In this case monotonic time would not work since the server probably
   does not know when the Fuchsia device last powered on. Using local time would
   require that the device and server agree on the timezone, which would be error
   prone.
1. Use local time to display the current time to the user as an analog clock
   face. Local time is most natural time standard for users so no other time
   standards are practical here.

Testing code that depends on time can be difficult on any platform. Tools and
best practices for testing time dependencies on Fuchsia are being developed and
will be linked here when available.

[1]: https://en.wikipedia.org/wiki/Time_standard
[2]: https://en.wikipedia.org/wiki/Coordinated_Universal_Time
[3]: https://en.wikipedia.org/wiki/Standard_time
