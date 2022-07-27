<!-- 
Fuchsia Interface Definition Language (FIDL) is the language used to describe
interprocess communication (IPC) protocols used by Fuchsia programs. FIDL
provides a simplified declaration syntax for providers to define interfaces as a
**protocol**. Supported data types include integers, floats, booleans, strings,
and [handles][glossary.handle]. These can be organized into more complex arrays,
vectors, structs, tables, and unions.
 -->
Fuchsia 接口定义语言（FIDL）是用来描述 Fuchsia 程序使用的进程间通信（IPC）协议的语言。FIDL 为供应商提供了一种简化的声明语法，以便将接口定义为一种**协议**。支持的数据类型包括整数、浮点数、布尔运算、字符串和[句柄][glossary.handle]。这些可以被组织成更复杂的数组、向量、结构体、表格和联合体。

<!-- 
Consider the following example FIDL protocol for an `Echo` interface:
 -->
考虑以下用于 `Echo` 接口的 FIDL 协议的示例。

```fidl
library fuchsia.examples;

{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples/echo.test.fidl" region_tag="max" adjust_indentation="auto" %}

{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples/echo.test.fidl" region_tag="echo" adjust_indentation="auto" %}
```

<!-- 
FIDL protocols describe a set of **methods** invoked by sending messages over
a channel. Channel messages are inherently asynchronous, with the sender and
receiver operating independently of each other. FIDL methods introduce
higher-level semantics that enable more idiomatic programming on the client and
server side of a FIDL transaction.
 -->
FIDL 协议描述了一组**方法**，在通过通道发送消息时被调用。通道消息本身是异步的，发送方和接收方彼此可以独立操作。FIDL 方法引入了更高层次的语义，使 FIDL 交易的客户端和服务器端的编程更加符合传统习惯。

<!-- 
FIDL supports the following method types:
 -->
FIDL 支持以下方法类型：

<!-- 
* **Two-way methods:** A typical method call that accepts optional parameters
  with a return type defined after the `->` operator. Two-way methods block
  until a response is received. In the `Echo` example, the `EchoString()`
  method is a two-way method.
 -->
* **双向方法**：一个典型的方法调用是，接受可选的参数，其返回类型定义在 `->` 操作符之后。双向方法会阻塞调用方直到收到响应。在 `Echo` 的例子中，`EchoString()` 方法是一个双向的方法。
<!-- 
* **One-way methods:** Asynchronous method calls that return immediately
  without waiting for a response. Methods without a declared return type are
  considered one-way from the client. In the `Echo` example, the `SendString()`
  method is a one-way method.
 -->
* **单向方法**：异步方法调用，立即返回而不等待响应。没有声明返回类型的方法被认为是客户端的单向方法。在 `Echo` 的例子中，`SendString()` 方法是一个单向的方法。
<!-- 
* **Events:** When necessary, a server may send unsolicited messages to the
  client, called events. Events declare their method name on the return side of
  the `->` operator. In the `Echo` example, the `OnString()` method is an event.
 -->
* **事件**：必要时，服务器端可以向客户端发送非请求信息，这种消息称为事件。事件在 `->` 操作符的返回端声明其方法名称。在 `Echo` 的例子中，`OnString()` 方法是一个事件。

<!-- 
Note: For more details on FIDL language syntax and supported types, see the
[FIDL Language specification](/reference/fidl/language/language.md).
 -->
注意：要获取 FIDL 语言语法和支持的类型的更多细节，请参阅 [FIDL 语言规范](/reference/fidl/language/language.md)。

[glossary.handle]: /glossary/README.md#handle
