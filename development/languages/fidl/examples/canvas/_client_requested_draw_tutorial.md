One way to improve the performance of the `Instance` protocol is to allow the
batching of lines: rather than sending a single `AddLine(...);` every time we
have a new line we'd like to add to the canvas, waiting for the reply, then
doing it again for the next line, we can instead batch many lines into a single
invocation of the new `AddLines(...);` call. The client can now decide how to
best segment a large collection of lines to be drawn.

Naively implemented, we would find ourselves in a situation where the server and
the client are completely unsynchronized: the client can flood the server with
unbounded `AddLines(...);` calls, and the server can similarly flood the client
with more `-> OnDrawn(...);` events than it can handle. The solution to both of
these problems is to add a simple `Ready() -> ();` method for synchronization
purposes. This method is called by the client whenever it is prepared to receive
the next draw update, with the response from the server indicating that the client
can proceed with more requests.

We now have some flow control in both directions. The protocol now implements
the *feed forward pattern*, allowing many uncontrolled calls before some
synchronizing "commit" call triggers the actual work on the server. This
prevents the client from overwhelming the server with work. Similarly, the
server is no longer allowed to send unbounded `-> OnDrawn(...);` events: each
event must follow a signal from the client, the `Ready() -> ();` call, that
indicates that it is ready to do more work. This is known as the *throttled
event pattern*.

The concrete implementations must apply some of these rules manually: the client
must close the connection if it receives an `-> OnDrawn(...);` event it did not
request via the `Ready() -> ();` method.

Note: The source code for this example is located at
[//examples/fidl/new/canvas/client_requested_draw](/examples/fidl/new/canvas/client_requested_draw).
This directory includes tests exercising the implementation in all supported
languages, which may be run locally by executing the following from
the command line: `fx set core.x64 --with=//examples/fidl/new:tests && fx test
canvas_clientrequesteddraw`.

The FIDL, CML, and realm interface definitions are as follows:

<div>
  <devsite-selector>
    <!-- FIDL -->
    <section>
      <h3 id="canvas-client_requested_draw-fidl">FIDL</h3>
      <pre class="prettyprint">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/canvas/client_requested_draw/fidl/canvas.test.fidl" highlight="diff_1" %}</pre>
    </section>
    <!-- CML -->
    <section style="padding: 0px;">
      <h3>CML</h3>
      <devsite-selector style="margin: 0px; padding: 0px;">
        <section>
          <h3 id="canvas-client_requested_draw-cml-client">Client</h3>
          <pre class="prettyprint">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/canvas/client_requested_draw/meta/client.cml" highlight="diff_1" %}</pre>
        </section>
        <section>
          <h3 id="canvas-client_requested_draw-server">Server</h3>
          <pre class="prettyprint">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/canvas/client_requested_draw/meta/server.cml" %}</pre>
        </section>
        <section>
          <h3 id="canvas-client_requested_draw-realm">Realm</h3>
          <pre class="prettyprint">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/canvas/client_requested_draw/realm/meta/realm.cml" %}</pre>
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
          <h3 id="canvas-client_requested_draw-rust-client">Client</h3>
          <pre class="prettyprint lang-rust">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/canvas/client_requested_draw/rust/client/src/main.rs" highlight="diff_1,diff_2,diff_3" %}</pre>
        </section>
        <section>
          <h3 id="canvas-client_requested_draw-rust-server">Server</h3>
          <pre class="prettyprint lang-rust">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/canvas/client_requested_draw/rust/server/src/main.rs" highlight="diff_1,diff_2,diff_3,diff_4,diff_5,diff_6" %}</pre>
        </section>
      </devsite-selector>
    </section>
    <!-- C++ (Natural) -->
    <section style="padding: 0px;">
      <h3>C++ (Natural)</h3>
      <devsite-selector style="margin: 0px; padding: 0px;">
        <section>
          <h3 id="canvas-client_requested_draw-cpp_natural-client">Client</h3>
          <pre class="prettyprint lang-cc">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/canvas/client_requested_draw/cpp_natural/client/main.cc" highlight="diff_1,diff_2,diff_3" %}</pre>
        </section>
        <section>
          <h3 id="canvas-client_requested_draw-cpp_natural-server">Server</h3>
          <pre class="prettyprint lang-cc">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/canvas/client_requested_draw/cpp_natural/server/main.cc" highlight="diff_1,diff_2,diff_3,diff_4,diff_5" %}</pre>
        </section>
      </devsite-selector>
    </section>
    <!-- C++ (Wire) -->
    <section style="padding: 0px;">
      <h3>C++ (Wire)</h3>
      <devsite-selector style="margin: 0px; padding: 0px;">
        <section>
          <h3 id="canvas-client_requested_draw-cpp_wire-client">Client</h3>
          <pre class="prettyprint lang-cc">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/canvas/client_requested_draw/cpp_wire/client/main.cc" highlight="diff_1,diff_2,diff_3" %}</pre>
        </section>
        <section>
          <h3 id="canvas-client_requested_draw-cpp_wire-server">Server</h3>
          <pre class="prettyprint lang-cc">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/canvas/client_requested_draw/cpp_wire/server/main.cc" highlight="diff_1,diff_2,diff_3,diff_4,diff_5" %}</pre>
        </section>
      </devsite-selector>
    </section>
    <!-- HLCPP -->
    <section style="padding: 0px;">
      <h3 id="canvas-client_requested_draw-hlcpp">HLCPP</h3>
      <devsite-selector style="margin: 0px; padding: 0px;">
        <section>
          <h3 id="canvas-client_requested_draw-hlcpp-client">Client</h3>
          <pre class="prettyprint lang-cc">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/canvas/client_requested_draw/hlcpp/client/main.cc" highlight="diff_1,diff_2,diff_3" %}</pre>
        </section>
        <section>
          <h3 id="canvas-client_requested_draw-hlcpp-server">Server</h3>
          <pre class="prettyprint lang-cc">{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/new/canvas/client_requested_draw/hlcpp/server/main.cc" highlight="diff_1,diff_2,diff_3,diff_4,diff_5" %}</pre>
        </section>
      </devsite-selector>
    </section>
  </devsite-selector>
</div>
