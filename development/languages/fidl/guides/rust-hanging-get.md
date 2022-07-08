# Hanging Gets in Rust

When consuming an API with a [hanging get][hanging-get-pattern], some care must
be taken to avoid losing the reference to the pending FIDL request. Some futures
combinators like [`Abortable`][rust-abortable] and
[`fuchsia_async::TimeoutExt`][fasync-timeout-ext] will drop the pending future,
which causes the reference to the FIDL request to be dropped too. Because
hanging gets are usually implemented in a stateful manner by protocol servers,
it may be invalid to call the hanging get method once a previous future on the
same `Proxy` has been dropped.

To avoid invalidating the `Proxy` when using such combinators, a good pattern is
to wrap the hanging get call in a `stream` by using
[`HangingGetStream`][hanging-get-stream-impl]:

```rust
// When you don't need to write down the type of the result, you can use a
// fn item, which has zero size and is statically dispatched when called.
let watch_foo_stream = HangingGetStream::new(proxy, FooWatcherProxy::watch_foo);
// Also you can use a capturing closure in that case.
let watch_bar_stream = HangingGetStream::new(proxy, |p| p.watch_bar(some_captured_var));

// If you do want to write down the type (for example when embedding this in
// another Future), you can achieve so by storing a fn pointer. A fn pointer
// can be obtained through coercion from a non-capturing closure or a fn item.
// That said, if you use a capturing closure, there is no way to name the type.
let watch_baz_stream: HangingGetStream<BazProxy, Baz> = HangingGetStream::new_with_fn_ptr(proxy, |p| p.watch_baz());
```

Another alternative is using the pattern below to create a stream.

```rust
fn hanging_get_stream(proxy: &FooProxy) -> impl futures::Stream<Item=Result<FooResult, fidl::Error>> + '_ {
    futures::stream::try_unfold(proxy, |proxy| {
       proxy.watch_foo().map_ok(move |watch_result| Some((watch_result, proxy)))
    })
}
```

Dropping a `Stream::next` future is always safe because it will not cause the
underlying FIDL request to be dropped.  If the stream itself is dropped while already waiting for a
response, the response will be ignored.  This is important if a FIDL server doesn't allow
multiple hanging get waiters at once.

[hanging-get-pattern]: /development/api/fidl.md#hanging-get
[hanging-get-stream-impl]: https://fuchsia-docs.firebaseapp.com/rust/async_utils/hanging_get/client/struct.HangingGetStream.html
[rust-abortable]: https://docs.rs/futures/0.3.5/futures/future/struct.Abortable.html
[fasync-timeout-ext]: https://fuchsia-docs.firebaseapp.com/rust/fuchsia_async/trait.TimeoutExt.html
