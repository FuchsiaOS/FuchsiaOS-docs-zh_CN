<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //docs/reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_smc_call

## Summary

Make Secure Monitor Call (SMC) from user space.

## Declaration

```c
#include <zircon/syscalls.h>
#include <zircon/syscalls/smc.h>

zx_status_t zx_smc_call(zx_handle_t handle,
                        const zx_smc_parameters_t* parameters,
                        zx_smc_result_t* out_smc_result);
```

## Description

`zx_smc_call()` makes a Secure Monitor Call (SMC) from user space. It supports the ARM SMC Calling
Convention using the `zx_smc_parameters_t` input parameter and `zx_smc_result_t` output parameter.
The input *handle* must be a resource object with sufficient privileges in order to be executed.

The majority of the parameters are opaque from `zx_smc_call()` perspective because they are
dependent upon the *func_id*. The *func_id* informs the Secure Monitor the service and function
to be invoked. The *client_id* is an optional field intended for secure software to track and
index the calling client OS. The *secure_os_id* is an optional field intended for use when there
are multiple secure operating systems at S-EL1, so that the caller may specify the intended
secure OS.

More information is available in the [ARM SMC Calling Convention documentation](
http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.den0028b/index.html).

## Rights

TODO(fxbug.dev/32253)

## Return value

`zx_smc_call()` returns **ZX_OK** if *handle* has sufficient privilege. The
return value of the smc call is returned via **out_smc_result** on success. In the event of
failure, a negative error value is returned.

## Errors

**ZX_ERR_BAD_HANDLE**  *handle* is not a valid handle.

**ZX_ERR_WRONG_TYPE**  *handle* is not a resource handle.

**ZX_ERR_ACCESS_DENIED**  *handle* does not have sufficient privileges.

**ZX_ERR_NOT_SUPPORTED**  smc_call is not supported on this system.

**ZX_ERR_INVALID_ARGS**  *parameters* or *out_smc_result* a null pointer
