# Frame Scheduling

## API and Timing Model {#timing-model}

* [session.fidl](/sdk/fidl/fuchsia.ui.scenic/session.fidl)
* [prediction_info.fidl](/sdk/fidl/fuchsia.scenic.scheduling/prediction_info.fidl)

## Frame Scheduler Development

[Life of a Pixel](life_of_a_pixel.md) shows how a client Present request is
integrated into a Scenic frame.

## Scenic contract with clients

If you want to use [`Present2`](/sdk/fidl/fuchsia.ui.scenic/session.fidl) in a
way where frame scheduling is both correct and performant, consider the
guarantees that Scenic provides surrounding:

* [Vsync accuracy](#vsync-accuracy)

### Vsync accuracy {#vsync-accuracy}

Scenic makes no guarantees as to the accuracy or precision of the future Vsync
information it provides to clients. Scenic merely forwards whatever information
it receives from the hardware which may be incorrect.

There are two identified Vsync issues, [minor drift](#minor-vsync-drift)
and [major disruptions](#major-vsync-disruptions), with some implications
for what [`Present2`](/sdk/fidl/fuchsia.ui.scenic/session.fidl) clients should
do.

#### Minor Vsync drift {#minor-vsync-drift}

One problem is when VSync occurs (on the scale of microseconds) before the time
that Scenic reports.

Assume a client receives a future Vsync time of 1000us, in a
[`FuturePresentationTimes`](/sdk/fidl/fuchsia.scenic.scheduling/prediction_info.fidl)
object. The client then calls
[`Present2`](/sdk/fidl/fuchsia.ui.scenic/session.fidl) with a requested
presentation time of 1000us, expecting that its content will be displayed at
1000us. However, it is possible that the true Vsync might actually occur at
995us, and the client will effectively drop a frame as Scenic will only
attempt to display the client's content at the following Vsync.

It is therefore recommended that clients request presentation times half a Vsync
interval offset from their true target time, to account for this drift in a
Vsync interval independent way.

#### Major Vsync disruptions {#major-vsync-disruptions}

Another problem is a Vsync occuring milliseconds after it was supposed to,
with the following Vsync resuming the previously established pattern.

Assume the display has a 60Hz refresh rate. You can then expect Vsyncs to
occur at times 0ms, 16.7ms, 33.3ms, 50ms and so on. On some hardware, a Vsync
can be delayed. This might lead to Vsyncs occuring at times 0ms, **22ms**,
33.3ms, 50ms and so on.

This issue happens sporadically and without warning, so there is no way to
effectively prevent it. In order to mitigate its effects, clients are
expected to take extra care to ensure that their
[`requested_presentation_time`](/sdk/fidl/fuchsia.ui.scenic/session.fidl)s
remain monotonically increasing. If not, the client's session will be shut down.

To have your `requested_presentation_time` remain monotonically increasing, add
the following logic to your scheduling code:

```cpp
// Do some operations.
requested_presentation_time = CalculateNextPresentationTime();
// Lower bound the requested time.
requested_presentation_time = std::max(requested_presentation_time, previous_requested_presentation_time);

previous_requested_presentation_time = requested_presentation_time;
session->Present2(requested_presentation_time, ...);
```

This code is correct because the new requested time will always be greater or
equal to the old requested time.

This code is performant because it will never cause you to miss a frame
unnecessarily, regardless of whatever `CalculateNextPresentationTime()` does.

## Present2 Best Practices Examples

These examples show how the API can be used for different use cases. The
examples assume that clients using the API have listeners registered for the
[`OnFramePresented`](/sdk/fidl/fuchsia.ui.scenic/session.fidl) and
[`OnScenicEvent`](/sdk/fidl/fuchsia.ui.scenic/session.fidl) event callbacks and
that `session` is an initialized Scenic
[Session](/sdk/fidl/fuchsia.ui.scenic/session.fidl)
channel. For examples of how to set up scenic, see
[Scenic examples](scenic.md#examples-of-using-scenic).

### Example 1 {#example1}

The simplest type of application creates and presents a new update every
time a previous one has been presented. This is reasonable for applications
with small workloads (takes less than a frame to create a frame) and
no requirements to minimize latency.

```cpp

void main() {
  PresentNewFrame();
}

void PresentNewFrame() {
  // Create a new update and enquee it.
  CreateAndEnqueueNewUpdate();

  // Flush enqueued events.
  Present2Args args;
  args.set_requested_presentation_time(0);
  args.set_acquire_fences({});
  args.set_release_fences({});
  args.set_requested_prediction_span(0);
  session->Present2(std::move(args), /*callback=*/[]{});
}

void OnFramePresented(...) {
  PresentNewFrame();
}

void CreateAndEnqueueNewUpdate() {
   // Enqueue commands to update the session
}


```

### Example 2 {#example2}

This example demonstrates how to write an application with small input-driven
updates and, where minimizing latency is important. It creates a new small
update upon receiving an input and immediately calls `Present2` attempting to
keep latency low as possible. This approach should only be used for very small
workloads since it creates some unnecessary work by not batching update creation.

```cpp

int64 num_calls_left_ = 0;
bool update_pending_ = false;

main() {
  session->RequestPresentationTimes(/*requested_prediction_span=*/0,
                     /*callback=*/[this](FuturePresentationTimes future_times){
    UpdateNumCallsLeft(future_times.remaining_presents_in_flight_allowed);
  };}
}

void UpdateNumCallsLeft(int64_t num_calls_left){
  num_calls_left_ = num_calls_left;
}

void OnScenicEvent(Event event) {
  if (IsInputEvent(event)) {
    CreateAndEnqueueUpdate(std::move(event));
    if (num_calls_left_ > 0) {
      PresentFrame();
    } else {
      update_pending_ = true;
    }
  }
}

void PresentFrame() {
  --num_calls_left_;
  update_pending_ = false;

  Present2Args args;
  args.set_requested_presentation_time(0);
  args.set_acquire_fences({});
  args.set_release_fences({});
  args.set_requested_prediction_span(0);
  session->Present2(std::move(args),
                    /*callback=*/[this](FuturePresentationTimes future_times){
    UpdateNumCallsLeft(future_times.remaining_presents_in_flight_allowed);
  };);
}

void OnFramePresented(FramePresentedInfo info) {
	UpdateNumCallsLeft(info.num_presents_allowed);
  if (frame_pending_ && num_calls_left_ > 0) {
     PresentFrame();
  }
}


```

### Example 3 {#example3}

This example demonstrates how to write an input-driven application
that batches inputs.

```cpp

struct TargetTimes {
  zx_time_t latch_point;
  zx_time_t presentation_time;
};

int64 num_calls_left_ = 0;
bool update_pending_ = false;
bool frame_pending_ = false;
zx_time_t last_targeted_time_ = 0;
std::vector<Event> unhandled_input_events_;
async_dispatcher dispatcher_;

void UpdateNumCallsLeft(int64_t num_calls_left){
  num_calls_left_ = num_calls_left;
}

zx_duration_t UpdateCreationTime() {
  // Return a prediction for how long an update could take to create.
}

TargetTimes FindNextPresentationTime(
                      std::vector<PresentationInfo> future_presentations) {
  // Select the next future time to target.
  zx_time_t now = time.Now();
  for(auto times : future_presentations) {
    if (times.latch_point > now + UpdateCreationTime()
        && times.presentation_time > last_targeted_time_) {
      return {times.latch_point, times.presentation_time};
    }
  }

  // This should never be reached.
  return {now, now};
}

void CreateAndEnqueueNewUpdate(std::vector<Event> input_events) {
   // Enqueue commands to update the session.
}

void OnScenicEvent(Event event) {
  if (IsInputEvent(event)) {
    unhandled_input_events_.push_back(std::move(event));
    RequestNewFrame();
  }
}

void RequestNewFrame() {
  if (update_pending_) {
    return;
  } else {
    update_pending_ = true;
    ScheduleNextFrame();
  }
}

void PresentFrame() {
  present_pending_ = false;

  session->RequestPresentationTimes(/*requested_prediction_span=*/0,
                    /*callback=*/[this](FuturePresentationTimes future_times){
    if (future_times.remaining_presents_in_flight_allowed > 0) {
      // No present calls left. Need to wait to be returned some by previous
      // frames being completed in OnFramePresented(). This could happen when
      // Scenic gets overwhelmed or stalled for some reason.
      present_pending_ = true;
      return;
    }

    TargetTimes target_times =
          FindNextPresentationTime(future_times.future_presentations);
    last_targeted_time_ = target_time.presentation_time;


    // Wait until slightly before the deadline to start creating the update.
    zx_time_t wakeup_time = target_times.latch_point - UpdateCreationTime();
    async::PostTaskForTime(
      dispatcher_,
      [this, presentation_time] {
        update_pending_ = false;
        present_pending_ = false;

        CreateAndEnqueueUpdate(std::move(unhandled_input_events_));

        Present2Args args;
        // We subtract a bit from our requested time (1 ms in this example)
        // for two reasons:
        // 1. Future presentation times aren't guaranteed to be entirely
        // accurate due to hardware vsync drift and other factors.
        // 2. A presetnt call is guaranteed to be presented "at or later than"
        // the requested presentation time.
        // This guards against against `Present2` calls getting accidentally
        // delayed for an entire frame.
        args.set_requested_presentation_time(
              target_times.presentation_time - 1'000'000);
        args.set_acquire_fences({});
        args.set_release_fences({});
        args.set_requested_prediction_span(0);
        session->Present2(std::move(args), /*callback=*/[]{};);
      },
      wakeup_time);
  };}
}

void OnFramePresented(FramePresentedInfo info) {
  if (frame_pending_ && info.num_presents_allowed > 0) {
     PresentFrame();
  }
}


```
