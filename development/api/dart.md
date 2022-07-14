# Dart API Readability Rubric

[TOC]

## Overview

This document describes heuristics and rules for writing Dart libraries that are
published in the Fuchsia SDK.

Unless otherwise specified, Fuchsia library authors should adhere to all the
heuristics and rules recommended by the Dart team itself under [Effective
Dart](https://www.dartlang.org/guides/language/effective-dart). Author’s should
familiarize themselves with all sections,
[Style](https://www.dartlang.org/guides/language/effective-dart/style),
[Documentation](https://www.dartlang.org/guides/language/effective-dart/documentation),
[Usage](https://www.dartlang.org/guides/language/effective-dart/usage) and
[Design](https://www.dartlang.org/guides/language/effective-dart/design) prior
to reading this rubric.

### Terminology

There are some terms of art that Dart uses that conflict with Fuchsia’s
terminology.

- [Fuchsia
  package](/src/sys/pkg/bin/pm/README.md#structure-of-a-fuchsia-package): A
  Fuchsia package is one or more collections of files that provide one or more
  programs, components or services for a Fuchsia system.
- Fuchsia library: An informal definition for implementation code used by
  Fuchsia, usually found in lib or lib/src directories. Libraries are a
  convention, most policies for libraries are enforced socially or fallback to
  language specific approaches and tooling.
- [Dart package](https://dart.dev/guides/packages): The Dart package system is
  used to share software like libraries and tools within the Dart ecosystem,
  e.g. via Pub. Often a package is a collection of files with a minimum of a
  pubspec.yaml file and at least one Dart file, in-tree Dart packages will also
  have a `BUILD.gn` file.
- [Dart library](https://dart.dev/tools/pub/package-layout#public-libraries): A
  collection of Dart code (classes, constants, typedefs, etc.) isolated to a
  single namespace and corresponding to a single entry point, e.g. `import
  'package:enchilada/enchilada.dart';` imports the enchilada library. Note that
  Dart libraries have a privacy boundary, e.g. private implementation details
  are not visible or accessible outside of the library. A Dart package can
  contain multiple Dart Libraries.

When writing Dart code it is important to understand the distinction in
terminology in order to remain clear when communicating with team members whose
primary language might be one of the other supported languages (C++, Rust,
etc.).

- A Fuchsia package can contain components implemented as Dart binaries.
- A Dart binary is defined within a Dart package, and often has dependencies on
  other Dart packages.
- Code shared as a library in Fuchsia’s tree written in Dart is implemented as a
  Dart package.

### Focus on the Interfaces

Public classes should expose a clean user interface that clearly describes the
API surface and is free from internal implementation details. Classes that
contain more than a minimal amount of functionality should expose their API in
an abstract class with the implementation inside a private implementation file.
Doing so allows for the users of the classes to focus on the public methods and
forces the implementer to think about the usage of the class before
implementation.

### Consider Composability

When designing the API consider how it will fit into the larger Dart ecosystem
of libraries. For example, if writing an API that delivers events, consider
using Streams instead of callbacks because they compose better with libraries
like Flutter.

## Lint Rules

Dart code written against the Fuchsia SDK should pass all the lint rules
specified by the
[analysis_options.yaml](https://fuchsia.googlesource.com/topaz/+/HEAD/tools/analysis_options.yaml)
file, which lives in the topaz repository. These lint rules will help to automate
the review API review process. There are situations where a lint rule may be in
conflict with a specific API and may need to be explicitly ignored. If a file is
opting out of a lint rule the developer must provide a comment explaining the
reasoning for opting out of the lint rule.

## Library Structure

When organizing the structure of a Dart package it is important to follow the
recommendations laid out by the [Effective
Dart](https://www.dartlang.org/guides/language/effective-dart) style guide.
Additionally, developers should consider how their code is exported. For a more
complicated package, developers should avoid a singular catch all top-level
export file and rather expose a top level file per logical grouping of classes
that make sense to be pulled under one import line. This allows users of the
library the ability to have finer grained control over which sections of the
library they import.

## Comments/Documentation

All comments should adhere to [Effective Dart:
Documentation](https://www.dartlang.org/guides/language/effective-dart/documentation)
as well as the [Fuchsia Documentation](documentation.md) guide.

## Dependencies

Packages written for the Dart Fuchsia SDK should not take on third party
dependencies that are not themselves also in the Fuchsia SDK. Exceptions will be
made for the following, well established, dependencies that are likely to be
present in all environments. Any packages that should be added to this list
must be approved by the [API
Council](/contribute/governance/api_council.md).

- [logger](https://pub.dev/packages/logging)
- [meta](https://pub.dev/packages/meta)
- [intl](https://pub.dev/packages/intl)
- [flutter](https://flutter.dev/)


Packages that do take on external dependencies should consider whether they
want to reexport those symbols. If the dependency is reexported then the
generated documentation will generate documentation for the external dependency.
However, reexporting the dependency will create a tight coupling between package
versions.

## Formatting

Code should be formatted using `dart format`. This is an opinionated tool that
cannot be configured. Formatting all of our code with this tool will ensure
consistency. In Fuchsia, you can use `fx format-code` will run `dart format` on
all staged dart files.

## Files

- DO name files after their public class name
- PREFER placing each class into their own files, even if they’re private. It
  should be rare for multiple classes to live in the same file. Only private,
  small, simple and standalone classes can share a file with a public class.
- AVOID creating utility classes or libraries, these tend to turn into code
  dumping grounds. Instead, use precise naming that clearly communicates the
  purpose of the code being created.
- DON’T use the `part of` directive to avoid tight coupling of classes.

### Methods

- PREFER using named parameters vs positional parameters for public methods on
  public classes that have greater than 2 parameters. This aids code refactor
  and allowed adding extra parameters without breaking the public API contract.
- AVOID using functions that can do more than one thing like `void
  updateAndCommit();` but prefer explicit naming.

### Constructors

- PREFER using named parameters with Constructors that have more than two
  parameters.
- DO use the meta package to indicate which parameters are required.
- DO assert on required parameters.
- DO throw exceptions/errors for public APIs that will have detrimental side
  effects if invalid input is passed to constructors, since asserts do not run
  in release builds.

```
/// Constructs a [Car] object
///
/// If [id] is not provided one will be
/// generated with a UUID4 format.
Car({
  @required this.make,
  @required this.model,
  this.id,
}) : assert(make != null),
     assert(model != null);
```

## Naming
If a method will use a cached object, or create it if it doesn’t exist, avoid
introducing or into the name.

```
class Node {
  //BAD
  Node getOrCreateChild(String name);
              
  //GOOD
  Node child(String name);
}
```

When adding a function or interface that will have methods invoked in response
to another action, name the methods add<NAME>Listener() and
remove<NAME>Listener(). The objects that implement the <NAME>Listener interface
should name the invoked methods on<EVENT>.

```
class MediaController {
  void addMediaListener(MediaListener listener) {}
  void removeMediaListener(MediaListener listener) {}
}

abstract class MediaListener {
  void onPause();
  void onPlay();
}
```

When appending an item to your object prefer the name add<Name> instead of
append to follow the dart list naming.

When deciding between using a single member abstract or a plain `Function` as a
`Listener` object consider how your API might evolve over time. If you expect
that you may add more methods to the listener use a single member abstract to
allow for the evolution but if the API is not likely to change use a plain
function.

```
// This could logically grow to include an onDoubleTap()
// method so it makes sense to use a single member abstract.
abstract class TapListener {
  void onTap();
}
void addTapListener(TapListener listener) { ... } 

// This will likely never need more methods so it can 
// clearly take a function type.
void addOnCloseListener(void Function() listener) { ... }
```

## Preferred Types
Concrete data types should be used instead of lower level primitives. The
following types should be used when possible:

- [Duration](https://api.dartlang.org/stable/2.4.0/dart-core/Duration-class.html)
  when working with a span of time.
- [DateTime](https://api.dartlang.org/stable/2.4.0/dart-core/DateTime-class.html)
  when working with dates.

If there is not a concrete type that can be used to represent your object at a
higher level your API should expose one. For example, if we had an API that
dealt with currency we would create a `Currency` data type instead of working
with `num` types.

```
// BAD
int getCash() { ... }

// GOOD
Currency getCash() { ... }
```

Your API should avoid returning unstructured JSON data but rather transform any
JSON into a typed value.

```
// BAD
Map<String, dynamic> getCar() => {
    'make': 'Toyota',
    'year': 2019, 
}

// Good
Car getCar() => Car(make: 'Toyota', year: 2019);
```

## Internationalization
If a package exposes a user visible string the string should be
internationalized. In the absence of an ability to internationalize a user
visible string the API should return data in which a user of a library can
construct an internationalized string.

Exceptions and log messages do not need to be internationalized if they are not
intended to be user visible.

## Error Handling
All error handling should adhere to [Effective Dart: Error
handling](https://www.dartlang.org/guides/language/effective-dart/usage#error-handling).


## Error vs. Exception
Error and its subclasses are for programmatic errors that shouldn’t be
explicitly caught. An Error indicates a bug in your code, it should unwind the
entire call stack, halt the program, and print a stack trace so you can locate
and fix the bug.

Non-Error exception classes are for runtime errors. If your API implementation
throws an exception, it should be documented as part of the public API and it’s
expected behavior. This will facilitate programmatic handling of the exception
by API clients.

Except in a few special circumstances, idiomatic Dart should throw Errors, but
never catch them. They exist specifically to not be caught so that they take
down the app and alert the programmer to the location of the bug.

Note: often times people refer to Error when they mean Exception and vice versa.
Especially developers that are coming from a different language. Apply your
knowledge of their difference when developing your Dart API.

Your public API should throw well defined and typed exceptions so that users can
catch them and react appropriately. If you are not in control of all the code
that is being called by your package, maybe because you are using a third party
library, you may not be able to know exactly which exceptions may be thrown. If
this is the case, you can either attempt to catch the exception and wrap it in a
type that you create or clearly document that an exception of unknown type may
be thrown.

If your API can fail in more than one way the exception should clearly indicate
the failure method. Consider throwing different types of exceptions or adding a
code to the exception so the caller can respond appropriately. Also, don’t
forget to publicly document all the exceptions that are potentially thrown by a
given method.

```
enum ErrorCode { foo, bar }

class MyException implements Exception {
  final ErrorCode code;
  MyException(this.code);
}

/// Throws MyException(ErrorCode.foo) if condition is true or
/// throws MyException(ErrorCode.bar) if not 
void baz(bool condition) {
  If (condition) {
    throw MyException(ErrorCode.foo);
  } else {
    throw MyException(ErrorCode.bar);
  }
}
```

### Assertions vs. Exceptions
Assertions should only be used to verify conditions that should be logically
impossible to be false due to programmer error, not user or data input. These
conditions should only be based on inputs generated by your own code. Any checks
based on external inputs should use exceptions.

Use asserts when you are in full control of the inputs. For example verify
private functions' arguments with asserts, and using exceptions for public
functions arguments.

In Dart all assertions are compiled out from the production/release builds.
Therefore, your program must work just as well when all assertions are removed.
Do not directly assert on a value returned directly from a function as this can
cause the code to not be included in release build since the entire body of the
assert is removed in release builds.

```
// BAD
assert(foo()); // foo is not executed

// GOOD
final success = foo();
assert(success);
```

### FIDL Exception Handling
In Fuchsia, the generated Dart FIDL bindings are always asynchronous, thus all
methods return a `Future` even if there is no return value (`Future<void>` is
used). Also, when connecting to a particular service the connection is assumed
to be successful even though it can fail to connect or disconnect in the future.
For these reasons, the caller of any FIDL api should always assume that a
specific call can fail and handle that appropriately when needed.

```
final _proxy = fidl_myService.MyServiceProxy();
connectToService('fuchsia-pkg://fuchsia.com/my_service#meta/my_service.cmx', _proxy);

_proxy
  .doSomething()
  .catchError((e, s) {
    // handle the error if needed
  });
```

## Testing
Please review [Dart](https://www.dartlang.org/guides/testing) and
[Flutter](https://flutter.dev/docs/testing) testing guides.

- DO test for `Future<T>` when disambiguating a `FutureOr<T>` whose type
  argument could be Object.
- DON’T use `@visibleForTesting` on public API.

The API surface of your package should be well tested. However, the public API
should not need to leak internal details for the class to be testable. Consider
the following example:

```
class Foo {
  // services is exposed for testing
  Foo({GlobalServices services = GlobalServices()}) { … }

  // Connects to the global service with the given name.
  Connection connectToGlobalService(String name) {
     return services.connect(name);
  }
}
```

Rather, consider writing your class as an abstract class so the user does not
need to know about the injection of global services but tests can directly
inject the global services into the implementation. These avoids leaking
implementation details to the user and provides an API that the user cannot
abuse or mess up. This has the added advantage of allowing the API to evolve if
the GlobalServices class evolves without having to change the callers of the
method.

```
// foo.dart
abstract class Foo {
  factory Foo() => FooImpl(services: GlobalServices);
  connectToGlobalService(String name);
}

// internal/foo_impl.dart
class FooImpl implements Foo {
  FooImpl({GlobalServices services}) { … }

  // Connects to the global service with the given name.
  Connection connectToGlobalService(String name) {
     return services.connect(name);
  }
}
```

Dart does not allow a private class/function to be accessed from within a test.
This has the effect that any private classes cannot be tested. This may be ok if
there is a corresponding public class/function that can exercise the private
members but this may not always be the case. In these situations it is best to
move the private class into its own file, which does not get exported by the
top-level export, and make it public. The tests can now access your private
members.

```
/// BAD - this code does not make _Taco directly testable

// dinner.dart
class Dinner {
  final _taco = _Taco();
  
  void eat() => _taco.consume();
}

// We have no way to directly test this class
class _Taco {
  void consume() {}
}

/// GOOD - this code makes Taco directly testable by moving it to its own private file

// dinner.dart
import '_taco.dart';

class Dinner {
  final _taco = Taco();
  void eat() => _taco.consume();
}

// _taco.dart 
class Taco {
  void consume() {}
}

// _taco_test.dart
import 'package:dinner/src/_taco.dart' // ignore: implementation_imports

void main() {
  test('taco consumption', () {
    expect(_Taco().consume(), runsNormally);
  });
}
```

## Design Patterns
### Disallowing Subclassing
It can be useful for a library to declare a common base class without allowing
developers to extend the common base class. The common pattern for supporting
this is to declare a private constructor on your public base class. This has the
effect of allowing subclasses within the same file to extend the base class
while not allowing users of your library to subclass the base class.

```
/// Base class
abstract class A {
  // private constructor disallows instantiation outside of this file
  ._();
  /// Concrete implementation of foo
  void foo() {}
}

/// A concrete implementation of [A]
class B extends A {
  () : super._();

  @override
  void foo() {
    // B implementation
    super.foo();
  }
}
```

It is important to note that this pattern does not restrict users from
subclassing the child class since it has a public constructor. If this
restriction is required see the factory constructors pattern below.

This pattern is useful if the implementation surface is small since the pattern
requires all of the subclasses to live in the same file as the base class or to
use the part of directive, which is discouraged. If the surface area is too large
for a single file consider an alternate pattern.


### Factory Constructors

There are times when a user only needs to interact with a single interface but
which may have a different implementation depending on how the object was
constructed. Requiring the user to know about the different implementations can
add unnecessary APIs, and only serves to confuse the user. In this
situation you can define an abstract base class that defines the API surface
and create factory constructors that vend the appropriate private class.

```
// Publicly exported class
abstract class Foo {
  
  factory Foo() => FooImpl();

  factory Foo.withNamespace(String namespace) => NamespacedFoo(namespace);

  void update(String value);
  void revert();
}

// Private implementations not exported in public API
class FooImpl implements Foo {
  final _values = <String>[];

  @override
  void update(String name) => _values.add(name);

  @override
  void revert() => _values.removeLast();
}

class NamespacedFoo extends FooImpl {
  final String namespace;
  NamespacedFoo(this.namespace);

  @override
  void update(String name) => super.update('$namespace/$name');
}
```

Note: If you need to add the restriction that the base class cannot be extended
you can implement the pattern defined in Disallowing Subclassing that adds a
private constructor to the public base class

### Working with FIDLs

Try to make a clear distinction between regular object types and FIDL types.
This makes it easier for the maintainers of the code to identify FIDL types from
other types and take the necessary precautions when needed. Consider using the
as when importing a FIDL service and prefixing it with “fidl_”, this makes it
very to identify FIDL types across the entire file.

```
import 'package:fidl_fuchsia_foo/fidl_async.dart' as fidl_foo;

// now it is clear that the return type bar comes from fidl_foo
fidl_foo.Bar myMethod(String baz) {...} 
```

When subclassing FIDL types extend them so they can be interchanged with the
generated FIDL files. Usually, wrappers decorate the existing type with
additional functionality that compliments the original object. However, by
extending it from the original FIDL it allows the existing and new API to work
with original FIDL types instead of the more concrete types, which is useful when
interacting with other FIDLs or when developers are not using your wrapper.

### Decoupling implementation concerns

Try to avoid interfaces that cover multiple areas of concerns. By breaking down
the concerns users can have more flexibility with how they choose to combine the
interfaces and allows composed objects to be passed to methods with specific
concerns.

```
void main() {
  final restaurant = lookupRestaurant();
  map.display(restaurant);
  phone.call(restaurant);
}

abstract class Callable {
  String get phoneNumber;
}

abstract class Location {
  String get address;
  String get displayName;
}

class Restaurant implements Callable, Location {
  final String name;
  final String phoneNumber;
  final String address;
  String get displayName => name;

  Restaurant(this.name, this.phoneNumber, this.address);
}

class Map {
  void display(Location descriptor) {}
}

class Phone {
  void call(Callable callable) {}
}
```

### Iteration of Modifiable Collections
When exposing an API that can modify some sort of collection it is important to
protect against modifying the collection during iteration. When iterating over
an internal collection consider making a copy of the backing collection to
iterate. This will protect from exceptions being thrown for concurrent
modification of the underlying collection.

```
class Controller {
  final _listeners = <void Function()>[];
  void addListener(void Function() f) => _listeners.add(f);
  bool removeListener(void Function() f) => _listeners.remove(f);

  void notify() {
    // Make a copy to avoid modification of _listeners during iteration.
    for (final f in List.of(_listeners)) {
      // This method can safely call add/remove listeners
      f();
    }
  }    
}
```

## Anti Patterns

The following patterns should be avoided when writing Dart libraries for the
Fuchsia Dart SDK. Exposing Internal Details for Testing It may be tempting to
expose certain aspects of your API for testing concerns. However, doing so can
clutter your public interface and leak implementation details that the user
does not need to know about or may come to rely on. See the [Testing](#Testing)
section for more details

### Accepting/Returning dynamic Types

Dart provides a dynamic type for which the compiler will allow any type to be
passed to a function and returned from a function. This can be useful in some
situations like json encoding/decoding but in the general case it should be
avoided. Using dynamic types prevents the compiler from performing static type
checking at compile time and introduces hard to debug run-time errors.

In situations where an API might need to accept/return multiple input types
consider using generics or defining an interface that the object implements
instead. In situations where this will not work, consider defining multiple
methods that call through to the private dynamic accepting function.

### Using Private Methods Across Files

Dart distinguishes private members from public members by prefixing them with
the underscore. This creates isolation between files reduces coupling. This can
be overridden by using the `part of` directive at the top of a file. This
directive has the effect of combining multiple files and allowing them to access
each others private members. Doing this makes it hard to rationalize about what
is public and what is private and creates tight coupling between classes. Rather
than using this directive, it is recommended to only interact with another
object via its public interfaces. If classes must interact via private
interfaces it is recommended to keep them in the same file to clearly indicate
their relationship.

### Global Static Variables

Global static variables can be useful in sharing state across a library but they
can easily introduce race conditions and hard to debug code. Global variables
can also be accessed by users of your library, which may introduce unexpected
side effects. It is strongly recommended that you avoid global static variables
in public libraries.

If there is a reason that your package does need to use a global static variable
it is recommended to use zone-local static variables instead to isolate the
variable from users of your library.

```
void startComputation() {
  runZoned(() async {
    await collectScores(getValues());
    print('Scores: ${Zone.current[#scores]}');
  }, zoneValues: {#scores: <int>[]});
}

Future<void> collectScores(Stream<int> scores) async {
  await for (int value in scores) {
    Zone.current[#scores].add(value);
  }
}

// scores will not be affected by the call to startComputation.
final scores = <int>[1, 2, 3];

void main() {
  startComputation();
}
```
