# Accessibility focus

## Overview

The Accessibility Focus (or the a11y focus), tracks which element on the screen a user using some assistive technology is focusing on. At the moment, only the Fuchsia Screen Reader keeps track of the a11y focus.

More specifically, the a11y focus is the currently focused [semantic node][semantics] of the Scenic view That is the [terminal view of the focus chain][focus-chain].

It is important to also understand that [input focus][input-focus] normally matches, but is not, the same thing as the a11y focus. It can be the case that a user is focusing an input text field with the Screen Reader on, in which case the input focus and the a11y focus would match. However, if the keyboard for a text field is shown but the user is exploring other parts of the screen with the Screen Reader, the a11y focus is different from the input focus.

## Recoverable a11y Focus

The a11y focus is permanent, even when the user changes to a different view. This means that the a11y manager  caches the semantic node that is focused in a view, referencing the view by its viewref KoID. If the [focus chain][focus-chain] is updated to point to a view and a node was previously focused there, the a11y focus will be updated to point to that same node.

This is necessary, for example, to allow users to save their location in an application, even when they switch between views displaying content. In a traditional desktop environment, this could be the simple use case of having a tab open, then doing an alt tab to change some music in a music player, and then returning to the tab. The location of where the user was on the page would be restored and they could continue from there.

## Changes in the Accessibility Focus

Changes in the a11y focus cause the Screen Reader to speak the new focused semantic node, so that users are aware of a navigational change.

## Accessibility Focus recovery strategies

For now, if a semantic node that had the a11y focus is destroyed, the strategy to recover the a11y focus is to traverse the semantic tree in Breadth First Search and select [the first describable semantic node][describable-nodes].

Other strategies exist and are expected to be implemented as the platform matures.

## Future work

Other assistive technologies may bennefit from the a11y focus, such as a switch access. This feature is not implemented yet.

[semantics]: concepts/accessibility/semantics.md
[focus-chain]: development/graphics/scenic/concepts/focus_chain.md
[input-focus]: https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=95672
[describable-nodes]: https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=96257
