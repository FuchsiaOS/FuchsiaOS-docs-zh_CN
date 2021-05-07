# Tracing libraries

For tracing, you can use the following libraries:

* [libtrace-provider: Trace provider library](#libtrace-provider)
* [libtrace: The C and C++ trace event library](#libtrace-trace-event)
* [libtrace-reader: Trace reader library](#libtrace-reader)

## libtrace-provider: Trace provider library {#libtrace-provider}

This library provides C and C++ functions to register a process's trace
engine with the Fuchsia tracing system. For tracing to work in your process,
you must initialize the trace provider at some point during its execution.
Alternatively, you can implement your own trace handler to register the
trace engine in some other way.

The trace provider requires an asynchronous dispatcher to operate.

* {C++}

  Note: This example uses `fdio` to set up the FIDL channel with the trace
  manager. For more information, see
  [`fdio`](/docs/concepts/system/life_of_an_open.md#fdio).

  ```c++
  #include <lib/async-loop/cpp/loop.h>
  #include <lib/async-loop/default.h>
  #include <trace-provider/provider.h>

  int main(int argc, char** argv) {
    // Create a message loop.
     async::Loop loop(&kAsyncLoopConfigNoAttachToCurrentThread);

    // Start a thread for the loop to run on.
    // Alternatively, you could use async_loop_run() to run on the current thread.
    zx_status_t status = loop.StartThread();
    if (status != ZX_OK) exit(1);

    // Create the trace provider.
    trace::TraceProviderWithFdio trace_provider(loop.dispatcher());

    // Do something...

    // The loop and trace provider will shut down once the scope exits.
    return 0;
  }
  ```

* {C }

  ```c
  #include <lib/async-loop/cpp/loop.h>
  #include <lib/async-loop/default.h>
  #include <trace-provider/provider.h>

  int main(int argc, char** argv) {
    zx_status_t status;
    async_loop_t* loop;
    trace_provider_t* trace_provider;

    // Create a message loop.
    status = async_loop_create(&kAsyncLoopConfigNoAttachToCurrentThread, &loop);
    if (status != ZX_OK) exit(1);

    // Start a thread for the loop to run on.
    // Alternatively, use async_loop_run() to run on the current thread.
    status = async_loop_start_thread(loop, "loop", NULL);
    if (status != ZX_OK) exit(1);

    // Create the trace provider.
    async_dispatcher_t* dispatcher = async_loop_get_dispatcher(loop);
    trace_provider = trace_provider_create(dispatcher);
    if (!trace_provider) exit(1);

    // Do something...

    // Tear down.
    trace_provider_destroy(trace_provider);
    async_loop_shutdown(loop);
    return 0;
  }
  ```

## libtrace: The C and C++ trace event library {#libtrace-trace-event}

This library provides macros and inline functions for instrumenting C and C++
programs with trace points for capturing trace data during trace execution.

See [`//zircon/system/ulib/trace/include/lib/trace/event.h`](/zircon/system/ulib/trace/include/lib/trace/event.h).

* {C++}

  This example records trace events marking the beginning and end of the
  execution of the `DoSomething` function together with its parameters.

  ```c++
  #include <trace/event.h>

  void DoSomething(int a, std::string b) {
    TRACE_DURATION("example", "DoSomething", "a", a, "b", b);

    // Do something
  }
  ```

* {C }

  This example records trace events marking the beginning and end of the
  execution of the `DoSomething` function together with its parameters.

  Unlike in C++, it is necessary to specify the type of each trace argument.
  In C++ such annotations are supported but are optional since the compiler
  can infer the type itself.

  ```c
  #include <trace/event.h>

  void DoSomething(int a, const char* b) {
    TRACE_DURATION("example", "DoSomething", "a", TA_INT32(a), "b", TA_STRING(b));

    // Do something
  }
  ```

### Suppress tracing within a compilation unit {#suppress-tracing-within-a-compilation-unit}

To completely suppress tracing within a compilation unit, define the `NTRACE`
macro prior to including the trace headers. This causes the macros to
behave as if tracing is always disabled so they will not produce trace
records and they will have zero runtime overhead.

```c
#define NTRACE
#include <trace/event.h>

void DoSomething(void) {
  // This will never produce trace records because the NTRACE macro was
  // defined above.
  TRACE_DURATION("example", "DoSomething");
}
```

## libtrace-reader: Trace reader library {#libtrace-reader}

This library provides C++ types and functions for reading trace archives.

See [`//zircon/system/ulib/trace-reader/include/trace-reader/reader.h`](/zircon/system/ulib/trace-reader/include/trace-reader/reader.h).

