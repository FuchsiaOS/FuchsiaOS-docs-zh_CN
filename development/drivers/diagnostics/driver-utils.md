# Driver utilities

Driver utilities are Fuchsia applications that communicate with devices used for
diagnostics and exported by drivers. For example Inter-Integrated Circuit (I2C)
devices can be scanned and communicated through the
[i2cutil](/src/devices/i2c/bin) command line utility. For example:

```
i2cutil ping
/dev/class/i2c/000: OK
[00164.657] 04506:05266> i2c: error on bus
Error -1
/dev/class/i2c/001: ERROR
/dev/class/i2c/002: OK
/dev/class/i2c/003: OK
/dev/class/i2c/004: OK
```

## API

The communication mechanism between between drivers and applications is
[FIDL](/docs/development/languages/fidl/README.md), and hence the FIDL API
exported by any given driver fully defines what diagnostics can be performed on
it. For instance for I2C, from
[i2c.fidl](/sdk/fidl/fuchsia.hardware.i2c/i2c.fidl), there is a
`Transfer()` FIDL method that allows for writes and reads from I2C devices.

Note: [Banjo](/docs/development/drivers/tutorials/banjo-tutorial.md) should not be used for
driver utilities application to driver communication.

TODO(fxbug.dev/45662): Add inspect usage description.

## Discovery

The Fuchsia driver model defines a `devfs` filesystem (see
[Device Model](/docs/concepts/drivers/device_driver_model/device-model.md), which is the mechanism
through which userspace services and applications gain access to devices. As a
user, you can navigate the `devfs` filesystem to see what devices are exported,
note that there are multiple ways to access the same device, you can use the
`lsdev` command to find relationships as in:

```
lsdev /dev/class/i2c/000
topological path for /dev/class/i2c/000: /dev/sys/platform/05:00:2/aml-i2c/i2c/i2c-2-44
```

## Creating new driver utilities

### Discovery

You can discover existing drivers with `devfs`. New devices become discoverable
when their driver performs a `DdkAdd()` (for C++ drivers) operation.

### API

Existing utilities like `spiutil` make use of currently existing FIDL APIs. To
extend the functionality exported by an existing driver, the FIDL API the
existing driver exports can be extended/evolved by following
[FIDL ABI and API compatibility guide][abi-api-compat].
In cases when there is no existing FIDL API, you need to add new FIDL files to
a folder within [/sdk/fidl](/sdk/fidl).

To enable FIDL communication in C++ drivers that do not already offer a FIDL
API, complete the following steps:

1. Make the device messagable by deriving from `ddk::Messageable`.
2. Add a DdkMessage method to handle incoming FIDL messages.
3. Add methods for the FIDL protocol methods of the given FIDL API.

For instance for [SPI](/src/devices/spi/drivers/spi/spi.h):

```
using SpiChildType = ddk::Device<SpiChild, ddk::Messageable>;
class SpiChild : public SpiChildType,
                 public fuchsia_hardware_spi::Device::Interface,
                 public ddk::SpiProtocol<SpiChild, ddk::base_protocol> {
...
  zx_status_t DdkMessage(fidl_incoming_msg_t* msg, fidl_txn_t* txn);
...
  // FIDL methods.
  void Transmit(fidl::VectorView<uint8_t> data, TransmitCompleter::Sync completer) override;
...
```

### Utility

To implement the Fuchsia application that would communicate with the device,
call into the FIDL API. For this utilize the FIDL bindings for your language of
choice, for C++:

* [LLCPP](/docs/reference/fidl/bindings/llcpp-bindings.md)
* [HLCPP](/docs/reference/fidl/bindings/hlcpp-bindings.md)

For example for I2C in [i2cutil](/src/devices/i2c/bin) using LLCPP we have:

```
fuchsia_hardware_i2c::Device2::SyncClient client((zx::channel(channel)));
auto read = client.Transfer(...);
```

This calls the `Transfer()` method to write and read from an I2C device.

<!-- xrefs -->
[abi-api-compat]: /docs/development/languages/fidl/guides/compatibility/README.md
