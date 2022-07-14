<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0000" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
{# Fuchsia RFCs use templates to display various fields from _rfcs.yaml. View the #}
{# fully rendered RFCs at https://fuchsia.dev/fuchsia-src/contribute/governance/rfcs #}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Summary

Developers need to work with a variety of Fuchsia-based products, each
supporting a potentially different Fuchsia ABI revision. Developers
use host-side tools to connect to Fuchsia systems and interact with
those systems. These tools need to have a defined compatibility policy
with regard to the Fuchsia system they are connecting to in order to
ensure the user gets predictable and clear behavior.

This RFC lays out a set of policies that describe when tools are
intended to be compatible with Fuchsia-based product releases, and
some initial steps we intend to take to enforce compatibility.

The extremely short version of the policy is that, if an SDK ships
with a supported developer tool, that developer tool has the same
compatibility guarantee as a component that targets the most recent
ABI revision supported by that SDK.  As a result, the tool will be
guaranteed to work with any subsequent Fuchsia product
bundle (as described in [RFC-0100]) that supports that ABI.

## Motivation

Fuchsia has a well defined notion of what it means for a component to
be compatible with a given system.  As detailed in [RFC-0002], each
release of the Fuchsia SDK supports a set of API levels and ABI
revisions.  If a component targets a given ABI revision, and the
Fuchsia product bundle - the set of artifacts needed to run a
Fuchsia product, excluding the SDK and its tools - supports that
revision, then the component is compatible with the given product
bundle.

New Fuchsia platform releases continue to support ABI revisions for a
specific length of time (or _compatibility window_) since their
introduction.  (An upcoming RFC is going to lay out a new policy for
how we determine the length of compatibility windows.)  As a result,
the component will be compatible with new Fuchsia builds for the
duration of the compatibility window.

There is no similar notion of compatibility for tools released with
the Fuchsia SDK, which currently have ad hoc compatibility mechanisms
that mostly reflect the intent of specific teams and developers.   This
RFC addresses that shortcoming.

The mismatch between tools and components can cause significant user
frustration.  Many Fuchsia users download product bundles separately
from tools.  This leads to a situation where developers use tools that
support a different ABI from the ones supported by the products they
are trying to monitor and manage.  This results in errors that
developers find difficult to diagnose and resolve.

In this RFC, we set an initial policy for when tools are expected to
work with a given Fuchsia product bundle, and describe steps we intend
to take to enforce and communicate this policy.  This policy is not
intended to be complete.  There are other aspects of SDK tool
compatibility that we do not address here, such as guarantees about
the lifetime of command line options, or tool compatibility with older
C/C++/FIDL headers.

## Stakeholders

_Facilitator:_

abarth

_Reviewers:_

abarth (versioning, Fuchsia TL)
amituttam (tools)
dschuyler (SDK)
sebmarchand (customer representation)
sethladd (SDK, versioning)

_Consulted:_

ffx Team
CTF team
Editor Team
wilkinsonclay (developer relations)
yaar (customer representation)

_Socialization:_

This RFC was socialized with representatives from the component
framework and developer areas, as well as discussed on mailing lists
with stakeholders in Fuchsia versioning.

## Design

### Definitions

An _SDK Tool_ is an executable that ships with the SDK.  Note that,
for the purposes of this policy, individual `ffx` plugins count as
individual tools.

_Partner API_ and _Partner Tools_ are API and Tools supported for use
by targeted individuals and teams who work closely with the Fuchsia
team on feature development.  They are shipped in a _Partner SDK_.

_Public API_ and _Public Tools_ are API and Tools that are supported
for use by anyone.  They are shipped in a _Public SDK_.  Note that, as
of the writing of this document, there is no public SDK, only partner
SDK.

In this document, we use the term _stable_ to refer to both a)
supported partner tools / API that target public or partner ABI and b)
hypothetical future public tools / API that target public API.  In the
future, it is likely that public and partner platform surface area
will have different compatibility windows, but the distinction is not
germane to this document.

An SDK tool is _supported_ if it does not have any indication (through
command line flags, the naming of the tool itself
(e.g. `foo_internal`), or documentation) that it is experimental.  As
an example, the `ffx` tool has several experimental subcommands that
are not considered supported.  Unsupported tools are not guaranteed to
work, even if the system claims compatibility with a specific ABI
revision.

### Policy

This RFC introduces the requirement that supported developer tools
shipped with SDKs must exclusively interact with the platform via ABI
/ API supported by the corresponding SDK when interacting with the
system.

If a tool is supported for partner use, it must only interact using
partner or public ABI / API.  If a tool is supported for public use,
it must only interact using public ABI / API.  Supported SDK tools
must not use internal or experimental ABI / API.

By following this discipline, supported SDK tools will be guaranteed
to be compatible with all Fuchsia products developed at head or from a
branch made during the compatibility window for those ABIs and ABIs.

Stable developer tools may target formerly stable API / ABI - that is,
stable API / ABI that has been deprecated or removed from the
platform.  For example, the Fuchsia project may find it useful to have
tools that can flash new images onto devices running images that
predate the compatibility window.  Such behavior should be clearly
described in documentation.  In such cases, unless the tools
themselves are deprecated, they must continue to work with stable API
/ ABI.

In cases other than deprecated and removed API / ABI, when stable
tools target non-stable API/ABI, it is considered a bug, and there
must be a plan to transition them to stable API/ABI or remove them.
This includes currently supported tools in the SDK that use non-stable
API.

For the purposes of experimentation, the SDK may contain non-stable
developer tools.  These must be labeled as such.  Non-stable developer
tools may use non-stable ABI / API.  Documentation for non-stable
tools must indicate that they are not supported.  Invocations of
non-stable tools should make it clear that they are not supported
(e.g., by requiring an extra command line flag).  Such tools may
produce warnings when used.

The fact that individual `ffx` plugins are considered separate tools
means that, for example, `ffx foo` may be considered stable, but, in
the same release, `ffx bar` may not.  Other tools may have similar
policies; these should be well documented.

The Fuchsia platform provides no guarantee of forward compatibility
for its tools.  Recent versions of tools may not work with older
products.  Developers should make every effort to ensure that tools
provide clear and actionable errors in these cases.

## Implementation

Individual SDK tool owners may decide to enforce compatibility
requirements during development, via mechanisms such as testing and
build support.  These enforcement mechanisms may become mandatory for
SDK tools in the future.

The team working on the `ffx` tool plans to take the following actions
to enforce compatibility.  This list is not intended to be exhaustive;
it is simply a set of examples of actions teams can take to enforce
these policies.

  * Older versions of `ffx` will be run as part of the set of CTF
    tests (defined as `CTS tests` in [RFC-0015]), enforcing that newer
    platform builds maintain compatibility with tested features.

  * The command line and JSON interfaces for `ffx` will be versioned,
    and compatibility will be enforced for those versions.

  * The `ffx` tool, and the underlying Overnet transport, will have a
    target ABI revision with which it is compatible, which is likely
    to be the revision at `fuchsia.git` head when it was built.  The
    tool will report the target ABI revision to a service on the
    Fuchsia target.  The service will report back whether the ABI is
    in the supported set.  If it is not, then `ffx` will produce an
    error for the user, and direct the user to a compatible version of
    `ffx`.  We may need to revisit the specifics of this approach as
    compatibility enforcement evolves; in the short term, only static
    ABI revision (i.e., determined at assembly time) will be
    available, but in the longer term, there will be different ABI
    exposed by different parts of the system, and we will need to
    provide a more dynamic check.

  * Stable `ffx` plugins built in`fuchsia.git` will use build
    restrictions to restrict the set of FIDL definitions they use to
    those available in public and partner SDKs.

  * It will be an explicit goal of Fuchsia tooling owners to migrate
    `ffx` plugins intended for long term support away from unstable
    API and ABI.

  * ABI revision will be incorporated into product bundle metadata to
    help developers identify which versions of `ffx` will work with
    that product bundle prior to running it.  We will have a goal of
    attaching version identifying information to any input to `ffx`
    that needs to have it.


## Performance

These policies do not affect performance.  Enforcement of these
policies may affect performance; however, this is a quality of service
issue for an individual tool.

## Ergonomics

These policies improve tool ergonomics by making it clear when a given
tool is compatible with a given Fuchsia target.

## Backwards Compatibility

Many existing SDK tools do not support mechanisms to detect or enforce
compatibility guarantees, and many use non-stable API and ABI.  Some
tools will need to make a transition to long-term stable ABI.  As
always, transitions must be done carefully, to minimize disruptions to
users.  Developers should consider the option of treating non-stable
API and ABI used by their tools as if it were stable by continuing to
support it for the duration of a full compatibility window, and
performing a soft transition to the stable replacement.

## Security considerations

These policies introduce no known security considerations.

From time to time, security issues may require us to break a
compatibility guarantee.  For example, we may find security issues
with the Overnet transport that we use to communicate from the host to
the device.

## Privacy considerations

These policies do not relate to the collection of user data.

## Testing

Although it is a best practice to use testing to ensure that an
SDK-facing developer tool adheres to these policies, this RFC does not
require developer tools to do so.  The Fuchsia CTF (formerly known as
CTS) provides a mechanism for testing that some tool developers may
wish to employ.

## Documentation

Tool documentation should be updated to refer to this policy.

## Drawbacks, alternatives, and unknowns

One alternative is an ad hoc approach to compatibility: that is, each
tool provides its own minimum compatibility window.  In practice, we
have found that this causes a great deal of user confusion, as users
do not typically use a single tool in isolation.  For example, a user
may find it surprising if `ffx log` works and `ffx component` does not
work.

## Prior art and references

Tools for many systems face backwards compatibility challenges.  For
example, in Java, bytecode is tagged with a classfile version number.
The JVM has a set of classfile version numbers it understands.
Sufficiently old classfiles may not work.

<!-- xrefs -->

[RFC-0002]: /contribute/governance/rfcs/0002_platform_versioning.md
[RFC-0015]: /contribute/governance/rfcs/0015_cts.md
[RFC-0100]: /contribute/governance/rfcs/0100_product_metadata.md

