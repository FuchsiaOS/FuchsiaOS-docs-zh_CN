# Scripting Layer for Fuchsia (SL4F)

Scripting Layer for Fuchsia ([SL4F](/src/testing/sl4f/)) is an HTTP server that
allows end-to-end tests to access [driver services](#facades-in-sl4f) (for
instance, Audio, Screen, Input, Diagnostics, and WebDriver) on a Fuchsia device.

Note: Fuchsia integrates with
[Android Comms Testing Suite](https://android.googlesource.com/platform/tools/test/connectivity/+/HEAD/acts/README.md){: .external}
(ACTS), which enables Fuchsia devices to adapt the existing connectivity tests
for Android. SL4F serves as a bridge between ACTS and a Fuchsia device.

End-to-end tests run on a host machine, unlike other Fuchsia packages that are
served to a Fuchsia device and run on the device. To trigger actions on a
Fuchsia device (for example, “swipe right on the screen”) from a remote test
host, end-to-end tests send JSON-RPC requests to SL4F running on the device.
SL4F then translates these requests into Fuchsia-equivalent
[FIDL](/docs/development/languages/fidl) commands for various driver services on
the device.

<a name="figure-1"></a>
<figure>
  <img src="/docs/images/testing/fuchsia-e2e-test-support-system.png"
       alt="Fuchsia's end-to-end test support system">
  <figcaption><b>Figure 1</b>. A diagram of Fuchsia's end-to-end test support service.</figcaption>
</figure>

However, not all requests from a test host to a Fuchsia device need to go
through SL4F. For instance, a test can directly access a Fuchsia device’s
Flutter driver or Chrome driver over separate, dedicated HTTP connections (see
[Figure 1](#figure-1)), given the condition that the test host can access these drivers’
ports on the device. If not, the test setup may require port forwarding.

## Interaction between an end-to-end test and SL4F {#interaction-between-end-to-end-test-and-sl4f}

The following sequence walks through how an end-to-end test triggers the “swipe
right on the screen” action on a Fuchsia device:

1.  An end-to-end test runs on a host machine.
1.  The test starts SL4F on the Fuchsia device over SSH.
1.  SL4F listens for HTTP requests on port 80 on the device.
1.  The test sends a JSON-RPC request to SL4F, asking the “swipe right on the
    screen” action on the device.
1.  SL4F’s [Input facade](/src/testing/sl4f/src/input/facade.rs#) parses the
    request into a FIDL command.
1.  The device performs the “swipe right on the screen” action.

Once started, SL4F continues to run on the device and accept requests until the
test terminates SL4F.

## Facades in SL4F {#facades-in-sl4f}

Fuchsia extends the functionality of SL4F to introduce additional orchestration
and inspection mechanisms that are necessary for testing Fuchsia products. SL4F
structures this new functionality into groupings called facades. Each facade
exposes one or more JSON-RPC methods.

For end-to-end tests to trigger actions and verify states on a Fuchsia device, a
facade offers the following mechanisms:

*   Orchestration: A facade is a wrapper for FIDL APIs that runs as a HTTP
    service handler on a Fuchsia device.
*   Inspection: A facade keeps track of asynchronous events on a Fuchsia device.

SL4F currently supports several facades, for example:

*   Audio facade - Insert and capture audio.
*   Screen facade - Capture the screen.
*   Input facade - Inject input gestures on the touch screen.
*   Diagnostics facade - Read data published by Inspect from components.
*   Wlan facade - Manipulate the status of a wireless LAN device.
*   Netstack facade - Manipulate network interfaces.
*   File facade - Write and read files on a device’s storage.
*   WebDriver facade - Enable and disable devtools in a Chrome webrunner.
