Dart style guide
================

The Fuchsia project follows the guidelines in [Effective Dart][effective-dart],
but with some additions.

All code must be formatted using `dart format` before being checked in.

# Additional Style Rules

### DON'T follow the Flutter repository style guide.

The Flutter project's style guide is meant to be used for code in the Flutter
framework itself. It is not intended as style guidance for projects using
Flutter. All code in the Fuchsia repository should follow the standard
Dart style. Although we are not following the style, the Flutter's guide on
documentation and development is still useful.

### DO use trailing commas on all tree structures longer than one line.

Without trailing commas, code that builds widget trees or similar types of code
tends to be hard to read. Adding the trailing commas allows `dart format` to do
its job correctly.

#### Without trailing commas:

``` dart
children.add(Expanded(
  child: Center(
      child: Container(
          width: 64.0, height: 64.0, child: FuchsiaSpinner())),
));
```

#### With trailing commas:

``` dart
children.add(Expanded(
  child: Center(
      child: Container(
        width: 64.0,
        height: 64.0,
        child: FuchsiaSpinner(),
      ),
   ),
));
```

### DO order members using the Dart Analyzer.
In Visual Studio Code, this is the Dart: Organize Members command available
in the Command Palette. (Control+Shift+P or View -> Command Palette)

This formatter doesn’t appear to be available outside of the supported IDEs.

### PREFER to keep lines below 80 characters unless it would be more readable.
This is a slight amendment from the general Dart [rule][dartstyle-80-chars].
Unlike that rule, it is fine to have lines above 80 characters in the Fuchsia
repository, as long as it improves readability, and `dart format` won't
automatically truncate the line.

# Additional Usage Rules

## Repositories and Files

### DO prefix library names in `/lib` and `/public/lib` with `lib.`
#### Example:

```
Dart_library("lib.settings") {
  package_name = "lib.settings"
  ...
}
```

### PREFER minimizing the number of public members exposed in a package.
This can be done by only making things public when needed, and keeping all
implementation detail libraries in the `/src` directory. Assume anything
public in the `lib` directory will be re-used.

### CONSIDER exporting publicly visible classes in a single `.dart` file.

For multiple classes that are used together but are in different files,
it’s more convenient for users of your library to import a single file rather
many at once. If the user wants narrower imports they can always restrict
visibility using the `show` keyword.

This also helps minimize the publicly visible surface.

Example:

``` dart
/// In src/apple.dart
class Apple {}

/// In src/orange.dart
class Orange {}

/// In src/veggies.dart
class Potato {}
class Tomato {}

/// In botanical_fruits.dart
export 'src/apple.dart';
export 'src/orange.dart';
// Can also be: export 'src/veggies.dart' hide Potato;
export 'src/veggies.dart' show Tomato;

/// In squeezer.dart
import 'package:plants/botanical_fruits.dart' show Orange;

```

### DO import all files within a package using relative paths.

Mixing and matching relative and absolute paths within a single package
causes Dart to act as if there were two separate imports of identical files,
which will introduce errors in typechecking. Either format works as long as
it is consistent. Within the Fuchsia repository, relative paths are used.

This does not apply to external libraries, as only the absolute path can be
used.

#### Good:

``` dart
import 'access_point.dart';
```

#### Bad:

``` dart
import 'package:wifi/access_point.dart';
```

### DO use namespacing when you import FIDL packages.

This adds clarity and readability. FIDL namespaces (library statements) are not
respected in Dart (e.g. `fuchsia.io.Node` becomes `Node`).
Because of tight namespaces, people tend to use more generic names in FIDL
(Error, File, Node, etc.), which result in more collisions/ambiguity in Dart.

#### Good:

``` dart
import 'package:fidl_fuchsia_file/fidl.dart' as file_fidl;
...

file_fidl.File.get(...) ...
```

#### Bad:

``` dart
import 'package:fidl_fuchsia_file/fidl.dart';
...

File.get(...) ...
```

### DO use namespacing when there is ambiguity, e.g. in class names.

There are often functions or classes that can collide, e.g. `File` or `Image`.
If you don't namespace, there will be a compile error.

#### Good:

``` dart
import 'dart:ui' as ui;

import 'package:flutter/material.dart';
...

ui.Image(...) ...
```

#### Bad:

``` dart
import 'dart:ui';

import 'package:flutter/material.dart';
...

Image(...) ... // Which Image is this?
```

### PREFER to use `show` if you only have a few imports from that package. Otherwise, use `as`.

Using `show` can avoid collisions without requiring you to prepend
namespaces to types, leading to cleaner code.

#### Good:

``` dart
import 'package:fancy_style_guide/style.dart' as style;
import 'package:flutter/material.dart';
import 'package:math/simple_functions.dart' show Addition, Subtraction;
```

#### Bad:

``` dart
import 'package:flutter/material.dart show Container, Row, Column, Padding,
  Expanded, ...;
```

## Coding practices

### DON'T use `new` or use `const` redundantly.

Dart 2 makes the `new` optional for constructors, with an aim at removing them
in time. The `const` keyword is also optional where it can be inferred by the
compiler.

`const` can be inferred in:

* A const collection literal.
* A const constructor call.
* A metadata annotation.
* The initializer for a const variable declaration.
* A switch case expression&mdash;the part right after case before the :, not
  the body of the case.

This guidance will eventually be part of Effective Dart due to the changes for
Dart 2.

#### Good:

``` dart
final foo = Foo();
const foo = Foo();
const foo = const <Widget>[A(), B()];
const Foo(): bar = Bar();
```

#### Bad:

``` dart
final foo = new Foo();
const foo = const Foo();
foo = const [const A(), const B()];
const Foo(): bar = const Bar();
```

### DON'T do useful work in assert statements.

Code inside assert statements are not executed in production code. Asserts
should only check conditions and be side-effect free.

### PREFER to use `const` over `final` over `var`.

This minimizes the mutability for each member or local variable.

### PREFER return `Widget` instead of a specific type of Flutter widget.

As your project evolves, you may change the widget type that is returned in your
function. For example, you might wrap your widget with a Center. Returning
`Widget` simplifies the refactoring, as the method signature wouldn't have to
change.

#### Good:

``` dart
Widget returnContainerWidget() {
  return Container();
}
```

#### Bad:

``` dart
Container returnContainerWidget() {
  return Container();
}
```

# Additional Design Rules

### PREFER storing state in Models instead of state.

When storing state that Flutter widgets need to access, prefer to use
`ScopedModel` and `ScopedModelDescendant` instead of `StatefulWidget`.
A `StatefulWidget` should contain only internal widget state that can be lost
without any consequences.

Examples of stuff to store in a `ScopedModel`:

* User selections
* App state
* Anything that needs to be shared by widgets

Examples of stuff to store in a `StatefulWidget`'s `State`:

* Animation state that doesn't need to be controlled

### AVOID mixing named and positional parameters.

Instead, `@required` should be used in place of required positional parameters.

### PREFER named parameters.

In most situations, named parameters are less error prone and easier to read
than positional parameters, optional or not. They give users to pass in the
parameters in whatever order they please, and make Flutter trees especially
clearer.

In the Fuchsia repository, positional parameters should be reserved for simple
operational functions with only a few parameters.

#### Good:

``` dart
int add(int a, int b);
int addNumbers(int a, [int b, int c, int d]);
Foo fromJson(String json);
void load(String file);

Widget buildButton({
  @required Widget child,
  VoidCallback onTap,
  double width,
  bool isDisabled = false,
});
```

#### Bad:

``` dart
int add({int a, int b});
Foo fromJson({@required String json});

Widget buildButton(
  Widget child,
  VoidCallback onTap, [
  double width,
  bool isDisabled = false,
]);
```

### DO add [logging statements][dart-logging]

[effective-dart]: https://www.dartlang.org/guides/language/effective-dart
[dart-logging]: /docs/development/languages/dart/logging.md
[dartstyle-80-chars]: https://www.dartlang.org/guides/language/effective-dart/style#avoid-lines-longer-than-80-characters
