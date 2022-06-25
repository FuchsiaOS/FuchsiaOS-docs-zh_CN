# Graphical Presenter component role

## Overview

[Graphical presenter][glossary.GraphicalPresenter] is a component role, within
the session, that presents multiple graphical views to the user; most commonly,
views from the session's [components][glossary.component].

Although the same session component could also implement the Graphical Presenter
role, there is a [FIDL][glossary.FIDL] protocol,
[`fuchsia.element.GraphicalPresenter`],
that allows a [session][glossary.session]
component to delegate Graphical Presenter responsibilities another component. This makes it possible
for each component to execute in different runtimes
 (such Rust versus Flutter/Dart) if needed.

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
1. A [ViewSpec][glossary.ViewSpec], through which the session conveys where and how it wants the
view to be displayed in the [scenic][glossary.scenic] scene
graph.
2. An `AnnotationController`, which is a handle to the element's annotations, through
which the presenter can update the respective element's annotations.

In return, the caller is given a handle to
the view, as a [ViewController][glossary.ViewController]. The caller can destroy
the view (remove it from the display) by releasing or closing its `ViewController`.

The developer guide includes an [example implementation of presenting an
element's view][example-present-view].

[glossary.GraphicalPresenter]: glossary/README.md#GraphicalPresenter
[glossary.component]: glossary/README.md#component
[glossary.FIDL]: glossary/README.md#FIDL
[glossary.session]: glossary/README.md#session
[glossary.ViewSpec]: glossary/README.md#ViewSpec
[glossary.scenic]: glossary/README.md#scenic
[glossary.ViewController]: glossary/README.md#ViewController
[Element API]: concepts/session/element.md
[Element Annotations]: concepts/session/element.md#element-annotations
[example-present-view]: development/sessions/roles-and-responsibilities.md#presenting-an-elements-view
[`PresentView()`]: https://fuchsia.dev/reference/fidl/fuchsia.element#fuchsia.element/GraphicalPresenter.PresentView
[`fuchsia.element.GraphicalPresenter`]: https://fuchsia.dev/reference/fidl/fuchsia.element#GraphicalPresenter
