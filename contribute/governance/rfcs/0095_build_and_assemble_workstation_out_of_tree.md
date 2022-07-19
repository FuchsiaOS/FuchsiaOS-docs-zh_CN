{% set rfcid = "RFC-0095" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }} - {{ rfc.title }}
<!-- *** DO NOT EDIT ABOVE THIS LINE -->


## Summary

The Workstation product, defined as the workstation_session, ermine, terminal
and simple browser, is currently built and assembled in the Fuchsia source tree.
There is no clear distinction between Fuchsia as a platform and the workstation
product. This requires that the platform and the product must be built at the
same time to assemble the final product. We propose that this coupling be broken to
allow the Workstation components to be built separately from the platform and
that the final product be assembled outside of the Fuchsia source tree. This
proposal does not address removing the implementation of the Workstation product from the
Fuchsia source tree but rather focuses on the dependent problem of out-of-tree
product configuration and assembly. This work will lay the groundwork needed to
eventually move the Workstation product out-of-tree completely but that is out
of the scope of this document.

It is important to call out that the terminal is considered to be a fundamental
component for the Workstation product, however, the terminal is used by other
products outside of the Workstation product. Therefore the development of the
terminal component will remain in fuchsia.git and it will be consumed by the
Workstation as part of the platform until a further decision is made regarding
the future of this component.

## Motivation

There is currently no way to build and assemble a product on Fuchsia without
building the entire Fuchsia platform at the same time. We can build individual
components outside of fuchsia.git using the SDK but those artifacts need to flow
back to Fuchsia Global Integration to be assembled into a final product. We
would like to be able to offer stable versions of the platform for products to
be built on without needing to have access to Fuchsia Global Integration.

There are still many unknowns when it comes to building products out-of-tree
which is why we want to start with the Workstation product. The Workstation
product is not a production-grade, supported user-facing product, it is a
reference for developers and an environment for early adoption of platform
features and testing, so there is a higher tolerance for slowdowns at this
stage. The Workstation product is also meant to be the environment in which
enthusiasts can explore Fuchsia; by removing the need to build the entire
platform we can make it easier for developers to play with Fuchsia.

## Design

Moving Workstation out-of-tree builds on top of the [Standalone Image Assembly
Tool RFC-0072](/contribute/governance/rfcs/0072_standalone_image_assembly_tool.md),
this proposal assumes that this tool exists.

### Repository

The source code for the Workstation repository will be hosted on
[git-on-borg](https://fuchsia.googlesource.com). Hosting the code on git-on-borg will allow
us to use existing Fuchsia infrastructure and tooling which will allow us to
focus on the task of building and assembling the product.

For the remainder of this document workstation.git will refer to the repository
hosting the code for the Workstation product.

### Infrastructure

The focus of this project is to build and assemble the Workstation product
out-of-tree and thus we will not be moving the development of the components out
of fuchsia.git until we have worked out how to do this. Since the component
development will remain in-tree, we can rely on the existing test and build
infrastructure to verify that the Workstation product does not regress. The
process of rolling new versions of the platform, sdk, experiences.git and other
dependencies will be done manually with a script to allow the team to focus on
the problem at hand. When we begin to transition the development of the
component out-of-tree we will need to revisit how we do continuous integration.

The daily builds of the platform will need to be automated but we can use the
existing Fuchsia infrastructure. A daily builder will need to be created which
builds the packages required to assemble the Workstation product and upload those
artifacts to some storage repository. The technical design for this workflow still
needs to be figured out. We will consult with infrastructure and security before
we start working on this.

Building the Workstation product, as opposed to core, will ensure that we have
all the artifacts that we need for product assembly even if we do not build them
at this point. The artifacts that we do build out-of-tree will be used in place
of the artifacts that rolled with the platform. Doing this will also allow us to
use the existing Workstation builder which will minimize the amount of extra
infrastructure we will need to manage.

### Dependency Management

The Workstation repository will use a combination of git submodules and Bazel
workspaces for dependency management. Git submodules will be used to pull in external
repositories whereas Bazel will be used to download prebuilts and toolchains
required by the build.

Bazel's [toolchains](https://docs.bazel.build/versions/master/toolchains.html) will
be used in conjunction with workspace rules to download the appropriate prebuilts
that are required for their respective build rules. One limitation at this point is
that many of our prebuilts are stored in [CIPD](https://chrome-infra-packages.appspot.com/)
and require the cipd command line tool. Initial prototypes require that the cipd
tool be somewhere on the developer's path. We are actively looking into alternaives
and do not think this will be a long term requirement.

### Directory Structure

The directory structure for the Workstation repository will closely resemble the
fuchsia.git directory structure to keep consistency for Fuchsia developers. The
root directory will contain various metadata files needed for the project as
well as the following top-level directories.

- //src - Source code used to build the Workstation product.
- //tools - scripts and tooling to support the build.
- //src/experiences - the existing [experiences.git](https://fuchsia.googlesource.com/experiences) repository.
- //prebuilt - any prebuilts that are needed.
- //third_party - third party code that is used by the src directories.

### Build System

The Workstation product will be built with the [Bazel](https://bazel.build)
build system. The decision to use Bazel diverges from fuchsia.git which uses
gn/ninja. This decision means that we will not be able to lean on the expertise
that the Fuchsia team has acquired or reuse the existing build rules. However,
the current gn SDK does not contain any logic for building or testing Flutter
applications which is the majority of the Workstation product. The team would
need to write these rules no matter what build system they choose. The sample
Bazel SDK that exists in-tree does have some rudimentary logic for building
Dart and Flutter applications that we can build off of.

The Workstation team has chosen to use Bazel over gn/ninja for a few main reasons
that they feel makes this a more attractive build system to use.

- open source adoption: Bazel is not the most popular build system but it does
have more adoption than gn/ninja and thus should feel more familiar to the
open source community.

- ecosystem of rules: Bazel has a rich ecosystem of existing rules which help to
promote reuse.

- dependency management: Bazel can manage external dependencies.

- Fuchsia SDK Bazel SDK productionization: Using Bazel for the Workstation product
gives us the opportunity to push changes into a Fuchsia Bazel SDK which can be
used by other projects.

- single build/analysis phase: A common complaint for developers coming to Fuchsia
is that they do not know why a package or tool is not included in their build. This
often comes down to a user not including the target in their gn args. Bazel removes
this failure mode by requiring the user to explicitly specify what they want to
build, run or test.

- hermetic builds: Bazel has hermetic builds by default whereas gn does not.

During this phase we will need to write build rules to build the experiences
repository against the workstation.git buildroot. We will utilize the
[new_local_repository](https://docs.bazel.build/versions/master/be/workspace.html#new_local_repository) feature of Bazel. Using this rule will allow us to
keep the Bazel specific build rules in workstation.git to build against
sources mapped in from the experiences repository. It is important to note that
this setup is prone to build breakages because the Bazel build rules are hosted
in a separate repository from the actual source code. These breakages will only
affect the team that is working on getting out-of-tree assembly and not any
in-tree developers. We are aware of the trade-off here but feel it is better to
disrupt the workflow of the out-of-tree developers instead of the in-tree
developers. The out-of-tree developers will be responsible for fixing the
breakages as they occur.

//WORKSPACE

```
new_local_repository(
    name = "ermine",
    path = "vendor/experiences/session_shells/ermine/shell/",
    build_file = "src/ermine/BUILD.ermine",
)
```

//src/ermine/BUILD.ermine

```
flutter_component(name = "ermine_component", ...)
flutter_package(name = "ermine", deps = ":ermine_component")
```

### Language Support

The Workstation components are currently written in Dart and Rust of which Dart
is the only language that is currently supported for out-of-tree development.
The workstation_session and terminal components are written in Rust and Ermine
and simple-browser are written in dart. We will start by focusing our attention
on building the Dart components while continuing to build the Rust components
in-tree and delivering them as part of the platform.

There are currently two blockers for building the workstation_session
out-of-tree. The first is that the workstation_session is using several private
APIs which means that it could not be built out-of-tree even if we had the
language support. The second blocker is the lack of out-of-tree Rust support.
The workstation_session is currently being broken up into smaller platform level
components which is eliminating its dependency on private interfaces and
out-of-tree Rust support is being explored. We will make a decision about the
fate of the workstation_session as these two projects progress. If the
workstation_session is completely migrated off of private interfaces but Rust
support is not going to come soon after then we will rewrite the session in C++
whereas if Rust support lands before or soon after then we will keep the session
written in Rust.

### Sanitizers

The fuchsia.git tree supports various sanitizers for both local development and
in the infrastructure builders. Support for these system in the workstation.git
repository is not an immediate goal since we will not be moving source code out
of fuchsia.git until we have proven that the build and assembly system works with
the mapped in repository. However, we should block on moving code out of fuchsia.git
until we have sanitizer support since it introduces a regression from the current
setup.

When we get local builds working properly we will add support for passing build
time flags to enable the various sanitizers locally. This will allow us to build
against the mapped in sources with sanitizers enabled. When we start moving code
out of fuchsia.git we will set up builders to match the current experience of
committing to fuchsia.git.

### Code Search

Fuchsia code can currently be searched on [code search](https://cs.opensource.google/fuchsia).
The workstation.git repository will be added to this same project as a separate
project which will allow it to be searchable and viewable online.

### Code Coverage

We do not plan to enable code coverage for workstation.git at the beginning of
the project and do not have any plans for the near term. The main reason for this
is that the majority of the Workstation code is written in Dart which does not
currently have sufficient enough code coverage support to warrant us investing
the time needed to enable it.

We will be losing parity with the current experience for writing c++ code since
it does show incremental code coverage but the scope of work to enable this in
workstation.git is too large to take on at this point. However, it does present
a good opportunity to work with the Fuchsia build team to make these tools more
readily available for out-of-tree developers.

### Third Party Support

All of the source code written in workstation.git will need to share the same
third party libraries. The tooling to support this has been solved many times
over in other large repositories so we will not have to start from scratch but
it does represent a quantity of work that we will have to do and support for the
entirety of the project.

Licensing compliance will match what is used by the Fuchsia Project. Compliance
for third_party repositories will be monitored by the top-level OWNERS of the
repository as part of the code review process which brings these dependencies in.

We currently do not have any solid plans for checking licenses and generating
NOTICE files during builds and during code checks. We will need to study how
this is done in the fuchsia.git repository and port that logic to this repository.

### Binary Size Monitoring

Monitoring the size of our binaries is important for the long term health of our
project. There exist tooling in the Fuchsia build system which monitors the size
of binaries that are produced during each commit. Building these systems again
for the Workstation product is out of scope for the early phases of this project
but it is something we will need to add eventually.

We need to investigate how tightly coupled these systems are with the current
fuchsia.git build system to see if we can simply integrate with them in their
current forms or if we will need to do work to decouple them. If we need to work
to decouple the tooling from the build system we should focus on making these
tools reusable to scale out to other product integrators in the future.

### Performance Testing

There is currently a project taking place to add performance monitoring to the
Workstation product. This project is already proceeding with a focus on making
the monitoring available to out-of-tree users by only using tooling that is
available in the public SDK. We plan to move these tests into the workstation.git
repository as the code moves out of fuchsia.git.

### OWNERS

The initial set of contributors to the workstation.git repository will be small
compared to the set of contributors to fuchsia.git. For this reason, enforcement of
OWNERS files will not be a priority during the initial phases of this project but
we will continue to add them to the appropriate directories to facilitate communication.
As we start to move code out of fuchsia.git we will introduce OWNERS files and
enforce the same rules for code reviews that we do in fuchsia.git.

Contributions to the workstation.git repository will be governed by the same
policies that govern contributions to fuchsia.git since they will be hosted under
the same project.

### Bug Tracking

The repository will use the existing [Fuchsia Monorail](https://bugs.fuchsia.dev/p/fuchsia)
bug tracker to track bugs. The reason for using Monorail is because it is already
set up for tracking Fuchsia bugs and because the board support for the Workstation
product will still remain in fuchsia.git. There is a top-level Workstation component
which can be used for general triage and then there are several existing components
for tracking the various features of the Workstation product.

### Flow of Artifacts

In order to assemble the Workstation product outside of fuchsia.git we will need
to provide prebuilts of the Fuchsia platform to be consumed by the Workstation
product. This implies that we will have two classes of artifacts, the artifacts
that comprise the Platform and then the artifacts that are layered on top to
assemble the final Workstation product image. The Platform images will be built
ahead of time in to be consumed by the workstation.git repository whereas the
Workstation components will be built at the time that the final product image
will be assembled. The set of artifacts that make up the Platform is not clearly
defined at this point as the Platform is constantly evolving but the Workstation
components will include the workstation_session, Ermine and simple-browser. It
should be noted that we do not propose to move any driver development out-of-tree
as part of this proposal even if it is a driver that only supports the
Workstation product.

To allow workstation.git to consume the prebuilt artifacts we will need to set
up a builder of fuchsia.git to create the prebuilts and an autoroller which will
pull the artifacts into workstation.git. We may be able to extend one of the
builders which is currently running today but we have not fully investigated
if that is possible.

The fuchsia.git builder will run once a day and will build the Fuchsia platform.
Exactly how this builder will be configured is yet to be determined because we
need to understand exactly what gets built. The builder will upload all of the
appropriate artifacts to a location that the autoroller can consume. The exact
location and mechanism for uploading these artifacts is still to be determined.
Included with the artifacts will be a manifest file which includes all of the dependency
versions used to create the platform. This includes, but is not limited to, the
SDK version, the version of experiences.git, the Flutter and Dart versions, the
Fuchsia toolchain version, etc.

The autoroller will be run multiple times a day to check for new versions of the
platform. Running multiple times will ensure that we pick up the most recent
versions regardless of flaky rolls or builds that may come at different times
throughout the day. The autoroller will poll the latest version of the platform
and pull in those artifacts. The roller will also read the manifest file which
contains all of the dependent versions and update those as well. This will ensure
that we do not have version skew between the platform and the various dependencies.

The required data are mostly files containing file system partitions, such as
fuchsia.zbi and fvm.sparse.blk. These files contain all the data necessary to boot,
as well as all packages in the ‘base’ and ‘cache’ lists. These files will be
collected into a .far “product package”, and the metadata found at the URL of
this package will contain all necessary metadata to build component packages with
a matching SDK. Packages built against this metadata can be appended to base and
cache by modifying the system partitions contained in the “product package”.
There is parallel work for creating a standalone out-of-tree image assembler, so
the list of specific artifacts that are bundled in the “product package” is
subject to change depending on the API for the new tool.

The daily builder will upload the artifacts in a way that will allow us to
access the "product package" by a given version. A fixed repository will contain
references to all versions of the platform, to enable the roller to identify the
latest build or a specific build suitable for ABI compatibility. The specific
process which will enable this is currently being explored external to this
proposal.

When assembling the final product, the build system should first download the
“product package”, which is identified by URL. It will then download a matching
SDK. (The SDK could also be bundled within the “product package”). It should
build packages against that SDK, creating a meta.far file for each package, as
well as collecting all necessary blobs into a single location. The assembly tool
will then append these blobs and packages to the “product package”, resulting in
a completed product build. The resulting artifacts should also have all necessary
information to generate OTA packages.

### ABI Considerations

Compiling the Workstation product separately from the platform exposes us to
potential ABI incompatibility issues. We need to make sure that we are always
compiling the Workstation components with an SDK release version that is either
equal to or older than, but within the compatibility window per
[RFC-0002](/contribute/governance/rfcs/0002_platform_versioning.md),
the release version of the platform that the Workstation product is being
assembled with. In order to ensure that we maintain this invariant, we will
include the SDK version in metadata that rolls along with the platform and the
workstation.git repository will be updated to use this version of the SDK.

It should be noted that this ABI consideration needs to be extended to other
petals that contribute code to the Workstation product, like the Flutter and
Dart runners. To minimize the risk of ABI breakages coming from these petals we
will initially roll them as part of the platform which means that they will have
already been validated in fuchsia’s global integration.

As we start to move petals out of the platform surface we will need to make sure
that they are compatible with our product. For petals that come back into Fuchsia
Global Integration we can propagate metadata about the versions as we build the
platform itself. However, there is a future in which some of our dependencies do not
come into Fuchsia Global Integration which means we will not have version information
included in the platform rolls. We will need to use platform versioning to make
sure we are including the correct versions and we will need to work out a way to
handle out dated versions.

## Implementation

The process to build and assemble the Workstation product out of fuchsia.git
will take place over the course of several phases, each one building on the
previous. This phased approach is required since pieces of the system which
support out-of-tree development are currently in progress.

### Phase 1

The initial phase for getting the Workstation to build out-of-tree is to set up
the development environment. We will create the workstation.git repository to host the code.
This repository will contain a bootstrap script
which will allow users to get up and running. The repository will use Bazel workspaces
and git submodules to manage third_party code, prebuilts and the vendored in
experiences repository.

### Phase 2

The second phase will focus on getting the build system set up to build the
Flutter components. We will use the existing Bazel rules that live in fuchsia.git
as a starting point to write the build rules. The end state will be when we can
successfully build a Flutter component that can be launched on an already running
device.

At the same time as setting up the rules for building the Flutter components we
will explore how to do assembly out-of-tree using the product assembly tool.
This process will be done manually by just copying all of the artifacts that are
built in fuchsia.git into the workstation.git repository’s out directory and trying
to create a bootable image. The end state will be when we can provision a device with
an image that is assembled out-of-tree but built in tree.

### Phase 3

The third phase of this transition is when we work towards bringing all of these
pieces together to assemble the entire product. This includes setting up all of
the infrastructure required to build the platform artifacts and upload them to
the TUF repository on a scheduled basis. This builder will run in Fuchsia
infrastructure. The workstation.git repository will roll the new version with an
autoroller. The autoroller will update a manifest file in the workstation.git
repository, run integration tests and commit this change. This differs from
fuchsia.git which commits version updates to an integration repository to
coordinate version changes.

The workflow for building and assembling will be:

1. Roll the new version of the platform by updating a manifest entry which
identifies the platform version and all dependent versions. How the versions are
read and identified needs to be designed.
1. Use Bazel and git to download the appropriate prebuilts and third party repositories
needed to build the in-tree components.
1. Use the platform download tool to download all of the platform artifacts from
the TUF repository.
1. Build the in-tree components.
1. Assemble the built components and the prebuilt components into a final image.

## Performance

This project should not have any impact on performance of the Workstation
product since it is only building and assembling. However, we should start to
see significant improvements in the build times for Workstation developers since
they will not need to build the platform.

## Security considerations

We are reusing much of the same infrastructure that fuchsia.git uses but we need
to ensure that Workstation is being built and released in a secure manner. We will
work with the security and infrastructure teams to create a secure release
process and build pipeline.

The Workstation product is only intended to be a reference product but we will want
to make sure we are still following proper security protocols. Once we get closer
to doing any sort of release we will need to work with security to ensure that
we are building our releases in a security compliant way.

## Privacy considerations

There is no impact on privacy to consider at this time. We will be using open
tooling that does not collect user data.

## Testing

The Workstation product has a suite of tests that are run on each commit. These
tests include unit tests, integration tests, e2e tests and performance tests. The
code for these tests will be ported to the workstation.git repository as the
code for the Workstation components are ported to worksation.git.

The logic for these tests live in experiences.git so we will be able to run them
since we are mapping this repository into workstation.git until we actually move
the code. All of these tests are using code and tools that are available in the
public SDK so we will be able to run them out-of-tree but will need to update our
build system to ensure that they can be built and run outside of fuchsia.git.

The tests will be run on every commit to the workstation.git repository to prevent
regressions. This will require collaboration with the infrastructure team to figure
out how to build and run these tests in CQ. The current infrastructure assumes that
fuchsia.git is present so we will need to develop a method that is build system
and repository layout agnostic.

## Documentation

We will only need to add documentation to workstation.git which explains
how to get the repository set up for development. The Workstation development
will remain in-tree so we should continue to keep pointing our documentation to
that workflow.

## Drawbacks, alternatives, and unknowns

One alternative is to just do all of this work inside of the Fuchsia tree but
using tooling that exists entirely in the sdk. We moved away from this approach
because it makes it too easy to hide our usages of private interfaces. Moving
the assembly completely out-of-tree forces us to rely just on the sdk.

There are still some unknowns around exactly what we need to upload to the TUF
repository, the shape of the upload and the format of the metadata that we
upload with the artifacts. We feel that these specifics will be identified as we
start working through the process and we will amend this RFC as needed.
