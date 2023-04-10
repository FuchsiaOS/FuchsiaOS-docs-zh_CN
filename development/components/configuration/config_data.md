# Product-specific configuration with `config_data()`

{% include "docs/_common/_deprecation_notice.md" %}

Sometimes a component is defined in one repository but its data is defined in
another repository. For instance `fuchsia.git` defines a font provider service,
but the `workstation_eng` product configuration (defined in a different repository)
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

## Using `config_data()` in your component

Note: If you are using legacy components,
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

## Testing `config_data()`

A component under test in a test realm can have a `"config-data"` directory
routed to it in much the same way as a production component would.

If you would like to offer a component under test different configuration data,
simply use the appropriate value for `for_pkg` and `subdir` that would route
your test data to your test component.

## How `config_data()` works

All `config_data()` targets that are defined in your build configuration collect
their files into a single package called `config-data`. This package is defined
in the system assembly as part of the base package set. The contents of this
package replicate the parameters in `config_data()` definitions, so that they
can be routed as subdirectories to components that expect them.

## Known issues

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

Due to the above issues, prefer using a different
[configuration mechanism][config-mechanisms] if possible.

[config-mechanisms]: mechanisms.md
[config-migration]: /docs/development/components/v2/migration/features.md#config-data
[icu-data]: /docs/development/internationalization/icu_data.md
[icu-data-configs]: /src/lib/icu/tzdata/BUILD.gn
[resource]: /build/dist/resource.gni
