# Integration testing

The `ffx` command supports a feature that creates an isolated environment for
running integration tests.

`ffx` is useful when writing integration tests which need to interact with the Fuchsia
environment. However, since `ffx` is primarily designed for developers, it inspects the
current environment for configuration and also starts a daemon in the background to
coordinate communication with Fuchsia devices. This makes it more complex to write
automated tests that use `ffx` since the configuration and daemon should be isolated
in order to avoid side effects, or interference from the global environment.

To achieve this isolation, `ffx` supports using _isolation directories_. This feature
creates a new isolated environment for `ffx` to run in, including a user level
configuration. The `ascendd` socket, which is the connection to the `ffx` daemon, is
created in this directory.

All `ffx` invocations which use an isolate must specify it on the `ffx` commandline with
the `--isolate-dir` option. This can also be specified by setting the `__FFX_ISOLATE_DIR__`
environment variable.

When the test is completed, the test author needs to clean up the isolate directory.

## Rust isolate library

In the Fuchsia source tree, developers should use
[`//src/developer/ffx/lib/isolate`][ffx-lib-isolate] in their test to create and interact
with the isolate directory.

### Initialize the global context for a test

To avoid initializing the global context from the host environment, tests need to create
a test environment to initialize the global data. The environment is cleaned up when the
return value is dropped, so it must be in scope for the life of the test.

```rust
let test_env = ffx_config::test_init().await?;
```

### Create the isolate

There are two methods to create a new Isolate, depending on the environment
that ffx will operate. For tests that run as part of the in-tree test and
rely on the build output directory structure, use `Isolate::new_in_test()`.
For tests that are part of a ffx subtool, or SDK based, use
`Isolate::new_in_sdk()`.
If the test is interacting with externally provisioned devices, the path to the
_SSH private key_ should be passed in as well. If the test is initializing the
device or starting an emulator, SSH keys will be generated as needed, but a
path to where to store the keys is needed to be configured after the isolate
is created.

The isolate directory is cleaned up when the isolate is dropped,
so it must live for the entire test.

#### Example Isolate::new_in_test()

```rust
  let test_case_name = "my test";
  let ssh_path = std::env::var("FUCHSIA_SSH_KEY").unwrap().into();
  let test_env = ffx_config::test_init().await
      .expect("Setting up test environment");
  // This takes advantage of knowing that Rust tests are down one level from the
  // build output root directory.
  let build_root =
        std::env::current_exe().unwrap().canonicalize().unwrap().parent().unwrap().to_owned();
  let isolate = ffx_isolate::Isolate::new_in_test(test_case_name,
                                                  build_root,
                                                  ssh_path,
                                                  &test_env.context).await
            .expect("create isolate");
```

#### Example Isolate::new_in_sdk()

```rust
  let test_case_name = "my test";
  let ssh_path = std::env::var("FUCHSIA_SSH_KEY").unwrap().into();
  let test_env = ffx_config::test_init().await
      .expect("Setting up test environment");
  let isolate = ffx_isolate::Isolate::new_with_sdk(test_case_name, ssh_path, &test_env.context)
            .await
            .expect("create isolate");
```

## Running ffx commands

To run commands in the context of the isolate, the `Isolate::ffx()` method is used.
This wrapper adds the correct options to the ffx command line to use the isolate directory.

```rust {:.devsite-disable-click-to-copy}
let args = vec!["target list"];
let output = isolate.ffx(args).await?;
```

## Configuration inside the isolate

Configuring values and defaults for `ffx` inside the isolate is done using the `ffx config`
commandline:

```rust {:.devsite-disable-click-to-copy}
let args = vec!["config", "set", "ssh.pub", &path_to_ssh_authorized_keys.to_string_lossy()];
let output = isolate.ffx(args).await?;
```

## Log files

The log file path for `ffx` when using an isolate is configured when the isolate is created.
A common practice is for the test framework to set the environment variable
`__FUCHSIA_TEST_OUTDIR__` for files that need to be accessible as output from the test. The
log directory is created in a subdirectory of `__FUCHSIA_TEST_OUTDIR__` if it is configured.
The path to the log directory is accessed with `log_dir()`.

```rust {:.devsite-disable-click-to-copy}
let log_dir = isolate.log_dir();
```

## Default target

Some test frameworks may allocate a device for a test to run. The isolate reads the
`__FUCHSIA_DEVICE_ADDR__` environment variable and sets it as the default target in the
configuration.

Note: A side effect of providing the `FUCHSIA_DEVICE_ADDR` value is that `mdns` discovery
of devices is disabled in the `ffx` daemon to improve isolation.

## Analytics configuration

The isolate disables analytics collection via configuration.

<!-- Reference links -->

[ffx-lib-isolate]: /src/developer/ffx/lib/isolate
