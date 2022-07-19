{% import 'docs/_common/_doc_widgets.md' as widgets %}
# Software isolation model

<<../../../_common/intro/_sandboxing_intro.md>>

<<../../../_common/intro/_sandboxing_sandboxing.md>>

<<../../../_common/intro/_sandboxing_namespaces.md>>

## Exercise: Namespaces

In this exercise, you'll explore the contents of a component's namespace in
more detail using the shell.

<<../_common/_start_femu.md>>

### Find a component in the hub

Fuchsia provides the [Hub](/concepts/components/v2/hub.md) as a
diagnostic interface to obtain information about component instances running
on the system. You can explore the components and their namespaces using the
hub's directory structure.


<aside class="key-point">
The contents of the hub are organized according to the hierarchy of
{{ widgets.glossary_simple ('realm', 'component realms') }}in the system.
You'll explore more about what this structure means shortly.
</aside>


Connect to a device shell prompt and enter the following `ls` command to list
the components of the `core` realm under `/hub-v2/children/core/children`:

```posix-terminal
fssh ls /hub-v2/children/core/children
```

```none {:.devsite-disable-click-to-copy}
account
activity
agis
appmgr
battery_manager
bluetooth-core
brightness_manager
bt-a2dp
bt-avrcp
build-info
...
```

This is a list of many of the core Fuchsia system components. To see
more details about a specific component, list its directory contents.

Try this for the `http-client` component:

```posix-terminal
fssh ls /hub-v2/children/core/children/network/children/http-client
```

```none {:.devsite-disable-click-to-copy}
client
children
component_type
debug
exec
id
moniker
resolved
url
```

### Explore the namespace and outgoing directory

You'll find a running component's **namespace** under the `exec/in` path inside
the hub.

```posix-terminal
fssh ls /hub-v2/children/core/children/network/children/http-client/exec/in
```

```none {:.devsite-disable-click-to-copy}
config
pkg
svc
```

Here are some quick highlights of each element:

*   `config/`: configuration data for the component
*   `pkg/`: the contents of the component's package
*   `svc/`: system services available to the component

List the contents of the incoming `svc/` directory. This
directory contains
[service nodes](https://fuchsia.dev/reference/fidl/fuchsia.io#NodeInfo)
representing the system services provided to this component.

```posix-terminal
fssh ls /hub-v2/children/core/children/network/children/http-client/exec/in/svc
```

```none {:.devsite-disable-click-to-copy}
fuchsia.logger.LogSink
fuchsia.net.name.Lookup
fuchsia.posix.socket.Provider
```

Each of these services is accessible over a well-known protocol defined by a
[Fuchsia Interface Definition Language (FIDL)][glossary.FIDL] interface.
Components provide system services through their **outgoing directory**, which
is mapped to the `exec/out` path inside the hub.

List the contents of the outgoing `svc/` directory to see the system services
this component provides.

```posix-terminal
fssh ls /hub-v2/children/core/children/network/children/http-client/exec/out/svc
```

```none {:.devsite-disable-click-to-copy}
fuchsia.net.http.Loader
```

We'll explore FIDL protocols and how to access various services in more detail
later on.

<aside class="key-point">
  <b>Extra Credit</b>
  <p>Take a look at the other directory entries in the hub and see what else
  you can discover!</p>
</aside>
