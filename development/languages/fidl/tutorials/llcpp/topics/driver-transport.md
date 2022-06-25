# Using LLCPP over the driver transport

When a protocol has a `@transport("Driver")` attribute, the protocol is defined
to use the driver transport:

```fidl
library example;

@transport("Driver")
protocol Protocol {
    TwoWay(struct {
        request uint32;
    }) -> (struct {
        response uint32;
    });
};
```

The driver transport builds on building blocks from the [Fuchsia driver
runtime][fdf], which comes with different memory management requirements and
threading model than Zircon channels. As such, the FIDL client and server types
are different from those found in protocols over Zircon channels, to provide a
tailored API.

As a general rule, most client/server types under the `fidl::`
namespace will have a counterpart in the `fdf::` namespace, that provides the
same features but is specialized to the driver runtime. For example, whereas one
would write `fidl::WireClient` to declare an asynchronous
client over the Zircon channel transport, to use the example protocol above one
would write `fdf::WireClient<example::Protocol>`. The generated FIDL header is
of the form `#include <fidl/example/cpp/driver/wire.h>`.

<!-- TODO: Better example/annotated walk-through similar to other tutorials. -->
LLCPP over the driver transport is under iteration. For now the tests may serve
as [examples][async-example] of using the bindings.


<!-- xrefs -->
[fdf]: /sdk/lib/driver_runtime/
[async-example]: /sdk/lib/fidl_driver/tests/transport/wire_async_two_way_test.cc
