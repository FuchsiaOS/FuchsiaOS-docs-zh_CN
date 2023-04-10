<!-- Generated with fx rfc -->
<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0200" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
{# Fuchsia RFCs use templates to display various fields from _rfcs.yaml. View the #}
{# fully rendered RFCs at https://fuchsia.dev/fuchsia-src/contribute/governance/rfcs #}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->
## Summary

<!-- Editorial note from kayce@: We are styling adb as lowercase throughout this document because that's how the Android docs style it: https://developer.android.com/studio/command-line/adb -->

This RFC proposes adding [Android Debug Bridge][adb-cli] (adb) protocol and interface support to
Fuchsia for hardware testing purposes. This will enable Fuchsia device interaction with stock adb
client, which has several applications in the current hardware testing workflows. adb support will
also enable us to discover and interact with Fuchsia devices from a Windows host, which is currently
not supported by Fuchsia tools. Additionally adding support for adb will allow us to reuse most of
the tests, tools and processes built around `adb shell` for hardware validation and manufacturing.
To add adb interface support, Fuchsia's USB peripheral device configuration will be updated to use a
new adb interface which will replace (or work alongside) the existing USB CDC ethernet interface on
builds where adb is enabled. The adb protocol support will be limited to the features deemed
necessary by hardware validation and manufacturing use cases. adb services supported will be Fuchsia
specific and will not try to mimic Android adb services.

## Motivation

adb support will be very useful in hardware validation and manufacturing testing scenarios:

* There are several tools, libraries, and frameworks built around adb
([1](https://pypi.org/project/pyadb/),[2](https://webadb.github.io/),[3](https://marketplace.visualstudio.com/items?itemName=vinicioslc.adb-interface-vscode)
  and many more) that can be reused.
* Prebuilt binary distributions of the adb client for various platforms are widely available and are
  easy to set up.
* Existing test frameworks that rely on adb to provide device discovery, connection and command
  execution services would work without any changes.
* Developers are familiar with adb due to its popularity and open source community support is also
  extensive.
* It can further extend use cases of `ffx` by tunneling [overnet][overnet] connections in
  environments currently not supported like Windows. adb for Windows is well-tested and widely used
  and the adb Windows driver is readily available from Google for installation.
* adb supports USB peripheral based discovery and communication which has lower latency compared to
  the [mDNS](https://en.wikipedia.org/wiki/Multicast_DNS) based discovery currently used in Fuchsia.
  Reducing the latency is important for manufacturing use cases.

Considering that adb is a lightweight and generally stable tooling, it will be a good addition to
the Fuchsia toolset.

## Stakeholders

_Facilitator:_

leannogasawara@google.com

_Reviewers:_

* curtisgalloway@google.com
* rdzhuang@google.com
* gkalsi@google.com
* prashanthsw@google.com
* jeremymason@google.com
* surajmalhotra@google.com

_Consulted:_

* palmer@google.com

_Socialization:_ A proof of concept was developed and discussed with relevant stakeholders.

## Design

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT",
"RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be interpreted as described in [IETF
RFC 2119](https://tools.ietf.org/html/rfc2119).

### Overview

[Android Debug Bridge (adb)][adb-cli] consists of three major parts - client, server and daemon. The
client and server run on the host machine and communicate with each other using sockets. The adb
daemon (adbd) runs on the device and is connected to the adb server usually through USB. The
communication between the adb daemon and server is defined in the [adb protocol][adb-protocol].

For adb to discover Fuchsia devices, we will have to expose a new USB adb
[interface][usb-interface]. To do this, the USB peripheral configuration will have to be updated and
a new usb adb function driver will have to be written. An adb server running on the host will
connect to the device through this interface. Once connected, the adb server will send/receive adb
protocol messages on the USB adb interface backed by the usb adb function driver. To encode/decode
these adb protocol messages, we need to write an adb daemon component. Depending on the service
requested, the adb daemon will forward the request to the appropriate component. We might have to
write new adb service components to bridge connections between existing components and the adb
daemon. We intend to support only a subset of the adb commands (See [adb services](#adb-services)).
The diagram below illustrates the proposed adb software stack:

<!-- Image source - https://docs.google.com/drawings/d/1JCdJTtUZVdlDFs2fytyVlHwx3nPLZTIrEyFaDz8zcYA/edit -->

![Alt text: USB adb stack shown: On the device - New driver usb-adb-function driver added on top of
usb-peripheral driver. New component adb.cm is added on top of usb-adb-function driver. New services
adb-shell.cm, adb-ffx.cm and many more could exist on top of adb.cm. On the host - Stock adb client
and adb server interact with the device through the usb host stack
](resources/0200_support_adb_protocol_and_interface_for_hardware_testing/USB_adb_stack.png)

Each part is discussed in detail in the following sections.

### adb interface and device discovery

adb will only be available on Fuchsia boards that support USB peripheral mode. In order to include
adb support to a Fuchsia product, the product configuration must include adb packages and configure
boot args to specify usb adb interface. By default adb interface will be enabled in hardware testing
products only. adb should not be enabled in user or production builds to limit the attack surface on
those builds.

Fuchsia's USB peripheral device configuration will be updated to use a new adb interface which will
replace (or work alongside) the existing USB CDC ethernet interface on builds where adb is enabled.
The new interface will follow adb interface requirements:

* USB class: `vendor`
* USB subclass: `0x42`
* Protocol: `1`

The adb server running on the host constantly polls for new USB devices with interface descriptors
matching the properties listed above. If found, it takes note of the USB serial number mentioned in
the USB device descriptor and uses it to identify the device. adb client will use this serial number
to route requests to the device. On Fuchsia this serial number is passed by the bootloader or
derived from MAC address or is a hardcoded fallback serial number, whichever is available in the
order listed.

### USB adb function driver

This driver is responsible for handling USB requests for the adb interface. This driver will only be
responsible for accounting for USB packets and callbacks.

### adb component

This component understands the adb protocol and is responsible for parsing and routing the messages
to appropriate services.

### adb services

When an adb client sends a command to the adb server, the server might reach out to the daemon with
a request to connect to a service on the device such as shell or logger service. The adb daemon
looks into a list of registered service providers to find a match. If matched, the registered
service provider requests a zircon socket. adb daemon forwards all communications from the adb
client related to the service to this socket. Examples of service requested are shell, logcat, port
forwarding, and file sync. The idea is to support these services by having separate components for
each. For example, we could have an `adb-shell` component that opens a dash shell and manages
pty-device-based communication between the adb transport and the dash shell; we could also have an
adb-ffx component that facilitates linking of adb transport and overnet. The list of services

The interface between services and adb component can be in these lines:

```fidl
// Max length of arguments passed in adb OPEN message.
const MAX_ARGS_LENGTH uint64 = 1024

/// A Provider is a provider for one service.
/// The interaction between the adb daemon and Provider is as follows:
///    - adb daemon is started eagerly by core.cml
///    - When an request for a service comes in, adb daemon starts up a lazy component serving
///      Provider and calls ConnectToService, handing it a socket.
///    - If the service has already been started, it opens that service and hands it a socket.
///    - adb daemon and Provider communicate over the socket.
@discoverable
protocol Provider {
    /// Connect `socket` to the service (called in response to adb OPEN message).
    /// `args` provides additional arguments passed by the client if any.
    ConnectToService(resource struct {
        socket zx.handle:SOCKET;
        args string:<MAX_ARGS_LENGTH, optional>;
    }) -> (struct {}) error zx.status;
};
```

adb protocol specifies several [services][adb-services], but we intend to support only a subset of
those - shell, logcat, sync (for adb push/pull). These services may not emulate all behaviors of adb
on other platforms and will be tailored for Fuchsia, such as, shell commands will have to match
Fuchsia supported commands and logs might be in Fuchsia system log format. adb shell service is
discussed in detail in the next section. Adding support for more services in the future will be
considered on a case by case basis.

### Example adb service : adb shell

This section describes interaction between adb services and adb daemon by considering the example of
adb shell service. The adb shell component will be responsible for providing adb shell service by
bridging adb shell I/O to a [dash shell](https://en.wikipedia.org/wiki/Almquist_shell) I/O through
[dash
launcher](https://cs.opensource.google/fuchsia/fuchsia/+/main:src/sys/tools/debug-dash-launcher/)
service. By default adb-shell.cm will provide a shell similar to that of using `ffx component
explore` over sshd-host.cm or something with more restricted capabilities. If we migrate to a
different user interface in Fuchsia, adb-shell can be migrated to it as well. To limit the
capabilities on specific product configurations and for specific use cases, a custom shell provider
with limited capabilities can substitute this component. It will be similar to using `ffx component
explore <specific-moniker>`.

The adb daemon will request a new adb shell session (and therefore a new dash session) every time
the user requests a new adb shell instance. Closing of the connection from either adb client or dash
will close the entire session. The sequence of interactions is shown in the following sequence
diagram:

<!-- Image source - https://docs.google.com/drawings/d/183lXMgYPMV9kwuGUYT-YOWUxAwUW1cch9jxzj4Fn-Es/edit?usp=sharing&resourcekey=0-_XX0uZS2dOfPwJDGUxfDRA -->
![Alt text: adb shell sequence diagram shown: On the host, adb client send "adb shell" message to
adb server. adb server then sends OPEN("shell") protocol message to adb.cm on the device. adb.cm
starts adb-shell.cm based on pre-configured service-component mapping and then calls
ConnectToService() API of adb-shell.cm. adb-shell.cm inturn calls LaunchWIthSocket protocol offered
by debug-dash-launcher.cm which spawns a dash shell and pty device. adb.cm responds back with
READY() message to the adb server. adb transport channel between adb client and dash is now
established. ](resources/0200_support_adb_protocol_and_interface_for_hardware_testing/adb_shell.png)

### Authentication and encryption

The adb protocol supports authentication using an RSA key pair. It also supports TLS encryption.
Both of these features are optional. For the initial implementation, these features will not be
implemented as the intended usage is only on developer or test builds running in restricted
environments. Also, since USB is the only transport supported, it will limit the attack methods to
some extent. In future, if these were to be supported, updates will have to be made to the adb
daemon alone.

### Maintenance

The current adb protocol version `0x01000000` will be supported. Future updates to the protocol will
be taken on a case by case basis. The adb protocol is not known to change often and has always been
backwards compatible.

## Implementation

Implementation can be divided into three parts.

* Adding support in different boards and builds.
* adb daemon
  * Parts of it will be imported from the Android codebase following the approval of
    [OSRB](/docs/contribute/governance/policy/osrb-process.md).
* adb service
  * Current plan is to support `adb-shell` and `adb-ffx`.
  * This phase might extend if other commands have to be supported.

A [proof of concept][poc-cl] exists today and it will serve as reference for the implementation.

## Performance

_Compute impact_ There should not be any significant compute overhead by adding adb support. adb
will be used instead of the ssh daemon and/or overnetstack, both of which rely on similar types of
drivers and components and hence the overall CPU usage of the system should remain the same.

_Impact on size_ The image size will increase with addition of adb daemon and services roughly by
1MB. Note that this would be only for development and testing builds that are assembled with adb
support. There should not be any significant impact on runtime memory footprint as adb will be used
instead of the existing tools.

_Latency_ Latency for command processing in adb would be slightly better than that of ssh or other
network based services due to the absence of an additional network stack. Device discovery latency
is expected to be smaller as well as it is based on USB enumeration and not on periodic broadcasting
as done in mDNS.

## Backwards compatibility

There are three parts to this - One is the backwards compatibility of the adb protocol itself, which
is outside the scope of control of the Fuchsia project. Nonetheless adb protocol is known to have
maintained backwards compatibility to date. Secondly, the adb services supported - deprecation of
services would affect host side commands/scripts that rely on them. Proper migration tactics will
have to be used when making such changes. Third, the shell commands. For instance, deprecating a
hypothetical CLI `xyz-tool` means scripts that run adb shell xyz-tool will no longer be functional.
This concern falls in the scope of Fuchsia tools and is not specific to adb and hence will not be
addressed.

## Security considerations

As for the security considerations for the adb transport, there are provisions for enabling
authentication and encryption. But these will not be enabled in the initial implementation. Since
adb will be available only in specific builds like developer builds or test builds and not in user
builds this should not be a major concern.

Regarding security considerations of services exposed through adb, these are no different from the
existing services on Fuchsia. The interaction surface exposed by that `adb-shell` / `adb-ffx` is the
same or smaller than that of dash-shell /`ffx`. For specific use cases, a tightly scoped shell
tailored for the use case can be used by replacing the shell provider from adb-shell to
custom-shell.

The initial implementation would support only USB transport. This will limit possible attack methods
when compared to network based connections. Also, adb daemon is a component in itself, hence any
vulnerabilities in it will be sandboxed to that specific process and only affect adb operations.

## Privacy considerations

The data exposed by the adb services is the same as that of the `ffx component explore` or SSH both
of which are vetted for privacy concerns.  Additionally, this technology will be restricted to
developer or test builds and will not be deployed in user/production builds. The USB adb interface
would however expose the device serial number and this is the same serial number used for other USB
interfaces like CDC ethernet interface. This is mostly configured by the bootloader, if not derived
from the MAC address. Care must be taken during product configuration to not use device IDs of
significant importance directly.

## Testing

All parts of the adb stack will be unit tested. Eventually, integration tests between the adb daemon
and the adb services will be added. Since the contracts between the adb subsystems are FIDL based
(except for the USB adb driver), the tests can be hermetic. Device enumeration test will be added
for the USB adb interface. If required, host side E2E test for adb will be added. For E2E tests, we
might need adb installed on the test host machines. The E2E tests / integration tests can be used
for [performance](/docs/development/performance/fuchsiaperf_format.md)
testing and command latency testing. Periodic stress testing of the adb connectivity by frequent
plug/unplug of USB will be considered for reliability testing. Fuzzing of the adb daemon
implementation will also be considered.

## Documentation

These documents will need to be added:

* List of adb features supported
* Guide for expanding adb services
* User guide

## Drawbacks, alternatives, and unknowns

Supporting adb comes at a cost of maintenance. There is an overhead for having two sets of tooling
for communication and device control - namely `ffx` and adb. Although these two will have
overlapping features, the use cases for each are differentiable and they will share the same backend
implementation. `ffx` caters to a wide variety of developer workflows, but currently has limitations
on host side scripting and varied platform support. adb can be used to fill in these gaps for
hardware testing. The maintenance cost is further reduced by sharing the services on the device
between `ffx` and adb. For example, dash launcher, overnetstack, log sink etc., can be used with adb
as well.

### Alternative: Port ffx to Windows

The current roadblocks for this are getting rust toolchain support for Windows and adding USB link
to `ffx`. Additionally, existing test frameworks that rely on adb will have to be altered to use
`ffx` or will have to provide a translation shim between adb and `ffx`. This strategy can be
revisited in the future. adb provides a convenient solution while development of `ffx` for Windows
is in process.

### Alternative: Run host side ffx in a Linux virtual machine

Use a Linux Virtual Machine(VM) running on Windows and pass through the device USB connections. With
this setup, the existing `ffx` workflows will work for both device discovery and interactions. But
these  time taken to set up the VM could be significant and may not be stable/reliable as well.
Also, some organizations limit the use of VMs on managed Windows machines. Using a Docker container
is another alternative, but USB forwarding may not work depending on the Windows version.

### Alternative: USB serial

Add support for USB CDC ACM peripheral device to Fuchsia. Windows contains a [USB serial][usbser]
driver in-box and works for any USB CDC ACM interface .This would allow us to use the USB port as a
serial communication device. The drawbacks with this approach are lack of a rich command set (only
shell will be available), no separation of logs and shell, and single instance support. Also,
existing test frameworks based on adb would have to be updated.

## Prior art and references

* [adb overview](https://android.googlesource.com/platform/packages/modules/adb/+/refs/tags/android-13.0.0_r3/OVERVIEW.TXT) - Overview of adb client, server and daemon
* [adb protocol][adb-protocol] - adb message types and fields
* [adb on Android](https://android.googlesource.com/platform/packages/modules/adb/+/refs/tags/android-13.0.0_r3/) - Android adb implementation

[poc-cl]: https://fuchsia-review.googlesource.com/c/fuchsia/+/710726
[usbser]:
    https://docs.microsoft.com/en-us/windows-hardware/drivers/usbcon/usb-driver-installation-based-on-compatible-ids
[overnet]: https://cs.opensource.google/fuchsia/fuchsia/+/main:src/connectivity/overnet/README.md
[adb-protocol]:
    https://android.googlesource.com/platform/packages/modules/adb/+/refs/tags/android-13.0.0_r3/protocol.txt
[adb-cli]: https://developer.android.com/studio/command-line/adb
[adb-services]:
    https://android.googlesource.com/platform/packages/modules/adb/+/refs/tags/android-13.0.0_r3/SERVICES.TXT
[usb-interface]: https://www.beyondlogic.org/usbnutshell/usb5.shtml#InterfaceDescriptors
