<!--
    (C) Copyright 2019 The Fuchsia Authors. All rights reserved.
    Use of this source code is governed by a BSD-style license that can be
    found in the LICENSE file.
-->

# Lifecycle of a USB request

Caution: This page may contain information that is specific to the legacy
version of the driver framework (DFv1).

## Glossary

*   HCI -- Host Controller Interface: A host controller interface driver is
    responsible for queueing USB requests to hardware, and managing the state
    of connected devices while operating as a USB host.
*   DCI -- Device controller interface: A device controller interface is
    responsible for queueing USB requests to a USB host that the device is
    connected to.

## Allocation

The first step in a USB request's lifecycle is allocation. USB requests contain
data from all of the drivers in the request stack in a single allocation. Each
driver that is upstream of a USB device driver should provide a
[GetRequestSize](/sdk/banjo/fuchsia.hardware.usb/usb.fidl#96) method --
which returns the size it needs to contain its local request context. When a
USB device driver allocates a request, it should invoke this method to
determine the size of the parent's request context.

Note: It is important to ensure that the lifetime of a request does not exceed
the lifetime of your driver. If your driver is released with outstanding
requests, you will encounter a use-after-free scenario when the parent tries to
send the request back to you by invoking your callback. Drivers should not
reply to unbind until all outstanding requests have been returned.

### C example {#c-example}

```c
size_t parent_req_size = usb_get_request_size(&usb);
usb_request_t* request;
usb_request_alloc(&request, transfer_length, endpoint_addr,
parent_req_size+sizeof(your_context_struct_t));
usb_request_complete_callback_t complete = {
      .callback = usb_request_complete,
      .ctx = your_context_pointer,
};
your_context_pointer.completion = complete;
usb_request_queue(&usb, request, &complete);
...

void usb_request_complete(void* cookie, usb_request_t* request) {
    your_context_struct_t data;
    // memcpy is needed to ensure alignment
    memcpy(&data, cookie, sizeof(data));
    // Do something here to process the response
    // ...

    // Requeue the request
    usb_request_queue(&data.usb, request, &data.completion);
}
```

### C++ example

```c++
parent_req_size = usb.GetRequestSize();
std::optional<usb::Request<void>> req;
status = usb::Request<void>::Alloc(&req, transfer_length,
endpoint_addr, parent_req_size);
usb_request_complete_callback_t complete = {
      .callback =
          [](void* ctx, usb_request_t* request) {
            static_cast<YourDeviceClass*>(ctx)->YourHandlerFunction(request);
          },
      .ctx = this,
  };
usb.RequestQueue(req->take(), &complete);
```

### C++ example (with lambdas)

```c++
size_t parent_size = usb_.GetRequestSize();
using Request = usb::CallbackRequest<sizeof(std::max_align_t) * 4>;
std::optional<Request> request;
Request::Alloc(&request, max_packet_size, endpoint_address,
parent_size, [=](Request request) {
    // Do some processing here.
    // ...
    // Re-queue the request
    Request::Queue(std::move(request), usb_client_);
});
```

## Submission

You can submit requests using the
[RequestQueue](/sdk/banjo/fuchsia.hardware.usb/usb.fidl#22) method,
or -- in the case of CallbackRequests (as seen [here](#c-example)), using
`Request::Queue` or simply `request.Queue(client)`. In all cases, ownership of
the USB request is transferred to the parent driver (usually `usb-device`).

The typical lifecycle of a USB request (from a device driver to either a host
controller or device controller is as follows):

*   The USB device driver queues the request
*   The `usb-device` core driver receives the request, and now owns the request
    object.
*   The `usb-device` core driver injects its own callback (if the direct flag
    is not set), or passes through the request (if the direct flag is set) to
    the HCI or DCI driver.
*   The HCI or DCI driver now owns the request. The HCI or DCI driver submits
    this request to hardware.
*   The request completes. When this happens, if the direct flag was set, the
    callback in the device driver is invoked, and the device driver now owns
    the request. If the direct flag is not set, the usb-device (core) driver
    now owns the request.
*   If the core driver owns the request; it is added to a queue for dispatch by
    another thread.
*   The core driver eventually invokes the callback, and the request is now
    owned by the device driver. The device driver can now re-submit the
    request.

## Cancellation

Requests may be cancelled by invoking
[CancelAll](/sdk/banjo/fuchsia.hardware.usb/usb.fidl#89). When
[CancelAll](/sdk/banjo/fuchsia.hardware.usb/usb.fidl#89) completes, all
requests are owned by the caller. Drivers implementing a
[CancelAll](/sdk/banjo/fuchsia.hardware.usb/usb.fidl#89) function (such
as the usb-device core driver and any HCI/DCI drivers) are responsible for
transferring ownership to their children with a `ZX_ERR_CANCELLED` status code.

## Implementation notes for writers of HCI, DCI, or filter drivers

### Implementing [GetRequestSize](/sdk/banjo/fuchsia.hardware.usb/usb.fidl#96)

The value returned by
[GetRequestSize](/sdk/banjo/fuchsia.hardware.usb/usb.fidl#96) should
equal the value of your parent's
[GetRequestSize](/sdk/banjo/fuchsia.hardware.usb/usb.fidl#96) + the size
of your request context, including any padding that would be necessary to
ensure proper alignment of your data structures (if applicable). If you are
implementing an HCI or DCI driver, you must include `sizeof(usb_request_t)` in
your size calculation in addition to any other data structures that you are
storing. `usb_request_t` has no special alignment requirements, so it is not
necessary to add padding for that structure.

### Implementing [RequestQueue](/sdk/banjo/fuchsia.hardware.usb/usb.fidl#22)

Implementors of
[RequestQueue](/sdk/banjo/fuchsia.hardware.usb/usb.fidl#22) temporarily
assumes ownership of the USB request from its client driver. As an implementor
of [RequestQueue](/sdk/banjo/fuchsia.hardware.usb/usb.fidl#22), you are
allowed to access all fields of the `usb_request_t`, as well as any private
data that you have appended to the `usb_request_t` structure (by requesting
additional space through
[GetRequestSize](/sdk/banjo/fuchsia.hardware.usb/usb.fidl#96)), but you
are not allowed to modify any data outside of your private area, which starts
at `parent_req_size` bytes (past the end of `usb_request_t`).

## Example USB request stack (HCI)

xHCI (host controller) -> `usb-bus` -> `usb-device` (core USB device driver) ->
`usb-mass-storage`

## Example USB request stack (DCI)

`dwc2` (device-side controller) -> `usb-peripheral` (peripheral core driver) ->
`usb-function` (core function driver) -> `cdc-eth-function` (ethernet
peripheral mode driver)
