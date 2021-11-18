<!--
    (C) Copyright 2019 The Fuchsia Authors. All rights reserved.
    Use of this source code is governed by a BSD-style license that can be
    found in the LICENSE file.
-->

<!--
# Lifecycle of a USB request
-->

# 一个USB请求的生命周期

<!--
## Glossary
-->

## 术语表

<!--
*   HCI -- Host Controller Interface: A host controller interface driver is
    responsible for queueing USB requests to hardware, and managing the state
    of connected devices while operating as a USB host.
*   DCI -- Device controller interface: A device controller interface is
    responsible for queueing USB requests to a USB host that the device is
    connected to.
-->

*   HCI -- 主机控制器接口（Host Controller Interface）：一个主机控制器接口驱动
    负责维护向硬件发送的USB请求的队列，并且在作为USB主设备时管理所连接设备的状态。
*   DCI -- 设备控制器接口（Device controller interface）：一个设备控制器负责维护
    向该设备连接的USB主设备发送的USB请求的队列。

<!--
## Allocation
-->

## 分配

<!--
The first step in a USB request's lifecycle is allocation. USB requests contain
data from all of the drivers in the request stack in a single allocation. Each
driver that is upstream of a USB device driver should provide a
[GetRequestSize](/sdk/banjo/fuchsia.hardware.usb/usb.fidl#96) method --
which returns the size it needs to contain its local request context. When a
USB device driver allocates a request, it should invoke this method to
determine the size of the parent's request context.
-->

一个USB请求的生命周期的第一步是分配（allocation）。在一次分配中，USB请求
包含了来自请求栈（request stack）中所有驱动的数据。每一个位于USB设备驱动
上游的驱动应当提供一个[GetRequestSize](/sdk/banjo/fuchsia.hardware.usb/usb.fidl#96)
方法——它返回请求所需要的大小，用于包含它的本地请求上下文（local request context）。
当一个USB设备驱动分配请求时，它应当调用这个方法以确定父方法（parent）的请求上下文的大小。

<!--
Note: It is important to ensure that the lifetime of a request does not exceed
the lifetime of your driver. If your driver is released with outstanding
requests, you will encounter a use-after-free scenario when the parent tries to
send the request back to you by invoking your callback. Drivers should not
reply to unbind until all outstanding requests have been returned.
-->

注意：您需要保证一个请求的生命周期不超过您的驱动的生命周期，这很重要。如果您
的驱动在释放时带有尚未处理的请求，在父方法尝试通过调用您的回调以向您发回请求时，
您将会碰到“释放之后再使用”的情况。除非所有的尚未处理的请求都被返回完成，驱动不应当
回复来解绑（unbind）。

<!--
### C example {#c-example}
-->

### C代码示例 {#c-example}

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

<!--
### C++ example
-->

### C++代码示例

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

<!--
### C++ example (with lambdas)
-->

### C++代码示例（使用lambda表达式）

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

<!--
## Submission
-->

## 提交

<!--
You can submit requests using the
[RequestQueue](/sdk/banjo/fuchsia.hardware.usb/usb.fidl#22) method,
or -- in the case of CallbackRequests (as seen [here](#c-example)), using
`Request::Queue` or simply `request.Queue(client)`. In all cases, ownership of
the USB request is transferred to the parent driver (usually `usb-device`).
-->

您可以使用[RequestQueue](/sdk/banjo/fuchsia.hardware.usb/usb.fidl#22)来提交请求，
也可以在回调请求（CallbackRequests）的情况下（如[这里](#c-example)所示），使用`Request::Queue`
或者简简单单的 `request.Queue(client)`。所有情况下，USB请求的所有权被移交至父驱动（parent driver）
（通常是`usb-device`）。

<!--
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
-->

一个USB请求的典型生命周期如下所示（从一个设备驱动到一个主机控制器或者设备控制器）：

*   USB设备驱动将请求加入队列
*   `usb-device`核心驱动接收这个请求，并拥有了这个请求对象。
*   如果“direct” 标志位（flag）没有被设置，`usb-device`核心驱动向这个请求注入它自己的回调；如果
    “direct” 标志位被设置，`usb-device`核心驱动直接向HCI或者DCI驱动转发这个请求。
*   HCI或者DCI拥有了这个请求。HCI或者DCI驱动向硬件提交这个请求。
*   请求完成了。这个时候，如果“direct” 标志位被设置，设备驱动中的回调会被调用，并且设备驱动就拥有了
    这个请求。如果“direct” 标志位没有被设置，USB设备的（核心）驱动就拥有了这个请求。
*   如果核心驱动拥有了这个请求，这个请求就被加入到一个队列里，用于被另一个线程分派（dispatch）。
*   核心驱动最终调用回调，并且这个请求被设备驱动所拥有。设备驱动现在可以重新提交这个驱动。

<!--
## Cancellation
-->

## 取消

<!--
Requests may be cancelled by invoking
[CancelAll](/sdk/banjo/fuchsia.hardware.usb/usb.fidl#89). When
[CancelAll](/sdk/banjo/fuchsia.hardware.usb/usb.fidl#89) completes, all
requests are owned by the caller. Drivers implementing a
[CancelAll](/sdk/banjo/fuchsia.hardware.usb/usb.fidl#89) function (such
as the usb-device core driver and any HCI/DCI drivers) are responsible for
transferring ownership to their children with a `ZX_ERR_CANCELLED` status code.
-->

要取消请求，可以调用[CancelAll](/sdk/banjo/fuchsia.hardware.usb/usb.fidl#89)。当
[CancelAll](/sdk/banjo/fuchsia.hardware.usb/usb.fidl#89)完成时，所有的请求都被调用者
所拥有。实现了[CancelAll](/sdk/banjo/fuchsia.hardware.usb/usb.fidl#89)函数的驱动（例如
USB设备核心驱动和任何HCI/DCI驱动）有责任在将所有权移交给子方法（children）时使用
`ZX_ERR_CANCELLED`状态码。

<!--
## Implementation notes for writers of HCI, DCI, or filter drivers
-->

## 写给实现HCI，DCI或者过滤（filter）驱动的程序员的笔记

<!--
### Implementing [GetRequestSize](/sdk/banjo/fuchsia.hardware.usb/usb.fidl#96)
-->

### 实现 [GetRequestSize](/sdk/banjo/fuchsia.hardware.usb/usb.fidl#96)

<!--
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
-->

[GetRequestSize](/sdk/banjo/fuchsia.hardware.usb/usb.fidl#96)返回的值应该等于您的父方法的
[GetRequestSize](/sdk/banjo/fuchsia.hardware.usb/usb.fidl#96)的值加上您的请求上下文的大小，包括
任何必要的填充（padding），以保证您的数据结构正确对齐（如果适用）。如果您在实现一个HCI或者DCI驱动，
您在计算大小时，除了任何其他您存储的数据结构，您还必须包含`sizeof(usb_request_t)`。`usb_request_t`
没有特别的对齐要求，所以没有必要为这个数据结构添加填充。

<!--
### Implementing [RequestQueue](/sdk/banjo/fuchsia.hardware.usb/usb.fidl#22)
-->

### 实现 [RequestQueue](/sdk/banjo/fuchsia.hardware.usb/usb.fidl#22)

<!--
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
-->

[RequestQueue](/sdk/banjo/fuchsia.hardware.usb/usb.fidl#22)的实现者暂时从
它的客户驱动（client driver）取得了USB请求的所有权。作为
[RequestQueue](/sdk/banjo/fuchsia.hardware.usb/usb.fidl#22)的一名实现者，您能够访问
`usb_request_t`的所有字段（field），以及任何您向`usb_request_t`数据结构追加的私有数据
（通过使用[GetRequestSize](/sdk/banjo/fuchsia.hardware.usb/usb.fidl#96)请求额外的空间）。
但是您不能修改私有数据之外的任何数据，也就是从第`parent_req_size`个字节开始
（在`usb_request_t`的结束处之后）。

<!--
## Example USB request stack (HCI)
-->

## USB请求栈的例子（HCI）

<!--
xHCI (host controller) -> `usb-bus` -> `usb-device` (core USB device driver) ->
`usb-mass-storage`
-->

xHCI （主机控制器） -> `usb-bus` -> `usb-device` (USB设备核心驱动) ->
`usb-mass-storage`

<!--
## Example USB request stack (DCI)
-->

## USB请求栈的例子（DCI）

<!--
`dwc2` (device-side controller) -> `usb-peripheral` (peripheral core driver) ->
`usb-function` (core function driver) -> `cdc-eth-function` (ethernet
peripheral mode driver)
-->

`dwc2` (设备侧控制器) -> `usb-peripheral` (外围核心驱动) ->
`usb-function` (核心功能驱动) -> `cdc-eth-function` (以太网外围模式驱动)
