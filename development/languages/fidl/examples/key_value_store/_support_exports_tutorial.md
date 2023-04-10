A simple way to extend the key-value store to support exporting backups would be
to simply add a new method that stops the world, serializes the state of the
store, and sends it back as a FIDL `vector<Item>`. There are two downsides to
this approach, however. The first is that it puts all of the burden of the
backup on the server - a client pays nothing to ask for a backup operation that
is very expensive to the server. The second is that it involves a great deal of
copying: the client is almost certainly just going to write the resulting backup
to some backing datastore, like a file or a database, as soon as it receives it.
Having it decode this (potentially very large) FIDL object, just so that it can
immediately re-encode it as it forwards it to whatever protocol will do the
actual storage, is very wasteful.

### Reasoning

A better solution is to use zircon's [virtual memory
objects][docs-zx-concepts-vmos]. Instead of constantly copying bytes back and
forth in a [bucket brigade][wiki-bucket-brigade], we can mint a VMO to hold the
backup data on the client, send it to the server, then forward it back to our
target data store without deserializing in between. As long as the target data
store's protocol has allowances for accepting data transported using a VMO, this
is the preferred way to accomplish expensive operations like this. In fact,
Fuchsia's file system, for instance, implements this exact pattern. A benefit of
this approach is that it forces the client to do some work when asking the
server for an expensive operation, minimizing the work imbalance between the two
parties.

[docs-zx-concepts-vmos]: /concepts/kernel/concepts.md#shared_memory_virtual_memory_objects_vmos
[wiki-bucket-brigade]: https://en.wikipedia.org/wiki/Bucket_brigade

FIDL value types can be *persisted* to any byte-oriented storage medium, using
the [FIDL data persistence][fidl-persistence] binary format. We will persist the
newly introduced FIDL type `Exportable` into the VMO. The object will be encoded
and written to the storage (in this case, a VMO that could later be saved as a
file), and decoded from it when the data needs to be accessed again, in much the
same way that a message is encoded, transported, and decoded again later when
using FIDL over IPC.

To do this securely and adhere to the [principle of least privilege][wiki-polp],
we should constrain the privileges the handle representing our VMO may carry.
Enter *handle rights*, FIDL's first-class method of describing the privileges
available to a particular handle type. In this case, we allow the `empty` VMO
passed to the server in the `Export` request to be read from, queried for size,
resized, and written to. When the VMO is returned, we remove right to resize and
write, ensuring that no process, not even malicious actors in some far away
component, can modify this data as it moves through the system.

[fidl-persistence]: /contribute/governance/rfcs/0120_standalone_use_of_fidl_wire_format.md#convention_for_data_persistence
[wiki-polp]: https://en.wikipedia.org/wiki/Principle_of_least_privilege

### Implementation

Note: The source code for this example is located at
[//examples/fidl/new/key_value_store/support_exports](/examples/fidl/new/key_value_store/support_exports).
This directory includes tests exercising the implementation in all supported
languages, which may be run locally by executing the following from
the command line: `fx set core.x64 --with=//examples/fidl/new:tests && fx test
keyvaluestore_supportexports`.

The FIDL, CML, and realm interface definitions are as follows:

<div>
  <devsite-selector>
    <!-- FIDL -->
    <section>
      <h3 id="key_value_store-support_exports-fidl">FIDL</h3>
      <pre class="prettyprint">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/key_value_store/support_exports/fidl/key_value_store.test.fidl" highlight="diff_1,diff_2,diff_3" %}</pre>
    </section>
    <!-- CML -->
    <section style="padding: 0px;">
      <h3>CML</h3>
      <devsite-selector style="margin: 0px; padding: 0px;">
        <section>
          <h3 id="key_value_store-support_exports-cml-client">Client</h3>
          <pre class="prettyprint">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/key_value_store/support_exports/meta/client.cml" highlight="diff_1" %}</pre>
        </section>
        <section>
          <h3 id="key_value_store-support_exports-server">Server</h3>
          <pre class="prettyprint">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/key_value_store/support_exports/meta/server.cml" %}</pre>
        </section>
        <section>
          <h3 id="key_value_store-support_exports-realm">Realm</h3>
          <pre class="prettyprint">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/key_value_store/support_exports/realm/meta/realm.cml" %}</pre>
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
          <h3 id="key_value_store-support_exports-rust-client">Client</h3>
          <pre class="prettyprint lang-rust">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/key_value_store/support_exports/rust/client/src/main.rs" highlight="diff_1,diff_2" %}</pre>
        </section>
        <section>
          <h3 id="key_value_store-support_exports-rust-server">Server</h3>
          <pre class="prettyprint lang-rust">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/key_value_store/support_exports/rust/server/src/main.rs" highlight="diff_1,diff_2,diff_3" %}</pre>
        </section>
      </devsite-selector>
    </section>
    <!-- C++ (Natural) -->
    <section style="padding: 0px;">
      <h3>C++ (Natural)</h3>
      <devsite-selector style="margin: 0px; padding: 0px;">
        <section>
          <h3 id="key_value_store-support_exports-cpp_natural-client">Client</h3>
          <pre class="prettyprint lang-cc">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/key_value_store/support_exports/cpp_natural/client/main.cc" highlight="diff_1" %}</pre>
        </section>
        <section>
          <h3 id="key_value_store-support_exports-cpp_natural-server">Server</h3>
          <pre class="prettyprint lang-cc">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/key_value_store/support_exports/cpp_natural/server/main.cc" highlight="diff_1,diff_2,diff_3" %}</pre>
        </section>
      </devsite-selector>
    </section>
    <!-- C++ (Wire) -->
    <section style="padding: 0px;">
      <h3>C++ (Wire)</h3>
      <devsite-selector style="margin: 0px; padding: 0px;">
        <section>
          <h3 id="key_value_store-support_exports-cpp_wire-client">Client</h3>
          <pre class="prettyprint lang-cc">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/key_value_store/support_exports/cpp_wire/client/main.cc" highlight="diff_1" %}</pre>
        </section>
        <section>
          <h3 id="key_value_store-support_exports-cpp_wire-server">Server</h3>
          <pre class="prettyprint lang-cc">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/key_value_store/support_exports/cpp_wire/server/main.cc" highlight="diff_1,diff_2,diff_3" %}</pre>
        </section>
      </devsite-selector>
    </section>
    <!-- HLCPP -->
    <section style="padding: 0px;">
      <h3 id="key_value_store-support_exports-hlcpp">HLCPP</h3>
      <devsite-selector style="margin: 0px; padding: 0px;">
        <section>
          <h3 id="key_value_store-support_exports-hlcpp-client">Client</h3>
          <pre class="prettyprint lang-cc">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/key_value_store/support_exports/hlcpp/TODO.md" region_tag="todo" %}</pre>
        </section>
        <section>
          <h3 id="key_value_store-support_exports-hlcpp-server">Server</h3>
          <pre class="prettyprint lang-cc">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/key_value_store/support_exports/hlcpp/TODO.md" region_tag="todo" %}</pre>
        </section>
      </devsite-selector>
    </section>
  </devsite-selector>
</div>
