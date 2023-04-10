# Registering a trace provider

Trace providers must register with the trace manager in order
to participate in tracing. Drivers don't have to register as a trace provider
since the devhost process does it through `libdriver.so`.

To register a trace provider, you must do the following:

Note: For more information on the Fuchsia tracing system, see
[Fuchsia tracing system](/docs/concepts/kernel/tracing-system.md).

* [Register with the trace manager](#register-with-the-trace-manager)
* [Give the trace manager component access](#give-trace-manager-component-access)

## Register with the trace manager {#register-with-the-trace-manager}

To register as a trace provider, you can use the `libtrace-provider` library
to provide an asynchronous loop in your component's code.

Note: For more information on tracing libraries, see
[Tracing libraries](/docs/reference/tracing/libraries.md).

For example:

* {C++}

  Note: This example uses `fdio` to set up the FIDL channel with Trace Manager. For
  more information, see
  [`fdio`](/docs/concepts/filesystems/life_of_an_open.md#fdio).

  ```cpp
  #include <lib/async-loop/cpp/loop.h>
  #include <lib/async-loop/default.h>
  #include <lib/trace-provider/provider.h>
  // further includes

  int main(int argc, const char** argv) {
    // process argv

    async::Loop loop(&kAsyncLoopConfigAttachToCurrentThread);
    trace::TraceProviderWithFdio trace_provider(
        loop.dispatcher(), "my_trace_provider");

    // further setup

    loop.Run();
    return 0;
  }
  ```

* {C }

  ```c
  #include <lib/async-loop/cpp/loop.h>
  #include <lib/async-loop/default.h>
  #include <lib/trace-provider/provider.h>

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

* {Rust}

  ```rust
  fn main() {
      fuchsia_trace_provider::trace_provider_create_with_fdio();
      // ...
  }
  ```

## Setup Routing {#setup-routing}

Ensure that your component requests the appropriate tracing capabilities by
including the following in your component manifest:

* {.cml file}

  ```json5
  {
    include: [
      "trace/client.shard.cml",
    ],
    ...
  }
  ```

This allows your component to communicate with trace_manager using the
fuchsia.tracing.provider.Registry protocol as well as forward the offer to its
children.

If your component uses a Chromium-based `fuchsia.web` service and you would like to be able to collect
trace data from it, ensure that your `Context` is provided both
the `fuchsia.tracing.provider.Registry` and `fuchsia.tracing.perfetto.ProducerConnector`
capabilities.

Note: For more information on how capabilities are passed to the `Context`, see the documentation for
[`fuchsia.web/CreateContextParams.service_directory`](https://fuchsia.dev/reference/fidl/fuchsia.web#CreateContextParams.service_directory).

Once you have registered your component as a trace provider, you can enable
tracing in your code. For more information, see
[Adding tracing in your code](/docs/development/tracing/tutorial/adding-tracing-in-code.md).
