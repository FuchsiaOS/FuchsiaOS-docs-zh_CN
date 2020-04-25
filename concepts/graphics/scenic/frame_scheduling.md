<!-- # Frame Scheduling -->

# 帧调度

<!-- ## API and Timing Model {#timing-model} -->

## API 与时序模型 {#timing-model}

* [session.fidl](/sdk/fidl/fuchsia.ui.scenic/session.fidl)
* [prediction_info.fidl](/sdk/fidl/fuchsia.scenic.scheduling/prediction_info.fidl)

<!-- ## Frame Scheduler Development -->

## 帧调度器的实现

<!-- [Life of a Pixel](life_of_a_pixel.md) shows how a client Present request is
integrated into a Scenic frame. -->

[像素的一生](life_of_a_pixel.md) 一文描述了客户端的 `Present` 请求被合成为
Scenic 中一帧的全过程。

<!-- ## Present2 Best Practices Examples -->

## `Present2` 最佳实践范例

<!-- These examples show how the API can be used for different use cases. The
examples assume that clients using the API have listeners registered for the
[`OnFramePresented`](/sdk/fidl/fuchsia.ui.scenic/session.fidl) and
[`OnScenicEvent`](/sdk/fidl/fuchsia.ui.scenic/session.fidl) event callbacks and
that `session` is an initialized Scenic
[Session](/sdk/fidl/fuchsia.ui.scenic/session.fidl)
channel. For examples of how to set up scenic, see
[Scenic examples](scenic.md#examples-of-using-scenic). -->

下列范例展示了 `Present2` API 的不同应用场景。假设 API 客户端已经注册了监听
[`OnFramePresented`](/sdk/fidl/fuchsia.ui.scenic/session.fidl) 和
[`OnScenicEvent`](/sdk/fidl/fuchsia.ui.scenic/session.fidl) 事件的回调函数，
`session` 是一个已初始化的 Scenic 
[Session](/sdk/fidl/fuchsia.ui.scenic/session.fidl) 通道。关于怎样配置
Scenic 的范例，请参见
[Scenic 范例](scenic.md#examples-of-using-scenic).

<!-- ### Example 1 {#example1} -->

### 范例 1 {#example1}

<!-- The simplest type of application creates and presents a new update every
time a previous one has been presented. This is reasonable for applications
with small workloads (takes less than a frame to create a frame) and
no requirements to minimize latency. -->

最简单的应用会在每一帧被展示时，创建并展示下一帧的更新。适用于工作量较小
的应用程序（创建一帧所用时间小于一帧），且不需要最小化延迟时。

<!-- 
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


``` -->

```cpp

void main() {
  PresentNewFrame();
}

void PresentNewFrame() {
  // 创建下一帧的更新，并将其插入 Session 队列.
  CreateAndEnqueueNewUpdate();

  // 刷新已入队的命令.
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
   // 将命令插入队列，以更新 Session
}


```

<!-- ### Example 2 {#example2} -->

### 范例 2 {#example2}

<!-- This example demonstrates how to write an application with small input-driven
updates and, where minimizing latency is important. It creates a new small
update upon receiving an input and immediately calls `Present2` attempting to
keep latency low as possible. This approach should only be used for very small
workloads since it creates some unnecessary work by not batching update creation. -->

该范例，展示了实现含有由输入驱动的小更新的应用程序，在这类程序中，最小化延迟十分重要。
该程序在接收输入事件时，会创建一个小更新，并立刻调用 `Present2` 尝试将延迟降到最低。
这一方法只适用于工作量非常小的情况，因为该方法无法批量更新，会导致许多不必要的工作。

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

<!-- ### Example 3 {#example3} -->

### 范例 3 {#example3}

<!-- This example demonstrates how to write an input-driven application
that batches inputs. -->

该范例展示了怎样实现一个由输入驱动，且能分批发送界面更新的应用程序。

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
