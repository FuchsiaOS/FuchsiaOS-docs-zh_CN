<!-- 
# Health check
 -->
# 健康检查

<!-- 
Health check is a standardized inspection metric.  Adding a `fuchsia.inspect.Health` child
to an Inspect Node gives that node the contained health information. This information can
be aggregated by system-wide health-checking tools.
 -->
健康检查（health check）是一种标准化的检查指标。向审视节点添加一个 `fuchsia.inspect.Health` 子节点，会为该子节点提供已被包含的健康信息。该信息能够通过系统范围的健康检查工具进行汇总。

<!-- 
## The layout of the health check node
 -->
## 健康检查节点布局

<!-- 
The following properties and metrics are exported in any health check node:
 -->
下面的属性和指标被导出至任何健康检查节点中：

<!-- 
| Name | Type | Description |
|------|------|-------------|
| `start_timestamp_nanos` | int64 | The monotonic clock system timestamp at which this health node was initialized (i.e. first became `STARTING UP`) |
| `message` | String | If `status==UNHEALTHY`, this includes an optional failure detail message. |
| `status` | Enum | `STARTING_UP`:<br>The health node was initialized but not yet marked running. |
|          |      | `OK`:<br>The subsystem reporting to this health node is reporting healthy. |
|          |      | `UNHEALTHY`:<br>The subsystem reporting to this health node is reporting unhealthy. |
 -->
| 名称 | 类型 | 描述 |
|------|------|-------------|
| `start_timestamp_nanos` | int64 | 该健康节点初始化时的单调时钟系统时间戳（即：首先变为 `STARTING UP`）|
| `message` | String | 如果 `status==UNHEALTHY`，那么它包含可选的故障细节消息。|
| `status` | Enum | `STARTING_UP`：<br>该健康节点已初始化，但尚未标记为运行。|
|          |      | `OK`：<br>向该健康节点报告的子系统报告状况良好。 |
|          |      | `UNHEALTHY`:<br>向该健康节点报告的子系统报告状况不佳。 |

<!-- 
## User guide
 -->
## 用户指南

<!-- 
The following example illustrates the use of [iquery] for getting information about
the component health status.

Examples:
 -->
下面的示例说明了使用 [iquery]获取组件（component）健康状况信息的方法。

示例：

```none
$ iquery show `iquery list`
a.cmx:
  root:
    fuchsia.inspect.Health:
      start_timestamp_nanos = ...
      status = OK
    connections:
      0:
        fuchsia.inspect.Health:
          start_timestamp_nanos = ...
          status = STARTING_UP
    optional_database:
      fuchsia.inspect.Health:
        start_timestamp_nanos = ...
        status = UNHEALTHY
        message = "Cannot open local.file"
b.cmx:
  root:
    fuchsia.inspect.Health:
      start_timestamp_nanos = ...
      status = OK
c.cmx:
  root:
    fuchsia.inspect.Health:
      start_timestamp_nanos = ...
      status = UNHEALTHY
      message = "Failed to connect to fuchsia.example.RequiredService"
```

```none
$ iquery show 'a.cmx:root/fuchsia.inspect.Health:status' 'b.cmx:root/fuchsia.inspect.Healh:status' 'c.cmx:root/fuchsia.inspect.Health:status'
a:
  root:
    fuchsia.inspectHealth:
      status = Ok
b:
  root:
    fuchsia.inspectHealth:
      status = Ok
c:
  root:
    fuchsia.inspectHealth:
      status = Ok
```

<!-- 
## Using health checks in components
 -->
## 在组件中使用健康检查

<!-- 
The following sections explain how to use the library in Fuchsia components written in
various programming languages.
 -->
下面的部分用不同编程语言解释了如何使用 Fuchsia 组件中的库。

* {C++}

<!--
  ```cpp
    #include <lib/async-loop/cpp/loop.h>
    #include <lib/async-loop/default.h>
    #include <lib/sys/cpp/component_context.h>
    #include <lib/sys/inspect/cpp/component.h>

    int main(int argc, char** argv) {
      async::Loop loop(&kAsyncLoopConfigAttachToCurrentThread);
      auto context = sys::ComponentContext::CreateAndServeOutgoingDirectory();
      sys::ComponentInspector inspector(context.get());
      inspector.Health().StartingUp();

      // ...Do startup work...

      inspector.Health().Ok();
      inspector.Health().Unhealthy("I'm not feeling well.");
      inspector.Health().Ok();

      loop.Run();
      return 0;
    }
  ```
 -->
  ```cpp
    #include <lib/async-loop/cpp/loop.h>
    #include <lib/async-loop/default.h>
    #include <lib/sys/cpp/component_context.h>
    #include <lib/sys/inspect/cpp/component.h>

    int main(int argc, char** argv) {
      async::Loop loop(&kAsyncLoopConfigAttachToCurrentThread);
      auto context = sys::ComponentContext::CreateAndServeOutgoingDirectory();
      sys::ComponentInspector inspector(context.get());
      inspector.Health().StartingUp();

      // ...进行启动工作...

      inspector.Health().Ok();
      inspector.Health().Unhealthy("I'm not feeling well.");
      inspector.Health().Ok();

      loop.Run();
      return 0;
    }
  ```

* {Rust}

<!-- 
  ```rust
    use fuchsia_inspect as inspect;
    use fuchsia_inspect::health;

    fn main() {
      // If you have your own inspector, it's also possible to export its health.

      /* inspector needs to be initialized */
      let inspector = /* ... */
      let mut node = inspector::root();
      let mut health = fuchsia_inspect::health::Node(node);
      // ...
      health.set_ok();
      health.set_unhealthy("I'm not feeling well.");
      health.set_ok();  // The component is healthy again.
    }
  ```
 -->
  ```rust
    use fuchsia_inspect as inspect;
    use fuchsia_inspect::health;

    fn main() {
      // 如果您有您自己的检查器，那么也可以导出它的健康状态。

      /* 检查器需要初始化 */
      let inspector = /* ... */
      let mut node = inspector::root();
      let mut health = fuchsia_inspect::health::Node(node);
      // ...
      health.set_ok();
      health.set_unhealthy("I'm not feeling well.");
      health.set_ok();  // 组件恢复健康。
    }
  ```


* {Dart}

  ```dart
    import 'package:fuchsia_inspect/inspect.dart' as inspect;

    void main(List<String> args) {
      final inspector = inspect.Inspect();
      inspector.health.setStartingUp();
      // ...Do startup work...
      inspector.health.setOk();
      inspector.health.setUnhealthy("I'm not feeling well.");
      inspector.health.setOk();
    }
  ```


[iquery]: /docs/reference/diagnostics/consumers/iquery.md
