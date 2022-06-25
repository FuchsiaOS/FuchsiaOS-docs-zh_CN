# Building drivers

Caution: This page may contain information that is specific to the legacy
version of the driver framework (DFv1). Also the workflows documented on
this page may only be specific to the Fuchsia source checkout
(`fuchsia.git`) environment.

This document demonstrates how to build a driver, highlighting best
practices for defining packages, components, and drivers.

## Concepts {#concepts}

Drivers are a type of component, so before going much further it is important
to understand how to
[build components](development/components/build.md).

## Driver package GN templates {#driver-package}

Driver authors should be using driver-specific GN templates.
Fuchsia defines the following GN templates to define driver components and
driver packages:

*   [`fuchsia_driver_component.gni`](/build/drivers/fuchsia_driver_component.gni)
*   [`fuchsia_driver_package.gni`](/build/drivers/fuchsia_driver_package.gni)

Below is a hypothetical package containing one driver component:

```gn
import("//build/drivers.gni")


driver_bind_rules("bind") {
    rules = "driver.bind"
    bind_output = "my-driver.bindbc"
}

fuchsia_driver("driver") {
  output_name = "my-driver"
  sources = [ "my_driver.cc" ]
}

fuchsia_driver_component("component") {
  component_name = "my-driver"
  deps = [
      ":driver",
      ":bind",
  ]
}

fuchsia_driver_package("my_package") {
  deps = [ ":component" ]
}
```

Note the following details:
*   The `fuchsia_driver_component()` template declares the component.
    It depends on the driver shared library (the `fuchsia_driver()`), as well
    as the driver's bind rules (the `driver_bind_rules()`).
*   The `fuchsia_driver_component()` automatically generates a component manifest
    for the driver. We will see what that looks like in a later section.
*   Both the component and package names are derived from their target names.
    In the example above, these names come together to form the URL for
    launching the component:
    `fuchsia-pkg://fuchsia.com/my_package#meta/my_driver.cm`.

### What does the auto-generated component manifest look like?

When you use a `fuchsia_driver_component` template it will auto-generate
the component manifest for the driver. For the above example, it will look like
the following

```
{
    program: {
        runner: 'driver',
        binary: driver/my_driver.so,
        bind: meta/bind/my_driver_bind.bindbc
    }
}
```

Note the following details:
*   The `binary` field points to the driver shared library.
*   The `bind` field points to the driver's bind rules file.

### Can I include my own component manifest?

Sure! In order to write your own component manifest, simply add
it into the project as a file and update the `fuchsia_driver_component`
to point to it:

```
fuchsia_driver_component("component") {
  component_name = "my-driver"
  manifest = "meta/my-own-manifest.cml
  deps = [
      ":driver",
      ":bind",
      ]
}
```

## Including your driver in the build.

In order to include your driver in the build, it needs to go into
two special places.

The first is
[`//build/drivers/all_drivers_list.txt`](/build/drivers/all_drivers_list.txt).
If you don't do this, then you will see a build error reminding you about this.
The `all_drivers_list.txt` file should contain all of the driver labels included
in the fuchsia repository. This list is kept up-to-date so the Driver Framework
team can ensure that all drivers continue to be supported and updated.

If your driver can only build for x64 then please add it to:

`//build/drivers/all_drivers_lists_x64.txt`

If your driver can only build for arm64 then please add it to:

`//build/drivers/all_drivers_lists_arm64.txt`

The second location is:
[`//bundles:drivers-build-only`](//bundles/BUILD.gn).
There will also be a build error if you forget to add your driver here.
To add to this list, you should make sure to add your driver component to
the local driver group in your source location.

For example, for a new driver added under `//src/ui/input/drivers`, you would
add an entry to `//src/ui/input:drivers`.

For the `drivers-build-only` target, you need to be sure that you're including
the path to your `fuchsia_driver_components()` target, and not point to your
`fuchsia_driver()` target directly.

## Including your driver on a device

If you want to include your driver on an actual device there are two options:
you can include the driver in the bootfs or as a base package.

### Including your driver in bootfs

You can include your driver in bootfs depending on the product, the board, or
just your local environment. You're going to want to add the path to your
`fuchsia_driver_component` target to one of these three locations:

* product: `product_bootfs_labels` in the relevant `.gni` file in `//products`
* board: `board_bootfs_labels` in the relevant `.gni` file in `//boards`
* locally: `board_bootfs_labels` in `fx args`

### Including your driver in a base package

You can include your driver in a base package depending on the product, the board, or
just your local environment. You're going to want to add the path to your
`fuchsia_package` target to one of these three locations:

* product: `product_driver_package_labels` in the relevant `.gni` file in `//products`
* board: `board_driver_package_labels` in the relevant `.gni` file in `//boards`
* locally: `base_driver_package_labels` in `fx args`
