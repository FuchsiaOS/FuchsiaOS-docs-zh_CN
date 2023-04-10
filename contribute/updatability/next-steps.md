# Platform updatability next steps

Fuchsia is [designed to update](how.md). However it's also a work in progress.
This section highlights some of the work that is ongoing or hasn’t started yet
that will promote Fuchsia’s ability to change and update.

## Versioning and compatibility metadata throughout the surface

- [FIDL availability annotations][rfc-0083] are
  [not yet implemented][fxb-67858]. Once this is implemented, the next step is
  to apply these annotations to FIDL files.
- Availability annotations are
  [not yet implemented for C/C++ headers][fxb-60532] in the Fuchsia IDK. Once
  this is implemented, the next step is to apply these annotations to SDK header
  files.
- FIDL API summaries are useful for detecting API-breaking changes. Currently we
  have [no equivalent for C/C++ APIs][fxb-82514]. Potentially-breaking changes
  are detected by comparing content hashes to a known reference, which produces
  false detections. For instance reformatting a header has no bearing on APIs or
  ABIs, but will falsely trigger the change detector.
- Support for vDSO backward compatibility. For instance if a component is known
  to target an older ABI revision then Fuchsia could load a vDSO shim into its
  address space that exports the old ABI and calls into the most recent vDSO
  ABI.
- There is no mechanism yet for
  [indicating what Fuchsia SDK version was used to build a given package or component][fxb-36484].
  Introducing such a mechanism and using it to annotate prebuilt packages is
  useful for instance to detect incompatibility.

## Complete Fuchsia ABI migrations to FIDL

While Fuchsia ABIs are largely defined in FIDL, there are still some legacy ABIs
that are defined in other terms that don’t have the same affordances for
updatability.

- While syscalls are defined in FIDL, some of their input and output types are
  still defined as C structures in Zircon headers. For instance
  [`zx_object_get_info`][zx-object-get-info] is defined in FIDL, but its output
  type (the `buffer` parameter) is a byte buffer that’s opaque to the FIDL
  definition and is formatted in terms of a `zx_info_*_t` C struct.
- [The processargs protocol][procargs] is used to bootstrap newly-created
  processes with startup capabilities. The format of the bootstrap message is
  defined as C structs, and should be [converted to FIDL][fxb-34556].

## Complete the Fuchsia SDK

The Fuchsia IDK and SDKs built on top of it can be used to develop Fuchsia
components without checking out Fuchsia’s source code or using Fuchsia’s own
build system. However there remain developer use cases that are not yet possible
out-of-tree.

- Products cannot yet be
  [assembled out-of-tree][decentralized-product-integration]. Specifically, some
  platform components can only receive configuration values in-tree, while the
  [scalable configuration solution][structured-config] is still in the design
  phase. Once out-of-tree product assembly is possible, this can be demonstrated
  for instance by
  [building and assembling the Workstation reference product out-of-tree][workstation-oot].
- Support for [out-of-tree component testing][oot-component-testing] and
  [out-of-tree system testing][oot-system-testing] is not yet complete. Some
  test runtime features are only available in-tree, and the
  [Test Runner Framework][trf] currently cannot be extended to support new
  testing frameworks out-of-tree. There does not yet exist a Fuchsia system
  automation framework for writing system tests behind the SDK.
- Drivers can’t yet be developed against a
  [stable driver runtime][stable-driver-runtime]. There is an
  [in-tree Driver Development Kit (DDK)][driver-development], but out-of-tree
  driver development is not yet supported. There is ongoing work towards
  demonstrating the first out-of-tree Fuchsia driver.

## Finish the Component Framework migration

The [new Component Framework][cf-intro] (aka CFv2) has full control over the
sandbox of component instances, and expresses sandboxes in terms of
[capabilities] that decouple contracts from implementation details.

The legacy Component Framework (`appmgr`) supported sandbox features that
allowed access to certain global namespaces and would expose components to
platform implementation details that had no affordances for updatability such
as versioning or transition support. The new Component Framework either enforces
[isolation][principles-secure] or forbids access to these namespaces entirely.
For instance the [hub] becomes hierarchically contained, only offering
information about the component’s realm and its children but never of its parent,
since a global hub is a form of [ambient authority][principles-secure].

The [components v2 migration][cfv2-migration] is an ongoing multi-year effort.
Currently the team is focused on
[migrating system components][cfv2-sys-migration].

## Deprecate unlisted platform ABIs

The fullness of the platform surface should be strictly defined, and expressed
in terms such as FIDL that afford for updatability via such mechanisms as
versioning and support for transitions. Currently there exist some aspects of
the platform surface that don’t meet these requirements.

- [Retrieving build information][build-info] uses a modern FIDL protocol.
  [Previously][build-info-old] this was done using a now-deprecated `appmgr`
  sandbox feature, which offered this information to components using raw file
  access that did not afford for updatability. This is now deprecated, new usage
  is [discouraged][cfv2-sys-migration-build-info], but there are still a few
  allowlisted legacy usages remaining.
- Some out-of-tree component testing frameworks launch
  [test doubles][test-double] for platform components by specifying their
  [`fuchsia-pkg://` launch URLs][package-url]. These URLs don’t have updatability
  affordances. Instead out-of-tree components find themselves exposed to
  platform implementation details such as the names of specific packages
  containing specific components that implement certain platform capabilities.
  These tests often break between Fuchsia platform and SDK revisions, even
  though no API or ABI breaking change is registered in the SDK or platform
  surface. Beyond deprecating existing usages, we should
  [take steps to prevent these issues from reoccurring][fxb-84117].
- [Scripting Layer for Fuchsia (SL4F)][sl4f] is a system automation framework
  for testing. SL4F is driven by a host-side test using a JSON-RPC/HTTPS
  protocol that is implemented to the specification of a
  [pre-existing system][sl4a]. This decision accelerated porting a
  [large inventory of connectivity tests][acts]. However the JSON-RPC/HTTPS
  protocol doesn’t have the same affordances for updatability as those found in
  FIDL and that benefit `ffx` plugins, nor does it have a schema. Therefore
  moving forward we should not support SL4F for system automation by out-of-tree
  tests, and introduce an alternative solution.

## Formalize diagnostics contracts

Fuchsia supports multiple diagnostics tools for understanding the system’s
internal state and for troubleshooting problems. Internal diagnostics exposes
implementation details by its nature, surfacing such information as process
names and hierarchies.

This information is useful for instance when troubleshooting a defective system
or when collecting a [snapshot][fx-snapshot] such as after a crash. In such
instances, internal implementation details are of interest. However, diagnostics
don’t make for good public contracts.

- Runtime diagnostics information, such as a particular component’s [log][logs]
  or an [Inspect] property, can be read at runtime using the
  [`fuchsia.diagnostics.ArchiveAccessor` capability][archiveaccessor], as
  specified by a [selector][selectors]. The selector consists of a component
  [moniker][monikers], a diagnostics hierarchy path selector, and a property
  selector. Monikers may expose platform implementation details such as the
  [topology] and names of platform components. Hierarchy and property selectors
  may also be considered implementation details, and in addition don’t have
  updatability mechanisms. These are known instances of out-of-tree components in
  Fuchsia-based products that use diagnostics selectors to read system
  diagnostics information at runtime. These components are exposed to platform
  implementation details and often break when these details change. Clients of
  Fuchsia diagnostics that are outside of the platform itself should be ported
  to using strictly-defined FIDL protocols to read precisely the information
  that they need, and the open-ended `ArchiveAccessor` capability should be
  restricted for further use by out-of-tree components.
- Components generate diagnostics information in different ways, such as an
  internal hierarchy of [Inspect] properties or as unstructured text in logs.
  Most platform components that do this don’t promise a particular schema for
  this information. Even Inspect which has structure and types doesn’t have all
  the updatability affordances found in FIDL for instance. Therefore processing
  platform diagnostics offline, such as in a tool that’s not provided by the SDK
  or in a product-specific dashboard, is bound to break.
- Performance tools such as [tracing], [CPU performance monitoring][cpu-trace],
  ,and [`mem`][fx-mem] collect and expose performance information in such terms
  as the names of processes and their interrelationships. This information is
  useful to investigate the performance of some systems, but it reflects private
  implementation details, not public contracts.

## Deprecate legacy developer tools

Several [SDK tools][sdk-tools] are offered to Fuchsia developers, most
importantly [`ffx`][ffx] and its many
[commands][ffx-reference]. The new
`ffx` tool interacts between the host and a Fuchsia target in terms defined in
FIDL, which affords for updatability. However some legacy tools are still offered
to out-of-tree developers that don’t have the same updatability affordances.

- SSH is supported (such as with the [`fssh` tool][fssh] and provides the host
  with a developer experience similar to a remote root shell on a target Fuchsia
  device. The client may circumvent the intended platform surface such as by
  directly observing, starting, and killing system processes.
- SCP (file copy over SSH) is similarly supported, and offers unrestricted
  access to global namespaces and to mutable filesystems, again circumventing
  the intended platform surface.
- Certain developer features ([example][fxb-82740]) are exposed to developers as
  legacy shell components rather than as `ffx` commands. This exposes developers
  to platform implementation details that can’t easily change such as the names
  of packages and components, and expresses inputs and outputs as human-readable
  text rather than typed and structured data.
- Some `ffx` commands, for instance [`ffx component`][ffx-component], expose
  internal implementation details such as component monikers and topologies.
  These are meant for diagnostics and troubleshooting, not as an API.

## Compatibility testing

API and ABI compatibility can be checked using continuous integration with
static analysis tools and build systems. In addition, the Fuchsia
[Compatibility Test Suite (CTS)][cts] can test different supported combinations
of platform and SDK versions. These tests can extend the notion of compatibility
from APIs and ABIs to also ensure that expected behaviors that are important are
preserved.

The CTS project is new and coverage is fairly low. CTS is a form of defense in
depth, so increasing coverage helps ensure that compatibility is as intended,
even if CTS coverage never reaches 100% of the platform surface.

## Further reading

- [Platform updatability best practices](best-practices.md)

[acts]: https://android.googlesource.com/platform/tools/test/connectivity/+/HEAD/acts
[archiveaccessor]: https://fuchsia.dev/reference/fidl/fuchsia.diagnostics#ArchiveAccessor
[build-info]: /docs/development/build/build_information.md
[build-info-old]: https://cs.opensource.google/fuchsia/fuchsia/+/1b21e5d7b36df3f5dde647684dd321f1aee21372:docs/development/build/build_information.md
[capabilities]: /docs/concepts/components/v2/capabilities/README.md
[cf-intro]: /docs/concepts/components/v2/introduction.md
[cfv2-migration]: /docs/contribute/open_projects/components/migration.md
[cfv2-sys-migration]: /docs/development/components/v2/migration/README.md
[cfv2-sys-migration-build-info]: /docs/development/components/v2/migration/features.md#build-info
[cpu-trace]: /docs/development/tracing/advanced/recording-a-cpu-performance-trace.md
[cts]: /docs/development/testing/ctf/overview.md
[decentralized-product-integration]: /docs/contribute/roadmap/2021/decentralized_product_integration.md
[driver-development]: /docs/development/drivers/developer_guide/driver-development.md
[ffx]: /docs/development/tools/ffx/overview.md
[ffx-reference]: https://fuchsia.dev/reference/tools/sdk/ffx.md
[ffx-component]: https://fuchsia.dev/reference/tools/sdk/ffx.md#component
[fssh]: https://fuchsia.dev/reference/tools/sdk/fssh.md
[fx-mem]: https://fuchsia.dev/reference/tools/fx/cmd/mem
[fx-snapshot]: https://fuchsia.dev/reference/tools/fx/cmd/snapshot
[fxb-34556]: https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=34556
[fxb-36484]: https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=36484
[fxb-60532]: https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=60532
[fxb-67858]: https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=67858
[fxb-82514]: https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=82514
[fxb-82740]: https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=82740
[fxb-84117]: https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=84117
[hub]: /docs/concepts/components/v2/hub.md
[inspect]: /docs/development/diagnostics/inspect/README.md
[logs]: /docs/reference/diagnostics/logs/README.md
[monikers]: /docs/concepts/components/v2/identifiers.md#monikers
[oot-component-testing]: /docs/contribute/roadmap/2021/oot_component_testing.md
[oot-system-testing]: /docs/contribute/roadmap/2021/oot_system_testing.md
[package-url]: /docs/concepts/packages/package_url.md
[procargs]: /docs/concepts/process/program_loading.md#the_processargs_protocol
[rfc-0083]: /docs/contribute/governance/rfcs/0083_fidl_versioning.md
[sdk-tools]: https://fuchsia.dev/reference/tools/sdk/README.md
[selectors]: /docs/reference/diagnostics/selectors.md
[sl4a]: https://android.googlesource.com/platform/external/sl4a/
[sl4f]: /docs/development/drivers/concepts/driver_development/sl4f.md
[stable-driver-runtime]: /docs/contribute/roadmap/2021/stable_driver_runtime.md
[structured-config]: /docs/contribute/roadmap/2021/structured_configuration.md
[test-double]: /docs/contribute/testing/principles.md#test_doubles_stubs_mocks_fakes
[topology]: /docs/concepts/components/v2/topology.md
[tracing]: /docs/concepts/kernel/tracing-system.md
[trf]: /docs/development/testing/components/test_runner_framework.md
[principles-secure]: /docs/concepts/principles/secure.md
[workstation-oot]: /docs/contribute/roadmap/2021/workstation_out_of_tree.md
[zx-object-get-info]: /docs/reference/syscalls/object_get_info.md
