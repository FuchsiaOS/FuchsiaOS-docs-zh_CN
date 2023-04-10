# Using new C++ bindings over the driver transport

When a protocol has a `@transport("Driver")` attribute, the protocol operates
over the driver transport rather than the zircon channel transport:

```fidl
library fuchsia.example;

@transport("Driver")
protocol Protocol {
    TwoWay(struct {
        request uint32;
    }) -> (struct {
        response uint32;
    });
};
```

The driver transport builds on the [Fuchsia driver runtime][fdf] , which has
different memory management constraints and a different threading model than
with Zircon channels. As such, the FIDL client and server types are different
from those found in protocols over Zircon channels, to provide a tailored API.

Most client/server types under the `fidl::` namespace will have a counterpart in
the `fdf::` namespace, that provides the same features but is specialized to the
driver runtime. For example, whereas one would write `fidl::Client` to declare
an asynchronous client over the Zircon channel transport, to use the example
protocol above one would write `fdf::Client<fuchsia_example::Protocol>`. The
generated FIDL header is of the form
`#include <fidl/fuchsia.example/cpp/driver/fidl.h>`.

Similarly, when using the client and server APIs restricted to wire types,
whereas one would write `fidl::WireClient` to declare an asynchronous client
over the Zircon channel transport, to use the example protocol above one would
write `fdf::WireClient<fuchsia_example::Protocol>`. One may include the wire
subset header at `#include <fidl/fuchsia.example/cpp/driver/wire.h>`.

<!-- TODO(105299): Better example/annotated walk-through similar to other tutorials. -->
C++ bindings over the driver transport is under iteration. For now the tests may
serve as [examples][async-example] of using the bindings.

<!-- xrefs -->
[fdf]: /sdk/lib/driver/runtime/
[async-example]: /sdk/lib/fidl_driver/tests/transport/wire_async_two_way_test.cc
