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

For information about `{% verbatim %}{{source_file_part}}{% endverbatim %}`
and similar syntax see [GN placeholders][gn-placeholders]{:.external}.

See [resource.gni][resource-gni] for more usage instructions on the `resource()`
template, and related templates `resource_group()` and `resource_tree()`.

More examples using `resource()`, `resource_group()`, and `resource_tree()` can
be found in [additional packaged resources][additional-packaged-resources].

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

[additional-packaged-resources]: /docs/development/components/build.md#additional-packaged-resources
[build-components]: /docs/development/components/build.md
[gn-placeholders]: https://gn.googlesource.com/gn/+/HEAD/docs/reference.md#placeholders
[metafar]: /docs/concepts/packages/package.md#metafar
[resource-gni]: /build/dist/resource.gni
