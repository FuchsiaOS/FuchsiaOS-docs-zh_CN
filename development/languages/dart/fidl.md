# FIDL


[FIDL targets][fidl] generate implicit Dart bindings targets. To use the
bindings generated for:

```
//foo/bar
//foo/bar:blah
```

add a dependencies in BUILD.gn:

```
deps = [
   ...
   "//foo/bar",
   "//foo/bar:blah",
   ...
]
```

There are 3 files generated for dart from FIDL.  These are found in
`out/default/dartlang/gen/<path-to-target>/<fidl-servicename>_package/lib`

* fidl.dart - the synchronous bindings
* fidl_async.dart - the asynchronous bindings
* fidl_test.dart - the stubbed out implementation of the service.


```dart
import "package:fidl_foo_bar/fidl.dart";
import "package:fidl_foo_bar_blah/fidl_async.dart";
```


## Known issues

### Multiple FIDL targets in a single BUILD file

If two FIDL targets coexist in a single BUILD file:

* Their respective, generated files will currently be placed in the same
  subdirectory of the output directory.  This means that files belonging to one
  target will be available to clients of the other target, and this will likely
  confuse the analyzer.  This should not be a build issue now but could become
  one once the generated Dart files are placed in separate directories if
  clients do not correctly set up their dependencies.
* Depending on one of these targets from *another* FIDL target that is used by
  a Dart package leads to a `Unable to read Dart source ...` error. The
  bindings generator for FIDL builds Dart package names based on the directory
  structure containing the included FIDL file, while GN (used to compute
  dependencies for the Dart package) does so using the full GN target name. For
  example: depending on `lib/foo/fidl:bar` generates a package like
  `lib.foo.fidl._bar`. Depending on the top-level target `lib/foo/fidl`
  generates the package `lib.foo.fidl`, which coincides with the Dart FIDL
  binding's assumptions.
  
## Calling a FIDL service

The generated bindings for Dart require the importing of fuchsia_services.


```dart
import 'package:fuchsia_services/services.dart';
```


In order to use the Launcher service to start services that implement a FIDL interface,
you need to have the `fuchsia.sys.Launcher` service declared in the .cmx


[fidl]: /build/fidl/fidl.gni "FIDL"
