# Clock

## NAME

clock - Kernel object used to track the progress of time.

## SYNOPSIS

A clock is a one dimensional affine transformation of the
[clock monotonic](/docs/reference/syscalls/clock_get_monotonic.md) reference
timeline, which may be atomically adjusted by a clock maintainer, and observed by
clients.

## DESCRIPTION

### Properties

The properties of a clock are established when the clock is created and cannot
be changed afterwards. Currently, three clock properties are defined.

#### **ZX_CLOCK_OPT_MONOTONIC**

When set, the clock is guaranteed to have monotonic behavior. This is to say
that any sequence of observations of the clock is guaranteed to produce a
sequence of times that are always greater than or equal to the previous
observations. A monotonic clock can never go backwards, although it _can_ jump
forwards. Formally:

Given a clock _C_, Let C(x) be the function that maps from the reference
timeline _C's_ timeline. C(x) is a piecewise linear function made up of all the
affine transformation segments over all time as determined  by _C's_ maintainer.
_C_ is monotonic if and only if:

for all _R<sub>1</sub>_, _R<sub>2</sub>_ : _R<sub>2</sub> >= R<sub>1</sub>_

_C(R<sub>2</sub>) >= C(R<sub>1</sub>)_

#### **ZX_CLOCK_OPT_CONTINUOUS**

When set, the clock is guaranteed to have continuous behavior. This is to say
that any update to the clock transformation is guaranteed to be first order
continuous with the previous transformation segment. Formally:

Let _C<sub>i</sub>(x)_ be the _i<sub>th</sub>_ affine transformation segment of
_C(x)_. Let _R<sub>i</sub>_ be the first point in time on the reference timeline
for which _C<sub>i</sub>(x)_ is defined. A clock _C_ is continuous if and only
if: for all _i_

_C<sub>i</sub>(R<sub>i + 1</sub>) = C<sub>i + 1</sub>(R<sub>i + 1</sub>)_

#### **Backstop Time**

The backstop time of a clock represents the minimum value that a clock may ever
be set to. Since clocks can only tick forwards, and never backwards, it is
impossible for an observer of a clock to ever receive a value that is less than
the backstop time configured by a clock's creator.

A backstop time may be provided via the `zx_create_args_v1_t` structure at
creation time. Otherwise, it will default to 0.

During clock update operations, any attempt to set the clock's value to
something less than the backstop time will fail with **ZX_ERR_INVALID_ARGS**. A
clock that has not been initially set will always report the backstop time
configured for the clock. Backtop times may never be less than the default
value of zero.

### Implied properties

+ The reference clock for all clock objects in the system is clock monotonic.
+ The nominal units of all clock objects are specified to be nanoseconds. This
  property is not configurable.
+ The units of frequency adjustment for all clock objects are specified to be
  parts per million, or PPM.
+ The maximum permissible range of frequency adjustment of a clock object is
  specified to be [-1000, +1000] PPM. This property is not configurable.

### Additional creation options

#### **ZX_CLOCK_OPT_AUTO_START**
When you use this option during clock creation, the clock begins in a started
state instead of the default non-started state. See [Starting a
clock](#starting-a-clock) for details.

### Reading the clock

Given a clock handle, users may query the current time given by that clock using
the `zx_clock_read()` syscall. Clock reads **ZX_RIGHT_READ** permissions. Clock
reads are guaranteed to be coherent for all observers. This is to say that, if
two observers query the clock at exactly the same reference time _R_, that they
will always see the same value _C(R)_.

### Reference timelines, `zx_ticks_get()`, and `zx_clock_get_monotonic()`

As noted earlier, zx_clock_get_monotonic() is the reference timeline for all
user-created zircon clocks. This means that if a user knows a clock instance's
current transformation, then given a value on the clock instance's timeline, the
corresponding point on the clock monotonic timeline may be computed (and
vice-versa). It also means that in the absence of a rate adjustment made to the
kernel clock, clock monotonic and the kernel clock will tick at exactly the same
rate.

In addition to the clock monotonic timeline, the zircon kernel also exposes the
"ticks" timeline via `zx_ticks_get()` and `zx_ticks_per_second()`. Internally,
ticks is actually the reference timeline for clock monotonic and is read
directly from an architecture appropriate timer unit accessible to the kernel.
Clock monotonic is actually a linear transformation of the ticks timeline
normalized to nanosecond units. Both timelines start ticking from zero as the
kernel starts.

Because clock monotonic is a static transformation based on ticks, and all kernel
clocks are transformations based on clock monotonic, ticks may also serve as a
reference clock for kernel clocks in addition to clock monotonic.

### Fetching the clock's details

In addition to simply reading the current value of the clock, advanced users who
possess **ZX_RIGHT_READ** permissions may also read the clock and get extended
details in the process using `zx_clock_get_details()`. Upon a successful call,
the details structure returned to callers will include:

+ The current clock monotonic to clock transformation.
+ The current ticks to clock transformation.
+ The current symmetric [error bound estimate](#error-bound) (if any) for the
  clock.
+ The last time the clock was updated as defined by the clock monotonic
  reference timeline.
+ An observation of the system tick counter, which was taken during the
  observation of the clock.
+ All of the static properties of the clock defined at creation time.
+ A generation nonce.

Advanced users may use these details to not only compute a recent `now` value
for the clock (by transforming the reported ticks-now observation using the
ticks-to-clock transformation, both reported by the get details operation), but
to also:

+ Know whether the clock transformation has been changed since the last
  `zx_clock_get_details()` operation (using the generation nonce).
+ Compose the clock transformation with other clocks' transformations to reason
  about the relationship between two clocks.
+ Know the clock maintainer's best estimate of [error bound](#error-bound).
+ Reason about the range of possible future values of the clock relative to the
  reference clock based on the last correction time, the current transformation,
  and the maximum permissible correction factor for the clock (see the maximum
  permissive range of frequency adjustment described in the |Implied properties|
  section above.

### Starting a clock and clock signals {#starting-a-clock}

Immediately after creation, a clock has not yet been started. All attempts to
read the clock will return the clock's configured backstop time, which defaults
to 0 if unspecified during creation.

A clock begins running after the very first update operation performed by a
clock's maintainer, which **must** include a set-value operation. The clock
will begin running at that point with a rate equal to the reference clock plus
the deviation from nominal specified by the maintainer.

Clocks also have a **ZX_CLOCK_STARTED** signal, which may be used by users to
know when the clock has actually been started. Initially, this signal is not
set, but it becomes set after the first successful update operation. Once
started, a clock will never stop and the **ZX_CLOCK_STARTED** signal will always
be asserted.

Initially, the clock is a clone of clock monotonic, which makes the
transformation between the clock monotonic timeline and synthetic timeline the
identity function. This clock may still be [maintained](#maintaining-a-clock)
after creation, subject to the limitations imposed by rights, the
**ZX_CLOCK_OPT_MONOTONIC** and **ZX_CLOCK_OPT_CONTINUOUS** properties, and the
configured backstop time.

If a clock is created with the **ZX_CLOCK_OPT_AUTO_START** options, it cannot be
configured with a backstop time that is greater than the current clock
monotonic time. If this was allowed, this would result in a state where a
clock's current time is set to a time before its backstop time.



### Maintaining a clock {#maintaining-a-clock}

Users who possess **ZX_RIGHT_WRITE** permissions for a clock object may act as a
maintainer of the clock using the `zx_clock_update()` syscall. Three parameters
of the clock may be adjusted during each call to `zx_clock_update()`, but not
all three need to be adjusted each time. These values are:

+ The clock's absolute value.
+ The frequency adjustment of the clock (deviation from nominal expressed in
  ppm)
+ The absolute [error bound estimate](#error-bound) of the clock (expressed in
  nanoseconds)

Changes to a clocks transformation occur during the syscall itself. The
specific reference time of the adjustment may not be specified by the user.

Any change to the absolute value of a clock with the **ZX_CLOCK_OPT_MONOTONIC**
property set on it which would result in non-monotonic behavior will fail with a
return code of **ZX_ERR_INVALID_ARGS**.

The first update operation is what starts a clock ticking and **must** include a
set-value operation.

Aside from the very first set-value  operation, all attempts to set the absolute
value of a clock with the **ZX_CLOCK_OPT_CONTINUOUS** property set on it will
fail with a return code of **ZX_ERR_INVALID_ARGS**

### Notes on the clock error bound estimate {#error-bound}

The `zx_clock_get_details()` syscall provides the user with a number of fine
grained details about a clock, including the "error bound estimate". This
value, expressed in nanoseconds, represents the clock maintainer's best current
estimate of how wrong the clock currently might be relative to the reference(s)
being used by the maintainer. For example, if a user fetched a time `X` with an
error bound estimate of `E`, then the clock maintainer is attempting to say that
it believes that the clock's actual value is somewhere in the range `[ X-E, X+E ]`.

The level of confidence in this estimate is _not_ specified by the kernel APIs.
It is possible that some clock maintainers are using a strict bound, while
others are using a bound that is not provable but provides "high confidence",
while others still might have little to no confidence in their estimates.

In the case that a user needs to understand the objective quality of the error
estimates they are accessing (for example, to enforce certificate validity
dates, or DRM license expiration), they should understand which component in the
system is maintaining their clock, and what guarantees that maintainer provides
when it comes to the confidence levels of its published error bound estimates.

## SYSCALLS

 - [clock transformations](/docs/concepts/kernel/clock_transformations.md)
 - [`zx_clock_create()`](/docs/reference/syscalls/clock_create.md) - create a clock
 - [`zx_clock_read()`](/docs/reference/syscalls/clock_read.md) - read the time of the clock
 - [`zx_clock_get_details()`](/docs/reference/syscalls/clock_get_details.md) - fetch the details of a clock's relationship to clock monotonic
 - [`zx_clock_update()`](/docs/reference/syscalls/clock_update.md) - adjust the current relationship of a clock to the clock monotonic reference.
