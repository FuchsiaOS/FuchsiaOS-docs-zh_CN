# Third-party Dart packages


There are two types of third-party dependencies in the Fuchsia tree:
- extracted from [pub][pub];
- sync'd from Git.

## Pub dependencies

Pub dependencies are host at [`//third-party/dart-pkg`][dart-3p]. That project
is regularly kept up-to-date with [a script][dart-3p-script] that relies on the
`pub` tool to resolve versions and fetch sources for the packages that are used
in the tree.
This script uses a set of canonical local packages which are assumed to be
providing the necessary package coverage for the entire tree.

Additionally, projects may request third-party dependencies to be imported
through the following procedure:
1. create a `dart_dependencies.yaml` file in the project
2. add the desired dependencies in that file:
```
name: my_project
dependencies:
  foo: ^4.0.0
  bar: >=0.1.0
```
3. add a reference to the file in `//scripts/dart/update_3p_packages.py`
4. run that script
5. merge your changes to `dart_dependencies.yaml` to master
6. merge the files downloaded by running the 'update_3p_packages.py' script, and the script itself, to master.
7. in the '//topaz/manifest/dart' manifest, update the project node 'third_part/dart-pkg' revision attribute with the SHA from your commit in step 6.
8. merge your change to the '//topaz/manifest/dart' manifest file to master.


[pub]: https://pub.dartlang.org/ "Pub"
[dart-3p]: https://fuchsia.googlesource.com/third_party/dart-pkg/+/master "Third-party dependencies"
[dart-3p-script]: https://fuchsia.googlesource.com/scripts/+/master/dart/update_3p_packages.py "Dependencies script"
