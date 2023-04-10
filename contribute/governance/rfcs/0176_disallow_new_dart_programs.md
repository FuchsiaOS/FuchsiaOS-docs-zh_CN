<!-- Generated with `fx rfc` -->
<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0176" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
{# Fuchsia RFCs use templates to display various fields from _rfcs.yaml. View the #}
{# fully rendered RFCs at https://fuchsia.dev/fuchsia-src/contribute/governance/rfcs #}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Summary

We propose to create an allowlist for Dart programs in the Fuchsia Source Tree.

The initial allowlist will encompass all such existing Dart programs. Authoring
new Dart programs in the Fuchsia Source Tree will require an exemption.

Authoring new Dart programs which target Fuchsia in the out-of-tree "product"
environment will remain fully supported.

## Definitions

- **Dart program:** a reference to any dart program code, either library or
complete binary, targeted to run on either a host machine or a Fuchsia target.
- **In-tree:** program code inside of the [Fuchsia Source Tree][1], including
the contents of "//src/experiences" and the contents of "//vendor/".
- **Out-of-tree (OOT):** program code outside of the Fuchsia Source Tree which
targets Fuchsia.
- **Global Integration (GI):** The central ledger that defines the current state
of the various projects in the Fuchsia Source Tree.  This ledger is typically
updated by automated processes called "rollers".

## Motivation

The Flutter on Fuchsia team wants to remove support for the Dart programming
language from the in-tree environment.

Creating an allowlist will help the Flutter on Fuchsia team track and restrict
new usages of Dart and plan for their eventual migration. We will cover the
specific plans for migrating existing Dart programs in a future RFC.

There are several reasons the team wants to move Dart support out-of-tree:

- **Reduce Fuchsia Platform maintenance burden:** Removing Dart support in-tree
will enable the Fuchsia platform to develop more quickly. When all products are
developed and assembled out-of-tree, the Fuchsia platform will no longer need to
roll the Dart and Flutter runtimes into GI.  These rolls into GI are a large
source of maintenance burden today.  "Maintenance burden" includes support for
the Flutter->GI rollers, the third_party/dart->GI rollers for 3p dart packages,
and in-tree use of Dart analysis tools.
- **Increase Product/Platform separation:** Even with an allowlist in place,
Fuchsia product owners will still be free to incorporate Dart and Flutter into
their products via out-of-tree product assembly. Out-of-tree product assembly
will become the only mechanism by which product owners can incorporate Dart or
Flutter into their product.  This will clearly demarcate the Dart and Flutter
runtimes onto the Product side of the Platform-Product boundary.
- **Separate validation of the Fuchsia Platform and out-of-tree runtimes:** A
large piece of Dart usage in-tree today is in the form of integration tests
designed to validate the operation of the Dart and Flutter runtimes. Examples
include integration tests in the UI, FIDL, and Diagnostics domains which run
against the Dart runtime. These tests are an example of how product/platform
separation is not enforced right now.  They treat the platform itself as a
product to run against, and then attempt to validate both the platform's
contract and a (product) runtime together in the same test.
- **Reduce reliance on downstream projects in the Fuchsia SDK:** The Fuchsia SDK
publishes a set of Dart libraries, such as SL4F, which are intended to run
against a specific version of the Dart and Flutter runtimes.  Validating the
operation of these libraries requires using a roller to update the Dart and
Flutter runtimes in GI, then writing dart-based tests in-tree.  In the end, this
results in the same maintainability and product/platform separation problems
as called out above.
- **Encourage convergence to ffx-based workflows**: Disallowing the usage of
Dart for new host tools will serve as a forcing function for creating more
ffx-based workflows.
- **Dart language policy:** This proposal is effectively a stronger version of
the [existing language policy][2] that platform components should not be written
in Dart, now extended to platform tests and host tools.

## Stakeholders

_Facilitator:_

abarth@google.com

_Reviewers:_

- akbiggs@google.com (Flutter on Fuchsia)
- fmeawad@google.com (Performance)
- sanjayc@google.com (Workstation)
- yuanzhi@google.com (SL4F)
- shaibarack@google.com (Tech Debt WG)

_Consulted:_

- chaselatta@google.com (OOT)
- jaeheon@google.com (UI)
- crjohns@google.com (Diagnostics)
- yifeit@google.com (FIDL)
- dannyrosen@google.com (Tech Debt WG)
- mangini@google.com (Developer Relations)

_Socialization:_

This RFC's initial socialization proposal went through individual conversations
with all impacted teams, listed in either "Reviewer" or "Consulted" above.  The
RFC author then hosted a workshop during the "2022 Fuchsia SDK Summit" on the
contents of this RFC.

## Design

The Flutter on Fuchsia team will maintain a list of Dart programs which are
allowed to exist in the build graph of an in-tree Fuchsia build. The initial
contents of this list will be all existing Dart programs in-tree.

When designing the policy and the enforcement mechanism, we tried to keep the
following priniciples in mind:

- **Incrementality**: While the Flutter-on-Fuchsia team desires to ultimately
remove in-tree Dart programs, this migration must be done gradually in order to
avoid negatively impacting existing Dart users in-tree. Put another way, we do
not intend to delete any load-bearing Dart program in-tree without finding a
suitable replacement first. Because immediate migration is not possible, we MUST
put in place an allowlist before beginning to migrate any in-tree Dart programs.
This will slow or halt the addition of new programs without negatively impacting
existing programs.
- **Inclusivity**: Even though usage of Dart in-tree is declining already, and
platform components already may not be written in Dart, products built on
Fuchsia will continue to expand their usage of Dart. We MUST preserve the
ability for product owners to continue using Dart out-of-tree for their own
components and host tools. Thus, the allowlist MUST NOT apply to any out-of-tree
Dart programs.

## Implementation

We will generate the initial allowlist in fuchsia.git from the list of existing
Dart programs in-tree. In line with the incrementality design principle we
will include a wild-card entry in the allowlist for performance testing; this
wild-card entry will allow addition of new performance tests inside of
"//src/tests/end_to_end/perf".

After generating the initial list, we will implement the mechanism for enforcing
the allowlist at build time. This implementation will use GN visibility to
restrict the addition of new dart_library, flutter_library, dart_test,
flutter_test, and dart_tool targets to the build.  In this way it will behave
similarly to allowlists like the [driver shared libraries allowlist][3].

After this point, the in-tree GN build will emit a build-time error if a Dart
program is present in the build tree but not contained in the allowlist.
We will not restrict the expansion of existing Dart programs through the
allow-list mechanism, but we will discourage this in design and code-review.

## Performance

This is purely a policy and build-system change; we anticipate no performance
impact.

## Backwards Compatibility

The initial contents of the allowlist will contain all existing Dart programs
in-tree, so enforcement will be fully backwards-compatible.

## Security considerations

This is purely a policy and build-system change; we anticipate no security
impact.

## Privacy considerations

This is purely a policy and build-system change; we anticipate no privacy
impact.

## Testing

We will implement the allowlist as a build-time check, which means the main test
is that the Fuchsia Source Tree continues to build correctly.  If the tree
continues to build correctly, then this means that all Dart programs fall within
the constraints of the allowlist.

## Documentation

This RFC serves as documentation of our intention to create the allowlist.

We will need to update some documentation for developers:

- **Language Policy**: Update the fuchsia.dev language policy to reflect these
policy changes.
- **Tutorials**: Add a warning and allowlist instructions to fuchsia.dev
documentation that covers writing Dart programs.

## Drawbacks, alternatives, and unknowns

A major downside of this proposal is that writing new Dart programs in-tree will
become difficult.  Fuchsia developers often reach for Dart when writing host
tools due to its historical availability and ease-of-use.

Without the option of Dart available, the only viable replacement for writing
host tools which interact with a fuchsia device is to implement an ffx plugin.
We hope this is acceptable to developers because ffx is already the blessed and
well-lit path for writing such host tools using the Fuchsia SDK.

A major unknown of this proposal is how to accommodate Dart's deep integration
with the Fuchsia platform (FIDL bindings, conformance tests) in a world where
Dart programs are strictly built out-of-tree.  We also don't know at this time
how to coordinate the versioning for out-of-tree dart libraries against Fuchsia
SDK versions. The flutter-on-fuchsia team intends to address these unknowns in
a follow-up RFC.

## Prior art and references

The existing Fuchsia language policy for Dart: [Language Policy][2]

This RFC proposes a similar allowlist-based enforcement mechanism as the drviers
shared library allowlist: [Drivers Shared Library Allowlist RFC][3]

[1]: /glossary/README.md#platform-source-tree
[2]: /contribute/governance/policy/programming_languages.md#dart
[3]: /contribute/governance/rfcs/0090_drivers_shared_library_allowlist.md
