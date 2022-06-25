# [DEPRECATED] C language FIDL tutorial

Note: The C language bindings are deprecated in favor of the
[LLCPP bindings][llcpp-tutorial]. You should use LLCPP, possibly with C shims,
instead of the C bindings.
<!-- TODO(fxbug.dev/52464) link to LLCPP in C guide when written -->

## About this tutorial

This tutorial describes how to make client calls and write servers in C
using the FIDL InterProcess Communication (**IPC**) system in Fuchsia.

Refer to the [main FIDL page](../README.md) for details on the
design and implementation of FIDL, as well as the
[instructions for getting and building Fuchsia](/docs/get-started/README.md).

The [reference](#reference) section documents the bindings.

## Getting started

We'll use the
[`//examples/fidl/fuchsia.examples/echo.test.fidl`][echo-fidl] FIDL library
discussed in [Creating a FIDL library][fidl-intro].

```fidl
library fuchsia.examples;

{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples/echo.test.fidl" region_tag="max" adjust_indentation="auto" %}

{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples/echo.test.fidl" region_tag="echo" adjust_indentation="auto" %}
```

## `Echo` server

The example server code is in [//examples/fidl/c/server/echo_server.c][server]:

```c
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/c/server/echo_server.c" region_tag="main" adjust_indentation="auto" %}
```

### main()

**main()**:

1. creates a startup handle
2. initializes the asynchronous loop
3. adds the **connect()** function to handle the echo service
4. runs the asynchronous loop in the foreground via
   **async_loop_run()**.

When the async loop returns, we clean up and exit.

# Reference

This section describes the FIDL implementation for C, including the
libraries and code generator.

Consult the [C Family Comparison][c-family-comparison] document for an overview
of the similarities and differences between the C and C++ bindings.

## Design

### Goals

 * Support encoding and decoding FIDL objects with C11.
 * Generate headers that are compatible with C11 and C++14.
 * Small, fast, efficient.
 * Depend only on a small subset of the standard library.
 * Minimize code expansion through table-driven encoding and decoding.
 * Support two usage styles: raw and simple.

### Raw Usage Style

 * Optimized to meet the needs of low-level systems programming.
 * Represent data structures whose memory layout coincides with the wire format.
 * Support in-place access and construction of FIDL objects.
 * Defer all memory allocation decisions to the client.
 * Code generator only produces type declarations, data tables, and simple inline functions.
 * Client is fully responsible for dispatching incoming method calls on
   protocols (write their own switch statement and invoke argument decode
   functions).

### Simple Usage Style

 * Optimized to meet the needs of driver developers.
 * Supports only a simple subset of the FIDL language.
 * Represent data structures whose memory layout coincides with the wire format.
 * Defer all memory allocation decisions to the client.
 * Code generator produces simple C functions for sending, receiving, and
   dispatching messages.

### Encoding Tables

To avoid generating any non-inline code whatsoever, the C language bindings
instead produce encoding tables that describe how objects are encoded.

### Introspection Tables

To allow for objects to be introspected (eg. printed), the C language bindings
produce introspection tables that describe the name and type signature of each
method of each protocol and data structure.

Although small, introspection tables will be stripped out by the linker if
unused.

### Mapping FIDL Types to C Types

This is the mapping from FIDL types to C types that the code generator
produces.

| FIDL                                           | C Type                     |
|------------------------------------------------|----------------------------|
| `bits`                                         | typedef to underlying type |
| `bool`                                         | `bool`                     |
| `int8`                                         | `int8_t`                   |
| `uint8`                                        | `uint8_t`                  |
| `int16`                                        | `int16_t`                  |
| `uint16`                                       | `uint16_t`                 |
| `int32`                                        | `int32_t`                  |
| `uint32`                                       | `uint32_t`                 |
| `int64`                                        | `int64_t`                  |
| `uint64`                                       | `uint64_t`                 |
| `float32`                                      | `float`                    |
| `float64`                                      | `double`                   |
| `handle`, `handle:optional`                    | `zx_handle_t`              |
| `handle<T>`, `handle<T>:optional`              | `zx_handle_t`              |
| `string`, `string:optional`                    | `fidl_string_t`            |
| `string:<N,optional>`                          | `fidl_string_t`            |
| `vector<T>`, `vector<T>:optional`              | `fidl_vector_t`            |
| `vector<T>:<N,optional>`                       | `fidl_vector_t`            |
| `array<T, N>`                                  | `T[N]`                     |
| `client_end:P`, `client_end:<P,optional>`      | typedef to `zx_handle_t`   |
| `client_end:P`, `server_end:<P,optional>`      | typedef to `zx_handle_t`   |
| `struct`                                       | `struct Struct`            |
| `box<struct>`                                  | `struct Struct*`           |
| `union`                                        | `struct Union`             |
| `union:optional`                               | `struct Union*`            |
| `table`                                        | (not supported)            |
| `enum`                                         | typedef to underlying type |

## zircon/fidl.h

The `zircon/fidl.h` header defines the basic constructs of the FIDL wire format.
The header is part of the Zircon system headers and depends only on other Zircon
system headers and a small portion of the C standard library.

### fidl_message_header_t

```c
typedef struct fidl_message_header {
    zx_txid_t txid;
    uint32_t reserved0;
    uint32_t flags;
    uint32_t ordinal;
} fidl_message_header_t;
```

Defines the initial part of every FIDL message sent over a channel. The header
is immediately followed by the body of the payload. Currently, there are no
flags to be set, and so `flags` must be zero.

### fidl_string_t

```c
typedef struct fidl_string {
    // Number of UTF-8 code units (bytes), must be 0 if |data| is null.
    uint64_t size;

    // Pointer to UTF-8 code units (bytes) or null
    char* data;
} fidl_string_t;
```

Holds a reference to a variable-length string.

When decoded, **data** points to the location within the buffer where the string
content lives, or **NULL** if the reference is null.

When encoded, **data** is replaced by **FIDL_ALLOC_PRESENT** when the reference
is non-null or **FIDL_ALLOC_ABSENT** when the reference is null. The location of
the string's content is determined by the depth-first traversal order of the
message during decoding.

### fidl_vector_t

```c
typedef struct fidl_vector {
    // Number of elements, must be 0 if |data| is null.
    uint64_t count;

    // Pointer to element data or null.
    void* data;
} fidl_vector_t;
```

Holds a reference to a variable-length vector of elements.

When decoded, **data** points to the location within the buffer where the
elements live, or **NULL** if the reference is null.

When encoded, **data** is replaced by **FIDL_ALLOC_PRESENT** when the reference
is non-null or **FIDL_ALLOC_ABSENT** when the reference is null. The location of
the vector's content is determined by the depth-first traversal order of the
message during decoding.

### fidl_incoming_msg_t and fidl_outgoing_msg_t

Messages are represented by two structs, fidl_incoming_msg_t and
fidl_outgoing_msg_t, depending on whether they are being used on the incoming
or outgoing direction.

These structs are currently identical, but are separated because they will
diverge in the future.

```c
typedef struct fidl_incoming_msg {
    // The bytes of the message.
    //
    // The bytes of the message might be in the encoded or decoded form.
    // Functions that take a |fidl_incoming_msg_t| as an argument should document whether
    // the expect encoded or decoded messages.
    //
    // See |num_bytes| for the number of bytes in the message.
    void* bytes;

    // The handles of the message.
    //
    // See |num_bytes| for the number of bytes in the message.
    zx_handle_t* handles;

    // The number of bytes in |bytes|.
    uint32_t num_bytes;

    // The number of handles in |handles|.
    uint32_t num_handles;
} fidl_incoming_msg_t;

typedef struct fidl_outgoing_msg {
    // The bytes of the message.
    //
    // The bytes of the message might be in the encoded or decoded form.
    // Functions that take a |fidl_outgoing_msg_t| as an argument should document whether
    // the expect encoded or decoded messages.
    //
    // See |num_bytes| for the number of bytes in the message.
    void* bytes;

    // The handles of the message.
    //
    // See |num_bytes| for the number of bytes in the message.
    zx_handle_t* handles;

    // The number of bytes in |bytes|.
    uint32_t num_bytes;

    // The number of handles in |handles|.
    uint32_t num_handles;
} fidl_outgoing_msg_t;
```

Represents a FIDL message, including both `bytes` and `handles`. The message
might be in the encoded or decoded format. The ownership semantics for the
memory referred to by `bytes` and `handles` is defined by the context in which
the `fidl_incoming_msg_t` and `fidl_outgoing_msg_t` struct is used.

### fidl_txn_t

```c
typedef struct fidl_txn fidl_txn_t;
struct fidl_txn {
    // Replies to the outstanding request and complete the FIDL transaction.
    //
    // Pass the |fidl_txn_t| object itself as the first parameter. The |msg|
    // should already be encoded. This function always consumes any handles
    // present in |msg|.
    //
    // Call |reply| only once for each |txn| object. After |reply| returns, the
    // |txn| object is considered invalid and might have been freed or reused
    // for another purpose.
    zx_status_t (*reply)(fidl_txn_t* txn, const fidl_outgoing_msg_t* msg);
};
```

Represents a outstanding FIDL transaction that requires a reply. Used by the
simple C bindings to route replies to the correct transaction on the correct
channel.

## Raw Bindings

### fidl_encode / fidl_encode_msg

```c
zx_status_t fidl_encode(const fidl_type_t* type, void* bytes, uint32_t num_bytes,
                        zx_handle_t* handles, uint32_t max_handles,
                        uint32_t* out_actual_handles, const char** out_error_msg);
zx_status_t fidl_encode_msg(const fidl_type_t* type, fidl_outging_msg_t* msg,
                            uint32_t* out_actual_handles, const char** out_error_msg);
```

Declared in
[lib/fidl/coding.h](/zircon/system/ulib/fidl/include/lib/fidl/coding.h),
defined in
[encoding.cc](/zircon/system/ulib/fidl/encoding.cc).

Encodes and validates exactly **num_bytes** of the object in **bytes** in-place
by performing a depth-first traversal of the encoding data from **type**
to fix up internal references. Replaces internal pointers references with
`FIDL_ALLOC_ABSENT` or `FIDL_ALLOC_PRESENT` to indicate presence.
Extracts non-zero internal handle references out of **bytes**, stores up to
**max_handles** of them sequentially in **handles**, and replaces their location
in **bytes** with `FIDL_HANDLE_PRESENT` to indicate their presence. Sets
**out_actual_handles** to the number of handles stored in **handles**.

To prevent handle leakage, this operation ensures that either all handles within
**bytes** are moved into **handles** in case of success or they are all closed in
case of an error.

If a recoverable error occurs, such as encountering a null pointer for a
required sub-object, **bytes** remains in an unusable partially modified state.

All handles in **bytes** which were already been consumed up to the point of the
error are closed and **out_actual_handles** is set to zero. Depth-first traversal of
the object then continues to completion, closing all remaining handles in **bytes**.

If an unrecoverable error occurs, such as exceeding the bound of the buffer,
exceeding the maximum nested complex object recursion depth, encountering
invalid encoding table data, or a dangling pointer, the behavior is undefined.

On success, **bytes** and **handles** describe an encoded object ready to be sent
using `zx_channel_send()`.

If anything other than `ZX_OK` is returned, **error_msg_out** will be set.

Result is...

*   `ZX_OK`: success
*   `ZX_ERR_INVALID_ARGS`:
    *   **type** is null
    *   **bytes** is null
    *   **actual_handles_out** is null
    *   **handles** is null and **max_handles** != 0
    *   **type** is not a FIDL struct
    *   there are more than **max_handles** in **bytes**
    *   the total length of the object in **bytes** determined by the traversal
        does not equal precisely **num_bytes**
    *   **bytes** contains an invalid union field, according to **type**
    *   a required pointer reference in **bytes** was null
    *   a required handle reference in **bytes** was `ZX_HANDLE_INVALID`
    *   a bounded string or vector in **bytes** is too large, according to
        **type**
    *   a pointer reference in **bytes** does not have the expected value
        according to the traversal
    *   `FIDL_RECURSION_DEPTH` was exceeded (see
        [wire format][wire-format])

This function is effectively a simple interpreter of the contents of the
type. Unless the object encoding includes internal references that
must be fixed up, the only work amounts to checking the object size and the
ranges of data types such as enums and union tags.

### fidl_decode / fidl_decode_msg

```c
zx_status_t fidl_decode(const fidl_type_t* type, void* bytes, uint32_t num_bytes,
                        const zx_handle_t* handles, uint32_t num_handles,
                        const char** error_msg_out);
zx_status_t fidl_decode_msg(const fidl_type_t* type, fidl_incoming_msg_t* msg,
                            const char** out_error_msg);
```

Declared in
[lib/fidl/coding.h](/zircon/system/ulib/fidl/include/lib/fidl/coding.h),
defined in
[decoding_and_validating.cc](/zircon/system/ulib/fidl/decoding_and_validating.cc).

Decodes and validates the object in **bytes** in-place by performing a
depth-first traversal of the encoding data from **type** to fix up internal
references. Patches internal pointers within **bytes** whose value is
`FIDL_ALLOC_PRESENT` to refer to the address of the out-of-line data they
reference later in the buffer. Populates internal handles within **bytes**
whose value is `FIDL_HANDLE_PRESENT` to their corresponding handle taken
sequentially from **handles**.

To prevent handle leakage, this operation ensures that either all handles in
**handles** from **handles[0]** to **handles[num_handles - 1]** are moved into
**bytes** in case of success or they are all closed in case of an error.

The **handles** array is not modified by the operation.

If a recoverable error occurs, a result is returned, **bytes** remains in an
unusable partially modified state, and all handles in **handles** are closed.

If an unrecoverable error occurs, such as encountering an invalid **type**,
the behavior is undefined.

If anything other than `ZX_OK` is returned, **error_msg_out** will be set.

Result is...

*   `ZX_OK`: success
*   `ZX_ERR_INVALID_ARGS`:
    *   **type** is null
    *   **bytes** is null
    *   **handles** is null but **num_handles** != 0.
    *   **handles** is null but **bytes** contained at least one valid handle
        reference
    *   **type** is not a FIDL struct
    *   the total length of the object determined by the traversal does not equal
        precisely **num_bytes**
    *   the total number of handles determined by the traversal does not equal
        precisely **num_handles**
    *   **bytes** contains an invalid union field, according to **type**
    *   a required pointer reference in **bytes** is `FIDL_ALLOC_ABSENT`.
    *   a required handle reference in **bytes** is `ZX_HANDLE_INVALID`.
    *   **bytes** contains an optional pointer reference that is marked
        as `FIDL_ALLOC_ABSENT` but has size > 0.
    *   a bounded string or vector in **bytes** is too large, according to
        **type**
    *   a pointer reference in **bytes** has a value other than
        `FIDL_ALLOC_ABSENT` or `FIDL_ALLOC_PRESENT`.
    *   a handle reference in **bytes** has a value other than
        `ZX_HANDLE_INVALID` or `FIDL_HANDLE_PRESENT`.
    *   `FIDL_RECURSION_DEPTH` was exceeded (see
        [wire format][wire-format])

This function is effectively a simple interpreter of the contents of the
type. Unless the object encoding includes internal references that
must be fixed up, the only work amounts to checking the object size and the
ranges of data types such as enums and union tags.

### fidl_epitaph_write

```c
zx_status_t fidl_epitaph_write(zx_handle_t channel, zx_status_t error);
```

Declared in
[lib/fidl/epitaph.h](/zircon/system/ulib/fidl/include/lib/fidl/epitaph.h),
defined in
[epitaph.c](/zircon/system/ulib/fidl/epitaph.c).

This function sends an epitaph with the given error number down the given
channel.  An epitaph is a special message, with ordinal 0xFFFFFFFF, which
contains an error code.  The epitaph must be the last thing sent down the
channel before it is closed.

### Sending Messages

The client performs the following operations to send a message through a
channel.

*   Obtain a buffer large enough to hold the entire message.
*   Write the message header into the buffer, which includes the transaction id
    and method ordinal.
*   Write the message body into the buffer, which includes the method arguments
    and any secondary objects (see
    [wire format][wire-format]
    for a definition of secondary objects).
*   Call **fidl_encode()** to encode the message and handles for
    transfer, taking care to pass a pointer to the **encoding table** of the
    message.
*   Call **zx_channel_write()** to send the message buffer and its associated
    handles.
*   Discard or reuse the buffer. (No need to release handles since they were
    transferred.)

For especially simple messages, it may be possible to skip the encoding step
altogether (or do it manually).

### Receiving Messages

The client performs the following operations to receive a message through a
channel.

*   Obtain a buffer large enough to hold the largest possible message that can
    be received by this protocol. (May dynamically allocate the buffer after
    getting the incoming message size from the channel.)
*   Call **zx_channel_read()** to read the message into the buffer and its
    associated handles.
*   Dispatch the message based on the method ordinal stored in the message
    header. If the message is invalid, close the handles and skip to the last
    step.
*   Call **fidl_decode()** to decode and validate the message and handles
    for access, taking care to pass a pointer to the **encoding table** of the
    message.
*   If the message is invalid, skip to last step. (No need to release handles
    since they will be closed automatically by the decoder.)
*   Consume the message.
*   Discard or reuse the buffer.

For especially simple messages, it may be possible to skip the encoding step
altogether (or do it manually).

### Closing Channels

The C language bindings do not provide any special affordances for closing
channels.  Per the FIDL specification, an epitaph must be sent as the last
message prior to closing a channel.  Code should call **fidl_epitaph_write()**
prior to closing a channel.

### Dispatching Messages

The C language bindings do not provide any special affordances for dispatching
protocol method calls. The client should dispatch manually based on the
protocol method ordinal, such as by using a **switch** statement.

## Simple Bindings

The simple C bindings provide easy-to-use C bindings for a subset of the FIDL
language.

### Simple Layout

In order to generate simple C bindings for a protocol, the protocol must have
the `@for_deprecated_c_bindings` attribute. This attribute enforces that the
protocol, including the types referenced by it, conform to the language subset
supported by FIDL.

Specifically, every message in the protocol (including both requests and
response) must not have any secondary objects except strings and vectors of
handles or primitives (see [wire format][wire-format] for a definition of
secondary objects). This invariant simplifies the memory ownership semantics.
Additionally, all strings and vectors must have explicit non-maximal length
bounds. `vector<int64>:64` is a vector with such a bound, while `vector<int64>`
lacks an explicit non-maximal bound. This requirement simplifies buffer
management for clients that receive these values.

For example, structs and unions can embed other structs and unions, but they
cannot contain nullable references to other structs or unions because nullable
structs and unions are stored out-of-line in secondary objects. Nullable handles
and protocols are allowed because they're stored inline as `ZX_HANDLE_INVALID`.

Below is an example of a protocol that meets these requirements:

```fidl
library unn.fleet;

type SolarPosition = struct {
    coord array<int64,3>;
};

type Alert = flexible enum {
    GREEN = 1;
    YELLOW = 2;
    RED = 3;
};

@for_deprecated_c_bindings
protocol SpaceShip {
    AdjustHeading(struct {
        destination SolarPosition;
    }) -> (struct {
        result int8;
    });
    ScanForLifeforms() -> (struct {
        life_signs vector<uint32>:64;
    });
    SetDefenseCondition(struct {
        alert Alert;
    });
};
```

### Client

For clients, the simple C bindings generate a function for each method that
takes a channel as its first parameter. These functions are safe to use from any
thread and do not require any coordination:

```c
zx_status_t unn_fleet_SpaceShipSetDefenseCondition(
    zx_handle_t channel,
    const unn_fleet_Alert* alert);
```

If the method has a response, the generated function will wait synchronously for
the server to reply. If the response contains any data, the data is returned to
the caller through out parameters:

```c
zx_status_t unn_fleet_SpaceShipAdjustHeading(
    zx_handle_t channel,
    const unn_fleet_SolarPosition* destination,
    int8_t* result);
```

The `zx_status_t` returned by these functions indicates whether the transport
was successful. Protocol-level status is communicated through out parameters.

### Server

For servers, the simple C bindings generate an ops table that contains a
function pointer for every method in the protocol and a dispatch method that
decodes the `fidl_incoming_msg_t` and calls the appropriate function pointer:

```c
typedef struct unn_fleet_SpaceShip_ops {
    zx_status_t (*AdjustHeading)(void* ctx,
                                 const unn_fleet_SolarPosition* destination,
                                 fidl_txn_t* txn);
    zx_status_t (*ScanForLifeforms)(void* ctx, fidl_txn_t* txn);
    zx_status_t (*SetDefenseCondition)(void* ctx, const unn_fleet_Alert* alert);
} unn_fleet_SpaceShip_ops_t;

zx_status_t unn_fleet_SpaceShip_dispatch(
    void* ctx,
    fidl_txn_t* txn,
    fidl_incoming_msg_t* msg,
    const unn_fleet_SpaceShip_ops_t* ops);
```

The `ctx` parameter is an opaque parameter that is passed through the dispatch
function to the appropriate function pointer. You can use the `ctx` parameter to
pass contextual information to the method implementations.

The `txn` parameter is passed through the dispatch function to function pointers
for methods that have responses. To reply to a message, the implementation of
that method should call the appropriate reply function:

```c
zx_status_t unn_fleet_SpaceShipScanForLifeforms_reply(
    fidl_txn_t* txn,
    const uint32_t* life_signs_data,
    size_t life_signs_count);
```

For example, `ScanForLifeforms` might be implemented as follows:

```c
static zx_status_t SpaceShip_ScanForLifeforms(void* ctx, fidl_txn_t* txn) {
    uint32_t life_signs[4] = {42u, 32u, 79u, 23u};
    return unn_fleet_SpaceShipScanForLifeforms_reply(txn, life_signs, 4);
}
```

These reply functions encode the reply and call through the `reply` function
pointer on `fidl_outgoing_msg_t`.

### Binding

FIDL also provides `fidl_bind`, defined in
[lib/fidl/bind.h](/zircon/system/ulib/fidl-async/include/lib/fidl-async/bind.h),
that binds a generated dispatch function to an `async_dispatcher_t`.
The `fidl_bind` function creates an `async_wait_t` that waits for messages on
the channel and calls through the given dispatcher (and ops table) when they
arrive.

<!-- xrefs -->
[c-family-comparison]: /docs/development/languages/fidl/guides/c-family-comparison.md
[echo-fidl]: /examples/fidl/fuchsia.examples/echo.test.fidl
[echo-server]: /examples/fidl/c/server/echo_server.c
[fidl-intro]: /docs/development/languages/fidl/tutorials/fidl.md
[llcpp-tutorial]: /docs/development/languages/fidl/tutorials/llcpp
[wire-format]: /docs/reference/fidl/language/wire-format
