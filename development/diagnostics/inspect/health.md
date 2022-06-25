# Health check

Health check is a standardized inspection metric.  Adding a `fuchsia.inspect.Health` child
to an Inspect Node gives that node the contained health information. This information can
be aggregated by system-wide health-checking tools.

## The layout of the health check node

The following properties and metrics are exported in any health check node:

| Name | Type | Description |
|------|------|-------------|
| `start_timestamp_nanos` | int64 | The monotonic clock system timestamp at which this health node was initialized (i.e. first became `STARTING UP`) |
| `message` | String | If `status==UNHEALTHY`, this includes an optional failure detail message. |
| `status` | Enum | `STARTING_UP`:<br>The health node was initialized but not yet marked running. |
|          |      | `OK`:<br>The subsystem reporting to this health node is reporting healthy. |
|          |      | `UNHEALTHY`:<br>The subsystem reporting to this health node is reporting unhealthy. |

## User guide

The following example illustrates the use of [`ffx inspect`][ffx-inspect] for getting information about
the component health status.

Examples:

```none {:.devsite-disable-click-to-copy}
$ ffx inspect show
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

```none {:.devsite-disable-click-to-copy}
$ ffx inspect show a.cmx:root/fuchsia.inspect.Health:status b.cmx:root/fuchsia.inspect.Healh:status c.cmx:root/fuchsia.inspect.Health:status
a:
  root:
    fuchsia.inspectHealth:
      status = OK
b:
  root:
    fuchsia.inspectHealth:
      status = OK
c:
  root:
    fuchsia.inspectHealth:
      status = UNHEALTHY
```

## Using health checks in components

The following sections explain how to use the library in Fuchsia components written in
various programming languages.

* {C++}

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

* {Rust}

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

<!-- Reference links -->

[ffx-inspect]: https://fuchsia.dev/reference/tools/sdk/ffx.md#inspect

