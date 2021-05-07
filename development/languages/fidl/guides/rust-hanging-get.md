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
let watch_foo_stream = HangingGetStream::new(Box::new(move || Some(proxy.watch_foo())));
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

[hanging-get-pattern]: /docs/concepts/api/fidl.md#hanging-get
[hanging-get-stream-impl]: https://fuchsia-docs.firebaseapp.com/rust/async_utils/hanging_get/client/type.HangingGetStream.html
[rust-abortable]: https://docs.rs/futures/0.3.5/futures/future/struct.Abortable.html
[fasync-timeout-ext]: https://fuchsia-docs.firebaseapp.com/rust/fuchsia_async/trait.TimeoutExt.html
