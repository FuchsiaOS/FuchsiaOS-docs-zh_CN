# Zircon System Interface Rubric

The Zircon system interface is expressed as the `libzircon.so` vDSO API surface.

Functions that are part of the interface must have names that start with `zx_`
and preprocessor macros must have names that start with `ZX_`.  Types defined as
part of the interface must have names that begin with `zx_` and end with `_t`.

Every function that is part of the interface must be documented with a markdown
file in https://fuchsia.googlesource.com/zircon/+/master/docs/syscalls/ and
linked from https://fuchsia.googlesource.com/zircon/+/master/docs/syscalls.md .

## Function Names

Functions must have names consisting entirely of lowercase letters and
underscores and that conform to the following grammar:

```
zx_<noun>_<verb>{_<direct-object>}
```

For example:

```
zx_handle_close, zx_channel_write, zx_object_signal_peer
```

Typically, the noun is a kernel object type but can be other nouns, such as
`clock` or `ticks` for which there is no corresponding kernel object. Other
functions use more abstract nouns, such as `system` or `status`.

The nouns and verbs must not contain underscores (to avoid confusing the
grammar). The noun and verb should each be single English words but acronyms (or
abbreviations) may be used if there is no suitable word or the word is too long.

The direct object may contain underscores.

Some functions perform composite operations. In such cases, the function may be
named by concatenating the names of the component operations.

Some functions operate on several types of kernel object, in which case the noun
is a more abstract object type. For example, functions with the noun `object`
operate on most kernel objects and functions with the noun `task` operate on
jobs, processes, and threads.

## Types

Use `zx_status_t` to represent success and failure.

Use fixed-size integer types. Functions must not use `short`, `int`, or
`unsigned long` (or similar types). Instead, use types such as `int16_t`,
`int32_t`, and `uint64_t`.

Use `size_t` for buffer lengths, element sizes, and element counts.

Use `void*` for pointers to arbitrary types in the caller's address space. Use
`zx_vaddr_t` / `zx_paddr_t` for addresses that might be in other address spaces.

Use `zx_time_t` for timeouts, which must be expressed as absolute deadlines in
nanoseconds in the `ZX_CLOCK_MONOTONIC` timebase. In scenarios were absolute
deadlines do not make sense (for example, timer slack), use `zx_duration_t` to
represent an amount of time in nanoseconds with no specific timebase.

## Parameters

### Receiver

The vast majority of functions act on a handle, which is a reference to a kernel
object of a type matching the *noun* in the function name. This handle is the
first argument to such functions and is referred to as the receiver.

Use the name `handle` for the receiver.

Object creation functions (eg, `zx_channel_create`, `zx_event_create`) may not
take a handle argument. These functions implicitly operate on the current
process.

### Options Parameter

Often functions include an `options` parameter to allow for flags which affect
the operation, and include room for further flags being added to future
revisions of the API.

Use the type `uint32_t` and the name `options` for the `options` parameter.

When present, an `options` parameter must be the first argument after the
receiver handle or the first argument overall if the function does not have a
receiver.

An `options` parameter is not required for all functions.

Individual option values must be defined as preprocessor macros that cast a
numeric literal to `uint32_t`. The options must be bit flags that can be
combined using the bitwise `|` operator.

### Handles

When a function is given a handle as a parameter, the function must either
always consume the handle or never consume, with the following exceptions:

 * If the function takes an `options` parameter, the function may have a
   non-default option to avoid consuming handles in various error conditions.

 * If the function does not take an `options` parameter, the function may avoid
   consuming handles if/when it returns `ZX_ERR_SHOULD_WAIT`.

### Buffers with Data, Count/Size, and/or Actual

Always accompany arrays or buffers with a count or size (of type `size_t`),
including strings. If the buffer is written by the function, the function must
have an out parameter that returns the count or size of the data written.

For read and write style operations, the pointer(s) to the buffer(s) are
followed by the buffer count(s) or size(s), and if a short read or write is
possible, an out parameter provides the actual count(s) or size(s) on success:

```
zx_status_t zx_socket_write(zx_handle_t handle, uint32_t options,
                            const void* buffer, size_t size, size_t* actual);
```

When there are multiple buffers, the buffers, lengths, and out parameters appear
interleaved in a consistent order. For example, see `zx_channel_read`:

```
zx_status_t zx_channel_read(zx_handle_t handle, uint32_t options,
                            void* bytes, zx_handle_t* handles,
                            uint32_t num_bytes, uint32_t num_handles,
                            uint32_t* actual_bytes, uint32_t* actual_handles);
```

### Outputs

An out parameter is a scalar value written by the function. For example, a
function that returns the number of CPUs by writing to a `uint32_t` has an out
parameter. If the function populates a buffer provided by the client, the buffer
isn’t an out parameter.

Out parameters always come at the end of the parameter list.

An out parameter must not also be an in parameter. For example, if a function
has an out parameter through which it returns the number of bytes written to a
buffer, that parameter must not also be used by the function to receive the
length of the buffer from the caller.

## Return Type

The vast majority of functions have a return type of `zx_status_t`, which is
`ZX_OK` on success and `ZX_ERR_...` on failure.

Do not return other values through `zx_status_t`, for example using the
positive value range. Instead, use an out parameter.

Other return types may be used for functions that cannot fail. For example,
`zx_thread_exit` never fails to exit the thread and has a return type of void.
Similarly, `zx_clock_get` cannot fail to get the current time and has a return
type of `zx_time_t`.

## Function-specific rules

### zx_object_get_property versus zx_object_get_info

There are two similar mechanisms for exposing data about objects:
`zx_object_get_property` and `zx_object_get_info`. Prefer exposing data through
`zx_object_get_property` if (a) the property can be set using
`zx_object_set_property` or (b) the property exist across multiple types of
objects. In other case, consider including the data in the general
`zx_object_get_info` topic for the type of object that has the property.
