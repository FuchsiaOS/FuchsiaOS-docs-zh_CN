{% set rfcid = "RFC-0022" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

Note: Formerly known as [FTP](../deprecated-ftp-process.md)-003.

## Summary

A FIDL struct member may have a default value.
Today, support of defaults is partially implemented (see section below),
this proposal aims to clarify how defaults ought to behave.

## Motivation

1. It provides regularity across the language binding, provides protections
   from inconsistent or unexpected uses, and
2. Eliminates laborious manual, member-by-member initializations when a
   language demands explicit initializations, and

Non-motivation includes:

* Saving bytes in wire format

It is *not* a motivation to save bytes in wire format or to save processing
power in doing encoding or decoding.

## Today's Implementation

Defaults can be [expressed][grammar] in the FIDL language on struct members:

* (+) There is support for numerical literals, boolean literals, and string
  literals.
* (-) No type checking is provided by `fidlc` of assignability of a literal to a
  struct member.
  It is possible to have a string literal "hello" assigned to a bool,
  a negative number assigned to a uint, or an out-of-bound number assigned
  to an int16.
* (-) Language binding support is inconsistent, today it only exists for
  C++, and Dart bindings.
  There is no support for Go, C, or Rust.

For example (from [//zircon/system/host/fidl/examples/types.test.fidl][example-types]):

```fidl {:.devsite-disable-click-to-copy}
struct default_values {
    bool b1 = true;
    bool b2 = false;
    int8 i8 = -23;
    int16 i16 = 34;
    int32 i32 = -34595;
    int64 i64 = 3948038;
    uint8 u8 = 0;
    uint16 u16 = 348;
    uint32 u32 = 9038;
    uint64 u64 = 19835;
    float32 f32 = 1.30;
    float64 f64 = 0.0000054;
    string s = "hello";
};
```

## Design

Default values MAY be defined on struct members.
Defaults appear at the end of a field definition with a C-like `= {value}` pattern.

### Syntax

```fidl {:.devsite-disable-click-to-copy}
// cat.fidl

enum CatAction : int8 {
    SIT = -10;
    WALK = 0;
    SNEAK = 2;
};

struct Location {
    uint8 pos_x = 10;  // Position X
    uint8 pos_y;       // Position Y. Default unspecified. Fall-back to 0
    float32 pos_z = 3.14;  // Position Z.
    float32 pos_t;         // Default unspecified. Fall-back to 0.0
};

struct Cat {
    string name;      // Automatic default to empty string
    CatAction action = CatAction::SNEAK;
    Location loc;

};
```

### Semantics

Please refer to [RFC-006](/docs/contribute/governance/rfcs/0066_programmer_advisory_explicit.md), which clarified the semantics of defaults,
and requirements on bindings.

### Supported Types

* Primitive types:
    * `bool`, `int8`, `int16`, `int32`, `int64`, `uint8`, `uint16`, `uint32`,
      `uint64`, `float32`, `float64`
* Non-nullable `string`, `string:N`
    * `string:N` shall zero out the memory that is reserved, and not used.

### Unsupported Types

* `array<T>:N`
    * Set to zero
* Non-nullable types: `vector<T>`, `vector<T>:N`
    * Set to zero
* Nullable types: `string?`, `string:N?`, `vector<T>?`, `vector<T>:N?`
    * Set to null
* `handle`
* `struct`
    * While each individual member in the `struct` may have a default,
      a `struct` itself does not have a default.
* `union`
    * To avoid any conflict, any default value of a member of the `union`,
      or that of a substructure (in any depth) of the `union` shall be
      ignored.

### Nuances of Defaults

The focus is on the value itself, and not on the *manner* of assigning the
value.
This implies two things at least:

* There is no distinction - if a default value is used because the parameter
  of interest was explicitly assigned by another mechanism, or not.
* There is no extra (transparent) layer of logic to assign values at the
  time of marshalling or unmarshalling.

## Implementation

Here are some example implemention ideas for C, Rust, and Go Bindings

```fidl {:.devsite-disable-click-to-copy}
// in FIDL "default.fidl"
struct Location {
    uint8 pos_x = 10;
    uint8 pos_y = 20;
    uint8 pos_x;       // Should be set to "zero" according to above.
};
```

```c {:.devsite-disable-click-to-copy}
// C binding "defaults/fidl.h"
typedef struct _Location_raw {
   uint8_t pos_x;
   uint8_t pos_y;
   uint8_t pos_z
} Location;

Location Location_default = { 10, 20, 0 }; // Or in the source file.
                                           // May be used for memcmp,
memcpy, etc.

#define Location(my_instance) Location my_instance = Location_default;
```

```c {:.devsite-disable-click-to-copy}
// C code "example.c"
#include <fidl.h>
void showme(Location loc) {
    printf("(%u, %u, %u)\n", loc.pos_x, loc.pos_y, loc.pos_z);
}

int main() {
    Location(alpha);
    Location beta;
    Location gamma = Location_default;
    showme(alpha); showme(beta); showme(gamma);
    return 0;
}
```

```rust {:.devsite-disable-click-to-copy}
// Rust binding
struct Location {
    pos_x: u8,
    pos_y: u8,
    pos_z: u8,
}
impl std::default::Default for Location {
    fn default() -> Self { Self { pos_x: 10, pos_y: 20, pos_z: 0 } }
}
```

```golang {:.devsite-disable-click-to-copy}
// Go binding, using export control
type location struct {
    pos_x  uint8
    pos_y  uint8
    pos_z  uint8
}

Func NewLocation() location {
    loc := location{}
    loc.pos_x = 10
    loc.pos_y = 20
    // loc.pos_z = 0  Maybe ommited.
    return loc
}
```

## Performance

n/a

## Security

n/a

## Testing

n/a

## Backwards compatibility

This change makes the FIDL file source backward-incompatible. No ABI or
wire format change is needed.

## Drawbacks, alternatives, and unknowns

It is not evaluated if implementation of this in all language bindings will
be straightforward.

## Prior art and references

[Protocol buffer][proto3-defaults], Flat buffer provides default values. Golang has a concept
of *zero values* where variables declared without an explicit initial values
are explicitly initialized as zero.

An open source approach:

```golang {:.devsite-disable-click-to-copy}
// From https://github.com/creasty/defaults
type Sample struct {
        Name   string `default:"John Smith"`
        Age    int    `default:"27"`
        Gender Gender `default:"m"`

        Slice       []string       `default:"[]"`
        SliceByJSON []int          `default:"[1, 2, 3]"` // Supports JSON format
        Map         map[string]int `default:"{}"`
        MapByJSON   map[string]int `default:"{\"foo\": 123}"`

        Struct    OtherStruct  `default:"{}"`
        StructPtr *OtherStruct `default:"{\"Foo\": 123}"`

        NoTag  OtherStruct               // Recurses into a nested struct even without a tag
        OptOut OtherStruct `default:"-"` // Opt-out
}
```

<!-- xref -->

[grammar]: /docs/reference/fidl/language/grammar.md
[example-types]: https://fuchsia.googlesource.com/fuchsia/+/1d98ab5e39255f8305825a18cd385198d6517569/zircon/system/host/fidl/examples/types.test.fidl#45
[proto3-defaults]: https://developers.google.com/protocol-buffers/docs/proto3#default
