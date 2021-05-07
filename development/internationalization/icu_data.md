# ICU timezone data

The ICU timezone data in Fuchsia is provided dynamically through the ICU data
files (`icudtl.dat`).  These are loaded on demand by programs, provided that
the program's package is configured to make the file available to the program
at runtime.

Section below shows how to do that.

# Making the ICU data available to packages

In order for the ICU data files to be visible to a program in Fuchsia, it first
needs to be made available in the package that contains the program.  This is
an example use from [the wisdom
server](/garnet/examples/intl/wisdom/rust/server/BUILD.gn).

```gn
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="garnet/examples/intl/wisdom/rust/server/BUILD.gn" region_tag="icudata" adjust_indentation="auto" %}
```

Note the section that adds a resource for `icudtl.dat`.

```gn
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="garnet/examples/intl/wisdom/rust/server/BUILD.gn" region_tag="icudata_resource" adjust_indentation="auto" %}
```

Since the library's footprint can be large, we do not simply import it as a
public dependency on any rust program that uses ICU.

# Modifying the time zone information on a system

In development you may need to check or set the time zone ID.  The program `setui_client`
allows you to do exactly this.  See more information about `setui_client` by running `run setui_client.cmx intl --help` on a target.

# Using the ICU data

You *must* load the ICU data in your program to make the locale data available.
If you do not do that, no locale data will be available, and your ICU code will
behave as if the set of i18n data is empty.

The APIs to load the code made available are different per language, so please
refer to the pages below for specific examples:

- [C++ library for ICU data loading](/src/lib/icu_data/cpp)
- [Rust library for ICU data loading](/src/lib/icu_data/rust)

## Rust example

In Rust, you should use the `icu_data::Loader`, which will automatically do the
right thing. Here is an example from the ICU data tests showing this approach.

Note that at least one instance of `icu_data::Loader` must be kept alive for as
long as your code needs ICU data. Since it is difficult to predict when this
data is needed, and the ICU liveness rules are a bit confusing, it is probably
best to simplify things and keep an `icu_data::Loader` handy for the lifetime
of the program.

```rust
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="src/lib/icu_data/rust/icu_data/src/lib.rs" region_tag="loader_example" adjust_indentation="auto" %}
```

In code, this amounts to instantiating a `icu_data::Loader` and keeping at
least one instance of it alive for the lifetime of the program.  The `Loader`
can be cloned at will and copied around: the ICU data will be loaded before the
first time it is needed, it will be unloaded when it is not needed, and will be
reloaded again if needed again.

```rust
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="garnet/examples/intl/wisdom/rust/server/src/main.rs" region_tag="loader_example" adjust_indentation="auto" %}
```

Perhaps a more robust approach to maintaining a live `icu_data::Loader` is to
pass a possibly cloned instance of a `Loader` into whatever struct requires ICU
data.  This will ensure that even in face of code refactors, the code that
needs live ICU data always has it available:

```rust
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="garnet/examples/intl/wisdom/rust/client/src/wisdom_client_impl.rs" region_tag="loader_example" adjust_indentation="auto" %}
```

# See also

* [ICU use in Fuchsia](icu.md)
