# Timer Slack

[Timer objects](/docs/reference/kernel_objects/timer.md) have a concept of slack. Slack
defines how the system may alter the timer's deadline. Slack allows
the system to internally coalesce timers and timer-like events to
improve performance or efficiency.

Slack is made up of two components, type and amount. Type describes
how slack can be applied:

+ **ZX_TIMER_SLACK_CENTER** coalescing is allowed with earlier and
  later timers.
+ **ZX_TIMER_SLACK_EARLY** coalescing is allowed only with earlier
  timers.
+ **ZX_TIMER_SLACK_LATE** coalescing is allowed only with later
  timers.

Amount is the allowed deviation from the deadline. For example, a
timer with **ZX_TIMER_SLACK_EARLY** and 5us may fire up to 5us before
its deadline. A timer with **ZX_TIMER_SLACK_CENTER** and 7ms may fire
anywhere from 7ms before its deadline to 7ms after its deadline.

## Timer-like Syscalls

Slack may also be applied to blocking syscalls that accept a deadline
argument, like [`zx_nanosleep()`].

## Defaults and Job Policy

For Timer objects, slack is specified when creating and setting
timers. For other syscalls that take a deadline, but no slack
parameters, the slack type and amount are specified by the job's
policy. See [`zx_job_set_policy()`].

[`zx_job_set_policy()`]: /docs/reference/syscalls/job_set_policy.md
[`zx_nanosleep()`]: /docs/reference/syscalls/nanosleep.md
