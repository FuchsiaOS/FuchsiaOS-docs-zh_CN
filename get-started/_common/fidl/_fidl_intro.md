Fuchsia Interface Definition Language (FIDL) is the language used to describe
interprocess communication (IPC) protocols used by Fuchsia programs. FIDL
provides a simplified declaration syntax for providers to define interfaces as a
**protocol**. Supported data types include integers, floats, booleans, strings,
and [handles][glossary.handle]. These can be organized into more complex arrays,
vectors, structs, tables, and unions.

Consider the following example FIDL protocol for an `Echo` interface:

```fidl
library fuchsia.examples;

{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples/echo.test.fidl" region_tag="max" adjust_indentation="auto" %}

{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples/echo.test.fidl" region_tag="echo" adjust_indentation="auto" %}
```

FIDL protocols describe a set of **methods** invoked by sending messages over
a channel. Channel messages are inherently asynchronous, with the sender and
receiver operating independently of each other. FIDL methods introduce
higher-level semantics that enable more idiomatic programming on the client and
server side of a FIDL transaction.

FIDL supports the following method types:

* **Two-way methods:** A typical method call that accepts optional parameters
  with a return type defined after the `->` operator. Two-way methods block
  until a response is received. In the `Echo` example, the `EchoString()`
  method is a two-way method.
* **One-way methods:** Asynchronous method calls that return immediately
  without waiting for a response. Methods without a declared return type are
  considered one-way from the client. In the `Echo` example, the `SendString()`
  method is a one-way method.
* **Events:** When necessary, a server may send unsolicited messages to the
  client, called events. Events declare their method name on the return side of
  the `->` operator. In the `Echo` example, the `OnString()` method is an event.

Note: For more details on FIDL language syntax and supported types, see the
[FIDL Language specification](reference/fidl/language/language.md).

[glossary.handle]: glossary/README.md#handle
