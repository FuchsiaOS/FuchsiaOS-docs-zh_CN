{% set rfcid = "RFC-0036" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

Note: Formerly known as [FTP](../deprecated-ftp-process.md)-036.

## Rejection rationale

### Pros

* Confusion around name being important for ABI or not exists:
  xunion, struct, and protocols all look similar, but have
  different rules.

### Cons

* However, all concerned felt this was far outweighed by the
  confusion that introducing ordinals on structs would add,
  especially when compared to protobufs.

* There are other efforts addressing "does this change ABI", namely:
  * DIFL
  * API diffing, e.g. signature for libraries

* Name matters for text formats (JSON, FIDLText, etc.), and when messages
  are used in this context, name changes cannot occur.

## Summary

To better convey ABI implications of re-ordering and renaming fields, we
propose a **syntactic change to introduce ordinals for structs fields**,
with similar syntactic rules then those for tables.

## Motivation

Focusing solely on whether members can be safely renamed or re-ordered in
various declarations, we have syntactic differences, which have evolved
organically, and do not convey anything about ABI implications of possible
changes.

Furthermore, the current struct declaration syntax makes it difficult for
the compiler to provide help and [guidance when changes occur](#compiler-guidance).

Let's look at examples, these are chosen to be small and uniform:

```fidl
struct Name {      table Name {        enum Name {
    T abc;           1: T abc;           ABC = 1;
    U xyz;           2: U xyz;           XYZ = 2;
};                 };                  };

protocol Name {    xunion Name {       bits Name {
    Abc(T t);        T abc;              ABC = 1;
    Xyz(U u);        U xyz;              XYZ = 2;
};                 };                  };
```

Some observations from an ABI standpoint:

* Re-ordering: All but the struct can be re-ordered without any impact.
* Renaming:
  * struct, table, enum, and bits can be renamed with no impact
  * protocol, and xunion will have their ABI impacted upon rename.

(From a source compatibility standpoint, most bindings will be source
compatible under re-order, and incompatible under rename.)

Informed from these observations, we propose to introduce an ordinal for
struct declarations.
The example above would now be:

```fidl
struct Name {
    1: T abc;
    2: U xyz;
};
```

Specifically:

* Ordinals must start at 1 and no gaps are allowed in the ordinal space (if
  the largest ordinal is 7, then all of 1,2,3,4,5,6,7 must be present). See
  [rationale below](#disallow-reserved-keyword).
* No two fields can claim the same ordinal.
* The field ordinal determines the placement of a field with a struct, not
  its syntactic position.
* No change to the JSON IR in v1, the ordinal is conveyed through the order
  of members in struct declaration. See planned change to JSON IR in v2.

### Compiler guidance

To exemplify the guidance, which the compiler can provide with the proposed
syntax, we consider a few examples and compare their handling.

#### Removing a field (middle)

```fidl
No Ordinals        With Ordinals
----------------   -------------
struct Name {      struct Name {
    T abc;           1: T abc;
-   U def;       -   2: U def;
    V ghi;           3: V ghi;
};                 };
----------------   ---------------
Breaks ABI, no     Breaks ABI,
compiler help      compiler error
```

#### Removing a field (end)

```fidl
No Ordinals        With Ordinals
----------------   -------------
struct Name {      struct Name {
    T abc;           1: T abc;
    U def;           2: U def;
-   V ghi;       -   3: V ghi;
};                 };
----------------   ---------------
Breaks ABI, no     Breaks ABI, no
compiler help      compiler help
```

#### Add a field

```fidl
No Ordinals        With Ordinals
----------------   -------------
struct Name {      struct Name {
    T abc;           1: T abc;
+   U def;       +   3: U def;
    V ghi;           2: V ghi;
};                 };
----------------   ---------------
Breaks ABI, no     Breaks ABI, no
compiler help      compiler error
```

#### Reorder fields

```fidl
No Ordinals        With Ordinals
----------------   -------------
struct Name {      struct Name {
+   U def;       +   2: U def;
    T abc;           1: T abc;
-   U def;       -   2: U def;
    V ghi;           3: V ghi;
};                 };
----------------   ---------------
Breaks ABI, no     Safe
compiler warning
```

### Disallow 'reserved' keyword

Since we are aligning the ordinal rules for structs on that of tables, we
could look to also allow the 'reserved' keyword.

We should do the exact opposite: properly parse an accidental use of the
reserved keyword, and provide a clear compiler error and explanation. For
instance "Cannot reserve member in structs. Adding or removing members
alters a struct layout, consider instead neutral members manually
initialized."

There are also additional important reasons **not** to allow the
'reserved' keyword:

1. Unlike for tables, introducing padding in a struct must be done with an
   explicit size (i.e. number of bytes);
2. Using padding in structs is done for very specific purposes, when
   developers need a specific memory layout.
   This use case is rare, or even nonexistent since FIDL layout is always
   8 bytes aligned.
3. Implementation-wise, we've clarified and explained in [RFC-0066:
   Programmer Advisory Explicit Defaults](contribute/governance/rfcs/0066_programmer_advisory_explicit.md) that guaranteeing
   certain values be initialized is too strong of a requirement for certain
   bindings (e.g. C, LLCPP).
   As a result, should we introduce 'reserved' slots in structs,
   we would need to expose that to backends, in order to expose that to
   developers for proper initialization.
   All this seems unnecessary.

#### Down the road JSON IR

In order to both support ordering of fields (by ordinal) and ordering for
documentation purposes (which should respect declaration order), it would
be better to:

* Represent declaration order as the order in which fields are presented in
  the "members" key.
* Represent ordinal order by introducing an "ordinal" key.

## Design

TBD

# Implementation strategy

1. Introduce support for the new syntax, while at the same time support the
   previous one;
2. Migrate all source files to the new syntax;
3. Add a warning when using the previous syntax, give a one week period to
   ensure no new uses of the previous syntax are added;
4. Remove support for the previous syntax.

## Ergonomics

This proposal improves ergonomics by conveying ABI implications to
developers through syntax. See an [opposing view on this
below](#drawback_struct-and-tables-could-be-confused).

## Documentation and examples

At least:

* [Language Specification][language]
* [Grammar][grammar]
* Examples using structs

## Backwards compatibility

This is not source level backwards compatible.
See [Implementation Strategy](#implementation-strategy) to soft migrate.

## Performance

No impact.

## Security

No impact.

## Testing

Unit testing in `fidlc` to verify among others:

* Parsing;
* Ordinals start at 1, and may not have gaps;
* No change to JSON IR.

## Drawbacks, alternatives, and unknowns

### Alternative: Ordinal Hashing for Tables

We also considered using ordinal hashing for tables: the syntactic change
would be dropping explicit ordinals, making structs be the only declarations
with this syntax (whereas it used to be on protocols and tables).

Firstly, the benefits of having explicit ordinals for structs would remain.
Developers could still re-order fields syntactically, and changing
an ordinal would indicate ABI breakage.

Secondly, we are unlikely to act on the exploration to remove ordinals
from tables since the tradeoff between run-time cost (less performance)
outweigh the ergonomic benefits.

### Drawback: Struct and tables could be confused

With the syntax between struct and tables converging, and the introduction
of ordinals, some may confuse structs with tables, and mistakenly believe
that removing fields is ABI compatible.
While removing a field in the middle of a struct would cause an error due to
a gap appearing in the ordinal sequence, removing the field(s) with the
largest ordinal(s) would be silent.

## Prior art and references

TBD

<!-- xrefs -->
[language]: reference/fidl/language/language.md
[grammar]: reference/fidl/language/grammar.md

