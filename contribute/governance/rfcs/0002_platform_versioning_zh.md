{% set rfcid = "RFC-0002" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

## Summary

This document proposes the notion of an *API level* and an *ABI revision* to
the Fuchsia platform. End-developers build against a *target API level*, which
determines which declarations are visible to the application. The
*target API level* also becomes embedded in the compiled application as a
*target ABI revision*, which indicates the semantics the application expects
from the platform. A given release of the Fuchsia platform typically supports
multiple ABI revisions, which lets the platform run older applications while
still providing a path for evolving the platform.

## Motivation

Currently, the Fuchsia platform evolves through a series of *soft transitions*.
To change part of the [Fuchsia System Interface], the platform first introduces
the new interface. Applications then migrate to the new interface. After all the
applications have migrated to the new interface, the platform then removes the
old interface.

Using this approach, the platform can evolve only as fast as the slowest
application. In order to complete a soft transition, the platform needs to wait
for the last application to migrate off the old interface. As the number of
applications increases and the coupling between the platform and the
applications decreases, soft transitions take increasingly longer to execute.
Eventually, we will be unable to evolve the platform using soft transitions.

This RFC addresses the following problem statement:

> How can the Fuchsia platform continue to evolve while being able to run a
growing number of older applications over a longer period of time?

## Why now?

Several of our customers are requesting more stability from the platform. If we
offer that stability now, we will slow down our ability to evolve the platform.
In order to meet these current customer needs, the platform needs to be able to
offer longer compatibility windows without grinding the project to a halt.

Additionally, the experience from Windows is that we would benefit from
embedding target ABI revisions in applications prior to being required to
provide binary compatibility with those applications for a long period of time.
Windows missed that opportunity and now tries to guess the target ABI revision
for binaries using heuristics, which creates significant developer pain.

## Terminology

A *release* of Fuchsia is a build of the Fuchsia operating system and associated
packages that is deployed to a user population. A release has a version number
that identifies the set of software artifacts contained in the release.

*Backwards compatibility* refers to the ability of a newer release of Fuchsia to
run binaries intended to run on older release of Fuchsia.

The *Fuchsia IDK* is an artifact used by development environment integrators to
expose the Fuchsia platform to developers to build applications that
run on Fuchsia. The Fuchsia IDK is published by the Fuchsia project and defines
the contract between the Fuchsia platform and applications that run on Fuchsia.
The IDK tools define the contract between the Fuchsia IDK tools and the
development environment integrators' environments.

A *soft transition* is a technique for breaking down a backwards-incompatible
change into a sequence of smaller changes to the platform and a set of known
binaries such that compatibility is maintained locally at each step.

## Design

The design described in this document is to version the [Fuchsia System
Interface], which lets the platform and the applications agree about the
semantics the application expects from the platform.

Specifically, if an application works on a given release of Fuchsia, then the
application should continue to work on future releases of Fuchsia unless Fuchsia
intentionally drops support for the application. This design does not address
the converse problem of creating a new application that works on older releases
of Fuchsia.

### Versioning

The Fuchsia platform uses two version identifiers, an *API level* and an
*ABI revision*. Both these versions identify the *interface* provided by the
platform rather than the *implementation* of that interface. Releases of Fuchsia
use a different versioning scheme, which identifies the specific implementation
in that release.

A given API level implicates a specific ABI revision, but multiple API levels
might implicate the same ABI revision.

#### API level

A Fuchsia *API level* denotes a set of APIs available when building an
application. A given release of the [Fuchsia IDK] typically supports multiple
API levels. The APIs available at a given supported API level should be
consistent across IDK releases.

> *Example.* Consider `pkg/fit`, which is a C++ library in the SDK. The `fit`
library declares a number of functions, each of which is an API exposed by the
library. The API defines that set of functions, which means two IDK releases
should expose the same set of functions in the `fit` library at the same API
level.

Syntactically, a Fuchsia *API level* is an unsigned, 64-bit integer[^1]. 
As the platform evolves (see *Evolution* below), API levels are assigned in
increasing order and are intended to be understood by human beings, including end-developers.

#### ABI revision

A Fuchsia *ABI revision* denotes the semantics of the [Fuchsia System Interface]
that an application expects the platform to provide. A given release of Fuchsia
typically supports multiple ABI revisions, but semantics for a given supported
ABI revision should be consistent (see *Evolution* below) across Fuchsia
releases.

> *Example.* Consider `zx_clock_get_monotonic`, which is a function exposed by
the vDSO as part of the [Fuchsia System Interface]. The ABI revision specifies
both whether this function exists and what happens when this function is called,
which means the semantics of `zx_clock_get_monotonic` should be consistent
across Fuchsia releases at the same ABI revision.

Syntactically, a Fuchsia *ABI revision* is an unsigned, 64-bit integer. An ABI
revision is an opaque identifier without internal structure. To create an
identifer for a new ABI revision, select a unsigned, 64-bit integer at random
among values that have never been used to identify a Fuchsia ABI revision
before.

Identifiers for ABI revisions are chosen at random to prevent end-developers
from guessing a future ABI revision identifier and forming expectations about
the semantics of a future version of the [Fuchsia System Interface]. As a
result, ABI revisions are intended to be understood by machines and only rarely
interpreted by human beings.

#### Evolution

The platform increases the API level whenever the platform adds or removes an
API from the [Fuchsia IDK] or when the ABI revision changes. In practice, the
project might batch changes by increasing the API level on some defined cadence
(e.g., once a day or once a week).

The platform changes the ABI revision whenever the platform makes a
*backwards-incompatible* change to the semantics of the
[Fuchsia System Interface]. In practice, the project might batch
backwards-incompatible changes by changing the ABI revision on some defined
cadence (e.g., every six weeks or every six months).

In the limit, every change in semantics is potentially backwards-incompatible,
but, in practice, operating systems do make changes to their semantics without
breaking applications. For example, many popular operating systems add system
calls without breaking their applications.

> *Action item.* Create a document that details what changes to the Fuchsia
System Interface the platform considers to be backwards-compatible. The project
will likely need to refine that document over time as the project gains
implementation experience about what changes commonly do and do not break
applications in practice.

### Applications

End-developers select a single *target API level* when building a component.
The target API level controls which declarations in the [Fuchsia IDK]
are available when building the component. For example, a FIDL message
introduced in API level 7 is not available when building a component that
targets API level 6 but is available when building a component that targets API
level 7 or 8 (assuming the message was not deprecated in API level 8).

As part of building a component, the tools in the SDK include the
*target ABI revision* associated with the target API level in the manifest of
the component. In this way, each component declares the semantics that the
developer expected the platform to provide when they built their component. A
given package can contain many components, each of which can select whichever
target ABI revision they prefer.

### Platform

The platform maintains a list of *supported ABI revisions*. The platform
provides binary compatibility for components that target a supported ABI
revision, which means the platform will attempt to provide those components the
platform semantics indicated by their target ABI revision.

> *Example.* Consider the transition from the `fuchsia.foo.Bar` protocol to the
`fuchsia.foo.Bar2` protocol. Suppose a component, `baz.cm`, has a target ABI
revision that indicates that the component expects the platform to provide the
`fuchsia.foo.Bar`. When running `baz.cm`, the platform will route requests for
`fuchsia.foo.Bar` to the appropriate implementation. However, when running
components with a target ABI revision after the transition to
`fuchsia.foo.Bar2`, the platform will no longer route requests for
`fuchsia.foo.Bar` to an implementation because components targeting that ABI
revision should be using `fuchsia.foo.Bar2` instead.

At some point, the platform might wish to remove support for a given ABI
revision. Such removals are often gated on a tail of important components that
still rely on the old ABI revision. Rather than maintaing the full semantics
implied by the older ABI revisions, the platform maintains a list of *legacy
components* along with a table of *quirks* necessary to run those specific
components. A quirk is a compatibility shim that lets a legacy component use an
otherwise unsupported interface. Using this mechanism, the platform can remove
general support for an older ABI revision while still being able to run certain
important components that target that older ABI revision.

> *Example.* Suppose the platform no longer supports any ABI revisions that
include `fuchsia.foo.Bar` but that `baz.cm` is an important component that has
not migrated to `fuchsia.foo.Bar2`. The project can treat `baz.cm` as a legacy
component with the `needs-fuchsia-foo-bar` quirk. Even though the platform does
not support the target ABI revision for `baz.cm`, the platform can continue to
run `baz.cm` by routing its request for `fuchsia.foo.Bar` to a compatibility
shim, perhaps implemented using `fuchsia.foo.Bar2`. The compatibility shim does
not need to support the full semantics implied by `fuchsia.foo.Bar`. Instead,
the compatibility shim need only work well enough to keep `baz.cm` (and the
other specific components with the `needs-fuchsia-foo-bar` quirk) working.

The platform cannot run components that neither target a supported ABI revision
nor are listed as legacy components because the platform does not know what
semantics those components expect.

### Lifecycle {#lifecycle}

Every element of the [Fuchsia System Interface][Fuchsia System Interface]
(e.g., a system call or a FIDL message) goes through the following lifecycle:

 1. The element is *introduced* into the platform. End-developers cannot use the
    API until Fuchsia releases an SDK with a new API level that includes that
    element. If the element can be introduced without breaking the ABI (e.g.,
    adding a system call), then the semantics of existing ABI revisions can be
    updated to include the newly introduced element. Otherwise, the element must
    be hidden from components that target older ABI revisions to avoid breaking
    them.
 2. If possible, the element can be *extended* by introducing child elements.
    For example, a FIDL table can be extended by introducing new fields.
    Introducing a child element starts another instance of the element lifecycle
    for that child element, including requiring a new API level to make the API
    for that element visible to end-developers. An element can be extended only
    if adding child elements does not break the existing API or ABI.
 3. The element might be *deprecated*. Components that target older ABI
    revisions can still use the element when running on newer platform releases.
    However, end-developers that target a newer API
    level can no longer use the element.
 4. The element is a *legacy* once the platform no longer supports any ABI
    revisions between the *introduction* and *deprecation* of the element. At
    this point, the platform need only support the element insofar as the
    element is actually used by a specific legacy component by way of a quirk.
 5. Once none of the legacy components use the element, the element can be
    *removed* from the platform entirely.

### Dynamics {#dynamics}

This approach incentivizes developers to migrate away from deprecated interfaces
by coupling access to new APIs to performing those migrations. Specifically, to
gain access to a newly introduced API, the developer must change their target
API level, which requires them to migrate off any interfaces that were
deprecated in that API level.

## Implementation

Implementing this design involves many layers of the Fuchsia system. This
document provides a sketch of the changes needed at each implicated layer, but
the detailed designs for those layers are left to subsequent documents.

### FIDL {#fidl}

FIDL should offer a way to annotate the range of API levels in which each
protocol element is available. The FIDL toolchain should be aware of the
target API level and generate code appropriate for that API level.

When a protocol element (e.g., a field in a table or a message in protocol) is
deprecated at a given API level, we would ideally like components that target
that API level to be able to receive messages containing that protocol element
but would like to prevent those components from sending messages that contain
that protocol element.

### System headers

The system headers should let the end-developer specify a target API level and
then adjust the set of APIs that are visible using those headers according to
the target API level. In addition, the system headers should define macros that
can be used to limit the visibility of declarations in other libraries to
certain API levels.

### vDSO

The system should offer multiple vDSOs, each of which supports a list of ABI
revisions. When possible, the system should evolve by extending the vDSO in a
backwards-compatible way, but, when not possible, the system can mint a new vDSO
with a separate list of supported ABI revisions.

Extending the vDSO increases the attack surface for existing binaries because
those existing binaries can gain access to the vDSO extensions. When deciding
whether to extend an existing vDSO or whether to mint a new vDSO, the project
should consider the security implications as well as the compatibility
implications.

The vDSO could offer a function that checks whether the vDSO supports a given
ABI revision, but the vDSO should not directly expose the list of supported ABI
revisions because exposing that list to applications would let applications break
when the list is extended.

### Process framework

When launching a process, the client should inform the process launcher which
ABI revision the process expects. The process launcher should use that
information to select an appropriate vDSO and process bootstrap message for the
newly launched process.

> *Open problem.* What ABI revision should we use when creating processes that
do not have a component manifest? One possibility is to put the ABI revision in
the ELF data for the executable rather than (in addition to?) in the component
manifest. Another possibility is to add the ABI revision to the
`fuchsia.ldsvc.Loader` protocol, which is typically routed to the source of the
executable.

### Component framework

The tools that build component manifests should take the target API level as a
command-line parameter and embed the corresponding ABI revision in the component
manifests they create.

While not needed immediately, components will eventually want to modulate
capability routes according to ABI revision. For example, a component might wish
to stop offering a certain service to one of its child components. Removing the
service immediately could break compatibility with older versions of that child
component. Instead, the parent might want to offer the service only to children
that target an older ABI revision.

Similarly, the platform might wish to route capabilities for specific legacy
components to specialized destinations that provide compatibility shims. For
example, we could define a routing *quirk* that gets applied for specific legacy
components that have that quirk in the quirk table.

### SDK

The SDK should specify the API levels supported by the SDK and the mapping
between those API levels and their ABI revision in some machine-readable format
(e.g., in its JSON metadata). The SDK integrations should be modified to let
end-developers specify a target API level and to supply the target API level as
a command line argument to all the tools that require that value.

## Performance

This proposal attempts to minimize the performance impact of platform versioning
by intervening primary during build and discovery. The compatibility shims used
to run legacy components could have a significant performance impact, but the
project can evaluate those performance implications on a case-by-case basis
when adding a component to the list of legacy components.

## Security considerations {#security-considerations}

This proposal should have a positive impact on security because the proposal
will make it easier to migrate the Fuchsia software ecosystem to newer APIs,
which presumably have better security properties than older APIs.

Additionally, the ability to allocate new ABI revisions makes it possible to
avoid exposing new ABIs to existing applications, which can reduce the attack
surface exposed to those applications. When deciding whether to extend an
existing ABI or whether to allocate a new ABI revision, the project should
consider the security benefits of allocating a new ABI revision.

This proposal does provide a mechanism for malicious applications to select
different, potentially older, code paths in the platform, for example by claiming
to target an older ABI revision. As the platform evolves, the project will need
to treat code that supports older ABI revisions with the same security diligence
that the project treats code that supports newer ABI revisions.

## Privacy considerations

This proposal should have a positive impact on privacy because the proposal
will make it easier to migrate the Fuchsia software ecosystem to newer APIs,
which presumably have better privacy properties than older APIs.

## Testing

This proposal somewhat increases the testing matrix because the platform behaves
different depending on the ABI revision of the running component. We will need
to factor this increase in the testing matrix into the design of the Fuchsia
Compatibility Test Suite (CTS). For example, the project might want to version
CTS according to the ABI revision to ensure that the platform does not regress
its support for older ABI revisions as it evolves.

## Documentation

The documentation for the platform should be updated to annotate every API with
its current stage in the lifecycle as well as its lifecycle history (e.g., when
the API was introduced, deprecated, and/or removed). These annotations should be
derived from the same source-of-truth that control whether applications have
access to these API when targeting a specific API level. For example, the
`fidldoc` tool should understand the API level annotations in the FIDL source
files and generate the appropriate annotations in the generated documentation.

Whenever the platform creates a new ABI revision identifier, the project should
update the documentation to describe in what ways the new ABI revision is not
backwards compatible with the previous ABI revision and what action, if any,
end-developers should take when updating their applications.

In addition, the project should have some conceptual documentation that explains
why the platform has API levels and how to upgrade from one API level to
another.

## Drawbacks, Alternatives, and Unknowns

### What are the costs of implementing this proposal?

The main cost of implementing this proposal is increased operational complexity
when evolving the platform. Adding a new API now requires coordination across
the project to release that API in a new API level. Similarly, deprecating an
ABI is more involved because deprecation happens in several steps.

The system itself will also become more complicated because the behavior of the
system will be partially dependent on the ABI revision of each component.

### What other strategies might solve the same problem?

Another strategy, which is used by some other platforms, is to never remove
functionality. For example, the web platform evolves almost entirely additively.
In some ways, that approach is simpler because the system would not need a
mechanism to deprecate functionality.

Another approach might be to use different version identifiers for different
parts of the system rather than a single API level that applies to the entire
system. To a certain extent, Fuchsia uses this approach as well. For example,
the file systems each have their own version identifiers, which is used for the
contract between the on-disk representation and the in-memory code for the file
system. Using a single API level for the entire system implies a degree of
coordination about the evolution of contract between the platform and
applications.

## Prior Art and References {#prior-art-and-references}

There is a vast amount of prior art on this subject. The proposal in this
document builds directly on the experience of Android, Windows, and macOS/iOS.

### Android

Android has the concept of an API level. Every platform interface on Android is
annotated with the API level at which the interface was introduced. Android
applications also specify their target API level in their manifest using the
[`uses-sdk`] element. In principle, Android could use this API level mechanism
to deprecate and remove older interfaces.

### Windows

Windows makes heavy use of a concept similar to ABI revision, which appears as
the [`SupportedOS`] entry in application manifests. Windows uses a GUID to
identify the ABI revision that the application is targetting, which is similar
to the proposal in this document to use an opaque 64-bit integer.

In Windows, the `SupportedOS` GUIDs are associated with specific releases of
Windows. For example, `e2011457-1546-43c5-a5fe-008deee3d3f0` identifies Windows
Vista. However, later versions of Windows (e.g., Windows 7) understand the
`e2011457-1546-43c5-a5fe-008deee3d3f0` GUID and provide compatibility with the
Windows Vista ABI. The proposal in this document decouples the ABI revision from
platform releases, which is more flexible.

### macOS, iOS

Both macOS and iOS use the [`API_AVAILABLE`] and `@available` annotations to
control whether a declaration is available when building an application.
System libraries (aka frameworks) also use "linked on or after" checks and
explicit quirk tables to support legacy applications that require older
semantics from the platform.

Apple has used these mechanisms successfully to migrate applications for these
operating systems from older APIs to newer APIs.

[^1]: [RFC-0083: FIDL Versioning][rfc-0083] amends this, restricting
    API levels to 63 bits in order to reserve the high bit for other uses.

[Fuchsia System Interface]: concepts/packages/system.md
[Fuchsia IDK]: development/idk/README.md
[`uses-sdk`]: https://developer.android.com/guide/topics/manifest/uses-sdk-element
[`SupportedOS`]: https://docs.microsoft.com/en-us/windows/win32/win7appqual/compatibility---application-manifest#leveraging-feature-capabilities
[`API_AVAILABLE`]: https://developer.apple.com/documentation/swift/objective-c_and_c_code_customization/marking_api_availability_in_objective-c
[rfc-0083]: contribute/governance/rfcs/0083_fidl_versioning.md
