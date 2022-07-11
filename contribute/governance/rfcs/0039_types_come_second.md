{% set rfcid = "RFC-0039" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

Note: Formerly known as [FTP](../deprecated-ftp-process.md)-039.

_"You're Go-ing to love it"_

## Rejection rationale

When this proposal was drafted, and socialized, there was a strong consensus to
consider syntax changes all at once, rather than one at a time (see also
[RFC-0038](/contribute/governance/rfcs/0038_seperating_layout_from_constraints.md)).
We also wanted one person to be the syntax arbiter, rather than risk designing
by committee.

Eventually, this proposal was obsoleted by
[RFC-0050](/contribute/governance/rfcs/0050_syntax_revamp.md) which met
both conditions sought.

## Summary

We propose to:

* Allow **inline declarations of structs, unions, and tables**;
* Flip the order in which field names (or argument/return names) appear with
  respect to types, specifically to have the **type appear after names**.

(We stop short of introducing anonymous declarations, since we would likely want
improved bindings support to ensure the ergonomics are good.)

## Motivation

Quickly:

* We're starting to see more patterns where combination of various declarations
  to describe 'one conceptual message' is routine. For instance:
  * Container struct, whose last field is a table (initially empty) to leave the
    door open to extensions.
  * Container union, where variants are tables to have flexibility.
  * Container table, where fields are grouped in structs, and ordinals loosely
    match 'version numbers'.
* Additionally, support for empty struct, unions, and tables offers the
  low-level pieces to build Algebraic Data Type support from (from a layout
  standpoint, not bindings).
* All of these use cases are pushing us towards allowing inline declarations.
* With inline declarations, it is easier to read the field name first, then have
  a type description, which could straddle multiple lines. See examples below.

## Design

Some examples:

* Simple struct or table:

  ```fidl
  struct Name {
      field int32;
  };

  table Name {
      1: field int32;
  };
  ```

* Protocols:

  ```fidl
  protocol Name {
      Method(arg int32) -> (ret int32);
  };
  ```

* Struct with extension:

  ```
  struct Name {
      field1 T1;
      field2 T2;
      ...;
      ext table NameExt {};
  };
  ```

* Union variants:

  ```fidl
  union Name {
      variant1 table NameVariant1 {
          ...
      };
      variant2 table NameVariant2 {
          ...
      };
      ...
  };
  ```

* Grouped fields by version:

  ```fidl
  table Name {
      1: v1 struct NameGroupV1 {
          ...
      };
      2: v2 struct NameGroupV2 {
          ...
      };
      ...
  };
  ```

Notes:

* Scoping wise, while we would consider all declaration names to be top-level
  (and hence enforce uniqueness on a per-library basis), we would not allow
  inline declarations from being referenced, i.e. only single use.

## Ergonomics

This proposal improves ergonomics by conveying ABI implications to developers
through syntax.

## Documentation and examples

At least:

* [Language Specification](/reference/fidl/language/language.md)
* [Grammar](/reference/fidl/language/grammar.md)
* Examples using structs

## Backwards Compatibility

This is not source level backwards compatible.

--------------------------------------------------------------------------------

Note: This RFC was rejected early during its socialization phase, which explains
the multiple missing sections (e.g. "Implementation strategy", or "Drawbacks,
alternatives, and unknowns").
