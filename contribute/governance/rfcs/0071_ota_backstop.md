{% set rfcid = "RFC-0071" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!--
*** This should begin with an H2 element (for example, ## Summary).
-->

## Summary

This document proposes a plan to prevent devices from installing over-the-air (OTA) updates
backwards across a version boundary.

## Motivation

When the storage stack makes breaking changes to a filesystem format, they roll the major version
number of the format, which prevents drivers running on older system versions from attempting to
mount and use the images in the new format.

Having an equivalent version number in the system update stack would prevent users from attempting
to OTA backwards to a driver version that does not support the filesystem image their device
contains. In other words: It would allow us to fail the "backwards OTA" operation before the device
is bricked.

This would add value because:

* it would be highly useful for any application that persists state. For instance,
  applications maintaining a sqlite database, which has a schema that could change over time.
* specifically, it would be highly useful for the storage team, since they have in the past had to
  invest a lot of time into triaging issues that were ultimately caused by reverse-OTAing across a
  version boundary.
* this would reinforce that Fuchsia does not support backwards OTAs; they are strictly best effort.

It's important to note that this proposal doesn't change which OTA sequences are supported and which
are not. It just makes this support explicit. **The main purpose of the OTA backstop is to prevent
developer devices from being put into invalid states.** For production devices, the
no-backwards-OTAs invariant should be primarily enforced by release management.

Without this proposal, attempting to backwards-OTA across an incompatible boundary will cause
problems when developers attempt to boot the device (e.g. the filesystem format might not be
supported by the driver). With this proposal, developers will find out about this before they do the
OTA (and the error will be much clearer), which is a better developer experience.

## Background

### Terminology

An _OTA_ is a mechanism for upgrading the underlying operating system. Fuchsia devices can receive
and install OTA updates to system and application software.

A _Stepping Stone_ build is a build that cannot be skipped over in OTAs. For example, consider three
sequential releases A, B, and C. Traditionally, we'd need to support OTAs from `A->B`, `B->C`, and
`A->C`. If we declare `B` as a stepping stone release, this removes the `A->C` edge, so the only way
for A to upgrade to C is to OTA `A->B` then `B->C`. In practice, this is useful for risky migrations
and for reducing the number of forward OTAs we need to test.

### How the OTA backstop relates to stepping stones

The OTA backstop and stepping-stone releases are both primitives that we have to do safe migrations
(for example, storage format migrations). The exact playbook for how the OTA backstop and
stepping-stone releases should be used is out of scope of this RFC. Instead, here we provide an
example of how these primitives may be used to support a safe migration.

Consider a storage format migration. The steps we might take are:

1. Add support for the new format, but don't enable/migrate it yet. Bump the OTA backstop.
2. Wait some time.
3. Enable the new format with one of the above migration strategies.

For cases where we do actually migrate devices, we have two further steps we can take to enable
cleanup:

4. Cut a stepping-stone release that includes (3).
5. Remove the migration code and support for the old format.

The stepping stone release allows us to assume that devices will have gone through a build that has
the migration code, and thus we can remove read support for the old format going forward.

Bumping the OTA backstop in (1) ensures that devices don't downgrade to a version that doesn't have
support for the new format.

## Policy for bumping the backstop

The backstop should be bumped one-off as needed. The vast majority of changes should not require
backstop bumps. If this RFC is approved, an official playbook doc should be published to describe
specific steps for bumping the backstop. In the meantime, here we propose a high level overview of
this policy.

When proposing a CL to bump the backstop, authors should:

* Provide a link to an issue on bugs.fuchsia.dev which describes why the bump is necessary and how
  developers can proceed if they absolutely need to downgrade their device across the backstop
  (e.g. the answer is probably flash or pave).
* Obtain [//src/sys/pkg/OWNERS](/src/sys/pkg/OWNERS) approval.

## Design

Let's introduce an `epoch.json` file to be present both in the [update package](concepts/
packages/update_pkg.md) and on the system. It should be a JSON file with two string keys:

* "version", which should have a single string value for the `epoch.json` schema version. In
  practice, this will not be checked when performing updates -- this key only exists to make it
  obvious when `epoch.json` schema changes are made in production.
* "epoch", which should have a single integer value for the OTA backstop. If the epoch of the update
  package < epoch of system, we should fail OTAs in the prepare phase with `UNSUPPORTED_DOWNGRADE`.

For example, `epoch.json` may look like:

```json
{
  "version": "1",
  "epoch": 5
}
```

In order to safely bump the epoch, let's also introduce an `epoch_history` file that gets compiled
into `epoch.json` via the build system. The `epoch_history` file could be in the form:

```
0=Initial epoch (https://fxbug.dev/66089)
1=Storage format migration (https://fxbug.dev/XXXXX)
...
N=Most recent change (https://fxbug.dev/YYYYY)
```

The `epoch_history` file should be manually bumped each time a backwards incompatabile change is
introduced.

While the intermediary `epoch_history` file adds another layer of complexity, this approach is
advantageous because:

* It provides a log of all version bump changes (forced documentation!)
* It produces a merge conflict if two people try to bump the epoch for different reasons.

## Implementation

The changes will occur entirely in the platform (specifically, the system update stack).

In order to land the change, we need to:

* Add `epoch_history` to //src/sys/pkg/bin/system-updater.
  * Also, make a script that converts `epoch_history` to `epoch.json`.
  * Have the build system use this script to add `epoch.json` to the system-updater's out directory.
* Modify the [BUILD](https://cs.opensource.google/fuchsia/fuchsia/+/main:build/resources/BUILD.gn;l=2
  074;drc=2f584c4a62374f37361ac04875e60b5459fcc3b5) so that `epoch.json` also gets put into
  the update package.
* The system-updater should examine `epoch.json` at the end of the [Prepare](https://cs.opensource.g
  oogle/fuchsia/fuchsia/+/main:src/sys/pkg/bin/system-updater/src/update.rs;l=373;drc=91f4bd84db87
  4b5693f4f8040e4f5a39facc701b) phase.
  * If there is no `epoch.json` in the update package or there is a problem with deserializing it,
    assume epoch is 0. We deliberately ignore errors so that we can still OTA if the `epoch.json`
    schema changes.
  * If there is no `epoch.json` in system-updater's out directory or if there is a problem with
    deserializing it, fail because this is unexpected. Consider using the [`include_str`]
    (https://doc.rust-lang.org/std/macro.include_str.html) macro to read from the out directory.
  * If epoch in update package < epoch in system-updater, fail prepare with reason
    `UNSUPPORTED_DOWNGRADE`. We'll need to create a new [PrepareFailureReason](https://cs.opensource
    .google/fuchsia/fuchsia/+/main:src/sys/pkg/fidl/fuchsia.update.installer/progress.fidl;l=221;d
    rc=02b3415cbc6b0bc446bbd03571e17b823941faed) for `UNSUPPORTED_DOWNGRADE`.

## Security

This is not a security feature. However, it may interact with security features to improve developer
workflows. For example, consider a rollback protection feature that refuses to boot an image below
version `N`. If we increment the epoch when we land image version `N`, this will prevent developers
from downgrading an unbootable version because those downgrades will fail at the OTA backstop.

Beyond that, we choose to embed `epoch.json` in the system-updater binary (rather than in
config-data) to make OTAs resilient to config-data corruption.

## Privacy, and performance considerations

N/A

## Testing

We can use the existing system update testing framework in //src/sys/pkg, which is a mix of unit
and integration tests.

Additionally, the [OTA e2e tests](/src/sys/pkg/tests/system-tests/) will ensure both the backstop is
non-decreasing and in a valid format. For example:

* if build `N` lowers the OTA backstop, we'll fail in CI to OTA from build `N-1` to `N`.
* if build `N` produces an invalid `epoch.json` in the system-updater, we'll fail in CI to OTA from
* build `N` to `N'`.

## Documentation

We'll need to create a document to describe the policy for updating `epoch_history`.

Also, we'll need to modify:

* [Update package documentation](concepts/packages/update_pkg.md).
* OTA documentation (not yet posted on fuchsia.dev).

## Drawbacks, alternatives, and unknowns

### What are the costs of implementing this proposal?

The main cost of implementing this proposal is increased platform complexity, since we are adding
yet another version identifier to the platform.

### What other strategies might solve the same problem?

Another strategy is to officially support all backwards OTAs. This is impractical because we can't
write code resilient to future changes if we don't know what those changes are.

Another strategy is to explicitly prohibit _all_ backwards OTAs (even ones that would otherwise be
possible). For example, we could automatically bump the backstop on every new build. We decided not
to do this because in practice, some developers _do_ rely on these backwards OTAs and we'd like to
not break these developers.

Another approach might be to directly integrate with Fuchsia platform versioning (see
[RFC-0002](contribute/governance/rfcs/0002_platform_versioning.md)). However, there are
several ambiguous questions with this. For example, should all backwards OTAs across an API level be
prevented, or should we pick specific levels? Who would we break? Since there is precedent on
Fuchsia for using different version identifiers for different parts of the system (e.g. file
systems have their own version identifiers), it seems that would be a simpler option.

## Prior art and references

[Android](https://source.android.com/devices/tech/ota) has more info on OTAs.

## Acknowledgements

James Sullivan contributed to the motiviation and stepping stone sections. Zach Kirschenbaum wrote
the original design doc, which was reviewed by Dan Johnson.
