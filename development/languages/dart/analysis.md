# Analysis


Analysis is run as part of the Fuchsia build.

For each `dart_library` target, an analysis script gets
also generated in the output directory under:
```sh
out/<build-type>/gen/path/to/package/package.analyzer.sh
```
Running this script will perform an analysis of the target's sources.
Note that other templates usually define a Dart library they build upon. For
example, a `flutter_app` `//foo/bar` will yield a `//foo/bar:bar_dart_library`
target which can also be analyzed.

As with standard Dart packages, analysis options are defined in an
`analysis_options.yaml` file, which must be placed at the package root.
This file may refer to a common set of options by way of an `include` directive:
```
include: relative/path/to/options.file
```
A canonical set is available at `//topaz/tools/analysis_options.yaml`.
It is customary to merely include that set from a local options file:
```
include: path/to/topaz/tools/analysis_options.yaml
```

Analysis may be disabled altogether for a given target with:
```
dart_library("foo") {
  disable_analysis = true
}
```

The `//scripts/run-dart-action.py` script makes it easy to run the analysis over
multiple targets:
```sh
scripts/run-dart-action.py analyze --out out/<build-type> --tree //topaz/shell/*
```

Regular analyzer flags may also be passed:
```sh
scripts/run-dart-action.py analyze --out out/<build-type> --fatal-warnings --lints
```
This holds true for the individual analysis scripts.
