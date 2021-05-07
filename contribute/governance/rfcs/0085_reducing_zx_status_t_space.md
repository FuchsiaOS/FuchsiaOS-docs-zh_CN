{% set rfcid = "RFC-0085" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

## Summary

This document proposes reducing the range of valid `zx_status_t` values
from all 32-bit signed integers to the smaller range `[-2^30, 0]`, and
deprecating the use of application-defined error codes. This allows
`zx_status_t` to be easily embedded as a subrange of other types.

## Motivation

`zx_status_t` is a simple error type used to communicate whether
a particular action was successful or not. It is defined as being
a signed 32-bit integer. The value `ZX_OK` (0) indicates an operation
was successful. All other values indicate an error of some form,
with both system-defined error codes (negative values) and
application-defined error codes (positive values) being supported.

The `zx_status_t` type is used widely throughout Fuchsia. For example,
it is used in the Zircon kernel (both internally and as the return
value for syscalls); used in many FIDL protocols to indicate errors;
used in several system libraries to report errors; and frequently used
internally in pure application code as a convenient method of
reporting errors between functions.

While the current definition of `zx_status_t` allows various different
error conditions to be communicated, it is currently only able to
communicate a single success value. Over time, Fuchsia developers
have discovered several use-cases where it would be useful to
communicate other non-error information back to the caller:

* **Non-error warnings**: Some functions want to emit warnings, where
  a function was mostly successful, but with a potential problem. For
  example, a request to write to a buffer succeeded, but the buffer
  was smaller than the amount of data available.

* **Additional information about the state of an object**: For
  example, kernel IPC primitives may wish to indicate that more data
  is pending after a read completes, or provide more fine grained
  threshold information on types like sockets.

* **Flow control**: A common pattern in the kernel and system libraries
  is for callbacks to return either an error, the value `ZX_ERR_STOP` to
  indicate the callback should no longer be called, or `ZX_ERR_NEXT` to
  indicate the callback should continue to be called. These special
  values, despite not being errors _per se_, are currently assigned
  codes in the system-defined error space.

* **Communicating small payloads**: Functions may wish to
  communicate small amounts of data as part of their result when the
  call is successful. For example, the current implementation of the
  `zx_debuglog_read` syscall naughtily uses the positive values of
  `zx_status_t` to return the number of bytes read into a buffer on
  success.

While all of these use-cases could theoretically be solved by using
additional out parameters or more complex compound types, in certain
performance-sensitive use cases, it would be more efficient if functions
could have a simple integer/register return value that could handle the
use cases above.

While in this document, we use the terms _function_ and _return code_,
the ideas apply similarly to FIDL calls, syscalls, and so on. Similarly,
while this design gives examples in C++, the same ideas apply to other
languages including C, Rust, and Go.

## Design

In this proposal, `zx_status_t` remains a signed 32-bit integer, and
will continue to be defined as having a single success code `ZX_OK` (0).
All other values should continue to be treated as errors.

However, we update the definition of `zx_status_t` so that:

*  The range of valid `zx_status_t` values is `[-2^30, 0]` (that
   is, `-1073741824` to `0`). All values in this range will be
   system-defined error codes or the single success code `ZX_OK`.

*  Application-defined error codes (currently defined as all positive
   `zx_status_t` values) are deprecated, as described below in
   "Backwards Compatibility".

By limiting the range of values a `zx_status_t` may take on, users are
able to embed the range of `zx_status_t` values in another type without
fear that error code values will overlap non-error return values. For
example, a function may define a type `result_or_count_t` where negative
values are defined to correspond to the `zx_status_t` error codes, while
non-negative values correspond to a count of elements processed.

We require function implementers to define _new_ types, and not just
emit values in the unused parts of the `zx_status_t` space. This ensures
that users of the function are clear what the function returns, and how
it should be interpreted: if a function has the return type
`zx_status_t`, users are guaranteed that `ZX_OK` is the only valid
success value.

### Examples

The following sections demonstrate how the different use cases above can
be handled by assuming a limited range of `zx_status_t`.

#### Additional status information

Today, users of `zx_channel_read` can only determine there are no
messages waiting by performing a failing read on a channel. Using this
proposal, `zx_channel_read` could introduce a new return type that
provides this "more messages are waiting" state as part of the return value,
avoiding the extra syscall:

```c
/// Keep reading messages until none remain on the channel.
do {
  // Read from the channel.
  zx_channel_read_result_t result =
      zx_channel_read(channel, buffer, /*options=*/ZX_GET_CHANNEL_STATE);

  // `zx_channel_read_result_t` defines negative values to correspond
  // to `zx_status_t` error codes.
  if (result < 0) {
    return static_cast<zx_status_t>(result);
  }

  // Otherwise, the result is defined to be a bitmap indicating
  // the state of the channel.
} while ((result & ZX_CHANNEL_MORE_MESSAGES_WAITING) != 0);
```

#### Flow control

Instead of relying on the error codes `ZX_ERR_NEXT` and `ZX_ERR_STOP`
(and relying on documentation to inform callers what codes need to be
handled), a new type can be introduced that uses the non-negative space
to indicate flow control:

```c
// Negative values are zx_status_t error codes, while non-negative
// values must be one of the constants below.
using zx_iteration_status_t = int32_t;
constexpr int32_t ZX_ITERATION_CONTINUE = 0;
constexpr int32_t ZX_ITERATION_DONE = 1;

// ...

while (true) {
  zx_iteration_status_t result = Next(thing);
  if (result < 0) {
    return result;  // error
  }
  if (result == ZX_ITERATION_DONE) {
    break;
  }
  // ...
}
```

#### Mixing a payload into the response

`zx_debuglog_read` already uses the non-negative space to return a small
payload (the number of bytes read), which currently violates the strict
definition of `zx_status_t`. This proposal would allow
`zx_debuglog_read` to define a new type that makes explicit how return
values should be interpreted:

```c
// Read the debug log. Returns a negative value on error, otherwise
// the number of bytes read from the debug log.
zx_debuglog_read_result_t result = zx_debuglog_read(buffer);
if (result < 0) {
  return result;  // error
}
print_log(/*buffer=*/buffer, /*size=*/result);
```

#### Application-defined error codes

Applications that wish to define their own error codes can continue to
do so, but should define a type making it clear how the values should
be interpreted:

```c
enum ApplicationError {
  INVALID_AUTHORIZATION = 1,
  TOO_MANY_OUTSTANDING_REQUESTS = 2,
  // ...
}

// Zero indicates success. Negative values map to `zx_status_t` error
// codes. Positive values map to `ApplicationError` error codes.
using app_status_t = int32_t;
```

Because `zx_status_t` only occupies the range `[-2^30, 0]`, applications
have the ability to use the range `[-2^31, -2^30)` for
application-defined error codes if they so wish, freeing the positive
space for other return codes if so desired.

## Backwards compatibility

This proposal updates the valid range of `zx_status_t` to the values
`[-2^30, 0]`, all of which are system-defined. There are, however,
a small set of applications currently using the positive space for
application-specific codes.

As part of the implementation in this RFC, we will migrate in-tree users of
positive status code to a new (non-`zx_status_t`) type.

## Implementation

Steps required to implement this RFC are as follows:

* Update documentation (both Markdown docs and source comments) describing the
  semantics `zx_status_t` to match what is proposed in this spec.

* Update the `zx_debuglog_read` syscall to use a custom
  (non-`zx_status_t`) type.

* Update current users of the positive `zx_status_t` status range to use
  a new type that better describes what other errors are being produced.

## Performance

First are the performance mechanisms of this encoding scheme _per
se_. For most cases, the performance change should be small and
marginal compared to other schemes for encoding this information. For
system calls, which have either expensive or scarce parameters, there
may be a mild positive performance improvement.

Second are the performance changes of system APIs changing to use the
success-with-more-information scheme. Part of the motivation of this
change is to communicate more information to userspace to allow it to
make more sophisticated decisions, allowing for greater performance.

Overall, we expect the performance gains unlocked by the capability for
system calls to communicate additional information will outweigh
slightly more complex code evaluating status values.

Besides performance, this change and future changes using this
capability may change code size in a number of binaries, especially in
generated FIDL bindings.

## Security considerations

Embedding the `zx_status_t` range into other types has the potential to
cause confusion, and hence risks introducing software bugs. Functions or
protocols that perform such an embedding should carefully evaluate if
the benefits outweigh the risk of confusion.

Migrating functions with out-of-spec uses of `zx_status_t` and
application-specific error codes to have more explicit types should
reduce the chance of confusion.

## Privacy considerations

This proposal does not interact with user data in a meaningful way and
there should be no impact on privacy.

## Testing

Unit tests will be developed for the few new functions described above.

## Documentation

In tree markdown documentation and in-code comments will be updated to
reflect the new definition of `zx_status_t`.

## Drawbacks, alternatives, and unknowns

## Drawback: `zx_status_t` values from untrusted sources and FIDL bindings

Currently we have no way to prevent out-of-range `zx_status_t` values from
being transmitted over channels. Applications that receive `zx_status_t` values
from untrusted sources and require them to be in range will need to manually
validate them. Long term, it may be desirable to update the FIDL binding
generators to check for and reject out-of-range `zx_status_t` values, though
this will be work independent of this RFC.

### Alternative: Kernel out parameters

Another design space for the kernel would be to use additional out
arguments. This approach has several drawbacks:

Changing the type of these system calls is a much more invasive
change, and would require a longer migration.

Either all of these system calls would need to have a variant with the
out parameter, or have callers uninterested in the extra information
pass in null. Both are ergonomic downgrades.

Using an out argument to convey a handful of bits of information is
an inefficient use of a scarce and expensive resource, as it would
consume one of a small number of registers (especially on x86_64), or
be a quite expensive `user_copy` through a pointer.

### Alternative: Leaving non-negative values of `zx_status_t` free to use

This proposal proposes limiting the range of valid `zx_status_t` values
to assist with embedding the range in other types, but doesn't permit
applications to use the unused range directly.

A previous proposal split the `zx_status_t` space, reserving negative values
for errors and allowing functions to use non-negative values as desired.
Allowing the reuse of the `zx_status_t` type for function-specific purposes has
the benefit that it is easy for developers to start returning additional
payloads from functions (no additional types need to be created), and also
proved convenient for allowing kernel syscalls to start returning additional
data to certain callers without breaking the ABI of existing callers.

The down side of this is that it becomes unclear what ranges of values
a function may return by looking at their type alone. Additionally, widely-used
idioms such as the following:

```
zx_status status = CallFunction();
if (status != ZX_OK) {
  return status;
}
```

would no longer be guaranteed correct if `CallFunction` uses the positive
space of return codes.

The risk of confusion (and hence bugs) due to `zx_status_t` having many
different possible interpretations led us to reject this alternative.

### Alternative: Partitioning into error and success codes

Previous proposals have suggested partitioning `zx_status_t` into
ranges of error codes and success codes, with each of those ranges
further split into system-defined and application-defined ranges.

This split has a few downsides:

* The utility of system-defined _success_ codes is not clear:
  while error codes are frequently propagated up the call stack, success
  codes will tend to be either handled immediately or simply discarded.
  There is less need for a set of success codes with globally understood
  semantics.

* Limiting positive values to simply success codes prevents other,
  more efficient uses of the value, such as returning a bitfield
  about the current state of an object, or returning small payloads
  such as the number of bytes.

* Re-purposing the positive space for success codes requires migration
  of existing code that is currently using it for application-specific
  error codes.

Allowing the entire non-negative space to be used by each individual
function reduces migration burden and offers developers more
flexibility.

## Prior art and references

* The Linux kernel internally uses the negative range for errors,
  keeping the positive range for function-specific purposes. Most
  syscalls split this single value into a return code and thread-local
  `errno` variable at the syscall boundary.

* The UEFI spec partitions its status space into negative values
  (errors), zero (success), and positive values (warnings). Both the
  error and warning ranges are further split into an "EFI reserved"
  range and an OEM range, using the second-most significant bit.

## Version History

* _2021-03-12_: Removed part of the proposal suggesting a new error
  code indicating that an out-of-range `zx_status_t` value was received.
  Instead, in-tree code will be audited to remove such usages, and FIDL binding
  generators will be updated to avoid out-of-range values from propagating
  across process boundaries.

* _2021-03-09_: Modified the proposal to redefine the range of
  valid `zx_status_t` values to be `[-2^30, 0]`. This reduced range allows
  `zx_status_t` to more easily be embedded as a subrange of other types.

* _2021-02-10_: Modified the proposal to split negative values in
  `zx_status_t` between application errors and system errors, but to
  keep all non-negative values available for applications without
  further interpretation.

* _2020-10-09_: Initial proposal to split the `zx_status_t` error
  space into four partitions: application errors and system errors as
  negative values; while system success codes and application success
  codes would be the positive values. 0 would remain `ZX_OK`.

[status_def]: https://cs.opensource.google/fuchsia/fuchsia/+/main:zircon/system/public/zircon/errors.h;l=8-13;drc=c26ad0985e3c06356a157d1eabc38c2c13c40b96
