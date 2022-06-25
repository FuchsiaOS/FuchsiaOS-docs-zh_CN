# Provide data files to components

## Goal

In the guide on [building components][build-components] we saw several examples
of defining components for executables and then packaging them. In this document
we review ways of making data files available to components in the same package.

## Hermetic data files with `resource()`

At runtime, components are able to read the contents of their own package by
accessing the path `/pkg/` in their namespace. The `resource()` template may be
used to add contents to the package that may be accessed this way.

{# note: the verbatim tags below are to avoid issues with the fuchsia.dev template engine #}
```gn
import("//build/dist/resource.gni")
{% verbatim %}
# List of greetings
resource("greetings") {
  sources = [ "greetings.txt" ]
  outputs = [ "data/{{source_file_part}}" ]
}{% endverbatim %}
```

See the [resource][resource] template for more usage instructions.
For information about `{% verbatim %}{{source_file_part}}{% endverbatim %}`
and similar syntax see [GN placeholders][gn-placeholders]{:.external}.

Using `resource()` is also demonstrated in [additional packaged resources][additional-packaged-resources].

### Including resources with a component

Add a dependency on the resource target from a component in order to ensure that
the resource(s) are included in the same package.

```gn
import("//build/components.gni")

# Sends a random greeting to a client
executable("greeter") {
  sources = [ "greeter.cc" ]
  deps = [ ... ]
}

fuchsia_component("greeting_server") {
  deps = [
    ":greeter",
    ":greetings",
  ]
  manifest = "meta/greeting_server.cml"
}
```

In the example above, at runtime the component will be able to read the file
in its namespace at the path `/pkg/data/greetings.txt`. This will work regardless
of what package(s) (defined with `fuchsia_package()`) this component is
included in.

### Packaging conventions

*   Small data files (less than 4kb) should be packaged under `meta/`, though
    this is not required. Packaging small files under `meta/` allows the
    packaging system to archive them in a single [`meta.far` file][metafar],
    which is a more efficient way to store small files.

*   Otherwise, data files are usually packaged under `data/` by convention,
    though again this is not a technical requirement.

### Using different resources in different packages

Sometimes it's desirable to package the same component with different data
files.

```gn
import("//build/dist/resource.gni")
import("//build/components.gni")

# Sends a random greeting to a client
executable("greeter") {
  sources = [ "greeter.cc" ]
  deps = [ ... ]
}

fuchsia_component("greeting_server") {
  deps = [ ":greeter" ]
  manifest = "meta/greeting_server.cml"
}

# List of production greetings.
# Contains only the finest greetings and the best regards.
resource("greetings") {
  sources = [ "greetings.txt" ]
  outputs = [ "data/greetings.txt" ]
}

fuchsia_package("greeting") {
  deps = [
    ":greeting_server",
    ":greetings",
  ]
}

# Greetings for testing.
# Contains exactly one greeting so that tests are reproducible.
resource("test_greeting") {
  testonly = true
  sources = [ "test_greeting.txt" ]
  outputs = [ "data/greetings.txt" ]
}

# Connects to the greeting server.
# Ensures that the expected greeting is sent back.
fuchsia_test_component("greeting_test_client") {
  ...
}

fuchsia_test_package("greeting_integration_test") {
  test_components = [ ":greeting_test_client" ]
  deps = [
    ":greeting_server",
    ":test_greeting",
  ]
}
```

In the example above, the same `greeting_server` component is added to two
packages, one for production and another for testing. In both cases the
component will find a file under `/pkg/data/greetings.txt`. However the
contents of this file will vary between the production version and the testing
version, depending on the package association.

## Product-specific configuration with `config_data()`

Sometimes a component is defined in one repository but its data is defined in
another repository. For instance `fuchsia.git` defines a font provider service,
but the `workstation` product configuration (defined in a different repository)
defines which fonts are available to the font provider.

The `config_data()` template allows developers to make files available to
components in a different package without having to directly change the contents
of that package.

{# note: the verbatim tags below are to avoid issues with the fuchsia.dev template engine #}
```gn
import("//build/config.gni")
{% verbatim %}
config_data("workstation_fonts") {
  for_pkg = "fonts"
  sources = [
    "RobotoMono-Light.ttf",
    "RobotoMono-Medium.ttf",
    "RobotoMono-Regular.ttf",
  ]
  outputs = [ "fonts/{{source_file_part}}" ]
}{% endverbatim %}
```

### Using `config_data()` in your component

Note: If you are using [legacy components][legacy-components],
see [configuration data][config-migration] in the components migration guide.

Include the following in your component manifest (`.cml`) file:

```json5
{
    use: [
        {
            directory: "config-data",
            rights: [ "r*" ],
            path: "/config/data",
        },
    ],
}
```

At runtime your component will be able to read the config files at the path
`/config/data`.

For the above to work, `"config-data"` must be offered to your component.
For instance your parent may have a declaration that looks like this:

```json5
{
    children: [
        {
            name: "font-provider",
            url: "fuchsia-pkg://fuchsia.com/fonts#meta/font-provider.cm",
        },
    ],
    offer: [
        {
            directory: "config-data",
            from: "parent",
            to: [ "#font-provider" ],
            subdir: "fonts",
        },
    ],
}
```

Note that both `for_pkg = ...` and `subdir: ...` above are coordinated in that
they set the same value `"fonts"`.

### Testing `config_data()`

A component under test in a test realm can have a `"config-data"` directory
routed to it in much the same way as a production component would.

If you would like to offer a component under test different configuration data,
simply use the appropriate value for `for_pkg` and `subdir` that would route
your test data to your test component.

### How `config_data()` works

All `config_data()` targets that are defined in your build configuration collect
their files into a single package called `config-data`. This package is defined
in the system assembly as part of the base package set. The contents of this
package replicate the parameters in `config_data()` definitions, so that they
can be routed as subdirectories to components that expect them.

### Known issues

*   The `config-data` package that collects all files from `config_data()`
    definitions is part of the base set of packages. As a result its contents do
    not update in the `fx serve` developer workflow. To update data files you
    must repave or OTA your device, or if using an emulator rebuild the system
    image and restart the emulator.

*   Defining `config_data()` also requires making changes to component manifest
    files as shown above. Some of the strings used are repeated in multiple
    places, which is error-prone. When mistakes are made they can be difficult
    to troubleshoot.

*   `config_data()` target definitions know about the name of the package(s)
    of components that are expected to use this data. This promotes brittle
    contracts that are difficult and perilous to evolve. For instance in order
    for the platform to offer [ICU data][icu-data] to out-of-tree components and
    their tests, there exists a
    [hard-coded list of out-of-tree package names][icu-data-configs] in the
    Fuchsia source tree.

Due to the above, always prefer using `resource()` if possible.

[additional-packaged-resources]: /docs/development/components/build.md#additional-packaged-resources
[build-components]: /docs/development/components/build.md
[config-migration]: /docs/development/components/v2/migration/features.md#config-data
[gn-placeholders]: https://gn.googlesource.com/gn/+/HEAD/docs/reference.md#placeholders
[icu-data]: /docs/development/internationalization/icu_data.md
[icu-data-configs]: /src/lib/icu/tzdata/BUILD.gn
[legacy-components]: /docs/concepts/components/v1/README.md
[metafar]: /docs/concepts/packages/package.md#metafar
[resource]: /build/dist/resource.gni
