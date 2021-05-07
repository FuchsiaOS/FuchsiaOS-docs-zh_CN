# Flutter-on-Fuchsia Velocity

 * Project lead: dworsham@google.com
 * Area(s): Flutter

## Problem statement

Fuchsia's Flutter integration is currently languishing under technical debt and
out-of-tree workflows with sharp edges.  These two factors make it difficult
for existing or new engineers to contribute to Flutter-on-Fuchsia in a
meaningful way.  They have also led the Flutter team to largely abandon
maintenance of Flutter-on-Fuchsia and transfer ownership of that task back to
the Fuchsia team.

The out-of-tree location of the Fuchsia-Flutter code (known as the
`flutter_runner`) combined with the sequence of rollers between the upstream
repository and GI means that changes to the Dart VM or Flutter engine often do
not show up in GI for multiple weeks after they are introduced upstream.

Ultimately it is difficult to make changes to the `flutter_runner` right now
and mistakes are very costly due to roll times. Since several products use
flutter for the primary shell, any bugs have a high user-facing cost and
sometimes can't be fixed for weeks.

## Solution statement

Flutter on Fuchsia will migrate to become a Custom Flutter Engine Embedder
using the well-defined Embedder API (and ABI) created by the Flutter team.  As
part of this we will remove the native VM hooks for `dart:zircon` and
`dart:fuchsia` and reimplement these packages using `dart:ffi` (because the
embedder API does not allow for custom native hooks).

This refactoring will allow the Fuchsia-specific code to be maintained
independently of the core flutter engine code, with the 2 being separated by a
well-defined ABI.  In the process of disentangling the Fuchsia-specific code
from the code flutter engineer code, it also removes a lot of technical debt
(~8kLOC of legacy code scheduled for deletion).

## Dependencies

Moving the flutter embedder code into `//sdk` will be a large workflow change
for Fuchsia engineers working on Flutter.  Those engineers will no longer
submit changes to `//flutter/shell/platform/fuchsia` in the flutter engine
tree, but instead will submit those changes into the fuchsia tree.

The `flutter_runner_unittests` are moved in-tree as well, so they run on
fuchsia CI instead.  These tests are small and self-contained so moving them is
not expected to be problematic.

Finally, engineers working in the flutter embedder code will be limited to only
the APIs exposed by the Flutter team in `embedder.h`

## Risks and mitigations

 * Risk: `dart:zircon` and `dart:fuchsia` migration to FFI might encounter
   unexpected hurdles
 * Risk: lack of test coverage over the code that is being transformed for this
   task

 * Mitigation: Prototype an FFI-based `dart:zircon` API and verify it works
 * Mitigation: Improve test coverage
 * Mitigation: Can (and did) migrate the legacy render code (in `flow/`)
   without migrating `dart:zircon` and `dart:fuchsia`
