# Thread safe asynchronous code

Writing correct asynchronous programs with multiple threads requires care in
C++. Here we describe a particular pattern that helps avoid errors, and which
will integrate well with the C++ FIDL bindings and component runtime.

## Background

### Asynchronous runtimes

The [async][async-readme] library defines the *interface*
for initiating asynchronous operations on Fuchsia. It defines an opaque
`async_dispatcher_t` type, and associated functions.

There are several *implementations* of this dispatcher interface. A popular one
is [`async_loop_t`][async-loop] and its C++ wrapper
[`async::Loop`][async-loop-cpp]. Libraries that performs asynchronous work
generally should not know what is the concrete implementation. Instead they
would call functions over the `async_dispatcher_t*` interface.

### Thread safety

The reader should familiarize themselves with the terminology around
[thread safety][thread-safety] if needed. See [CppCon 2018: Geoffrey Romer
“What do you mean "thread-safe"?”] for a definition of thread safety that is
endorsed by Google's C++ team.

A program that upholds thread safety avoids data races: broadly, reading and
writing the same data without a defined ordering between those operations (see
precise definition of a [data race][data-race] in the C++ standard). These races
are a source of errors because they lead to undefined behavior at run-time.

An individual C++ type also has categorizations around thread-safety. Referring
common practice interpretations from [abseil][abseil-thread-safety]:

- A C++ object is *thread-safe* if concurrent usages does not cause data races.
- A C++ object is *thread-unsafe* if any concurrent usage may cause data races.

One may wrap a thread-unsafe type with *synchronization primitives* e.g. mutexes
to make it thread-safe. This is called adding *external synchronization*. Doing
so adds overhead, and not all users will use that type concurrently. Hence it's
common for a library to be thread-unsafe by default, and require the user to add
synchronization if desired. Such types may have comments like the following:

```c++
// This class is thread-unsafe. Methods require external synchronization.
class SomeUnsafeType { /* ... */ };
```

## Thread safety in asynchronous code

Achieving thread safety gets more subtle in asynchronous code due to the
presence of callbacks. Consider the following snippet:

```c++
// |CsvParser| asynchronously reads from a file, and parses the contents as
// comma separated values.
class CsvParser {
 public:
  void Load() {
    reader_.AsyncRead([this] (std::string data) {
      values_ = Parse(data);
    });
  }

  std::vector<std::string> Parse(const std::string& data);

 private:
  FileReader reader_;
  std::vector<std::string> values_;
};
```

`AsyncRead` will complete the work in the background, then call the lambda
specified as the callback function when the work completes. Because the lambda
captures `this`, it is commonly referred to as an "upcall": the `reader_` that
is owned by an instance of `CsvParser` makes a call to the owner.

Let's consider how to avoid races between this callback and the destruction of
`CsvParser`. Adding a mutex in `CsvParser` won't help, because the mutex would
be destroyed if `CsvParser` is destroyed. One may require that `CsvParser` must
always be reference counted, but that results in an opinionated API and tends to
recursively cause everything referenced by `CsvParser` to also be reference
counted.

If we ensure that there is always a defined ordering between the destruction of
`CsvParser` and the invocation of the callback, then the race condition is
avoided. On Fuchsia, the callback is typically scheduled on an
`async_dispatcher_t` object (termed *dispatcher* in short). A common pattern is
to use a single threaded dispatcher:

- Use an `async::Loop` as the dispatcher implementation.
- Only run one thread to service the loop.
- Only destroy upcall targets on that thread, and cancel future upcalls at the
  same time. For example, destroy the `CsvParser` within a task posted to that
  dispatcher.

Since the same thread invokes asynchronous callbacks and destroys the instance,
there must be a defined ordering between those operations.

The general case of the above pattern is to ensure *synchronized access*: every
access (including construction and destruction) of an object will observe the
side-effects of previous accesses. In other literature about threading, you may
see the term *synchronized access* always associated with locking, for example
taking a mutex lock before accessing an object. In Fuchsia C++, locks alone
would not be sufficient as discussed above, and we use properties of the async
dispatcher to achieve synchronized access, such that user code does not have to
take locks. The next section will go into detail.

## Synchronized dispatchers {#synchronized-dispatcher}

A *synchronized dispatcher* is one where posted tasks are run in order, and each
task will observe the side-effects from previous tasks.

Because objects dealing with asynchronous logic are accessed from dispatchers,
one cannot also access the object from arbitrary threads, as the dispatcher
might be concurrently accessing the same object, resulting in data races. In
fact, one must always access the object from a single dispatcher associated with
that object. The dispatcher must also ensure ordering between operations. We
call such dispatchers *synchronized dispatchers*. There are two ways for a
dispatcher to qualify as *synchronized*:

### Support sequences

A dispatcher may promise that tasks posted on that dispatcher always run with a
strict ordering. Such dispatchers are said to support *sequences*: sequential
execution domains which runs a series of tasks where one task will observe all
side-effects from previous tasks, but where the underlying execution may hop
from one thread to another.

Synchronized driver dispatchers (e.g.
[`fdf::SynchronizedDispatcher`][fdf-dispatcher]) are an example of dispatchers
that support sequences. See [driver threading model][driver-threading-model]. On
the other hand, `async::Loop` does not support sequences, as the user may call
`Loop::StartThread` many times to introduce multiple threads that race to
execute tasks in that loop.

### Stay single threaded

If the dispatcher does not support sequences, then code running on
tasks posted to that dispatcher are ordered if that dispatcher is only serviced
by a single thread, for example, a single-threaded `async::Loop`.

In summary, either the dispatcher supports sequences in which case the object
must be used on that sequence, or the code runs on a single dispatcher thread
and the object must be used on that thread. *Use* covers construction,
destruction, and calling instance methods.

Synchronized dispatchers are a unit of [concurrency][concurrency]: tasks posted
to the same synchronized dispatcher are never run concurrently alongside one
another. Tasks posted to different synchronized dispatchers may potentially run
concurrently alongside one another.

### Check for synchronized dispatchers {#check-synchronized}

The `async` library offers a `BasicLockable` type,
[`async::synchronization_checker`][synchronization-checker]. You may call
`.lock()` or lock the checker using a `std::lock_guard` whenever a function
requires *synchronized access*. Doing so checks that the function is called from
a dispatcher with such a guarantee, without actually taking any locks. If the
check fails, the program will panic. It is recommended that thread-unsafe types
check for synchronization at runtime by carrying a checker. Here is a full
example:

```cpp
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/cpp/synchronization_checker/main.cc" region_tag="synchronization_checker" adjust_indentation="auto" exclude_regexp="^TEST|^}" %}
```

`fidl::Client` is another example of types that check for synchronized access at
runtime: destroying a `fidl::Client` on a non-dispatcher thread will lead to a
panic. There are other C++ classes in the Fuchsia code base that do the same.
They will usually highlight this with a comment such as the following:

```c++
// This class is thread-unsafe. Instances must be used and managed from a
// synchronized dispatcher.
class SomeAsyncType { /* ... */ };
```

See [C++ FIDL threading guide][cpp-threading-guide] for a concrete discussion of
this scenario when using FIDL bindings.

### Discard callbacks during destruction

You may have noticed that for the `ChannelReader` example above to work, the
callback passed to `wait_.Begin(...)` must be silently discarded, instead of
called with some error, if `ChannelReader` is destroyed. Indeed the
[documentation][async-wait] on `async::WaitOnce` mentions that it "automatically
cancels the wait when it goes out of scope".

During destruction, some C++ objects would discard the registered callbacks if
those have yet to be called. These kind of APIs are said to guarantee *at most
once delivery*. `async::Wait` and `async::Task` are examples of such objects.
This style works well when the callback references a single receiver that owns
the wait/task, i.e. the callback is an upcall. These APIs are typically also
thread-unsafe and requires the aforementioned *synchronized access*.

Other objects will always call the the registered callback exactly once, even
during destruction. Those calls would typically provide an error or status
indicating cancellation. They are said to guarantee *exactly once delivery*.

One should consult the corresponding documentation when using an
asynchronous API to understand the cancellation semantics.

It is possible to convert an *exactly once* API into an *at most once* API by
discarding the upcall if the object making the upcalls is already destroyed.
[`closure-queue`][closure-queue] is a library that implements this idea;
destroying a `ClosureQueue` will discard unexecuted callbacks scheduled on that
queue.

### Use an object belonging to a different synchronized dispatcher

To maintain synchronized access, one may manage and use a group of objects on
the same synchronized dispatcher. Those objects can synchronously call into one
another without breaking the synchronization checks. A special case of this is
an application that runs everything on a single `async::Loop` with a single
thread, typically called the "main thread".

More complex applications may have multiple synchronized dispatchers. When
individual objects must be used from their corresponding synchronized
dispatcher, a question arises: how does one object call another object if they
are associated with different dispatchers?

A time-tested approach is to have the objects send messages between one another,
as opposed to synchronously calling their instance methods. Concretely, this
could mean that if object `A` needs to do something to object `B`, `A` would
post an asynchronous task to `B`'s dispatcher. The task (usually a lambda
function) may then synchronously use `B` because it is already running under
`B`'s dispatcher and will be synchronized with other tasks that use `B`.

When tasks are posted to a different dispatcher, it's harder to safely discard
them when the receiver object goes out of scope. Here are some approaches:

- One may use [`async_patterns::DispatcherBound`][dispatcher-bound] to both own
  and make calls to a child object that lives on a different synchronized
  dispatcher.
- One may use [`async_patterns::Receiver`][receiver] to let other objects make
  calls on their objects, without forcing an ownership relationship. The calls
  are silently canceled if the receiver is destroyed.
<!-- TODO(fxbug.dev/119641): Document other async_patterns helpers when they
     land. -->
- One may reference count the objects, and pass a weak pointer to the posted
  task. The posted task should do nothing if the pointer is expired.

Golang is a popular [example][golang] that baked this principle into their
language design.

## Prior arts

Lightweight mechanisms of ensuring a set of tasks execute one after the other,
without necessarily starting operating system threads, is a recurring theme:

- The Chromium project defines a similar sequence concept: [Threading and
tasks in Chrome][chrome].
- The Java Platform added [virtual threads][java].

[async-readme]: /zircon/system/ulib/async/README.md
[async-loop]: /zircon/system/ulib/async-loop/include/lib/async-loop/loop.h
[async-loop-cpp]: /zircon/system/ulib/async-loop/include/lib/async-loop/cpp/loop.h
[async-wait]: /zircon/system/ulib/async/include/lib/async/cpp/wait.h
[concurrency]: https://slikts.github.io/concurrency-glossary/?id=concurrent-order-independent-vs-sequential
[dispatcher-bound]: /sdk/lib/async_patterns/cpp/dispatcher_bound.h
[receiver]: /sdk/lib/async_patterns/cpp/receiver.h
[driver-threading-model]: /docs/concepts/drivers/driver-dispatcher-and-threads.md#threading-model
[fdf-dispatcher]: /sdk/lib/driver/runtime/include/lib/fdf/cpp/dispatcher.h
[thread-safety]: https://en.wikipedia.org/wiki/Thread_safety
[data-race]: http://eel.is/c++draft/intro.races#21
[abseil-thread-safety]: https://abseil.io/blog/20180531-regular-types#data-races-and-thread-safety-properties
[cpp-threading-guide]: /docs/development/languages/fidl/tutorials/cpp/topics/threading.md
[closure-queue]: /zircon/system/ulib/closure-queue/include/lib/closure-queue/closure_queue.h
[chrome]: https://chromium.googlesource.com/chromium/src/+/master/docs/threading_and_tasks.md
[java]: https://openjdk.org/jeps/425
[golang]: https://go.dev/blog/codelab-share
[synchronization-checker]: /zircon/system/ulib/async/include/lib/async/cpp/sequence_checker.h
[CppCon 2018: Geoffrey Romer “What do you mean "thread-safe"?”]: https://youtube.com/watch?v=s5PCh_FaMfM
