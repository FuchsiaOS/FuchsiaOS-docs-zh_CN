# FIDL in drivers

Caution: This page may contain information that is specific to the legacy
version of the driver framework (DFv1).

A device may implement the `ddk::Messagable` mixin in order to be messaged by devfs
when a client tries to connect to the driver. A driver has to implement `ddk::Messagable`
with the specific [FIDL](/development/languages/fidl/README.md) interface it would like
to speak.

For example a driver implementing the
[`fuchsia_input_report::InputDevice`](/sdk/fidl/fuchsia.input.report/device.fidl)
interface might have a class definition like the following:

```
class InputReportDriver;
using DriverDeviceType = ddk::Device<InputReportDriver, ddk::Unbindable,
                                     ddk::Messageable<fuchsia_input_report::InputDevice>::Mixin>;
class InputReportDriver : public DriverDeviceType{
    // Implement the class methods here.
};
```

Clients that wish to speak to this device would open the relevant device file in
`/dev/class/input-report/` and begin sending FIDL messages.
