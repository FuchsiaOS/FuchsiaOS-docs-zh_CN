# Platform Surface Area ("Plasa")

Fuchsia Platform Surface Area (hereafter: "Plasa" for short)  is an explicit
declarative record of the [Fuchsia System Interface][fsi].  It is intended to
capture the pertinent state at the time the [Fuchsia Integrator Development Kit
(IDK)][idk] has been built.

This subsection of [`fuchsia.dev`][fxdev] describes Plasa for users and
implementors alike.

## Documentation

* [Platform Surface Area Manifest][plasadoc] design document

## Uses

Plasa provides input to the following efforts:

* Platform versioning per [RFC-0002][rfc2]
* [Compatibility Test Suite (CTS)][cts] per [RFC0015][rfccts]

## Limitations

Currently, Plasa provides the following platform surface fragments:

* The [FIDL][fidl] API fragments for all public FIDL APIs in the
  [Fuchsia SDK][sdk]
* The C and C++ API fragments for all public C++ APIs in the Fuchsia SDK.

While this falls short of fully describing the entire Fuchsia System Interface,
work is underway to backfill the missing platform surface fragments.

[cts]: /development/testing/cts/overview.md
[fidl]: /concepts/fidl/overview.md
[fsi]: /concepts/packages/system.md
[fxdev]: https://fuchsia.dev
[idk]: /development/idk/README.md
[plasadoc]: plasa_manifest.md
[rfc2]: /contribute/governance/rfcs/0002_platform_versioning.md
[rfccts]: /contribute/governance/rfcs/0015_cts.md
[sdk]: /contribute/governance/rfcs/0106_manifest_includes_in_sdk.md
