# Fuchsia source tree

In this section, you will learn about the organization of the Fuchsia source
code and the tools used to manage the open source project.

## Source code management

Fuchsia uses the [jiri](https://fuchsia.googlesource.com/jiri) tool to manage
git repositories in the Fuchsia project. It synchronizes a local checkout of the
source code with the
[Global Integration manifest](https://fuchsia.googlesource.com/integration) and
provides the necessary facilities to contribute changes back to Fuchsia. Global
Integration is the central ledger that defines the current state of the various
projects in the Fuchsia tree.

<aside class="key-point">
The bootstrap script performs the step described in this section for you when
you <a href="/docs/get-started/get_fuchsia_source.md">download the source</a>.
</aside>

You initialize a local jiri checkout using the `import` command with an XML
manifest that declares all the repositories and how they are organized. The
import for the default Global Integration manifest is as follows:


```posix-terminal
jiri import -name=integration flower https://fuchsia.googlesource.com/integration
```

This command adds the manifest to a local `.jiri_manifest` file at the root of
your local checkout.


```xml {:.devsite-disable-click-to-copy}
<manifest>
  <imports>
    <import manifest="flower" name="integration"
            remote="https://fuchsia.googlesource.com/integration" />
  </imports>
</manifest>
```

<aside class="key-point">
  <b>Fuchsia is a flower</b>
  <p>Notice that the default integration manifest is named
  <a href="https://fuchsia.googlesource.com/integration/+/refs/heads/main/flower">flower</a>.
  This metaphor is often applied to the Fuchsia source code, where the core
  Fuchsia platform is considered the
  <a href="https://fuchsia.googlesource.com/integration/+/refs/heads/main/stem">stem</a>
  with additional external dependencies and related projects are the
  <strong>petals</strong>.</p>
  <p>The flower manifest is a single aggregation point for the stem and various
  petal projects.</p>
</aside>

Once a local checkout is initialized on your development machine, jiri can pull
the latest changes from Global Integration at any time with one command:

```posix-terminal
jiri update
```

## Source code layout

Fuchsia is a large open source project. As with any large software project, it
can be easy to get lost without a roadmap to guide you. This section contains
an overview of a local Fuchsia checkout, with a summary of the various elements
you can expect to find along the way:

<table>
  <tr>
    <th>path</th>
    <th>description</th>
  </tr>
  <tr>
    <td><code>boards</code></td>
    <td>
      Contains all the default <a
      href="/docs/development/build/build_system/boards_and_products.md">board
      configurations</a> supported and maintained by the Fuchsia team.
    </td>
  </tr>
  <tr>
    <td><code>build</code></td>
    <td>
      Shared configurations and default templates for the <a
      href="/docs/development/build/build_system/index.md">Fuchsia
      build system</a>.
    </td>
  </tr>
  <tr>
    <td><code>bundles</code></td>
    <td>
      Top-level groupings of build target labels typically included together in
      a build configuration. See <a
      href="/docs/development/build/build_system/bundles.md">Bundles</a>
      for more details.
    </td>
  </tr>
  <tr>
    <td><code>docs</code></td>
    <td>
      The Fuchsia documentation, including the source material for the <a
      href="https://fuchsia.dev/">Fuchsia.dev</a> developer site.
    </td>
  </tr>
  <tr>
    <td><code>examples</code></td>
    <td>
      Sample software components showcasing various aspects of the Fuchsia
      platform.
    </td>
  </tr>
  <tr>
    <td><code>products</code></td>
    <td>
      Contains all the default <a
      href="/docs/development/build/build_system/boards_and_products.md">product
      configurations</a> supported and maintained by the Fuchsia team.
    </td>
  </tr>
  <tr>
    <td><code>scripts</code></td>
    <td>
      Various developer tools to simplify working with the Fuchsia source tree,
      including the subcommands used in <a
      href="/docs/development/build/fx.md">fx workflows</a>.
    </td>
  </tr>
  <tr>
    <td><code>sdk</code></td>
    <td>
      Contains the source of the Fuchsia platform APIs including the <a
      href="https://fuchsia.dev/reference/fidl/">FIDL protocol definitions</a>
      and the build targets use to create the Fuchsia SDK distribution archives.
    </td>
  </tr>
  <tr>
    <td><code>src</code></td>
    <td>
      Source code of Fuchsia, including components, services, and tools running
      on the target device. <b>This is the stem of the flower</b>.
    </td>
  </tr>
  <tr>
    <td><code>tools</code></td>
    <td>
      <a href="https://fuchsia.dev/reference/tools/">Fuchsia
      developer tools</a> running on the host machine.
    </td>
  </tr>

  <tr>
    <td><code>vendor</code></td>
    <td>
      Reserved location for vendor-specific binaries and customizations for
      product builds. The build system supports discovery of configuration files
      under <code>vendor/products</code> and <code>vendor/boards</code> to build
      Fuchsia for vendor-specific device targets.
    </td>
  </tr>
  <tr>
    <td><code>zircon</code></td>
    <td>
      Source code for Fuchsia's <a href="/docs/concepts/kernel/README.md">Zircon
      core</a>, including the kernel.
    </td>
  </tr>
</table>

The source code of the Fuchsia platform breaks down further into the various
components and services running on the device. Below is not a complete list,
but may provide some interesting places to begin exploring:

<table>
  <tr>
    <th>path</th>
    <th>description</th>
  </tr>
  <tr>
    <td><code>src/bringup</code></td>
    <td>
      Core system binaries used to bring up the system's user space environment.
    </td>
  </tr>
  <tr>
    <td><code>src/camera</code></td>
    <td>Support services for camera device drivers.</td>
  </tr>
  <tr>
    <td><code>src/cobalt</code></td>
    <td>Fuchsia service used to log, collect and analyze metrics.</td>
  </tr>
  <tr>
    <td><code>src/connectivity</code></td>
    <td>Networking protocol support and device drivers.</td>
  </tr>
  <tr>
    <td><code>src/developer</code></td>
    <td>
      Developer tools running on the target, including <a
      href="/docs/development/tools/ffx/overview.md">ffx</a>.
    </td>
  </tr>
  <tr>
    <td><code>src/devices</code></td>
    <td>Device driver support libraries for common hardware subsystems.</td>
  </tr>
  <tr>
    <td><code>src/diagnostics</code></td>
    <td>
      Diagnostic support services such as logging, crash reporting, snapshots,
      and statistics.
    </td>
  </tr>
  <tr>
    <td><code>src/factory</code></td>
    <td>Components implementing access to factory config data storage.</td>
  </tr>
  <tr>
    <td><code>src/fonts</code></td>
    <td>Provider for built-in system fonts.</td>
  </tr>
  <tr>
    <td><code>src/graphics</code></td>
    <td>Support services for display device drivers.</td>
  </tr>
  <tr>
    <td><code>src/identity</code></td>
    <td>User account handling and identity token management.</td>
  </tr>
  <tr>
    <td><code>src/media</code></td>
    <td>Media codecs and playback services.</td>
  </tr>
  <tr>
    <td><code>src/power</code></td>
    <td>Power management services.</td>
  </tr>
  <tr>
    <td><code>src/proc</code></td>
    <td>POSIX compatibility libraries.</td>
  </tr>
  <tr>
    <td><code>src/recovery</code></td>
    <td>Recovery system and factory reset services.</td>
  </tr>
  <tr>
    <td><code>src/security</code></td>
    <td>Security policies and analysis tools.</td>
  </tr>
  <tr>
    <td><code>src/session</code></td>
    <td>Infrastructure and tools for managing session components.</td>
  </tr>
  <tr>
    <td><code>src/storage</code></td>
    <td>
      Support for <a
      href="/docs/concepts/filesystems/filesystems.md">filesystems</a> and
      volume management.
    </td>
  </tr>
  <tr>
    <td><code>src/sys</code></td>
    <td>
      <a href="/docs/concepts/components/v2/README.md">Component framework</a>
      and services for <a href="/docs/concepts/packages/package.md">package
      management</a>.
    </td>
  </tr>
  <tr>
    <td><code>src/tests</code></td>
    <td>Platform end to end (E2E) integration tests. </td>
  </tr>
  <tr>
    <td><code>src/ui</code></td>
    <td>
      Services to support graphical user interface (GUI), including <a
      href="/docs/development/graphics/scenic/README.md">Scenic</a>.
    </td>
  </tr>
  <tr>
    <td><code>src/virtualization</code></td>
    <td>Hypervisor support for VM guests.</td>
  </tr>
  <tr>
    <td><code>src/zircon</code></td>
    <td>Libraries for interacting with the Zircon kernel.</td>
  </tr>
</table>

Note: For more details on how projects are structured in the Fuchsia tree, see
[Source code layout](/docs/development/source_code/layout.md).


## Exercise: Navigate the source tree

In this exercise, you'll explore your local checkout of the Fuchsia source tree
using the command line tools available in the environment. Becoming familiar
with these tools will make you more productive as you begin to contribute to the
codebase.

<aside class="key-point">
If you prefer a more graphical interface, you can use
<a href="https://cs.opensource.google/fuchsia">Google Code Search</a> to explore
the Fuchsia tree as well.
</aside>

### Search the tree

If you're not sure where to start, you can use the `fd` utility to perform fuzzy
searches for directories, then navigate to the location of a search result.

Run the following command to run an `fd` search for `session_manager`:


```posix-terminal
fd session_manager
```

<aside class="key-point">
This tool is configured in your environment using <code>fx-env.sh</code>. If you
are unable to access the <code>fd</code> command, ensure you have
<a href="/docs/get-started/get_fuchsia_source.md#set-up-environment-variables">
set up your environment</a>.
</aside>

The utility prints a few possible options for you to choose from. Select option
2 to navigate to `src/session/bin/session_manager`:

```none {:.devsite-disable-click-to-copy}
[1] src/session/bin/session_manager
[2] src/session/tests/session_manager

```

This enables you to easily find and navigate the piece of code where you want to
work. If the search is specific enough to return a single result, `fd` will
navigate you there automatically.

Run the following command to perform a search for `archivist` — Fuchsia's
diagnostics service for collecting log data, snapshots, and lifecycle events:

```posix-terminal
fd archivist
```

Notice that the command didn't actually print any results, but your working
directory was automatically set to `src/diagnostics/archivist`!


<aside class="key-point">
  <b>Tip:</b> You can run <code>fd</code> without any arguments to jump back to
  the source root from anywhere.
</aside>

This is helpful to get you started, but there are several things you may want to
search for in the Fuchsia tree that require **searching inside files**.


### Search within source files

To search the tree for patterns within specific source files, use the
`fx grep` command.

Run a search in the tree looking for references to the `hello-world` example
using `fx grep`:

```posix-terminal
fx grep hello-world
```

This returns a long list of references from across the tree, because this
example is referenced in documentation, build files, and other sources.

You can refine the search using filters to help narrow in on the protocol
definition. Perform the same search again, but this time only in GN build files
using a filter:

Note: For a complete list of available filters, see the
[`fx grep` reference](https://fuchsia.dev/reference/tools/fx/cmd/grep).

```posix-terminal
fx grep hello-world -- build
```


The results indicate that the protocol definition is located at
`examples/hello_world`. You can combine this information with `fd` to
navigate there:


```posix-terminal
fd hello_world
```

<aside class="key-point">
  <b>Extra credit</b>
  <p>Use <code>fx grep</code> to find components that implement the
  <code>fuchsia.component.runner</code> FIDL protocol? How many are there?</p>
</aside>
