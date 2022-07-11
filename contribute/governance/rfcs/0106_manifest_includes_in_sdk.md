<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0106" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

## Summary

This document proposes adding component manifest shards to the
[Integrator Development Kit (IDK)][idk]. This will create a standard affordance
for publishing component manifest shards to out-of-tree (OOT) developers and
establish a common pattern for making manifest shards portable between different
developer environments.

Readers are expected to be familiar with the following:

*   [Component manifests][component-manifests]
*   Particularly, the [`include` syntax][manifests-include] in component
    manifests.

## Motivation

### Aligning workflows across environments

Since their introduction, component manifest shards and the `include` syntax in
`cml` and `cmx` have seen [wide adoption][cs-shards]. They are used for a
variety of tasks such as [reducing boilerplate][elf-runner-streams],
[encapsulating implementation details][vulkan-runtime-deps],
[simplifying developer workflows][trf-inventory], and in
[core realm variations][rfc-0089].

Currently the mechanism for processing manifest includes relies on the source
layout of the Fuchsia tree. This works fine for manifests that live in the
Fuchsia tree, but doesn't port well to OOT development. Consider for instance
this guide for [logging in C/C++ components][cpp-logging]. The guide opens by
instructing developers to add the following to their component manifest:

```json5
{
  include: [ "sdk/lib/diagnostics/syslog/client.shard.cml" ],
}
```

However, the guide notes that the above only works in-tree (IT). Since OOT
developers are also in the target audience for this guide, the guide instructs
these developers to instead add the following to their component manifest,
effectively inlining the contents of the shard:

```json5
{
  use: [
    { protocol: "fuchsia.logger.LogSink" },
  ],
}
```

The same issue is present elsewhere. For instance the [Test Runner
Framework][trf-inventory] inventory of test runners (for supporting various
types of tests on Fuchsia, for instance C/C++ GoogleTest, Rust libtest, gotest
on Go etc') has instructions that only work IT. This hinders the proliferation
of good testing practices to OOT developers.

This compromise defeats the purpose of including a manifest shard in the first
place, as clients are exposed to an implementation detail of Fuchsia logging.
Furthermore it makes it more difficult to orchestrate large scale changes in the
future, such as a planned future soft transition to a different protocol for
publishing logs from components. Finally it adds friction between IT and OOT
developer workflows.

### Obscuring implementation details

If we revisit the `LogSink` shard example, we encourage clients of the syslog
library to include this shard as it brings into their component's sandbox the
capabilities needed to make that client library work.

Currently that set of capabilities is very simple, just a single protocol, but
it's expected to change in the future. For instance instead of the current
protocol, where clients connect a socket to syslog and then buffer characters
into that socket to log them, we may introduce a system for writing structured
data in a specialized format into VMOs and rotating them between the client and
the server, not unlike how tracing works today.

Obscuring these details behind the shard reduces the cognitive load on the
component author, who just wants to write to syslog (a common utility from their
perspective) and doesn't care about those details. But this also allows the
syslog maintainer to evolve those details over the time, without requiring the
attention of the component owner.

This also works in the opposite direction (exposing capabilities from a
component rather than consuming them) and with different types of capabilities
(for instance [directory capabilities][directory-capabilities] rather than
[protocol capabilities][protocol-capabilities]). Consider for instance this
[Inspect discovery and hosting][inspect-discovery-hosting] guide. The guide
explains that developers are required to add the following to their component
manifest:

```json5
{
    capabilities: [
        {
            directory: "diagnostics",
            rights: [ "connect" ],
            path: "/diagnostics",
        },
    ],
    expose: [
        {
            directory: "diagnostics",
            from: "self",
            to: "framework",
        },
    ],
}
```

Alternatively developers could simply include a shard. This saves the developer
from 14 lines of boilerplate, the obscurity of having to deal with concepts
such as exposing a directory capability to the framework (which the majority of
component developers don't need to be familiar with), and the trouble of having
to change this mechanism if needed in the future. Lastly, if shards could be
portable between IT and OOT component manifests then the inspect guide would be
considerably shorter as it wouldn't need to specify a separate set of
instructions for IT and OOT developers.

### Simplifying the consumption of system capabilities

Another example that's more sophisticated is the various capabilities that may
be needed when working with the web engine, directly as a FIDL client rather
than via a client library as in the previous example. The web runtime is
powerful and robust, with today's web apps being capable of performing many
privileged device operations given the required capabilities.
[`fuchsia.web`][fuchsia-web] defines the different capabilities that the web
engine implementation would need to be able to host certain web apps.

In order to simplify consuming those same capabilities from the environment and
then providing them to the web engine, we have shards that slice up these
capabilities by common categories. For instance this shard defines a baseline
set of "table stakes" capabilities that the web engine requires:

```json
{
    "sandbox": {
        "services": [
            "fuchsia.device.NameProvider",
            "fuchsia.fonts.Provider",
            "fuchsia.intl.PropertyProvider",
            "fuchsia.logger.LogSink",
            "fuchsia.memorypressure.Provider",
            "fuchsia.process.Launcher",
            "fuchsia.sysmem.Allocator",
        ]
    }
}
```

This shard defines additional capabilities that are needed if the web app
is not local and/or requires the network:

```json
{
    "sandbox": {
        "services": [
            "fuchsia.net.name.Lookup",
            "fuchsia.net.interfaces.State",
            "fuchsia.netstack.Netstack",
            "fuchsia.posix.socket.Provider"
        ]
    }
}
```

Whereas this shard is required in order to render the web app in a Scenic View:

```json
{
    "sandbox": {
        "services": [
            "fuchsia.accessibility.semantics.SemanticsManager",
            "fuchsia.ui.input.ImeService",
            "fuchsia.ui.input.ImeVisibilityService",
            "fuchsia.ui.scenic.Scenic"
        ]
    }
}
```

Additional shards exist for instance to unlock hardware acceleration for
graphics or for hardware media codecs.

This is a non-trivial amount of information. Keeping it in its own shard is
tidier than adding it to a pile of other services present in a component's
sandbox. In addition, this once again makes evolution easier.

Note that this is a hypothetical example. These shards exist IT today. This is
an exploration of the possibility of moving these shards to the IDK to make them
available OOT, not a promise to do so. Also, the content of the shards above is
still in flux, and is used here as an illustrative example but not as reference
documentation.

## Implementation

### Packaging manifest shards in the IDK distribution

C/C++ development targeting Fuchsia is made possible by setting up the include
path (as [specified to the compiler][clang-i-dir] via the `--include-directory`
or `-I` flag) to one or more directories that contain a particularly nested
hierarchy of subdirectories and header files. This makes code that includes
Fuchsia-specific headers portable between IT and OOT builds.

For instance the following line of C code is valid both IT and OOT:

```cpp
#include <lib/zx/process.h>
```

This is because both the IT build and OOT builds targeting Fuchsia add to their
include directories a path that has the file `lib/zx/process.h` below it. In a
Fuchsia checkout the corresponding include directory is
`//zircon/system/ulib/zx/include/`, whereas in an OOT build this path will need
to be `$FUCHSIA_SDK/pkg/zx/include/`. See also: [IDK layout][idk-layout].

Setting up directories in an include path to make include directives portable is
also known as setting up a "sysroot".

We will deploy component manifest shards similarly, below `$FUCHSIA_SDK/pkg/` in
sub-paths that are conceptually associated with the purpose of the shards, and
set up the `--includepath` flag in [`cmc`][cmc] accordingly in IT and OOT
builds.

For instance, the syslog shard used as an example above might be:

*   Included as follows: `include: [ "syslog/client.shard.cml" ]`.
*   Found IT under `//sdk/lib/syslog/client.shard.cml`, hence we would configure
    `cmc` IT with `--includepath $FUCHSIA_CHECKOUT/sdk/lib/`.
*   Found OOT under `$FUCHSIA_SDK/pkg/syslog/client.shard.cml`, hence we would
    configure `cmc` OOT with `--includepath $FUCHSIA_SDK_ROOT/pkg/`.

### Portable shards vs local shards

It's expected that some shards are to be made available to OOT developers, while
others are only to be used IT. Above we reviewed some examples for shards that
can be used OOT. An example for a shard that is only useful IT is
[this shard][test-manager-common-shard] which is used to share a large portion
of complex capability routing between two component definitions, one of which is
a system component and the other is a test double for that component. There is
no use for this specific shard OOT.

Therefore some shards should be made portable and published in the IDK while
others should remain private to a particular repository.

We propose codifying this distinction using a common notation for relative and
absolute paths. Paths used in manifest `include` directives that should resolve
to portable shards should have no leading `//`, for instance
`syslog/client.shard.cml`. These will be resolved against the sysroot of shards.
On the other hand, paths to shards that are purely local to a certain repository
should begin with `//` and resolve against the source root (or checkout root) of
their repository.
For instance [this shard][test-manager-common-shard] should be included IT via
the path `//src/sys/test_manager/meta/common.shard.cml`.

### Build system integration with `cmc`

Build systems, such as the [in-tree GN & Ninja build][fuchsia-build-system] and
any out-of-tree builds targeting Fuchsia, already integrate with `cmc`. Such
integrations will need to be amended to afford for the new include behavior.
Specifically for the following [`cmc` subcommands][fx-cmc]:

*   `compile`
*   `include`
*   `check-includes`

The invocations of `cmc` will need to specify the following flags:

*   `--includeroot`: path to resolve `//`-prefixed include paths against.
*   `--includepath`: zero or more paths to resolve other paths against, in the
    order specified (first match).

### Shards as SDK atoms

Shards will be included in the IDK by the build system, similar to how other IDK
elements are treated. We will reuse the existing `sdk_atom()` template,
specifying the `id` parameter according to how we'd like the
[IDK layout][idk-layout].

### Process for adding shards to the IDK

Shards can specify contractual expectations that may overlap with the platform
surface. For instance the syslog shard references a protocol in the Fuchsia
namespace - `fuchsia.logger.LogSink` - that is well known to be offered by a
Fuchsia system component. Therefore shards that are published in the IDK will be
treated as APIs and as SDK atoms, and will undergo [API review][api-review] via
the same process that's used today for instance to add or modify FIDL files that
are published in the IDK. It's also possible to use the `sdk_atom()` notion of a
category, for instance first introducing a shard as "internal" (not to be
distributed OOT) and then elevating it to a higher-exposure category via the
existing process.

## Future work

### Port `expect_includes()`

Fuchsia's GN build offers a template for expecting that dependent components
include a certain shard in their manifest (see
[this guide][components-build-includes]). One of the benefits that this offers
is that it directs developers that added a dependency on, for instance, a client
library for a particular service, to also include a manifest shard in their
component that ensures that their library has access to capabilities that it
requires at runtime to operate correctly.

IT developers have given very positive feedback for this, and some OOT
developers expressed interest. This template or the concept of it can be ported
to the GN SDK using GN metadata, as well as to the Bazel/Blaze SDK using Bazel
Aspects or Bazel Providers.

See also: [fxbug.dev/77007][fxb-77007]

## Performance

This proposal has no impact on runtime performance since all work is done at
build time.

Processing includes at build time adds some extra work. However this is expected
to have no impact on clean build wall time. We know from previous research that
work that the build does that's not on the critical path generally does not
contribute to build latency, and also that work that's derived directly from
sources (such as `cmc` invocations, `fidlc` invocations, etc') has virtually
zero presence on the critical path since this work parallelizes well and can be
scheduled very flexibly.

As an example, consider [this change][fxr-401013] where a common operation in
`cmc` was made about 300ms faster, but despite having thousands of invocations
of `cmc` in the build we measured no impact on build wall time.

## Ergonomics

Attention has been given to keeping the manifest includes mechanism simple and
transparent. For instance it is possible to produce a post-processed component
manifest, with include directives replaced with the contents of included files
(transitively if needed), with a simple command.

```posix-terminal
fx cmc include {{ "<var>" }}manifest{{ "</var>" }} --includepath $(fx get-src-dir)
```

Note: `fx cmc` is an IT shortcut for launching the `cmc` binary, possibly
building it from source. For OOT usage replace this invocation with the path to
the prebuilt `cmc`.

This is similar in principle to running the C preprocessor on C/C++ source code
(see: [`man cpp`][man-cpp]).

In addition, `cmc` generates simple errors that are easy to troubleshoot,
including for unusual error cases such as include cycles. All supported errors
are unit tested.

The `cmc` tool itself is already bundled as a prebuilt in the Fuchsia IDK. It is
fully hermetic and runs without any external dependencies. It can be invoked
from the command line, no build system integration is required.

## Backwards compatibility

Changing shards in the IDK will affect newly-built OOT components once they pick
up the latest IDK release, but won't affect old prebuilts. Care must be taken to
avoid breaking changes and use standard practices for platform changes: soft
transitions over multiple releases, support windows, and communicating with
stakeholders.

As with all matters of platform evolution, changes should be tested thoroughly,
and breaking changes should be managed in some way that affords for breakage,
for instance using some versioning mechanism. Note that questions of [platform
versioning][rfc-0002], and subsets of this problem such as [FIDL
versioning][rfc-0083], remain open at this time and are outside the scope of
this RFC.

## Security considerations

Component manifests define capability routing and sandboxes, which have direct
implications on security. However the goal of manifest includes is not to
ultimately produce a different manifest, but rather to produce the same manifest
in more ergonomic ways. As long as the same final manifest is produced after
includes are processed, there are no implications on security.

To help developers understand what their manifests would look like after
includes are processed they can use the `cmc include` demonstrated above.

## Testing

The existing functionality in `cmc` is already covered by unit and integration
tests. Test coverage for `cmc` is >90%, and specifically coverage for the
include subcommand is complete. Any outstanding changes needed to the Fuchsia
IDK or to OOT integrations will be tested as instructed and expected by the SDK
team.

Component manifests that are in the IDK should be treated as any other IDK
artifacts in terms of CTS test coverage.

## Documentation

The [`include` syntax][manifests-include] is already documented.

The `cmc include` command is self-documented via `cmc help include`.

Existing documentation that currently instructs OOT developers to use a
replacement for manifest includes will be updated to no longer make this
distinction between IT and OOT development once it's no longer necessary.

## Drawbacks, alternatives, and unknowns

### Do nothing

We can maintain the status quo, at the cost of not solving the problems stated
in the motivation section above.

### Organize shards in the IDK as top-level artifacts

As an alternative to publishing component manifest shards in various possible
locations in the IDK, such as under `$FUCHSIA_SDK_ROOT/pkg` or
`$FUCHSIA_SDK_ROOT/fidl` (based on the logical association of the shard), we
also explored the alternative of establishing a single base directory for
component manifest shards. For instance:
`$FUCHSIA_SDK_ROOT/component_manifests/`. This is similar to how all `.fidl`
files are organized under `$FUCHSIA_SDK_ROOT/fidl/`, that is to say they are
aggregated by type (being FIDL files) rather than by another logical association
(for instance `pkg/async/`, `pkg/memfs/`, `pkg/zx/`).

This alternative was rejected based on feedback from SDK customers, stating that
they prefer the logical association of the SDK's contents over having different
types of files be spread across different base directories. The accepted design
allows for instance to put the component manifest shard for using syslog next to
C++ headers for writing to syslog from components written in C++.

## Prior art and references

Component manifest includes are inspired by C/C++ includes.

The `cmc include` command is inspired by the `cpp` command, which runs the C
preprocessor on a given file and prints the postprocessed result. See:
[`man cpp`][man-cpp].

[api-review]: /contribute/governance/api_council.md#api_review
[clang-i-dir]: https://clang.llvm.org/docs/ClangCommandLineReference.html#cmdoption-clang-i-dir
[cmc]: https://fuchsia.dev/reference/tools/sdk/cmc.md
[component-manifests]: /concepts/components/v2/component_manifests.md
[components-build-includes]: /development/components/build.md#component-manifest-includes
[cpp-logging]: /development/languages/c-cpp/logging.md
[cs-shards]: https://cs.opensource.google/search?q=file:%5C.shard.cm&sq=&ss=fuchsia
[directory-capabilities]: /concepts/components/v2/capabilities/directory.md
[elf-runner-streams]: /concepts/components/v2/elf_runner.md#forwarding_stdout_and_stderr_streams
[fuchsia-build-system]: /development/build/build_system/fuchsia_build_system_overview.md
[fx-cmc]: https://fuchsia.dev/reference/tools/fx/cmd/cmc
[fxb-77007]: https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=77007
[fxr-401013]: https://fuchsia-review.googlesource.com/c/fuchsia/+/401013
[fuchsia-web]: https://fuchsia.dev/reference/fidl/fuchsia.web
[idk]: /development/idk/README.md
[idk-layout]: /development/idk/layout.md
[inspect-discovery-hosting]: /reference/diagnostics/inspect/tree.md
[man-cpp]: https://man7.org/linux/man-pages/man1/cpp.1.html
[manifests-include]: /concepts/components/v2/component_manifests.md#include
[protocol-capabilities]: /concepts/components/v2/capabilities/protocol.md
[rfc-0002]: /contribute/governance/rfcs/0002_platform_versioning.md
[rfc-0083]: /contribute/governance/rfcs/0083_fidl_versioning.md
[rfc-0089]: /contribute/governance/rfcs/0089_core_realm_variations.md
[test-manager-common-shard]: /src/sys/test_manager/meta/common.shard.cml
[trf-inventory]: /development/testing/components/test_runner_framework.md#inventory_of_test_runners
[vulkan-runtime-deps]: /development/graphics/magma/concepts/vulkan.md#runtime_dependencies
