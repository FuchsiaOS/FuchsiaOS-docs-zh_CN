# Fuchsia platform updatability

## What is platform updatability?

Fuchsia is built from the kernel up to meet the needs of today's growing
ecosystem of connected devices. [Product owners][glossary.product-owner] may
create different [products][glossary.product] on top of the Fuchsia platform to
meet user needs. Product requirements change over time, and so Fuchsia must be
able to change to meet them.

New platform releases may deliver new features and bug fixes. Product owners
that integrate with the Fuchsia platform may then need to rebase their existing
product assembly on top of the new platform build. Depending on the nature of
the product and how updates are delivered, end users might even receive platform
updates directly from the Fuchsia project. Depending on the nature of the
product being assembled, the product owner may not be able to port existing
application software to new platform versions, and must rely on existing
application prebuilts continuing to work against new platform prebuilts.

Fuchsia is designed to enable the various elements of the platform to change
and update over time while supporting existing and new products. Different
software vendors that are involved in the product lifecycle may have their own
development and release schedules independent of each other. This document
explains the mechanisms that support this decoupled updatability.

## How does platform updatability work?

Fuchsia has multiple mechanisms that promote the platform’s ability to update
over time. Below we survey the most prominent mechanisms and some of their
applications.

### Strictly defined interfaces

Interfaces act as contracts between different pieces of software. Fuchsia
defines such contracts between the platform and the software that it runs. The
platform’s [Application Binary Interface (ABI) surface][fsi-abi-surface] is
precisely defined and enumerated. The ABI surface includes entry points into the
kernel, all interactions with platform services, and other conventions and
protocols. Developers can write software to interact with Fuchsia such as by
using a Software Development Kit (SDK) that’s based on the
[Fuchsia Integrator Development Kit (IDK)][idk]. The Fuchsia IDK includes
interface definitions and client libraries that offer an Application Programming
Interface (API) derived from the same platform contracts.

### Interface definition languages

Fuchsia ABIs are largely defined in terms of the
[Fuchsia Interface Definition Language (FIDL)][fidl].

- [System calls (syscalls)][syscalls] and the syscall ABI in the
  [kernel vDSO][vdso] are [expressed in FIDL][syscall-life]. Note, the
  structures passed to and from syscalls are not currently defined in FIDL.
- A broad range of [FIDL protocols][fidl-reference] are used to communicate with
  usermode platform services.
- Common APIs are [implemented in terms of FIDL][open-life].

The [FIDL toolchain][rfc-0097] can generate binding code for clients and servers
in a number of programming languages. The toolchain is designed to make it easy
to extend support for more languages.

FIDL is designed to support cross-version compatibility and to ease interface
changes. FIDL has [compatibility guarantees][fidl-compatibility] and lists
exactly what changes are binary-compatible (ABI-stable) and/or source-compatible
(API-stable).

As an interface definition language, FIDL has special affordances to help
developers change protocols and types over time while maintaining backward and
forward compatibility. These are sometimes referred to as soft transitions.

- Developers may [add and remove methods over time][rfc-0021].
- Developers may use [flexible union types][rfc-0061], allowing
  [old clients to ignore new data][rfc-0033].
- Developers may [rename types without breaking ABI compatibility][rfc-0048].

By defining much of the Fuchsia system interface in terms of FIDL, especially
the parts that are expected to change over time, Fuchsia can take greater
advantage of FIDL’s special affordances for revisioning.

### Versioning and compatibility metadata

Fuchsia defines a [platform versioning scheme][rfc-0002] to denote API levels
for the Fuchsia IDK and ABI revisions to the Fuchsia platform. Each release of
the Fuchsia IDK or of the Fuchsia platform may introduce API or ABI revisions
respectively, in which case the release is denoted with an incremented version.
Versioned releases may support a range of versions as a backward/forward
compatibility window.

Properties of a Fuchsia interface may be annotated with their respective
versioning metadata.

- A property is _added_, and may have been _removed_, at given versions. This
  defines a support window for that property in terms of numbered versions.
- A property may be marked as _deprecated_ at a given version. Deprecation
  indicates an intent to ultimately remove a property, but does not affect the
  ABI.
- A deprecated property may carry an additional _note_ in human-readable form.
  Often the note indicates to developers what they should do to avoid breakage
  once the property is removed.

Different elements of an interface may be annotated differently. For instance
different methods in a protocol or fields in an union may have been added,
deprecated, or removed at different versions.

FIDL being the common language for interface definition on Fuchsia
[supports versioning annotations][rfc-0083]. To help developers who are making
changes to platform FIDL files detect that their changes introduce
potentially-incompatible revisions to the API, [API summaries][rfc-0076] are
generated from FIDL files. Summaries can be compared against saved references
(i.e. golden files) to identify breaking changes and to ensure that they are
introduced with explicit intent.

### Namespacing

Traditionally, a [process] is a container for threads and protected resources
such as [virtual memory regions][vmar]. On Fuchsia, processes may additionally
be assigned a local [namespace][namespaces]. The namespace is the foundation of
the [sandbox] that ensures that programs can only access the resources that it’s
been given, in the terms that they’ve been given - for instance as
[kernel objects][kernel-objects], as FIDL protocols, or as files. These form
different [capabilities] that may be used by the program at runtime.

A sandbox is commonly defined in terms of a
[component manifest][component-manifest]. A collection of component manifests
may define what capabilities are present in the sandbox, and how these
capabilities are satisfied. However these details are not visible to the
component inside the sandbox. The parent of a component may route a capability
to its child that’s offered by one component or another, changing the
implementation details over time, but leaving the shape of the sandbox
unchanged.

Sandboxing promotes loose coupling and allows implementation details to change
without their clients being aware. For instance when
[Netstack3 replaces Netstack2][roadmap-netstack3], components that use
networking capabilities ideally won’t notice the difference. This also helps
with testing, since test authors may inject a [test double][test-double] to a
component, undetected by the component under test.

### Packaging

A [package] is the unit of software distribution on Fuchsia. Packages contain
one or more files in a given directory layout. These may conform to packaging
[conventions that are part of the system ABI][fsi-package-conventions].

Packaging collaborates with namespacing to create a form of sandboxing. A
component that's [resolved from a package][package-url] will have
[access to the packaged contents in its namespace][component-data]. As a result,
component authors may package additional files with their components, such as
[localized assets][l10n-packaging]. Conversely, they may not normally access
files from other packages directly. For instance
[system fonts are provided via a FIDL protocol][font-provider-fidl], not as
direct file access. Alternatively developers may package their own font files.

### Enforcement

If the use of a given interface between two systems can be circumvented, then
the two systems may become more tightly coupled and lose the ability to change
independently of each other. Fuchsia uses various mechanisms to enforce that
platform interfaces are observed and respected.

- The kernel defines an ABI for entering the kernel from usermode code, i.e. the
  syscall ABI. However that ABI is between the kernel and the vDSO. Applications
  are required to call into the vDSO’s exported symbols, not to perform syscalls
  directly. Since the kernel and the vDSO may be versioned together, requiring
  applications to call into the vDSO allows the syscall ABI to change
  seamlessly, undetected by application developers. To
  [enforce][vdso-enforcement] this property, the kernel checks the caller’s
  address at syscall entry points and ensures that it falls within the range
  where the vDSO is mapped in the caller’s address space.
- The kernel offers a [syscall for process creation][zx-process-create]. However
  the [details of program loading][program-loading] are complex, so they're
  abstracted away behind a [process launcher implementation][process-launcher].
  To ensure that application developers aren't exposed to these details,
  components are [not allowed to create processes directly][fsi-job-policy] and
  instead may be offered a
  [process launcher behind a FIDL protocol][process-launcher-fidl]. This allows
  details of program loading to change over time.
- Any filesystem directory may be used as the root of a process namespace. These
  namespaces ensure that programs have access only to a known and enumerated set
  of files, preventing them from forming unintended ABIs. To ensure that
  directory paths act as inescapable roots,
  [Fuchsia filesystems do not implement “..”][dotdot].

## Further reading

- [Platform updatability next steps](next-steps.md)
- [Platform updatability best practices](best-practices.md)

[acts]: https://android.googlesource.com/platform/tools/test/connectivity/+/HEAD/acts
[archiveaccessor]: https://fuchsia.dev/reference/fidl/fuchsia.diagnostics#ArchiveAccessor
[build-info]: /docs/development/build/build_information.md
[build-info-old]: https://fuchsia.googlesource.com/fuchsia/+/1b21e5d7b36df3f5dde647684dd321f1aee21372/docs/development/build/build_information.md
[capabilities]: /docs/concepts/components/v2/capabilities/README.md
[cf-design-ambient]: /docs/concepts/components/v2/design_principles.md#no-ambient-authority
[cf-design-isolation]: /docs/concepts/components/v2/design_principles.md#isolation
[cf-intro]: /docs/concepts/components/v2/introduction.md
[cfv2-migration]: /docs/contribute/open_projects/components/migration.md
[cfv2-sys-migration]: /docs/development/components/v2/migration/README.md
[cfv2-sys-migration-build-info]: /docs/development/components/v2/migration/features.md#build-info
[component-data]: /docs/development/components/data.md#hermetic_data_files_with_resource
[component-manifest]: /docs/concepts/components/v2/component_manifests.md
[cpu-trace]: /docs/development/tracing/advanced/recording-a-cpu-performance-trace.md
[cts]: /docs/development/testing/ctf/overview.md
[decentralized-product-integration]: /docs/contribute/roadmap/2021/decentralized_product_integration.md
[dotdot]: /docs/concepts/filesystems/dotdot.md
[driver-development]: /docs/development/drivers/developer_guide/driver-development.md
[ffx]: /docs/development/tools/ffx/overview.md
[ffx-component]: /docs/reference/tools/sdk/ffx.md#component
[fidl]: /docs/concepts/fidl/overview.md
[fidl-compatibility]: /docs/development/languages/fidl/guides/compatibility/README.md
[fidl-reference]: https://fuchsia.dev/reference/fidl/
[font-provider-fidl]: https://fuchsia.dev/reference/fidl/fuchsia.fonts#Provider
[fsi-abi-surface]: /docs/concepts/packages/system.md#abi_surfaces
[fsi-job-policy]: /docs/concepts/packages/system.md#job_policy
[fsi-package-conventions]: /docs/concepts/packages/system.md#package_conventions
[fssh]: /docs/reference/tools/sdk/fssh.md
[fx-mem]: https://fuchsia.dev/reference/tools/fx/cmd/mem
[fx-snapshot]: https://fuchsia.dev/reference/tools/fx/cmd/snapshot
[fxb-34556]: https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=34556
[fxb-36484]: https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=36484
[fxb-60532]: https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=60532
[fxb-67858]: https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=67858
[fxb-82514]: https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=82514
[fxb-82740]: https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=82740
[fxb-84117]: https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=84117
[glossary.product]: /docs/glossary/README.md#product
[glossary.product-owner]: /docs/glossary/README.md#product-owner
[hub]: /docs/concepts/components/v2/hub.md
[idk]: /docs/development/idk/README.md
[inspect]: /docs/development/diagnostics/inspect/README.md
[kernel-objects]: /docs/reference/kernel_objects/objects.md
[l10n-packaging]: /docs/development/internationalization/localization/packaging.md
[logs]: /docs/reference/diagnostics/logs/README.md
[namespaces]: /docs/concepts/process/namespaces.md
[oot-component-testing]: /docs/contribute/roadmap/2021/oot_component_testing.md
[oot-system-testing]: /docs/contribute/roadmap/2021/oot_system_testing.md
[open-life]: /docs/concepts/filesystems/life_of_an_open.md#fidl
[package]: /docs/concepts/packages/package.md
[package-url]: /docs/concepts/packages/package_url.md
[procargs]: /docs/concepts/process/program_loading.md#the_processargs_protocol
[process]: /docs/reference/kernel_objects/process.md
[process-launcher]: /docs/concepts/process/process_creation.md#fuchsiaprocesslauncher
[process-launcher-fidl]: https://fuchsia.dev/reference/fidl/fuchsia.process#Launcher
[program-loading]: /docs/concepts/process/program_loading.md
[rfc-0002]: /docs/contribute/governance/rfcs/0002_platform_versioning.md
[rfc-0021]: /docs/contribute/governance/rfcs/0021_soft_transitions_methods_add_remove.md
[rfc-0033]: /docs/contribute/governance/rfcs/0033_handling_unknown_fields_strictness.md
[rfc-0048]: /docs/contribute/governance/rfcs/0048_explicit_union_ordinals.md
[rfc-0061]: /docs/contribute/governance/rfcs/0061_extensible_unions.md
[rfc-0076]: /docs/contribute/governance/rfcs/0076_fidl_api_summaries.md
[rfc-0083]: /docs/contribute/governance/rfcs/0083_fidl_versioning.md
[rfc-0097]: /docs/contribute/governance/rfcs/0097_fidl_toolchain.md
[roadmap-netstack3]: /docs/contribute/roadmap/2021/netstack3.md
[sandbox]: /docs/concepts/components/v2/introduction.md#what_is_sandboxing
[sdk-tools]: /docs/reference/tools/sdk/README.md
[selectors]: /docs/reference/diagnostics/selectors.md
[sl4a]: https://android.googlesource.com/platform/external/sl4a/
[sl4f]: /docs/development/drivers/concepts/driver_development/sl4f.md
[stable-driver-runtime]: /docs/contribute/roadmap/2021/stable_driver_runtime.md
[structured-config]: /docs/contribute/roadmap/2021/structured_configuration.md
[syscall-life]: /docs/concepts/kernel/life_of_a_syscall.md
[syscalls]: /docs/concepts/kernel/concepts.md#system_calls
[test-double]: /docs/contribute/testing/principles.md#test_doubles_stubs_mocks_fakes
[topology]: /docs/concepts/components/v2/topology.md
[tracing]: /docs/concepts/kernel/tracing-system.md
[trf]: /docs/development/testing/components/test_runner_framework.md
[vdso]: /docs/concepts/kernel/vdso.md
[vdso-enforcement]: /docs/concepts/kernel/vdso.md#enforcement
[vmar]: /docs/reference/kernel_objects/vm_address_region.md
[workstation-oot]: /docs/contribute/roadmap/2021/workstation_out_of_tree.md
[zx-object-get-info]: /docs/reference/syscalls/object_get_info.md
[zx-process-create]: /docs/reference/syscalls/process_create.md
