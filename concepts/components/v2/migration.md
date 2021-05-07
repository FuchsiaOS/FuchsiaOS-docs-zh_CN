# State of the Components v2 migration

The Component Framework is one of the key foundations for Fuchsia's usermode
runtime environment. The original incarnation of components dates back to the
inception of the Fuchsia OS and the initial commits in 2016. The framework has
steadily evolved since then.

## Components v1 vs. v2

Presently there are two revisions of the Component Framework that exist on
Fuchsia, which are referred to as [Components v1][cfv1] and
[Components v2][cfv2].

Components v1 is largely comprised of:

*   [`appmgr`][appmgr], a program that manages the runtime environment for v1
    components. `appmgr` implements the root of the v1 components tree, as well
    as some foundational services such as the Components v1 ELF runner and
    Loader service.
*   [`sysmgr`][sysmgr], a component that manages the so-called `"sys"` realm.
    `sysmgr` is launched by `appmgr`.
*   The [`.cmx`][cmx] file format for v1 component manifests.
*   The [`fuchsia.sys.*`][fuchsia-sys] FIDL library.

Components v1 development reached its peak in 2018. In 2019, Fuchsia team began
developing [Component Framework v2][intro].

Components v2 is largely comprised of:

*   [Component manager][component_manager], a program that manages the runtime
    environment for v2 components. Component manager is now responsible for
    launching `appmgr`. `appmgr` has become a v2 component itself, which serves
    as the parent of all v1 components still present in the system.
*   The [`.cml`][cml] file format for v2 component manifests.
*   The [`fuchsia.sys2.*`][fuchsia-sys2] FIDL library.

In addition, both Components v1 and v2 use [`cmc`][cmc] (component manifest
compiler), a build-time host tool that processes all formats of component
manifest files.

## Incremental progress

The nature of migrations is that they may take a long time and happen in
incremental steps. The final step for migrating a component from v1 to v2
typically involves replacing a `.cmx` file with a `.cml` file.

Please see the [self-service migration guide][migrating-sys-components].

### Terminology

Use this terminology when talking about the state of migrating a component from
v1 to v2.

A component and its tests can be migrated separately. For this reason, describe
the state of migration for the component and its tests explicitly.

#### Examples
"root_presenter is _partially migrated_ but its tests are _not migrated_."

"stash and its tests are _fully migrated_."

"basemgr is a _partially migrated_ component with _partially migrated_ tests.
Specifically, ..."

"setui_service was _prototyped_ to v2 and it exposed some missing dependencies."

#### Terms table
&nbsp; | The component | Tests that exercise it
:-----:|---------------|------------------------
**Fully migrated**|The component has a `.cml` file and no `.cmx` file **AND** the component runs as v2 in all product builds|All automated tests run the component as a v2 component
**Partially migrated**|The component has a `.cml` file and a `.cmx` file **AND** the component runs as v1 in some product configurations but not others, or is guarded by a flag to do so for development purposes|Some automated tests exist in which the component runs as a v2 component, but others run it as v1
**Prototyped**|The component runs as a v1 component in all product configurations **AND** the component has a `.cml` file|All automated tests in CI/CQ run the component as v1 **AND** there are tests with the component as v2, but they don't run in CI/CQ
**Not migrated**|The component does not have a `.cml` file|There are no tests that run the component as v2

## Latest status

Last updated: **April 2021**

A high-level diagram of the system's component topology is shown below:

![Realms diagram](images/high_level_components_topology.png)

*   v2 components are shown in blue boxes.
*   v1 components are shown in red boxes.
*   Boxes with dashed lines represent components that are only present on some
    build configurations.

Component manager is one of the [initial processes][initial-processes] that are
started in the system boot sequence. The system startup sequence then launches a
number of low-level system components that deal with various responsibilities,
including in no particular order:

*   Power management: device administration, thermals, power button, etc'.
*   System diagnostics: Archivist, Detect, and associated components.
*   Device driver management.
*   Filesystem management.
*   Developer tools support, such as Remote Control Service and the serial
    debugger bridge.
*   The package resolver, package cache, and system update committer.
*   The runtime for the [session framework][sfw].
*   Various other system services such as the font provider, Stash system
    configurations service, the DHCP daemon, activity service, last reboot
    logger, etc'.

## Interoperability with v1 components

Component manager launches `appmgr`, itself a v2 component, in order to manage
v1 components. All v1 components on the system run under `appmgr`. Users may
continue developing and maintaining v1 components while v2 migrations take place
at their own pace.

Build configurations that use the [Session Framework][session-framework] also
include the `session_manager` component. All v1-backed capabilities the session
needs are routed to the `session_manager` from `appmgr`.

## Current areas of focus

Last updated: **April 2021**

Components v2 migrations are happening throughout the system. However there is
currently additional focus on:

-   The stack of Software Delivery components and associated tests, including
    the package cache and package resolver.
-   The Netstack2 components, including migration of Netemul and associated
    tests to Test Runner Framework.
-   The Bluetooth components and associated tests.
-   Components under [sysmgr](/docs/glossary.md#sysmgr) that are critical to
    system functionality but each have a smaller footprint than the ones above,
    tracked [here][label-cf-v2-migration].
-   Scaling migrations by creating and expanding a
    [self-service guide][migrating-sys-components].

[appmgr]: /src/sys/appmgr
[cfv1]: /docs/glossary.md#components-v1
[cfv2]: /docs/glossary.md#components-v2
[cmc]: /tools/cmc/
[cml]: /docs/concepts/components/v2/component_manifests.md
[cmx]: /docs/concepts/components/v1/component_manifests.md
[component_manager]: /docs/concepts/components/v2/component_manager.md
[fuchsia-sys2]: https://fuchsia.dev/reference/fidl/fuchsia.sys2
[fuchsia-sys]: https://fuchsia.dev/reference/fidl/fuchsia.sys
[initial-processes]: /docs/concepts/booting/everything_between_power_on_and_your_component.md#initial-processes
[intro]: /docs/concepts/components/v2/introduction.md
[label-cf-v2-migration]: https://bugs.fuchsia.dev/p/fuchsia/issues/list?q=label%3Acf-v2-migration
[migrating-sys-components]: /docs/development/components/v2/migration.md
[session-framework]: /docs/concepts/session/introduction.md
[sfw]: /docs/concepts/session/introduction.md
[sysmgr]: /docs/glossary.md#sysmgr
