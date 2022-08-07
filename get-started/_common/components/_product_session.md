## Session framework

Sessions are components that encapsulate a product’s user experience. The
session framework serves as a boundary between the Fuchsia platform and the
product-level user interaction. Each Fuchsia product defines a single session
instance as the root of the product experience, which may or may not manage
additional child components.

The `session_manager` platform component starts the session component on boot
and offers it a fixed set of capabilities necessary to support the session
framework APIs for elements such as window management, input event handling, or
accessibility.

Note: For more details on the session framework, see
[Introduction to the session framework](/concepts/session/introduction.md).
