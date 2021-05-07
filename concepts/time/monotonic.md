# Monotonic Time

Monotonic time is a measurement of the time since the system was powered on and
is maintained by the kernel. Note that the monotonic time may not always reset
to zero on reboot and the behavior during sleep/suspend has not yet been
defined.

Monotonic time is the most reliable time standard on Fuchsia and reading
monotonic time is usually cheaper than reading UTC or local time. Monotonic time
is always available and it always increases continuously and monotonically.
Monotonic time is locked to the frequency of the underlying hardware oscillator
and does not attempt to correct for any errors in that oscillator.

Since monotonic time counts from power on it, is only meaningful in the context
of a single power cycle on a single Fuchsia device.

Components may read monotonic time using
[`zx_clock_get_monotonic`](/docs/reference/syscalls/clock_get_monotonic.md).