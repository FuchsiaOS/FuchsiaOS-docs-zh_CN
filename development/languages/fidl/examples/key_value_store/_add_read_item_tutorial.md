### Reasoning 
The original write-only key-value store is now extended with the
ability to read items back out of the store.

### Implementation

Note: The source code for this example is located at
[//examples/fidl/new/key_value_store/add_read_item](/examples/fidl/new/key_value_store/add_read_item).
This directory includes tests exercising the implementation in all supported
languages, which may be run locally by executing the following from the command
line `fx set core.x64 --with=//examples/fidl/new:tests && fx test
keyvaluestore_addreaditem`.

The changes applied to the FIDL and CML definitions are as follows:

<div>
  <devsite-selector>
    <!-- FIDL -->
    <section>
      <h3>FIDL</h3>
      <pre class="prettyprint">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/key_value_store/add_read_item/fidl/key_value_store.test.fidl" highlight="diff_1,diff_2,diff_3" %}</pre>
    </section>
    <!-- CML -->
    <section style="padding: 0px;">
      <h3>CML</h3>
      <devsite-selector style="margin: 0px; padding: 0px;">
        <section>
          <h3 id="key_value_store-add_read_item-cml-client">Client</h3>
          <pre class="prettyprint">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/key_value_store/add_read_item/meta/client.cml" highlight="diff_1" %}</pre>
        </section>
        <section>
          <h3 id="key_value_store-add_read_item-cml-server">Server</h3>
          <pre class="prettyprint">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/key_value_store/add_read_item/meta/server.cml" %}</pre>
        </section>
        <section>
          <h3 id="key_value_store-add_read_item-cml-realm">Realm</h3>
          <pre class="prettyprint">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/key_value_store/add_read_item/realm/meta/realm.cml" %}</pre>
        </section>
      </devsite-selector>
    </section>
  </devsite-selector>
</div>

Client and server implementations for all languages change as well:

<div>
  <devsite-selector>
    <!-- Rust -->
    <section style="padding: 0px;">
      <h3>Rust</h3>
      <devsite-selector style="margin: 0px; padding: 0px;">
        <section>
          <h3 id="key_value_store-add_read_item-rust-client">Client</h3>
          <pre class="prettyprint lang-rust">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/key_value_store/add_read_item/rust/client/src/main.rs" highlight="diff_1" %}</pre>
        </section>
        <section>
          <h3 id="key_value_store-add_read_item-rust-server">Server</h3>
          <pre class="prettyprint lang-rust">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/key_value_store/add_read_item/rust/server/src/main.rs" highlight="diff_1,diff_2" %}</pre>
        </section>
      </devsite-selector>
    </section>
    <!-- C++ (Natural) -->
    <section style="padding: 0px;">
      <h3>C++ (Natural)</h3>
      <devsite-selector style="margin: 0px; padding: 0px;">
        <section>
          <h3 id="key_value_store-add_read_item-cpp_natural-client">Client</h3>
          <pre class="prettyprint lang-cc">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/key_value_store/add_read_item/cpp_natural/TODO.md" region_tag="todo" %}</pre>
        </section>
        <section>
          <h3 id="key_value_store-add_read_item-cpp_natural-server">Server</h3>
          <pre class="prettyprint lang-cc">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/key_value_store/add_read_item/cpp_natural/TODO.md" region_tag="todo" %}</pre>
        </section>
      </devsite-selector>
    </section>
    <!-- C++ (Wire) -->
    <section style="padding: 0px;">
      <h3>C++ (Wire)</h3>
      <devsite-selector style="margin: 0px; padding: 0px;">
        <section>
          <h3 id="key_value_store-add_read_item-cpp_wire-client">Client</h3>
          <pre class="prettyprint lang-cc">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/key_value_store/add_read_item/cpp_wire/TODO.md" region_tag="todo" %}</pre>
        </section>
        <section>
          <h3 id="key_value_store-add_read_item-cpp_wire-server">Server</h3>
          <pre class="prettyprint lang-cc">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/key_value_store/add_read_item/cpp_wire/TODO.md" region_tag="todo" %}</pre>
        </section>
      </devsite-selector>
    </section>
    <!-- HLCPP -->
    <section style="padding: 0px;">
      <h3>HLCPP</h3>
      <devsite-selector style="margin: 0px; padding: 0px;">
        <section>
          <h3 id="key_value_store-add_read_item-hlcpp-client">Client</h3>
          <pre class="prettyprint lang-cc">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/key_value_store/add_read_item/hlcpp/TODO.md" region_tag="todo" %}</pre>
        </section>
        <section>
          <h3 id="key_value_store-add_read_item-hlcpp-server">Server</h3>
          <pre class="prettyprint lang-cc">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/key_value_store/add_read_item/hlcpp/TODO.md" region_tag="todo" %}</pre>
        </section>
      </devsite-selector>
    </section>
  </devsite-selector>
</div>
