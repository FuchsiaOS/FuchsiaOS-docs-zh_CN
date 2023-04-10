<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0119" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Summary

1. Enact a policy preferring relative paths or source-absolute paths over
   system-absolute paths in several enumerated instances and with an eye towards
   future use cases.
1. Move to enforce this policy in automated ways, establishing a regression
   stop.
1. Clean up pre-existing usages of system-absolute paths.

## Background

Readers who are familiar with the subject may wish to [skip ahead](#design).

### Paths

### Definitions

Readers should familiarize themselves with the concepts of
[paths][path-wikipedia]{:.external}.

Below we use the following definitions:

* System-absolute path: a path rooted at a local filesystem. Often denoted with
  a `/` prefix.
* Source-absolute or project-absolute path: a path relative to the root of a
  source tree or a project checkout. Often denoted with a `//` prefix.
* CWD-relative path: a path relative to the current working directory.

Below we generalize by referring to system-absolute paths as absolute paths, and
to other paths as relative paths, since they are expressed in relative terms to
another path.

### Paths in the Fuchsia build system

Paths are used in a build system to refer to input and output files of actions.
The [Fuchsia build system][fuchsia-build] uses [GN][gn] to define its build
graph. There is an [established best practice][gn-best-rebase-path] to prefer
expressing paths in GN as being relative to another root directory, such as the
[root build directory][gn-root-build-dir] or to the source root.

### Paths in generated code

Code may reference paths for a variety of reasons. Source code that is checked
in can't reference absolute paths because they are not portable - they won't
make sense on the Commit Queue (CQ) machines and will be rejected, or even if
they did then the same absolute paths won't make sense on another engineer's
machine when they check out the same sources. However, code that is generated on
a particular machine and isn't checked in may include absolute paths and still
work (successfully build and/or run).

Fuchsia utilizes many tools to generate source code. For instance [FIDL][fidl]
uses [`fidlc`][fidlc] and [Banjo][banjo] uses a similar tool.

## Motivation

There are several reasons to prefer relative paths to absolute paths in the
build system, in tools invocations, in generated source code, and in other
artifacts.

### Portable artifacts

As explained above, relative paths can be portable because they are relative to
a reference point that two parties can agree on, such as the root of a Fuchsia
source checkout or the root of a build outputs directory. In many situations
it's not only preferable for paths to be portable, it is a requirement.

#### Portable artifacts for distributed builds

Distributing build actions is the act of sending the inputs to a build action,
such as compiling C/C++ files into an object file, to a remote server for
execution. The remote server may perform the action on the client's behalf and
return the results. Often times the remote server will rely on a content-based
cache to skip the action entirely.

Distributing build actions has many benefits that are outside the scope of this
document.

Since the client and the server may not agree on absolute paths, relative paths
may be required when specifying the details of the invocation to distribute as
well as in the contents of any uploaded artifacts as they refer to each other.
This is particularly true for distributed build systems that rely on caching,
where the invocation details are used as part of the cache key. Even when
absolute paths are allowed, their use can defeat caching mechanisms, since two
clients could check out the same source code but then send requests to the
server that differ in absolute paths.

The presence of absolute paths in build paths or in generated code has caused
issues with distributed builds before. For instance Fuchsia developers may use
[Goma][goma]{:.external} to distribute C/C++ build actions. Fuchsia users of
Goma experienced outages before when changes were introduced that used absolute
paths through C/C++ include directories. In some instances distributed builds
would fail and force a local fallback, which was slower. In other instances
distributed builds would succeed, but fail to hit the cache, resulting in an
order of magnitude increase in backend load which led to cascading failures.

Another similar failure mode when distributing build actions is absolute paths
in tool invocations. We previously [found][fxr-545687] that it is useful to
check for such paths before distributing actions and to reject them in the form
of a build action failure and a helpful error.

In [other instances][fxb-75437-c4] absolute paths caused build correctness
issues.

#### Portable artifacts for pipelining

When distributing actions it is sometimes desirable to have a pipeline of remote
servers for different tasks. For instance some machines might be more suitable
to running a build and others more suitable to running a test that was built.
Sometimes actions are expressed as a graph that forks and joins, for instance
building a suite of tests and then forking to multiple machines each running a
shard of the tests, then joining the results.

Where in the previous case paths were exchanged between a client and a server,
in this case paths are exchanged between different servers in a pipeline. The
different nature of exchange notwithstanding, relative paths are preferred for
the same reasons.

Absolute paths can cause a breakage in the pipeline. For instance the pipeline
that produces [test coverage][test-coverage] broke in the past when coverage
reports that were generated in an earlier phase contained absolute paths to
source code which then failed to resolve in different servers that ran a later
stage of the coverage report production pipeline. The tool that produces
coverage mapping files used absolute paths, which we [changed][llvm-d87928]
to relativize paths to a given base.

Similar breakages occurred when working with another form of code
instrumentation - absolute paths were once used in debug info files, in
records that are used to resolve relative PC offsets to source code lines.

#### Portable artifacts for caching

Build outputs may be cached to accelerate subsequent builds, known as
incremental builds. Such caches are often kept locally, such as on a developer's
workstation or on a particular instance of a build server. In theory build
caches may also be exchanged between different build workers (workstations and
servers), provided that the network capacity affords for this and that there are
no security & privacy concerns.

Fuchsia currently does not reuse build caches between different machines because
it is known that some build outputs contain absolute artifacts. Instead, Fuchsia
developers and Fuchsia distributed builders will at most use their own localized
caches from previous builds that they ran. This is a significant lost
opportunity for optimization and increased engineering productivity.

### Reproducible artifacts

The use of absolute paths in artifacts prevents us from achieving [reproducible
builds][reproducible-builds]{:.external}.

Reproducible builds are not a stated goal for Fuchsia at this time. However it
is interesting to consider the benefits of reproducible builds, with the
understanding that this can be a desired property in the future and that the use
of absolute paths in artifacts would prevent us from achieving reproducibility.

Also note that there are other sources of artifact irreproducibility, most
commonly timestamps, that are outside the scope of this RFC.

#### Minimal work

Fuchsia currently runs tests on devices by producing a full system image and
[paving] the device. In the future we may want to accelerate this process, such
as by pushing to a test device only the blobs that had changed since the last
time it was updated. It is expected that most changes to be tested only affect a
small number of blobs relative to their base change, so this method of operation
would bring up test devices significantly faster.

If absolute paths leak into artifacts then many more blobs may be invalidated
between different changes to be tested than is absolutely needed.

### Out-of-tree Fuchsia builds

Above we describe some problems that Fuchsia has experienced due to absolute
paths, and some ways in which absolute paths make it difficult for Fuchsia to
evolve and improve. The urgency of solving the problems with absolute paths is
informed by historical context. For instance, historically Fuchsia did not
leverage incremental builds or caches, and therefore the project and the people
involved in it learned to tolerate deficiencies that kept Fuchsia from adopting
more incremental builds and caches.

If Fuchsia is successful then other projects will consume code and artifacts
from Fuchsia and develop for Fuchsia. It is safe to assume that at least some of
these projects will expect a system of build rules and tools that is more
friendly to different needs than those of the Fuchsia project. For instance some
of these customers may be operating at such as scale where [incremental builds
are a necessity][jmmv-no-clean]{:.external} and [so is
caching][jmmv-caching]{:.external}. If Fuchsia offers barriers to achieving
these properties then Fuchsia developers and other customers will face barriers
to adoption.

#### Distributed trust

If all artifacts of a build are reproducible then this opens the door to new
properties for a build system. For instance [reproducible
builds][reproducible-builds]{:.external} can act as a distributed alternative to
a cryptographic chain of trust for verifying the integrity of distributed
binaries. Untrusting parties can audit these binaries by simply attempting to
reproduce them from the same sources and build system. Failure to reproduce the
binaries may be evidence of malicious tampering.

If absolute paths are used in artifacts then untrusting parties will never be
able to reproduce identical results.

### Convenience

Relative paths are easier to use when troubleshooting. There is more often an
expectation of consistency, so one could for instance compare paths between a
successful operation and an unsuccessful operation and spot any meaningful
differences.

Absolute paths can be very convenient when working strictly locally. For
instance an absolute path can be copied from a tool invocation and used in a
different local shell environment and is guaranteed to work as it's not for
instance sensitive to the current working directory. Furthermore any two paths
that are absolute and normalized (for instance `.` and `..` parts are resolved,
links are followed, etc) can be checked for equivalence by string identity.
Since any path can be made absolute and normalized, and since such a
transformation is idempotent, this offers a simple equivalence check for paths
in a local environment. However there is nothing restricting users from
performing this transformation on absolute paths if they find it more convenient,
but since this transformation from relative to absolute and/or normalized form
is destructive it cannot be performed in the other direction. Therefore it is
more inclusive of all use cases to prefer the relative form.

## Design

### Policy

We will promote the [documented GN best practice][gn-best-rebase-path] to a
general policy, and apply it more broadly than just to `BUILD.gn` files.
Specifically we will recommend the following:

1. Paths that are passed to tools by the build system as command line arguments
   should be relative to the current working directory where the tool is invoked
   (in the case of GN/Ninja that's expressed as `root_build_dir`).
1. Paths in files generated at build time should be relative to the same root
   build directory. For instance: generated source code, package manifests,
   [depfiles][ninja-depfile].
1. Paths that are generated at runtime should be relative to the project source
   root. For instance: file info in crashes, test coverage reports.

### Enforcement

New tools will be introduced to sanitize the Fuchsia build against the presence
of absolute paths in tool invocations and in artifacts. These tools will be
exercised in CQ to prevent regressions.

### Cleanup

The tools above will have an affordance for an allowlist, which will be
initialized to list all existing violations of the policy. A cleanup effort will
be initiated to reduce the size of the allowlist to zero. Regressions will not
be admitted into the allowlist under normal circumstances.

## Implementation

The implementation details of how the enforcement tools will operate don't rise
to the level of an RFC. However some sketches for ideas are presented below for
the benefit of the curious reader.

### Sanitizing Ninja files

Running [GN][gn] produces a `build.ninja` file that describes a build graph. The
description of this graph includes all tools invocations, including paths to the
tools to invoke and paths that are passed to these tools as arguments.
Additional files used are referenced in a [depfile][ninja-depfile]{:.external}.

These files can be processed with `strings` to produce tokens that can then be
filtered for the appearance of being absolute paths. This simple scanner can be
implemented for instance as a host test that can run subject to all build
variants.

### Sanitizing files referenced by the build

In addition to sanitizing the Ninja files, we can also tokenize and sanitize any
files specified as inputs or outputs to build actions. We will be able to
discover all such files from the Ninja graph or the depfile, assuming that the
build is [hermetic][hermetic-actions].

### Check for presence of strings that are absolute paths

We could scan all files under the build output directory (`out/`), produce the
strings, and check if any are absolute paths, then emit an error. It's not a
fool-proof protection but rather an additional and simple line of defense.

### Rejecting absolute paths in the action tracer

We already have a [tool that wraps GN actions][action-tracer], and we already
use it to [reject absolute paths in depfiles][fxb-75451]. We could extend this
mechanism further.

Note that we currently only use the action tracer to wrap custom actions, which
make a subset of all build actions. We do this over the same performance
concerns that are explained above. From this perspective, perhaps leaning
further on action tracing is not a robust comprehensive strategy.

### Invalidating absolute paths

A simple approach to keep all relative paths within the checkout in working
order while completely invalidating absolute paths is to generate Ninja, then
move the checkout directory to somewhere else (or simply rename it), then build.

```shell
$ fx gen
$ mv $FUCHSIA_DIR ${FUCHSIA_DIR}_renamed
$ fx build
```

If any build invocations reference paths under the checkout as absolutes then
the build will fail.

This approach is very simple to implement, is portable, and bears no performance
overhead. Some downsides include that the error messages in case of breakage
will be confusing to the uninitiated, and that it's still possible to leak
absolute paths in generated artifacts (for instance depfiles, srcgen, debug
info).

### Changes to the runtime environment

Another approach would be to change the runtime environment of the build in such
a way that absolute paths are either rendered invalid or rendered harmless. For
instance [some projects][arch-linux-chroot]{:.external} use concepts such as
[`chroot`][chroot]{:.external} to form a sandbox at the checkout root. [Other
build systems][bazel-sandboxfs]{:.external} use [special
filesystems][fuse]{:.external} to achieve sandboxing.

Runtime approaches are worth considering as they can create stronger correctness
guarantees. However there are [performance
concerns][sandboxfs-performance]{:.external} and challenging
[portability][jmmv-osxfuse]{:.external} [issues][jmmv-execs]{:.external} to
consider.

## Security considerations

Paths may be used in such places where the interpretation of the path (what
actual file it's resolved to) may affect sensitive system behavior. For
instance, an allowlist of files that may be mapped into memory as executable
pages.

In such cases it is better to use project-relative paths than for instance paths
that are relative to the directory containing the list where they're specified,
or paths that are relative to the CWD of a tool that processes this list. This
is because the resolution of project-relative paths is unambiguous within the
project where they are defined, whereas other forms of relative paths may be
interpreted differently based on global mutable state (such as the CWD).

## Privacy considerations

Absolute paths can occasionally leak personally-identifying information. For
instance a person's username is often found as a part of an absolute path
containing files in that person's checkout or build output directory. Replacing
absolute paths with source-relative paths eliminates this outlet for PII.

## Testing

Above we explored some ways to implement checks that catch the use of absolute
paths. Since these checks are implemented as build steps or as host tests, they
can run in CQ and act as continuous tests.

Another way to ensure that absolute paths are not used is to make their presence
intolerable to a critical aspect of the engineering workflow. For instance if
absolute paths break a certain action that is distributed, and that action is
part of CQ, then developers can no longer introduce breakages.

## Documentation

When tools that enforce that paths are not absolute fail, they should produce an
error that links to an appropriate troubleshooting page. As inspiration, when
the [action tracer][action-tracer] which enforces that build actions are
hermetic fails it produces an error message with a link to the [hermetic build
actions][hermetic-actions] page.

## Drawbacks, alternatives, and unknowns

We can do nothing, at a loss of opportunity in the distributed build space and
the reproducibility space.

We can enact a policy but not move to enforce it. The likely consequence will be
not being able to make meaningful progress on distributed builds or on
reproducibility.

We can punt on this issue, at the cost of taking in additional regressions over
time, as is the nature of entropic decay.

## Prior art and references

A great Jedi once said: "Only a Sith deals in absolutes."

[action-tracer]: /docs/contribute/open_projects/build/hermetic_actions.md#reproducing_the_issue
[arch-linux-chroot]: https://wiki.archlinux.org/title/DeveloperWiki:Building_in_a_clean_chroot
[banjo]: /docs/development/drivers/tutorials/banjo-tutorial.md
[bazel-sandboxfs]: https://blog.bazel.build/2017/08/25/introducing-sandboxfs.html
[chroot]: https://en.wikipedia.org/wiki/Chroot
[fidl]: /docs/development/languages/fidl/README.md
[fidlc]: /docs/development/languages/fidl/guides/cli.md
[fuchsia-build]: /docs/development/build/build_system/fuchsia_build_system_overview.md
[fuse]: https://github.com/bazelbuild/sandboxfs
[fxb-75437-c4]: https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=75437#c4
[fxb-75451]: https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=75451
[fxr-545687]: https://fuchsia-review.googlesource.com/c/fuchsia/+/545687
[gn]: https://gn.googlesource.com/gn/
[gn-best-rebase-path]: /docs/development/build/build_system/best_practices_templates.md#prefer-relative-paths-from-rebase-path
[gn-dev-thread]: https://groups.google.com/a/chromium.org/g/gn-dev/c/WOFiYgcGgjw/m/89f3H7nnAQAJ
[gn-rebase-path]: https://gn.googlesource.com/gn/+/refs/heads/main/docs/reference.md#func_rebase_path
[gn-root-build-dir]: https://gn.googlesource.com/gn/+/refs/heads/main/docs/reference.md#var_root_build_dir
[goma]: https://chromium.googlesource.com/infra/goma/server/
[hermetic-actions]: /docs/development/build/hermetic_actions.md
[jmmv-caching]: https://jmmv.dev/2021/02/google-monorepos-and-caching.html
[jmmv-no-clean]: https://jmmv.dev/2020/12/google-no-clean-builds.html
[jmmv-osxfuse]: https://jmmv.dev/2020/01/osxfuse-hardlinks-dladdr.html
[jmmv-execs]: https://jmmv.dev/2017/10/fighting-execs-sandboxfs-macos.html
[llvm-d87928]: https://reviews.llvm.org/D87928
[ninja-depfile]: https://ninja-build.org/manual.html#_depfile
[path-wikipedia]: https://en.wikipedia.org/wiki/Path_(computing)
[paving]: /docs/development/build/fx.md#what-is-paving
[reproducible-builds]: https://reproducible-builds.org
[sandboxfs-performance]: https://blog.bazel.build/2017/08/25/introducing-sandboxfs.html
[test-coverage]: /docs/contribute/testing/coverage.md
