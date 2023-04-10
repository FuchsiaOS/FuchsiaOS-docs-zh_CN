{% import 'docs/_common/_doc_widgets.md' as widgets %}
# Software isolation model

<<../../_common/intro/_sandboxing_intro.md>>

<<../../_common/intro/_sandboxing_sandboxing.md>>

<<../../_common/intro/_sandboxing_namespaces.md>>

## Exercise: Namespaces

Most processes in Fuchsia represent executable programs associated with a
[component](/docs/glossary/README.md#component), where the component declaration
is responsible for constructing the namespace that process can see.

In this exercise, you'll explore the contents of a component's namespace.

<<../_common/_start_femu.md>>

### Find a target component

You learned in the previous section that processes associated with a component
are named with a `.cm` extension. Recall the following example process list:

```none {:.devsite-disable-click-to-copy}
TASK                     PSS PRIVATE  SHARED   STATE NAME
j: 1027               507.8M  507.4M                 root
  p: 1061             564.4k    564k     36k         bin/bootsvc
  p: 1150            4264.4k   4264k     36k         bin/component_manager
  j: 1479             228.4k    228k
    p: 1583           228.4k    228k     36k         pwrbtn-monitor.cm
  j: 1484             532.4k    532k
    p: 1599           532.4k    532k     36k         svchost.cm
  j: 1544             402.4k    304k
    p: 1633           402.4k    304k    232k         netsvc.cm
  j: 1681             296.4k    296k
    p: 1733           296.4k    296k     36k         console-launcher.cm
  j: 1799            7232.4k   7232k
    p: 1825          7232.4k   7232k     36k         archivist.cm
  ...
{{ '<strong>' }}  j: 31294           1872.2K   1872K {{ '</strong>' }}
{{ '<strong>' }}    p: 31331         1872.2K   1872K     20K         http-client.cm {{ '</strong>' }}
```

For this exercise, you'll use `http-client.cm` as your target to explore.

### Connect to the target component

In order to explore a component's namespace, you need to determine the unique
identifier for that component within the system. This is known as the component
[moniker](/docs/glossary/README.md#moniker).

<aside class="key-point">
The moniker relates to the hierarchy of components within the system.
You'll explore more about what this structure means shortly.
</aside>

Use the `ffx component show` command to list additional details about the
component, including the component moniker:

```posix-terminal
ffx component show http-client.cm
```

The command prints output similar to the following:

```none {:.devsite-disable-click-to-copy}
$ ffx component show http-client.cm
{{ '<strong>' }}               Moniker: /core/network/http-client {{ '</strong>' }}
                   URL: #meta/http-client.cm
                  Type: CML static component
                  ...
```

You can use the `ffx component explore` command to open an interactive shell
inside the target component's environment. Try this for the `http-client`
component:

```posix-terminal
ffx component explore /core/network/http-client
```

**Inside the explore shell**, list the contents of the root directory using the
`ls` command:

<pre class="devsite-click-to-copy">
<span class="no-select">[explore shell] $ </span>ls
</pre>

```none {:.devsite-disable-click-to-copy}
bin
exposed
ns
out
runtime
svc
```

### Explore the namespace

You'll find the component's **namespace** under the `/ns` path inside the
environment.

**Inside the explore shell**, list the contents of the namespace:

<pre class="devsite-click-to-copy">
<span class="no-select">[explore shell] $ </span>ls /ns
</pre>

```none {:.devsite-disable-click-to-copy}
config
pkg
svc
```

Here are some quick highlights of each element:

*   `config/`: configuration data for the component
*   `pkg/`: the contents of the component's package
*   `svc/`: system services available to the component

**Inside the explore shell**, list the contents of the incoming `/ns/svc`
directory. This directory contains
[service nodes](https://fuchsia.dev/reference/fidl/fuchsia.io#NodeInfo)
representing the system services provided to this component.

<pre class="devsite-click-to-copy">
<span class="no-select">[explore shell] $ </span>ls /ns/svc
</pre>

```none {:.devsite-disable-click-to-copy}
fuchsia.logger.LogSink
fuchsia.net.name.Lookup
fuchsia.posix.socket.Provider
```

Each of these services is accessible over a well-known protocol defined by a
[Fuchsia Interface Definition Language (FIDL)][glossary.FIDL] interface.
We'll explore FIDL protocols and how to access various services in more detail
later on.

**Inside the explore shell**, type `exit` to return to the shell on your
development machine:


<pre class="devsite-click-to-copy">
<span class="no-select">[explore shell] $ </span>exit
</pre>

<aside class="key-point">
  <b>Extra Credit</b>
  <p>Read through the
  <a href="/docs/development/sdk/ffx/explore-components.md">Explore components</a>
  guide to learn about other directory entries in this component's environment.
  Then connect to a different target component. How are the contents different
  between two components?</p>
</aside>
