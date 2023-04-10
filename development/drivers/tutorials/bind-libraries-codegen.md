# Bind library code generation tutorial

Bind libraries can auto-generate code to help driver authors refer to items in
bind libraries from driver code. Currently the build will produce code for C++ and Rust.
This tutorial will go into detail on using these auto-generated code targets.

This guide assumes familiarity with the following concepts:

 * [Driver Bind Rules](/docs/development/drivers/tutorials/bind-rules-tutorial.md)
 * [Driver Binding](/docs/development/drivers/concepts/device_driver_model/driver-binding.md)

## An example bind library

First let's define an example bind library with some different definitions. We will also make it
depend on another existing bind library. This is optional but for completeness we'll include it.

### BUILD.gn

```gn {:.devsite-disable-click-to-copy}
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/drivers/bindlib_codegen/BUILD.gn" region_tag="example_bind_lib_target" %}
```

Most of the time the build target name and the library name are the same (it's
not necessary to supply the `name` property, just put the library name as the build target name).
But here we define them separately to help distinguish where each one is used later on.

### mybindlib.bind

```none {:.devsite-disable-click-to-copy}
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/drivers/bindlib_codegen/mybindlib.bind" %}
```

## Auto-generated libraries

### The generated build target

* {C++}

  The build will automatically give us a new target based on the `target_name` of the bind library,
  `:{target_name}_cpp`

  In our example from earlier this was `my_bindlib_target`. So the target will be
  `:my_bindlib_target_cpp`.

* {Rust}

  The build will automatically give us a new target based on the `target_name` of the bind library,
  `:{target_name}_rust`

  In our example from earlier this was `my_bindlib_target`. So the target will be
  `:my_bindlib_target_rust`.

### Using the generated library

* {C++}

  This is a `source_set` target that contains a header file with constants generated from the
  bind library. Driver authors can use this when they are creating node properties. They just need
  to create a dependency from their executable or other C++ based target on this.

  The include path and namespace will be based on the `library_name` of the bind library (with
  some replacements of the `.`s in the library name),
  `#include <bind/{library_name with slashes}/cpp/bind.h>` and
  `namespace bind_{library_name with underscores}`.

  In our example from earlier the `library_name` was `fuchsia.example.library`. So we will have
  `#include <bind/fuchsia/example/library/cpp/bind.h>` and `namespace bind_fuchsia_example_library`.

* {Rust}

  This is a `rustc_library` target, which is a Rust crate with a root module with constants
  generated from the bind library. Driver authors can use this when they are creating node
  properties. They just need to create a dependency from their Rust based targets on this.

  The crate name to reference in use statements will be based on the `library_name` of the
  bind library (with some replacements of the `.`s in the library name),
  `use bind_{library_name with underscores};`.

  In our example from earlier the `library_name` was `fuchsia.example.library` so we will have
  `use bind_fuchsia_example_library;`.

### The generated header file

* {C++}

  ```cpp {:.devsite-disable-click-to-copy}
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/drivers/bindlib_codegen/cpp_codegen.h.golden" exclude_regexp="// Copyright.*|// Use of.*|// found in.*" %}
  ```

* {Rust}

  ```rust {:.devsite-disable-click-to-copy}
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/drivers/bindlib_codegen/rust_codegen.rs.golden" exclude_regexp="// Copyright.*|// Use of.*|// found in.*" %}
  ```

### The generated constants

#### Keys

For every key in the bind library, there is a string constant identifier of the same name,
but in all uppercase. The value of the string will be the full name of the key.
The full name contains the `library_name` of the bind library and the key name, with no
casing adjustments, separated by a `.`.

* {C++}

  If we have a key called `keyName` in a library called `fuchsia.example.library`, this will be the
  generated constant: `std::string KEYNAME = "fuchsia.example.library.keyName";`.

* {Rust}

  If we have a key called `keyName` in a library called `fuchsia.example.library`, this will be the
  generated constant: `pub const KEYNAME: &str = "fuchsia.example.library.keyName";`.

#### Values

For every value defined for a key in the bind library there is a constant based on the type
of the key (see table below). The identifier of the constant is the all uppercase version of
the key and value name separated by an underscore.

For example if there is a key `foo` with two values defined for it `bar` and `baz`, then there are
two identifiers for the values, `FOO_BAR` and `FOO_BAZ`.

* {C++}

  Key type | C++ constant type | C++ constant value
  -------- | ----------------- | ------------------
  uint     | uint32_t          | Integer value from entry
  string   | std::string       | String value from entry
  bool     | bool              | Boolean value from entry
  enum     | std::string       | Full name of the enum (see below)

* {Rust}

  Key type | Rust constant type | Rust constant value
  -------- | ------------------ | ------------------
  uint     | u32                | Integer value from entry
  string   | &str               | String value from entry
  bool     | bool               | Boolean value from entry
  enum     | &str               | Full name of the enum (see below)


The full name of an enum contains the `library_name`, key name, and value name,
all separated by `.`s.

* {C++}

  If we have a value called `someValue` under a enum based key `enumeratedKey` in our library
  called `fuchsia.example.library`, then the generated constant will be
  `std::string ENUMERATEDKEY_SOMEVALUE = "fuchsia.example.library.enumeratedKey.someValue";`.

* {Rust}

  If we have a value called `someValue` under a enum based key `enumeratedKey` in our library
  called `fuchsia.example.library`, then the generated constant will be
  `pub const ENUMERATEDKEY_SOMEVALUE: &str = "fuchsia.example.library.enumeratedKey.someValue";`.

#### Dependencies

* {C++}

  Because our bind library example depended on `//src/devices/bind/fuchsia.pci`, another bind
  library, our generated code has automatically included the header generated from that as well.
  So by including this header, the code can refer to values from the base bind library as well.

* {Rust}

  Because our bind library depended on `//src/devices/bind/fuchsia.pci`, another bind
  library, our generated code has automatically imported the generated crate from that as well.
  The difference with the C++ header is that in Rust the user will have to nest the crate names
  when referring to a dependency crate. See example below for what this looks like.

### Example usage

#### BUILD.gn

* {C++}

  ```gn {:.devsite-disable-click-to-copy}
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/drivers/bindlib_codegen/BUILD.gn" region_tag="example_cpp_target" %}
  ```

* {Rust}

  ```gn {:.devsite-disable-click-to-copy}
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/drivers/bindlib_codegen/BUILD.gn" region_tag="example_rust_target" %}
  ```

#### parent-driver

* {C++}

  ```cpp {:.devsite-disable-click-to-copy}
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/drivers/bindlib_codegen/parent-driver.cc" region_tag="code" %}
  ```

* {Rust}

  ```rust {:.devsite-disable-click-to-copy}
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/drivers/bindlib_codegen/parent-driver.rs" region_tag="code" %}
  ```

## Auto-generated libraries in the SDK

To learn how to auto-generate bind library artifacts in the Fuchsia SDK
development environment, see
[Create a new bind library for a driver](/docs/development/sdk/create-new-bind-library-for-driver.md).
