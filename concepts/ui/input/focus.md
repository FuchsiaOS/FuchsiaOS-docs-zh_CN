# Focus

The user typically interacts with one UI window at a time. At a high level,
*focus* identifies one UI window which gets the user's interaction.

## Focus identifies one view

A UI window is constructed with a Fuchsia [view][view-glossary], typically just
one. Focus moves between these views, and those views must be connected to the
global view tree. When focus moves to a view, that view is notified that it has
gained focus, and the previously focused view is notified that it has lost
focus.

There is always a focused view. If a focused view becomes disconnected from the
view tree, it loses focus, and the view's parent gains focus.

## UI client's use of focus is app-specific

When a UI client's view gains focus, the UI client can use that bit to drive
app-specific user-interaction behavior. For example, it could start blinking the
cursor to indicate that its view is active, or intensify widget colors, or
perform UI layout changes.

## Focus and input modalities

Some input modalities, like keyboard and shortcut, build their interaction
models on top of focus. For example, only the focused view may interact with the
user's keyboard. A view that loses focus cannot receive keyboard events.

Other input modalities, like touch and mouse, may play a stronger role in
driving focus changes. Some examples are given below.

When new input modalities are added to Fuchsia, they will need design work to
fit alongside existing focus models.

## Each product defines when focus moves

Fuchsia has some rules on the *placement* of focus, but it doesn't say much
about the *movement* of focus. Mechanically, focus is moved between views
through the use of [Focuser][focuser-api] API calls, but each product should
define its own policy on when focus ought to move between views.

For example, a workstation-type product can choose to move focus to the view
that is under a mouse cursor's click. Or a touchscreen-type product can choose
to move focus to the view that is under a user's finger, when it initially
touches the screen. Or an accessibility feature can choose to move focus to a
view, driven by vocal commands or special gestures.

## Each product builds on focus to make its own guarantees

Through creative use of focus rules, a product can construct guarantees specific
to its own use. For example, a workstation-type product may wish to ensure that
a lockscreen cannot "leak" keyboard events to any UI window behind the lock
dialog. If a UI window's view is disconnected from the view tree, it cannot
receive focus, and thus cannot interact with the keyboard.

[view-glossary]: /docs/glossary#view
[focuser-api]: https://fuchsia.dev/reference/fidl/fuchsia.ui.views#Focuser
