# Developing an ffx plugin

This page describes the basic steps for creating a plugin for `ffx`.

The plugin system employs a combination of GN build rules and Rust attributes
to decouple plugin code from `ffx` internals.

## GN Build Rule {#gn-rule}

Use the [`ffx_plugin()`](/src/developer/ffx/build/ffx_plugin.gni) build rule
template in your project's `BUILD.gn` file to create a build target for your
plugin.

Your `BUILD.gn` file should look similar to the following example:

```gn
import("//src/developer/ffx/build/ffx_plugin.gni")

ffx_plugin("ffx_example") {
  version = "0.1.0"
  edition = "2018"
  with_unit_tests = true
  deps = []
  sources = [
    "src/args.rs",
    "src/lib.rs",
  ]
}
```

Note: `ffx_plugin()` wraps the `rustc_library()` build template, so the same set
of attributes are available.

Inside the `src/` directory, the project should contain two source files:

-   `src/args.rs`: Defines the CLI parameters for your plugin.
-   `src/lib.rs`: Contains the main plugin source code implementation.

## Arguments {#args}

Create the file `src/args.rs` containing the plugins supported arguments:

```rust
use {argh::FromArgs, ffx_core::ffx_command};

#[ffx_command()]
#[derive(FromArgs, Debug, PartialEq)]
#[argh(subcommand, name = "example", description = "an example")]
pub struct ExampleCommand {}
```

This uses the [argh](https://docs.rs/argh/0.1.3/argh/) crate and more
documentation can be found [here](https://docs.rs/argh/0.1.3/argh/). This
struct has been decorated by the `ffx_command` attribute that signifies that
your plugin should run when someone types the following command:

```posix-terminal
fx ffx example
```

If you want to add more parameters for your plugins, you add them to
this struct.

An example parameter would look like this:

```rust
use {argh::FromArgs, ffx_core::ffx_command};

#[ffx_command()]
#[derive(FromArgs, Debug, PartialEq)]
#[argh(subcommand, name = "example", description = "an example")]
pub struct ExampleCommand {
    #[argh(positional)]
    /// example optional positional string parameter
    pub example: Option<String>,
}
```

See more documentation:
- [Argh](https://docs.rs/argh/0.1.3/argh/)

## Plugin {#plugin}

Create the file `src/lib.rs` containing the plugin implementation:

```rust
use {
    anyhow::Result,
    ffx_core::ffx_plugin,
    ffx_example_args::ExampleCommand,
};

#[ffx_plugin()]
pub async fn example(_cmd: ExampleCommand) -> Result<()> {
    println!("Hello from the example plugin :)");
    Ok(())
}
```

Plugin methods need to accept the argh command created in the `src/args.rs`
file as a parameter even if they do not use them.

Note: The `ffx_example_args::ExampleCommand` in the above example is generated
automatically by the `ffx_plugin()` template. For more details, see
[plugin internals](plugin-internals.md).

## Integration {#integration}

Add your plugin library as a dependency to `ffx` to include it in the build.
Edit the `plugin_deps` array in the [`ffx` build target][ffx-build] to add your
`ffx_plugin()` target to the top level:

```gn
  plugin_deps = [
    "//path/to/your/plugin/dir:ffx_example",
    ...
  ]
```

Note: Alternatively, you can add your plugin to the dependency list of another
plugin to create a subcommand.

To build and test your plugin, build `ffx`:

```posix-terminal
fx build ffx
```

You should now see the output when you run your example command:

```none {:.devsite-disable-click-to-copy}
$ fx ffx example
Hello from the example plugin :)
```

## Unit tests {#unit-tests}

If you want to unit test your plugin, just follow the standard method for
testing [rust code][rust-testing] on a host. The `ffx_plugin()` GN template
generates a `<target_name>_lib_test` library target for unit tests when the
`with_unit_tests` parameter is set to `true`.

If your `lib.rs` contains tests, they can be invoked using `fx test`:

```posix-terminal
fx test ffx_example_lib_test
```

If fx test doesn't find your test, check that the product configuration includes your test. You can include all the ffx tests with this command:
```posix-terminal
fx set ... --with=//src/developer/ffx:tests
```

## FIDL protocols {#fidl-proxy}

FFX plugins can communicate with a target device using FIDL protocols through
[Overnet][overnet]. To access FIDL protocols from your plugin, follow the
instructions in this section.

1.  Add the FIDL Rust bindings as a dependency to the plugin's `BUILD.gn` file.
    The following example adds bindings for the `fuchsia.device` FIDL library:

    ```gn
    import("//src/developer/ffx/build/ffx_plugin.gni")

    ffx_plugin("ffx_example") {
      version = "0.1.0"
      edition = "2018"
      with_unit_tests = true
      deps = [
        "//sdk/fidl/fuchsia.device:fuchsia.device-rustc",
      ]
      sources = [
        "src/args.rs",
        "src/lib.rs",
      ]
    }
    ```

1.  Import the necessary bindings int your plugin implementation. The following
    example imports `NameProviderProxy` from `fuchsia.device`:

    ```rust
    use {
        anyhow::Result,
        ffx_core::ffx_plugin,
        ffx_example_args::ExampleCommand,
        fidl_fuchsia_device::NameProviderProxy,
    };
    ```

1.  Include the FIDL proxy can be used in the plugin implementation. Plugins can
    accept proxies in the parameters list:

    ```rust
    pub async fn example(
        name_proxy: NameProviderProxy,
        _cmd: ExampleCommand,
    ) -> Result<()> { }
    ```

1.  Map the proxy type to a [component selector][component-select] representing
    the component providing the FIDL protocol in the `ffx_plugin()` annotation:

    ```rust
    #[ffx_plugin(
        NameProviderProxy = "core/appmgr:out:fuchsia.device.NameProvider"
    )]
    ```

The example plugin implementation in `src/lib.rs` should now look like the
following:

```rust
use {
    anyhow::Result,
    ffx_core::ffx_plugin,
    ffx_example_args::ExampleCommand,
    fidl_fuchsia_device::NameProviderProxy,
};

#[ffx_plugin(
    NameProviderProxy = "core/appmgr:out:fuchsia.device.NameProvider"
)]
pub async fn example(
    name_proxy: NameProviderProxy,
    _cmd: ExampleCommand,
) -> Result<()> {
    if let Ok(name) = name_proxy.get_device_name().await? {
        println!("Hello, {}", name);
    }
    Ok(())
}
```

Repeat these steps to include additional FIDL proxies to your `ffx` plugin.

The following FIDL proxies are built into `ffx`, and do not require additional
dependencies or mappings:

- [DaemonProxy](/sdk/fidl/fuchsia.developer.ffx/daemon.fidl)
- [Remote Control Service (RCS)](/sdk/fidl/fuchsia.developer.remotecontrol/remote-control.fidl)

You can simply add the above proxies to your plugin's parameter list to access
them  in your implementation.

### Proxy selector maps {#selector-maps}

`ffx` and the Remote Control Service (RCS) provide a mechanism for maintaining
compatibility with existing selectors used by `ffx` plugins if the selector
representing a given FIDL proxy changes. For example:

-   The FIDL proxy is provided by a new component
-   The FIDL protocol name changes
-   The proxy selector varies across product builds

RCS supports this using *selector maps* that override the selectors defined in
an `ffx` plugin's source and map it to a different value. To override a given
selector, add an entry to
[`//src/developer/remote-control/data/selector-maps.json`][selector-maps]
in the following format:

```json {:.devsite-disable-click-to-copy}
{
  ...
  "original/moniker:out:fuchsia.MyService": "some/new/moniker:expose:fuchsia.MyOtherService"
}
```

This example enables RCS to override references to
`original/moniker:out:fuchsia.MyService` in `ffx` plugins and route them to
`some/new/moniker:expose:fuchsia.MyOtherService` in any build which contains
the mapping.

[component-select]: development/tools/ffx/commands/component-select.md
[ffx-build]: /src/developer/ffx/BUILD.gn
[overnet]: /src/connectivity/overnet/
[rust-testing]: development/languages/rust/testing.md
[selector-maps]: /src/developer/remote-control/data/selector-maps.json
