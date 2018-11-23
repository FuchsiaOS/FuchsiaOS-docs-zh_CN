# FIDL


[FIDL targets][fidl] generate implicit Dart bindings targets. To use the
bindings generated for:
```
//foo/bar
//foo/bar:blah
```
add a dependency on:
```
//foo/bar:bar_dart
//foo/bar:blah_dart
```
and import the resulting Dart sources with:
```
import "package:foo.bar/baz.dart";
import "package:foo.bar..blah/baz.dart";
```


## Known issues

### Multiple FIDL targets in a single BUILD file

If two FIDL targets coexist in a single BUILD file:

* their respective, generated files will currently be placed in the same
  subdirectory of the output directory.  This means that files belonging to one
  target will be available to clients of the other target, and this will likely
  confuse the analyzer.  This should not be a build issue now but could become
  one once the generated Dart files are placed in separate directories if
  clients do not correctly set up their dependencies.
* depending on one of these targets from *another* FIDL target that is used by
  a Dart package leads to a `Unable to read Dart source ...` error. The
  bindings generator for FIDL builds Dart package names based on the directory
  structure containing the included FIDL file, while GN (used to compute
  dependencies for the Dart package) does so using the full GN target name. For
  example: depending on `lib/foo/fidl:bar` generates a package like
  `lib.foo.fidl._bar`. Depending on the top-level target `lib/foo/fidl`
  generates the package `lib.foo.fidl`, which coincides with the Dart FIDL
  binding's assumptions.


[fidl]: https://fuchsia.googlesource.com/build/+/master/fidl/fidl.gni "FIDL"
