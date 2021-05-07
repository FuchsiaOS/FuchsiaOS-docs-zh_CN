# Rust stress test library

This document explains how to write a [stress test](stress_tests.md) using the rust stress test
library. The library can be found at `//src/sys/lib/stress-test`. It implements the test loop and
the concurrency and synchronization primitives required for running these tests.

### Writing a stress test

#### Define the GN build targets

Define `rustc_binary`, `fuchsia_component` and `fuchsia_test_package` GN build targets
for your test:

```
rustc_binary("filesystem-stressor-bin") {
    deps = [
        ...
        "//src/sys/lib/stress-test",
        ...
    ]
    sources = [
        ...
    ]
}

fuchsia_component("filesystem-stressor") {
    deps = [ ":filesystem-stressor-bin" ]
    manifest = "meta/filesystem-stressor.cmx"
    testonly = true
}

fuchsia_test_package("filesystem-stress-tests") {
  test_components = [
    ":filesystem-stressor"
  ]
}
```

#### Write an actor

Every actor must implement the `Actor` trait. The actor trait is one method `perform()` that is
invoked by an `ActorRunner`. When invoked, the actor must perform exactly one operation and return
its result to the runner. An actor must stores all the connections necessary to perform operations.

```rust
pub trait Actor: Sync + Send + 'static {
    // ActorRunner invokes this function, instructing the actor
    // to perform exactly one operation and return result.
    async fn perform(&mut self) -> Result<(), ActorError>;
}
```

An actor can indicate the following with the return result:

* `Ok(())`: Operation succeeded and is added to the global operation count.

* `Err(ActorError::DoNotCount)`: The operation must not be counted towards the global operation
count.

* `Err(ActorError::ResetEnvironment)`: The environment must be reset and the operation must not be
counted towards the global operation count.

When an actor encounters an unexpected error, it should panic, thus stopping the test.

Since actors are operating on the same environment, it is possible that their operations will
collide. For example, for a filesystem stress test, actors may operate on the same set of files.
If such collisions are desirable, you must setup actors to handle such collisions gracefully.
If not, the actor should panic, causing the test to stop.

An actor can intentionally break the system-under-test, requiring the environment to be reset. For
example, for a filesystem stress test, an actor can randomly sever the connection between the
filesystem and the underlying block device. In this example, other actors should request a new
environment with `ActorError::ResetEnvironment`, and the environment will re-establish connections
for all of the actors.

Note: The mutable connections of the actor should be marked public, so that the environment can
update them during reset.

```rust
pub struct FilesystemActor {
    /// Store a connection to the root of filesystem here
    pub root_directory: Directory
    ...
}

impl FilesystemActor {
    pub fn new(root_directory: Directory) -> Self {
        ...
    }
}

#[async_trait]
impl Actor for FilesystemActor {
    async pub fn perform(&mut self) -> Result<(), ActorError> {
        // Choose exactly one operation to do on the filesystem
        // using the root_directory
        self.root_directory.delete_all_files();
    }
}
```

#### Write an Environment

The environment provides the basic configuration for the stress test - the exit criteria,
the actors and a reset method.

```rust
pub trait Environment: Send + Sync + Debug {
    /// Returns the target number of operations to complete before exiting
    fn target_operations(&self) -> Option<u64>;

    /// Returns the number of seconds to wait before exiting
    fn timeout_seconds(&self) -> Option<u64>;

    /// Return the runners for all the actors
    fn actor_runners(&mut self) -> Vec<ActorRunner>;

    /// Reset the environment, when an actor requests one
    async fn reset(&mut self);
}
```

An environment can store additional configuration for the test. You can provide this configuration
through the command line with the `argh` crate.

An actor is shared between a runner and the environment and hence it must be wrapped as an
`Arc<Mutex<dyn Actor>>`. Runners hold the lock while an actor is performing an operation.
This means that the environment can only acquire an actor's lock between operations.

An environment is instructed to reset when an actor determines that the current instance of the
system-under-test has been broken. The environment is expected to create a new instance for the
system-under-test and lock on the actors to update their connections to the new instance.

The environment must also implement the `Debug` trait. Stress tests log the environment
when the test starts and if the test panics. It is common practice to print out parameters that are
valuable for reproducing the test, such as the random seed used.

```rust
#[derive(Debug)]
pub struct FilesystemEnvironment {
    fs_actor: Arc<Mutex<FilesystemActor>>,
    seed: u128,
    ...
}

impl Environment {
    pub fn new() -> Self {
        ...
    }
}

#[async_trait]
impl Environment for FilesystemEnvironment {
    fn target_operations(&self) -> Option<u64> {
        // By specifying None here, the test will run without an operation limit
        None
    }

    fn timeout_seconds(&self) -> Option<u64> {
        // By specifying None here, the test will run without a time limit
        None
    }

    fn actor_runners(&mut self) -> Vec<ActorRunner> {
        vec![
            ActorRunner::new(
                "filesystem_actor",  // debug name
                60,  // delay (in seconds) between each operation (0 means no delay)
                self.fs_actor.clone()), // actor
            )
        ]
    }

    async fn reset(&mut self) {
        // If the actor is performing an operation, this will remain
        // locked until the operation is complete.
        let actor = self.fs_actor.lock().await;

        // Now the environment can update the actor before it is run again.
        actor.root_directory = ...;

        // Releasing the lock will resume the runner.
    }
}
```

#### Write the main function

The main function of a stress test is straightforward, since most of the logic is
implemented in the Environment and Actors. Use the main function to collect command-line
arguments (if any), initialize logging and set log severity.

Note: The stress test library offers a `StdoutLogger` that prints all logs to stdout. This
functionality can be used by any stress test that runs as a v1 (cmx) component.

```rust
#[fuchsia_async::run_singlethreaded]
async fn main() {
    // Print all logs to stdout.
    stress_test::StdoutLogger::init();

    // Create the environment
    let env = FilesystemEnvironment::new();

    // Run the test.
    // Depending on the exit criteria, this may never return.
    stress_test::run_test(env).await;
}
```

#### Running stress tests locally

Since a stress test is a part of a `fuchsia_test_package`, one of the easiest ways to run it
is with the `fx test` command:

```
fx test filesystem-stress-tests
```

Note: The stress test runs with the command line arguments defined in the component's manifest.

To run the test with custom command line arguments, use `fx shell run`:

```
fx shell run fuchsia-pkg://fuchsia.com/filesystem-stress-tests#meta/filesystem-stressor.cmx <args>
```

#### Running stress tests on infrastructure

A stress test is identified by infrastructure through the `stress-tests` tag that is attached to
the `fuchsia_test_package` or `fuchsia_unittest_package` GN Build Target.

```
fuchsia_test_package("filesystem-stress-tests") {
  test_components = [
    ":filesystem-stressor"
  ]
  test_specs = {
    environments = [
      {
        dimensions = {
          device_type = "QEMU"
        }
        tags = [ "stress-tests" ]
      },
    ]
  }
}
```

A dedicated `core.qemu-x64-stress` builder identifies these tests and runs each test component in
the package for a maximum of 22 hours.

Note: On infra bots, a stress test is required to show "signs of life" which is usually some form of
output to show that the test is still running and has not hung.

Note: Stress tests are currently restricted to the `QEMU` device type, since they run for long
periods of time.


## Debugging a stress test

The framework uses the rust `log` crate to log messages. The test logs the environment object at
start and if the test panics.

```
--------------------- stressor is starting -----------------------
Environment {
    seed: 268479717856254664270968796173957499835,
    filesystem_actor: { ... }
    ...
}
------------------------------------------------------------------
```

If debug logging is enabled, individual actor operations and operation counts are also logged.

```
DEBUG: [0][filesystem_actor][389] Sleeping for 2 seconds
DEBUG: [0][filesystem_actor][389] Performing...
DEBUG: [0][filesystem_actor][389] Done!
DEBUG: Counters -> [total:403] {"filesystem_actor": 403}
```
