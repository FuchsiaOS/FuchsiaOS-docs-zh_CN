# Threading tips in tests

Note: This guide is applicable to both the new driver framework (DFv2) and the
legacy version of the driver framework (DFv1).

Quite a few asynchronous types used in driver writing are thread-unsafe and
they [check][check] that they are always used from their associated synchronized
dispatcher, to ensure memory safety, for example:

* `{fdf,component}::OutgoingDirectory`
* `{fdf,fidl}::Client`, `{fdf,fidl}::WireClient`
* `{fdf,fidl}::ServerBinding`, `{fdf,fidl}::ServerBindingGroup`

If you are testing driver async objects containing these types, using them from
the wrong execution context will lead to a crash, where the stack trace involves
"synchronization_checker". This is a safety feature to prevent silent data
corruption. Here are some tips to avoid the crashes.

## Write single-threaded tests

The most straightforward approach is to execute the test assertions and use
those objects from the same thread, typically the main thread:

Example involving `async::Loop`:

```cpp {:.devsite-disable-click-to-copy}
TEST(MyRegister, Read) {
  async::Loop loop{&kAsyncLoopConfigAttachToCurrentThread};
  // For illustration purposes, this is the thread-unsafe type being tested.
  MyRegister reg{loop.dispatcher()};
  // Use the object on the current thread.
  reg.SetValue(123);
  // Run any async work scheduled by the object, also on the current thread.
  ASSERT_OK(loop.RunUntilIdle());
  // Read from the object on the current thread.
  EXPECT_EQ(obj.GetValue(), 123);
}
```

Example involving `fdf::TestSynchronizedDispatcher` on the main thread:

```cpp {:.devsite-disable-click-to-copy}
TEST(MyRegister, Read) {
  fdf::TestSynchronizedDispatcher test_driver_dispatcher_{
      fdf::kDispatcherDefault};
  // For illustration purposes, this is the thread-unsafe type being tested.
  MyRegister reg{dispatcher.dispatcher()};
  // Use the object on the current thread.
  reg.SetValue(123);
  // Run any async work scheduled by the object, also on the current thread.
  ASSERT_OK(fdf_testing_run_until_idle());
  // Read from the object on the current thread.
  EXPECT_EQ(obj.GetValue(), 123);
  ASSERT_OK(dispatcher.Stop());
}
```

Note that `fdf::TestSynchronizedDispatcher` may also be driven by driver runtime
managed thread pool if a driver needs to make synchronous calls. Objects
associated with driver dispatchers backed by the thread pool should not be
accessed from the main thread. As a simple rule, if the associated dispatcher is
not started with `kDispatcherDefault`, then it may not be running on the main
thread.

When using the async objects from a single thread, their contained
`async::synchronization_checker` will not panic.

## Call blocking functions

The single-threaded way breaks down if you need to call a blocking function
which blocks on the dispatcher processing some messages. If you first call the
blocking function and then run the loop, that will deadlock because the loop
will not be run until the blocking function returns, and the blocking function
will not return unless the loop is run.

To call blocking functions, you need a way to run that function and run the loop
on different threads. In addition, the blocking function should not directly
access the object associated with the dispatcher without synchronization,
because that may race with the loop thread.

To address both concerns, you can wrap the thread-unsafe async object in a
[`async_patterns::TestDispatcherBound`][test-dispatcher-bound], which ensures
that all accesses to the wrapped object happen on its associated dispatcher.

Example involving `async::Loop`, reusing the `MyRegister` type from earlier:

```cpp {:.devsite-disable-click-to-copy}
// Let's say this function blocks and then returns some value we need.
int GetValueInABlockingWay();

TEST(MyRegister, Read) {
  // Configure the loop to register itself as the dispatcher for the
  // loop thread, such that the |MyRegister| constructor may use
  // `async_get_default_dispatcher()` to obtain the loop dispatcher.
  async::Loop loop{&kAsyncLoopConfigNoAttachToCurrentThread};
  loop.StartThread();

  // Construct the |MyRegister| on the loop thread.
  async_patterns::TestDispatcherBound<MyRegister> reg{
      loop.dispatcher(), std::in_place};

  // Schedule a |SetValue| call on the loop thread and wait for it.
  reg.SyncCall(&MyRegister::SetValue, 123);

  // Call the blocking function on the main thread.
  // This will not deadlock, because we have started a loop thread
  // earlier to process messages for the |MyRegister| object.
  int value = GetValueInABlockingWay();
  EXPECT_EQ(value, 123);

  // |GetValue| returns a value; |SyncCall| will proxy that back.
  EXPECT_EQ(reg.SyncCall(&MyRegister::GetValue), 123);
}
```

In this example, access to the `MyRegister` object happens on its corresponding
`async::Loop` thread. This in turn frees the main thread to make blocking calls.
When the main thread would like to interact with `MyRegister`, it needs to do so
indirectly using `SyncCall`.

Another common pattern in tests is to set up FIDL servers on a separate thread
with `TestDispatcherBound` and have the test fixture class used on the main
test thread. The `TestDispatcherBound` objects will become members of the test
fixture class.

Example involving `fdf::TestSynchronizedDispatcher`:

In drivers, often the blocking work itself happens inside a driver. For example,
the blocking work may involve a synchronous FIDL call made by the driver, over a
FIDL protocol that is faked out during testing. In the following example, the
`BlockingIO` class represents the driver, and the `FakeRegister` class
represents a fake implementation of some FIDL protocol used by `BlockingIO`.
When `FakeRegister` does not speak FIDL over driver transports, an `async::Loop`
is a good way to isolate it to separate thread such that we can make the
blocking call from the main thread.

```cpp {:.devsite-disable-click-to-copy}
// Here is the bare skeleton of a driver object that makes a synchronous call.
class BlockingIO {
 public:
  // Let's say this function blocks to update the value stored in a
  // |FakeRegister|.
  void SetValueInABlockingWay(int value);

  /* Other details omitted */
};

TEST(BlockingIO, Read) {
  // Configure the loop to register itself as the dispatcher for the
  // loop thread, such that the |FakeRegister| constructor may use
  // `async_get_default_dispatcher()` to obtain the loop dispatcher.
  async::Loop register_loop{&kAsyncLoopConfigNoAttachToCurrentThread};
  register_loop.StartThread();

  // Construct the |FakeRegister| on the loop thread.
  async_patterns::TestDispatcherBound<FakeRegister> reg{
      register_loop.dispatcher(), std::in_place};

  // Start a driver dispatcher for |BlockingIO| and register it as the
  // dispatcher for the main thread.
  fdf::TestSynchronizedDispatcher driver_dispatcher{fdf::kDispatcherDefault};

  // Construct the |BlockingIO| on the main thread.
  BlockingIO io{driver_dispatcher.dispatcher()};

  // Call the blocking function. The |register_loop| will respond to it in the
  // background.
  io.SetValueInABlockingWay(123);

  // Check the value from the fake.
  // |GetValue| returns an |int|; |AsyncCall| will proxy that back.
  EXPECT_EQ(fdf::WaitFor(reg.AsyncCall(&FakeRegister::GetValue).ToFuture()), 123);
}
```

When the dispatcher of the driver object is backed by the main thread, there is
no need to go through a `TestDispatcherBound`. We can safely use the
`BlockingIO` driver object, including making the `SetValueInABlockingWay` call,
from the main thread.

When the `FakeRegister` fake object lives on the `register_loop` thread, we need
to use a `TestDispatcherBound` to safely interact with it from the main thread.
Instead of `SyncCall` which blocks without doing anything else, here we find the
`fdf::WaitFor(object.AsyncCall(&SomeMethod).ToFuture())` pattern. `fdf::WaitFor`
can wait on a `std::future` while concurrently running driver dispatchers backed
by the main thread. Running those dispatchers will be necessary if `SomeMethod`
involves talking to an async object associated with one of those dispatchers. In
some driver dispatcher configurations, all dispatchers will be backed by threads
from a managed thread pool, in which case you can use `SyncCall` while the
thread pool works in the background. The `AsyncCall` and `fdf::WaitFor`
combination still works in those configurations, so you may default to that if
in doubt.

Blocking on the future directly, i.e.
`object.AsyncCall(&SomeMethod).ToFuture().get()`, is equivalent to `SyncCall`.

### Granularity of DispatcherBound objects

The following guide applies to both `TestDispatcherBound` and its production
counterpart, [`DispatcherBound`][dispatcher-bound].

When serializing access to an object to occur over a specific synchronized
dispatcher, it's important to consider which other objects need to be used from
that same dispatcher. It can be more effective to combine both objects inside
the same `DispatcherBound`.

For instance, if you're using a `component::OutgoingDirectory`, which
synchronously calls into FIDL server implementations like adding a binding to a
`fidl::ServerBindingGroup`, you must ensure that both objects are on the same
dispatcher.

If you only put the `OutgoingDirectory` inside a `[Test]DispatcherBound` to work
around its synchronization checker, but leave the `ServerBindingGroup` somewhere
else e.g. on the main thread, you'll get a crash when the `OutgoingDirectory`
object calls into the `ServerBindingGroup` from the dispatcher thread, tripping
the checker in `ServerBindingGroup`.

To solve this problem, you can place the `OutgoingDirectory` and the objects it
references, such as the `ServerBindingGroup` or any server state, inside a
bigger object, and then put that object in a `DispatcherBound`. This way, both
the `OutgoingDirectory` and the `ServerBindingGroup` will be used from the same
synchronized dispatcher, and you won't experience any crashes. You can see an
[example test][fastboot] that uses this technique.

In general, it's helpful to divide your classes along concurrency boundaries. By
doing so, you'll ensure that all the objects that need to be used on the same
dispatcher are synchronized, preventing potential crashes or data races.

## See also

* [Isolated state async C++ RFC][rfc], which explains the theoretical framework
  behind the synchronization checking.
* [Thread safe asynchronous code][thread-safe-async], which has guidance for
  general production code not just tests.

[check]: /development/languages/c-cpp/thread-safe-async.md#check-synchronized
[thread-safe-async]: /development/languages/c-cpp/thread-safe-async.md
[dispatcher-bound]: /sdk/lib/async_patterns/cpp/dispatcher_bound.h
[test-dispatcher-bound]: /sdk/lib/async_patterns/testing/cpp/dispatcher_bound.h
[fastboot]: https://cs.opensource.google/fuchsia/fuchsia/+/973929b93c3a5e7609ed9e7443756b32c08140e5:src/firmware/lib/fastboot/test/fastboot-test.cc;l=891-936
[rfc]: https://fuchsia-review.googlesource.com/c/fuchsia/+/802588
