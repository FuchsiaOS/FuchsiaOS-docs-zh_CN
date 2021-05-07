# Using FIDL proxies with plugins

FFX plugins can communicate with a target device using FIDL through
[Overnet](/src/connectivity/overnet/).

The examples in this doc extend the example code from the [plugins](/docs/development/tools/ffx/development/plugins.md) page.

First we add a dependency to the plugin's `BUILD.gn` file:

```GN
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

This makes the FIDL proxy bindings available for usage in the
plugin. In this case you can now import `NameProviderProxy`.

```rust
use {
    anyhow::Result,
    ffx_core::ffx_plugin,
    ffx_example_args::ExampleCommand,
    fidl_fuchsia_device::NameProviderProxy,
};

```

Now that the type is imported, the proxy can be used in the plugin
function. FFX plugins can accept proxies in the parameter list:

```rust
pub async fn example(
    name_proxy: NameProviderProxy,
    _cmd: ExampleCommand,
) -> Result<()> { }
```

In order to correctly connect a proxy to the FIDL service on the
target, you will need to map the proxy type to a component selector
that can be used to find the FIDL service.  More about component
selectors can be found on the [Component Select](/docs/development/tools/ffx/commands/component-select.md)
page. This mapping is passed into the ffx_plugin annotation at the top
of the function signature:

```rust
#[ffx_plugin(
    NameProviderProxy = "core/appmgr:out:fuchsia.device.NameProvider"
)]
```

Putting it all together, your `src/lib.rs` file should look like:

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

And that's it.  The plugin should now be able to communicate with
target device using native FIDL calls.  FFX plugins can accept any
number of proxies as long as the same steps are followed and the
proxies are correctly mapped to component selectors.

There are two exceptions to this rule.  FFX already knows how to
communicate with two proxies without mappings.  It's enough to just
add these proxies to the parameter list without changing the
ffx_plugin annotation:

- [DaemonProxy](/sdk/fidl/fuchsia.developer.bridge/daemon.fidl)
- [Remote Control Service (RCS)](/sdk/fidl/fuchsia.developer.remotecontrol/remote-control.fidl)
