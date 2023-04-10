This baseline case showcases an end-to-end implementation of a simple canvas
using FIDL. This design specifies the `Canvas` protocol, which allows a client
to add lines to the canvas via the `AddLine` method, and receive draw updates
from the server via the `OnDrawn` event.

The protocol we design here will be functional, but suboptimal in terms of both
performance and flow control. For instance, our current "refresh rate" is a
glacial 1 frame per second. What happens if we decide to update at 60 frames per
second (ie, roughly every 16ms, rather than once a second)? Is it possible that
`OnDrawn` events will overwhelm the client? Conversely, what happens when a
client loads send many `AddLine` requests at once, perhaps after loading them
from a file? Will the server now be crushed under the load?

This unthrottled implementation is best regarded as a first pass - a relatively
simple protocol that demonstrates some functionality, but one that could use
some improvement to extract optimal performance, especially under stress.

Note: The source code for this example is located at
[//examples/fidl/new/canvas/baseline](/examples/fidl/new/canvas/baseline).
This directory includes tests exercising the implementation in all supported
languages, which may be run locally by executing the following from
the command line: `fx set core.x64 --with=//examples/fidl/new:tests && fx test
canvas_baseline`.

First, we need to define our interface definitions and test harness. The FIDL,
CML, and realm interface definitions set up a scaffold that arbitrary
implementations can use:

<div>
  <devsite-selector>
    <!-- FIDL -->
    <section>
      <h3>FIDL</h3>
      <pre class="prettyprint">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/canvas/baseline/fidl/canvas.test.fidl" %}</pre>
    </section>
    <!-- CML -->
    <section style="padding: 0px;">
      <h3>CML</h3>
      <devsite-selector style="margin: 0px; padding: 0px;">
        <section>
          <h3 id="canvas-baseline-cml-client">Client</h3>
          <pre class="prettyprint">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/canvas/baseline/meta/client.cml" %}</pre>
        </section>
        <section>
          <h3 id="canvas-baseline-cml-server">Server</h3>
          <pre class="prettyprint">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/canvas/baseline/meta/server.cml" %}</pre>
        </section>
        <section>
          <h3 id="canvas-baseline-cml-realm">Realm</h3>
          <pre class="prettyprint">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/canvas/baseline/realm/meta/realm.cml" %}</pre>
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
          <h3 id="canvas-baseline-rust-client">Client</h3>
          <pre class="prettyprint lang-rust">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/canvas/baseline/rust/client/src/main.rs" %}</pre>
        </section>
        <section>
          <h3 id="canvas-baseline-rust-server">Server</h3>
          <pre class="prettyprint lang-rust">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/canvas/baseline/rust/server/src/main.rs" %}</pre>
        </section>
      </devsite-selector>
    </section>
    <!-- C++ (Natural) -->
    <section style="padding: 0px;">
      <h3>C++ (Natural)</h3>
      <devsite-selector style="margin: 0px; padding: 0px;">
        <section>
          <h3 id="canvas-baseline-cpp_natural-client">Client</h3>
          <pre class="prettyprint lang-cc">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/canvas/baseline/cpp_natural/client/main.cc" %}</pre>
        </section>
        <section>
          <h3 id="canvas-baseline-cpp_natural-server">Server</h3>
          <pre class="prettyprint lang-cc">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/canvas/baseline/cpp_natural/server/main.cc" %}</pre>
        </section>
      </devsite-selector>
    </section>
    <!-- C++ (Wire) -->
    <section style="padding: 0px;">
      <h3>C++ (Wire)</h3>
      <devsite-selector style="margin: 0px; padding: 0px;">
        <section>
          <h3 id="canvas-baseline-cpp_wire-client">Client</h3>
          <pre class="prettyprint lang-cc">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/canvas/baseline/cpp_wire/client/main.cc" %}</pre>
        </section>
        <section>
          <h3 id="canvas-baseline-cpp_wire-server">Server</h3>
          <pre class="prettyprint lang-cc">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/canvas/baseline/cpp_wire/server/main.cc" %}</pre>
        </section>
      </devsite-selector>
    </section>
    <!-- HLCPP -->
    <section style="padding: 0px;">
      <h3>HLCPP</h3>
      <devsite-selector style="margin: 0px; padding: 0px;">
        <section>
          <h3 id="canvas-baseline-hlcpp-client">Client</h3>
          <pre class="prettyprint lang-cc">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/canvas/baseline/hlcpp/client/main.cc" %}</pre>
        </section>
        <section>
          <h3 id="canvas-baseline-hlcpp-server">Server</h3>
          <pre class="prettyprint lang-cc">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/canvas/baseline/hlcpp/server/main.cc" %}</pre>
        </section>
      </devsite-selector>
    </section>
  </devsite-selector>
</div>
