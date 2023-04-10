# Running the Rust test suite on Fuchsia

This guide details the process for running the Rust compiler test suite on an
emulated Fuchsia image.

## Prerequisites

Before running the Rust test suite on Fuchsia, you'll need to
[build a custom Rust toolchain for Fuchsia]. Building a Fuchsia image is not
required for running the test suite.

[build a custom Rust toolchain for Fuchsia]: /docs/development/build/rust_toolchain.md

## Running the Rust test suite

1. (Optional) Set a custom temporary directory:

   ```posix-terminal
   export TEST_TOOLCHAIN_TMP_DIR={{ '<var>' }}TEMP_DIR{{ '</var>' }}
   ```

   If a temporary directory isn't set, then it will be named `tmp~` and will be
   created next to `rust_toolchain.py`. If you use a custom temporary directory,
   you'll need to set `TEST_TOOLCHAIN_TMP_DIR` to the same path in every shell
   that you use to interact with that same test environment.

   You can set `TEST_TOOLCHAIN_TMP_DIR` to different values in different shells
   to manage multiple test environments simultaneously.

2. Start the testing environment:

   ```posix-terminal
   DEV_ROOT={{ '<var>' }}DEV_ROOT{{ '</var>' }}
   TEST_TOOLCHAIN={{ '<var>' }}RUST_SRC{{ '</var>' }}/src/ci/docker/scripts/fuchsia-test-runner.py

   python3 $TEST_TOOLCHAIN start \
     --rust $DEV_ROOT/rust/install/fuchsia-rust \
     --sdk $DEV_ROOT/sdk \
     --target-arch {{ '<var>' }}x64|arm64{{ '</var>' }}
   ```

   Note: If the host architecture doesn't match the target architecture,
   emulation may be too slow to run the test suite effectively. In most cases,
   you'll want them to match.

3. Run the rust test suite:

   ```posix-terminal
   DEV_ROOT={{ '<var>' }}DEV_ROOT{{ '</var>' }}
   TEST_TOOLCHAIN={{ '<var>' }}RUST_SRC{{ '</var>' }}/src/ci/docker/scripts/fuchsia-test-runner.py

   ( \
     source $DEV_ROOT/rust/fuchsia-env.sh && \
     $DEV_ROOT/rust/x.py \
       --config $DEV_ROOT/rust/fuchsia-config.toml \
       --stage=2 \
       test {{ '<var>' }}TEST_SUITE{{ '</var>' }} \
       --target {{ '<var>' }}x86_64|aarch64{{ '</var>' }}-fuchsia \
       --run=always \
       --jobs 1 \
       --test-args --target-rustcflags \
       --test-args -L \
       --test-args --target-rustcflags \
       --test-args $DEV_ROOT/sdk/arch/{{ '<var>' }}x64|a64{{ '</var>' }}/sysroot/lib \
       --test-args --target-rustcflags \
       --test-args -L \
       --test-args --target-rustcflags \
       --test-args $DEV_ROOT/sdk/arch/{{ '<var>' }}x64|a64{{ '</var>' }}/lib \
       --test-args --target-rustcflags \
       --test-args -Cpanic=abort \
       --test-args --target-rustcflags \
       --test-args -Zpanic_abort_tests \
       --test-args --remote-test-client  \
       --test-args $TEST_TOOLCHAIN \
   )
   ```

   The test results will be printed to standard output.

4. Stop the testing environment:

   ```posix-terminal
   DEV_ROOT={{ '<var>' }}DEV_ROOT{{ '</var>' }}
   TEST_TOOLCHAIN={{ '<var>' }}RUST_SRC{{ '</var>' }}src/ci/docker/scripts/fuchsia-test-runner.py

   python3 $TEST_TOOLCHAIN stop
   ```

## Investigating test failures

Running the test suite produces many logs and other artifacts to aid in
investigating failing test suite tests. These can be dumped to standard output
during `fuchsia-test-runner.py stop` if the `--verbose` flag was used to start the
testing environment.

Note: These files are cleaned up by default when running the `stop` command. You
can preserve them for investigation by running `stop` with the `--no-delete`
argument. They can later be deleted using the `delete-tmp` command. The
`delete-tmp` command can also be used to clean up crashed and broken test
environments.

| Path | Description |
|------|-------------|
| `emulator_log` | Logs from the running emulator. This will typically contain detailed information about why tests crashed or failed to start. |
| `ffx_daemon_log` | Logs from the running `ffx` daemon. |
| `package_server_log` | Logs from the running `pm` server. |
| `test_env.json` | The configuration settings for the test environment. This can be useful for locating specific details like the `libstd` and `libtest` being used, or the address of the emulator. |
| `vdl_output` | The launched device proto. |
| `ffx_isolate` | The isolation environment for `ffx`. The `ffx` configuration settings can be found in here. |
| `output` | The raw output from `ffx test` for each test suite test. Each test directory contains all of the details `ffx` received about the test execution. These can be useful for finding relevant sections for specific tests in `emulator_log`. |
| `packages` | The package stages for each test. |
| `rust-testing` | The repository serving directory. |
| `ssh` | The SSH keys that the test runner uses to access the running emulator. These can be used to log into the emulator with `ssh -i ssh/fuchsia_ed25519 EMULATOR_ADDRESS`. The emulator address can be located in `test_env.json`. |

### Debugging test failures

1. Read the standard output and error from the test. This can be found in the
   `output` directory for the test, and will additionally be reported by
   compiletest.
2. Examine the detailed emulator logs. These can be found in `emulator_log` by
   searching for the name of the test that matches the name of the `output`
   directory.
3. Debug the test with `zxdb`. You can easily attach to a test by running:

   ```posix-terminal
   DEV_ROOT={{ '<var>' }}DEV_ROOT{{ '</var>' }}
   TEST_TOOLCHAIN={{ '<var>' }}RUST_SRC{{ '</var>' }}/src/ci/docker/scripts/fuchsia-test-runner.py

   python3 $TEST_TOOLCHAIN debug \
     --rust-src {{ '<var>' }}RUST_SRC{{ '</var>' }} \
     --test {{ '<var>' }}TEST_PATH{{ '</var>' }}
   ```

   Note: If you have the Fuchsia source available, you can additionally pass
   `--fuchsia-src` with the Fuchsia source path and `zxdb` will include source
   for zircon and Fuchsia.

   Then set any relevant breakpoints and run the test with:

   ```posix-terminal
   DEV_ROOT={{ '<var>' }}DEV_ROOT{{ '</var>' }}
   TEST_TOOLCHAIN={{ '<var>' }}RUST_SRC{{ '</var>' }}/src/ci/docker/scripts/fuchsia-test-runner.py

   ( \
     source $DEV_ROOT/rust/fuchsia-env.sh && \
     $DEV_ROOT/rust/x.py \
       --config $DEV_ROOT/rust/fuchsia-config.toml \
       --stage=2 \
       test {{ '<var>' }}TEST_SUITE{{ '</var>' }} \
       --target {{ '<var>' }}x86_64|aarch64{{ '</var>' }}-fuchsia \
       --run=always \
       --jobs 1 \
       --test-args --target-rustcflags \
       --test-args -L \
       --test-args --target-rustcflags \
       --test-args $DEV_ROOT/sdk/arch/{{ '<var>' }}x64|a64{{ '</var>' }}/sysroot/lib \
       --test-args --target-rustcflags \
       --test-args -L \
       --test-args --target-rustcflags \
       --test-args $DEV_ROOT/sdk/arch/{{ '<var>' }}x64|a64{{ '</var>' }}/lib \
       --test-args --target-rustcflags \
       --test-args -Cpanic=abort \
       --test-args --target-rustcflags \
       --test-args -Zpanic_abort_tests \
       --test-args --remote-test-client  \
       --test-args $TEST_TOOLCHAIN \
       --rustc-args -Cdebuginfo=2 \
       --rustc-args -Copt-level=0 \
       --rustc-args  -Cstrip=none \
   )
   ```

   And `zxdb` will catch any crashes and break at any breakpoints you define.
   This command is the same as the one above with the additional
   `-C debuginfo=2 -C opt-level=0` flags for debugging.

### Tips

To tighten your iteration loop, you can run `x.py test` with the `-vv` argument
to have compiletest print the exact commands it runs along the way. You can
quickly rerun a single test by running the `compiletest` invocation printed
immediately before the test is run.
