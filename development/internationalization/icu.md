# International Components for Unicode (ICU) use in Fuchsia

Fuchsia uses the [ICU library](http://site.icu-project.org/) for the commonly
shared internationalization services such as date, time, timezone, locale and
language handling.

The ICU library consists roughly of two different parts: the ICU library *code*
which contains the ICU algorithms, and the ICU library *data*, which contains
locale-specific information that is packaged for independent reuse.

Note: For a complete example of using the ICU library in Fuchsia, see
[`//examples/intl/wisdom`][wisdom].

## Add ICU library dependencies

Include the necessary library dependencies in your `BUILD.gn` file:

* {C++}

  The ICU library is a single third-party dependency `//third_party/icu`:

  ```gn
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/intl/wisdom/cpp/client/BUILD.gn" region_tag="icu_library" adjust_indentation="auto" %}
  ```

* {Rust}

  The `rust_icu` library is subdivided into several crates that correspond to
  specific ICU4C headers:

  ```gn
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/intl/wisdom/rust/client/BUILD.gn" region_tag="icu_library" adjust_indentation="auto" %}
  ```

## Import ICU headers

Add the imports for the specific ICU library features your program requires:

* {C++}

  The [ICU documentation][cpp-reference]{: .external} provides additional
  information about the APIs the library supports.

  ```cpp
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/intl/wisdom/cpp/client/intl_wisdom_client.cc" region_tag="imports" adjust_indentation="auto" %}
  ```

* {Rust}

  The [rust_icu reference][rust-reference]{: .external} provides additional
  information about the APIs the library supports.

  ```rust
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/intl/wisdom/rust/client/src/wisdom_client_impl.rs" region_tag="imports" adjust_indentation="auto" %}
  ```

## Initialize ICU data

In Fuchsia, the ICU data *must* be loaded by the program at runtime. Initialize
the ICU data with the `icu_data` library:

* {C++}

  ```cpp
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/intl/wisdom/cpp/server/intl_wisdom_server_impl.cc" region_tag="loader_example" adjust_indentation="auto" %}
  ```

* {Rust}

  ```rust
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/intl/wisdom/rust/server/src/main.rs" region_tag="loader_example" adjust_indentation="auto" %}
  ```

For more details instructions on loading ICU data from various sources,
see [ICU timezone data](icu_data.md).

You are now ready to use the ICU library features in your Fuchsia program.

<!-- xrefs -->
[cpp-reference]: https://unicode-org.github.io/icu/
[rust-reference]: https://docs.rs/crate/rust_icu/1.0.1
[wisdom]: /examples/intl/wisdom/
