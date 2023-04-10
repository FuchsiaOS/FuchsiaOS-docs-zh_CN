In this example, you will create a basic calculator server & client which shows the
fundamental setup needed to first define and then serve and consume a FIDL protocol.

Note: The source code for this example is located at
[//examples/fidl/new/calculator/baseline](/examples/fidl/new/calculator/baseline).
This directory includes tests exercising the implementation in all supported
languages, which may be run locally by executing the following from
the command line: `fx set core.x64 --with=//examples/fidl/new:tests && fx test
calculator_baseline`.

First, you will define the interface definitions and test harness. The
interface definition (the `.fidl` file itself) is the starting point for any new
FIDL protocol. Additionally, the calculator includes the necessary CML and realm
definitions to create a client-server pattern which can be used as project
scaffolding for arbitrary implementations.

See below for the FIDL code:

<div>
  <devsite-selector>
    <!-- FIDL -->
    <section>
      <h3 id="calculator-baseline-fidl">FIDL</h3>
      <pre class="prettyprint">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/calculator/baseline/fidl/calculator.test.fidl" %}</pre>
    </section>
    <!-- CML -->
    <section style="padding: 0px;">
      <h3>CML</h3>
      <devsite-selector style="margin: 0px; padding: 0px;">
        <section>
          <h3 id="calculator-baseline-cml-client">Client</h3>
          <pre class="prettyprint">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/calculator/baseline/meta/client.cml" %}</pre>
        </section>
        <section>
          <h3 id="calculator-baseline-server">Server</h3>
          <pre class="prettyprint">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/calculator/baseline/meta/server.cml" %}</pre>
        </section>
        <section>
          <h3 id="calculator-baseline-realm">Realm</h3>
          <pre class="prettyprint">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/calculator/baseline/realm/meta/realm.cml" %}</pre>
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
          <h3 id="calculator-baseline-rust-client">Client</h3>
          <pre class="prettyprint lang-rust">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/calculator/baseline/rust/TODO.md" region_tag="todo" %}</pre>
        </section>
        <section>
          <h3 id="calculator-baseline-rust-server">Server</h3>
          <pre class="prettyprint lang-rust">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/calculator/baseline/rust/TODO.md" region_tag="todo" %}</pre>
        </section>
      </devsite-selector>
    </section>
    <!-- C++ (Natural) -->
    <section style="padding: 0px;">
      <h3>C++ (Natural)</h3>
      <devsite-selector style="margin: 0px; padding: 0px;">
        <section>
          <h3 id="calculator-baseline-cpp_natural-client">Client</h3>
          <pre class="prettyprint lang-cc">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/calculator/baseline/cpp_natural/TODO.md" region_tag="todo" %}</pre>
        </section>
        <section>
          <h3 id="calculator-baseline-cpp_natural-server">Server</h3>
          <pre class="prettyprint lang-cc">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/calculator/baseline/cpp_natural/TODO.md" region_tag="todo" %}</pre>
        </section>
      </devsite-selector>
    </section>
    <!-- C++ (Wire) -->
    <section style="padding: 0px;">
      <h3>C++ (Wire)</h3>
      <devsite-selector style="margin: 0px; padding: 0px;">
        <section>
          <h3 id="calculator-baseline-cpp_wire-client">Client</h3>
          <pre class="prettyprint lang-cc">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/calculator/baseline/cpp_wire/TODO.md" region_tag="todo" %}</pre>
        </section>
        <section>
          <h3 id="calculator-baseline-cpp_wire-server">Server</h3>
          <pre class="prettyprint lang-cc">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/calculator/baseline/cpp_wire/TODO.md" region_tag="todo" %}</pre>
        </section>
      </devsite-selector>
    </section>
    <!-- HLCPP -->
    <section style="padding: 0px;">
      <h3 id="calculator-baseline-hlcpp">HLCPP</h3>
      <devsite-selector style="margin: 0px; padding: 0px;">
        <section>
          <h3 id="calculator-baseline-hlcpp-client">Client</h3>
          <pre class="prettyprint lang-cc">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/calculator/baseline/hlcpp/TODO.md" region_tag="todo" %}</pre>
        </section>
        <section>
          <h3 id="calculator-baseline-hlcpp-server">Server</h3>
          <pre class="prettyprint lang-cc">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/calculator/baseline/hlcpp/TODO.md" region_tag="todo" %}</pre>
        </section>
      </devsite-selector>
    </section>
  </devsite-selector>
</div>

Creating a FIDL protocol from the ground up as is shown in this example can be
a more common scenario for certain developers, such as platform developers.
However, other types of developers also benefit from learning how to construct
a FIDL protocol even if they won't typically do so. This helps you learn how
everything about FIDL fits together, including the syntax, grammar, language
features, how to serve and consume a given FIDL protocol, and how the build
system works. For next steps, the examples which follow this baseline show how
to extend an existing FIDL protocol, which is expected to be a fairly common
practice.
