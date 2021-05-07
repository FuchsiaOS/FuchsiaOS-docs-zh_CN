# Errors

This describes the set of userspace-exposed errors used in Zircon. The first section provides the
canonical names and description of each error code. The second section provides the concrete values.

Within the kernel, errors are typically resulted as variables of type `status_t` and errors are
defined by macros of the form `ZX_ERR_CANONICAL_NAME` e.g. `ZX_ERR_INTERNAL`. All error cases are negative
values and success is represented by a non-negative value.

In userspace the syscall dispatch layer (libzircon) exposes the result values as variables of type
`zx_status_t` that currently use the same spelling and values as the kernel for errors, but which
will transition to using 0 for success and positive values for errors.

See [Kernel internal errors](kernel_internal_errors.md) for a list of kernel-internal values.

## Descriptions

Each category represents a class of errors. The first error code in each category is the generic
code for that category and is used when no more specific code applies. Further error codes (if any)
within a category represent particular types of errors within the class. In general, more specific
error codes are preferred where possible.

Errors are described in terms of an operation, arguments, a subject, and identifiers. An operation
is typically a function or system call. Arguments are typically the parameters to the call. The
subject of an operation is the primary object the operation acts on, typically a handle and
typically passed as the first argument. Identifiers are typically numbers or strings intended to
unambiguously identify a resource used in the operation.

## Categories

### Success
**ZX\_OK**
 Operation succeeded.

### General errors

These indicate that the system hit a general error while attempting the operation.

**INTERNAL**
  The system encountered an otherwise unspecified error while performing the operation.

**NOT\_SUPPORTED**
  The operation is not supported, implemented, or enabled.

**NO\_RESOURCES**
  The system was not able to allocate some resource needed for the operation.

**NO\_MEMORY**
  The system was not able to allocate memory needed for the operation.

### Parameter errors

These indicate that the caller specified a parameter that does not specify a valid operation or that
is invalid for the specified operation.

**INVALID\_ARGUMENT**
  An argument is invalid.

**WRONG\_TYPE**
  The subject of the operation is the wrong type to perform the operation.
  Example: Attempting a message\_read on a thread handle.

**BAD\_SYSCALL**
  The specified syscall number is invalid.

**BAD\_HANDLE**
  A specified handle value does not refer to a handle.

**OUT\_OF\_RANGE**
  An argument is outside the valid range for this operation.
   Note: This is used when the argument may be valid if the system changes state, unlike
    INVALID\_ARGUMENT which is used when the argument will never be valid.

**BUFFER\_TOO\_SMALL**
  A caller provided buffer is too small for this operation.

### Precondition or state errors

These indicate that the operation could not succeed because the preconditions for the operation are
not satisfied or the system is unable to complete the operation in its current state.

**BAD\_STATE**
  The system is unable to perform the operation in its current state.
   Note: FAILED\_PRECONDITION is a reserved alias for this error

**NOT\_FOUND**
  The requested entity was not found.

**TIMED\_OUT**
  The time limit for the operation elapsed before the operation completed.

**ALREADY\_EXISTS**
  An object with the specified identifier already exists.
  Example: Attempting to create a file when a file already exists with that name.

**ALREADY\_BOUND**
  The operation failed because the named entity is already owned or controlled by another entity.
  The operation could succeed later if the current owner releases the entity.

**HANDLE\_CLOSED**
  A handle being waited on was closed.

**REMOTE\_CLOSED**
  The operation failed because the remote end of the subject of the operation was closed.

**UNAVAILABLE**
  The subject of the operation is currently unable to perform the operation.
  Note: This is used when there's no direct way for the caller to observe when the subject will be
  able to perform the operation and should thus retry.

**SHOULD\_WAIT**
  The operation cannot be performed currently but potentially could succeed if the caller waits for
  a prerequisite to be satisfied, for example waiting for a handle to be readable or writable.
  Example: Attempting to read from a channel that has no messages waiting but has an open
  remote will return **SHOULD\_WAIT**. Attempting to read from a channel that has no messages
  waiting and has a closed remote end will return **REMOTE\_CLOSED**.

### Permission errors

**ACCESS\_DENIED**
  The caller did not have permission to perform the specified operation.

### Input/output errors

**IO**
  Otherwise unspecified error occurred during I/O.

**IO\_REFUSED**
  The entity the I/O operation is being performed on rejected the operation.
  Example: an I2C device NAK'ing a transaction or a disk controller rejecting an invalid command.

**IO\_DATA\_INTEGRITY**
  The data in the operation failed an integrity check and is possibly corrupted.
  Example: CRC or Parity error.

**IO\_DATA\_LOSS**
  The data in the operation is currently unavailable and may be permanently lost.
  Example: A disk block is irrecoverably damaged.
