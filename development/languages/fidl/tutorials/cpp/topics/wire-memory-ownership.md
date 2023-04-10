# Memory ownership of wire domain objects

This document provides an overview of the tools available to manage memory when
using the wire domain objects from the new C++ bindings.

## Wire domain objects trade safety for performance

The wire domain objects of the C++ bindings, also termed "wire types", are thin
types whose memory layout matches that of the
[FIDL wire format][fidl-wire-format] of their source `*.fidl` types as closely
as possible. Convenience wrappers and accessors are provided, but they do not
attempt to encapsulate the structure of the underlying wire format. With this
design, fields oftentimes may be accessed in-place and serialized with less
copying. Understanding the layout of the [FIDL wire format][fidl-wire-format]
would greatly assist in interacting with wire types. In particular, wire types
do not own their out-of-line children, as defined by the FIDL wire format.

The wire types keep unowned references to objects using:

* For a string, `fidl::StringView`.
* For a vector of objects, `fidl::VectorView`.
* For an out-of-line object, `fidl::ObjectView`.
* For a table named `Foo`, a `Foo` class that wraps a vector view, referencing
  a collection of envelope headers represented by a `fidl::WireTableFrame<Foo>`,
  which in turn references the fields of the table.
* For a union named `Foo`, a `Foo` class that stores the ordinal and an
  [envelope header][envelope-header]:
  * Small fields <= 4 bytes use the *inline representation* and are stored
    inside the header.
  * Larger fields > 4 bytes use the *out-of-line representation* are stored
    out-of-line and referenced by the header.
* For a `MyMethod` request message, a `MyMethodRequestView` that wraps a pointer
  to the request. This definition is scoped to the containing
  `fidl::WireServer<Protocol>` class.

With the exception of a union with an active member using the *inline
representation*, these are non-owning views that only keep a reference and do
not manage the object lifetime. The lifetime of the objects must be managed
externally. This means that the referenced objects must outlive the views.

Copying a view will alias the references within the view. Moving a view is
equivalent to a copy and does not clear the source view.

For memory safety reasons tables are immutable. The default constructor for a
table returns an empty table. To create a table with fields you must use a
builder. The members of tables may be mutable but you can't add or remove
members after creation.

For simplicity and consistency with tables, unions are also immutable. Their
default constructor puts them in the absent state. It's a runtime error to send
an absent union unless the union is marked `optional` in the library definition.
To get a union `Foo` with a member `bar` call the static factory function
`Foo::WithBar(...)`. The arguments are either the value (for values inlined into
the envelope), a `fidl::ObjectView` of the value (for larger values) or an
arena and constructor arguments for the value.

### fidl::StringView

Defined in [lib/fidl/cpp/wire/string_view.h][string-view-src]

Holds a reference to a variable-length string stored within the buffer. C++
wrapper of **fidl_string**. Does not own the memory of the contents.

`fidl::StringView` may be constructed by supplying the pointer and number of
UTF-8 bytes (excluding trailing `\0`) separately. Alternatively, one could pass
a C++ string literal, or any value that implements `[const] char* data()`
and `size()`. The string view would borrow the contents of the container.

It is memory layout compatible with **fidl_string**.

### fidl::VectorView\<T\>

Defined in [lib/fidl/cpp/wire/vector_view.h][vector-view-src]

Holds a reference to a variable-length vector of elements stored within the
buffer. C++ wrapper of **fidl_vector**. Does not own the memory of elements.

`fidl::VectorView` may be constructed by supplying the pointer and number of
elements separately. Alternatively, one could pass any value that supports
[`std::data`](https://en.cppreference.com/w/cpp/iterator/data), such as a
standard container, or an array. The vector view would borrow the contents of
the container.

It is memory layout compatible with **fidl_vector**.

### fidl::Array\<T, N\>

Defined in [lib/fidl/cpp/wire/array.h][array-src]

Owns a fixed-length array of elements. It is similar to `std::array<T, N>`, but
designed to be memory layout compatible with FIDL arrays, and standard-layout.
The destructor closes handles if applicable e.g. a `fidl::Array` of handles.

### Message views in request/response handlers

The request handlers in server implementations receive a view of the request
message. They do not own the buffer backing the view.

The data behind the request view is only guaranteed to live until the end of the
method handler. Therefore, if the server wishes to make a reply asynchronously,
and the reply makes use of the request message, the user needs to copy relevant
fields from the request message to owned storage:

```c++
// A FIDL method called "StartGame".
virtual void StartGame(
    StartGameRequestView request, StartGameCompleter::Sync completer) {
  // Suppose the request has a `foo` field that is a string view,
  // we need to copy it to an owning type e.g. |std::string|.
  auto foo = std::string(request->foo.get());
  // Make an asynchronous reply using the owning type.
  async::PostDelayedTask(
      dispatcher_,
      [foo = std::move(foo), completer = completer.ToAsync()]() mutable {
        // As an example, we simply echo back the string.
        completer.Reply(fidl::StringView::FromExternal(foo));
      });
}
```

Similarly, the response handlers and event handlers passed to a client also only
receive a view of the response/event message. Copying to user-owned storage is
required if they need to be accessed after the handler returns:

```c++
// Suppose the response has a `bar` field that is a table:
//
// type Bar = table {
//     1: a uint32;
//     2: b string;
// };
//
// we need to copy the table to an owned type by copying each element.
struct OwnedBar {
  std::optional<uint32_t> a;
  std::optional<std::string> b;
};
// Suppose we are in a class that has a `OwnedBar bar_` member.
client_->MakeMove(args).Then([](fidl::WireUnownedResult<TicTacToe::MakeMove>& result) {
  assert(result.ok());
  auto* response = result.Unwrap();
  // Create an owned value and copy the wire table into it.
  OwnedBar bar;
  if (response->bar.has_a())
    bar.a = response->bar.a();
  if (response->bar.has_b())
    bar.b = std::string(response->bar.b().get());
  bar_ = std::move(bar);
});
```

## Creating wire views and objects

At the high level there are two ways to create wire objects: using an arena, or
unsafely borrowing memory. Using arenas is the safer and is performant in most
cases. Unsafely borrowing memory is very prone to errors and corruptions, but
may be called for when one needs to control every single byte of allocation.

### Create wire objects using arenas

Wire objects integrate with an arena interface `fidl::AnyArena`, typically in
their constructors or factory functions, which lets users inject custom
allocation behavior. The FIDL runtime provides a standard implementation of the
arena interface, `fidl::Arena`. The arena manages the lifetime of the allocated
wire objects (it owns the objects). As soon as the arena is destroyed, all the
objects it has allocated are deallocated and their destructors are called.

`fidl::Arena` is defined in [lib/fidl/cpp/wire/arena.h][arena-src].

The objects are first allocated within a buffer which belongs to the arena (an
inline field of the arena). The default size of the buffer is 512 bytes. A
different size can be selected using `fidl::Arena<size>`. By tweaking the size,
one could make all their allocated objects created during a request fit on the
stack, thus avoiding costlier heap allocations.

When the inline buffer is full, the arena allocates more buffers on the heap.
Each of these buffers is 16 KiB. If one needs an object larger than 16 KiB, the
arena will use a bespoke buffer with enough space to accommodate the necessary
size.

The standard pattern for using the arena is:

* Define a local variable arena of type `fidl::Arena`.
* Allocate objects using the arena.
* Send the allocated objects by making a FIDL method call or making a reply
  via a completer.
* Upon exiting the function scope, all of these local variables are
  automatically de-allocated.

<!-- TODO(fxbug.dev/103483): Write an example for the above pattern, and link
to that. -->

The arena needs to outlive all the view types that refer to objects within it.

See [wire domain object tutorial][wire-domain-object-tutorial] for annotated
examples of how to use the arena in practice to build tables, unions, etc.

### Create wire views borrowing unowned data

In addition to the managed allocation strategies, it is also possible to
directly create pointers to memory unowned by FIDL. This is discouraged, as it
is easy to accidentally create use-after-free bugs. Most view types offer an
`FromExternal` factory function to explicitly borrow pointers to objects that
are not managed by the FIDL runtime.

To create an `ObjectView` from an external object using
`fidl::ObjectView<T>::FromExternal`:

```c++
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/domain_objects/advanced.cc" region_tag="wire-external-object" adjust_indentation="auto" exclude_regexp="^TEST|^}" %}
```

To create a `VectorView` from an external collection using
`fidl::VectorView<T>::FromExternal`:

```c++
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/domain_objects/advanced.cc" region_tag="wire-external-vector" adjust_indentation="auto" exclude_regexp="^TEST|^}" %}
```

To create a `StringView` from an external buffer using
`fidl::StringView::FromExternal`:

```c++
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/domain_objects/advanced.cc" region_tag="wire-external-string" adjust_indentation="auto" exclude_regexp="^TEST|^}" %}
```

A `StringView` can also be created directly from string literals without using
`FromExternal`. This is safe since [string literals][string-literals] have
[static lifetime][static-storage-duration].

```c++
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/domain_objects/advanced.cc" region_tag="wire-external-string-literal" adjust_indentation="auto" exclude_regexp="^TEST|^}" %}
```

To create a wire union borrowing a member stored externally, pass an
`ObjectView` referencing the member to the corresponding union factory function:

```c++
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/domain_objects/advanced.cc" region_tag="wire-union-external-member" adjust_indentation="auto" exclude_regexp="^TEST|^}" %}
```

Wire tables store references to a `fidl::WireTableFrame<SomeTable>`, which is
responsible for keeping track of field metadata. To create a wire table
borrowing an external frame, pass an `ObjectView` to an `ExternalBuilder`.

An example of setting a field that is inlined into the frame:

```c++
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/domain_objects/advanced.cc" region_tag="wire-table-external-frame-inline" adjust_indentation="auto" exclude_regexp="^TEST|^}" %}
```

An example of setting a field that is stored out-of-line from the frame:

```c++
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/cpp/domain_objects/advanced.cc" region_tag="wire-table-external-frame-out-of-line" adjust_indentation="auto" exclude_regexp="^TEST|^}" %}
```

<!-- xrefs -->
[arena-src]: /sdk/lib/fidl/cpp/wire/include/lib/fidl/cpp/wire/arena.h
[array-src]: /sdk/lib/fidl/cpp/wire/include/lib/fidl/cpp/wire/array.h
[envelope-header]: /reference/fidl/language/wire-format/README.md#envelopes
[string-view-src]: /sdk/lib/fidl/cpp/wire/include/lib/fidl/cpp/wire/string_view.h
[vector-view-src]: /sdk/lib/fidl/cpp/wire/include/lib/fidl/cpp/wire/vector_view.h
[fidl-wire-format]: /reference/fidl/language/wire-format/README.md
[wire-domain-object-tutorial]: /development/languages/fidl/tutorials/cpp/basics/domain-objects.md#using-wire
[string-literals]: https://en.cppreference.com/w/cpp/language/string_literal
[static-storage-duration]: https://en.cppreference.com/w/c/language/static_storage_duration
