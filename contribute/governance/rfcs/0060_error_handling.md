{% set rfcid = "RFC-0060" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

Note: Formerly known as [FTP](../deprecated-ftp-process.md)-014.

## Summary

APIs often need to indicate that requests could not be successfully completed.
Often there is useful contextual information associated with the error that
allows the caller to take corrective action.
This proposes a syntax that will allow FIDL interfaces to describe how errors
will be reported.

## Motivation

Most programming languages offer error handling constructs like exceptions,
Futures/Promises, result types or error types.
APIs written in languages that don't, typically invent their own conventions
like `errno`, `zx_status_t` and `GError`.

FIDL methods can return multiple values, so a method may return both a value and
an error code, but this is done in an inconsistent ad-hoc way.
Usually interfaces authors have been putting the error code before the method
result, but about 20% of the time it's the other way around.
Sometimes interfaces return a `struct` that contains both a status code and a
result.
Statuses are represented as `bool`s, `int`s, `enum`s, `string`s and `struct`s.

This diversity of APIs is hard for developers to understand.
The lack of explicit syntax to differentiate a method result from error
information makes it impossible to produce idiomatic bindings.

## Design

We should extend the FIDL method syntax to allow interfaces to describe
different *result* return type and *error* return type.
Events never have error types.
The syntax looks like:

```fidl
interface Example {
  // This method returns no values, failures or completion.
  1: NoReturn();

  // This method returns no values or failures but informs
  // clients of completion.
  2: Completion() -> ();

  // This method returns a single value.
  3: SingleValue() -> (int32 result);

  // This method returns multiple values.
  4: MultipleValue() -> (int32 foo, string bar);

  // This method doesn't return any values but can indicate a
  // failure reason.
  5: CanFail() -> () error int32;

  // This method can succeed with a result or fail with a reason.
  6: WinOrLose() -> (string result) error ExampleError;
};
```

Methods that don't declare an error type are serialized and bound exactly as
they are today.
The return arguments are encoded as a FIDL `struct`.

Methods that do declare an error type are serialized as a FIDL union of the
result and error return types.
So a method like:

```fidl
interface Example {
  1: Method() -> (string result, string other_result) error int32;
};
```

Would be encoded like this one:

```fidl
struct ExampleMethodResult {
  string result;
  string other_result;
};
[Result]
union ExampleMethodReturn {
  ExampleMethodResult result;
  int32 err;
};
interface Example {
  1: Method() -> (ExampleMethodReturn return);
};
```

Error types must be `int32`, `uint32`, or an `enum` of one of those types.

All existing interfaces will be source and binary-compatible though ideally
they would evolve to use the new error syntax.

In the FIDL IR, unions that represent a result or error will be differentiated
from other unions because the frontend compiler will attach the `[Result]`
attribute to them.
Existing language bindings will continue to work but can be updated to support
more idiomatic language features for representing failed method calls.

We would propose that Dart returns failures through `Future` errors.
They should be a subclass of a special `Error` type defined in `package:fidl`
so that application level errors can be easily differentiated from binding
and protocol errors.

Rust should use `std::result::Result`.
C++ could use `std::expected` if that ever becomes a real thing but in the
meantime we could implement enough of that proposal to satisfy the needs of
FIDL.
Go bindings should use a custom error type for error returns.

## Implementation strategy

This would be implemented in the following steps:

* Update `fidlc` to support the new syntax.
* Check that it is defining the correct encoding.
* Update FIDL language documentation.
* Update bindings to use more idiomatic error handling.
* Update the [FIDL Compatibility Test interface][testinterface]
  to test errors and add support to all of the language bindings.
* Validate that language bindings correctly interoperate.
* Update documentation.
* Encourage interface authors to evolve their interfaces to use error
  return types.

## Documentation and examples

This is a significant change to FIDL.
The [language] and [wire-format] documentation would have to be updated to
describe the new syntax and how it is serialized.
The [FIDL tutorial][tutorial] should be updated to give examples of how to
use error returns correctly.
The [API techniques rubric][rubric] needs to be updated to describe appropriate use
of this feature.

## Backwards compatibility

Most existing FIDL interfaces will remain compatible with this change.
The only breaking change is that `error` becomes a reserved word.

## Performance

There should be very little performance impact.

## Security

Standardizing error reporting semantics will simplify code that calls FIDL
methods.
Explicit is better than implicit.

## Testing

This will need tests for `fidlc`, compatibility tests and probably language
binding specific tests to ensure that the bindings are idiomatic.

## Drawbacks, alternatives, and unknowns

This adds some complexity to the language but that complexity simply describes
semantics that are already implicitly expressed in our interfaces.

One suggestion that this proposal does not encompass is to have a standard
error enum that describes the category of error and give hints to a caller on
how to proceed.
This pattern is common to `errno`, `zx_status_t`, and [HTTP status
codes][http].
Those examples try to capture more detail than we think is appropriate.
[`grpc_status_code`][grpc_status_code] is a better model for us, capturing
high-level errors.
We are considering adding a generic error enum to a standard FIDL library that
interface authors can choose to use instead of inventing their own.

There was extensive discussion of how to fold application errors (e.g., record
not found) with transport level FIDL errors (e.g., message parsing failed).
Currently all transport level FIDL errors result in the channel being closed so
there's no error code to fold.
We would like to be able to recover from such errors but that will be
proposed in a future FTP.
Keeping the errors to 32 bits leaves much opportunity open for error folding.

Earlier versions of this proposal allowed errors to be arbitrary data types.
This could have encouraged anti-patterns like returning error message strings
and would have limited our flexibility as we seek to align errors with epitaphs
and re-examine error folding.
We're adopting a more conservative idea of error representation for now.

## Prior art and references

* [GRPC returns a status code with each result][grpc]
* [DBus method calls return either result data or an error object][dbus]
* [All COM calls return an HRESULT code][com]
* [Binder statuses include a standard error code as well as a app-specific code
  and string][binder]

<!-- xrefs -->
[binder]: https://android.googlesource.com/platform/frameworks/native/+/1651ced/include/binder/Status.h
[com]: https://docs.microsoft.com/en-us/windows/desktop/learnwin32/error-handling-in-com
[dbus]: https://dbus.freedesktop.org/doc/dbus-tutorial.html#callprocedure
[grpc]: https://grpc.io/docs/guides/error.html
[http]: https://tools.ietf.org/html/rfc1945#section-9
[language]: /docs/reference/fidl/language/language.md
[rubric]: /docs/development/api/fidl.md
[testinterface]: /src/tests/fidl/compatibility/compatibility_service.test.fidl
[tutorial]: /docs/development/languages/fidl/tutorials/overview.md
[wire-format]: /docs/reference/fidl/language/wire-format/README.md
[grpc_status_code]: https://github.com/grpc/grpc/blob/HEAD/include/grpc/impl/codegen/status.h#L26
