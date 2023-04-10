Retrieve a component's data exposed by Fuchsia's Inspect API. This data can be
any set of specialized information that a Fuchsia component is programmed to
collect while it is running on the device.

Note: For a component to collect and expose inspect data, the implementation of
inspect operations and data types must be placed in the component’s code.
Developers use this inspect feature to collect and expose information that will
be helpful for debugging the component or the system. For details, see
[Fuchsia component inspection overview][inspect-overview].

The tasks include:

- Scan the list of components on the device that expose inspect data (for
  example, the `bootstrap/archivist` component).
- Scan the list of selectors provided by the `bootstrap/archivist` component.
- Inspect a specific set of data from the `bootstrap/archivist` component.

In VS Code, do the following:

1. Click the **TERMINAL** tab on the VS Code panel.

1. In the terminal, view the list of components on the device that expose
   inspect data:

   ```posix-terminal
   tools/ffx inspect list
   ```

   This command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   $ tools/ffx inspect list
   <component_manager>
   bootstrap/archivist
   bootstrap/driver_manager
   bootstrap/fshost
   bootstrap/fshost/blobfs
   bootstrap/fshost/fxfs
   ...
   core/ui/scenic
   core/vulkan_loader
   core/wlancfg
   core/wlandevicemonitor
   core/wlanstack
   ```

   Notice that the `bootstrap/archivist` component is on the list.

1. View all available selectors for the `bootstrap/archivist` component:

   ```posix-terminal
   tools/ffx inspect selectors bootstrap/archivist
   ```

   This command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   $ tools/ffx inspect selectors bootstrap/archivist
   bootstrap/archivist:root/archive_accessor_stats/all/inspect/batch_iterator/get_next:errors
   bootstrap/archivist:root/archive_accessor_stats/all/inspect/batch_iterator/get_next:requests
   bootstrap/archivist:root/archive_accessor_stats/all/inspect/batch_iterator/get_next:responses
   ...
   ```

   Each of these selectors represents a different type of data you can inspect.

1. Inspect the `bootstrap/archivist` component for the recent events data:

   ```posix-terminal
   tools/ffx inspect show bootstrap/archivist:root/events/recent_events
   ```

   This command prints output similar to the following:

   ```none {:.devsite-disable-click-to-copy}
   $ tools/ffx inspect show bootstrap/archivist:root/events/recent_events
   bootstrap/archivist:
     metadata:
       filename = fuchsia.inspect.Tree
       component_url = fuchsia-boot:///#meta/archivist.cm
       timestamp = 705335717538
     payload:
       root:
         events:
           recent_events:
             361:
               @time = 6272744049
               event = component_stopped
               moniker = core/trace_manager/cpuperf_provider
             362:
               @time = 6283370267
               event = log_sink_requested
               moniker = core/session-manager
             ...
             556:
               @time = 415796882099
               event = log_sink_requested
               moniker = core/debug_agent
             557:
               @time = 453898419738
               event = component_started
               moniker = core/ffx-laboratory:hello_world
             558:
               @time = 453899964568
               event = log_sink_requested
               moniker = core/ffx-laboratory:hello_world
             559:
               @time = 453900332656
               event = log_sink_requested
               moniker = core/ffx-laboratory:hello_world
             560:
               @time = 495458923475
               event = component_stopped
               moniker = core/ffx-laboratory:hello_world
   ```

   This data records all the events triggered by components on the device so
   far.

<!-- Reference links -->

[inspect-overview]: /docs/development/diagnostics/inspect/README.md
