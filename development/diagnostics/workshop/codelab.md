# Diagnostics and testing codelab

This document contains a codelab for debugging with diagnostics and tests. It is currently
intended for developers writing tests within fuchsia.git.

## Prerequisites

Set up your development environment.

This codelab assumes you have completed [Getting Started](/docs/get-started/README.md) and have:

1. A checked out and built Fuchsia tree.
2. A device or emulator (`ffx emu`) that runs Fuchsia.
3. A workstation to serve components (`fx serve`) to your Fuchsia device or emulator.

To build and run the examples in this codelab, add the following arguments
to your `fx set` invocation:

Note: Replace core.x64 with your product and board configuration.

```
fx set core.x64 \
--release \
--with //examples/diagnostics/workshop \
--with //examples/diagnostics/workshop:tests
```

## Introduction

There is an example component that serves a protocol called [ProfileStore][profile-store]:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/workshop/fidl/profile_store.test.fidl" region_tag="profile_store_fidl" adjust_indentation="auto" %}
```

This protocol allows creation, deletion, and inspection of user profiles, which contain a
name and balance. The component has a bug - profile deletion does not work.

Code for the codelab is located in [//examples/diagnostics/workshop](/examples/diagnostics/workshop).

## Run the component

In addition to the main component serving ProfileStore, there are a number of components that
connect to and interact with ProfileStore. All components are in the
`fuchsia-pkg://fuchsia.com/profile_store_example` package.

  * `#meta/profile_store.cm` - serves ProfileStore
  * `#meta/add_olive.cm` - Connects to ProfileStore and adds a profile called 'Olive'
  * `#meta/add_balance_olive.cm` - Connects to ProfileStore and adds balance to the 'Olive' profile
  * `#meta/withdraw_balance_olive.cm` - Connects to ProfileStore and withdraws balance from the
  'Olive' profile
  * `#meta/add_jane.cm` - Connects to ProfileStore and adds a profile called 'Jane'
  * `#meta/delete_olive.cm` - Connects to ProfileStore and deletes the 'Olive' profile

Capabilities are routed by the `#meta/laboratory_server.cm` component.

You can interact with the components using the `ffx component` command, and inspect output from
components using `ffx log`.
First, run `ffx log --tags workshop` in a shell. This shell will contain all output from
components. In a different shell, run the toy components:

```bash
# setup server
ffx component create /core/ffx-laboratory:profile_store fuchsia-pkg://fuchsia.com/profile_store_example#meta/laboratory_server.cm

# setup first client
ffx component create /core/ffx-laboratory:profile_store/clients:add_olive fuchsia-pkg://fuchsia.com/profile_store_example#meta/add_olive.cm

# see the results of the previous two steps
ffx component show profile_store

# add a profile key and read it
ffx component start /core/ffx-laboratory:profile_store/clients:add_olive
ffx component create /core/ffx-laboratory:profile_store/clients:reader fuchsia-pkg://fuchsia.com/profile_store_example#meta/profile_reader.cm
ffx component start /core/ffx-laboratory:profile_store/clients:reader

# demonstrate persistence
ffx component stop /core/ffx-laboratory:profile_store/profile_store
ffx component start /core/ffx-laboratory:profile_store/clients:reader

# update balance
ffx component create /core/ffx-laboratory:profile_store/clients:add_balance_olive fuchsia-pkg://fuchsia.com/profile_store_example#meta/add_balance_olive.cm
ffx component start /core/ffx-laboratory:profile_store/clients:add_balance_olive
ffx component start /core/ffx-laboratory:profile_store/clients:reader

# add second profile
ffx component create /core/ffx-laboratory:profile_store/clients:add_jane fuchsia-pkg://fuchsia.com/profile_store_example#meta/add_jane.cm
ffx component start /core/ffx-laboratory:profile_store/clients:add_jane
ffx component start /core/ffx-laboratory:profile_store/clients:reader

# update balance
ffx component create /core/ffx-laboratory:profile_store/clients:withdraw_balance_olive fuchsia-pkg://fuchsia.com/profile_store_example#meta/withdraw_balance_olive.cm
ffx component start /core/ffx-laboratory:profile_store/clients:withdraw_balance_olive
ffx component start /core/ffx-laboratory:profile_store/clients:reader

# delete olive (this will not work as there is a bug in the server code)
ffx component create /core/ffx-laboratory:profile_store/clients:delete_olive fuchsia-pkg://fuchsia.com/profile_store_example#meta/delete_olive.cm
ffx component start /core/ffx-laboratory:profile_store/clients:delete_olive
ffx component start /core/ffx-laboratory:profile_store/clients:reader
```

## Debugging with diagnostics

Diagnostics provides multiple products that help component authors debug their components both
while developing and in the field.

For this workshop we'll be exploring three core technologies:

- [Structured logging](#structured-logging)
- [Inspect](#inspect)
- [Triage](#triage)

### Structured logging

Diagnostics provides structured logging libraries to allow components to write logs.
To help find the bug, we'll be adding a few logs to the profile store component.

The first step when adding logging to a component, is to include the logging library
in your binary dependencies. To do this, update your [BUILD.gn][profile-store-build] as follows:

```
source_set("lib") {
  ...
  public_deps = [
    ...
    "//sdk/lib/syslog/cpp",
  ]
}
```

Logging is initialized the moment we call one of the logging macros. However, the libraries provide
some utilities that should be called in `main()` such as configuring the tags (if desired only, this
is optional).

Tags can be useful to later query the logs of a group of components. For our purposes we can add
the `workshop` tag:

```
#include <lib/syslog/cpp/log_settings.h>
...
syslog::SetTags({"workshop", "profile_store_server"});
```

Now, it's time to write some logs. We'll be using the `FX_SLOG` macro which allows to write
structured keys and values.

For example, we can add the following log when we get a request on `ProfileStore::Open` but
the profile file doesn't exist:

```
#include <lib/syslog/cpp/macros.h>
...
FX_SLOG(WARNING, "Profile doesn't exist", KV("key", key.c_str()));
```

Try adding that log, build (`fx build`), relaunch your component (`ffx component start ...`) and
then run: `ffx log --tags workshop`.

What other logs could we add that would help identify the log? Please experiment!

A solution can be found in [this patch](https://fuchsia-review.googlesource.com/c/fuchsia/+/684632).

### Inspect

Inspect allows components to expose state about themselves. Unlike logs, which are a stream,
Inspect represents a live view into the component current state.

Reading through the [Inspect quickstart][inspect-quickstart] would be a good first step. If you'd
like to dive deeper into Inspect, you can also follow the [Inspect codelab][inspect-codelab].

To get started, first add library dependencies:

```
source_set("lib") {
  ...
  public_deps = [
    ...
    "//sdk/lib/sys/inspect/cpp",
  ]
}
```

Next, initialize Inspect in `main.cc`:

```
#include <lib/sys/inspect/cpp/component.h>
...
// Immediately following the line defining "startup".
auto inspector = std::make_unique<sys::ComponentInspector>(startup.get());
inspector->Health().Ok();
```

Now you can view Inspect after restarting the `profile_store`:

```
# Note that double \\ is necessary! ':' must be escaped in a "diagnostic selector."
ffx inspect show core/ffx-laboratory\\:profile_store/profile_store
```

You should see your component's status is "OK". Inspect is most
useful when integrated with your class hierarchy. Arbitrary values
are rooted on `inspect::Node`, including more Nodes! Try to modify ProfileStore such that the following compiles:

```
  // In main.cc
  std::unique_ptr<ProfileStore> profile_store =
      std::make_unique<ProfileStore>(loop.dispatcher(), inspector->GetRoot().CreateChild("profiles"));
```

Hint: You will need to update the `ProfileStoreTests` class if you
change the constructor for `ProfileStore`. You can just pass
`inspect::Node()` as the new parameter.

Now that you have basic Inspect configured, what intrumentation could be useful to add to
help prevent/find the bug in this component?
- Consider adding an `inspect::Node` to each `Profile`, and create
  them using `CreateChild` on the node you passed to `ProfileStore`.
- Consider using `inspect::LazyNode` (`node.CreateLazyNode(...)`)
  to dynamically create hierarchies.

A possible solution can be found in this patch:
https://fuchsia-review.googlesource.com/c/fuchsia/+/682671

### Triage

Triage allows to write rules to automatically process inspect snapshots and find potential issues
or gather stats that the snapshots might contain.

Reading through the [Triage codelab][triage-codelab] would be a good first step as well as reading
through the [triage config guide][triage-config-guide].

To get started, create a new file at `examples/diagnostics/workshop/triage/profile_store.triage`
with the following contents:

```
{
  select: {
    profile_status: "INSPECT:core/ffx-laboratory\\:profile_store/profile_store:fuchsia.inspect.Health:status",
  },
  act: {
    profile_status_ok: {
      type: "Warning",
      trigger: "profile_status != 'OK'",
      print: "Profile store is not healthy.",
    }
  }
}
```

Note: Before checking in a `.triage` file, you would need to update a list in the `BUILD.gn` file.
For this example, that step is omitted.

If you followed the Inspect quick start in the last section, run
`ffx triage --config examples/diagnostics/workshop/triage/`. If `profile_store` is running and
reporting its status is OK, you won't see any failures! Try changing the call to
`Health().Ok()` in `main.cc` to `Health().StartingUp()` and run
`ffx triage --config examples/diagnostics/workshop/triage/` again. This time you should see the warning.

Try writing a triage configuration that could have helped spot the bug in snapshots gathered in the
field.

A possible solution (built on top of the Inspect solution) can be found in this patch:
https://fuchsia-review.googlesource.com/c/fuchsia/+/684762

## Verifying with tests

This section covers adding tests to verify the fix.

This example contains [example unit tests][example-unittests] and an
[example integration test][example-integration-test], including some tests that are
disabled due to the bug.

Try writing new tests that could help prevent the bug in the component. Feel free to either modify
the example tests, or create new tests from scratch using the flows below.

### Adding new unit tests

The code structure of unit tests is highly dependent on the runtime used. This section walks
through the process of setting up new unit tests that verify the behavior of the ProfileStore
C++ class.

Create a new file in `examples/diagnostics/workshop/profile_store_unittest.cc` with these contents:

```c++
#include "src/lib/testing/loop_fixture/test_loop_fixture.h"

#include <gtest/gtest.h>

#include "fuchsia/examples/diagnostics/cpp/fidl.h"
#include "lib/fidl/cpp/interface_ptr.h"
#include "profile_store.h"

class ProfileStoreTests : public gtest::TestLoopFixture {};

TEST_F(ProfileStoreTests, SampleTest) {
    ProfileStore store(dispatcher());
    fidl::InterfacePtr<fuchsia::examples::diagnostics::ProfileStore> store_ptr;
    store.AddBinding(store_ptr.NewRequest(dispatcher()));

    store_ptr->Delete("my_key", [&](bool successful) { EXPECT_FALSE(successful); });
    RunLoopUntilIdle();
}
```

This sets up a minimal unit test which creates a ProfileStore, and creates a client to interact
with it under an asynchronous test loop.
Next, you will create a component manifest for the test, which defines how to run the test.
Create a new component manifest for the test in
`examples/diagnostics/workshop/meta/profile_store_unittests.cm` with the following contents:

```
{
    include: [
        // Needed for gtest runners
        "//src/sys/test_runners/gtest/default.shard.cml",
        // Needed so that logs are created
        "syslog/client.shard.cml",
    ],
    program: {
        binary: "bin/profile_store_unittests",
    },
    use: [
        {
            // ProfileStore uses /data to store profiles. We'll use the tmp
            // storage provided to the test.
            storage: "tmp",
            path: "/data",
        },
    ],
}
```

Finally, add new build rules to `examples/diagnostics/workshop/BUILD.gn`:

```

# Builds the test binary.
executable("test_bin") {
  testonly = true
  output_name = "profile_store_unittests"

  sources = [
    "profile_store_unittest.cc",
  ]

  deps = [
    ":lib",
    "//src/lib/fxl/test:gtest_main",
    "//src/lib/testing/loop_fixture",
  ]
}

# Creates a test component and test package.
fuchsia_unittest_package("profile_store_unittests") {
  deps = [ ":test_bin" ]
  manifest = "meta/profile_store_unittests.cml"
}

# Update the existing group("tests") to include the new package as a dep
group("tests") {
  testonly = true
  deps = [
    # new dependency
    ":profile_store_unittests",

    ":profile_store_example_unittests",
    "example-integration:tests",
  ]
}
```

Next, verify that the test builds and runs.

```bash
# Build is needed the first time so that fx test becomes aware of the new test.
# For subsequent test executions, fx build is automatically invoked.
fx build examples/diagnostics/workshop:tests

fx test profile_store_unittests
```

You are now ready to modify the test code to verify behavior.

### Adding a new integration test

The `fx testgen` command autogenerates integration test boilerplate setup to use
[RealmBuilder][realm-builder]. To use it, you will need to find the compiled component manifest
of our profile_store component in our output directory.

```bash
# find the manifest in output directory.
find $(fx get-build-dir) -name profile_store.cm

# generate integration tests.
fx testgen --cm-location {{ '<var>' }}find result{{ '</var>' }} --out-dir examples/diagnostics/workshop/tests -c
```

This should generate a few files under `examples/diagnostics/workshop/tests`. Before running the
tests, there are a few build rules that need to be updated:

 * In the newly generated `examples/diagnostics/workshop/tests/BUILD.gn`
  * Replace `{COMPONENT_FIDL_BUILD_TARGET}` with the build target for the ProfileStore fidl -
  `//examples/diagnostics/workshop/fidl:fuchsia.examples.diagnostics`
  * Replace `{COMPONENT_BUILD_TARGET}` with the build target for the ProfileStore component -
  `//examples/diagnostics/workshop:profile_store`/
 * In `examples/diagnostics/workshop/BUILD.gn`
  * Add "tests" to deps in the `group("tests")` definition. This ensures GN can find the new test.

Next, verify that the test builds and runs.

```bash
# Build is needed the first time so that fx test becomes aware of the new test.
# For subsequent test executions, fx build is automatically invoked.
fx build examples/diagnostics/workshop:tests

fx test profile_store_test
```

Once the test runs, you are ready to modify the boilerplate to write useful tests.

[example-integration-test]: /examples/diagnostics/workshop/example-integration
[example-unittests]: /examples/diagnostics/workshop/profile_unittest_example.cc
[inspect-codelab]: /docs/development/diagnostics/inspect/codelab/codelab.md
[inspect-quickstart]: /docs/development/diagnostics/inspect/quickstart.md
[gtest]: https://github.com/google/googletest
[profile-store]: /examples/diagnostics/workshop/fidl/profile_store.test.fidl
[profile-store-build]: /examples/diagnostics/workshop/BUILD.gn
[realm-builder]: /docs/development/testing/components/realm_builder.md
[triage-codelab]: /docs/development/diagnostics/triage/codelab.md
[triage-config-guide]: /docs/development/diagnostics/triage/config.md
