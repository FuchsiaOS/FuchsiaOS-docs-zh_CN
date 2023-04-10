<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_clock_create

## Summary

Create a new clock object.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_clock_create(uint64_t options,
                            const void* args,
                            zx_handle_t* out);
```

## Rights

None.

## Description

Creates a new zircon clock object. See [clocks](/reference/kernel_objects/clock.md) for an
overview of clock objects.

### Options

The following options are defined for clock objects:

+ **ZX_CLOCK_OPT_MONOTONIC** : When set, creates a clock object that is
  guaranteed to never run backwards. Monotonic clocks must always move forward.
+ **ZX_CLOCK_OPT_CONTINUOUS** : When set, creates a clock that is guaranteed to
  never jump either forwards or backwards. Continuous clocks may only be
  maintained using frequency adjustments and are, by definition, also monotonic.
  Attempting to create a clock object with the **ZX_CLOCK_OPT_CONTINUOUS** option
  specified, but without the **ZX_CLOCK_OPT_MONOTONIC** option specified is an
  error, which will be signalled with **ZX_ERR_INVALID_ARGS**.
+ **ZX_CLOCK_OPT_AUTO_START** : When set, creates a clock that is started
  automatically for the user. You don't need to call zx_clock_update() to start
  the clock running. Initially, the clock will be a clone of clock monotonic,
  meaning that the internal transformation from clock monotonic to the newly
  created synthetic clock is the identity function. The created clock does not
  have to be created with either the **ZX_CLOCK_OPT_MONOTONIC** or
  **ZX_CLOCK_OPT_CONTINUOUS** flags set, however. Once created, users may still
  update the clock within the limits defined by the monotonic and continuous
  properties specified at create time, the handle rights, and the backstop time
  of the clock.

### Arguments

One additional creation-time argument may be specified when configuring the clock, the backstop
time. See [clocks](/reference/kernel_objects/clock.md) for more details about backstop times.

In order to configure a backstop time, a user must pass a `zx_clock_create_args_v1_t` structure to
the `zx_clock_create` call via the `args` parameter. Additionally, the `options` bits must have
`ZX_CLOCK_ARGS_VERSION(1)` set in them.

For example, a user who wished to create a monotonic clock with a backstop time of 5500 might do
something like the following.

```c
#include <zircon/syscalls.h>
#include <zircon/syscalls/clock.h>

zx_handle_t MakeAClock() {
  zx_clock_create_args_v1_t args;
  zx_handle_t the_clock;
  zx_status_t status;

  args.backstop_time = 5500;
  status = zx_clock_create(ZX_CLOCK_ARGS_VERSION(1) | ZX_CLOCK_OPT_MONOTONIC, &args, &the_clock);
  if (status != ZX_OK) {
    // Log the error
    return ZX_HANDLE_INVALID;
  }

  return the_clock;
}
```

Users do not have to supply an arguments structure. If an explicit backstop is not required, users
may omit the version bits from the options parameter and simply pass nullptr for args.

## Return value

On success, returns **ZX_OK** along with a new clock object via the *out*
handle. Handles to newly created clock objects will have the **ZX_RIGHT_READ**
and **ZX_RIGHT_WRITE** rights assigned to them.

## Errors

 - **ZX_ERR_INVALID_ARGS** : An invalid option flag was specified, a bad args
   structure version or pointer was passed, **ZX_CLOCK_OPT_CONTINUOUS** was
   specified without also specifying **ZX_CLOCK_OPT_MONOTONIC**, or the initial
   backstop time of an automatically started clock is after the current clock
   monotonic time.
 - **ZX_ERR_NO_MEMORY**  Failure due to lack of memory.

## See also

 - [clocks]
 - [`zx_clock_get_details()`]
 - [`zx_clock_read()`]
 - [`zx_clock_update()`]

[clocks]: /reference/kernel_objects/clock.md
[`zx_clock_get_details()`]: clock_get_details.md
[`zx_clock_read()`]: clock_read.md
[`zx_clock_update()`]: clock_update.md
