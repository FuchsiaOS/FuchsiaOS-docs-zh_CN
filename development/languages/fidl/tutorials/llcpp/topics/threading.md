# LLCPP threading guide

LLCPP bindings accommodate a diverse set of threading models. Depending on the
architecture of your application, there are different classes and usage styles
to choose from. This document covers the tools and techniques to use FIDL in
non-trivial threading environments.

## Background: life of a FIDL connection

Within the lifetime of a FIDL connection, these occurrences are significant from
the perspective of thread-safety and preventing use-after-free:

![Figure: user code invokes to-binding calls on FIDL binding objects, binding
invokes to-user calls on user code, teardown cancels all
those](images/llcpp-fidl-lifecycle.svg){: width="80%"}

- **To-binding calls**: these are calls made by user code on a FIDL messaging
  object, i.e. inbound from the perspective of the FIDL runtime. For example:

  - Making a FIDL method call on a client is a to-binding call.
  - Making a reply from a server implementation using completers is also a
    to-binding call.

- **To-user calls**: these are calls made by the FIDL runtime on user objects
  (including callbacks provided by the user), i.e. outbound from the perspective
  of the FIDL runtime. For example:

    - A server message dispatcher invoking FIDL method handlers on a server
      implementation are to-user calls.
    - A FIDL client delivering the response to a two-way FIDL method to the user
      via a callback is also a to-user call.
    - Error handlers are also to-user calls.

  To-user calls are also sometimes called "upcalls" since the user objects are
  one layer above the FIDL bindings from the bindings' perspective.

- **Teardown**: actions that stop the message dispatch. In particular, when
  teardown is complete, no more to-user calls will be made by the bindings;
  to-binding calls will fail or produce void/trivial effects. Examples:

  - An error happening during dispatch.
  - Destroying a `fidl::WireClient`.
  - Calling `fidl::WireSharedClient::AsyncTeardown()`.

  Teardown usually leads to the closing of the client/server endpoint.

- **Unbind**: actions that stop the message dispatch, and additionally recover
  the client/server endpoint that was used to send and receive messages. Doing
  so necessarily involves teardown. Examples:

  - Calling `fidl::ServerBindingRef::Unbind()`.

### Use-after-free risks during teardown

Unless otherwise explicitly stated, all **to-binding** calls in LLCPP are safe
to invoke from multiple threads, including threads that do not belong to the
async dispatcher associated with the particular FIDL binding.

However, this thread-safety property does not prevent all forms of memory
corruption when using FIDL bindings. When destroying a set of related objects
including FIDL clients or servers, care must be taken to order their destruction
such that **to-user** calls made by the FIDL bindings runtime do not end up
calling into a destroyed object.

To give a concrete example, suppose a `MyDevice` object owns a FIDL client and
makes a number of two-way FIDL calls, passing a lambda that captures `this` as
the result callback every time. It is unsafe to destroy `MyDevice` while the
client could still be dispatching messages in the meantime. This is often the
case when the user destroys `MyDevice` (or other business objects) from a
non-dispatcher thread, i.e. not the thread that is monitoring and dispatching
messages for the current FIDL binding.

Similar use-after-free risks exist at destruction time when handling events and
when handling method calls from a server.

There are a few solutions to this problem, all in the spirit of adding mutual
exclusion between the destruction of user objects and **to-user** calls:

<ol>
    <li id="solution_1_scheduling">
        <b>Scheduling</b>: ensure that the destruction of relevant user objects
        is never scheduled in parallel with any to-user calls.
    </li>
    <li id="solution_2_ref_counting">
        <b>Reference-counting</b>: reference-count the user objects such that
        they are not destroyed until the binding teardown is complete.
    </li>
    <li id="solution_3_two_phase_shutdown">
        <b>Two-phase shutdown</b>: provide a notification when binding teardown
        is complete, such that the user could arrange the business objects to
        destruct after that.
    </li>
</ol>

LLCPP natively supports all above approaches. Ref-counting is inappropriate in
some situations, so it is an opt-in functionality when using the bindings.

## Client-side threading

There are two client types that supports async operations: `fidl::WireClient`
and `fidl::WireSharedClient`. For a precise reference of their semantics, refer
to their documentation in the [client header][client-header].

### WireClient

`fidl::WireClient` supports [solution #1 (scheduling)](#solution_1_scheduling)
by checking that it is bound and destroyed on the same dispatcher thread that
reads and handles messages from the channel. You may make FIDL method calls on
it from multiple threads, but the client object itself cannot be moved to
another object which is then destroyed on another thread. This ensures that the
containing user object is not destroyed while a FIDL message or error event is
being dispatched. It is suitable for single-threaded and object oriented usage
styles.

`fidl::WireClient` can only be used with a single-threaded async dispatcher.
One particular usage of `async::Loop` is creating a single worker thread via
`loop.StartThread()`, and joining that and shutting down the loop via
`loop.Shutdown()` from a different thread. Here, two threads are technically
involved, but this is safe from the perspective of mutual exclusive access, and
`fidl::WireClient` is designed to allow this usage.

`fidl::WireClient` reports errors via the `on_fidl_error` virtual method of the
event handler. User-initiated teardown (e.g. by destroying the client) is not
reported as an error to the event handler.

`fidl::WireClient` does not own the event handler. Instead, the user object
which owns the client may implement the event handling interface, and pass a
borrowed pointer to the client object.

A typical usage of `fidl::WireClient` may look like the following:

```cpp
class MyDevice : fidl::WireAsyncEventHandler<MyProtocol> {
 public:
  MyDevice() {
    client_.Bind(std::move(client_end), dispatcher, /* event_handler */ this);
  }

  void on_fidl_error(fidl::UnbindInfo error) {
    // Handle errors...
  }

  void DoThing() {
    // Capture |this| such that the |MyDevice| instance may be accessed
    // in the callback. This is safe because destroying |client_| silently
    // discards all pending callbacks registered through |Then|.
    client_->Foo(args).Then([this] (fidl::WireUnownedResult<Foo>&) { ... });
  }

 private:
  fidl::WireClient<MyProtocol> client_;
};
```

Notice that there's nothing in particular that is needed when `MyDevice` is
destroyed - the client binding will be torn down as part of the process, and
the threading checks performed by `WireClient` are sufficient to prevent this
class of use-after-frees.

#### Additional use-after-free risks with `ThenExactlyOnce`

When a client object is destroyed, pending callbacks registered through
`ThenExactlyOnce` will asynchronously receive a cancellation error. Care is
needed to ensure any lambda captures are still alive. For example, if an object
contains a `fidl::WireClient` and captures `this` in async method callbacks,
then manipulating the captured `this` within the callbacks after destroying the
object will lead to use-after-free. To avoid this, use `Then` to register
callbacks when the receiver object is destroyed together with the client. Using
the `MyDevice` example above:

```cpp
void MyDevice::DoOtherThing() {
  // Incorrect:
  client_.Foo(request).ThenExactlyOnce([this] (fidl::WireUnownedResult<Foo>& result) {
    // If |MyDevice| is destroyed, this pending callback will still run.
    // The captured |this| pointer will be invalid.
  });

  // Correct:
  client_.Foo(request).Then([this] (fidl::WireUnownedResult<Foo>& result) {
    // The callback is silently dropped if |client_| is destroyed.
  });
}
```

You may use `ThenExactlyOnce` when the callback captures objects that need to be
used exactly once, such as when propagating errors from a client call used as
part of fulfilling a server request:

```cpp
class MyServer : public fidl::WireServer<FooProtocol> {
 public:
  void FooMethod(FooMethodRequestView request, FooMethodCompleter::Sync& completer) override {
    bar_.client->Bar().ThenExactlyOnce(
        [completer = completer.ToAsync()] (fidl::WireUnownedResult<Bar>& result) {
          if (!result.ok()) {
            completer.Reply(result.status());
            return;
          }
          // ... more processing
        });
  }

 private:
  struct BarManager {
    fidl::WireClient<BarProtocol> client;
    /* Other internal state... */
  };

  std::unique_ptr<BarManager> bar_;
};
```

In the above example, if the server would like to re-initialize `bar_` while
keeping `FooProtocol` connections alive, it may use `ThenExactlyOnce` to
reply a cancellation error when handling `FooMethod`, or introduce retry logic.

### WireSharedClient

`fidl::WireSharedClient` supports [solution #2 (reference
counting)](#solution_2_ref_counting) and [solution #3 (two-phase
shutdown)](#solution_3_two_phase_shutdown). Unlike `WireClient` where destroying
a client immediately guarantees that there are no more **to-user** calls,
destroying a `WireSharedClient` merely initiates asynchronous bindings teardown.
The user may observe the completion of the teardown asynchronously. In turn,
this allows moving or cloning a `WireSharedClient` to a different thread
than the dispatcher thread, and destroying/calling teardown on a client while
there are parallel **to-user** calls (e.g. a response callback). Those two
actions will race (the response callback might be canceled if the client is
destroyed early enough), but `WireSharedClient` will never make any more to-user
calls once it notifies its teardown completion.

There are two ways to observe teardown completion:

* [Owned event handler](#owned_event_handler)
* [Custom teardown observer](#custom_teardown_observer)

#### Owned event handler

Transfer the ownership of an event handler to the client as an implementation of
`std::unique_ptr<fidl::WireAsyncEventHandler<Protocol>>` when binding the
client. After teardown is complete, the event handler will be destroyed. It is
safe to destroy the user objects referenced by any client callbacks from within
the event handler destructor.

Here is an example showing this pattern:

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/client_shared/main.cc" region_tag="owned_event_handler" %}
```

#### Custom teardown observer

Provide an instance of `fidl::AnyTeardownObserver` to the bindings.
The observer will be notified when teardown is complete. There are several
ways to create a teardown observer:

- `fidl::ObserveTeardown` takes an arbitrary callable and wraps it in a
  teardown observer:

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/client_shared/main.cc" region_tag="custom_callback" %}
```

- `fidl::ShareUntilTeardown` takes a `std::shared_ptr<T>`, and arranges the
  binding to destroy its shared reference after teardown:

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/client_shared/main.cc" region_tag="share_until_teardown" %}
```

Users may create custom teardown observers that work with other pointer types
e.g. `fbl::RefPtr<T>`.

`WireSharedClient` caters to systems where business logic states are managed by
a framework (drivers are one example, where the driver runtime is the managing
framework). In this case, the bindings runtime and the framework will co-own the
user objects: the bindings runtime will inform the framework it has surrendered
all user object references, at which point the framework can schedule the
destruction of the user objects, modulo other ongoing asynchronous teardown
processes happening to the same group of objects. An asynchronous teardown does
not require synchronizing across arbitrary to-user calls, and helps to prevent
deadlocks.

The pattern of initiating teardown first, then destroying the user objects after
teardown complete is sometimes called *two-phase shutdown*.

### Simple decision tree

When in doubt, here are some rules of thumb to follow when deciding which client
type to use:

- If your app is single-threaded, use `WireClient`.

- If your app is multi-threaded but consists of multiple single-threaded
  dispatchers, and you can guarantee that each client is only bound and
  destroyed on their respective single dispatcher thread: still able to use
  `WireClient`.

- If your app is multi-threaded and the FIDL clients are not guaranteed to be
  destroyed on their respective dispatcher threads: use `WireSharedClient` and
  take on the two-phase shutdown complexity.

## Server-side threading

`fidl::WireClient` and `fidl::WireSharedClient` both teardown the binding when
they destruct. Different from clients, there is no RAII type on the server side
that teardown the binding. The rationale is that servers in simpler applications
are created in response to a connection attempt made by a client, and often stay
around continuing processing client requests until the client closes their
endpoint. When the application is shutting down, the user may shutdown the async
dispatcher which then synchronously tears down all server bindings associated
with it.

As applications grow more complex however, there are scenarios for proactively
shutting down server implementation objects, which involves tearing down the
server bindings. Drivers for example need to stop relevant servers when the
device is removed.

There are two ways a server could voluntarily teardown the binding on their end:

- `fidl::ServerBindingRef::Close` or `fidl::ServerBindingRef::Unbind`.
- `SomeCompleter::Close` where `SomeCompleter` is a method completer provided to
  a server method handler.

For a precise reference of their semantics, refer to their documentation in the
[server header][server-header].

All methods above only initiate teardown, hence may safely race with in-progress
operations or parallel **to-user** calls (e.g. method handlers). Consequently,
the trade-off is that we need to practice some care in maintaining the lifetime
of the server implementation object. There are two cases:

* [Initiating teardown from the single dispatcher
  thread](#initiating_teardown_from_the_single_dispatcher_thread)
* [Initiating teardown from an arbitrary
  thread](#initiating_teardown_from_an_arbitrary_thread)

### Initiating teardown from the single dispatcher thread

When the async dispatcher (`async_dispatcher_t*`) passed to `fidl::BindServer`
only has one thread backing it, and teardown is initiated from that thread (e.g.
from within a server method handler or a task running on this dispatcher), then
the binding will not make any calls on the server object after `Unbind`/`Close`
returns. It is safe to destroy the server object at this point.

If the unbound handler is specified, the binding _will_ make one final
**to-user** call that is the unbound handler soon after, usually at the next
iteration of the event loop. The unbound handler has the following signature:

```cpp
// |impl| is the pointer to the server implementation.
// |info| contains the reason for binding teardown.
// |server_end| is the server channel endpoint.
// |Protocol| is the type of the FIDL protocol.
void OnUnbound(ServerImpl* impl, fidl::UnbindInfo info,
               fidl::ServerEnd<Protocol> server_end) {
  // If teardown is manually initiated and not due to an error, |info.ok()| will be true.
  if (info.ok())
    return;
  // Handle errors...
}
```

If the server object was destroyed earlier on, the callback must not access the
`impl` variable as it now points to invalid memory.

### Initiating teardown from an arbitrary thread

If the application cannot guarantee that the teardown is always initiated from
the single dispatcher thread, then there could be ongoing **to-user** calls
during teardown. To prevent use-after-free, we may implement a similar two-phase
shutdown pattern as found on the client side.

Suppose a server object is allocated on the heap for each incoming connection
request:

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/server/main.cc" region_tag="create_server" %}
```

We could destroy the server object at the end of the `unbound_handler` callback.
Here the code accomplishes this by deleting the heap allocated server at the end
of the callback.

```cpp
class EchoImpl {
 public:
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/server/main.cc" region_tag="bind_server" %}

  // Later, when the server is shutting down...
  void Shutdown() {
    binding_->Unbind();  // This stops accepting new requests.
    // The server is destroyed asynchronously in the unbound handler.
  }
};
```

Note: if the server is always managed in a `unique_ptr` or a `shared_ptr`, you
may pass the smart pointer directly to `fidl::BindServer` which has the relevant
special casing for these pointer types. The pointer is destroyed after the
unbound handler returns. The example above manually arranges this to show that
other custom teardown logic may also be inserted.

The two-phase shutdown pattern is necessary to accommodate the possibility of
parallel server method handler calls at the point of initiating teardown. The
bindings runtime will call the unbound handler after these **to-user** calls
return. In particular, if a server method handler takes a long time to return,
the unbinding procedure could be delayed by an equal amount of time. It is
recommended to offload long running handler work to a thread pool and make the
reply asynchronously via `completer.ToAsync()`, thus ensuring prompt return of
method handlers and timely unbinding. The reply will be discarded if the server
binding has been torn down in the meantime.

## Interacting with the async dispatcher

All asynchronous request/responses handling, event handling, and error handling
are done through the `async_dispatcher_t*` provided when binding a client or
server. With the exception of shutting down the dispatcher, you can expect that
**to-user** calls will be executed on a dispatcher thread, and not nested within
other user code (no reentrancy issues).

If you shutdown the dispatcher while there are any active bindings, the teardown
may be completed on the thread executing shutdown. As such, you must not take
any locks that could be taken by the teardown observers provided to
`fidl::WireShareClient` or the unbound handler provided to `fidl::BindServer`
while executing `async::Loop::Shutdown`/`async_loop_shutdown`. (You should
probably ensure that no locks are held around shutdown anyway since it joins all
dispatcher threads, which may take locks in user code).

[client-header]: https://cs.opensource.google/fuchsia/fuchsia/+/main:sdk/lib/fidl/cpp/wire/include/lib/fidl/cpp/wire/client.h
[server-header]: https://cs.opensource.google/fuchsia/fuchsia/+/main:sdk/lib/fidl/cpp/wire/include/lib/fidl/cpp/wire/channel.h
[wire-client]: https://cs.opensource.google/fuchsia/fuchsia/+/main:sdk/lib/fidl/cpp/wire/include/lib/fidl/cpp/wire/client.h?q=llcpp%2Fclient.h%20WireClient&ss=fuchsia%2Ffuchsia
[wire-shared-client]: https://cs.opensource.google/fuchsia/fuchsia/+/main:sdk/lib/fidl/cpp/wire/include/lib/fidl/cpp/wire/client.h?q=llcpp%2Fclient.h%20WireSharedClient&sq=&ss=fuchsia%2Ffuchsia
