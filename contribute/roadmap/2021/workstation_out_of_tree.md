# Build and Assemble Workstation Out of Tree

 * Project lead: chaselatta@google.com
 * Area(s): Experiences

## Problem statement

The source code for the workstation product is currently hosted in a separate
repository, mapped into fuchsia at `//src/experiences`, and can only be built
against the fuchsia buildroot. This structure allows the workstation product to
use internal APIs and does not provide a clear boundary between platform and
product. We should be able to develop the components that comprise the
Workstation product independently from the Fuchsia platform and still assemble
them into a product outside of fuchsia.git.

## Solution statement

The workstation product is actively used by fuchsia developers and thus the
development workflow needs to be iteratively updated to limit the amount of
disruption felt by in-tree developers. To this end, the migration of the
workstation product can be broken down into two phases. The first phase focuses
on us learning how to do product builds and assembly out-of-tree without
changing any of the current workflows. The second phase focuses on us
untangling the workstation product from the platform's test and build story.

### Phase 1 - Build and Assemble out of tree

The initial phase will allow the PDK working group to work towards out-of-tree
assembly by mimicking the way we develop the Workstation product currently but
instead of mapping the experiences repo into fuchsia.git it will be mapped into
a repository that has no knowledge of how to build the platform. More specific
details will be included in a forthcoming RFC.

The currently thinking about how to do this would follow these steps:

 1. Migrate the workstation, as defined by the `workstation_session`, ermine
    and simple browser, off of all internal APIs. We are excluding the Terminal
    from this at this point since its graphic library is not ready for out of
    tree migration. The workstation will still be built in-tree but will only
    rely on the API which is in the public SDK. Note, this transition does not
    include any of the x64 board support or any drivers.
 2. Create a repository to host the workstation product. We plan to host the
    repository on github.com but there is still an open question about which
    organization this would live under.
 3. Create a workflow which maps the experiences repository into the github
    repository using `jiri`. At this point, the workstation repository will not
    be buildable but will give us a place to start.
 4. Teach the github repository how to build the artifacts. We cannot use the
    existing BUILD.gn files for this since they are specific to the fuchsia.git
    source tree so we will instead use bazel to build the workstation
    artifacts. To accomplish this we will use bazel's concept of local
    repositories to create BUILD files for the components in the experiences
    repository without having to change the experiences repository. This will
    introduce breakages in the github repository if developers change
    dependencies or files but this is something that the out-of-tree team can
    be responsible for fixing. There are several other steps that will need to
    happen before we can successfully build a component out-of-tree.
    * Work with the rust team to get out of tree build support for rust
      components.
    * Start pulling the fuchsia SDK into the github repository.
    * Start pulling in all the required toolchains (dart, fuchsia, rust) and
      hooking them up to bazel.
 5. Assemble the workstation product out-of-tree using artifacts that were
    built from the fuchsia source tree. This step requires that we set up a
    daily builder to release the platform. This builder will build the
    artifacts and upload them to a TUF repository which can be consumed by the
    workstation product. When we want to roll the platform into the workstation
    repository we will update a manifest which includes the TUF repository url
    containing all the artifacts and update our SDK and toolchain versions to
    match those that were used to build the platform. The artifacts would be
    downloaded from the TUF repository and assembled using the product assembly
    tool. A couple of notes for this step, the work required for this step is
    captured in the decentralized product integration proposal and the RFC for
    the product assembly tool. We will also need to include more artifacts than
    what will actually represent the platform at this point, for example, the
    flutter runner is likely not going to be part of the platform but we will
    include it at this step to simplify the amount of repositories that we need
    to fetch; eventually, we will remove these artifacts and obtain them
    elsewhere as the infrastructure to support this evolves. Note, the process
    to roll artifacts into the github repository can be manual at this point to
    reduce the need to build out infrastructure.

### Phase 2 - Decouple Workstation the product from Fuchsia the platform

The Workstation product is currently a load-bearing part of the platform
development and test story. The second phase will focus on detangling this
dependency and removing the Workstation entirely from the fuchsia tree.  The
specifics of how we are going to do this are not entirely clear but as we work
them out we will submit an RFC for this phase of the project.

The current thinking is as follows but will evolve as we understand the problem
more:

 1. Create the infrastructure which runs Workstation tests in CI. We would like
    to use as much open infrastructure as we can at this point to both reduce
    the overhead for the fuchsia infrastructure team and to learn what would be
    needed for teams to do this on their own outside of our infrastructure. We
    could use cirrus-ci, which is what Flutter is currently using, to run tests
    that can execute on a linux machine. This includes host tests and tests
    that can run in femu. For hardware tests, we will use luci infrastructure.
 2. Once we know how to run e2e tests and performance tests outside of the
    fuchsia source tree we can start migrating them into the github repository.
    Our e2e tests and performance tests are currently serving two purposes,
    validating that the product is working correctly and validating that the
    platform does not regress. These tests should only be used to verify that
    the product is working as expected and to decide whether to accept a
    version of the platform. By removing the tests from the fuchsia tree we
    will introduce a testing gap where we are using e2e tests to validate the
    platform.  For each test that is moved we will need to identify which parts
    of the platform will lose test coverage and write a CTS test to cover the
    gap.  This will require coordination with the CTS team to identify what
    work needs to be done here.
 3. When all of the tests are migrated we can delete the workstation components
    from the fuchsia source tree and move them into the github repository. This
    will effectively decouple the platform from the workstation product.
    However, in doing this we will break current workflows and we will need to
    address these.
    * Day-to-day development using the workstation: many engineers use the
      workstation product to test their changes, this applies to teams like
      accessibility, input, sfw and many more. We would need to figure out how
      these teams will work going forward if they don’t have the workstation
      product. We have some ideas here but this needs more research. Some
      initial thoughts are to create local TUF servers for developers that will
      let them assemble workstation from local fuchsia artifacts. Another
      thought would be to make it very simple to create purpose built products
      for individual teams to use in their own workflows. This would be similar
      to creating a component to exercise some functionality in tests.
    * Workstation release program: the workstation program currently has a
      weekly walkthrough and has an on-call rotation that promotes canary
      builds and integrates with the test team. These programs will need to
      transition to using workflows that do not depend on in-tree tools. The
      build walkthroughs are moving to using builds from the ota channels so
      that should be covered but we currently do not have tooling that allows
      us to release or promote builds outside of the fuchsia tree. We will need
      to work with the MOS team to make these tools available out-of-tree.

## Dependencies

### FPM

The decentralized product integration workflow will drive most of the work that
goes into making the tooling needed available to this project. This project is
a natural extension of that workflow.

### Workstation Experiences

The workstation team team will need to update the existing workstation
components to be buildable entirely against the SDK. They will also need to
change their existing workflows to move to working in an out-of-tree
environment. This will most likely cause a disruption to their process as we
iron out the workflows.

### SFW

The session framework team will need to make the current, legacy rust libraries
available as a fidl interfaces which are in the public SDK. They will need to
create components which serve these interfaces instead of having the
workstation session call into private libraries directly. This could come in
the form of a refactor of the workstation session as well.

### Rust

The Fuchsia Rust team will need to support out of tree development if we are to
build the workstation session in rust. If this work cannot be done by the rust
team then the workstation team will take on the work to rewrite the session in
another language.

### SDK/Toolchain

The SDK and toolchain will need to roll into the github repository so we can
build the components. The process to set up the integration will be done by the
workstation out-of-tree team and they will also be responsible for fixing any
breakages. The only impact this will have on the SDK and toolchain team will
come in helping to diagnose issues. We would like to purposely limit the amount
of work required by these teams to support us so that we can use the
workstation as a datapoint for scaling to other partners. As this project nears
completion of phase 2 we will be in a position that we can start treating
workstation more like a 1p customer which has SLAs against breaking SDK
changes, however, I believe that this should be done not necessarily as a way
to prevent the breakage of workstation but rather to give the SDK team a
location in which they can experiment with ways to roll out changes without
breaking customers in a more scalable way.

### Build

The workstation product will be built using bazel. This will take more setup
time than if we were to use gn but it will allow us to build against a
different build root without having to make changes to the BUILD.gn rules in
fuchsia. The workstation team will be responsible for writing these build rules
and maintaining them. If the fuchsia build team decides to explore using bazel
in the future we can work together to try to make these available to both
repositories.

### Infrastructure

For the initial phase of this project where we focus on just getting out of
tree assembly working we will need help from the infrastructure team to help
set up a daily builder to publish a daily snapshot of the platform. This
builder will be monitored by the workstation out-of-tree team but they will
likely need help from infrastructure to debug build failures.

As we move into phase 2 of the project we will need to set up continuous
integration to ensure that the Workstation product does not break. This
infrastructure will span across fuchsia infrastructure and open source
infrastructure. The open source infrastructure will be used for running tests
on femu, building release candidates, rolling in sdks/toolchain/etc. The
workstation team will need to collaborate with the infrastructure team on the
best approaches to do this but they should be responsible for doing the work to
see what we are missing to allow non-fuchsia teams to do this. The tests that
need to run on devices will need to run in fuchsia infrastructure because we
will not have an open source device lab. This collaboration will look similar
to the collaboration that we currently have with the flutter team which runs on
our device lab.

### CTS

As we start to untangle the Workstation product from the platform we will need
to start moving the workstation tests that are used for platform validation
into CTS. This will require collaborating with the CTS team to ensure that the
test infrastructure exists to support this and will likely involve working with
other area owners to start migrating/writing their tests in cts.

### DX

The DX team will need to be involved to ensure that we have appropriate
workflows for out of tree developers and that we have appropriate documentation
on fuchsia.dev.

## Risks and mitigations

There is no direct risk to our existing customers because the workstation is a
completely separate product. The workstation runs on the session framework
whereas our existing customers run on the modular framework.

It is not likely that this project will have any adverse impact on fuchsia
infrastructure that is used for CI/CQ since we will be moving code and
infrastructure out of fuchsia.git and using our own infrastructure. The
workstation product will use open source infrastructure which is still to be
determined (cirrus-ci, travis, jenkins, etc) and will run outside of fuchsia.
We will need to use fuchsia infrastructure for on-device testing which could
have a negative impact on our infrastructure.

There is a risk that we will introduce workflow changes which slow down the
development process for in-tree developers. We already have this problem with
developing other out-of-tree clients, such as Chromium and Flutter, so we can
look at those workflows to identify situations that can be improved. We can
help to mitigate any potential problems by working with the ux-research team to
identify which workflows will break before making changes.

The biggest risk from this project is that we will not have a load-bearing
product to validate the platform against in CI/CQ. This will require us to make
sure that we identify the gaps that will be opened up when the Workstation
product is gone and replace them with something that does not need the
workstation product.
