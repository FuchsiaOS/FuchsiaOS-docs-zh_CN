A useful operation for key-value stores is in-order iteration: that is, when
given a key, to return a (usually paginated) list of elements that appear after
it, in order.

### Reasoning

In FIDL, this is best done using an iterator, which is generally implemented as
a separate protocol over which this iteration can occur. Using a separate
protocol, and therefore a separate channel, has a number of benefits, including
de-interleaving the iteration pull requests from other operations done over the
main protocol.

The client and server side of the channel connection for protocol `P` can be
represented as FIDL data types, as a `client_end:P` and `server_end:P`,
respectively. These types are collectively known as *protocol ends*, and
represent the other (non-`@discoverable`) way of connecting a FIDL client to its
corresponding server: over an existing FIDL connection!

Protocol ends are specific instances of a general FIDL concept: the *resource
type*. A resource type is intended to contain FIDL handles, which necessitates
extra restrictions on how the type can be used. The type must be always be
unique, as the underlying resource is mediated by some other capability manager
(usually the Zircon kernel). Duplicating such a resource via a simple in-memory
copy, without involving the manager, is impossible. To prevent such duplication,
all resource types in FIDL are always move-only.

Finally, the `Get()` method of the `Iterator` protocol itself makes use of a
*size constraint* on the return payload. This limits the amount of data that may
be transmitted in a single pull, allowing for some measure of resource use
control. It also creates a natural pagination boundary: rather than a giant dump
of all of the results at once, the server only needs to prepare small batches at
a time.

### Implementation

Note: The source code for this example is located at
[//examples/fidl/new/key_value_store/add_iterator](/examples/fidl/new/key_value_store/add_iterator).
This directory includes tests exercising the implementation in all supported
languages, which may be run locally by executing the following from the command
line: `fx set core.x64 --with=//examples/fidl/new:tests && fx test
key_value_store_add_iterator`.

The FIDL, CML, and realm interface definitions are as follows:

<div>
  <devsite-selector>
    <!-- FIDL -->
    <section>
      <h3 id="key_value_store-add_iterator-fidl">FIDL</h3>
      <pre class="prettyprint">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/key_value_store/add_iterator/fidl/key_value_store.test.fidl" highlight="diff_1,diff_2,diff_3" %}</pre>
    </section>
    <!-- CML -->
    <section style="padding: 0px;">
      <h3>CML</h3>
      <devsite-selector style="margin: 0px; padding: 0px;">
        <section>
          <h3 id="key_value_store-add_iterator-cml-client">Client</h3>
          <pre class="prettyprint">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/key_value_store/add_iterator/meta/client.cml" highlight="diff_1" %}</pre>
        </section>
        <section>
          <h3 id="key_value_store-add_iterator-server">Server</h3>
          <pre class="prettyprint">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/key_value_store/add_iterator/meta/server.cml" %}</pre>
        </section>
        <section>
          <h3 id="key_value_store-add_iterator-realm">Realm</h3>
          <pre class="prettyprint">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/key_value_store/add_iterator/realm/meta/realm.cml" %}</pre>
        </section>
      </devsite-selector>
    </section>
  </devsite-selector>
</div>

Client and server implementations can then be written in any supported language:

<div>
  <devsite-selector>
    <!-- Rust -->
    <section style="padding: 0px;">
      <h3>Rust</h3>
      <devsite-selector style="margin: 0px; padding: 0px;">
        <section>
          <h3 id="key_value_store-add_iterator-rust-client">Client</h3>
          <pre class="prettyprint lang-rust">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/key_value_store/add_iterator/rust/client/src/main.rs" highlight="diff_1,diff_2" %}</pre>
        </section>
        <section>
          <h3 id="key_value_store-add_iterator-rust-server">Server</h3>
          <pre class="prettyprint lang-rust">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/key_value_store/add_iterator/rust/server/src/main.rs" highlight="diff_1,diff_2,diff_3,diff_4,diff_5,diff_6" %}</pre>
        </section>
      </devsite-selector>
    </section>
    <!-- C++ (Natural) -->
    <section style="padding: 0px;">
      <h3>C++ (Natural)</h3>
      <devsite-selector style="margin: 0px; padding: 0px;">
        <section>
          <h3 id="key_value_store-add_iterator-cpp_natural-client">Client</h3>
          <pre class="prettyprint lang-cc">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/key_value_store/add_iterator/cpp_natural/TODO.md" region_tag="todo" %}</pre>
        </section>
        <section>
          <h3 id="key_value_store-add_iterator-cpp_natural-server">Server</h3>
          <pre class="prettyprint lang-cc">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/key_value_store/add_iterator/cpp_natural/TODO.md" region_tag="todo" %}</pre>
        </section>
      </devsite-selector>
    </section>
    <!-- C++ (Wire) -->
    <section style="padding: 0px;">
      <h3>C++ (Wire)</h3>
      <devsite-selector style="margin: 0px; padding: 0px;">
        <section>
          <h3 id="key_value_store-add_iterator-cpp_wire-client">Client</h3>
          <pre class="prettyprint lang-cc">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/key_value_store/add_iterator/cpp_wire/TODO.md" region_tag="todo" %}</pre>
        </section>
        <section>
          <h3 id="key_value_store-add_iterator-cpp_wire-server">Server</h3>
          <pre class="prettyprint lang-cc">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/key_value_store/add_iterator/cpp_wire/TODO.md" region_tag="todo" %}</pre>
        </section>
      </devsite-selector>
    </section>
    <!-- HLCPP -->
    <section style="padding: 0px;">
      <h3 id="key_value_store-add_iterator-hlcpp">HLCPP</h3>
      <devsite-selector style="margin: 0px; padding: 0px;">
        <section>
          <h3 id="key_value_store-add_iterator-hlcpp-client">Client</h3>
          <pre class="prettyprint lang-cc">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/key_value_store/add_iterator/hlcpp/TODO.md" region_tag="todo" %}</pre>
        </section>
        <section>
          <h3 id="key_value_store-add_iterator-hlcpp-server">Server</h3>
          <pre class="prettyprint lang-cc">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/key_value_store/add_iterator/hlcpp/TODO.md" region_tag="todo" %}</pre>
        </section>
      </devsite-selector>
    </section>
  </devsite-selector>
</div>
