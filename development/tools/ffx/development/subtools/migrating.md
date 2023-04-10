# Migrating existing plugins to subtools

## Why migrate

The new [FHO](/development/tools/ffx/architecture/fho.md) subtool library
provides some helpful macros for migrating an existing plugin over to the new
subtool interface. By migrating your plugin to a subtool, you have these benefits:

* A clearer interface boundary for [errors](errors.md).
* Type-safe machine [writer](writers.md) output and eventually schema validation.
* More flexibility in what kinds of inputs your subtool takes from the fuchsia
and `ffx` environment (see the FHO docs linked above).
* Less macro magic.
* Faster build times when built separately from `ffx` itself.

Please note that if you are starting a new subtool, you can skip a lot of the
steps that were involved in the legacy plugin system, so you should follow the
[getting started](getting-started.md) instructions instead.

## Non-recursive plugins

Plugins that don't have sub-plugins, with no `plugin_deps` section in their
`ffx_plugin()` action, are relatively simple to migrate. Migrating
plugins that have sub-plugins is treated as a separate task.

The following instructions are largely based on the differences between the
[`ffx daemon echo`](/src/developer/ffx/plugins/daemon/echo) plugin as originally
written and the [`ffx echo`](/src/developer/ffx/tools/echo) subtool used as a
proof of concept in developing the new subtool interface.

### Migrating the rust `lib.rs`:

Given a plugin `lib.rs` file that looks like this:

```rust
use anyhow::Result;
use ffx_core::ffx_plugin;
use ffx_echo_args::EchoCommand;
use ffx_writer::Writer;
use fidl_fuchsia_developer_ffx::EchoProxy;

#[ffx_plugin(EchoProxy = "daemon::protocol")]
pub async fn echo(echo_proxy: EchoProxy, cmd: EchoCommand, #[ffx(machine = String)] mut writer: Writer) -> Result<()> {
    // implementation here
    Ok(())
}
```

The simplest way to migrate to the new plugin system is to remove the
`#[ffx_plugin]` macro and add the following derive-macro-based code:

```rust
use fho::{FfxTool, FfxMain, MachineWriter, Result};
use ffx_echo_args::EchoCommand;
use fidl_fuchsia_developer_ffx::EchoProxy;

#[derive(FfxTool)]
pub struct EchoTool {
    #[command]
    cmd: EchoCommand,
    #[with(fho::daemon_protocol())]
    echo_proxy: ffx::EchoProxy,
}

#[async_trait(?Send)]
impl FfxMain for EchoTool {
    type Writer = MachineWriter<String>;
    async fn main(self, writer: MachineWriter<String>) -> Result<()> {
        // implementation here
        Ok(())
    }
}
```

Unlike the previous macro, there are no restrictions on naming the members of
the `EchoTool` struct, and you can even implement your own loaders by deriving
a trait from FHO if you want to do something more complicated.

The `main()` function is given this struct and the writer as moved objects, so
you can feel free to deconstruct them however you like.

Also note that the `Result` type comes from FHO and not `anyhow`. At this
boundary, there is now a specific error type that holds information about whether
or not the error is user-surfaceable and how to present it. See [errors](errors.md)
for more information on how to work with this error type and its interactions
with the `ffx_error` and `ffx_bail` macros.

For now, most migrated plugins still need to be included into the main `ffx`
binary, and that means you'll need to also add the following macro invocation
to generate the legacy plugin entry points:

```rust
fho::embedded_plugin!(EchoTool);
```

### Adding a `main.rs`

Since your subtool can now be run independently, you will need to add a simple
`main.rs` that will call into FHO and your plugin lib to properly run when
`ffx` invokes it:

```rust
use ffx_tool_echo::EchoTool;
use fho::FfxTool;

#[fuchsia_async::run_singlethreaded]
async fn main() {
    EchoTool::execute_tool().await
}
```

In general, you won't ever have a `main.rs` any more complicated than
this.

### BUILD.gn for top level plugins

For existing plugins that still need to integrate with the existing `ffx` plugin
system, these plugins will continue using the `ffx_plugin` GN action since it sets all the
dependencies up properly for inclusion in the `ffx` binary.

If the plugin is a top level subcommand of `ffx`, then we'll also add an
`ffx_tool()` action as well to build the separately compiled plugin:

```gn
# Existing import for the plugin action
import("//src/developer/ffx/build/ffx_plugin.gni")
# New import for the ffx_tool action.
import("//src/developer/ffx/build/ffx_tool.gni")

ffx_plugin("ffx_echo") {
  version = "0.1.0"
  edition = "2021"
  with_unit_tests = true

  args_sources = [ "src/args.rs" ]

  sources = [ "src/lib.rs" ]

  deps = [ "//sdk/fidl/fuchsia.developer.ffx:fuchsia.developer.ffx_rust" ]

  test_deps = [ "//src/lib/fuchsia-async" ]
}

# This will generate the executable file for your plugin.
ffx_tool("ffx_echo_tool") {
  edition = "2021"
  output_name = "ffx-echo"
  deps = [
    ":ffx_echo",
    "//src/developer/ffx/lib/fho:lib",
    "//src/lib/fuchsia-async",
  ]
  sources = [ "src/main.rs" ]

  # To be included in the sdk in the future, add this:
  sdk_category = "partner"
  sdk_target_name = "ffx_echo_tool_sdk"
}
```

### Adding separate subtool building to the SDK

At this point, it will be *possible* to build the subtool but it won't be
actively built. In order to do that, you'll need to add it to the list of
"dual-mode plugins" in [the `ffx` config.gni](/src/developer/ffx/config.gni):

```gn
# ...snip...
dual_mode_plugins = [
  # ...
  "//path/to/your/plugin:ffx_echo_tool",
  # ...
]
# ...snip...
```

Now, running `fx build ffx` should cause both `ffx` and your plugin to be rebuilt,
and if you run `ffx commands` you should be able to see your command in the
"Workspace Commands" list.
