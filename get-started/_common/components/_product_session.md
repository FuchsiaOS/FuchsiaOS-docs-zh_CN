## Session components

Sessions are components that encapsulate a product’s user experience. The
session component serves as a boundary between the Fuchsia platform and the
product-level user interaction. Each Fuchsia product defines a single session
instance as the root of the product experience, which may or may not manage
additional child components.

The `session_manager` platform component starts the session component on boot
and offers it a set of capabilities necessary to support the product experience
such as window management, input event handling, or accessibility.