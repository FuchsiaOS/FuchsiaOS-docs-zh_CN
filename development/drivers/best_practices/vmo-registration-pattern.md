# VMO Registration Pattern

Caution: This page may contain information that is specific to the legacy
version of the driver framework (DFv1).

## Summary

When transferring bulk data between applications and peripheral hardware, it
becomes important to minimize the number of copies the data goes through. For
example, let us say an application would like to read a file from component
persistent storage. In order to do so, the application makes a request to read
the file to a filesystem, which in turn may need to send a request to a block
device. Depending on the block partition topology, there may be several layers
of drivers the request passes through before ultimately hitting a driver which
can perform a read operation.

A naive approach to the above may result in sending FIDL messages over Zircon
channels across every layer between the application and the hardware, resulting
in many copies of the data. As this is inefficient, we don’t do this. Following
a well established pattern found throughout the industry, we split our messages
into two planes: a control plane and a data plane. Messages sent over the
control plane are small and cheap to send, whereas messages in the data plane
contain the bulk data which would be expensive to copy. Messages sent over the
control plane generally use FIDL protocols built on top of Zircon channels.
Messages in the data plane are sent via a shared memory primitive, Zircon VMOs.

With this in mind, a naive implementation may choose to create a new VMO for
each transaction which gets transferred via the control plane until it reaches
the driver issuing DMA, achieving the desired goal of zero copies between the
application which placed the data in the VMO and the final driver. This however
may not be sufficiently performant for the following two reasons:

* In order to issue a DMA request, the memory must first be pinned, which
  requires calling into the kernel and optionally setting up page mappings in an
  IOMMU.
* If the final driver needs to copy the request into a special buffer (as not
  all hardware supports DMA), it must either map the VMO into its process or
  call into the kernel in order to copy the memory.

Since both of these are costly we need a better approach: using pre-registered
VMOs. This works by having the application send a one-time control message in
order to register a VMO with the final driver in the stack. The response to this
message returns an identifier which may be used to refer to the VMO in the
future. Control messages should simply refer to this identifier rather than
attaching a VMO handle. Upon registration, the final driver in the stack can
perform the costly pinning or mapping operations once, and cache the results.

## Notes on VMO Identifier

In order to ensure that we do not fall prey to confused deputy attacks, we must
uphold the same invariants with respect to the VMO identifier as the kernel does
with handles. In order to do this, the VMO identifier must be unique to the
client at each layer, and each layer must validate that the identifier is valid.
More specifically, using a koid as an identifier still requires that the server
checks that a VMO with that koid was registered by the client.

In order to lower the number of round trips, it is possible to allow the client
to name the VMO identifier as part of the registration API, allowing one-shot
VMO usage to be efficient. Alternatively, the protocol can state that the VMO’s
koid will always be used as the identifier.

## Zircon FIFOs

In order to additionally improve performance, some protocols may also opt to use
FIFOs for their control plane. FIFOs have reduced complexity allowing for lower
overhead. One of their limitations is that they may not transfer handles. As a
result, using the VMO registration pattern is a necessity in order to use FIFOs.
(Note that a channel must still be used to perform the registration.)

## Library

This pattern potentially adds a lot of complexity to the driver which maintains
the mappings between VMO and the identifier. A library has been created to aid
the implementation, and lives under
[//src/lib/vmo_store](https://cs.opensource.google/fuchsia/fuchsia/+/main:src/lib/vmo_store/).
See
[//src/connectivity/network/drivers/network-device/device](https://cs.opensource.google/fuchsia/fuchsia/+/main:src/connectivity/network/drivers/network-device/device/)
for example usage.

## Downsides of the Pattern

For low-throughput situations, this pattern is unnecessarily complex and should
likely be avoided.

VMO registration causes a one-shot operation to become 2 round trips. If
one-shots are common, FIDL protocols should be sure to continue to allow for
one-shot VMOs to be used in addition to pre-registered VMOs. This can also be
mitigated by allowing the client to provide the identifier for the VMO during
registration.

VMOs which are pre-registered may lead to “leaked” memory situations where a
client keeps registering VMOs and forgets to unregister them. Additionally, if
the server is not careful with managing its clients, it may forget to clean up
registered VMOs belonging to a client which may have disconnected from the
server.

VMOs which are pre-registered with a driver which pins the VMOs cause the pages
backing the VMO to no longer be pageable.

## Driver-Specific Considerations

Since some drivers reside in the same driver host process and we have a
mini-driver pattern whereby we hoist common logic into a “core” driver, it might
seem like the obvious thing to do would be to perform the VMO registration in
the core driver rather than the device-specific driver. This however is not a
good idea for the following reasons:

* The core driver needs to be informed whether to perform pinning or mapping
  operations by the device-specific driver.
* Pinning requires access to the [bus transaction initiator
  (BTI)](/reference/kernel_objects/bus_transaction_initiator.md) handle
  provided by the platform-bus or pci drivers. Passing a BTI handle up the
  driver stack is an anti-pattern.
* In the case mapping is necessary, this means that raw buffers are passed over
  FIDL. This is an anti-pattern as it may no longer be possible without a copy
  in future iterations of in-process inter-driver communication.
* In either case if the operation is asynchronous (which most are), then the
  core driver becomes responsible for ensuring that it doesn’t unpin/unmap the
  VMO while it’s still in use. This is particularly problematic in situations
  such as shutdown and suspend which aren’t as well tested.
* In cases such as the block stack, the core driver is bound multiple times
  recursively in the same driver host. The core driver would need to be aware of
  whether it is bound directly to the driver which talks to hardware or a filter
  layer.
