# Dart null safety

## Goal & motivation

Fuchsia's Dart build [transitioned][fxr583201] from `dartanalyzer` to `dart
analyze` in 2021. `dart analyze` supports checking
[sound null safety][dart-null-safety]{: .external}. This means runtime
null-dereference errors can be turned into edit-time analysis errors.

Dart code written before Fuchsia code was checked for null safety needs to be
migrated to take advantage of the null safety check. We would like to migrate
all Dart sources to support null safety, since this will eliminate a whole class
of runtime errors in Dart and Flutter applications.

## Technical background

With null safety, all of the variables in the following code are non-nullable:

```dart
// In null-safe Dart, none of these can ever be null.
var i = 42; // Inferred to be an int.
String name = getFileName();
final b = Foo();
```

To indicate that a variable might have the value null, just add `?` to its type
declaration:

```dart
int? aNullableInt = null;
```

See Dart's
["Understanding null safety"][dart-understanding-null-safety]{: .external} for
more details.

## How to help

### Picking a task

Search for Dart source files containing `// @dart=2.9` or
`TODO(https://fxbug.dev/84961)`, all these files need to be migrated. For
example you can code search for
[`"// @dart=2.9" lang:dart`][dart_2.9_codesearch].

### Doing a task

NOTE: You may see `import_of_legacy_library_into_null_safe` from Dart analysis.
This means one of the dependencies does not support null safety yet, and should
be migrated first. If this happens to be a third-party dependency, we
unfortunately have to
[wait for them to migrate first][wait-to-migrate]{: .external}.

Make sure the Dart sources you are migrating are
[included in your build](/development/build/fx.md#configure-a-build).
Remove `// @dart=2.9` and the `TODO` line above it, rebuild, then follow errors
and suggestions from Dart analysis. See
[Dart's migration guide][dart-migration-guide]{: .external} for more suggestions.

After you are done fixing all the errors from Dart analysis, add `null_safety =
true` to the build target in the corresponding `BUILD.gn` file containing the
sources you migrated.

### Completing a task

When your change is ready, send it out for the corresponding owners for review.
Preferable you can create a bug in Monorail to track the progress. If you do,
make sure to mark it blocking the [main bug 84961][fxb84961]{: .external}, and
close it after your change has merged.

## Examples

*   [`[cts][net] Enable null-safety`](https://fuchsia-review.googlesource.com/c/fuchsia/+/584343)
*   [`[examples] Enable null safety check`](https://fuchsia-review.googlesource.com/c/fuchsia/+/626781)

## Sponsors

Reach out for questions or for status updates:

*   <chaselatta@google.com>
*   <jayzhuang@google.com>

[dart-migration-guide]: https://dart.dev/null-safety/migration-guide#step2-migrate
[dart-null-safety]: https://dart.dev/null-safety
[dart-understanding-null-safety]: https://dart.dev/null-safety/understanding-null-safety
[dart_2.9_codesearch]: https://cs.opensource.google/search?q=%22%2F%2F%20@dart%3D2.9%22%20lang:dart&ss=fuchsia%2Ffuchsia
[fxb84961]: https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=84961
[fxr583201]: https://fuchsia-review.googlesource.com/c/fuchsia/+/583201
[wait-to-migrate]: https://dart.dev/null-safety/migration-guide#step1-wait
