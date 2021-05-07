{% set rfcid = "RFC-0019" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

Note: Formerly known as [FTP](../deprecated-ftp-process.md)-002.

## Summary

Add a mechanism to give more descriptive names to primitive types.
Remove status as a built-in feature of the language.  Introduce a zx
library to contain the primitive types from `<zircon/types.h>`.

## Motivation

Often developers want to assign more descriptive names to primitive
types.  For example, `status` is a more descriptive name for `int32`,
but `status` is built into the language and other types cannot be used
in the same way.

## Design

This proposal affects only the FIDL source language.

1. Extend the `using` keyword to be able to assign descriptive names
   to primitive types.  Specifically, add the following production to
   the FIDL grammar:

```
using-list = ( using | using-declaration )* ;
using-declaration = "using" , IDENTIFIER ,  "=" , primitive-type , ";" ;
```

1. Remove the `status` primitive type from the language.  The `status`
   primitive type can now be defined using the language itself instead
   of being a built-in feature.

1. Remove the `status` reserved word.  We no longer need to reserve
   the word `status` now that we can define the name in the language.
   Add a `zx` library that contains declarations for the primitive
   types defined by the Zircon system interface.  Ideally, this
   library would be generated from a future syscalls.abigen that
   contained this information.  For example:

```
library zx;

using status = int32;
using time = uint64;
using duration = uint64;
[...]
```

Notice that these declarations must appear in the using-list and must
refer directly to primitive types.  This approach avoids complexity in
the compiler because their use can be immediately translated into the
underlying primitive type.

Further, notice that there are no proposed changes to the generated
code in any target language.  These types are represented in the
target languages using the underlying primitive types.

## Documentation and examples

Example usage

```
    1: AdvanceTime(zx.duration duration) -> (zx.status status);
```

## Backwards compatibility

This change is a non-backwards compatible change to the FIDL source
language because it removes the `status` primitive type.  However,
migrating existing clients of `status` is trivial because they can
simply use `zx.status` instead.

## Performance

The proposed change has no impact on performance.

## Security

The proposed change has no impact on security.

## Testing

The feature will be tested in the fidl-test suite in Zircon by adding
a FIDL file that uses each of the types from the `zx` library.

## Drawbacks, alternatives, and unknowns

This proposal is straightforward to implement in the FIDL frontend and
requires no changes to the language-specific backends.

An alternative is to not solve the problem and continue to use
primitive types directly.

## Prior art and references

This feature is very common in programming languages.  The syntax is
borrowed loosely from C++.
