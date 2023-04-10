<!--
Copyright 2022 The Fuchsia Authors. All rights reserved.
Use of this source code is governed by a BSD-style license that can be
found in the LICENSE file.

DO NOT EDIT. Generated from FIDL library zx by zither, a Fuchsia platform tool.

See //docs/reference/syscalls/README.md#documentation-generation for
regeneration instructions.
-->

# zx_object_get_property

## Summary

Ask for various properties of various kernel objects.

## Declaration

```c
#include <zircon/syscalls.h>

zx_status_t zx_object_get_property(zx_handle_t handle,
                                   uint32_t property,
                                   void* value,
                                   size_t value_size);
```

## Description

`zx_object_get_property()` requests the value of a kernel object's property.
Getting a property requires **ZX_RIGHT_GET_PROPERTY** rights on the handle.

The *handle* parameter indicates the target kernel object. Different properties
only work on certain types of kernel objects, as described below.

The *property* parameter indicates which property to get/set. Property values
have the prefix **ZX_PROP_**, and are described below.

The *value* parameter holds the property value, and must be a pointer to a
buffer of *value_size* bytes. Different properties expect different value
types/sizes as described below.

## PROPERTIES

Property values have the prefix **ZX_PROP_**, and are defined in

```
#include <zircon/syscalls/object.h>
```

### ZX_PROP_NAME

*handle* type: **(Most types)**

*value* type: `char[ZX_MAX_NAME_LEN]`

Allowed operations: **get**, **set**

The name of the object, as a NUL-terminated string.

### ZX_PROP_REGISTER_FS and ZX_PROP_REGISTER_GS

*handle* type: **Thread**

*value* type: `uintptr_t`

Allowed operations: **get**, **set**

The value of the x86 FS.BASE or GS.BASE register, respectively. `value` must
be a canonical address.

This is a software substitute for the `rdfsbase`, `wrfsbase` and `rdgsbase`,
`wrgsbase` instruction pairs supported on newer x86-64 CPUs, and should behave
exactly the same as using the CPU instructions directly (except that
attempting to set a noncanonical address as the value just gets an error
return rather than generating a machine exception).  When using a CPU that
supports these instructions (as reported by the `cpuid` instruction), it's
more efficient and simpler to use the machine instructions directly.

Only defined for x86-64.

### ZX_PROP_PROCESS_DEBUG_ADDR

*handle* type: **Process**

*value* type: `uintptr_t`

Allowed operations: **get**, **set**

The value of ld.so's `_dl_debug_addr`. This can be used by debuggers to
interrogate the state of the dynamic loader.

### ZX_PROP_PROCESS_BREAK_ON_LOAD

*handle* type: **Process**

*value* type: `uintptr_t`

Allowed operations: **get**, **set**

Determines whether the dynamic loader will issue a debug trap on every load of a
shared library. If set before the first thread of a process runs, it will also
trigger a debug trap for the initial load.

The dynamic loader sets the expected value of `ZX_PROP_PROCESS_DEBUG_ADDR` before
triggering this debug trap. Exception handlers can use this property to query the
dynamic loader's state.

When the dynamic loader issues the debug trap, it also sets the value of
`ZX_PROP_PROCESS_BREAK_ON_LOAD` to the address of the debug trap, so that
a debugger could compare the value with the address of the exception to
determine whether the debug trap was triggered by the dynamic loader.

Any non-zero value is considered to activate this feature. Setting this property to
zero will disable it. A debugger could also use this property to detect whether
there's already another debugger attached to the same process.

Note: Depending on the architecture, the address reported by the exception might be
different that the one reported by this property. For example, an x64 platform reports
the instruction pointer *after* it executes the instruction.  This means that an x64
platform reports an instruction pointer one byte higher than this property.

### ZX_PROP_PROCESS_VDSO_BASE_ADDRESS

*handle* type: **Process**

*value* type: `uintptr_t`

Allowed operations: **get**

The base address of the vDSO mapping, or zero.

### ZX_PROP_PROCESS_HW_TRACE_CONTEXT_ID

*handle* type: **Process**

*value* type: `uintptr_t`

Allowed operations: **get**

The context ID distinguishes different processes in hardware instruction tracing.
On Intel X86-64 this is the value of register CR3.

To obtain `ZX_PROP_PROCESS_HW_TRACE_CONTEXT_ID`, you must specify
`kernel.enable-debugging-syscalls=true` on the kernel command line. Otherwise,
the function returns **ZX_ERR_NOT_SUPPORTED**.

Currently only defined for X86.

### ZX_PROP_SOCKET_RX_THRESHOLD

*handle* type: **Socket**

*value* type: `size_t`

Allowed operations: **get**, **set**

The size of the read threshold of a socket, in bytes. Setting this will
assert ZX_SOCKET_READ_THRESHOLD if the amount of data that can be read
is greater than or equal to the threshold. Setting this property to zero
will result in the deasserting of ZX_SOCKET_READ_THRESHOLD.

### ZX_PROP_SOCKET_TX_THRESHOLD

*handle* type: **Socket**

*value* type: `size_t`

Allowed operations: **get**, **set**

The size of the write threshold of a socket, in bytes. Setting this will
assert ZX_SOCKET_WRITE_THRESHOLD if the amount of space available for writing
is greater than or equal to the threshold. Setting this property to zero
will result in the deasserting of ZX_SOCKET_WRITE_THRESHOLD. Setting the
write threshold after the peer has closed is an error, and results in a
ZX_ERR_PEER_CLOSED error being returned.

### ZX_PROP_JOB_KILL_ON_OOM

*handle* type: **Job**

*value* type: `size_t`

Allowed operations: **set**

The value of 1 means the Job and its children will be terminated if the
system finds itself in a system-wide low memory situation. Called with 0
(which is the default) opts out the job from being terminated in this
scenario.

### ZX_PROP_EXCEPTION_STATE

*handle* type: **Exception**

*value* type: `uint32_t`

Allowed operations: **get**, **set**

When set to `ZX_EXCEPTION_STATE_HANDLED`, closing the exception handle will
finish exception processing and resume the underlying thread.
When set to `ZX_EXCEPTION_STATE_TRY_NEXT`, closing the exception handle will
continue exception processing by trying the next handler in order.
When set to `ZX_EXCEPTION_STATE_THREAD_EXIT`, closing the exception handle will
cause the thread that generated the exception to exit.

### ZX_PROP_EXCEPTION_STRATEGY

*handle* type: **Exception**

*value* type: `uint32_t`

Allowed operations: **get**, **set**

If `ZX_EXCEPTION_STRATEGY_SECOND_CHANCE` is set, then the debugger gets a 'second
chance' at handling the exception if the process-level handler fails to do so.

This property can only be set when the handle corresponds to a debugger process
exception channel. Attempting to set this property when the exception channel
is any other type will result in ZX_ERR_BAD_STATE.

### ZX_PROP_STREAM_MODE_APPEND

*handle* type: **Stream**

*value* type: `uint8_t`

Allowed operations: **get**, **set**

This property will have a value of `1` when the Stream is in append mode and a
value of `0` the Stream is not in append mode. A stream in append mode will
atomically set the seek offset of the stream to the content size of the stream
prior to writing data in `zx_stream_writev()`.

## Rights

*handle* must have **ZX_RIGHT_GET_PROPERTY**.

If *property* is **ZX_PROP_PROCESS_DEBUG_ADDR**, *handle* must be of type **ZX_OBJ_TYPE_PROCESS**.

If *property* is **ZX_PROP_PROCESS_BREAK_ON_LOAD**, *handle* must be of type **ZX_OBJ_TYPE_PROCESS**.

If *property* is **ZX_PROP_PROCESS_VDSO_BASE_ADDRESS**, *handle* must be of type **ZX_OBJ_TYPE_PROCESS**.

If *property* is **ZX_PROP_SOCKET_RX_THRESHOLD**, *handle* must be of type **ZX_OBJ_TYPE_SOCKET**.

If *property* is **ZX_PROP_SOCKET_TX_THRESHOLD**, *handle* must be of type **ZX_OBJ_TYPE_SOCKET**.

## Return value

`zx_object_get_property()` returns **ZX_OK** on success. In the event of
failure, a negative error value is returned.

## Errors

**ZX_ERR_BAD_HANDLE**: *handle* is not a valid handle

**ZX_ERR_WRONG_TYPE**: *handle* is not an appropriate type for *property*

**ZX_ERR_ACCESS_DENIED**: *handle* does not have the necessary rights for the
operation

**ZX_ERR_INVALID_ARGS**: *value* is an invalid pointer

**ZX_ERR_NO_MEMORY**  Failure due to lack of memory.
There is no good way for userspace to handle this (unlikely) error.
In a future build this error will no longer occur.

**ZX_ERR_BUFFER_TOO_SMALL**: *value_size* is too small for *property*

**ZX_ERR_NOT_SUPPORTED**: *property* does not exist

## See also

 - [`zx_object_set_property()`]

[`zx_object_set_property()`]: object_set_property.md
