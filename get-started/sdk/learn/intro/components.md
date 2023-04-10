# Component-based software

<<../../../_common/intro/_components_intro.md>>

<<../../../_common/intro/_components_manager.md>>

<<../../../_common/intro/_components_capabilities.md>>

<<../../../_common/intro/_components_organization.md>>

## Exercise: Components

In this exercise, you'll explore the component instance tree and look in detail
at capability routing in action using some core system components.

<<../_common/_start_femu.md>>

### Explore system components

Open another terminal window and use the `component list` command to dump the
system's component tree:


```posix-terminal
ffx component list
```

You should see output similar to the (truncated) list below:

```none {:.devsite-disable-click-to-copy}
/
/bootstrap
/bootstrap/archivist
/bootstrap/base_resolver
/bootstrap/console
/bootstrap/console-launcher
/bootstrap/decompressor
/bootstrap/device_name_provider
/bootstrap/driver_manager
/bootstrap/fshost
/bootstrap/miscsvc
/bootstrap/netsvc
/bootstrap/power_manager
/bootstrap/ptysvc
/bootstrap/pwrbtn-monitor
/bootstrap/shutdown_shim
/bootstrap/svchost
/bootstrap/sysinfo
/bootstrap/virtual_console
/core
/core/activity
/core/appmgr
...
/core/debug_serial
/core/detect
/core/font_provider
/core/log-stats
/core/remote-control
/core/remote-diagnostics-bridge
/core/sampler
/core/system-update-committer
/core/temperature-logger
/core/test_manager
/core/full-resolver
/startup
```

This list represents the **component instance tree**, with organizational
components like `bootstrap`, `core`, and `startup` forming sub-trees
underneath the root.

The `component show` command provides more details about each component.

Use this command to see the details of `http-client` — a component that provides
an HTTP request service:

```posix-terminal
ffx component show http-client.cm
```

The command outputs the following report:

```none {:.devsite-disable-click-to-copy}
               Moniker: /core/network/http-client
                   URL: #meta/http-client.cm
                  Type: CML static component
       Component State: Resolved
 Incoming Capabilities: config
                        fuchsia.logger.LogSink
                        fuchsia.net.name.Lookup
                        fuchsia.posix.socket.Provider
                        pkg
  Exposed Capabilities: fuchsia.net.http.Loader
           Merkle root: d9e73f5b061f2f227e596e2e0079ff3a095fc69e192cf85e0d7621826c76356c
       Execution State: Running
          Start reason: '/core/feedback' requested capability 'fuchsia.net.http.Loader'
         Running since: ...
                Job ID: 41268
            Process ID: 41311
 Outgoing Capabilities: fuchsia.net.http.Loader
```

Notice a few of the details reported here:

1.  A unique identifier for the component instance (called a **moniker**).
1.  The package URL where this component was loaded from.
1.  The execution state of the component.
1.  The current job/process ID where the instance is running.
1.  A set of requested and exposed capabilities for the component.


### Trace a capability route

In the previous output, there are three capability groups listed:

* **Incoming Capabilities**: Capabilities that the component declares with
  `use`. These are provided to the component through its **namespace**.
* **Outgoing Capabilities**: Capabilities the component has published to its
  **outgoing directory**.
* **Exposed Capabilities**: Capabilities the component declares with
  `expose`. These are the component's **exposed services**.

One of the capabilities exposed by `http-client` to its parent **realm** is
[fuchsia.net.http.Loader](https://fuchsia.dev/reference/fidl/fuchsia.net.http#Loader).
This enables other components to issue HTTP requests.

Use the `component capability` command determine how many components interact with
this capability:

```posix-terminal
ffx component capability fuchsia.net.http.Loader
```

The command lists all the matching components:


```none {:.devsite-disable-click-to-copy}
Exposed:
  /core/network/http-client
  /core/network
Used:
  /core/cobalt
  /core/feedback
  /core
```

This indicates that the `cobalt` and `feedback` components use this capability
(i.e., have it listed under **Incoming Capabilities**). The common ancestor
between these components is `core`, which handles the routing of this capability
to the necessary children.


<aside class="key-point">
  <b>Extra credit</b>
  <p>A lot of components use the <code>fuchsia.logger.LogSink</code> capability,
  which is needed to read the system logs. You can list them using the same
  <code>component select</code> search for incoming capabilities.</p>
  <p>Can you find which component exposes this capability?</p>
</aside>
