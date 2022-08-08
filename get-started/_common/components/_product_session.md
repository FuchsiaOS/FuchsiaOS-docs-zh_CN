<!-- ## Session framework -->
## 会话框架

<!-- Sessions are components that encapsulate a product’s user experience. The
session framework serves as a boundary between the Fuchsia platform and the
product-level user interaction. Each Fuchsia product defines a single session
instance as the root of the product experience, which may or may not manage
additional child components. -->
会话是一系列封装了产品用户体验的组件。会话框架充当 Fuchsia 平台和产品级用户之间交流的边界。每个 Fuchsia 产品都将单个会话实例定义为产品体验的根，它可能管理，也可能不管理其他子组件。


<!-- The `session_manager` platform component starts the session component on boot
and offers it a fixed set of capabilities necessary to support the session
framework APIs for elements such as window management, input event handling, or
accessibility. -->
`session_manager` 平台组件在启动时打开会话组件，并为其提供一组固定的功能，以支持诸如窗口管理、输入事件处理或可访问元素的会话框架 API。

<!-- Note: For more details on the session framework, see
[Introduction to the session framework](/concepts/session/introduction.md). -->
注: 要获取关于会话框架的更多细节，请参阅[会话框架介绍](/concepts/session/introduction.md).
