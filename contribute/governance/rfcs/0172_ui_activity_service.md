<!-- Generated with `fx rfc` -->
<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0172" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
{# Fuchsia RFCs use templates to display various fields from _rfcs.yaml. View the #}
{# fully rendered RFCs at https://fuchsia.dev/fuchsia-src/contribute/governance/rfcs #}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Summary

This RFC introduces a new UI Activity Service which replaces and downscopes
the responsibilities of a pre-existing version.

The proposed service adds two new FIDL protocols:

- a private FIDL protocol `fuchsia.input.interaction.observation.Aggregator` to
  collect evidence of user input activity, and
- a partner FIDL protocol `fuchsia.input.interaction.Notifier` to notify
  clients of changes in user input activity state.

## Motivation

We would like to provide a service that notifies other parts of the system
whether recent user input activity has occurred. This service is useful to
inform other system services such as power-conserving protocols or screensaver
functionality.

There is a pre-existing ["Activity Service"][1] which establishes itself as a
source of truth regarding user idle state in the system. This proposal is a
departure from the current approach and introduces a new "UI Activity Service"
to indicate a narrower scope of responsibility–-over user input activity alone
rather than system activity altogether–-thereby enabling new features in some
products while reducing technical debt in others.

[1]: https://cs.opensource.google/fuchsia/fuchsia/+/main:src/sys/activity/

## Stakeholders

_Facilitator:_

leannogasawara@google.com

_Reviewers:_

- jsankey@google.com
- neelsa@google.com
- sanjayc@google.com
- quiche@google.com
- wittrock@google.com

_Consulted:_

- anwilson@google.com
- comfoltey@google.com
- fmil@google.com
- kpozin@google.com
- palmer@google.com
- yaar@google.com

_Socialization:_

This RFC was discussed during doc review with the Fuchsia Input team, and
requested reviews for [privacy][2] and [security][3].

[2]: https://fxbug.dev/99288
[3]: https://fxbug.dev/99287

## Design

### Activity State

**Activity** is user input interaction with the device that happened recently.

**Recently** is defined as some product-configured time threshold, e.g. 15
minutes.

When the device has not received activity recently, it is considered **idle**.

**User input interaction** is constrained to the use cases listed below for the
initial implementation.

### Requirements

The following user interactions MUST result in the service entering or renewing
an **active** state:

- A user touched a screen.
- A user interacted with a mouse or keyboard.
- A user pressed a media button, such as volume up or down.

The following user interactions SHOULD result in the service entering or
renewing an **active** state:

- A user enabled or disabled a screen reader. If the user does this using one of
  the input modes described above, the action would be captured implicitly.
  Alternatively, if this is done via a change to SetUI, it is recommended to be
  be treated as akin to opening the device lid.
- A user has opened the device lid.
- A user turned the device on.

The following interactions are not considered active user input activities due
to their corresponding rationales given inline. Therefore they MUST NOT result
in the service entering an **active** state:

- A user is watching a video or listening to an audio file:
  `fuchsia.media.ActivityReporter` should be used instead of the activity
  service discussed in this RFC.

### Protocol and service

We introduce a new internal FIDL protocol,
`fuchsia.input.interaction.observation.Aggregator` and a new FIDL protocol
`fuchsia.input.interaction.Notifier` to the partner SDK. These protocols will
be implemented and exposed by a new `Activity` class within the [Input
Pipeline][4] component.

[4]: /contribute/governance/rfcs/0096_user_input_arch.md#input-pipeline

#### `fuchsia.input.interaction.observation.Aggregator`

The clients of this API report that they believe they have evidence of user
input activity, and we treat their information as such. Therefore access is
restricted to in-tree components via capability routing. See
[Security considerations](#security-considerations) for more.

```
library fuchsia.input.interaction.observation;

using zx;

/// The Aggregator protocol collects evidence of user activity and uses this
/// evidence to set the system's activity state.
@discoverable
protocol Aggregator {
    /// Reports a discrete activity such as a keystroke.
    ReportDiscreteActivity(struct {
        activity DiscreteActivity;
        event_time zx.time;
    }) -> ();
};
```

Unlike its predecessor, which collected both discrete and ongoing events (i.e.
activities with a start and end time), the initial version of this protocol
defined by this RFC only collects "Discrete" [activities][5]. The ability to
collect ongoing events was initially introduced to support media playback,
which is explicitly _not_ a use case of this RFC.

[5]: https://cs.opensource.google/fuchsia/fuchsia/+/main:sdk/fidl/fuchsia.ui.activity/activity.fidl

#### `fuchsia.input.interaction.Notifier`

The clients of this API should want to subscribe to changes between user
interaction "active" and "idle" states as defined similarly to
[`fuchsia.ui.activity.State`][6]. This RFC plans to create a similar enum in
the `fuchsia.input.interaction` library that will be `flexible` in case of
future need for expansion. Access does not need to be restricted, see
[Privacy considerations](#privacy-considerations). Examples include:

- In-tree: Accessibility Manager silences the screen reader from verbalizing
  changes to a screen (e.g. every minute if a clock is displaying) when the
  device enters an idle state.
- Out-of-tree: A client returns to a home screen when the device enters an idle
  state.

```
library fuchsia.input.interaction;

/// The Notifier protocol offers a subscription interface through
/// which clients can watch for changes in the system's activity state.
@discoverable
protocol Notifier {
    /// Subscribe to changes in the system's state.
    /// The server will always respond immediately with the initial state,
    /// and after that whenever the system's state changes.
    WatchState(table {}) -> (resource struct {
        state State;
    });
};
```

The proposed protocol will differ from the existing
[`fuchsia.ui.activity.Notifier.WatchState`][7] protocol because it will use a
hanging get pattern, removing the need for a [Listener][8] protocol entirely. It
also does not send a timestamp.

[6]: https://cs.opensource.google/fuchsia/fuchsia/+/main:sdk/fidl/fuchsia.ui.activity/state.fidl
[7]: https://cs.opensource.google/fuchsia/fuchsia/+/main:sdk/fidl/fuchsia.ui.activity/provider.fidl;l=16
[8]: https://cs.opensource.google/fuchsia/fuchsia/+/main:sdk/fidl/fuchsia.ui.activity/provider.fidl;l=24

#### Lid Sensor

The UI Activity Service can use the `fuchsia.hardware.input` FIDL to watch for
lid sensor driver reports and transition to an active state upon receiving a
lid-opened report.

### Integrating with Input Pipeline

#### Space Considerations

Implementing the UI Activity Service (A) within the Input Pipeline (IP)
component instead of as its own component can save ~172 KB in disk space,
depending on the product and board configurations. For space-constrained
products, reducing size frees room for other improvements in the system.

| Build                          | A      | IP      | [IP + two FIDLs][9] |
| ------------------------------ | ------ | ------- | ------------------- |
| workstation_pro.chromebook-x64 | 196 KB | 1364 KB | 1388 KB             |
| core.astro                     | 184 KB | 688 KB  | 700 KB              |

[9]: https://fuchsia-review.googlesource.com/c/fuchsia/+/673448

#### Additional considerations

Integrating the UI Activity Service within Input Pipeline includes the
following:

- The service SHOULD be single-threaded because it is not necessary or
  beneficial to be multi-threaded at this time. Additionally, Input Pipeline is
  single-threaded, and converting the library to support a multi-threaded
  approach would introduce unnecessary complexity.
- The service SHOULD be implemented in Rust to avoid introducing the complexity
  required to manage multiple languages in one library.
- The service will not be available in recovery mode, since recovery mode does
  not use the Input Pipeline or Scene Manager. The absence of the activity
  service for recovery mode is not an issue because, by design, recovery mode
  uses a single highly-integrated component to minimize dependencies.
- The service SHOULD NOT process `InputReport`s by marking events handled or
  sending them to other components, as that makes the service aware of and
  responsible of more information than is necessary to determine whether
  activity has happened recently.
- The service SHOULD NOT have special knowledge of the sources of input events
  (as these SHOULD be reported via
  `fuchsia.input.interaction.observation.Aggregator`).
- The service therefore SHOULD NOT be implemented as an `InputHandler`.

### Configurable idle threshold

The idle threshold is how much time has passed since the last user activity for
the system to become idle. This can be set using [structured configuration][10]
in the Input Pipeline or Scene Manager component at the product level and will
be set to 15 minutes for the current products.

[10]: /contribute/governance/rfcs/0127_structured_configuration.md

## Implementation

The service will be implemented as follows:

1. Define the new `fuchsia.input.interaction.observation.Aggregator` and
   `fuchsia.input.interaction.Notifier` FIDL protocols.
2. Implement and unit test a new `Activity` service that is initialized within
   Input Pipeline to serve both protocols.
3. Relevant `InputHandlers` report activity via
   `fuchsia.input.interaction.observation.Aggregator`. Note: Reporting activity
   during this phase rather than during the binding phase is preferred because
   [`InputHandlers`][11] are responsible for dispatching information from
   `InputEvents` to other components or services.

   Affected handlers include:

- MediaButtonsHandler
- MouseInjectorHandler
- TouchInjectorHandler
- KeypressHandler (new)

[11]: /contribute/governance/rfcs/0096_user_input_arch.md#input-pipeline

4. Add integration tests for the following cases:

- `fuchsia.input.interaction.observation.Aggregator` informs Activity state
- `fuchsia.input.interaction.Notifier` is notified when system transitions to
  Active
- `fuchsia.input.interaction.Notifier` is notified when system transitions to
  Idle

=== `fuchsia.input.interaction.Notifier` is available for use by clients at
this point ===

5. Deprecate and remove `fuchsia.ui.activity.Tracker`,
   `fuchsia.ui.activity.Provider`, and `fuchsia.ui.activity.control.Control`

- Mark the FIDL protocols as deprecated with instructions to use the new
  protocols
- Migrate existing usages of `fuchsia.ui.activity.Provider` to
  `fuchsia.input.interaction.Notifier` [in [Cobalt][12], [Omaha][13], and
  [PowerManager][14]]
- Remove usage of `fuchsia.ui.activity.Tracker` (non-functional)
- Delete `//src/sys/activity`

[12]: https://cs.opensource.google/fuchsia/fuchsia/+/main:src/cobalt/bin/system-metrics/system_metrics_daemon.cc?q=fuchsia_ui_activity%20OR%20fuchsia.ui.activity%20OR%20fuchsia::ui::activity&ss=fuchsia%2Ffuchsia&start=71
[13]: https://cs.opensource.google/fuchsia/fuchsia/+/main:src/sys/pkg/bin/omaha-client/src/policy.rs?q=fuchsia_ui_activity%20OR%20fuchsia.ui.activity%20OR%20fuchsia::ui::activity&ss=fuchsia%2Ffuchsia&start=71
[14]: https://cs.opensource.google/fuchsia/fuchsia/+/main:src/power/power-manager/src/activity_handler.rs?q=fuchsia_ui_activity%20&ss=fuchsia%2Ffuchsia

## Performance

The UI Activity Service MUST NOT add latency to the processing of input events
in the system.

While the protocols proposed do not introduce asynchronous or computation-heavy
logic, we may want to introduce throttling to high-frequency input events. For
example, it's common for mice to send about 1000 events/second, so it may be
desirable to rate-limit the FIDL calls from `MouseInjectorHandler` to the
Activity Service.

We can monitor pre-existing performance tests, such as input latency tests, to
discern whether changes regress performance.

## Security considerations

The primary abuse vector for the UI Activity Service is to keep the system awake
by spuriously reporting activity using
`fuchsia.input.interaction.observation.Aggregator`. For instance, a malicious
application could periodically report activity to keep the system awake. This is
considered low risk given that
`fuchsia.input.interaction.observation.Aggregator` is only available in-tree,
not offered in the SDK, and will only be used by components in the UI realm.

Any service like this one poses the risk of delaying software updates, keeping
the screen unlocked, depleting the battery, and possibly denying service to
other applications on the device. Delegated data access is a risk not solved by
this RFC. Clients relying on idle state for features like the aforementioned
SHOULD implement their own maximum activity limit if relevant when using this
service, e.g. a forced system OTA once a user has been active for 24 hours.

Since this capability has to be routed through CFV2, it should be noted that the
platform only grants ability to report activity to Input Pipeline and
potentially A11y Manager at the platform level, and that products can further
route to trusted components.

## Privacy considerations

While the UI Activity Service does not expose any information about the nature
of user activity, it does share whether activity has occurred recently within a
preset time threshold via `fuchsia.input.interaction.Notifier`.

Whether someone is at or using their device at a specific time is privacy
impacting and sensitive, and therefore something we would not want untrusted
components to have access to. Since this capability has to be routed through
CFV2, it should be noted that the platform should only grant ability to observe
activity that are trusted at the platform level, and that products can further
route to trusted components.

Timestamps specifically will not be made available to subscribers.

## Testing

Our testing strategy includes

- Unit testing
- Integration testing using existing tools

## Documentation

The service will include a README overview, and new FIDL protocols will be
accompanied with comments inline.

## Drawbacks

### Input Pipeline Failures

If the Input Pipeline panics or deadlocks, the UI Activity Service would also
fail. In that case, subscribers to changes in idle state may want to default to
active-state behavior rather than idle-state behavior, and should have other
mitigating measures such as a maximum activity timeout to ensure critical
processes are still able to occur. If the Input Pipeline is not able to receive
or respond to Input Reports, there may be wider issues in the system.

## Alternatives

### Use Existing `fuchsia.ui.activity.Tracker` protocol

This approach prefers creating a new
`fuchsia.input.interaction.observation.Aggregator` protocol.

- The current protocol is in the partner SDK and cannot be restricted to the
  private SDK for use. Partner SDK inclusion will have additional CTS testing
  requirements.
- Aggregator more clearly indicates what the protocol is doing.
- The current set of use cases does not require use of `StartOngoingActivity` or
  `EndOngoingActivity`, and future needs may want to re-evaluate the current
  pattern.

### Modify Existing `fuchsia.ui.activity.Provider` protocol

This approach prefers creating a new `fuchsia.input.interaction.Notifier`
protocol.

- The current protocol can be controlled by the
  `fuchsia.ui.activity.control.Control` protocol.
- The `WatchState` protocol would have to be migrated to the proposed
  hanging-get pattern.
- The `WatchState` protocol unnecessarily releases a timestamp that we do not
  want clients to rely on without a concrete use case or intention for exposing
  it.

### Incorporate Other Activity Signals

Certain signals may be considered activity in some user flows but not others.
Accordingly, the correctness of how to interpret signals will vary depending on
the needs of a given form factor or service.

It is recommended that the `fuchsia.input.interaction.Notifier` protocol is
used to consume UI activity state, and can be referenced in combination with
other forms of activity (e.g., from audio, microphone, or camera) to determine
some final user or system state as is relevant on a case-by-case basis. For
example, a screensaver feature might consult just user input activity, whereas a
lockscreen feature might consult both user input activity and user
authentication state.

### Support Configuring a Subset of Activity Types to Consider

In the future, certain services may want to follow an activity state as
determined by a subset of user input modes, e.g. recent touch but not recent
keypress. This would introduce exponential combinations between the possible
input modes.

Given a current lack of interest in more elaborate features, we decided not to
prematurely introduce this complexity.

### Integrate with System Authentication for Fuchsia

Both System Authentication and UI Activity Service care about some notion of
user behavior, but with minimal practical overlap. For instance, System
Authentication cares about a more granular set of user presence states,
including user distance from the device and whether a user is an account owner.

While it might be possible to track user activity in a way which is amenable to
both use cases, it may not be necessary to proactively integrate these two
systems without a driving use case in mind.

## Future work

### Allow Subscribers to Configure an Idle Time Threshold

In the future, certain services may want to determine their own recency
thresholds for user input interactions. While this RFC does not propose a
solution, omitting this feature for simplicity of an initial implementation,
the protocols propose may be extended to add support when concrete use cases
arise.

## Prior art and references

- The [chrome.idle API][15] supports "active", "idle", and "locked" states.
- Fuchsia's [activity notifier][16] in Root Presenter was removed due to unuse.

[15]: https://developer.chrome.com/docs/extensions/reference/idle/
[16]: https://fuchsia-review.googlesource.com/c/fuchsia/+/539865
