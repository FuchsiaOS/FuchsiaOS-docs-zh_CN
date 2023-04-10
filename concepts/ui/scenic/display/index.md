# Display

Scenic mediates access to the physical displays that are attached to a Fuchsia
system. To let UI clients present content to a display, Scenic provides APIs
(e.g. [Flatland](/concepts/ui/scenic/flatland/index.md)) which allow
multiple client applications to show their content on-screen.

## Device pixel ratio and Screen orientation

Fuchsia products have a wide range of form factors, and have their own opinions
about the scale that app content should be displayed at, as well as the
on-screen orientation.

Scenic does not play a role in specifying such product policy. However, it does
provide the mechanisms that can be used to implement such policies. For example,
the transform hierarchy can be used by the System UI to scale and rotate the
scene as desired.

## Multiple displays and relative display placement

Fuchsia currently has minimal support for multiple displays. However, Scenic
will play a large role in this support. APIs will need to be added
([fxb/76985](https://fxbug.dev/76985), [fxb/117830](https://fxbug.dev/117830))
to allow clients (including the System UI) to:

* enumerate the available displays
* place content on specific displays
* implement "display spanning", which allows the mouse cursor, windows, etc.
  to be dragged between displays
* etc.
