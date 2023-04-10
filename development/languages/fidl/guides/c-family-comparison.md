# Comparing new C++ and high-level C++ language bindings

[TOC]

## Quick reference

Here's how to recognize if a particular type/function/identifier in C++ code is
part of the new C++ bindings or high-level C++ bindings.

Taking the [`examples.keyvaluestore.baseline`][kvstore] library as example:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/key_value_store/baseline/fidl/key_value_store.test.fidl" exclude_regexp="^//.*" %}
```

Here are how the various FIDL elements will map to in the C++ bindings. Note
that in the table "C++" refers to the new C++ bindings, and applies equally to
both natural domain objects and wire domain objects. "Natural" refers to the
natural domain objects in the new C++ bindings. "Wire" refers to the wire domain
objects in the new C++ bindings.

<div>
<table>
<thead>
  <tr>
    <th>FIDL element</th>
    <th></th>
    <th>C++ natural types</th>
    <th>Comments</th>
  </tr>
</thead>
<tbody>
  <tr>
    <td rowspan="2">Header include</td>
    <td>C++</td>
    <td>#include &lt;fidl/examples.keyvaluestore.baseline/cpp/fidl.h&gt;</td>
    <td>Format is `fidl/library name/cpp/fidl.h`</td>
  </tr>
  <tr>
    <td>HLCPP</td>
    <td>#include &lt;examples/keyvaluestore/baseline/cpp/fidl.h&gt;</td>
    <td>Format is `slash separated library name/cpp/fidl.h`</td>
  </tr>
  <tr>
    <td rowspan="2">The library</td>
    <td>C++</td>
    <td>::examples_keyvaluestore_baseline</td>
    <td rowspan="2">New C++ uses a single level namespace.<br>HLCPP uses nested namespaces.</td>
  </tr>
  <tr>
    <td>HLCPP</td>
    <td>::examples::keyvaluestore::baseline</td>
  </tr>
  <tr>
    <td rowspan="3">Item struct</td>
    <td>Natural</td>
    <td>::examples_keyvaluestore_baseline::Item</td>
    <td rowspan="3">On top of the namespace differences, the wire types are nested under "::wire".</td>
  </tr>
  <tr>
    <td>Wire</td>
    <td>::examples_keyvaluestore_baseline::wire::Item</td>
  </tr>
  <tr>
    <td>HLCPP</td>
    <td>::examples::keyvaluestore::baseline::Item</td>
  </tr>
  <tr>
    <td rowspan="3">WriteError enum</td>
    <td>Natural</td>
    <td>::examples_keyvaluestore_baseline::WriteError</td>
    <td rowspan="3">On top of the namespace differences, the wire types are nested under "::wire".<br>In case of enums and bits, the wire types and natural types are equivalent. There is just an extra type alias.<br></td>
  </tr>
  <tr>
    <td>Wire</td>
    <td>::examples_keyvaluestore_baseline::wire::WriteError</td>
  </tr>
  <tr>
    <td>HLCPP</td>
    <td>::examples::keyvaluestore::baseline::WriteError</td>
  </tr>
  <tr>
    <td rowspan="3">string:128</td>
    <td>Natural</td>
    <td>std::string</td>
    <td></td>
  </tr>
  <tr>
    <td>Wire</td>
    <td>fidl::StringView</td>
    <td></td>
  </tr>
  <tr>
    <td>HLCPP</td>
    <td>std::string</td>
    <td></td>
  </tr>
  <tr>
    <td rowspan="3">vector&lt;byte&gt;:64000</td>
    <td>Natural</td>
    <td>std::vector&lt;uint8_t&gt;</td>
    <td></td>
  </tr>
  <tr>
    <td>Wire</td>
    <td>fidl::VectorView&lt;uint8_t&gt;</td>
    <td></td>
  </tr>
  <tr>
    <td>HLCPP</td>
    <td>std::vector&lt;uint8_t&gt;</td>
    <td></td>
  </tr>
  <tr>
    <td rowspan="2">protocol Store</td>
    <td>C++</td>
    <td>::examples_keyvaluestore_baseline::Store</td>
    <td>A marker type that carries some information about the protocol</td>
  </tr>
  <tr>
    <td>HLCPP</td>
    <td>::examples::keyvaluestore::baseline::Store</td>
    <td>An abstract base class that contains methods in the protocol</td>
  </tr>
  <tr>
    <td rowspan="2">client_end:Store</td>
    <td>C++</td>
    <td>fidl::ClientEnd&lt;Store&gt;
    <td rowspan="2"></td>
  </tr>
  <tr>
    <td>HLCPP</td>
    <td>fidl::InterfaceHandle&lt;Store&gt;</td>
  </tr>
  <tr>
    <td rowspan="2">server_end:Store</td>
    <td>C++</td>
    <td>fidl::ServerEnd&lt;Store&gt;</td>
    <td rowspan="2"></td>
  </tr>
  <tr>
    <td>HLCPP</td>
    <td>fidl::InterfaceRequest&lt;Store&gt;</td>
  </tr>
  <tr>
    <td rowspan="3">Client and server types<br>for the Store protocol<br></td>
    <td>Natural</td>
    <td colspan="2">Client: fidl::Client&lt;Store&gt;<br>Synchronous client: fidl::SyncClient&lt;Store&gt;<br>Server interface: fidl::Server&lt;Store&gt;<br>Event handler interface: fidl::EventHandler&lt;Store&gt;</td>
  </tr>
  <tr>
    <td>Wire</td>
    <td colspan="2">Client: fidl::WireClient&lt;Store&gt;<br>Synchronous client: fidl::WireSyncClient&lt;Store&gt;<br>Server interface: fidl::WireServer&lt;Store&gt;<br>Event handler interface: fidl::WireEventHandler&lt;Store&gt;</td>
  </tr>
  <tr>
    <td>HLCPP</td>
    <td colspan="2">Client: fidl::InterfacePtr&lt;Store&gt;<br>Synchronous client: fidl::SynchronousInterfacePtr&lt;Store&gt;<br>Server interface: Store<br>Event handler interface: N/A. InterfacePtr has setters that take one callback per event declaration.</td>
  </tr>
</tbody>
</table>
</div>

<!-- TODO(fxbug.dev/111281): Replace the canvas reference with key value store
     to align with the above, once the key value store example has landed. -->

Here's the most common way to set up a client:

<div>
  <devsite-selector>
    <!-- C++ (Natural) -->
    <section>
      <h3>C++ (Natural)</h3>
      <pre class="prettyprint lang-cc">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/canvas/baseline/cpp_natural/client/main.cc" region_tag="connect-protocol" %}</pre>
    </section>
    <!-- C++ (Wire) -->
    <section>
      <h3>C++ (Wire)</h3>
      <pre class="prettyprint lang-cc">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/canvas/baseline/cpp_wire/client/main.cc" region_tag="connect-protocol" %}</pre>
    </section>
    <!-- HLCPP -->
    <section>
      <h3>HLCPP</h3>
      <pre class="prettyprint lang-cc">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/canvas/baseline/hlcpp/client/main.cc" region_tag="connect-protocol" %}</pre>
    </section>
  </devsite-selector>
</div>

See the [canvas][canvas] example for the full code listing and explanation.

Here's the most common way to implement a server:

<div>
  <devsite-selector>
    <!-- C++ (Natural) -->
    <section>
      <h3>C++ (Natural)</h3>
      <pre class="prettyprint lang-cc">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/canvas/baseline/cpp_natural/server/main.cc" region_tag="server-impl-short" %}
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/canvas/baseline/cpp_natural/server/main.cc" region_tag="addline-impl-short" %}
    // ...
  }
};</pre>
    </section>
    <!-- C++ (Wire) -->
    <section>
      <h3>C++ (Wire)</h3>
      <pre class="prettyprint lang-cc">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/canvas/baseline/cpp_wire/server/main.cc" region_tag="server-impl-short" %}
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/canvas/baseline/cpp_wire/server/main.cc" region_tag="addline-impl-short" %}
    // ...
  }
};</pre>
    </section>
    <!-- HLCPP -->
    <section>
      <h3>HLCPP</h3>
      <pre class="prettyprint lang-cc">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/canvas/baseline/hlcpp/server/main.cc" region_tag="server-impl-short" %}
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/canvas/baseline/hlcpp/server/main.cc" region_tag="addline-impl-short" %}
    // ...
  }
};</pre>
    </section>
  </devsite-selector>
</div>

See the [canvas][canvas] example for the full code listing and explanation.

## New C++ bindings {#cpp}

The new C++ bindings supports both low-level and high-level use cases, by
offering two families of generated domain objects, and corresponding client and
server APIs that speak those types.

Note: prefer natural types unless optimizing for critical performance and
memory allocation. Refer to the [C++ tutorials][cpp-prefer-natural].

### Natural types

*   Optimized to meet the needs of high-level service programming.
*   Represent data structures using idiomatic C++ types such as `std::vector`,
    `std::optional`, and `std::string`.
*   Use smart pointers to manage heap allocated objects.
*   Use `zx::handle` to manage handle ownership.
*   Can convert data between their wire (e.g. `fidl::StringView`) and natural
    type representations (e.g. `std::string`).

### Wire types

*   Optimized to meet the needs of low-level systems programming while providing
    slightly more safety and features than the C bindings.
*   Represent data structures whose memory layout coincides with the wire
    format, i.e. satisfying C++ Standard Layout. This opens the door to
    in-place encoding and decoding.
*   Generated structures are views of an underlying buffer; they do not own
    memory.
*   Support in-place access of FIDL messages.
*   Provide fine-grained control over memory allocation.
*   Use owned handle types such as `zx::handle`. Note that since generated
    structures are views of an underlying buffer, a parent structure will only
    own child handles if it also owns their underlying buffer. For example, a
    FIDL struct owns all the handles stored inline, but a FIDL vector of structs
    containing handles will be represented as a vector view, which will not own
    the out-of-line handles.

### Client and server APIs

*   Code generator produces more code compared to the C bindings. This includes
    constructors, destructors, copy/move functions, conversions between domain
    object families, protocol client implementations, and pure virtual server
    interfaces.
*   Users implement a server by sub-classing a provided server interface and
    overriding the pure virtual methods for each operation.
*   Clients supporting sync and async calls, and sync and async event handling.
*   Requires C++17 or above.

Refer to the [New C++ tutorial][cpp-tutorial] to get started.

## High-level C++ bindings

<<../../../../_common/_hlcpp_notice.md>>

*   Optimized to meet the needs of high-level service programming.
*   Represent data structures using idiomatic C++ types such as `std::vector`,
    `std::optional`, and `std::string`.
*   Use smart pointers to manage heap allocated objects.
*   Use `zx::handle` (libzx) to manage handle ownership.
*   Can convert data from in-place FIDL buffers to idiomatic heap allocated
    objects.
*   Can convert data from idiomatic heap allocated objects
    (e.g. `std::string`) to in-place buffers (e.g. as a `fidl::StringView`).
*   Code generator produces more code compared to the C bindings. This includes
    constructors, destructors, protocol proxies, protocol stubs, copy/move
    functions, and conversions to/from in-place buffers.
*   Client performs protocol dispatch by sub-classing a provided stub and
    implementing the virtual methods for each operation.
*   Both async and synchronous clients are supported. However, the async clients
    are not thread-safe.
*   Requires C++14 or above.

Refer to the [HLCPP tutorial][hlcpp-tutorial] to get started.

<!-- TODO(fxbug.dev/NNNNN): Guide for migrating HLCPP to new C++. -->

## Summary

Category                           | New C++ with wire types                   | New C++ with natural types             | High-level C++
-----------------------------------|-----------------------------------------------|--------------------------------------------|--------------------
**audience**                       | drivers and performance-critical applications | high-level services                        | high-level services
**abstraction overhead**           | RAII closing of handles [[1]](#footnote1)     | heap allocation, construction, destruction | heap allocation, construction, destruction
**type safe types**                | enums, structs, unions, handles, protocols    | enums, structs, unions, handles, protocols | enums, structs, unions, handles, protocols
**storage**                        | stack, user-provided buffer, or heap          | heap                                       | heap
**lifecycle**                      | manual or automatic free                      | automatic free (RAII)                      | automatic free (RAII)
**receive behavior**               | decode in-place                               | decode into heap                           | decode then move to heap
**send behavior**                  | copy or vectorize                             | copy                                       | copy
**calling protocol methods**       | free functions or proxy                       | free functions or proxy                    | call through proxies, register callbacks
**implementing protocol methods**  | manual dispatch or implement stub interface   | implement stub interface                   | implement stub object, invoke callbacks
**async client**                   | yes                                           | yes                                        | yes
**async server**                   | yes (unbounded) [[2]](#footnote2)             | yes (unbounded) [[2]](#footnote2)          | yes (unbounded)
**parallel server dispatch**       | yes [[3]](#footnote3)                         | yes [[3]](#footnote3)                      | no
**generated code footprint**       | large                                         | large                                      | large

--------------------------------------------------------------------------------

##### Footnote1

Generated types own all handles stored inline. Out-of-line handles e.g. those
behind a pointer indirection are not closed when the containing object of the
pointer goes away. In those cases, the bindings provide a `fidl::DecodedValue`
object to manage all handles associated with a call.

##### Footnote2

The bindings library defined in [lib/fidl](/sdk/lib/fidl/cpp/wire) can
dispatch an unbounded number of in-flight transactions via `fidl::BindServer`
defined in
[lib/fidl/cpp/wire/channel.h](/sdk/lib/fidl/cpp/wire/include/lib/fidl/cpp/wire/channel.h).

##### Footnote3

The bindings library [lib/fidl](/sdk/lib/fidl/cpp/wire) enables parallel
dispatch using the `EnableNextDispatch()` API defined in
[lib/fidl/cpp/wire/async_transaction.h](/sdk/lib/fidl/cpp/wire/include/lib/fidl/cpp/wire/async_transaction.h).

<!-- xrefs -->
[layout-attribute]: /docs/reference/fidl/language/attributes.md#layout
[cpp-tutorial]: /docs/development/languages/fidl/tutorials/cpp
[cpp-prefer-natural]: /docs/development/languages/fidl/tutorials/cpp/README.md#natural_and_wire_domain_objects
[hlcpp-tutorial]: /docs/development/languages/fidl/tutorials/hlcpp
[kvstore]: /docs/development/languages/fidl/examples/key_value_store
[canvas]: /docs/development/languages/fidl/examples/canvas
