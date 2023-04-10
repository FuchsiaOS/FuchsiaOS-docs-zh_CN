### Reasoning

The [key-value store baseline
example's](/docs/development/languages/fidl/examples/key_value_store#baseline)
implementation was a good starting point, but one major drawback is that data is
stored as raw bytes. FIDL is a richly typed language. Forcing data that is for
instance a UTF-8 string to be stored as an untyped byte array erases this
valuable type information for readers of the *.fidl file, as well as for
programmers using bindings generated from it.

### Implementation

The main goal of this change is to replace the baseline case's `vector<byte>`
typed `value` member with a `union` that stores many possible types. In fact, as
of this change a good survey of FIDL's
[value](/docs/reference/fidl/language/language.md#value-vs-resource) types is on
offer:

- All of FIDL's builtin *scalar types* are used as variants in the `Value`
  `union`: `bool`, `uint8`, `uint16`, `uint32`, `uint64`, `int8`,
  `int16`, `int32`, `int64`, `float32`, and `float64` (also known as FIDL's
  *primitive types*), as well as `string`.
- This `union` also features uses of FIDL's builtin `array<T, N>` and
  `vector<T>` type templates.
- All of FIDL's type layouts, namely `bits`, `enum`, `table`, `union`, and
  `struct`, are utilized in this example at least once.

The request and response payloads used for `WriteItem` have also been changed
from `struct`s to a named `table` and an inlined `flexible union`, respectively.
In fact, any of these three layouts may be used a request/response payload. The
latter two, known as *table payloads* and *union payloads, respectively, are
preferred in all but the most message size sensitive cases. This is because they
are much easier to extend in the future in a binary compatible way.

<div>
  <devsite-selector>
    <!-- FIDL -->
    <section>
      <h3 id="key_value_store-use_generic_values-fidl">FIDL</h3>
      <pre class="prettyprint">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/key_value_store/use_generic_values/fidl/key_value_store.test.fidl" highlight="diff_1,diff_2" %}</pre>
    </section>
    <!-- CML -->
    <section style="padding: 0px;">
      <h3>CML</h3>
      <devsite-selector style="margin: 0px; padding: 0px;">
        <section>
          <h3 id="key_value_store-use_generic_values-cml-client">Client</h3>
          <pre class="prettyprint">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/key_value_store/use_generic_values/meta/client.cml" highlight="diff_1" %}</pre>
        </section>
        <section>
          <h3 id="key_value_store-use_generic_values-server">Server</h3>
          <pre class="prettyprint">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/key_value_store/use_generic_values/meta/server.cml" %}</pre>
        </section>
        <section>
          <h3 id="key_value_store-use_generic_values-realm">Realm</h3>
          <pre class="prettyprint">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/key_value_store/use_generic_values/realm/meta/realm.cml" %}</pre>
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
          <h3 id="key_value_store-use_generic_values-rust-client">Client</h3>
          <pre class="prettyprint lang-rust">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/key_value_store/use_generic_values/rust/client/src/main.rs" highlight="diff_1,diff_2,diff_3" %}</pre>
        </section>
        <section>
          <h3 id="key_value_store-use_generic_values-rust-server">Server</h3>
          <pre class="prettyprint lang-rust">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/key_value_store/use_generic_values/rust/server/src/main.rs" highlight="diff_1,diff_2,diff_3,diff_4" %}</pre>
        </section>
      </devsite-selector>
    </section>
    <!-- C++ (Natural) -->
    <section style="padding: 0px;">
      <h3>C++ (Natural)</h3>
      <devsite-selector style="margin: 0px; padding: 0px;">
        <section>
          <h3 id="key_value_store-use_generic_values-cpp_natural-client">Client</h3>
          <pre class="prettyprint lang-cc">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/key_value_store/use_generic_values/cpp_natural/TODO.md" region_tag="todo" %}</pre>
        </section>
        <section>
          <h3 id="key_value_store-use_generic_values-cpp_natural-server">Server</h3>
          <pre class="prettyprint lang-cc">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/key_value_store/use_generic_values/cpp_natural/TODO.md" region_tag="todo" %}</pre>
        </section>
      </devsite-selector>
    </section>
    <!-- C++ (Wire) -->
    <section style="padding: 0px;">
      <h3>C++ (Wire)</h3>
      <devsite-selector style="margin: 0px; padding: 0px;">
        <section>
          <h3 id="key_value_store-use_generic_values-cpp_wire-client">Client</h3>
          <pre class="prettyprint lang-cc">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/key_value_store/use_generic_values/cpp_wire/TODO.md" region_tag="todo" %}</pre>
        </section>
        <section>
          <h3 id="key_value_store-use_generic_values-cpp_wire-server">Server</h3>
          <pre class="prettyprint lang-cc">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/key_value_store/use_generic_values/cpp_wire/TODO.md" region_tag="todo" %}</pre>
        </section>
      </devsite-selector>
    </section>
    <!-- HLCPP -->
    <section style="padding: 0px;">
      <h3 id="key_value_store-use_generic_values-hlcpp">HLCPP</h3>
      <devsite-selector style="margin: 0px; padding: 0px;">
        <section>
          <h3 id="key_value_store-use_generic_values-hlcpp-client">Client</h3>
          <pre class="prettyprint lang-cc">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/key_value_store/use_generic_values/hlcpp/TODO.md" region_tag="todo" %}</pre>
        </section>
        <section>
          <h3 id="key_value_store-use_generic_values-hlcpp-server">Server</h3>
          <pre class="prettyprint lang-cc">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/key_value_store/use_generic_values/hlcpp/TODO.md" region_tag="todo" %}</pre>
        </section>
      </devsite-selector>
    </section>
  </devsite-selector>
</div>
