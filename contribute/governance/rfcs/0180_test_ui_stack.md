<!-- Generated with `fx rfc` -->
<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0180" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
{# Fuchsia RFCs use templates to display various fields from _rfcs.yaml. View the #}
{# fully rendered RFCs at https://fuchsia.dev/fuchsia-src/contribute/governance/rfcs #}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Summary

This RFC is a design proposal for the Test UI Stack component, which will
provide ui-specific testing capabilities to test clients both in- and out-
of-tree.

## Motivation

Integration testing is essential for Fuchsia petal stability. However, there
are few out-of-tree ("OOT") hermetic UI integration tests today, because OOT
clients face significant barriers to writing them. Namely, they must:

- Understand esoterica of the UI stack, often well beyond the scope of their
  production use cases.
- Couple test behavior to internal implementation details of the UI stack.
- Use internal-only FIDL APIs to run the relevant UI components.
- Design tests to be resilient to the various migrations in progress within the
  UI stack (GFX -> flatland, root presenter -> scene manager, CFv1 -> CFv2).

The Test UI Stack component aims to mitigate these issues by handling low-level
UI details on behalf of test clients.

### Example use cases

- **Touch/mouse/keyboard input tests for client runtimes**: For these tests,
  the client would bring up the Test UI Stack, attach a view to the scene, wait
  until the view tree state quiesces, inject input, and observe how its view
  handled the incoming events.
- **Application tests**: These tests could run some portion of an application
  against the UI stack, and observe how it renders content, handles input,
  interacts with accessibility, etc.
- **UI-adjacent tests**: Some tests that don't explicitly exercise UI
  functionality may still require a UI presence. For example, tests for
  [fuchsia.web](https://cs.opensource.google/fuchsia/fuchsia/+/main:sdk/fidl/fuchsia.web/)
  may require the test client to present a view.

## Stakeholders

Who has a stake in whether this RFC is accepted? (This section is optional but
encouraged.)

_Facilitator:_

leannogasawara@google.com

_Reviewers:_

- Fuchsia Testing Architecture: crjohns@google.com
- UI + OOT Integration Testing: dworsham@google.com, jaeheon@google.com

_Consulted:_

List people who should review the RFC, but whose approval is not required.

- Input: quiche@google.com, neelsa@google.com
- Accessibility: neelsa@google.com, lucasradaelli@google.com
- Component Framework: yaneury@google.com, geb@google.com, cgonyeo@google.com
- Flutter: akbiggs@google.com
- Chromium: sergeyu@google.com
- Opal: cligh@google.com, anwilson@google.com, robinsontom@google.com

_Socialization:_

This RFC went through a design review with the Fuchsia Testing and Fuchsia
Input teams. We also consulted OOT UI client teams.

## Glossary

- **UI stack**: The set of Fuchsia components that vend essential UI services.
  Roughly, this set includes scenic, root presenter or scene manager, input
  pipeline, accessibility manager, shortcut manager, and text manager.
- **Test UI Stack**: Proposed component that exposes a facade of the UI stack
  (base UI services + helper services).
- **Base UI services**: The set of services exposed out of the [production ui
  realm](https://cs.opensource.google/fuchsia/fuchsia/+/main:src/ui/meta/ui.cml).
- **Helper services**: Test-only services that proxy low-level UI capabilities
  to clients via higher-level APIs.
- **Scene**: The hierarchy of [views](/docs/development/graphics/scenic/concepts/view_ref.md),
  which enable owners to present renderable content to a display, consume input,
  and interact with accessibility.

## Design

### Requirements

- Hermetic OOT integration tests involving UI must be easy to write.
- Hermetic OOT integration tests involving UI must be readable.
- Test affordances must not leak internals of the UI stack.
- Clients must be able to configure a test realm containing the UI stack.
- Clients must be able to extend the component topology defined by Test UI
  Stack to include test-specific configuration.
- Tests must be guaranteed to set up and tear down cleanly.
- Tests must be isolated from one another.
- Test UI Stack must not preclude use of RealmBuilder.
- Clients must be able to write tests in a language of their choice.

### Nice-to-have

- In-tree and out-of-tree UI integration tests should be directly
  analogous.

### Overview

We propose a Test UI Stack component, to be added to the Fuchsia partner SDK,
which will expose a facade of the production UI realm. Concretely, this
component will expose the following services:

1. Roughly, the set of public services exposed out of the production ui realm.
2. A set of test-only "helper services" that vend low-level UI capabilities
   through higher-level abstractions (e.g. input synthesis, screenshot, etc.).

A client can instantiate this component, route required UI services to the
component(s) under test, present a view to the scene, and use the various helper
services provided to drive its tests.

![Alt text:
Component topology shown:
Test manager -> Test fixture component
Test fixture component -> test ui stack component
Test fixture component -> test ui client
Test fixture component -> supporting components
Test fixture component -> local mocks
Test ui stack component  -> helper components
Test ui stack component -> base ui components Service routes
Helper components -> test ui stack component (helper services)
Base ui components -> test ui stack component (base ui services)
Test UI stack component -> test fixture component (base ui services, helper
services)
Test UI stack component -> test ui client (base ui services)
Test UI client -> test fixture component (fuchisa.ui.app.ViewProvider)
Supporting components -> test UI client (supporting services)
Local mocks -> test ui client (mocked services)
](resources/0180_test_ui_stack/test_ui_stack_topology.png)


Note that this design is agnostic to how the UI stack and test component
configure their respective realms. They could do so statically or via
[RealmBuilder](/docs/development/testing/components/realm_builder.md).

## Base UI topology

Initially, the Test UI Stack will include the following base UI components,
which mirror the "modern" production UI stack at the time of writing:

1. Scenic, configured to use flatland.
2. Scene manager
3. Accessibility Manager
4. Text manager
5. Shortcut manager
6. Cobalt (not a UI component, but required to run scenic)
7. Fake hardware display controller (again, not a UI component, but required
   by scenic)

Furthermore, the Test UI Stack will initially expose the following base UI
services to the test:

1. fuchsia.accessibility.semantics.SemanticsManager
2. fuchsia.ui.composition.Allocator
3. fuchsia.ui.composition.Flatland
4. fuchsia.ui.scenic.Scenic
5. fuchsia.ui.input.ImeService
6. fuchsia.ui.input3.Keyboard
7. fuchsia.ui.input3.KeyEventInjector
8. fuchsia.ui.shortcut.Manager
9. fuchsia.ui.shortcut.Registry

Note that the Test UI Stack can and will evolve to mirror the production one.

Depending on the progress of the [One UI Stack migration](/docs/contribute/governance/rfcs/0166_ui_stack.md),
we may also add a "legacy" variant of the Test UI Stack component that uses
root presenter and input pipeline in place of scene manager.

### Helper components

Along with the base UI components referenced above, the Test UI Stack will
include a set of narrowly-scoped helper components to provide low-level
UI-specific capabilities to clients via higher-level APIs. At launch, this
set may include:

1. An input synthesis component, which enables clients to inject text, mouse,
and touch input directly into the input pipeline.
2. A screenshot component, which enables clients to take screenshots
ergonomically.
3. A scene provider component, used to attach client views to the scene and
register privileged affordances on their behalf (e.g. scoped geometry
observers). Note that since the scene provider registers observers on behalf
of the client, the Test UI Stack need *not* expose any observer registry
services.

The Test UI Stack will expose helper services from these components, which
clients can use to drive tests.

The helper component abstraction offers several important benefits:

- Abstraction: The helper services present a stable, well-defined facade to
  clients, which helps minimize dependencies on internals of the UI stack.
- Simplicity: Vending ui capabilities through dedicated helper components
  keeps each FIDL API simple and specific, which improves DX for authoring and
  maintaining UI tests.
- Extensibility: We can easily expand our UI facade by adding new helper
  components.
- Compatibility with subpackages: We can transition to subpackages with no
  loss-of-function for clients.

### Configurability

Some clients may need to configure parameters like display rotation, pixel
density, etc. The Test UI Stack can accommodate these use cases with [structured
component
configuration](/docs/development/components/configuration/structured_config.md).
Clients can override parameters they wish to control,
which the Test UI Stack component can then propagate to the appropriate base UI
components.

### Example Use

The pseudo-C++ snippet below outlines a basic touch input test using the Test UI
Stack component.

```
// Client test code creates a RealmBuilder instance.
component_testing::RealmBuilder realm_builder;

// Instantiate Test UI Stack component by absolute URL in the test realm.
realm_builder.AddChild("test-ui-stack",
            "fuchsia-pkg://fuchsia.com/test-ui-stack#meta/test-ui-stack.cm");

// Add a test view component to the test realm, and route required UI services
// to it.
realm_builder.AddChild("test-view", ...);
realm_builder.AddRoute({
    .capabilities = {Protocol{fuchsia::ui::scenic::Scenic::Name_}},
    .source = ChildRef{"test-ui-stack"},
    .targets = {"test-view"}},
}});

// Expose fuchsia.ui.app.ViewProvider from the test view.
realm_builder.AddRoute({
    .capabilities = {Protocol{fuchsia::ui::app::ViewProvider::Name_}},
    .source = ChildRef{"test-view"},
    .targets = {ParentRef()}},
}});

// Build the test realm.
RealmRoot realm_root = realm_builder_.Build();

// Connect to the scene provider "helper service", and request to attach a
// test view to the scene.
std::optional<zx_koid_t> client_view_ref_koid;
fuchsia::ui::observation::geometry::Provider geometry_provider;
auto scene_provider = realm_root->Connect<fuchsia::ui::test::scene::Provider>();
auto view_provider = realm_root_->Connect<fuchsia::ui::app::ViewProvider>();
scene_provider->AttachView(std::move(view_provider), geometry_provider.NewRequest(),
  [&client_view_ref_koid](auto view_ref_koid) {
    // Save the client's ViewRef koid.
    client_view_ref_koid = view_ref_koid;
  });

// Wait for client view ref koid to become available.
RunLoopUntil([&client_view_ref_koid] {
  return client_view_ref_koid.has_value();
});

// Use registered geometry provider to wait for client view to render.
ASSERT_TRUE(geometry_provider.is_bound());
geometry_provider.Watch(...);
RunLoopUntil(...);

// Connect to input synthesis helper service, and use to inject input.
auto input_synthesis = realm_root->Connect<fuchsia::ui::test::input::Touch>();
input_synthesis->InjectTap(...);
```

## Implementation

Workstreams enumerated below can proceed in parallel.

### Workstream: Scene provider helper service

1. Land FIDL changes.
2. Implement scene provider helper component.
3. Refactor existing in-tree tests to use scene provider.

This workstream enables test to attach a view to the scene, which is a hard
reqeuirement for any graphics/input test.

### Workstream: Geometry observer

1. Make `fuchsia.ui.observation.geometry` protocol available OOT in the SDK.
2. Implement "scoped" geometry observer registry.

This workstream enables OOT clients to use geometry observer data agnostic to
the root of the scene graph, which may vary across different products and UI
stack configurations.

### Workstream: Input synthesis

1. Redesign input synthesis API for OOT use.
2. Add input synthesis FIDL library to the SDK.
3. Implement FIDL library.

This workstream enables OOT Test UI Stack users to inject input; today, there is
no alternative.

### Workstream: Refactor in-tree UITestManager library

1. Factor realm configuration out of the existing internal UITestManger class,
   into a new UITestRealm class, which can be shared with Test UI Stack.
2. (Optional) Implement a mechanism to share .cml with the production ui
   subrealms. If not now, we should do this cleanup once the One UI Stack
   migration is complete.

Once the workstreams above have completed, we can assemble the Test UI Stack
package in the partner SDK, and add it to the product build(s) against which OOT
clients run their tests.

## Performance

This design targets integration tests that are already multi-component, so we
expect the proposed extensions of test topology to have minimal performance
implications.

Some OOT tests may actually see improved performance, because they can rely on
more stable synchronization patterns.

## Security considerations

There are no security considerations for this RFC. Since the test UI stack
doesn't consume any system capabilities (except sysmem and vulkan), it is
incapable of doing anything a normal end-user vulkan program couldn't do.

## Privacy considerations

The Test UI Stack has no access to private or sensitive resources, so there are
no privacy considerations for this RFC.

## Testing

We can achieve sufficient confidence in the Test UI Stack's behavior as we write
tests to use it.

## Documentation

We intend to publish a developer guide explaining how to use the Test UI Stack.

## Drawbacks, alternatives, and unknowns

### Drawbacks

#### Duplicate boilerplate across clients

The proposed design leaves clients with some common boilerplate to write. We
may be able to eliminate this pain point with a custom UI-specific
implementation of `fuchsia.test.Suite`, which would enable clients to plug a
test client and test logic into a predefined UI test framework.

### Alternatives considered

See original [UI Test Manager RFC](https://fuchsia-review.googlesource.com/c/fuchsia/+/691540/7).
