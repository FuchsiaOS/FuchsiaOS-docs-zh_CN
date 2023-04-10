# Fuchsia Interface Definition Language

<<../../_common/fidl/_fidl_intro.md>>

## Creating a FIDL library

**FIDL libraries** group FIDL source files together. A library acts as a
namespace for the protocols it contains, and FIDL source files can implicitly
access all other declarations within the same library. FIDL source files must
**import** any declarations from another library.

The Fuchsia build system provides the `fidl()` build target to compile FIDL
source files into a library. The name of the library target must match the
`library` declarations in each source file. See the following `BUILD.gn` example
for the `fuchsia.examples` library:

```gn
# Import the fidl GN template.
import("//build/fidl/fidl.gni")

# Define a FIDL library target.
fidl("fuchsia.examples") {
  # FIDL source files contained in library
  sources = [
    "echo.fidl",
  ]
}
```

<aside class="key-point">
By default, the build system uses the <code>fidl()</code> GN target name as the
library name. You can override this behavior with the <code>name</code>
parameter.
</aside>

At build time, the FIDL Compiler (`fidlc`) frontend tool validates and compiles
the library source files into a JSON Intermediate Representation (IR). This JSON
IR format is the basis for the FIDL bindings.

## Generating FIDL bindings

Components consume FIDL protocols through generated code called
**FIDL bindings**. Bindings encode and decode requests and responses as
**FIDL messages** and transfer them over the underlying IPC channel. The
language-specific binding libraries provide wrappers around these structures to
align interactions with familiar programming idioms.

The client interface (sometimes referred to as a proxy) performs translation
between higher-level function calls and FIDL messages. On the server side,
bindings process incoming request messages and deliver them through an abstract
interface for components to implement.

![Diagram showing how FIDL bindings provide generated library code to translate
 function calls into FIDL messages for transport across process
  boundaries.](/get-started/images/fidl/fidl-bindings.png){: width="574"}


Note: For more details on the bindings specification and supported programming
languages, see the [Bindings Reference](/reference/fidl/bindings/overview.md).

At build time, the `fidlgen` backend tools generate bindings for supported
programming languages from the JSON IR library produced by `fidlc`. For example,
`fidlgen_rust` generates Rust bindings from the JSON IR.

The `fidl()` library target creates individual binding targets for each
supported language. Due to the nature of GN, these bindings are not generated
at build time unless they are included as a dependency.

See the following example `BUILD.gn` snippet that includes the generated
bindings target for the `fuchsia.examples` library:

* {Rust}

  ```gn
  deps = [
    "fidl/fuchsia.examples:fuchsia.examples_rust",
    ...
  ]
  ```

* {C++}

  ```gn
  deps = [
    "fidl/fuchsia.examples:fuchsia.examples",
    ...
  ]
  ```

## Exercise: Echo FIDL Library

In this section, you'll define a new FIDL library with a protocol called
`Echo` containing a single method that returns string values back to the
caller.

Start by creating a new directory for the FIDL library target:

```posix-terminal
mkdir -p vendor/fuchsia-codelab/echo-fidl
```

Create the following file and directory structure in the new project directory:

```none {:.devsite-disable-click-to-copy}
//vendor/fuchsia-codelab/echo-fidl
                        |- BUILD.gn
                        |- echo.fidl
```

Add a new FIDL interface file called `echo.fidl` with the following contents:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/routing/fidl/echo.fidl" region_tag="fidl_echo" adjust_indentation="auto" %}
```

`EchoString` is a two-way method that accepts an optional (nullable) string
value and returns the same value.

Add a `BUILD.gn` file with the following contents to declare the library target:

```gn
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/routing/fidl/BUILD.gn" region_tag="fidl_echo" adjust_indentation="auto" %}
```

Add the library target to the build configuration:

<!-- TODO(fxbug.dev/108355): Update this when fidl_toolchain is removed. -->

* {Rust}

  ```posix-terminal
  fx set workstation_eng.qemu-x64 --with vendor/fuchsia-codelab/echo-fidl:echo_rust
  ```

* {C++}

  ```posix-terminal
  fx set workstation_eng.qemu-x64 --with vendor/fuchsia-codelab/echo-fidl:echo_hlcpp
  ```

### Examine the FIDL bindings

The `fidl()` GN target compiles the FIDL interface and generates additional
build targets to provide the bindings in various languages. To examine the
bindings, you must compile the individual targets.

Compile the `fidl.examples.routing.echo` bindings:

* {Rust}

  ```posix-terminal
  fx build vendor/fuchsia-codelab/echo-fidl:echo_rust
  ```

* {C++}

  ```posix-terminal
  fx build vendor/fuchsia-codelab/echo-fidl:echo_hlcpp
  ```

Use GN to locate the generated source files for the target and open them in an
editor:

* {Rust}

  ```posix-terminal
  fx gn desc out/default/ vendor/fuchsia-codelab/echo-fidl:echo_rust.actual sources
  ```

* {C++}

  ```posix-terminal
  fx gn desc out/default/ vendor/fuchsia-codelab/echo-fidl:echo_hlcpp sources
  ```

Explore the contents of these files. Below is a summary of some of the key
generated interfaces:

* {Rust}

  <table>
    <tr>
    <th><strong>Interface</strong>
    </th>
    <th><strong>Description</strong>
    </th>
    </tr>
    <tr>
    <td><code>EchoMarker</code>
    </td>
    <td>Used to open a proxy and request stream for a given protocol.
    </td>
    </tr>
    <tr>
    <td><code>EchoProxy</code>
    </td>
    <td>
      Asynchronous client that transforms protocol methods into FIDL request
      messages sent over the IPC channel.
    </td>
    </tr>
    <tr>
    <td><code>EchoSynchronousProxy</code>
    </td>
    <td>
      Synchronous client that transforms protocol methods into FIDL request
      messages sent over the IPC channel.
    </td>
    </tr>
    <tr>
    <td><code>EchoRequest</code>
    </td>
    <td>
      Structured types for handling incoming requests for each protocol method.
    </td>
    </tr>
    <tr>
    <td><code>EchoRequestStream</code>
    </td>
    <td>
      Stream to handle incoming FIDL request messages over the IPC channel.
    </td>
    </tr>
    <tr>
    <td><code>EchoEchoStringResponder</code>
    </td>
    <td>
      Callback to send a return value for each proxy request as a FIDL response
      message.
    </td>
    </tr>
  </table>

* {C++}

  <table>
    <tr>
    <th><strong>Interface</strong>
    </th>
    <th><strong>Description</strong>
    </th>
    </tr>
    <tr>
    <td><code>EchoPtr</code>
    </td>
    <td>
      Asynchronous client that transforms protocol methods into FIDL request
      messages sent over the IPC channel.
    </td>
    </tr>
    <tr>
    <td><code>EchoSyncPtr</code>
    </td>
    <td>
      Synchronous client that transforms protocol methods into FIDL request
      messages sent over the IPC channel.
    </td>
    </tr>
    <tr>
    <td><code>Echo</code>
    </td>
    <td>
      Abstract class for a server component to override and handle incoming FIDL
      requests.
    </td>
    </tr>
    <tr>
    <td><code>EchoStringCallback</code>
    </td>
    <td>
      Callback to send a return value for each request as a FIDL response
      message.
    </td>
    </tr>
  </table>

<aside class="key-point">
<b>Asynchronous vs. synchronous clients</b>
<p>The FIDL toolchain generates client bindings that are asynchronous by default.
This means the client methods return immediately to the caller and responses
are delivered using a callback or future. Synchronous clients provide a
simplified API that guarantees responses are delivered before the client
method returns.</p>

<p>Synchronous clients are not available in all supported languages. For more
details, see the specifications for your chosen language in the
<a href="/reference/fidl/bindings/overview">Bindings Reference</a>.</d>
</aside>
