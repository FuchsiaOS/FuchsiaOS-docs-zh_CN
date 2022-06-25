# ICU timezone data

This document describes Fuchsia's support for loading ICU timezone data from
ICU data files (`icudtl.dat`). Programs load timezone data dynamically based on
the resources most appropriate for their application.

For more details on the general use of ICU APIs in Fuchsia, see
[International Components for Unicode use in Fuchsia](icu.md)


## Make the ICU data available

In order for the system default ICU data files to be visible to a program in
Fuchsia, you must include the resource dependency in your package:

```gn
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/intl/tz_version_parrot/rust/BUILD.gn" region_tag="icudata_resource" adjust_indentation="auto" %}
```

### Timezone configuration data

To provide a specific version of the timezone data files to your package as
configuration data, use the `icu_tzdata_config_data()` template in your
`BUILD.gn` file and include it as a dependency in your package declaration:

```gn
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/intl/tz_version_parrot/BUILD.gn" region_tag="tzdata_config" adjust_indentation="auto" %}
```

Then, request the `config-data` capability in your component to map the
subdirectory for your package into the component's namespace:

```json5
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/intl/tz_version_parrot/rust/meta/tz_version_parrot.cml" region_tag="config_data" adjust_indentation="auto" %}
```

Note: For more details on using `config_data` in your builds,
see [Product-specific configuration data][product-config]

## Load the default ICU data

You *must* load the ICU data in your program to make the locale data available.
Otherwise, no locale data will be available and your ICU code will behave as if
the set of i18n data is empty.

1.  Include the ICU data library dependency in your `BUILD.gn` file:

    * {C++}

      ```gn
      {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/intl/tz_version_parrot/cpp/BUILD.gn" region_tag="icudata_library" adjust_indentation="auto" %}
      ```

      Note: You can find the C++ library source at
      [`//src/lib/icu_data/cpp`](/src/lib/icu_data/cpp)

    * {Rust}

      ```gn
      {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/intl/tz_version_parrot/rust/BUILD.gn" region_tag="icudata_library" adjust_indentation="auto" %}
      ```

      Note: You can find the Rust library source at
      [`//src/lib/icu_data/rust`](/src/lib/icu_data/rust)

1.  Import the ICU data library into your source files:

    * {C++}

      ```cpp
      {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/intl/tz_version_parrot/cpp/test.cc" region_tag="imports" adjust_indentation="auto" %}
      ```

    * {Rust}

      ```rust
      {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/intl/tz_version_parrot/rust/src/lib.rs" region_tag="imports" adjust_indentation="auto" %}
      ```

1.  Initialize the ICU data loader:

    * {C++}

      ```cpp
      {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/intl/tz_version_parrot/cpp/test.cc" region_tag="loader_example" adjust_indentation="auto" %}
      ```

    * {Rust}

      ```rust
      {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/intl/tz_version_parrot/rust/src/lib.rs" region_tag="loader_example" adjust_indentation="auto" %}
      ```

      Note: At least one instance of `icu_data::Loader` must be kept alive for
      as long as your code needs ICU data. To simplify things, consider keeping
      a reference to the instance for the lifetime of the program.

You are now ready to use ICU data in your program.

### Load from the product configuration

To load the specific version of the ICU data provided by `icu_tzdata_config_data()`,
initialize the loader with the path to the data directory and revision file:

* {C++}

  ```cpp
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/intl/tz_version_parrot/cpp/test.cc" region_tag="loader_config_example" adjust_indentation="auto" %}
  ```

* {Rust}

  ```rust
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/intl/tz_version_parrot/rust/src/lib.rs" region_tag="loader_config_example" adjust_indentation="auto" %}
  ```

## Appendices

### Modifying the system time zone information

During development, use the `ffx setui` plugin to check or set the current
time zone ID on a Fuchsia target. For more information about the available
options, run the following commands:

```posix-terminal
ffx config set setui true // only need to run once
ffx setui intl --help
```

<!-- xrefs -->
[product-config]: development/components/data.md#product-specific_configuration_with_config_data
