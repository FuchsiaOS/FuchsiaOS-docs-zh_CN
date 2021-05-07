# zx_job_set_policy

## NAME

<!-- Updated by update-docs-from-fidl, do not edit. -->

Set job security and resource policies.

## SYNOPSIS

<!-- Updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_job_set_policy(zx_handle_t handle,
                              uint32_t options,
                              uint32_t topic,
                              const void* policy,
                              uint32_t policy_size);
```

## DESCRIPTION

Sets one or more security and/or resource policies to an empty job. The job's
effective policies is the combination of the parent's effective policies and
the policies specified in *policy*. The effect in the case of conflict between
the existing policies and the new policies is controlled by *options* values:

+ **ZX_JOB_POL_RELATIVE** : policy is applied for the conditions not specifically
  overridden by the parent policy.
+ **ZX_JOB_POL_ABSOLUTE** : policy is applied for all conditions in *policy* or
  the syscall fails.

After this call succeeds any new child process or child job will have the new
effective policy applied to it.

*topic* indicates the *policy* format. Supported values are **ZX_JOB_POL_BASIC_V1**,
**ZX_JOB_POL_BASIC_V2** and **ZX_JOB_POL_TIMER_SLACK**.

### **ZX_JOB_POL_BASIC_V2 and V1**

A *topic* of **ZX_JOB_POL_BASIC_V2** indicates that *policy* is an array of *count*
entries of:

```
typedef struct zx_policy_basic {
    uint32_t condition;
    uint32_t action;
    uint32_t flags;
} zx_policy_basic_v2_t;

```

A *topic* of **ZX_JOB_POL_BASIC_V1** indicates that *policy* is an array of *count*
entries of:

```
// Deprecated. Use zx_policy_basic_v2_t.
typedef struct zx_policy_basic {
    uint32_t condition;
    uint32_t policy;
} zx_policy_basic_v1_t;

```

Where *condition* is one of

+ **ZX_POL_BAD_HANDLE** a process under this job is attempting to
  issue a syscall with an invalid handle.  In this case,
  **ZX_POL_ACTION_ALLOW** and **ZX_POL_ACTION_DENY** are equivalent:
  if the syscall returns, it will always return the error
  **ZX_ERR_BAD_HANDLE**.
+ **ZX_POL_WRONG_OBJECT** a process under this job is attempting to
  issue a syscall with a handle that does not support such operation.
+ **ZX_POL_VMAR_WX** a process under this job is attempting to map an
  address region with write-execute access.
+ **ZX_POL_NEW_VMO** a process under this job is attempting to create
  a new vm object.
+ **ZX_POL_NEW_CHANNEL** a process under this job is attempting to create
  a new channel.
+ **ZX_POL_NEW_EVENT** a process under this job is attempting to create
  a new event.
+ **ZX_POL_NEW_EVENTPAIR** a process under this job is attempting to create
  a new event pair.
+ **ZX_POL_NEW_PORT** a process under this job is attempting to create
  a new port.
+ **ZX_POL_NEW_SOCKET** a process under this job is attempting to create
  a new socket.
+ **ZX_POL_NEW_FIFO** a process under this job is attempting to create
  a new fifo.
+ **ZX_POL_NEW_TIMER** a process under this job is attempting to create
  a new timer.
+ **ZX_POL_NEW_PROCESS** a process under this job is attempting to create
  a new process.
+ **ZX_POL_NEW_PROFILE** a process under this job is attempting to create
  a new profile.
+ **ZX_POL_AMBIENT_MARK_VMO_EXEC** a process under this job is attempting
  to use [`zx_vmo_replace_as_executable()`] with a **ZX_HANDLE_INVALID**
  as the second argument rather than a valid **ZX_RSRC_KIND_VMEX**.
+ **ZX_POL_NEW_ANY** is a special *condition* that stands for all of
  the above **ZX_NEW** conditions such as **ZX_POL_NEW_VMO**,
  **ZX_POL_NEW_CHANNEL**, **ZX_POL_NEW_EVENT**, **ZX_POL_NEW_EVENTPAIR**,
  **ZX_POL_NEW_PORT**, **ZX_POL_NEW_SOCKET**, **ZX_POL_NEW_FIFO**,
  and any future **ZX_NEW** policy. This will include any new
  kernel objects that do not require a parent object for creation.

Where *policy* for **ZX_JOB_POL_BASIC_V1** or *action* for **ZX_JOB_POL_BASIC_V2**
is one of

+ **ZX_POL_ACTION_ALLOW**  allow *condition*.
+ **ZX_POL_ACTION_DENY**  prevent *condition*.
+ **ZX_POL_ACTION_ALLOW_EXCEPTION**  generate an exception via the debug port.
  An exception generated this way acts as a breakpoint. The thread may be
  resumed after the exception. Once resumed, the *condition* triggering the
  exception will be allowed to complete as if no policy violation occurred.
+ **ZX_POL_ACTION_DENY_EXCEPTION**  just like **ZX_POL_ACTION_ALLOW_EXCEPTION**,
  but after resuming, the *condition* will be denied, usually resulting in
  **ZX_ERR_ACCESS_DENIED**.
+ **ZX_POL_ACTION_KILL**  terminate the process.

Where *flags* is one of

+ **ZX_POL_OVERRIDE_ALLOW** Allow to change this policy on child Jobs.
+ **ZX_POL_OVERRIDE_DENY** Don't allow to change this policy on child jobs.

Regardless of the override mode, as long a Job has any children its policy cannot
be mutated.

### **ZX_JOB_POL_TIMER_SLACK**

A *topic* of **ZX_JOB_POL_TIMER_SLACK** indicates that *policy* is:

```
typedef struct zx_policy_timer_slack {
    zx_duration_t min_slack;
    uint32_t default_mode;
} zx_policy_timer_slack_t;

```

*min_slack* specifies the minimum amount of slack applied to timers and
deadline-based events created by the job.

If the parent job's *min_slack* is greater than the specified *min_slack* then
the parent job's value is used instead. In other words, a job's *min_slack* is
the maximum of the specified value and its parent job's *min_slack*.

*default_mode* specifies how slack will be applied when not otherwise indicated
by the syscall arguments. A job's *default_mode* may be set regardless of its
parent job's *default_mode*. The possible values for *default_mode* are:

+ **ZX_TIMER_SLACK_CENTER**
+ **ZX_TIMER_SLACK_EARLY**
+ **ZX_TIMER_SLACK_LATE**

See [timer slack](/docs/concepts/kernel/timer_slack.md) for more information.

When setting timer slack policy, *options* must be **ZX_JOB_POL_RELATIVE** and
**count** must be 1.

## RIGHTS

<!-- Updated by update-docs-from-fidl, do not edit. -->

*handle* must be of type **ZX_OBJ_TYPE_JOB** and have **ZX_RIGHT_SET_POLICY**.

## RETURN VALUE

`zx_job_set_policy()` returns **ZX_OK** on success.  In the event of failure,
a negative error value is returned.

## NOTES

The **ZX_POL_BAD_HANDLE** policy never applies when calling [`zx_object_get_info()`]
with the topic **ZX_INFO_HANDLE_VALID**.  All other topics and all other syscalls that
take handles are subject to the policy if active.

## ERRORS

**ZX_ERR_INVALID_ARGS**  *policy* was not a valid pointer, or *count* was 0,
or *policy* was not **ZX_JOB_POL_RELATIVE** or **ZX_JOB_POL_ABSOLUTE**, or
*topic* was not **ZX_JOB_POL_BASIC**.

**ZX_ERR_BAD_HANDLE**  *handle* is not valid handle.

**ZX_ERR_WRONG_TYPE**  *handle* is not a job handle.

**ZX_ERR_ACCESS_DENIED**  *handle* does not have **ZX_POL_RIGHT_SET** right.

**ZX_ERR_BAD_STATE**  the job has existing jobs or processes alive.

**ZX_ERR_OUT_OF_RANGE** *count* is bigger than **ZX_POL_MAX** or *condition* is
bigger than **ZX_POL_MAX**.

**ZX_ERR_ALREADY_EXISTS** existing policy conflicts with the new policy.

**ZX_ERR_NOT_SUPPORTED** an entry in *policy* has an invalid value.

**ZX_ERR_NO_MEMORY**  Failure due to lack of memory.
There is no good way for userspace to handle this (unlikely) error.
In a future build this error will no longer occur.

## SEE ALSO

 - [`zx_job_create()`]
 - [`zx_object_get_info()`]
 - [`zx_process_create()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_job_create()`]: job_create.md
[`zx_object_get_info()`]: object_get_info.md
[`zx_process_create()`]: process_create.md
[`zx_vmo_replace_as_executable()`]: vmo_replace_as_executable.md
