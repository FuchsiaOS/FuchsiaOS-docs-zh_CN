<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0166" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
{# Fuchsia RFCs use templates to display various fields from _rfcs.yaml. View the #}
{# fully rendered RFCs at https://fuchsia.dev/fuchsia-src/contribute/governance/rfcs #}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Summary

This RFC clarifies the plan of record with respect to legacy components and APIs
used in the graphics, input and accessibility stacks (also called the "UI
stack") in the Fuchsia platform. It outlines the planned order of migrations to
get all Fuchsia product configurations using the same stack for graphics, input
and accessibility. It also clarifies testing standards around these
transitions. Specifically:

* The Root Presenter component is deprecated in favor of Scene Manager.
* The Input Pipeline library will run as part of Scene Manager on products with
  graphics.
* All contracts between UI components should be covered by platform integration
  tests to ensure consistent behavior across transitions.
* Scenic's GFX API is deprecated in favor of Flatland.

## Motivation

The Fuchsia Platform currently contains multiple ways of

* Processing user input
* Creating a graphical scene
* Wiring up accessibility services

Different Fuchsia product configurations currently use different subsets of this
functionality. This leads to confusion and difficulty debugging across different
products.

This RFC seeks to clarify which of the components and APIs involved are
deprecated, which are plan of record, and to ratify the planned order of the
migrations. It provides additional context on the architecture discussed in
[RFC-0096: User input
architecture](/docs/contribute/governance/rfcs/0096_user_input_arch.md) and
[RFC-0147: View System](/docs/contribute/governance/rfcs/0147_view_system.md).

## Stakeholders

Who has a stake in whether this RFC is accepted? (This section is optional but
encouraged.)

_Facilitator:_

cpu@google.com

_Reviewers:_

* Scenic, runtime integrations: emircan@google.com, jjosh@google.com,
  dworsham@google.com
* Accessibility: lucasradaelli@google.com
* Input: quiche@google.com, jaeheon@google.com

_Consulted:_

yeg@google.com

_Socialization:_

A detailed version of this plan was reviewed internally with Scenic, Input and
Accessibility teams.

## Glossary

* [Scenic](/docs/concepts/ui/scenic/index.md) - Fuchsia platform component that handles
  graphical composition, focus management, and routing of graphical input.
* [Gfx](/docs/concepts/ui/scenic/gfx/index.md) - Legacy Scenic graphics API. Found in
  [fuchsia.ui.scenic](https://fuchsia.dev/reference/fidl/fuchsia.ui.scenic) and
  [fuchsia.ui.gfx](https://fuchsia.dev/reference/fidl/fuchsia.ui.gfx/).
* [Flatland](/docs/concepts/ui/scenic/flatland/index.md) - Plan of record Scenic
  API. Found in
  [fuchsia.ui.composition](https://fuchsia.dev/reference/fidl/fuchsia.ui.composition).
* [Input
  Pipeline](https://cs.opensource.google/fuchsia/fuchsia/+/main:src/ui/bin/input-pipeline/) -
  Fuchsia library that handles routing and policy for input events. May run as
  part of Scene Manager or as a standalone component.
* [Scene
  Manager](https://cs.opensource.google/fuchsia/fuchsia/+/main:src/ui/bin/scene_manager/) -
  Fuchsia platform component that handles the setup and configuration of the
  Scenic scene, as well as starting the input pipeline. Currently missing some
  functionality, especially with respect to accessibility.
* [Root
  Presenter](https://cs.opensource.google/fuchsia/fuchsia/+/main:src/ui/bin/root_presenter/) -
  Legacy Fuchsia platform component that handles input routing and setup and
  configuration of the Scenic scene.
* [Accessibility Manager](/docs/concepts/accessibility/accessibility_framework.md) -
  Fuchsia platform component that handles accessibility services.

## Design

### Legacy vs. desired state

![Diagram showing Legacy vs. desired UI stack as described
below](resources/0166_ui_stack/legacyUI.svg "Legacy vs. Desired")

In the legacy UI stack, Root Presenter is responsible for interpreting input
events from the driver stack and for instantiating the scene graph using
GFX. Root Presenter dispatches input events to Scenic, which sometimes forwards
them to Accessibility Manager. Root Presenter sends commands to the
Accessibility Manager to control magnification and color correction. Flutter and
Chromium create views using GFX.

In the desired/future UI stack, Scene Manager is responsible for interpreting
input events from the driver stack and for instantiating the scene graph using
Flatland.  The Input Pipeline is instantiated as part of the Scene Manager
component and dispatches input events to Scenic. Scenic sometimes forwards input
events to the Accessibility Manager. Scene Manager sends commands to the
Accessibility Manager to control magnfication and color correction. Flutter and
Chromium create views using Flatland. The Virtual Keyboard Controller is a
separate component that is only responsible for passing messages about onscreen
keyboard state.

### What is changing

#### Deprecate Root Presenter

Scene management and input functionality will be removed from Root Presenter,
which will be renamed to "Virtual Keyboard Manager" to reflect the remaining
functionality. This removes a large amount of mostly-untested legacy code.

Any Root Presenter features not currently supported in Scene Manager
(e.g. Accessibility Magnification) will be implemented in Scene Manager.

#### Integration tests

All features of the UI stack (graphics, different forms of user input,
accessibility) will now require hermetic integration tests to ensure that the
contracts between platform components remain stable.

#### GFX to Flatland

All references to and implementations of the GFX API will be removed from
fuchsia.git. GFX is a legacy 3D API that is significantly more complex than is
warranted by the use cases Fuchsia serves today. Removing it simplifies the
system and removes a non-trivial chunk of poorly-understood API surface.

This also allows runtime implementations (Flutter, Chromium) to remove GFX
support from their codebases.

### Order of deprecation

To move all products to the same desired configuration, we must make changes to
both the component topology (replacing Root Presenter with Scene Manager) as
well as which API is used (GFX vs. Flatland). In order to provide solid test
coverage for the graphics API transition, we will first align all products on
the correct component topology. Once this is complete, we transition any
products remaining on GFX to Flatland.


## Implementation

In all phases migrations should be controlled with build flags to allow for easy
manual testing and rollbacks.

### Milestone 0 (completed): Flatland on Workstation only

The workstation configuration moved to use Flatland for Graphics in Q1 2022.

### Milestone 1 (completed): Other products move to Input Pipeline

As of Q2 2022 all other Fuchsia products moved to use the Input Pipeline for
input event routing (instead of Root Presenter). In products without Scene
Manager it temporarily runs as a standalone component.

### Milestone 2: Transition all products to Scene Manager

Currently Scene Manager lacks a number of features present in Root
Presenter. The work to bring Scene Manager to feature parity with Root Presenter
is tracked in
[fxbug.dev/98687](https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=98687).
Once this is completed and all features are covered by platform integration
tests we will transition any remaining products to use Scene Manager + GFX.

After this transition, input and scene management code in Root Presenter will be
removed. Root Presenter's virtual keyboard controller functionality will remain
and the component will be renamed to reflect its reduced set of
responsibilities.

### Milestone 3: Transition remaining products to Flatland

There is additional work in Flatland to cover all features supported in
GFX. This work is tracked in
[fxbug.dev/93979](https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=93979).
Once this work is completed and fully covered by platform integration tests we
will transition remaining products to Flatland.

After this is landed and stable, the API and implementation for GFX may be
removed. Note that this includes code in the Scenic component as well as
integration code in multiple UI frameworks.

## Performance

Milestone 3 could lead to performance improvements for some product
configurations due to efficiencies in Flatland. This is expected when Flatland
is able to delegate compositing to the display controller (i.e. without
involving the GPU), which will be available on some device types. Performance
should be monitored through each transition but is not expected to worsen.

## Security and Privacy considerations

This RFC does not introduce any new security or privacy concerns not already
covered by [Flatland API
reviews](/docs/contribute/governance/rfcs/0162_flatland.md).
No user data will be collected.

## Testing

At each stage of the migration, we will gate transitions on full integration
test coverage of the affected features. Because Root Presenter has historically
lacked such coverage this should lead to an overall improvement in coverage for
the UI stack.

## Documentation

References to Root Presenter and GFX should be removed from documentation as
milestones are completed.

## Drawbacks, alternatives, and unknowns

This is a complex, multi-quarter migration which will take a substantial
engineering effort to complete.

### Do nothing

We could continue to maintain the legacy functionality for the lifespan of all
products that use it. Historically this maintenance has been substantial due to
continuous changes in the rest of the platform as well as a lack of
comprehensive tests. Multiple UI stacks are also confusing, leading to
uncertainty about what code was actually running on a given product. While the
migration is costly, maintaining the status quo will require more effort over
time with worse results and depressed engineering velocity.

### Transition to Flatland first

This approach would require significant work to add Flatland support to Root
Presenter. Because this code lacks comprehensive test coverage, this is risky
and requires a large amount of throw-away work.

## Prior art and references

* [RFC-0096: User input
architecture](/docs/contribute/governance/rfcs/0096_user_input_arch.md)
* [RFC-0147:View System](/docs/contribute/governance/rfcs/0147_view_system.md)
