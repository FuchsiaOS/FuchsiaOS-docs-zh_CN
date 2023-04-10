<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //docs/reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_profile_create

## Summary

Create a scheduler profile.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_profile_create(zx_handle_t root_job,
                              uint32_t options,
                              const zx_profile_info_t* profile,
                              zx_handle_t* out);
```

## Description

`zx_profile_create()` creates a new [profile](/docs/reference/kernel_objects/profile.md) object.

The parameter *profile* specifies the settings in the profile, which in turn
will be applied to threads when [`zx_object_set_profile()`] is called. The
fields of *profile* are shown below. *options* must be zero.

```c
#define ZX_PROFILE_INFO_FLAG_PRIORITY (1 << 0)
#define ZX_PROFILE_INFO_FLAG_CPU_MASK (1 << 1)
#define ZX_PROFILE_INFO_FLAG_DEADLINE (1 << 2)
#define ZX_PROFILE_INFO_FLAG_NO_INHERIT (1 << 3)

typedef struct zx_profile_info {
  // A bitmask of ZX_PROFILE_INFO_FLAG_* values. Controls overall profile
  // options as well as determining which other fields below have been specified.
  // Other fields are considered unset.
  uint32_t flags;

  uint8_t padding1[4];

  union {
    struct {
      // Scheduling priority. |flags| must have ZX_PROFILE_INFO_FLAG_PRIORITY set.
      int32_t priority;
      uint8_t padding2[20];
    };

    // Scheduling deadline. |flags| must have ZX_PROFILE_INFO_FLAG_DEADLINE set.
    zx_sched_deadline_params_t deadline_params;
  };

  // CPUs that threads may be scheduled on. |flags| must have
  // ZX_PROFILE_INFO_FLAG_CPU_MASK set.
  zx_cpu_set_t cpu_affinity_mask;
} zx_profile_info_t;
```

The `flags` field controls overall profile options in addition to specifying
which fields in the `zx_profile_info_t` structure contain valid values. Values
in fields without a corresponding `flag` bit set will be ignored. This allows
fields with values of `0` and unset fields to be distinguished, even if
additional fields are added in future.

`ZX_PROFILE_INFO_FLAG_PRIORITY` and `ZX_PROFILE_INFO_FLAG_DEADLINE` specify the
`scheduling discipline` for the profile, either "fair scheduling" (for
`PRIORITY`) or "deadline scheduling" (for `DEADLINE`.  No more than one of these
disciplines may be selected at the same time.

By default, profiles participate in profile inheritance when assigned to a
thread which blocks in a futex with an assigned owner (see `zx_futex_wait()`).
Profiles created with the `ZX_PROFILE_INFO_FLAG_NO_INHERIT` do not.  When
threads assigned with such a profile block in a futex with an owner, the
owner of the futex receives no direct profile pressure from the blocking
thread (however, they may receive pressure from a different thread
blocked behind the blocking thread).

Deadline profiles must always be inheritable.  Attempts to create a
non-inheritable deadline profile will fail with ZX_ERR_INVALID_ARGS.


Upon success a handle for the new profile is returned.

## Rights

*root_job* must be of type **ZX_OBJ_TYPE_JOB** and have **ZX_RIGHT_MANAGE_PROCESS**.

Caller job policy must allow **ZX_POL_NEW_PROFILE**.

## Return value

Returns **ZX_OK** and a handle to the new profile (via *out*) on success. In the
event of failure, a negative error value is returned.

## Errors

**ZX_ERR_BAD_HANDLE**  *root_job* is not a valid handle.

**ZX_ERR_WRONG_TYPE**  *root_job* is not a job handle.

**ZX_ERR_ACCESS_DENIED**  *root_job* does not have **ZX_RIGHT_MANAGE_PROCESS**
right, or *root_job* is not a handle to the root job.

**ZX_ERR_INVALID_ARGS**  One or more of the arguments provided were invalid:

  * *profile* or *out* was an invalid pointer
  * *flags* contains an unknown option
  * *flags* either failed to specify a scheduling discipline, or a cpu
            affinity mask.
  * *flags* specified more than one scheduling discipline at once.
  * *flags* specified the scheduling discipline as deadline, but the profile
            was also marked as "no inherit".
  * *options* was not zero
  * *priority* was an invalid priority

**ZX_ERR_NO_MEMORY**  Failure due to lack of memory.

## See also

 - [`zx_object_set_profile()`]
 - [`zx_futex_wait()()`]

[`zx_object_set_profile()`]: object_set_profile.md
[`zx_futex_wait()`]: futex_wait.md
