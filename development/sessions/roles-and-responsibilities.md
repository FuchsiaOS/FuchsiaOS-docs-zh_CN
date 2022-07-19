# Session roles and responsibilities, by example

The session component encapsulate a product experience, within a component
topology. It launches all of the product-specific components (directly or
indirectly), manages the lifecycle of those components, and manages the flow of
control and information between the product-specific components (such as
user-facing apps) and device-specific components (such as input devices, audio,
and a display, if any).

The following sections demonstrate how a session author might implement some of
these responsibilities.

## Presenting an element's view {#presenting-an-elements-view}

In the following example, a session forwards an `[Elements](/glossary/README.md#element)` view to a
`[GraphicalPresenter](/glossary/README.md#graphicalpresenter)`,
by calling `PresentView()` with a
`[ViewSpec](/glossary/README.md#viewspec)`. The
`ViewSpec` includes the duplicated `[ViewRef](/glossary/README.md#ViewRef)` \(a sharable handle to the
`Element`'s View), and an optional set of initial, product-specific
`[Annotations](/glossary/README.md#element-annotation)`.

The component that implements the Graphical Presenter role knows how to open
the view on a connected display.

```rust
    fn present_view_for_element(
        graphical_presenter: &GraphicalPresenterProxy,
        element: &Element
    ) -> Result<ViewControllerProxy, Error> {
        let view_provider = element.connect_to_service::<ViewProviderMarker>()?;
        let token_pair = scenic::ViewTokenPair::new()?;
        let scenic::ViewRefPair {
            mut control_ref,
            mut view_ref
        } = scenic::ViewRefPair::new()?;
        let view_ref_dup = fuchsia_scenic::duplicate_view_ref(&view_ref)?;

        view_provider.create_view_with_view_ref(
            token_pair.view_token.value,
            &mut control_ref,
            &mut view_ref,
        )?;

        let annotations = element.get_annotations()?;

        let view_spec = ViewSpec {
            view_holder_token: Some(token_pair.view_holder_token),
            view_ref: Some(view_ref_dup),
            annotations: Some(annotations),
            ..ViewSpec::EMPTY
        };

        let (view_controller_proxy, server_end) = create_proxy::<ViewControllerMarker>()?;
        graphical_presenter.present_view(view_spec, Some(server_end))?;

        Ok(view_controller_proxy)
    }
```

## Handling input {#handling-input}

In the following example, if the `MouseHandler` detects a mouse-typed input
event, the handler sends the event to Scenic and returns an empty vector. On all
other types of input events, the `MouseHandler` returns a vector containing the
`[InputEvent](/glossary/README.md#InputEvent)` for the next
`[InputHandler](/glossary/README.md#InputHandler)` to process.

```rust
#[async_trait]
impl InputHandler for MouseHandler {
   async fn handle_input_event(
       &mut self,
       input_event: InputEvent,
   ) -> Vec<InputEvent> {
       match input_event {
           InputEvent {
               device_event: InputDeviceEvent::Mouse(mouse_event),
               device_descriptor: InputDeviceDescriptor::Mouse(mouse_descriptor),
           } => {
               // ... Handler specific details
               self.send_events_to_scenic(...)).await;
               vec![] // InputEvent is consumed because it was sent to Scenic
           }
           _ => vec![input_event], // InputEvent is returned for the next InputHandler
       }
   }
}
```
