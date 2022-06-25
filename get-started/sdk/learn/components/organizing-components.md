# Organizing components

<<../../../_common/components/_organizing_intro.md>>

<<../../../_common/components/_organizing_types.md>>

<<../../../_common/components/_organizing_identifying.md>>

<<../../../_common/components/_organizing_lifecycle.md>>

## Exercise: Integrate components

In order for a component to be invoked, it must be present in the active
component topology. For this exercise, you will add your component to the
`ffx-laboratory` — a restricted collection used for development inside the
product's **core realm**. Collections enable components to be dynamically
created and destroyed at runtime.

<<../_common/_start_femu_with_packages.md>>

### Add to the component topology

Create a new instance of the `echo` component using the following command:

```posix-terminal
ffx component create /core/ffx-laboratory:echo \
    fuchsia-pkg://fuchsiasamples.com/echo-example#meta/echo.cm
```

This command accepts two parameters:

* `/core/ffx-laboratory:echo`: This is the **component moniker**,
  representing the path inside the component topology for the component instance.
* `fuchsia-pkg://fuchsiasamples.com/echo-example#meta/echo.cm`: This is the
 **component URL**, indicating how Fuchsia should resolve the component from the
 package server.

A new component instance named `echo` now exists in the topology. Show the
details of the new instance using the following command:

```posix-terminal
ffx component show echo
```

You should see the following output:

```none {:.devsite-disable-click-to-copy}
               Moniker: /core/ffx-laboratory:echo
                   URL: fuchsia-pkg://fuchsiasamples.com/echo-example#meta/echo.cm
                  Type: CML dynamic component
       Component State: Unresolved
       Execution State: Stopped
```

Notice that the instance has been created, but the component URL has not been
resolved. Resolution happens when the framework attempts to start the instance.

### Start the component instance

Start the new `echo` component instance using the following command:

```posix-terminal
ffx component start /core/ffx-laboratory:echo
```

This command accepts one parameter:

* `/core/ffx-laboratory:echo`: This is the **component moniker**,
  representing the path inside the component topology for the component instance.

This causes the component instance to start, print a greeting to the log,
then exit. Open a new terminal window and filter the device logs for messages
from the example:

```posix-terminal
ffx log --filter echo
```

You should see the following output in the device logs:

```none {:.devsite-disable-click-to-copy}
[ffx-laboratory:echo][I] Hello, Alice, Bob, Spot!
```

### Explore the instance

Show the details of the `echo` instance again using the following command:

```posix-terminal
ffx component show echo
```

You should now see the following output:

```none {:.devsite-disable-click-to-copy}
               Moniker: /core/ffx-laboratory:echo
                   URL: fuchsia-pkg://fuchsiasamples.com/echo-example#meta/echo.cm
                  Type: CML dynamic component
       Component State: Resolved
 Incoming Capabilities: fuchsia.logger.LogSink
                        pkg
       Execution State: Stopped
```

The component state has changed to `Resolved` and you can see more details
about the component's capabilities.

Components have no ambient capabilities to access other parts of the system.
Every capability a component requires must be explicitly routed to it through
the component topology or provided by its environment.

The `echo` component requires the `fuchsia.logger.LogSink` capability to
write to the system log. You were able to successfully view the log output
because this capability is **offered** to components in the `ffx-laboratory`
collection from the `core` realm:

```json5 {:.devsite-disable-click-to-copy}
{
    collections: [
        {
            name: "ffx-laboratory",
        },
    ],
    offer: [
        {
            protocol: [ "fuchsia.logger.LogSink" ],
            from: "parent",
            to: "#ffx-laboratory",
        },
    ],
}
```

<aside class="key-point">
  <b>Reminder</b>
  <p>The required logging capabilities in the project are implicitly declared by
  the syslog manifest shard: <code>syslog/client.shard.cml</code>.
</aside>

### Destroy the instance

Clean up the `echo` instance using the following command:

```posix-terminal
ffx component destroy /core/ffx-laboratory:echo
```
