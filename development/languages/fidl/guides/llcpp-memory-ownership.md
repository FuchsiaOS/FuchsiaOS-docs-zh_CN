# LLCPP Memory Management

This document provides an overview of the tools available to manage memory when
using the LLCPP bindings.

## Memory ownership {#memory-ownership}

LLCPP keeps references to objects using:

*   `fidl::StringView` for a string.
*   `fidl::VectorView` for a vector of objects.
*   `fidl::ObjectView` for a reference to an object.

These are non-owning views that only keep a reference and do not manage the object lifetime. The
lifetime of the objects must be managed externally. That means that the referenced objects must
outlive the views.

In particular, LLCPP generated types do not own their out-of-line children, as defined by the FIDL
wire format.

### Create LLCPP object using the FidlAllocator

The FIDL allocator (`fidl::FidlAllocator`) can allocate LLCPP objects. It manages the lifetime of
the allocated LLCPP objects (it owns the objects). As soon as the allocator is deleted, all the
objects it has allocated are deallocated and their destructors are called.

The FIDL allocator is defined in [lib/fidl/llcpp/fidl_allocator.h](/zircon/system/ulib/fidl/include/lib/fidl/llcpp/fidl_allocator.h).

The objects are first allocated within a buffer which belongs to the allocator (this is a field of
the allocator). The default size of the buffer is 512 bytes. A different size can be selected using
`fidl::FidlAllocator<size>`.

When this buffer is full, the allocator allocates more buffers on the heap. Each of these buffers
is 16 KiB (if it needs to allocate an object bigger, it will use a buffer which fit the bigger
size).

The standard patern for using the allocator is:

*   Define a local variable allocator of type fidl::FidlAllocator.
*   Allocate objects using the allocator.
*   Send the allocated objects by making a FIDL method call or making a reply via a completer.
*   Leave the function; everything is deallocated.

Example:

```c++
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/unittests/main.cc" region_tag="tables" adjust_indentation="auto" exclude_regexp="^TEST|^}" %}
```

### Unowned data

In addition to the managed allocation strategies, it is also possible to directly
create pointers to memory unowned by FIDL. This is discouraged, as it is easy to
accidentally create use-after-free bugs. `FromExternal` exists to explicitly mark
pointers to FIDL-unowned memory.

To create an `ObjectView` from an external object using `fidl::ObjectView::FromExternal`.

```c++
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/unittests/main.cc" region_tag="external-object" adjust_indentation="auto" exclude_regexp="^TEST|^}" %}
```

To create a `VectorView` from an external collection using `fidl::VectorView::FromExternal`.

```c++
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/unittests/main.cc" region_tag="external-vector" adjust_indentation="auto" exclude_regexp="^TEST|^}" %}
```

To create a `StringView` from an external buffer using `fidl::StringView::FromExternal`.

```c++
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/unittests/main.cc" region_tag="external-string" adjust_indentation="auto" exclude_regexp="^TEST|^}" %}
```

A `StringView` can also be created directly from string literals without using `FromExternal`.

```c++
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/llcpp/unittests/main.cc" region_tag="stringview-assign" adjust_indentation="auto" exclude_regexp="^TEST|^}" %}
```

### fidl::StringView

Defined in [lib/fidl/llcpp/string_view.h](/zircon/system/ulib/fidl/include/lib/fidl/llcpp/string_view.h)

Holds a reference to a variable-length string stored within the buffer. C++
wrapper of **fidl_string**. Does not own the memory of the contents.

`fidl::StringView` may be constructed by supplying the pointer and number of
UTF-8 bytes (excluding trailing `\0`) separately. Alternatively, one could pass
a C++ string literal, or any value that implements `[const] char* data()`
and `size()`. The string view would borrow the contents of the container.

It is memory layout compatible with **fidl_string**.

### fidl::VectorView\<T\>

Defined in [lib/fidl/llcpp/vector_view.h](/zircon/system/ulib/fidl/include/lib/fidl/llcpp/vector_view.h)

Holds a reference to a variable-length vector of elements stored within the
buffer. C++ wrapper of **fidl_vector**. Does not own the memory of elements.

`fidl::VectorView` may be constructed by supplying the pointer and number of
elements separately. Alternatively, one could pass any value that supports
[`std::data`](https://en.cppreference.com/w/cpp/iterator/data), such as a
standard container, or an array. The vector view would borrow the contents of
the container.

It is memory layout compatible with **fidl_vector**.

### fidl::Array\<T, N\>

Defined in [lib/fidl/llcpp/array.h](/zircon/system/ulib/fidl/include/lib/fidl/llcpp/array.h)

Owns a fixed-length array of elements.
Similar to `std::array<T, N>` but intended purely for in-place use.

It is memory layout compatible with FIDL arrays, and is standard-layout.
The destructor closes handles if applicable e.g. it is an array of handles.
