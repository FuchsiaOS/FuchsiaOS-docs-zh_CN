# FIDL tutorial

Caution: This page may contain information that is specific to the legacy
version of the driver framework (DFv1).

This guide explains how to go about adding exporting a FIDL protocol from a
driver and utilize it in another driver. This guide assumes familiarity with the
following concepts:

*   [FIDL](/development/languages/fidl/README.md)
*   [Driver Binding](/development/drivers/concepts/device_driver_model/driver-binding.md)
*   [DDKTL](/development/drivers/concepts/driver_development/using-ddktl.md)
*   [LLCPP FIDL Bindings](/development/languages/fidl/tutorials/llcpp/README.md)

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
// LLCPP FIDL bindings
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

    // We start by creating a pair of endpoints. These are equivalent to a
    // zircon channel pair with better type safety.
    auto endpoints = fidl::CreateEndpoints<fuchsia_io::Directory>();
    if (endpoints.is_error()) {
      return endpoints.status_value();
    }

    // We add the FIDL protocol we wish to export to our child to our outgoing
    // directory. When a connection is attempted we will bind the server end of
    // the channel pair to our server implementation.
    device->outgoing_dir_.svc_dir()->AddEntry(
        fidl::DiscoverableProtocolName<fidl_examples_echo::Echo>,
        fbl::MakeRefCounted<fs::Service>(
            [device = device.get()](fidl::ServerEnd<fidl_examples_echo::Echo> request) mutable {
              device->Bind(std::move(request));
              return ZX_OK;
            }));

    // Utilizing the server end of the endpoint pair we created above, we bind
    // it to our outgoing directory.
    auto status = device->outgoing_dir_.Serve(std::move(endpoints->server));
    if (status != ZX_OK) {
      zxlogf(ERROR, "Failed to service the outoing directory");
      return status;
    }

    // We declare our outgoing protocols here. These will be utilize to
    // help the framework populate device properties which can be used for
    // binding.
    std::array offers = {
        fidl::DiscoverableProtocolName<fidl_examples_echo::Echo>,
    };

    status = device->DdkAdd(ddk::DeviceAddArgs("parent")
                                // The device must be spawned in a separate
                                // driver host.
                                .set_flags(DEVICE_ADD_MUST_ISOLATE)
                                .set_fidl_protocol_offers(offers)
                                // The client side of outgoing directory is
                                // provided to the framework. This will be
                                // forwarded to the new driver host that spawns to
                                // allow the child driver which binds the ability
                                // to connect to our outgoing FIDL protocols.
                                .set_outgoing_dir(endpoints->client.TakeChannel()));
    if (status == ZX_OK) {
      __UNUSED auto ptr = device.release();
    } else {
      zxlogf(ERROR, "Failed to add device");
    }

    return status;
  }

 private:
  // This is a helper routine which will bind the incoming request to our
  // server. Note that we continue to utilize the same framework provided
  // dispatcher to service the work.
  void Bind(fidl::ServerEnd<fidl_examples_echo::Echo> request) {
    fidl::BindServer<fidl::WireServer<fidl_examples_echo::Echo>>(device_get_dispatcher(parent()),
                                                                 std::move(request), this);
  }

  // This is the implementation of the only method our FIDL protocol requires.
  void EchoString(EchoStringRequestView request, EchoStringCompleter::Sync& completer) override {
    completer.Reply(request->value);
  }

  // This is a helper class which we use to serve the outgoing directory.
  svc::Outgoing outgoing_dir_;
};
```

## Child Driver (The Client)

### Binding

The first important thing to discuss is how the child driver will bind. It can
bind due to any number of device properties, but if you wish to bind based
solely on the FIDL protocol the parent exports, you will need to create the
following bind library:

```
library fidl.examples.echo;

bool Echo;
```

You will then need to use this library in your driver's bind rules:

```
using fidl.examples.echo;

fidl.examples.echo.Echo == true;
```

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
  auto client = fidl::BindSyncClient(std::move(endpoints->client));

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
