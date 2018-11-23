
# C++ Language Bindings

This document is a description of the Fuchsia Interface Definition Language
(FIDL) implementation for C++, including its libraries and code generator.

See [Overview][fidl-overview] for more information about FIDL's overall
purpose, goals, and requirements, as well as links to related documents.

This specification builds on the [C Language
Bindings](c.md) and reuses many of its elements where
appropriate.

[TOC]

## Design

### Goals

*   Support encoding and decoding FIDL messages with C++14.
*   Small, fast, efficient.
*   Depend only on a small subset of the standard library.
*   Minimize code expansion through table-driven encoding and decoding.
*   Support two usage styles using different generated code: native and
    idiomatic.
*   All code produced to support the idiomatic style can be stripped out if
    unused.
*   Reuse encoders, decoders, and data tables generated for C language bindings.

### Native Usage Style

*   Optimized to meet the needs of low-level systems programming while providing
    slightly more safety and convenience than the C bindings.
*   Represent data structures whose memory layout coincides with the wire
    format.
*   Support in-place access and construction of FIDL messages.
*   Defer all memory allocation decisions to the client.
*   Code generator only produces type declarations, data tables, and simple
    inline functions.
*   Client is fully responsible for dispatching incoming method calls on
    interfaces (write their own switch statement and invoke argument decode
    functions).
*   Essentially just like C language bindings but using zero-cost C++ features
    such as namespaces, string views, and array containers.
*   Specifically does not use owned handle types such as zx::handle (libzx)
    because they require making non-POD structs with constructors and
    destructors.
    *   This is worth discussing further. Using zx::handle would be convenient
        but it leads to awkward questions such as to whether destruction of FIDL
        structs should be recursive (follow pointers, iterate over array
        members). These are more easily tackled in the idiomatic form, hence it
        is tempting to let the native form be purely POD and avoid destructors.
    *   Could consider making a zx::unowned_handle which lacks a destructor and
        would therefore be POD.

### Idiomatic Usage Style

*   Optimized to meet the needs of high-level service programming.
*   Represent data structures using idiomatic C++ types such as std::vector,
    std::optional, and std::string.
*   Use smart pointers to manage heap allocated objects.
*   Use zx::handle (libzx) to manage handle ownership.
*   Can copy (move) data from in-place buffers to idiomatic heap allocated
    objects.
*   Can copy (move) data from idiomatic heap allocated objects to in-place
    buffers.
*   Code generator produces somewhat more code, including constructors,
    destructors, interface proxies, interface stubs, copy/move functions, and
    conversions to/from native style.
*   Client performs interface dispatch by subclassing a provided stub and
    implementing the virtual methods for each operation.

### Comparison of Usage Styles

<table>
  <tr>
   <td>
   </td>
   <td><strong>C style</strong>
   </td>
   <td><strong>C++ native style</strong>
   </td>
   <td><strong>C++ idiomatic style</strong>
   </td>
  </tr>
  <tr>
   <td><strong>audience</strong>
   </td>
   <td>drivers
   </td>
   <td>drivers
   </td>
   <td>high-level services
   </td>
  </tr>
  <tr>
   <td><strong>abstraction overhead</strong>
   </td>
   <td>almost zero
   </td>
   <td>almost zero
   </td>
   <td>heap allocation, construction, destruction
   </td>
  </tr>
  <tr>
   <td><strong>type safe types</strong>
   </td>
   <td>enums, structs, unions
   </td>
   <td>enums, structs, unions
   </td>
   <td>enums, structs, unions, handles, interfaces
   </td>
  </tr>
  <tr>
   <td><strong>storage</strong>
   </td>
   <td>in-place buffer
   </td>
   <td>in-place buffer
   </td>
   <td>heap
   </td>
  </tr>
  <tr>
   <td><strong>lifecycle</strong>
   </td>
   <td>manual free (POD)
   </td>
   <td>manual free (POD)
   </td>
   <td>automatic free (RAII)
   </td>
  </tr>
  <tr>
   <td><strong>receive behavior</strong>
   </td>
   <td>decode in-place
   </td>
   <td>decode in-place
   </td>
   <td>decode then move to heap
   </td>
  </tr>
  <tr>
   <td><strong>send behavior</strong>
   </td>
   <td>encode in-place
   </td>
   <td>encode in-place
   </td>
   <td>move to buffer then encode
   </td>
  </tr>
  <tr>
   <td><strong>calling interface methods</strong>
   </td>
   <td>manual proxy
   </td>
   <td>manual proxy
   </td>
   <td>call through proxies, register callbacks
   </td>
  </tr>
  <tr>
   <td><strong>implementing interface methods</strong>
   </td>
   <td>manual dispatch
   </td>
   <td>manual dispatch
   </td>
   <td>implement stub object, invoke callbacks
   </td>
  </tr>
  <tr>
   <td><strong>generated code</strong>
   </td>
   <td>structures, data tables, inline functions
   </td>
   <td>structures, data tables, inline functions
   </td>
   <td>structures, data tables, constructors, destructors, proxies, stubs, inline functions
   </td>
  </tr>
  <tr>
   <td><strong>generated code footprint</strong>
   </td>
   <td>small (data tables only)
   </td>
   <td>small (data tables only)
   </td>
   <td>moderate (data tables, constructors, destructors, proxies, and stubs)
   </td>
  </tr>
  <tr>
   <td><strong>friendliness</strong>
   </td>
   <td>surly
   </td>
   <td>aloof
   </td>
   <td>genial
   </td>
  </tr>
</table>

_Point for discussion: given how close the C++ native style is to the C style
(mostly syntactic benefits), we might want to consider deferring its
implementation or finding some other middle ground like in-place friendly
non-POD with weak RAII semantics (don't traverse pointers)._

## Code Generator

### Mapping Declarations

#### Mapping FIDL Types to C++ Types

This is the mapping from FIDL types to C types which the code generator
produces.

<table>
  <tr>
   <td><strong>FIDL</strong>
   </td>
   <td><strong>Native C++ Style</strong>
   </td>
   <td><strong>Idiomatic C++ Style</strong>
   </td>
  </tr>
  <tr>
   <td>bool
   </td>
   <td>bool,
<em>assuming sizeof(bool) ==1</em>
   </td>
   <td>bool
   </td>
  </tr>
  <tr>
   <td>int8
   </td>
   <td>int8_t
   </td>
   <td>int8_t
   </td>
  </tr>
  <tr>
   <td>uint8
   </td>
   <td>uint8_t
   </td>
   <td>uint8_t
   </td>
  </tr>
  <tr>
   <td>int16
   </td>
   <td>int16_t
   </td>
   <td>int16_t
   </td>
  </tr>
  <tr>
   <td>uint16
   </td>
   <td>uint16_t
   </td>
   <td>uint16_t
   </td>
  </tr>
  <tr>
   <td>int32
   </td>
   <td>int32_t
   </td>
   <td>int32_t
   </td>
  </tr>
  <tr>
   <td>uint32
   </td>
   <td>uint32_t
   </td>
   <td>uint32_t
   </td>
  </tr>
  <tr>
   <td>int64
   </td>
   <td>int64_t
   </td>
   <td>int64_t
   </td>
  </tr>
  <tr>
   <td>uint64
   </td>
   <td>uint64_t
   </td>
   <td>uint64_t
   </td>
  </tr>
  <tr>
   <td>float32
   </td>
   <td>float
   </td>
   <td>float
   </td>
  </tr>
  <tr>
   <td>float64
   </td>
   <td>double
   </td>
   <td>double
   </td>
  </tr>
  <tr>
   <td>handle, handle?
   </td>
   <td>zx_handle_t, or perhaps zx::unowned_handle
   </td>
   <td>zx::handle
   </td>
  </tr>
  <tr>
   <td>handle<T>, handle<T>?
   </td>
   <td>zx_handle_t, or perhaps zx::unowned_T
   </td>
   <td>zx::T <em>(subclass of zx::object<T>)</em>
   </td>
  </tr>
  <tr>
   <td>string
   </td>
   <td>fidl::string
   </td>
   <td>std::string
   </td>
  </tr>
  <tr>
   <td>string?
   </td>
   <td>fidl::string
   </td>
   <td>std::optional<std::string>
   </td>
  </tr>
  <tr>
   <td>vector<T>
   </td>
   <td>fidl::vector<T>
   </td>
   <td>std::vector<T>
   </td>
  </tr>
  <tr>
   <td>vector<T>?
   </td>
   <td>fidl::vector<T>
   </td>
   <td>std::optional<std::vector>
   </td>
  </tr>
  <tr>
   <td>array<T>:N
   </td>
   <td>fidl::array<T, N>
   </td>
   <td>std::array<T, N>
   </td>
  </tr>
  <tr>
   <td><em>Interface, Interface?</em>
   </td>
   <td><em>interface named typedef to zx_handle_t</em>
<p>
<em>eg. typedef zx_handle_t Foo;</em>
   </td>
   <td><em>Interface</em>Ptr
   </td>
  </tr>
  <tr>
   <td><em>request<Interface>, request<Interface>?</em>
   </td>
   <td><em>interface_request named typedef to zx_handle_t</em>
<p>
<em>eg. typedef zx_handle_t FooRequest;</em>
   </td>
   <td><em>Interface</em>Request
   </td>
  </tr>
  <tr>
   <td><em>Struct</em>
   </td>
   <td>struct <em>Struct</em>
   </td>
   <td><em>Struct</em>Ptr
   </td>
  </tr>
  <tr>
   <td><em>Struct?</em>
   </td>
   <td>struct <em>Struct*</em>
   </td>
   <td><em>Struct</em>Ptr
   </td>
  </tr>
  <tr>
   <td><em>Union</em>
   </td>
   <td>struct <em>Union</em>
   </td>
   <td><em>Union</em>Ptr
   </td>
  </tr>
  <tr>
   <td><em>Union?</em>
   </td>
   <td><em>struct Union*</em>
   </td>
   <td><em>Union</em>Ptr
   </td>
  </tr>
  <tr>
   <td><em>Enum</em>
   </td>
   <td><em>enum class Foo : data type</em>
   </td>
   <td><em>enum class Foo : data type</em>
   </td>
  </tr>
</table>

#### Mapping FIDL Identifiers to C++ Identifiers

TODO: discuss reserved words, name mangling

#### Mapping FIDL Type Declarations to C++ Types

TODO: discuss generated namespaces, constants, enums, typedefs, encoding tables

## Bindings Library

### Dependencies

Only depends on Zircon system headers, libzx, and a portion of the C and C++
standard libraries.

Does not depend on libftl or libmtl.

### Code Style

To be discussed.

The native bindings library uses C++ standard library style, eg. function names
are lower-case with underscores.

The idiomatic bindings library could use Google C++ style to match FIDL v1.0 but
though this may ultimately be more confusing, especially given style choices in
Zircon so we may prefer to follow the C++ standard library style here as well.

### Native Types

#### fidl::string

```cpp
class string {
   public:
    void init(size_t size, uint8_t* data);
    size_t size() const;
    bool empty() const;
    bool null() const;
    uint8_t* data();
    const uint8_t* data() const;
    uint8_t& operator[](size_t pos);
    const uint8_t& operator[](size_t pos) const;
    iterator begin();
    // etc...

   private:
    fidl_string string_;
};
```

Holds a reference to a variable-length string stored within the buffer. C++
wrapper of **fidl_string**.

No constructor or destructor so this is POD.

#### fidl::vector<T>

```cpp
template <typename T>
class vector {
   public:
    void init(size_t size, T* data);
    size_t size() const;
    bool empty() const;
    bool null() const;
    T* data();
    const T* data() const;
    T& operator[](size_t pos);
    const T& operator[](size_t pos) const;
    iterator begin();
    // etc...

   private:
    fidl_vector vector_;
};
```

Holds a reference to a variable-length vector of elements stored within the
buffer. C++ wrapper of **fidl_vector**.

No constructor or destructor so this is POD.

#### fidl::array<T, N>

```cpp
template <typename T, size_t N>
class array {
   public:
    size_t size() const;
    bool empty() const;
    bool null() const;
    T* data();
    const T* data() const;
    T& operator[](size_t pos);
    const T& operator[](size_t pos) const;
    iterator begin();
    // etc...;

   private:
    T[N] array_;
};

```

Holds a reference to a fixed-length array of elements stored within the buffer.
Similar to std::array<T, N> but intended purely for in-place use.

No constructor or destructor so this is POD.

#### fidl::buffer

```cpp
class buffer {
   public:
    buffer(size_t max_capacity = ZX_MAX_MESSAGE_SIZE);
    ~buffer();

    template <typename T>
    T* append();

    fidl::string append_string(size_t size);
    template <typename T>
    fidl::string append_string(const char* text);

    fidl::vector<T> append_vector(size_t size);

    const uint8_t* data() const;
    size_t size() const;

    zx_status_t encode(const fidl_encoding_table* encoding_table,
                       std::vector<zx_handle_t>* out_handles);

    zx_status_t decode(const fidl_encoding_table* encoding_table,
                       const std::vector<zx_handle_t>& out_handles);
};
```

Helper for constructing messages laid out in depth-first traversal order.
Generally takes care of the messy pointer arithmetic for building messages
in-place.

TBD: It might be even more convenient to make a more specialized encode_buffer
which knows the encoding table of the message being constructed. Maybe do
something similar for incoming messages.

##### Example

```cpp
zx_status_t say_hello(const zx::channel& channel, const char* text,
                      zx::handle token) {
    assert(strlen(text) <= MAX_TEXT_SIZE);

    fidl::buffer buf();

    auto header = buf.append<fidl_message_header>();
    header->transaction_id = 1;
    header->flags = 0;
    header->ordinal = example_Animal_Say_ordinal;

    auto args = buf.append<example::Animal::Say_args>();
    args->text = buf.append_string(text);
    args->token = std::move(token);

    std::vector<zx::handle> handles;
    zx_status_t status =
        buf.encode(example::Animal::Say_args::encoding, &handles);
    if (status == ZX_OK) {
        status =
            channel.write(0, buf.data(), buf.size(),
                          reinterpret_cast<const zx_handle_t*>(handles.data()),
                          handles.size());
    }
    return status;
}
```

### Idiomatic Types

TODO: adopt main ideas from FIDL 1.0

InterfacePtr<T> / interface_ptr<T>?

InterfaceRequest<T> / interface_req<T>?

async waiter

etc...

## Suggested API Improvements over FIDL v1

The FIDL v1 API for calling and implementing FIDL interfaces has generally been
fairly effective so we would like to retain most of its structure in the
idiomatic FIDL v2 bindings. However, there are a few areas that could be
improved.

TODO: actually specify the intended API

### Handling Connection Errors

Handling connection errors systematically has been a cause of concern for
clients of FIDL v1 because method result callbacks and connection error
callbacks are implemented by different parts of the client program.

It would be desirable to consider an API which allows for localized handling of
connection errors at the point of method calls (in addition to interface level
connection error handling as before).

See https://fuchsia-review.git.corp.google.com/#/c/23457/ for one example of how
a client would otherwise work around the API deficiency.

One approach towards a better API may be constructed by taking advantage of the
fact that std::function<> based callbacks are always destroyed even if they are
not invoked (such as when a connection error occurs). It is possible to
implement a callback wrapper which distinguishes these cases and allows clients
to handle them more systematically. Unfortunately such an approach may not be
able to readily distinguish between a connection error vs. proxy destruction.

Alternately we could wire in support for multiple forms of callbacks or for
multiple callbacks.

Or we could change the API entirely in favor of a more explicit Promise-style
mechanism.

There are lots of options here...

TBD (please feel free to amend / expand on this)

[fidl-overview]: ../README.md
