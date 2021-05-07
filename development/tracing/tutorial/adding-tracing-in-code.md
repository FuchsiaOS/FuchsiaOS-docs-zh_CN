# Adding tracing in your code

This guide shows how to add tracing to your code.

Note: For more information on the Fuchsia tracing system, see
[Fuchsia tracing system](/docs/concepts/tracing/README.md).

## Prerequisites

Before you begin, make sure you have completed the following:

* Familiarized yourself with the Fuchsia tracing system. See
  [Fuchsia tracing system](/docs/concepts/tracing/README.md).
* Registered your component as a tracing provider. See
  [Registering a trace provider](/docs/development/tracing/tutorial/registering-a-trace-provider.md).
* Included the `libtrace` library to capture trace data. See
  [libtrace: The C and C++ trace event library](/docs/reference/tracing/libraries.md#libtrace-trace-event).

## Add tracing to your component

Once your component is a trace provider, you can perform the following
types of tracing:

Note: The tracing macros should be added to the code of your component.
For more information on the C and C++ tracing macros, see
[Tracing: C and C++ macros](/docs/reference/tracing/c_cpp_macros.md).

* [Trace an instant event](#instant-event)
* [Disable tracing](#disable-tracing)
* [Determine if tracing is on](#determine-if-tracing-is-on)
* [Time an event](#time-an-event)

### Trace an instant event {#instant-event}

To write an instant event representing a single moment in time:

* {C and C++}

  ```c
  TRACE_INSTANT("helloworld", "hello_world_test", TRACE_SCOPE_PROCESS, "message", TA_STRING("Hello, World!"));
  ```

This example specifies a category of `helloworld`, a name of `hello_world_test`,
a scope of `TRACE_SCOPE_PROCESS`, and a key and value pair.

For more information on the `TRACE_INSTANT` macro, see
[TRACE_INSTANT](/docs/reference/tracing/c_cpp_macros.md#TRACE_INSTANT).

### Disable tracing {#disable-tracing}

There are cases where you might wish to entirely disable tracing such
as when you are about to make a final release. If the `NTRACE` macro
is in your code, then the tracing macros don't generate any code:

* {C and C++}

  ```c
  #define NTRACE  // disable tracing
  #include <lib/trace/event.h>
  ```

Make sure that you define the `NTRACE` macro before the `#include`statement.

In this example, the `rx_count` and `tx_count` fields are used only with
tracing, so if `NTRACE` is asserted, which indicates that tracing is disabled,
the fields don't take up space in the `my_statistics_t` structure.

* {C and C++}

  ```c
  typedef struct {
  #ifndef NTRACE  // reads as "if tracing is not disabled"
      uint64_t    rx_count;
      uint64_t    tx_count;
  #endif
      uint64_t    npackets;
  } my_statistics_t;
  ```

However, you do need to conditionally compile the code for managing
the recording of the statistics. For example, you can use the
`TRACE_INSTANT` macro:

* {C and C++}

  ```c
  #ifndef NTRACE
      status.tx_count++;
      TRACE_INSTANT("bandwidth", "txpackets", TRACE_SCOPE_PROCESS,
                    "count", TA_UINT64(status.tx_count));
  #endif  // NTRACE
  ```

For more information on the `NTRACE` macro, see
[NTRACE](/docs/reference/tracing/c_cpp_macros.md#NTRACE).

### Determine if tracing is on {#determine-if-tracing-is-on}

In some cases, you may need to determine if tracing is on at runtime.
If tracing is compiled in your code because `NTRACE` is not defined,
the `TRACE_ENABLED()` macro determines if tracing for your trace
provider is on.

If tracing is compiled out, then `TRACE_ENABLED()` always returns
false.

For example:

* {C and C++}

  ```c
  #ifndef NTRACE
      if (TRACE_ENABLED()) {
          int v = do_something_expensive();
          TRACE_INSTANT(...
      }
  #endif  // NTRACE
  ```

This example uses both the `#ifndef` and the `TRACE_ENABLED()` macro
together because the function `do_something_expensive()` might not exist in the
trace-disabled version of your code.

For more information on the `TRACE_ENABLED` macro, see
[TRACE_ENABLED](/docs/reference/tracing/c_cpp_macros.md#TRACE_ENABLED).

### Time an event {#time-an-event}

In some cases, you may need to time a function or procedure.

This example is from a `blobfs` vnode constructor. See
[//src/storage/blobfs/blobfs.cc](/src/storage/blobfs/blobfs.cc).

* {C++}

  ```cpp
  zx_status_t VnodeBlob::InitCompressed() {
      TRACE_DURATION("blobfs", "Blobfs::InitCompressed", "size", inode_.blob_size,
                     "blocks", inode_.num_blocks);
      ...
  ```

This example records the length of time spent in the constructor,
along with the size and number of blocks. Since this is a C++ example,
the data types can be inferred by the compiler.

For more information on the `TRACE_DURATION` macro, see
[TRACE_DURATION](/docs/reference/tracing/c_cpp_macros.md#TRACE_DURATION).

Once you have added a tracing statement to your component, you can now collect a
trace. For more information, see
[Recording a Fuchsia trace](/docs/development/tracing/tutorial/recording-a-fuchsia-trace.md).

