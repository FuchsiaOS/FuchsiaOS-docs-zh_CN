Sending unmetered one way calls back and forth produces a simple design, but
there are potential pitfalls: what if the server is much slower at processing
updates than the client sends them? For example, the client may load a drawing
consisting of many thousands of lines from some text file, and try to send them
all sequentially. How can we apply back pressure to the client to prevent the
server from being overwhelmed by this wave of updates?

By using the acknowledgement pattern and making the one way call `AddLine(...);`
into a two way `AddLine(...) -> ();`, we can provide feedback to the client.
This will allow the client to throttle its output as appropriate. In this
example, we'll simply have the client wait for the ack before sending the next
message it has waiting, though more complex designs could send messages
optimistically, and only throttle when they receive async acks less frequently
than expected.

Note: The source code for this example is located at
[//examples/fidl/new/canvas/diff_1](/examples/fidl/new/canvas/add_line_metered).
This directory includes tests exercising the implementation in all supported
languages, which may be run locally by executing the following from
the command line: `fx set core.x64 --with=//examples/fidl/new:tests && fx test
canvas_add_line_metered`.

First, we need to define our interface definitions and test harness. The FIDL,
CML, and realm interface definitions set up a scaffold that arbitrary
implementations can use:

<div>
  <devsite-selector>
    <!-- FIDL -->
    <section>
      <h3>FIDL</h3>
      <pre class="prettyprint">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/canvas/add_line_metered/fidl/canvas.test.fidl" highlight="diff_1" %}</pre>
    </section>
    <!-- CML -->
    <section style="padding: 0px;">
      <h3>CML</h3>
      <devsite-selector style="margin: 0px; padding: 0px;">
        <section>
          <h3 id="canvas-add_line_metered-cml-client">Client</h3>
          <pre class="prettyprint">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/canvas/add_line_metered/meta/client.cml" %}</pre>
        </section>
        <section>
          <h3 id="canvas-add_line_metered-cml-server">Server</h3>
          <pre class="prettyprint">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/canvas/add_line_metered/meta/server.cml" %}</pre>
        </section>
        <section>
          <h3 id="canvas-add_line_metered-cml-realm">Realm</h3>
          <pre class="prettyprint">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/canvas/add_line_metered/realm/meta/realm.cml" %}</pre>
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
          <h3 id="canvas-add_line_metered-rust-client">Client</h3>
          <pre class="prettyprint lang-rust">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/canvas/add_line_metered/rust/client/src/main.rs" highlight="diff_1" %}</pre>
        </section>
        <section>
          <h3 id="canvas-add_line_metered-rust-server">Server</h3>
          <pre class="prettyprint lang-rust">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/canvas/add_line_metered/rust/server/src/main.rs" highlight="diff_1" %}</pre>
        </section>
      </devsite-selector>
    </section>
    <!-- C++ (Natural) -->
    <section style="padding: 0px;">
      <h3>C++ (Natural)</h3>
      <devsite-selector style="margin: 0px; padding: 0px;">
        <section>
          <h3 id="canvas-add_line_metered-cpp_natural-client">Client</h3>
          <pre class="prettyprint lang-cc">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/canvas/add_line_metered/cpp_natural/client/main.cc" highlight="diff_1" %}</pre>
        </section>
        <section>
          <h3 id="canvas-add_line_metered-cpp_natural-server">Server</h3>
          <pre class="prettyprint lang-cc">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/canvas/add_line_metered/cpp_natural/server/main.cc" highlight="diff_1" %}</pre>
        </section>
      </devsite-selector>
    </section>
    <!-- C++ (Wire) -->
    <section style="padding: 0px;">
      <h3>C++ (Wire)</h3>
      <devsite-selector style="margin: 0px; padding: 0px;">
        <section>
          <h3 id="canvas-add_line_metered-cpp_wire-client">Client</h3>
          <pre class="prettyprint lang-cc">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/canvas/add_line_metered/cpp_wire/client/main.cc" highlight="diff_1" %}</pre>
        </section>
        <section>
          <h3 id="canvas-add_line_metered-cpp_wire-server">Server</h3>
          <pre class="prettyprint lang-cc">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/canvas/add_line_metered/cpp_wire/server/main.cc" highlight="diff_1" %}</pre>
        </section>
      </devsite-selector>
    </section>
    <!-- HLCPP -->
    <section style="padding: 0px;">
      <h3>HLCPP</h3>
      <devsite-selector style="margin: 0px; padding: 0px;">
        <section>
          <h3 id="canvas-add_line_metered-hlcpp-client">Client</h3>
          <pre class="prettyprint lang-cc">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/canvas/add_line_metered/hlcpp/client/main.cc" highlight="diff_1" %}</pre>
        </section>
        <section>
          <h3 id="canvas-add_line_metered-hlcpp-server">Server</h3>
          <pre class="prettyprint lang-cc">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/canvas/add_line_metered/hlcpp/server/main.cc" highlight="diff_1,diff_2" %}</pre>
        </section>
      </devsite-selector>
    </section>
  </devsite-selector>
</div>
