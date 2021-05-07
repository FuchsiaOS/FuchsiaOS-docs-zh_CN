# Graphical Presenter component role

## Overview

[Graphical Presenter] is a component role, within the session, that presents
multiple graphical views to the user; most commonly, views from the session's
[Elements].

Although the same session component could also implement the Graphical Presenter
roll, there is a [FIDL](#fidl) protocol, [`fuchsia.element.GraphicalPresenter`],
that allows a [session] component to delegate Graphical Presenter
responsibilities another component. This makes it possible for each component to
execute in different runtimes (such Rust versus Flutter/Dart) if needed.

## Displaying element views

For Fuchsia-based products running on devices with a graphical display, a
session implementation manages displayable components through the [Element API].
Each element renders its graphical representation in what is known as a "view".

A session will request the Element's view, and then pass that view to the
Graphical Presenter to be displayed, along with optional [Element
Annotations], used to communicate product-specific
presentation properties.

## PresentView request {#present-view}

The session calls the `GraphicalPresenter` method [`PresentView()`] to display a
given view. `PresentView()` takes:
1. A [`ViewSpec`], through which the session conveys where and how it wants the
view to be displayed in the [`Scenic`] scene
graph.
2. An `AnnotationController`, which is a handle to the element's annotations, through
which the presenter can update the respective element's annotations.

In return, the caller is given a handle to
the view, as a [`ViewController`]. The caller can destroy
the view (remove it from the display) by releasing or closing its `ViewController`.

The developer guide includes an [example implementation of presenting an
element's view][example-present-view].

[component]: /docs/glossary.md#component
[components]: /docs/glossary.md#component
[Element API]: /docs/concepts/session/element.md
[Elements]: /docs/glossary.md#element
[Element Annotations]: /docs/concepts/session/element.md#element-annotations
[example-present-view]: /docs/development/sessions/roles-and-responsibilities.md#presenting-an-elements-view
[`fuchsia.element.GraphicalPresenter`]: https://fuchsia.dev/reference/fidl/fuchsia.element#GraphicalPresenter
[Graphical Presenter]: /docs/glossary.md#graphical-presenter
[`PresentView()`]: https://fuchsia.dev/reference/fidl/fuchsia.element#fuchsia.element/GraphicalPresenter.PresentView
[`Scenic`]: /docs/glossary.md#scenic
[session]: /docs/glossary.md#session
[`ViewController`]: /docs/glossary.md#view-controller
[`ViewSpec`]: /docs/glossary.md#view-spec
