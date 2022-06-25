# LLCPP memory management

This document provides an overview of the tools available to manage memory when
using the wire domain objects from the LLCPP bindings.

## Wire domain objects

The domain objects from the LLCPP bindings, termed "wire types", are tightly
coupled to the memory layout of the FIDL wire format used by the current
application for performance. Convenience wrappers and accessors are provided,
but they do not attempt to encapsulate the structure of the underlying wire
format. Understanding the layout of the [FIDL wire format][fidl-wire-format]
would greatly assist in interacting with wire types. In particular, wire types
do not own their out-of-line children, as defined by the FIDL wire format.

LLCPP keeps unowned references to objects using:

* For a string, `fidl::StringView`.
* For a vector of objects, `fidl::VectorView`.
* For an out-of-line object, `fidl::ObjectView`.
* For a table named `Foo`, a `Foo` class that wraps a vector view, referencing
  a collection of envelope headers represented by a `fidl::WireTableFrame<Foo>`,
  which in turn references the fields of the table.
* For a union named `Foo`, a `Foo` class that stores the ordinal and an envelope
  header, which references out-of-line fields.
* For a `MyMethod` request message, a `MyMethodRequestView` that wraps a pointer
  to the request. This definition is scoped to the containing
  `fidl::WireServer<Protocol>` class.

These are non-owning views that only keep a reference and do not manage the
object lifetime. The lifetime of the objects must be managed externally. That
means that the referenced objects must outlive the views.

For memory safety reasons tables are immutable. The default constructor for a
table returns an empty table. To create a table with fields you must use a
builder. The members of tables may be mutable but you can't add or remove
members after creation.

For simplicity and consistency with tables, unions are also immutable. Their
default constructor puts them in the absent state. It's a runtime error to send
an absent union unless the union is marked `optional` in the library definition.
To get a union `Foo` with a member `bar` call the static factory function
`Foo::WithBar(...)`.The arguments are either the value (for values inlined into
the envelope), a `fidl::ObjectView` of the value (for larger values) or an
arena and constructor arguments for the value.

### fidl::StringView

Defined in [lib/fidl/llcpp/string_view.h](/sdk/lib/fidl/llcpp/include/lib/fidl/llcpp/string_view.h)

Holds a reference to a variable-length string stored within the buffer. C++
wrapper of **fidl_string**. Does not own the memory of the contents.

`fidl::StringView` may be constructed by supplying the pointer and number of
UTF-8 bytes (excluding trailing `\0`) separately. Alternatively, one could pass
a C++ string literal, or any value that implements `[const] char* data()`
and `size()`. The string view would borrow the contents of the container.

It is memory layout compatible with **fidl_string**.

### fidl::VectorView\<T\>

Defined in [lib/fidl/llcpp/vector_view.h](/sdk/lib/fidl/llcpp/include/lib/fidl/llcpp/vector_view.h)

Holds a reference to a variable-length vector of elements stored within the
buffer. C++ wrapper of **fidl_vector**. Does not own the memory of elements.

`fidl::VectorView` may be constructed by supplying the pointer and number of
elements separately. Alternatively, one could pass any value that supports
[`std::data`](https://en.cppreference.com/w/cpp/iterator/data), such as a
standard container, or an array. The vector view would borrow the contents of
the container.

It is memory layout compatible with **fidl_vector**.

### fidl::Array\<T, N\>

Defined in [lib/fidl/llcpp/array.h](/sdk/lib/fidl/llcpp/include/lib/fidl/llcpp/array.h)

Owns a fixed-length array of elements.
Similar to `std::array<T, N>` but intended purely for in-place use.

It is memory layout compatible with FIDL arrays, and is standard-layout.
The destructor closes handles if applicable e.g. it is an array of handles.

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
  // Create an owned value and copy the LLCPP table into it.
  OwnedBar bar;
  if (response->bar.has_a())
    bar.a = response->bar.a();
  if (response->bar.has_b())
    bar.b = std::string(response->bar.b().get());
  bar_ = std::move(bar);
});
```

## Creating LLCPP views and objects

### Create LLCPP objects using `fidl::Arena`

The FIDL arena (`fidl::Arena`) can allocate LLCPP objects. It
manages the lifetime of the allocated LLCPP objects (it owns the objects). As
soon as the arena is deleted, all the objects it has allocated are
deallocated and their destructors are called.

The FIDL arena is defined in
[lib/fidl/llcpp/arena.h](/sdk/lib/fidl/llcpp/include/lib/fidl/llcpp/arena.h).

The objects are first allocated within a buffer which belongs to the arena
(this is a field of the arena). The default size of the buffer is 512 bytes.
A different size can be selected using `fidl::Arena<size>`.

When this buffer is full, the arena allocates more buffers on the heap. Each
of these buffers is 16 KiB (if it needs to allocate an object bigger, it will
use a buffer which fit the bigger size).

The standard pattern for using the arena is:

*   Define a local variable arena of type `fidl::Arena`.
*   Allocate objects using the arena.
*   Send the allocated objects by making a FIDL method call or making a reply
    via a completer.
*   Leave the function; everything is deallocated.

Example for a table, which is more subtle than other types since it requires
allocating a separate data structure, `fidl::WireTableFrame<SomeTable>`, to keep
track of field metadata, in addition to the fields themselves we use a builder pattern:

Example:

```c++
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/unittests/main.cc" region_tag="tables" adjust_indentation="auto" exclude_regexp="^TEST|^}" %}
```

### Create LLCPP views of unowned data

In addition to the managed allocation strategies, it is also possible to
directly create pointers to memory unowned by FIDL. This is discouraged, as it
is easy to accidentally create use-after-free bugs. `FromExternal` exists to
explicitly mark pointers to FIDL-unowned memory.

To create an `ObjectView` from an external object using
`fidl::ObjectView::FromExternal`.

```c++
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/unittests/main.cc" region_tag="external-object" adjust_indentation="auto" exclude_regexp="^TEST|^}" %}
```

To create a `VectorView` from an external collection using
`fidl::VectorView::FromExternal`.

```c++
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/unittests/main.cc" region_tag="external-vector" adjust_indentation="auto" exclude_regexp="^TEST|^}" %}
```

To create a `StringView` from an external buffer using
`fidl::StringView::FromExternal`.

```c++
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/unittests/main.cc" region_tag="external-string" adjust_indentation="auto" exclude_regexp="^TEST|^}" %}
```

A `StringView` can also be created directly from string literals without using
`FromExternal`.

```c++
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/unittests/main.cc" region_tag="stringview-assign" adjust_indentation="auto" exclude_regexp="^TEST|^}" %}
```

<!-- xrefs -->
[fidl-wire-format]: /docs/reference/fidl/language/wire-format/README.md
