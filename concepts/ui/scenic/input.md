# Scenic input

Many input modalities are associated with a particular [View][view-glossary].
Events that are described in View coordinates (touchscreen, mouse) are handled
by Scenic, while others (keyboard, touchpad gestures) rely on the View Tree
topology but are not directly dispatched by Scenic.

## Pointer events dispatched to a View

Scenic handles input modalities that are described in View coordinates. For
example, Scenic dispatches touchscreen events and mouse events to a particular
View, and these events have a concrete position in that View's coordinate
system. Commonly, we call these "pointer events".

## View target determined with hit-testing

Scenic must decide which View should receive pointer events. Typically,
[hit-testing][hit-testing] is used to find the geometrically top most View,
which Scenic then uses as the target of event dispatch.

[view-glossary]: /docs/glossary#view
[hit-testing]: /docs/concepts/ui/scenic/hit_testing.md
