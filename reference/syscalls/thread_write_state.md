# zx_thread_write_state

## NAME

<!-- Updated by update-docs-from-fidl, do not edit. -->

Write one aspect of thread state.

## SYNOPSIS

<!-- Updated by update-docs-from-fidl, do not edit. -->

```c
#include <zircon/syscalls.h>

zx_status_t zx_thread_write_state(zx_handle_t handle,
                                  uint32_t kind,
                                  const void* buffer,
                                  size_t buffer_size);
```

## DESCRIPTION

`zx_thread_write_state()` writes one aspect of state of the thread. The thread
state may only be written when the thread is halted for an exception or the
thread is suspended.

The thread state is highly processor specific. See the structures in
zircon/syscalls/debug.h for the contents of the structures on each platform.

## STATES

See [`zx_thread_read_state()`] for the list of available states
and their corresponding values.

### ZX_THREAD_STATE_DEBUG_REGS

#### ARM

ARM has a variable amount of debug breakpoints and watchpoints. For this
architecture, `zx_thread_state_debug_regs_t` is big enough to hold the maximum
amount of breakpoints possible. But in most cases a given CPU implementation
holds a lesser amount, meaning that the upper values beyond the limit are not
used.

The kernel will write all the available registers in the hardware independent of
the given breakpoint/watchpoint count value. This means that all the correct
state must be set for the call.

You can get the current state of the registers by calling
[`zx_thread_read_state()`](thread_read_state.md#zx_thread_state_debug_regs).

#### ARM Debug Hardware Debug Registers

ARM debug registers are highly configurable via their DBGBCR<n> registers.
However, Zircon limits that functionality to _Unlinked Address Matching_ HW
breakpoints. This means that HW breakpoints will only issue exceptions upon
exception on the given address in the corresponding DBGBVR register.

Because of this, all the values within DBGBCR will be ignored except for the E
bit, which is used to determine whether that particular breakpoint is activated
or not. Said in another way, in order to activate a HW breakpoint, all that is
needed is to set the correct address in DBGBVR and write 1 to DBGBCR.

## RIGHTS

<!-- Updated by update-docs-from-fidl, do not edit. -->

*handle* must be of type **ZX_OBJ_TYPE_THREAD** and have **ZX_RIGHT_WRITE**.

## RETURN VALUE

`zx_thread_write_state()` returns **ZX_OK** on success.
In the event of failure, a negative error value is returned.

## ERRORS

**ZX_ERR_BAD_HANDLE**  *handle* is not a valid handle.

**ZX_ERR_WRONG_TYPE**  *handle* is not that of a thread.

**ZX_ERR_ACCESS_DENIED**  *handle* lacks **ZX_RIGHT_WRITE**.

**ZX_ERR_INVALID_ARGS**  *kind* is not valid, *buffer* is an invalid pointer,
*buffer_size* doesn't match the size of the structure expected for *kind* or
the given values to set are not valid.


**ZX_ERR_NO_MEMORY**  Failure due to lack of memory.
There is no good way for userspace to handle this (unlikely) error.
In a future build this error will no longer occur.

**ZX_ERR_BAD_STATE**  The thread is not stopped at a point where state
is available. The thread state may only be read when the thread is stopped due
to an exception.

**ZX_ERR_NOT_SUPPORTED**  *kind* is not supported.
This can happen, for example, when trying to read a register set that
is not supported by the hardware the program is currently running on.

#### ARM

**ZX_ERR_INVALID_ARGS**   If the address provided to a DBGBVR register is not
valid (ie. not addressable from userspace). Also if any value is set for a HW
breakpoint beyond the number provided by the platform (see above for
information about retrieving that number).

## SEE ALSO

 - [`zx_thread_read_state()`]

<!-- References updated by update-docs-from-fidl, do not edit. -->

[`zx_thread_read_state()`]: thread_read_state.md
