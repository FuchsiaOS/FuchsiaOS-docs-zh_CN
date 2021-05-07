# Input client library {#input-client-library}

## Overview {#overview}

The input library tracks available input devices, manages device state, and
allows the session to register handlers for events.

The session creates an input pipeline that is configured for the specific
product. First, the session decides which types of input devices the input
pipeline should support. Then, the session registers input handlers for input
events.

The input library provides implementations for common input handlers, such as
Scenic and input method editors (IME). After the session instantiates components
that consume input, the input pipeline directly sends input events to those
components through the registered input handlers.

## Input pipeline {#input-pipeline}

An input pipeline manages [`InputDeviceBinding`s] and [`InputHandler`s].

* An `InputDeviceBinding` represents a connection to a physical input device
  (e.g. mouse, keyboard).
* An `InputHandler` represents a client of [`InputEvent`s].

![Input Pipeline](images/input-pipeline.png)

An input pipeline routes input from physical devices to various clients by doing
the following:

1. Detects and binds to new input devices as they appear in
   `/dev/class/input-report`.
2. Propagates `InputEvent`s through `InputHandler`s.

Session authors are responsible for setting up input pipelines. More details on
how can be found in [input_pipeline.rs].

### InputDeviceBinding {#input-device-binding}

An `InputDeviceBinding` does the following:

1. Connects to an [`InputReport`] file located at `/dev/class/input-report/XXX`.
2. Generates `InputEvent`s from the [`DeviceDescriptor`] and incoming
   `InputReport`s.

The input pipeline creates and owns `InputDeviceBinding`s as new input
peripherals are connected to a device.

### InputHandler {#input-handler}

When an `InputHandler` receives an `InputEvent`, it does at least one of the
following:

* Forwards the `InputEvent` to the relevant client component.
* Outputs a vector of `InputEvent`s for the next `InputHandler` to process.

`InputHandler`s must satisfy at least one of these conditions, but otherwise
their implementation details can vary.

The developer guide includes an
[example implementation of an `InputHandler`][example-input-handler].

[`DeviceDescriptor`]: /docs/glossary.md#input-pipeline-device-descriptor
[`InputDeviceBinding`s]: /docs/glossary.md#input-pipeline-input-device-binding
[`InputEvent`s]: /docs/glossary.md#input-pipeline-input-event
[`InputHandler`s]: /docs/glossary.md#input-pipeline-input-handler
[`InputReport`]: /docs/glossary.md#input-report
[input_pipeline.rs]: /src/session/lib/input/src/input_pipeline.rs
[example-input-handler]: /docs/development/sessions/roles-and-responsibilities.md#handling-input
