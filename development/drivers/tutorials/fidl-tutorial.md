# FIDL tutorial

Caution: This page may contain information that is specific to the legacy
version of the driver framework (DFv1).

This guide explains how to go about adding exporting a FIDL protocol from a
driver and utilize it in another driver. This guide assumes familiarity with the
following concepts:

*   [FIDL](/development/languages/fidl/README.md)
*   [Driver Binding](/development/drivers/concepts/device_driver_model/driver-binding.md)
*   [DDKTL](/development/drivers/concepts/driver_development/using-ddktl.md)
*   [New C++ FIDL Bindings](/development/languages/fidl/tutorials/cpp/README.md)

## FIDL Protocol Definition

The following snippets will utilize this FIDL protocol:

```
library fidl.examples.echo;

const MAX_STRING_LENGTH uint64 = 32;

// The discoverable annotation is required, otherwise the protocol bindings
// will not have a name string generated.
@discoverable
protocol Echo {
    /// Returns the input.
    EchoString(struct {
        value string:<MAX_STRING_LENGTH, optional>;
    }) -> (struct {
        response string:<MAX_STRING_LENGTH, optional>;
    });
};
```

## Parent Driver (The Server)

We approximate here how a parent driver which implements the protocol being
called into would be written. Although not shown, we assume this class is
utilizing the DDKTL.

```
// This class implement the fuchsia.examples.echo/Echo FIDL protocol using the
// new C++ FIDL bindings
class Device : public fidl::WireServer<fidl_examples_echo::Echo> {

  // This is the main entry point for the driver.
  static zx_status_t Bind(void* ctx, zx_device_t* parent) {
    // When creating the device, we initialize it with a dispatcher provided by
    // the driver framework. This dispatcher is allows us to schedule
    // asynchronous work on the same thread as other drivers. You may opt to
    // create your own dispatcher which is serviced on a thread you spawn if you
    // desire instead.
    auto* dispatcher = fdf::Dispatcher::GetCurrent()->async_dispatcher();
    auto device = std::make_unique<Device>(parent, dispatcher);

    // We add the FIDL protocol we wish to export to our child to our outgoing
    // directory. When a connection is attempted we will bind the server end of
    // the channel pair to our server implementation.
    zx::result = device->outgoing_.AddService<fidl_examples_echo::EchoService>(
        fidl_examples_echo::EchoService::InstanceHandler({
            .echo = device->bindings_.CreateHandler(device.get(), dispatcher,
                                                    fidl::kIgnoreBindingClosure),
            }));

    // Utilizing the server end of the endpoint pair we created above, we bind
    // it to our outgoing directory.
    result = device->outgoing_.Serve(std::move(endpoints->server));
    if (result.is_error()) {
      zxlogf(ERROR, "Failed to service the outgoing directory");
      return result.status_value();
    }

    // We declare our outgoing protocols here. These will be utilize to
    // help the framework populate node properties which can be used for
    // binding.
    std::array offers = {
        fidl_examples_echo::Service::Name,
    };

    status = device->DdkAdd(ddk::DeviceAddArgs("parent")
                                // The device must be spawned in a separate
                                // driver host.
                                .set_flags(DEVICE_ADD_MUST_ISOLATE)
                                .set_fidl_service_offers(offers)
                                // The client side of outgoing directory is
                                // provided to the framework. This will be
                                // forwarded to the new driver host that spawns to
                                // allow the child driver which binds the ability
                                // to connect to our outgoing FIDL protocols.
                                .set_outgoing_dir(endpoints->client.TakeChannel()));
    if (status == ZX_OK) {
      [[maybe_unused]] auto ptr = device.release();
    } else {
      zxlogf(ERROR, "Failed to add device");
    }

    return status;
  }

 private:
  // This is the implementation of the only method our FIDL protocol requires.
  void EchoString(EchoStringRequestView request, EchoStringCompleter::Sync& completer) override {
    completer.Reply(request->value);
  }

  // This is a helper class which we use to serve the outgoing directory.
  component::OutgoingDirectory outgoing_;
  // This ensures that the fidl connections don't outlive the device object.
  fidl::ServerBindingGroup<fidl_examples_echo::Echo> bindings_;
};
```

## Child Driver (The Client)

### Binding

The first important thing to discuss is how the child driver will bind. It can
bind due to any number of node properties, but if you wish to bind based
solely on the FIDL protocol the parent exports, you will need the bind library
that the build automatically generates for you from the FIDL library
(For more information, see [Generated bind libraries](#generated-bind-libraries)).

You will depend on and use this bind library in your driver's bind rules:

```
using fidl.examples.echo;

fidl.examples.echo.Echo == fidl.examples.echo.Echo.ZirconTransport;
```

ZirconTransport is the transport method that the parent driver uses to
provide the Echo FIDL protocol to the child.

You can addition additional bind constraints if you desire. Note that the
property which we describe here is only added if the parent driver declares
their FIDL protocol offers at the time of adding the device.

### Client Code

The follow code snippet would be found in a child driver which has successfully
bound to the parent driver described above.

```
zx_status_t CallEcho() {
  // We start by creating a pair of endpoints. This is similar to the parent
  // driver but this time we are creating them for the actual protocol we intend
  // to use.
  auto endpoints = fidl::CreateEndpoints<fidl_examples_echo::Echo>();
  if (endpoints.is_error()) {
    zxlogf(ERROR, "Failed to create endpoints");
    return endpoints.status_value();
  }

  // We turn the client side of the endpoint pair into a synchronous client.
  fidl::WireSyncClient client{std::move(endpoints->client)};

  // The following method allows us to connect to the protocol we desire. This
  // works by providing the server end of our endpoint pair to the framework. It
  // will push this channel through the outgoing directory to our parent driver
  // which will then bind it to its server implementation. We do  not need to
  // name the protocol because the method is templated on the  channel type and
  // it is able to automatically derive the name from the type.
  auto status = DdkConnectFidlProtocol(std::move(endpoints->server));
  if (status != ZX_OK) {
    zxlogf(ERROR, "Failed to connect fidl protocol");
    return status;
  }

  // We can now utilize our client to make calls!
  constexpr std::string_view kInput = "Test String";

  auto result = client->EchoString(fidl::StringView::FromExternal(cpp17::string_view(kInput)));
  if (!result.ok()) {
    zxlogf(ERROR, "Failed to call EchoString");
    return result.status();
  }
  if (result->response.get() != kInput) {
    zxlogf(ERROR, "Unexpected response: Actual: \"%.*s\", Expected: \"%.*s\"",
           static_cast<int>(result->response.size()), result->response.data(),
           static_cast<int>(kInput.size()), kInput.data());
    return ZX_ERR_INTERNAL;
  }

  return ZX_OK;
}
```

## Generated bind libraries {:#generated-bind-libraries}

All FIDL libraries get an auto-generated bind library created from them. This is to help driver
authors create bind rules based on FIDL protocols and services provided by the parent, and the
transport method the parent uses to provide each one.

### The bind library

There are three possible transport methods put in these bind libraries: `Banjo`, `ZirconTransport`,
and `DriverTransport`. Currently it is safe to assume the value is `ZirconTransport`
(which is just regular FIDL over Zircon channels). The bind library contains constants for
protocols and these transport methods.

Each service and discoverable protocol defined in the FIDL library gets an enum in the
bind library with the values of the enum being the three transport methods.

Here is an example of one where the FIDL library contains a single discoverable protocol:

#### protocol.fidl {:#protocol-fidl}

```fidl {:.devsite-disable-click-to-copy}
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/drivers/fidl_bindlib_codegen/protocol.fidl" region_tag="fidl" %}
```

#### Generated lib {:#generated-lib}

```none {:.devsite-disable-click-to-copy}
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/drivers/fidl_bindlib_codegen/generated_lib.bind.golden" %}
```

### The build target

These generated bind libraries will be based on the FIDL library's
`library_name` and `target_name`. The bind library will have a target name of
`{fidl_target_name}_bindlib`, and its `library_name` will be the same as the FIDL's.

For example, if the FIDL library target is `//sdk/fidl/fidl.examples.echo:myecholibrary`,
then the auto-generated bind library target is
`//sdk/fidl/fidl.examples.echo:myecholibrary_bindlib`.

In practice, most FIDL libraries have the same `target_name` as the folder they are in, which
is usually the library name as well. So for example, if the FIDL library is
`//sdk/fidl/fidl.examples.echo`, the auto-generated bind library target is
`//sdk/fidl/fidl.examples.echo:fidl.examples.echo_bindlib`.

### The generated code targets

These generated bind libraries work exactly the same as if they were user-written
bind libraries. Code generation for user-written bind libraries is described in detail at
[Bind library code generation tutorial](/development/drivers/tutorials/bind-libraries-codegen.md).

### Example

Lets take the FIDL library shown [above](#protocol-fidl) and use it in an example.


#### FIDL (BUILD.gn)

```gn {:.devsite-disable-click-to-copy}
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/drivers/fidl_bindlib_codegen/BUILD.gn" region_tag="fidl" %}
```

This now gives us the generated bind library with the target name of `:my_fidl_target_bindlib`
and library name of `fuchsia.gizmo.protocol`. The generated source for the bind library was shown
[earlier](#generated-lib). We can use this to create bind rules for the child driver.

#### Child bind rules (BUILD.gn)

```gn {:.devsite-disable-click-to-copy}
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/drivers/fidl_bindlib_codegen/BUILD.gn" region_tag="child_bind_rules" %}
```

#### child-driver.bind

```none {:.devsite-disable-click-to-copy}
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/drivers/fidl_bindlib_codegen/child_driver.bind" exclude_regexp="// Copyright.*|// Use of.*|// found in.*" %}
```

We can use the auto-generated code targets to access constants for this bind library from
the parent driver code.

#### Parent driver (BUILD.gn)

* {C++}

  ```gn {:.devsite-disable-click-to-copy}
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/drivers/fidl_bindlib_codegen/BUILD.gn" region_tag="example_cpp_target" %}
  ```

* {Rust}

  ```gn {:.devsite-disable-click-to-copy}
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/drivers/fidl_bindlib_codegen/BUILD.gn" region_tag="example_rust_target" %}
  ```



#### Parent driver code

* {C++}

  ```cpp {:.devsite-disable-click-to-copy}
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/drivers/fidl_bindlib_codegen/bindlib_usage.cc" region_tag="code" %}
  ```

* {Rust}

  ```rust {:.devsite-disable-click-to-copy}
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/drivers/fidl_bindlib_codegen/bindlib_usage.rs" region_tag="code" %}
  ```
