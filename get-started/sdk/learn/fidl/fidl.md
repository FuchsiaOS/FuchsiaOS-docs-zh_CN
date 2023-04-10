# Fuchsia Interface Definition Language

<<../../../_common/fidl/_fidl_intro.md>>

## Creating a FIDL library

**FIDL libraries** group FIDL source files together. A library acts as a
namespace for the protocols it contains, and FIDL source files can implicitly
access all other declarations within the same library. FIDL source files must
**import** any declarations from another library.

The Fuchsia SDK provides the `fuchsia_fidl_library()` build target to compile
FIDL source files into a library. The name of the library target must match the
`library` declarations in each source file. See the following `BUILD.bazel`
example for the `fuchsia.examples` library:

```bazel
# Import the fidl template.
load("fuchsia_fidl_library")

# Define a FIDL library target.
fuchsia_fidl_library(
    name = "fuchsia.examples",
    srcs = [
        "echo.fidl",
    ],
    library = ""fuchsia.examples"",
    visibility = ["//visibility:public"],
)
```

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
 function calls into FIDL messages for transport across
  process boundaries.](/docs/get-started/images/fidl/fidl-bindings.png){: width="574"}

Note: For more details on the bindings specification and supported programming
languages, see the [Bindings Reference](/docs/reference/fidl/bindings/overview.md).

At build time, the `fidlgen` backend tools generate bindings for supported
programming languages from the JSON IR library produced by `fidlc`.
The Fuchsia SDK provides build templates to generate bindings for each supported
language. See the following `BUILD.bazel` example to generate HLCPP bindings for
the `fuchsia.examples` library:

```bazel
fuchsia_fidl_hlcpp_library(
    name = "fuchsia.examples.fidl_cc",
    library = ":fuchsia.examples",
    visibility = ["//visibility:public"],
    deps = [
        "@fuchsia_sdk//pkg/fidl_cpp",
    ],
)
```

Components that consume this library can use the bindings target as a dependency.

## Exercise: Echo FIDL Library

In this section, you'll define a new FIDL library with a protocol called
`Echo` containing a single method that returns string values back to the
caller.

### Create the FIDL library

Start by creating a new directory for the FIDL library target:

```posix-terminal
mkdir -p fuchsia-codelab/echo-fidl
```

After you complete this section, the project should have the following directory
structure:

```none {:.devsite-disable-click-to-copy}
//fuchsia-codelab/echo-fidl
                  |- BUILD.bazel
                  |- echo.fidl
```

Add a new FIDL interface file called `echo.fidl` with the following contents:

`echo-fidl/echo.fidl`:

```fidl
{% includecode gerrit_repo="fuchsia/sdk-samples/getting-started" gerrit_path="src/routing/fidl/echo.fidl" region_tag="fidl_echo" adjust_indentation="auto" %}
```

`EchoString` is a two-way method that accepts an optional (nullable) string
value and returns the same value.

Add a `BUILD.bazel` file with the following contents to declare the library
target:

`echo-fidl/BUILD.bazel`:

```bazel
{% includecode gerrit_repo="fuchsia/sdk-samples/getting-started" gerrit_path="src/routing/fidl/BUILD.bazel" region_tag="imports" adjust_indentation="auto" %}

{% includecode gerrit_repo="fuchsia/sdk-samples/getting-started" gerrit_path="src/routing/fidl/BUILD.bazel" region_tag="fidl_library" adjust_indentation="auto" %}
```

Run `bazel build` and verify that the build completes successfully:

```posix-terminal
bazel build --config=fuchsia_x64 //fuchsia-codelab/echo-fidl:examples.routing.echo.fidl_cc
```

### Examine the FIDL bindings

The FIDL bindings target compiles the FIDL interface and generates
language-specific bindings in the `bazel-bin/` directory:

```none {:.devsite-disable-click-to-copy}
bazel-bin/fuchsia-codelab/echo-fidl/_virtual_includes/
```

Locate and open the generated headers found in the above directory:

```posix-terminal
find bazel-bin/fuchsia-codelab/echo-fidl/_virtual_includes -name *.h
```

Explore the contents of these files. Below is a summary of some of the key
generated interfaces:

<table>
  <tr>
  <th><strong>Interface</strong>
  </th>
  <th><strong>Description</strong>
  </th>
  </tr>
  <tr>
  <td><code>Echo</code>
  </td>
  <td>
    Represents the FIDL protocol and acts as the main entry point for types and
    classes that both clients and servers use to interact with that protocol.
  </td>
  </tr>
  <tr>
  <td><code>EchoString</code>
  </td>
  <td>
    Representation of the FIDL protocol method, used by clients and servers to
    provide typed requests and responses.
  </td>
  </tr>
  <tr>
  <td><code>fidl::Server&lt;Echo&gt;</code>
  </td>
  <td>
    Abstract class for a server component to override and handle incoming FIDL
    requests.
  </td>
  </tr>
  <tr>
  <td><code>fidl::internal::NaturalClientImpl&lt;Echo&gt;</code>
  </td>
  <td>
    Asynchronous client that transforms protocol methods into FIDL request
    messages sent over the IPC channel. Provided when a <code>fidl::Client</code>
    is created for this protocol.
  </td>
  </tr>
  <tr>
  <td><code>fidl::internal::NaturalSyncClientImpl&lt;Echo&gt;</code>
  </td>
  <td>
    Synchronous client that transforms protocol methods into FIDL request
    messages sent over the IPC channel. Provided when a <code>fidl::SyncClient</code>
    is created for this protocol.
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
<a href="/docs/reference/fidl/bindings/overview">Bindings Reference</a>.</d>
</aside>
