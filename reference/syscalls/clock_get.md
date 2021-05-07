# zx_clock_get

## NAME

<!-- Updated by update-docs-from-fidl, do not edit. -->

Acquire the current time.

## STATUS

`zx_clock_adjust` is currently **DEPRECATED**. Do not make use of it in any new
code. See the [ALTERNATIVES](#alternatives) section of this page for the
updated way to fetch the values that used to be accessible via
`zx_clock_get()`.

## SYNOPSIS

<!-- Updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_clock_get(zx_clock_t clock_id, zx_time_t* out);
```

## DESCRIPTION

`zx_clock_get()` returns the current time of *clock_id* via
*out*, and returns whether *clock_id* was valid.

## SUPPORTED CLOCK IDS

**ZX_CLOCK_MONOTONIC** number of nanoseconds since the system was powered on.

**ZX_CLOCK_UTC** number of wall clock nanoseconds since the Unix epoch (midnight on January 1 1970) in UTC

**ZX_CLOCK_THREAD** number of nanoseconds the current thread has been running for.

## RIGHTS

<!-- Updated by update-docs-from-fidl, do not edit. -->

TODO(fxbug.dev/32253)

## RETURN VALUE

On success, `zx_clock_get()` returns **ZX_OK**.

## ERRORS

**ZX_ERR_INVALID_ARGS**  *clock_id* is not a valid clock id, or *out* is an invalid pointer.

## ALTERNATIVES {#alternatives}

`zx_clock_get()` has been **deprecated**. Refer to the instructions below on how
to gain access to the timelines that had been accessible via calls to
`zx_clock_get()`.

1. [`ZX_CLOCK_MONOTONIC`](#alternative-zx-clock-monotonic)
2. [`ZX_CLOCK_UTC`](#alternative-zx-clock-utc)
3. [`ZX_CLOCK_THREAD`](#alternative-zx-clock-thread)

### `ZX_CLOCK_MONOTONIC` {#alternative-zx-clock-monotonic}

`zx_clock_get(ZX_CLOCK_MONOTONIC, out_ptr)` has been replaced with calls to
[`zx_clock_get_monotonic()`](clock_get_monotonic.md). Unlike `zx_clock_get`,
`zx_clock_get_monotonic` is almost always implemented in a way that does not
need an actual syscall (which can provide a performance benefit) and it cannot
ever return an error (which can help to simplify code). For example:

```C
// Where old code used to say something like
zx_time_t now;
zx_status_t status = zx_clock_get(ZX_CLOCK_MONOTONIC, &now);
if (status != ZX_OK) {
  // Handle failure here
}

// It can now simply say the following instead
zx_time_t now = zx_clock_get_monotonic();
```

### `ZX_CLOCK_UTC` {#alternative-zx-clock-utc}

Supplying a clock that provides an accurate representation of the UTC timeline
is no longer a feature provided directly by the kernel. Instead, it
has become the responsibility of the Component Manager to distribute a handle to
a [kernel clock object](../kernel_objects/clock.md) to processes that
are permitted to access UTC. This clock handle (when available) is used by the
various runtimes to implement their version of access to UTC. In addition, some
runtimes may provide direct access to this clock handle allowing the user to get
even more detailed information about the clock.

User who wish to access UTC in their programs should make use of their runtime's
standard UTC APIs, or borrow a handle to the underlying clock object if they
need access to the native clock handle.

Here are a few examples of some of the ways that a user in a C runtime can gain
access to UTC.

```C
#include <inttypes.h>
#include <zircon/time.h>

// Fetching the clock using gettimeofday
#include <sys/time.h>
{
  struct timeval_t now_utc;
  int status = gettimeofday(&now_utc, NULL);
  if (status == 0) {
    zx_time_t nsec = ((zx_time_t)now_utc.tv_sec * ZX_SEC(1)) +
                     ((zx_time_t)now_utc.tv_usec * ZX_USEC(1))
    printf("It has been " PRId64 " nSec since the epoch.\n", nsec);
  } else {
    printf("gettimeofday(...) call failed (errno = %d)\n", errno);
  }
}

// Fetching the clock using clock_gettime
#include <time.h>
{
  struct timespec now_utc;
  int status = clock_gettime(CLOCK_REALTIME, &now_utc, NULL);
  if (status == 0) {
    zx_time_t nsec = ((zx_time_t)now_utc.tv_sec * ZX_SEC(1)) +
                     ((zx_time_t)now_utc.tv_nsec * ZX_NSEC(1))
    printf("It has been " PRId64 " nSec since the epoch.\n", nsec);
  } else {
    printf("clock_gettime(...) call failed (errno = %d)\n", errno);
  }
}

// Gaining direct access to the process's UTC reference clock object.
#include <zircon/clock.h>
#include <zircon/utc.h>
{
  // This is a borrowed handle. Do not close it, and do not replace it using
  // zx_utc_reference_swap while using it.
  zx_handle_t utc_clock = zx_utc_reference_get();

  if (utc_clock != ZX_HANDLE_INVALID) {
    zx_time_t nsec;
    zx_status_t status = zx_clock_read(utc_clock, &nsec);
    if (status == ZX_OK) {
      printf("It has been " PRId64 " nSec since the epoch.\n", nsec);
    } else {
      printf("zx_clock_read(...) syscall failed (status = %d)\n", status);
    }
  } else {
    printf("Error, our runtime has no clock assigned to it!");
  }
}
```

### `ZX_CLOCK_THREAD` {#alternative-zx-clock-thread}

The `ZX_CLOCK_THREAD` clock was never really a clock at all. Instead it
provided access to value that was a property of a thread object, specifically
the cumulative runtime of the calling thread.

Moving forward, this value can be accessed for any thread using
[`zx_object_get_info()`](object_get_info.md) using the topic
[`ZX_INFO_THREAD_STATS`](object_get_info.md#zx-info-thread-stats).
Users will need to have access to the handle thread they wish to query, and that
handle will need to have `ZX_RIGHT_INSPECT` rights set on it. This is generally
the case for threads in a process created by the user.

Here is an example of how to query this value for the current thread running in
the C/C++ runtime. The specific API for obtaining access to the current
thread handle, or for making a syscall, will vary from language runtime to
language runtime.

```C
#include <inttypes.h>
#include <threads.h>
#include <zircon/syscalls.h>
#include <zircon/syscalls/object.h>
#include <zircon/threads.h>

void test() {
  zx_info_thread_stats_t info;
  zx_status_t status;

  status = zx_object_get_info(thrd_get_zx_handle(thrd_current()),
                              ZX_INFO_THREAD_STATS,
                              &info, sizeof(info),
                              NULL, NULL);

  if (status != ZX_OK) {
    printf("Current thread has a runtime of " PRId64 "\n", info.total_runtime);
  } else {
    printf("Failed to fetch current thread runtime (status %d)\n", status);
  }
}

```

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_clock_get_monotonic()`]: clock_get_monotonic.md
[`zx_object_get_info()`]: object_get_info.md
