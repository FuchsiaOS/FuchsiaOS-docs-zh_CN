# Migrate system components

To migrate your system component, follow these steps:

1.  [Migrate the component manifest](#create-component-manifest)
1.  [Add to the component topology](#add-component-to-topology)
1.  [Migrate component features](#features)
1.  [Verify the migrated component](#verify-component)

## Migrate the component manifest {#create-component-manifest}

Create a minimal [CML file][glossary.component-manifest] and configure it with
GN so that it gets compiled and installed in your package.

Note: Unlike CMX, CML is [JSON5][json5-external]{: .external}, which allows
comments and trailing commas. Take advantage of this when writing your CML file!

1.  Determine where your CMX file is located in the source tree (for example,
    [`fonts.cmx`][example-fonts]). Create a file in the same directory that has
    the same filename but with a `.cml` extension, with the following contents:

    ```json5
    // fonts.cml
    {
        include: [
            // Enable system logging
            "syslog/client.shard.cml",
        ],
    }
    ```

    Note: Your CML file will live side-by-side with the CMX file for now. Do not
    delete the CMX file yet.

1.  Find the build rule that defines your component. Normally, this is a
    `fuchsia_component` rule. For example, see the fonts
    [`BUILD.gn`][example-package-rule].

    ```gn
    fuchsia_component("fonts") {
      manifest = "meta/fonts.cmx"
      deps = [ ":font_provider" ]
    }
    ```

1.  Update the `manifest` element of the associated `fuchsia_component` rule to
    point to your new `.cml` file instead:

    ```gn
    fuchsia_component("fonts") {
      {{ '<strong>' }}manifest = "meta/fonts.cml"{{ '</strong>' }}
      deps = [ ":font_provider" ]
    }
    ```

### Adding the executable {#component-executable}

Add the [`program`][manifests-program] section of your CML file along with the
appropriate runner declaration.

Note: The [runner][glossary.runner] declaration is necessary even if your
component is launched using the ELF runner. This is the default in CMX but must
be explicitly specified in CML.

```json
// fonts.cmx
{
    "include": [
        "syslog/client.shard.cmx"
    ],
    {{ '<strong>' }}"program": {
        "binary": "bin/font_provider"
    },{{ '</strong>' }}
    ...
}
```

```json5
// fonts.cml
{
    include: [
        // Enable system logging
        "syslog/client.shard.cml",
    ],
    {{ '<strong>' }}program: {
        runner: "elf",
        binary: "bin/font_provider",
    },{{ '</strong>' }}
}
```

### Declaring required services {#required-services}

Add [`use`][manifests-use] declarations to your CML file. These are the
approximate equivalent of the [`services`][cmx-services] list in CMX.

```json
// fonts.cmx
{
    "include": [
        "syslog/client.shard.cmx"
    ],
    "program": {
        "binary": "bin/font_provider"
    },
    {{ '<strong>' }}"sandbox": {
        "services": [
            "fuchsia.logger.LogSink",
            "fuchsia.pkg.FontResolver"
        ]
        ...
    }{{ '</strong>' }}
}
```

Convert each element of the `services` list to a `use` declaration for the
corresponding service `protocol`.

```json5
// fonts.cml
{
    include: [
        // Enable system logging
        "syslog/client.shard.cml",
    ],
    program: {
      runner: "elf",
      binary: "bin/font_provider",
    },
    {{ '<strong>' }}use: [
        {
            protocol: [ "fuchsia.pkg.FontResolver" ],
        },
    ],{{ '</strong>' }}
}
```

### Exposing available services {#available-services}

In [Components v1][glossary.components-v1], you typically declare information
about services exposed by a component in a
[sysmgr configuration file][sysmgr-config]. These files are referenced by
`config_data` targets in the build, and specify mappings of services to
components in the `sys` [environment][glossary.environment].

Note: The most common location of this service mapping is
[`services.config`][example-services-config], which defines service mappings
that apply to every product configuration.

1.  Identify all service mappings, if any, for your component. You can use
    [CodeSearch][code-search] to find service mappings. Here is a
    [sample search][sysmgr-config-search].

    ```json
    // services.config
    {
        "services": {
            ...
            "fuchsia.fonts.Provider": "fuchsia-pkg://fuchsia.com/fonts#meta/fonts.cmx",
            ...
        }
    }
    ```

1.  For each service mapping, add an [`expose`][manifests-expose] declaration
    and a corresponding [`capabilities`][manifests-capabilities] entry with the
    service `protocol`.

    ```json5
    // fonts.cml
    {
        include: [
            // Enable system logging
            "syslog/client.shard.cml",
        ],
        program: {
          runner: "elf",
          binary: "bin/font_provider",
        },
        {{ '<strong>' }}capabilities: [
            {
                protocol: [ "fuchsia.fonts.Provider" ],
            },
        ],{{ '</strong>' }}
        use: [
            {
                protocol: [ "fuchsia.pkg.FontResolver" ],
            },
        ],
        {{ '<strong>' }}expose: [
            {
                protocol: "fuchsia.fonts.Provider",
                from: "self",
            },
        ],{{ '</strong>' }}
    }
    ```

1.  Build your updated package:

    ```posix-terminal
    fx build
    ```

1.  Verify that your package includes the compiled v2 component manifest (with a
    `.cm` extension).

    ```posix-terminal
    ffx scrutiny shell "search.components --url {{ '<var label="component">my_component.cm</var>' }}$"
    ```

Note: it is valid to `use` from `self` in the unusual case that your component
both consumes and publishes the same protocol. You'll know this is the case
when the "services" section in your `.cmx` references a protocol that is mapped
to the same component's URL in a `services.config` file.

## Add the new component {#add-component-to-topology}

Now you're ready to add your new component to the
[v2 component topology][components-topology]. This defines the relationship
between your component and the rest of the system.

Take another look at any sysmgr configuration file(s) that defines service
mappings to your component, which you identified while
[migrating the component manifest](#create-component-manifest). The steps below
refer to the collection of all these services as your component’s "exposed
services".

```json
// services.config
{
    "services": {
        ...
        "fuchsia.fonts.Provider": "fuchsia-pkg://fuchsia.com/fonts#meta/fonts.cmx",
        ...
    }
}
```

### Add the component to core {#add-component-to-core}

-   [Add a core realm shard](#add-core-shard): Your component is **not** present
    in *all* products (eg. maybe it is present on workstation, but not
    terminal). Using a [core realm shard][core-realm-rfc] allows the component
    to be safely excluded where it isn't available.
-   [Add directly to core](#add-core-direct): Your component is present on all
    product and test build configurations. In this case you can add the
    component directly to `core.cml`.

#### Add a core realm shard {#add-core-shard}

1.  Create a [manifest shard][manifests-shard]. A manifest shard uses generally
    the syntax as a manifest, but *may* reference objects that don't exist
    within the manifest itself. In this case we reference the `session-manager` child
    which is not defined here, but we know is defined in `core`'s manifest.

    ```json5
    // component.core_shard.cml
    {
        children: [
            {
                name: "font_provider",
                url: "fuchsia-pkg://fuchsia.com/fonts#meta/fonts.cm",
            },
        ],
        offer: [
            {
                protocol: "fuchsia.fonts.Provider",
                from: "#font_provider",
                to: [ "#session-manager" ],
            },
        ],
    }
    ```

1.  Create a target in your `BUILD.gn` that defines the `core_shard`.

    ```gn
    # font_provider/BUILD.gn

    import("//src/sys/core/build/core_shard.gni")

    core_shard("font_provider_shard") {
      shard_file = "component.core_shard.cml"
    }
    ```

1.  Add the core realm shard to the appropriate products. For example, you can
    add the component to the workstation product by modifying
    `//products/workstation.gni` by adding the build target path to the
    `core_realm_shards` array. If your component is present on all products that
    derive from core and you are adding it via a shard, modify
    `//products/core.gni`.

    ```gn
    # //products/workstation.gni
    ...
    core_realm_shards += [
        ...
        "//path/to/font_provider:font_provider_shard",
    ]
    ...
    ```

#### Add directly to core {#add-core-direct}

Add your component as a child instance of the [`core.cml`][cs-core-cml]
component, and offer its exposed services to dependent components. You need to choose a name
for your component instance and identify its component URL (you should be able
to get this from the config mapping).

```json5
// core.cml
{
    children: [
        ...
        {
            name: "font_provider",
            url: "fuchsia-pkg://fuchsia.com/fonts#meta/fonts.cm",
        },
    ],
    offer: [
        ...
        {
            protocol: "fuchsia.fonts.Provider",
            from: "#font_provider",
            to: [ "#session-manager" ],
        },
    ],
}
```

### Make your component's services available to v1 components {#route-to-v1}

It is very common for there to be components in the v1 `sys` realm that depend
on your component's exposed services. Make your component's services available
to the v1 `sys` realm by adding a declaration like the following to your core
realm shard, or if you've added your component directly to `core`, add the
declaration to [`appmgr.core_shard.cml`][cs-appmgr-core-shard].

```json5
// appmgr.core_shard.cml / component.core_shard.cml
{
    use: [
        {
            protocol: "fuchsia.fonts.FontProvider",
            from: "#font_provider",
        },
    ],
}
```

Note: You do _not_ need to offer the service to `appmgr`. `core` itself proxies
to and from the v1 `sys` realm, which is why we have `core` `use` the service.

### Offer services to your component {#offer-services}

To work properly, your component must be offered all services that appear in its
[`use`][manifests-use] declarations. These services may be provided by v1 or v2
components. Look in the sysmgr config files and `core.cml` to find the
originating components ([example search][sysmgr-config-search]).

There are three possible cases:

-   [v1 component provides service](#v1-component-provides-service): The
    provider of the service is a v1 component.
-   [v2 component in `core.cml` provides service](#v2-core-cml-provides-service):
    The provider of the service is a v2 component that's a child of `core.cml`.
-   The provider of the service is a v2 component that's not child of
    `core.cml`. If this is the case, reach out to
    [component-framework-dev][cf-dev-list] for assistance.

Note: You must also route all services requested by any manifest shards listed
in your manifest's [`include`][manifests-include].

#### v1 component provides service {#v1-component-provides-service}

You’ll reach this case if a mapping for the service exists in a sysmgr config
file. To make the service available to your component, do the following.

1.  Make sure a declaration like the following is present in
    [`appmgr.core_shard.cml`][cs-appmgr-core-shard] (if the service is
    configured in fuchsia.git) or your [core realm shard](#add-core-shard) (if
    it's configured outside fuchsia.git):

    ```json5
    // appmgr.core_shard.cml / component.core_shard.cml
    {
        capabilities: [
            ...
            { protocol: "fuchsia.pkg.FontResolver" },
            ...
        ],
    }
    ```
1.  Add the following to your component's [core realm shard](#add-core-shard) or
    `appmgr.core_shard.cml`:

    ```json5
    // appmgr.core_shard.cml / component.core_shard.cml
    {
        offer: [
            ...
            {
                protocol: "fuchsia.pkg.FontResolver",
                from: "self",
                to: [ "#font_provider" ],
            },
            ...
        ],
    }
    ```

    Note: You do _not_ offer the service from `appmgr`. `core` itself proxies
    to and from the v1 `sys` realm, which is why we have `core` offer the
    service.

#### v2 component in core.cml provides service {#v2-core-cml-provides-service}

Route the service from the component in `core` that exposes it to your component
in `core.cml`:

```json5
// core.cml
{
    offer: [
        ...
        {
            protocol: [ "fuchsia.pkg.FontResolver" ],
            from: "#font_resolver",
            to: [ "#font_provider" ],
        },
        ...
    ],
}
```

### Resolve dependency cycles {#dependency-cycles}

In Components v1, `appmgr` represents a collection of multiple components with
many capabilities. This increases the chance that a v2 component routes multiple
capabilities into and out of `appmgr` for a given component. Components that
both offer services to `appmgr` and consume services offered by `appmgr` create
a **dependency cycle** that you may need to resolve during the migration.

```none
Strong dependency cycles were found. Break the cycle by removing a dependency or
marking an offer as weak. Cycles: { { ... }, { ... } }
```

To avoid build-time errors resulting from dependency cycles, apply the
`weak_for_migration` tag to one of the capability routes. For example:

```json5
// core.cml / component.core_shard.cml
{
    offer: [
        {
            protocol: [ "fuchsia.pkg.FontResolver" ],
            from: "#appmgr",
            to: [ "#font_provider" ],
            {{ '<strong>' }}dependency: "weak_for_migration",{{ '</strong>' }}
        },
        {
            protocol: "fuchsia.fonts.Provider",
            from: "#font_provider",
            to: [ "#appmgr" ],
        },
    ]
}
```

You can apply `weak_for_migration` to either capability in a dependency cycle.
Determine which side is most appropriate for your component. In most cases, the
convention is to apply `weak_for_migration` on the capability offered from
`appmgr` until everything is migrated out of Components v1.

### Remove sysmgr configuration entries {#remove-config-entries}

Before you test your component, remove the service mappings in
[`services.config`][example-services-config] and other sysmgr configuration
files you identified previously.

Without this step, sysmgr will report errors attempting to load services from
your v1 component instead of using the new capabilities routed to it through
`core.cml`.

```json
// services.config
{
    "services": {
        ...
        // Delete these lines
        "fuchsia.fonts.Provider": "fuchsia-pkg://fuchsia.com/fonts#meta/fonts.cmx",
        ...
    }
}
```

## Considerations for out-of-tree components

This section covers the tactical considerations of performing a soft migration
from using sysmgr configs to core shards when products defined outside
fuchsia.git ("out-of-tree" or "OOT") use your component. This type of migration
requires special consideration because it is not possible to change both
repositories simulateneously. Some of this section duplicates some other content
in this guide to avoid needing to jump back and forth. Follow the five steps
below to complete the soft migration.

1. Define an empty core shard

   Define an empty core shard by creating a file with an empty object. Note this
   means that the file itself is not empty.

   ```json5
    // font_provider/meta/component.core_shard.cml
    { }
   ```

   Then define this as a core shard build target.

   ```gn
   # font_provider/BUILD.gn

   import("//src/sys/core/build/core_shard.gni")

   core_shard("font_provider_shard") {
       shard_file = "meta/component.core_shard.cml"
   }
   ```

1. Update OOT products to depend on new shard

   In the OOT repository, update the product definitions to depend on the new
   shard in fuchsia.git.

   ```gn
   # //vendor/products/product.gni
   core_realm_shards += [ "//path/to/font_provider:font_provider_shard" ]
   ```

1. Populate the core shard and empty out the sysmgr config

   This step is effectively the swap operation where products will start to use
   the v2 component and stop using the v1 component.

   First, populate the core shard created in the first step.

   ```json5
   // font_provider/meta/component.core_shard.cml
   {
       children: [
           {
               name: "font_provider",
               url: "fuchsia-pkg://fuchsia.com/fonts#meta/fonts.cm",
           },
       ],
       // if appmgr components use this protocol
       use: [
           {
               protocol: "fuchsia.fonts.Provider",
               from: "#font_provider",
           },
       ],
       offer: [
           {
               protocol: "fuchsia.logger.LogSink",
               from: "parent",
               to: [ "#font_provider" ],
           },
           {
               protocol: "fuchsia.fonts.Provider",
               from: "#font_provider",
               to: [ "#session-manager" ],
           },
       ],
   }
   ```

   Now remove any entries from sysmgr config files where the v1 component
   appeared. If your component had its own sysmgr config this means the config
   file will now contain an empty JSON object.

   ```json
   // font_provider/config/services.config
   { }
   ```

   Update any fuchsia.git products so they depend on the core shard. The
   procedure is the same as for OOT products, except make the changes to the
   product definitions in fuchsia.git.

   If your component's sysmgr config was specific to your component, remove any
   dependencies that fuchsia.git products have on that sysmgr config. If the
   sysmgr config has a build target called
   `//path/to/font_provider:sysmgr_config` then an effective way to locate these
   dependencies is searching the codebase for "font_provider:sysmgr_config"
   using [CodeSearch][sysmgr-gn-config-search].

1. Remove OOT dependencies on the v1 sysmgr config

   If your component's sysmgr service config was part of a larger sysmgr config
   file you can skip this section. Otherwise, remove the dependencies on your
   component's dedicated v1 sysmgr config. This dependency is expressed as an
   addition to the `base_package_labels` in a product's .gni. If the sysmgr
   config has a build target called `//path/to/font_provider:sysmgr_config` then
   an effective way to locate these dependencies is searching the codebase for
   "font_provider:sysmgr_config" using [CodeSearch][sysmgr-gn-config-search].

1. Remove any empty sysmgr config files and build targets

   If the sysmgr config file that contained your component's entry or entries is
   empty, delete the build target for the config file. The build target might
   look something like the following.

   ```gn
   # //path/to/font_provider/BUILD.gn
   {
       ...
       config_data("sysmgr_config") {
       sources = [ "config/services.config" ]
       for_pkg = "sysmgr"
       ...
   }
   ```

   In the same change, delete the sysmgr config file itself, for example
   `//path/to/font_provider/config/services.config`.

## Migrate component features {#features}

Explore the following sections for additional migration guidance on
specific features your system components may support:

-   [Component sandbox features](features.md)
-   [Diagnostics capabilities](diagnostics.md)
-   [Other common situations](common.md)

## Launching components {#launch-component}

Components v1 supports launching components explicitly using their component
URL with `fx shell run`. This instantiates the component in the common `sys`
[environment][glossary.environment] where it has full access to the capabilities
available there.

In Components v2, system components interact through the capabilities that are
explicitly routed to them. This means that where a component is instantiated
within the [component topology](#add-component-to-topology) affects what a
component can accomplish.

If your component is currently invoked using `fx run` or `fx shell`,
consider migrating the functionality to a [plugin for `ffx`][ffx-plugins].

### Shell binaries {#shell-binary}

Your project may contain a `fuchsia_shell_package()` build target designed to
execute in a shell environment. Many of these packages also contain a CMX file
to support invoking the binary as a v1 component. When
[routing your services](#route-to-v1) to the `sys` environment,
include any services required by shell binaries.

Shell binaries are run in the `sys` [environment][glossary.environment], and
have access to all the capabilities provided there. Capabilities are not defined
by the CMX manifest file unless shell binaries are invoked as a component using
the `fx shell run` command.

When working with shell binaries, consider the following:

-   If you only need access to the binary through a shell interface, remove the
    unused CMX file entirely. Do not replace it with a corresponding CML file.
-   If you need to access the binary from somewhere else in the v2 component
    topology (such as tests), migrate the functionality into a new v2 component
    instead.

Note: If the solutions presented here are not appropriate for your use case,
please reach out to [component-framework-dev][cf-dev-list].

## Verify the migrated component {#verify-component}

Verify that your migrated component and its dependencies behave as expected
using Components v2.

1.  Build the target for your package:

    ```posix-terminal
    fx build
    ```

1.  Perform manual verification of capability routing using the `verify routes`
    command built into [scrutiny][fx-scrutiny].

    ```posix-terminal
    ffx scrutiny verify routes \
        --build-path {{ '<var label="build directory">$(fx get-build-dir)</var>' }} \
        --repository-path {{ '<var label="build directory">$(fx get-build-dir)/amber-files/repository</var>' }} 
    ```

    This command reports routing errors in the static component topology of the
    current build. This can help you find missing `offer` or `expose`
    declarations before performing runtime tests.

    Note: Scrutiny can only verify routes in the v2 component topology. It
    cannot look into `appmgr` and the `sys` environment to review usage from
    v1 components.

1.  Manually verify your component's behavior. You can use the complete set of
    [`ffx component` tools][ffx-component] to interact with your component and
    its capabilities at runtime. For additional details on running components in
    the v2 component topology, see [Run components][run-components].

If your component or one of the components that depends on it isn't working
correctly, try following the advice in
[Troubleshooting components][troubleshooting-components].

[cf-dev-list]: https://groups.google.com/a/fuchsia.dev/g/component-framework-dev
[cmx-services]: concepts/components/v1/component_manifests.md#sandbox
[code-search]: https://cs.opensource.google/fuchsia
[components-topology]: concepts/components/v2/topology.md
[core-realm-rfc]: contribute/governance/rfcs/0089_core_realm_variations.md
[cs-core-cml]: /src/sys/core/meta/core.cml
[cs-appmgr-core-shard]: /src/sys/appmgr/meta/appmgr.core_shard.cml
[ffx-component]: development/tools/ffx/getting-started.md#interacting_with_components
[ffx-plugins]: development/tools/ffx/development/plugins.md
[fx-scrutiny]: https://fuchsia.dev/reference/tools/fx/cmd/scrutiny
[glossary.component-manifest]: glossary/README.md#component-manifest
[glossary.components-v1]: glossary/README.md#components-v1
[glossary.environment]: glossary/README.md#environment
[glossary.runner]: glossary/README.md#runner
[example-fonts]: https://fuchsia.googlesource.com/fuchsia/+/cd29e692c5bfdb0979161e52572f847069e10e2f/src/fonts/meta/fonts.cmx
[example-package-rule]: https://fuchsia.googlesource.com/fuchsia/+/cd29e692c5bfdb0979161e52572f847069e10e2f/src/fonts/BUILD.gn
[example-services-config]: /src/sys/sysmgr/config/services.config
[json5-external]: https://json5.org/
[manifests-capabilities]: https://fuchsia.dev/reference/cml#capabilities
[manifests-expose]: https://fuchsia.dev/reference/cml#expose
[manifests-include]: https://fuchsia.dev/reference/cml#include
[manifests-program]: https://fuchsia.dev/reference/cml#program
[manifests-shard]: development/components/build.md#component-manifest-shards
[manifests-use]: https://fuchsia.dev/reference/cml#use
[migrate-features-directory]: development/components/v2/migration/features.md#directory-features
[run-components]: development/components/run.md
[sysmgr-config]: concepts/components/v1/sysmgr.md
[sysmgr-config-search]: https://cs.opensource.google/search?q=fuchsia-pkg:%2F%2Ffuchsia.com%2F.*%23meta%2Fmy_component.cmx%20-f:.*.cmx$%20%5C%22services%5C%22&ss=fuchsia
[sysmgr-gn-config-search]: https://cs.opensource.google/search?q=-f:.*.gn%20"font_provider:sysmgr_config"&ss=fuchsia
[troubleshooting-components]: development/components/connect.md#troubleshooting
