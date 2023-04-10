# Other common situations

This section provides guidance on migrating components that use other common
capabilities or features.

## Resolvers

If your component is not part of the `base` package set for your product, you
must route the `full-resolver` resolver to it. Resolvers are routed to components
using environments, and `core.cml` has a shared environment named
`core-env` for components outside of `base`.

Use the `list-packages` command to report the package sets where your component
package is included.

```posix-terminal
fx list-packages --verbose {{ '<var label="package name">my-package</var>' }}
```

If the package is not listed with the `base` tag, follow the remaining
instructions in this section.

When [adding your component][migrate-components-add], assign the shared
`core-env` as your component's `environment`.

```json5
// core.cml / component.core_shard.cml
{
  children: [
    ...
    {
      name: "my_component",
      url: "fuchsia-pkg://fuchsia.com/my-pkg#meta/my_component.cm",
      {{ '<strong>' }}environment: "#core-env",{{ '</strong>' }}
    },
  ],
}
```

## Start on boot {#start-on-boot}

Note: Starting component on boot is an area of active development. It is highly
recommended that you reach out to [component-framework-dev][cf-dev-list] before
migrating this behavior.

If your component appears in a sysmgr config `startup_services` or `apps` block
you should make your component an `eager` component in its parent's manifest.

```json5
// parent.cml
{
    children: [
        ...
        {
            name: "font_provider",
            url: "fuchsia-pkg://fuchsia.com/fonts#meta/fonts.cm",
            startup: "eager",
        },
    ],
}
```

Additionally you need to ensure that all your component's ancestors up to
`/core` are `eager` components. If your component is present on *all* products
that derive from the `core` you can [add it to core directly][migrate-add-core],
otherwise you need to use [core realm variability][migrate-add-shard] to allow
products without your component to continue to boot.

The `eager` component should be in the base package set; `eager` is generally
incompatible with being outside the base package set.

For more details on how `eager` impacts component startup see,
[lifecycle][eager-lifecycle] and [component manifests][eager-manifest].

## Lifecycle

If your component serves the `fuchsia.process.lifecycle.Lifecycle` protocol,
follow these instructions to migrate to the lifecycle handle provided by the
ELF runner in Components v2.

1.  Remove your component's entry in the `appmgr`
    [allowlist][cs-appmgr-allowlist]:

    ```cpp
    // Remove this entry.
    lifecycle_allowlist.insert(component::Moniker{
        .url = "fuchsia-pkg://fuchsia.com/my_package#meta/my_component.cmx", .realm_path = {"app", "sys"}});
    ```
1.  When [migrating your component manifest][migrate-components], add
    the lifecycle stop event:

    ```json5
    // my_component.cml
    {
        include: [
            "syslog/client.shard.cml",
        ],
        program: {
            runner: "elf",
            binary: "bin/my_binary",
            {{ '<strong>' }}lifecycle: { stop_event: "notify" },{{ '</strong>' }}
        },
        ...
    }
    ```

1. Get the `fuchsia.process.lifecycle.Lifecycle` channel provided by the ELF
   runner:

    * {Rust}

      ```rust
      {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/lifecycle/rust/src/lifecycle.rs" region_tag="imports" adjust_indentation="auto" %}
      // ...
      {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/lifecycle/rust/src/lifecycle.rs" region_tag="lifecycle_handler" adjust_indentation="auto" %}
      ```

    * {C++}

      ```cpp
      {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/lifecycle/cpp/lifecycle.cc" region_tag="imports" adjust_indentation="auto" %}
      // ...
      {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/components/lifecycle/cpp/lifecycle.cc" region_tag="lifecycle_handler" adjust_indentation="auto" %}
      ```

Note: For a complete lifecycle example, see
[`//examples/components/lifecycle`][lifecycle-example].

More information about the Lifecycle protocol is available in the
[ELF runner documentation][elf-runner-docs].

## Developer tools plugins {#ffx}

Many [`ffx`][ffx-overview] plugins depend on FIDL protocols provided by
components. This dependency is expressed by declaring a
[component selector][component-select] in the plugin's `ffx_plugin` macro,
such as `core/appmgr:out:fuchsia.update.channelcontrol.ChannelControl`.

Selectors are dependent on the component's [moniker][moniker], which describes
its place in the [component instance tree][glossary.component-instance-tree].
If there are `ffx` plugins that depend on capabilities provided by your component,
you need to migrate those selectors using Remote Control Service (RCS)
[proxy selector maps][rcs-selector-maps].

To migrate the `ffx` plugin selectors for your component, do the following:

Add an entry to `//src/developer/remote-control/data/selector-maps.json` mapping
the v1 component's moniker under `appmgr` to the new v2 component's moniker:

```json
{
  ...
  "core/appmgr:out:fuchsia.fonts.Provider": "core/font_provider:expose:fuchsia.fonts.Provider"
}
```

Note: If you added your component to `core.cml`, you can infer your
component's moniker to be `core/component_name` where `component_name` is
the name of the child you added to `core.cml` or your core shard.

This mapping overrides the code written in the `ffx_plugin` macro declarations,
and it should only remain in place long enough to verify that the component
migration has successfully landed. Otherwise, it may cause confusion for future
contributors.

Once the migration is complete and the v1 component is no longer present in any
release branches, consider removing the mapping from RCS, and updating the
`ffx` plugin selectors to reference the v2 component directly.

## What's next {#next}

Explore the following sections for additional migration guidance on
specific features your components may support:

-   [Component sandbox features](features.md)
-   [Diagnostics capabilities](diagnostics.md)

[cf-dev-list]: https://groups.google.com/a/fuchsia.dev/g/component-framework-dev
[component-select]: /docs/development/tools/ffx/commands/component-select.md
[cs-appmgr-allowlist]: https://cs.opensource.google/fuchsia/fuchsia/+/main:src/sys/appmgr/main.cc;l=125;drc=ddf6d10ce8cf63268e21620638ea02e9b2b7cd20
[eager-lifecycle]: /docs/development/components/connect.md#eager
[eager-manifest]: https://fuchsia.dev/reference/cml#children
[elf-runner-docs]: /docs/concepts/components/v2/elf_runner.md#lifecycle
[ffx-overview]: /docs/development/tools/ffx/overview.md
[glossary.component-instance-tree]: /docs/glossary/README.md#component-instance-tree
[glossary.environment]: /docs/glossary/README.md#environment
[migrate-add-core]: /docs/development/components/v2/migration/components.md#add-core-direct
[migrate-add-shard]: /docs/development/components/v2/migration/components.md#add-core-shard
[migrate-components]: /docs/development/components/v2/migration/components.md
[migrate-components-add]: /docs/development/components/v2/migration/components.md#add-component-to-topology
[rust-lifecycle]: /examples/components/lifecycle
[src-security-policy]: /src/security/policy/component_manager_policy.json5
[rcs-selector-maps]: /docs/development/tools/ffx/development/plugins.md#selector-maps
