# `fpromise::promise<>` User Guide

Welcome! You probably dislike writing code in C++ that describes multi-step
asynchronous operations.

`fpromise::promise<>`
[[1](/sdk/lib/fit-promise/include/lib/fpromise/promise.h)]
makes this a bit easier. This guide covers common problems in asynchronous
control flow programming and offers common usage patterns that solve those
problems in the `fpromise::promise<>` library.

## What makes asynchronous code challenging?

Within the `fpromise::promise<>` library an asynchronous task is defined as one that
is made up of multiple *synchronous* blocks of code with explicit suspend
points.

When defining an asynchronous task, there must be solutions for the following
problems:

1. **Expressing the flow of control**: how is the *sequence* of synchronous
   blocks and how data flows between them expressed? How is this done in an
   understandable way?

2. **Management of state & resources**: what intermediate state is needed to
   support task execution, and what external resources must be captured? How is
   this expressed and how is it done safely?

## Terminology
* `fpromise::promise<>` is a move-only object made up of a collection of lambdas or
  callbacks that describes an asynchronous task that eventually produces a
  value or an error.
* a *handler function* is a callback provided at promise creation.
* a *continuation function* is a callback provided to various *methods of
  continuation* on an existing promise.
* a `fpromise::executor` is responsible for scheduling and executing promises.
  Promises do not run until their ownership has been transferred to a
  `fpromise::executor`. At this point the executor is responsible for its scheduling
  and execution.
* `fpromise::context` is optionally passed to handler and continuation functions to
  gain access to the `fpromise::executor` and to low-level suspend and resume
  controls.

## Building & executing your first `fpromise::promise<>`

Let's write a simple promise.

```cpp
#include <lib/fpromise/promise.h>

...
fpromise::promise<> p = fpromise::make_promise([] {
  // This is a handler function.

  auto world_is_flat = AssessIfWorldIsFlat();
  if (world_is_flat) {
    return fpromise::error();
  }
  return fpromise::ok();
});
```

`p` now contains a promise that describes a simple task.

In order to run the promise, it must be scheduled it on an implementation of
`fpromise::executor`. The most commonly used executor is an `async::Executor`
[[2](/zircon/system/ulib/async/include/lib/async/cpp/executor.h)]
which schedules callbacks on an `async_dispatcher_t`. For the purposes of
testing and exploration, there is also `fpromise::single_threaded_executor` and its
associated method `fpromise::run_single_threaded()`
[[3](/sdk/lib/fit-promise/include/lib/fpromise/single_threaded_executor.h#72)]
which is used here.

```cpp
// When a promise is scheduled, the `fpromise::executor` takes ownership of it.
fpromise::result<> result = fpromise::run_single_threaded(std::move(p));
assert(result.is_ok());
```

## Building a more complex `fpromise::promise<>`

### Return, error types & resolution states

As mentioned above, the template arguments for `fpromise::promise<>` represent the
return and error types:

```cpp
fpromise::promise<ValueType, ErrorType>
```

The error type can be omitted and it will take the default error type of `void`
(e.g. `fpromise::promise<MyValueType>` is equivalent to `fpromise::promise<MyValueType,
void>`).

During execution, a promise must eventually reach one of the following states:

* Success: the handler function or the last continuation function (see below)
  has returned `fpromise::ok()`.
* Error: the handler function or some continuation function has returned
  `fpromise::error()`, *and* no subsequent continuation function has intercepted it.
* Abandoned: the promise was destroyed before resolving to either Success or
  Error.

### `.then()`, `.and_then()`, `.or_else()`: Chaining asynchronous blocks

Often complex tasks can be decomposed into smaller more granular tasks. Each of
these tasks needs to be asynchronously executed, but if there is some
dependency between the tasks, there is a need to preserve them. This can be
achieved through different combinators, such as:

* `fpromise::promise::then()` becomes useful for defining task dependency, as
  execute task 1 then task 2, regardless of task 1's status. The prior task's
  result is received through an argument of type `fpromise::result<ValueType,
  ErrorType>&` or `const fpromise::result<ValueType, ErrorType>&`.

```cpp
auto execute_task_1_then_task_2 =
    fpromise::make_promise([]() -> fpromise::result<ValueType, ErrorType> {
      ...
    }).then([](fpromise::result<ValueType, ErrorType>& result) {
      if (result.is_ok()) {
        ...
      } else {  // result.is_error()
        ...
      }
    });
```

* `fpromise::promise::and_then()` becomes useful for defining task dependency only
  in the case of task 1's success. The prior task's result is received through
  an argument of type `ValueType&` or `ValueType&`.

```cpp
auto execute_task_1_then_task_2 =
    fpromise::make_promise([]() { ... }).and_then([](ValueType& success_value) {
      ...
    });
```

* `fpromise::promise::or_else()` becomes useful for defining task dependency only in
  the case of task 1's failure. The prior task's result is received through an
  argument of type `ErrorType&` or `const ErrorType&`.

```cpp
auto execute_task_1_then_task_2 =
    fpromise::make_promise([]() { ... }).or_else([](ErrorType& failure_value) {
      ...
    });
```

### `fpromise::join_promises()` & `fpromise::join_promise_vector()`: Executing in parallel

Sometimes, multiple promises can be executed with no dependencies between them,
but the aggregate result is a dependency of the next asynchronous step. In this
case, `fpromise::join_promises()` and `fpromise::join_promise_vector()` are used to join
on the results of multiple promises.

`fpromise::join_promises()` is used when each promise is referenced by a variable.
`fpromise::join_promises()` supports heterogeneous promise types. The prior tasks'
results are received through an argument of type `std::tuple<...>&` or `const
std::tuple<...>&`.

```cpp
auto DoImportantThingsInParallel() {
  auto promise1 = FetchStringFromDbAsync("foo");
  auto promise2 = InitializeFrobinatorAsync();
  return fpromise::join_promises(std::move(promise1), std::move(promise2))
      .and_then([](std::tuple<fpromise::result<std::string>,
                              fpromise::result<Frobinator>>& results) {
        return fpromise::ok(std::get<0>(results).value() +
                       std::get<1>(results).value().GetFrobinatorSummary());
      });
}
```

`fpromise::join_promise_vector()` is used when the promises are stored in
`std::vector<>`. This has the added constraint that all promises must be
homogeneous (be of the same type). The prior tasks' results are received
through an argument of type `std::vector<fpromise::result<ValueType, ErrorType>>&`
or `const std::vector<fpromise::result<ValueType, ErrorType>>&`.

```cpp
auto ConcatenateImportantThingsDoneInParallel() {
  std::vector<fpromise::promise<std::string>> promises;
  promises.push_back(FetchStringFromDbAsync("foo"));
  promises.push_back(FetchStringFromDbAsync("bar"));
  return fpromise::join_promise_vector(std::move(promises))
      .and_then([](std::vector<fpromise::result<std::string>>& results) {
        return fpromise::ok(results[0].value() + "," + results[1].value());
      });
}
```

### `return fpromise::make_promise()`: Chaining or branching by returning new promises

It may become useful to defer the decision of which promises to chain together
until runtime. This method is in contrast with chaining that is performed
syntactically (through the use of consecutive `.then()`, `.and_then()` and
`.or_else()` calls).

Instead of returning a `fpromise::result<...>` (using `fpromise::ok` or `fpromise::error`),
the handler function may return a new promise, which will be evaluated after the
handler function returns.

```cpp
fpromise::make_promise(...)
  .then([] (fpromise::result<>& result) {
    if (result.is_ok()) {
      return fpromise::make_promise(...); // Do work in success case.
    } else {
      return fpromise::make_promise(...); // Error case.
    }
  });
```

This pattern is also useful to decompose what could be a long promise into
smaller readable chunks, such as by having a continuation function return the
result of `DoImportantThingsInParallel()` from the example above.

Note: See the gotcha "Handlers / continuation functions can return ..." below.

### Declaring and keeping intermediate state alive

Some tasks require state be kept alive only so long as the promise itself is
either pending or executing. This state is not suited to be moved into any
given lambda due to its need to be shared, nor is it appropriate to transfer
ownership to a longer-lived container due to a desire for its lifecycle to be
coupled to the promise.

Although not the only solution, usage of both `std::unique_ptr<>` and
`std::shared_ptr<>` are common patterns:

#### `std::unique_ptr<>`

```cpp
fpromise::promise<> MakePromise() {
  struct State {
    int i;
  };
  // Create a single std::unique_ptr<> container for an instance of State and
  // capture raw pointers to the state in the handler and continuations.
  //
  // Ownership of the underlying memory is transferred to a lambda passed to
  // `.inspect()`. |state| will die when the returned promise is resolved or is
  // abandoned.
  auto state = std::make_unique<State>();
  state->i = 0;
  return fpromise::make_promise([state = state.get()] { state->i++; })
      .and_then([state = state.get()] { state->i--; })
      .inspect([state = std::move(state)](const fpromise::result<>&) {});
}
```

#### `std::shared_ptr<>`

```cpp
fpromise::promise<> MakePromise() {
  struct State {
    int i;
  };
  // Rely on shared_ptr's reference counting to destroy |state| when it is safe
  // to do so.
  auto state = std::make_shared<State>();
  state->i = 0;
  return fpromise::make_promise([state] { state->i++; }).and_then([state] {
    state->i--;
  });
}
```

### `fpromise::scope`: Abandoning promises to avoid memory safety violations

`fpromise::scope` becomes useful to tie the lifecycle of a `fpromise::promise<>` to a
resource in memory. For example:

```cpp
#include <lib/fpromise/scope.h>

class A {
 public:
  fpromise::promise<> MakePromise() {
    // Capturing |this| is dangerous: the returned promise will be scheduled
    // and executed in an unknown context. Use |scope_| to protect against
    // possible memory safety violations.
    //
    // The call to `.wrap_with(scope_)` abandons the promise if |scope_| is
    // destroyed. Since |scope_| and |this| share the same lifecycle, it is safe
    // to capture |this|.
    return fpromise::make_promise([this] {
             // |foo_| is critical to the operation!
             return fpromise::ok(foo_.Frobinate());
           })
        .wrap_with(scope_);
  }

 private:
  Frobinator foo_;
  fpromise::scope scope_;
};

void main() {
  auto a = std::make_unique<A>();
  auto promise = a->MakePromise();
  a.reset();
  // |promise| will not run any more, even if scheduled, protected access to the
  // out-of-scope resources.
}
```

### `fpromise::sequencer`: Blocking a promise on a separate promise's completion

TODO: you can .wrap_with(sequencer) to block this promise on the completion of
the last promise wrapped with the same sequencer object

```cpp
#include <lib/fpromise/sequencer.h>
// TODO
```

### `fpromise::bridge`: integrating with callback-based async functions

TODO: fpromise::bridge is useful to chain continuation off a callback-based async
function

```cpp
#include <lib/fpromise/bridge.h>
// TODO
```

### `fpromise::bridge`: decoupling execution of a single chain of continuation

TODO: fpromise::bridge is also useful to decouple one chain of continuation into two
promises that can be executed on different `fpromise::executor` instances

## Common gotchas

### Sequences of `and_then` or `or_else` must have compatible types

When building promises using `and_then`, each successive continuation may have
a different *ValueType* but must have the same *ErrorType* because `and_then`
forwards prior errors without consuming them.

When building promises using `or_else`, each successive continuation may have a
different *ErrorType* but must have the same *ValueType* because `or_else`
forwards prior values without consuming them.

To change types in the middle of the sequence, use `then` to consume the prior
result and produce a new result of the desired type.

The following example does not compile because the error type returned by the
last `and_then` handler is incompatible with the prior handler's result.

```cpp
auto a = fpromise::make_promise([] {
  // returns fpromise::result<int, void>
  return fpromise::ok(4);
}).and_then([] (const int& value) {
  // returns fpromise::result<float, void>
  return fpromise::ok(value * 2.2f);
}).and_then([] (const float& value) {
  // ERROR!  Prior result had "void" error type but this handler returns const
  // char*.
  if (value >= 0)
    return fpromise::ok(value);
  return fpromise::error("bad value");
}
```

Use `then` to consume the result and change its type:

```cpp
auto a = fpromise::make_promise([] {
  // returns fpromise::result<int, void>
  return fpromise::ok(4);
}).and_then([] (const int& value) {
  // returns fpromise::result<float, void>
  return fpromise::ok(value * 2.2f);
}).then([] (const fpromise::result<float>& result) -> fpromise::result<float, const char*> {
  if (result.is_ok() && result.value() >= 0)
    return fpromise::ok(value);
  return fpromise::error("bad value");
}
```

### Handlers / continuation functions can return `fpromise::result<>` or a new `fpromise::promise<>`, not both

You may wish to write a handler that can return a `fpromise::promise<>` in one
conditional branch, and a `fpromise::ok()` or `fpromise::error()` in another. This is
illegal because there is no way for the compiler to cast a `fpromise::result<>` to a
`fpromise::promise<>`.

The workaround is to return a `fpromise::promise<>` that resolves to the result you
want:

```cpp
auto a = fpromise::make_promise([] {
  if (condition) {
    return MakeComplexPromise();
  }
  return fpromise::make_ok_promise(42);
});
```

### Continuation signatures

Have you seen an error message like this?

```
../../sdk/lib/fit-promise/include/lib/fpromise/promise_internal.h:342:5: error: static_assert failed "The provided handler's last argument was expected to be of type V& or const V& where V is the prior result's value type and E is the prior result's error type.  Please refer to the combinator's documentation for
 a list of supported handler function signatures."
```

or:

```
../../sdk/lib/fit-promise/include/lib/fpromise/promise.h:288:5: error: static_assert failed due to requirement '::fpromise::internal::is_continuation<fpromise::internal::and_then_continuation<fpromise::promise_impl<fit::function_impl<16, false, fpromise::result<fuchsia::modular::storymodel::StoryModel, void> (fpromise::context &)> >, (lambda at ../../src/modular/bin/sessionmgr/story/model/ledger_story_model_storage.cc:222:17)>, void>::value' "Continuation type is invalid.  A continuation is a callable object with this signature: fpromise::result<V, E>(fpromise::context&)."
```

This most likely means that one of the continuation functions has a signature
that isn't valid. The valid signatures for different continuation functions are
shown below:

For `.then()`:

```cpp
.then([] (fpromise::result<V, E>& result) {});
.then([] (const fpromise::result<V, E>& result) {});
.then([] (fpromise::context& c, fpromise::result<V, E>& result) {});
.then([] (fpromise::context& c, const fpromise::result<V, E>& result) {});
```

For `.and_then()`:

```cpp
.and_then([] (V& success_value) {});
.and_then([] (const V& success_value) {});
.and_then([] (fpromise::context& c, V& success_value) {});
.and_then([] (fpromise::context& c, const V& success_value) {});
```

For `.or_else()`:

```cpp
.or_else([] (E& error_value) {});
.or_else([] (const E& error_value) {});
.or_else([] (fpromise::context& c, E& error_value) {});
.or_else([] (fpromise::context& c, const E& error_value) {});
```

For `.inspect()`:

```cpp
.inspect([] (fpromise::result<V, E>& result) {});
.inspect([] (const fpromise::result<V, E>& result) {});
```

### Captures and Argument Lifecycle

Promises are composed of handler and continuation functions that are usually
lambdas. Care must be taken when constructing lambda capture lists to avoid
capturing memory that is will not be valid when the handler or continuation in
question executes.

For example, this promise captures memory that is guaranteed to be invalid
by the time Foo() returns (and thus, when the returned promise is scheduled and
executed).

```cpp
fpromise::promise<> Foo() {
  int i;
  return fpromise::make_promise([&i] {
    i++;  // |i| is only valid within the scope of Foo().
  });
}
```

Instances in real code are more nuanced. A slightly less obvious example:

```cpp
fpromise::promise<> Foo() {
  return fpromise::make_promise(
      [i = 0] { return fpromise::make_promise([&i] { i++; }); });
}
```

`fpromise::promise` eagerly destroys handler and continuation functions: the
outer-most handler will be destroyed once it returns the inner-most handler.
See "Declaring and keeping intermediate state alive" above for the correct
pattern to use in this case.

## >>> sections to write

* converting from one error type to another
* fpromise::bridge
* common gotchas:
captured state lifecycle
