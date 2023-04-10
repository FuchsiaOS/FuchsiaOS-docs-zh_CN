# Getting started with `ffx` subtools

FFX Subtools are the top-level commands that the [ffx cli](/development/tools/ffx/architecture/cli.md)
can run. These can be either compiled directly into `ffx` and/or build as separate
commands that can be found in the build output directory or the SDK, and they
will then be invoked using the [FHO tool interface](/development/tools/ffx/architecture/fho.md).

This document describes how to get started writing a new subtool for `ffx`.
If you already have a plugin that was written before the new interface and want
to migrate it to the new subtool interface, you can find more information on that
in the [migrating doc](migrating.md).

Note: The tool produced by this document will only be runnable as an external subtool.
The `ffx` maintainers will generally not be accepting new top level tools as being integrated into
the `ffx` binary unless there's a very strong reason for it, in which case please
ask the FFX team for guidance.

## Where to put it

First, create a directory somewhere in the fuchsia.git tree to hold
your subtool. Currently subtools exist in these locations:

* The [ffx plugins tree](/src/developer/ffx/plugins), where built-in only and
hybrid plugin/subtools go. New subtools should not generally be put here.
* The [ffx tools tree](/src/developer/ffx/tools), where external-run-only subtools
go. Putting it here makes it easier for the maintainers of `ffx` to assist with any issues or
update your subtool with any changes to the interface between `ffx` and the tool.
If you put it here and the FFX team isn't the primary maintainer of this tool,
you *must* put an `OWNERS` file in the directory you put it in that adds your
team's component and some individual owners so we know how to triage issues with
your tool.
* Somewhere in a project's own tree. This can make sense if the ffx tool is
simply a wrapper over an existing program, but if you do this you *must*
have your `OWNERS` files set up so that the FFX team can approve updates to the
parts that interact with `ffx`. You can do this by adding `file:/src/developer/ffx/OWNERS`
to your `OWNERS` file over the subdirectory the tool is in.

Other than not putting new tools in plugins, the decision of a specific location
may require discussion with the tools team to decide on the best place.

## What files

Once you've decided where your tool is going to go, create the source
files. Unlike the legacy plugin system, subtools aren't required to be broken out
into three separate rust libraries. Still, best practices are to have your
tool's code broken out into a library that implements things and a `main.rs` that
simply calls into that library.

The following file set would be a normal starting point:

```
BUILD.gn
src/lib.rs
src/main.rs
OWNERS
```

But of course you can break things up into more libraries if you want. Note that
these examples are all based on the example [echo subtool](/src/developer/ffx/tools/echo),
but parts may be removed or simplified for brevity. Take a look at the files in
that directory if anything here doesn't work or seems unclear.

### `BUILD.gn`

Following is a simple example of a `BUILD.gn` file for a simple subtool. Note
that, if you're used to the legacy plugin interface, the `ffx_tool` action doesn't
impose a library structure on you or do anything really complicated. It's a fairly
simple wrapper around the `rustc_binary` action, but adds some extra targets
for generating metadata, producing a host tool, and producing sdk atoms.

```gn
import("//build/rust/rustc_library.gni")
import("//src/developer/ffx/build/ffx_tool.gni")

rustc_library("lib") {
  name = "ffx_echo"
  edition = "2021"
  with_unit_tests = true

  deps = [
    "//src/developer/ffx/lib/fho:lib",
    "//third_party/rust_crates:argh",
    "//third_party/rust_crates:async-trait",
  ]

  test_deps = [
    #...
  ]

  sources = [ "src/lib.rs" ]
}

ffx_tool("ffx_echo") {
  edition = "2021"
  output_name = "ffx-echo"
  deps = [
    ":lib",
    "//src/developer/ffx/lib/fho:lib",
    "//src/lib/fuchsia-async",
  ]
  sources = [ "src/main.rs" ]

  sdk_category = "partner"
  sdk_target_name = "ffx_echo_sdk"
}

group("echo") {
  deps = [
    ":ffx_echo",
    ":ffx_echo_host_tool",
    ":ffx_echo_sdk",
  ]
}

group("tests") {
  testonly = true
  deps = [ ":lib_test" ]
}
```

### `main.rs`

The main rust file will usually be fairly simple, simply invoking FHO with
the right types to act as an entry point that `ffx` knows how to communicate
with:

```rust
use ffx_tool_echo::EchoTool;
use fho::FfxTool;

#[fuchsia_async::run_singlethreaded]
async fn main() {
    EchoTool::execute_tool().await
}
```

### `lib.rs`

This is where the main code of your tool will go. In here you will set up an
argh-based struct for command arguments and derive an `FfxTool` and `FfxMain`
implementation from a structure that will hold context your tool needs to run.

#### Arguments

```rust
#[derive(FromArgs, Debug, PartialEq)]
#[argh(subcommand, name = "echo", description = "run echo test against the daemon")]
pub struct EchoCommand {
    #[argh(positional)]
    /// text string to echo back and forth
    pub text: Option<String>,
}
```

This is the struct that defines any arguments your subtool needs after its
subcommand name.

#### The tool structure

```rust
#[derive(FfxTool)]
#[check(AvailabilityFlag("echo.enabled"))]
pub struct EchoTool {
    #[command]
    cmd: EchoCommand,
    #[with(daemon_protocol())]
    echo_proxy: ffx::EchoProxy,
}
```

This is the structure that holds context your tool needs. This includes things
like the argument structure defined above, any proxies to the daemon or a
device you might need, or potentially other things that you can define yourself.

There must be an element in this struct that references the argument type
described above, and it should have the `#[command]` attribute on it so that
the correct associated type can be set for the `FfxTool` implementation.

Anything in this structure must implement the `TryFromEnv` or have a `#[with()]`
annotation that points to a function that returns something that implements
`TryFromEnvWith`. There are also several implementations of these built in to
the `fho` library or other `ffx` libraries.

Also, the `#[check()]` annotation above the tool uses an implementation of
`CheckEnv` to validate that the command should be run without producing an item
for the struct itself. The one here, `AvailabilityFlag`, checks for
experimental status and exits early if it's not enabled. When writing a new
plugin, it should have this declaration on it to discourage people from relying
on it before it's ready for wider use.

#### The `FfxMain` implementation

```rust
#[async_trait(?Send)]
impl FfxMain for EchoTool {
    type Writer = MachineWriter<String>;
    async fn main(self, writer: Self::Writer) -> Result<()> {
        let text = self.cmd.text.as_deref().unwrap_or("FFX");
        let echo_out = self
            .echo_proxy
            .echo_string(text)
            .await
            .user_message("Error returned from echo service")?;
        writer.item(&echo_out)?;
        Ok(())
    }
}
```

Here you can implement the actual tool logic. You can specify a type for the
[`Writer`](writers.md) associated trait and that type will (through `TryFromEnv`) be
initialized for you based on the context `ffx` is run in. Most new plugins should
use the `MachineWriter<>` type, specifying a less generic type than the example
`String` above, but what makes sense will vary by tool. In the future, it may
be required that all new tools implement a machine interface.

Also, the result type of this function defaults to using the fho `Error` type,
which can be used to differentiate between errors that are due to user
interaction and errors that are unexpected. This maps to the way the legacy
plugin interface discriminated between normal `anyhow` errors and errors
produced by `ffx_error` or `ffx_bail`. More information on that can be found
in the [errors](errors.md) document.

#### Tests

Please add some tests! You can look at some existing examples of subtools to get
a better idea of how to do this, but it is otherwise much like writing any other
test for Fuchsia rust code.

### `OWNERS`

If this subtool is in the `ffx` tree, you will need to add an `OWNERS` file that
tells us who is responsible for this code and how to route issues with it in
triage. It should look something like the following:

```OWNERS
file:/path/to/authoritative/OWNERS

# COMPONENT: SomeComponent
```

It's better to add it as a reference (with `file:` or possible `include`) than
as a direct list of people, so that it doesn't get stale due to being out of the
way.

If your subtool is elsewhere, you will need to add the maintainers of `ffx` to the `OWNERS`
file for this directory so we can review updates to the interface between ffx
and your subtool:

```OWNERS
file:/src/developer/ffx/OWNERS

# any other project-specific things here
```

## Adding to the build

To add the tool to the GN build graph as a host tool, you'll need to reference it
in [the main list](/src/developer/ffx/tools/BUILD.gn) in the `ffx` tools gn file,
added to the `public_deps` of both the `tools` and `test` groups.

After this, if you `fx build ffx` you should be able to see your tool in the list
of `Workspace Commands` in the output of `ffx commands` and you should be able
to run it.

## Adding to the SDK

Once your tool has stabilized and you're ready to include it in the SDK, you'll
want to add it in to the `host_tools` molecule of the [sdk gn file](/sdk/BUILD.gn):

```gn
# ..snip..
sdk_molecule("host_tools") {
  visibility = [ ":*" ]

  _host_tools = [
    #...
    "//path/to/your/tool:ffx_echo_sdk", # <-- insert this
    #...
  ]
]
```

Note that before doing this, it should no longer have an experimental flag on it
and it should be considered relatively stable and well tested (as much as possible
without having already included it in the SDK, at least).
