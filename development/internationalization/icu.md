# International Components for Unicode (ICU) use in Fuchsia

Fuchsia uses the [ICU library](http://site.icu-project.org/) for the commonly
shared internationalization services such as date, time, timezone, locale and
language handling.

The ICU library consists roughly of two different parts: the ICU library *code*
which contains the ICU algorithms, and the ICU library *data*, which contains
locale-specific information that is packaged for independent reuse.

The code is available through appropriate shared libraries in
`//third_party/icu` (see below).

# Prerequisites

## `icu_data` library

In Fuchsia, the ICU data is made available to be loaded at runtime. Please see
the [ICU data use instructions](icu_data.md) on how to load this data.

## `icu_data` `BUILD.gn` rules need changing

Since `icu_data` needs the ICU data files to be made available in the Fuchsia
package, please see [ICU data use instructions](icu_data.md) for an example of
how to make the data files available.

# Using the ICU library

This section assumes that you have read and followed the instructions from the
Prerequisites section in full detail.

## C and C++

The ICU library is imported through a third-party dependency
`//third_party/icu`. As an example use of the library, one can look at the [C++
wisdom example][wisdomcpp].  This is a sample client-server collaboration that
requests, serves and prints on screen date and time information using several
different languages, calendars and scripts.

## Rust

The ICU library is available in rust programs as well, through a binding of the
ICU4C library into Rust.

The library is subdivided into [several
crates](https://fuchsia-docs.firebaseapp.com/rust/?search=rust_icu), each one
corresponding to a specific part of the ICU4C headers, and named after the
corresponding one.  Today, the functionality is partial, and is constructed to
serve Fuchsia's Unicode needs.

As a demonstration of the rust bindings for ICU4C, we made a rust equivalent of
the wisdom server.  This example is available as the [rust wisdom
example][wisdomrust].

[wisdomcpp]: /garnet/examples/intl/wisdom/cpp/
[wisdomrust]: /garnet/examples/intl/wisdom/rust/
