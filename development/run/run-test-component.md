# Run a test component

This document describes how to run a test component using the Fuchsia emulator (FEMU).

Note: This guide is specific to [components v2](/docs/concepts/components/v2).

A component instance is started in Fuchsia when another component requests a
[capability](/docs/concepts/components/v2/capabilities/README.md) from it. For
example, the test manager
(which is [a component](/docs/concepts/components/v2/introduction.md#everything_is_a_component_almost))
starts a test component in response to a request to run a test.

This guide uses the
<code>[hello-world-bin-test](/examples/components/basic/meta/hello-world-bin-test.cml)</code>
component in the <code>[basic](/examples/components/basic)</code> example package. When you
run this package’s `hello-world-tests` test suite, the test manager starts the
`hello-world-bin-test` component. As a result, the component’s test binary runs
on a Fuchsia device, or in this case, on the Fuchsia emulator.

The steps to run a test component are:

*   [Build a Fuchsia image](#build-a-fuchsia-image)
*   [Start the emulator](#start-the-emulator)
*   [Run the test suite](#run-the-test-suite)

## Prerequisites

Before you can run this test component, you must:

*   [Set up the Fuchsia development environment](/docs/get-started/get_fuchsia_source.md)

## Build a Fuchsia image {#build-a-fuchsia-image}

Configure and build your Fuchsia image to include the test component:

1.  To include a specific component, run the `fx set` command with the `--with`
    option:

    ```posix-terminal
    fx set core.qemu-x64 --with //examples/components/basic:hello-world-tests
    ```

    `//examples/components/basic` is the directory of the example package and
    `hello-world-tests` is the name of the build target defined in the package's
    <code>[BUILD.gn](/examples/components/basic/BUILD.gn)</code> file.

1.  Build your Fuchsia image:

    ```posix-terminal
    fx build
    ```

    When the `fx build` command completes, your new Fuchsia image now includes
    the `hello-world-bin-test` component, which can be
    [fetched and launched on demand](/docs/concepts/build_system/boards_and_products.md#universe).

## Start the emulator {#start-the-emulator}

Start the emulator with your Fuchsia image and run a
[package repository server](/docs/development/build/fx.md#serve-a-build):

Note: The steps in this section assume that you have
[set up and configured FEMU](/docs/get-started/set_up_femu.md).

1.  In a new terminal, start the emulator:

    ```posix-terminal
    fx emu -N
    ```

1.  Set the emulator to be your device:

    ```posix-terminal
    fx set-device
    ```

    If you have multiple devices, select `fuchsia-5254-0063-5e7a` (the emulator’s
    default device name), for example:

    <pre>
    $ fx set-device

    Multiple devices found, please pick one from the list:
    1) fuchsia-4407-0bb4-d0eb
    2) fuchsia-5254-0063-5e7a
    #? <b>2</b>
    New default device: fuchsia-5254-0063-5e7a
    </pre>

1.  In another new terminal, start a package repository server:

    ```posix-terminal
    fx serve
    ```

    Keep the `fx serve` command running as a package server for your device.

## Run the test suite {#run-the-test-suite}

Run the `hello-world-tests` test suite:

```posix-terminal
fx test hello-world-tests
```

This command prints the following output:

```none
$ fx test hello-world-tests

...

[0/1] 00:00 🤔  /home/fuchsia/.jiri_root/bin/fx shell run-test-suite fuchsia-pkg://fuchsia.com/hello-world-tests#meta/hello-world-bin-test.cm
 >> Runtime has exceeded 2 seconds (adjust this value with the -s|--slow flag)
Running test 'fuchsia-pkg://fuchsia.com/hello-world-tests#meta/hello-world-bin-test.cm'

[RUNNING]   tests::assert_0_is_0
[PASSED]    tests::assert_0_is_0
1 out of 1 tests passed...
fuchsia-pkg://fuchsia.com/hello-world-tests#meta/hello-world-bin-test.cm completed with result: PASSED

[1/1] 00:05 ✅  /home/fuchsia/.jiri_root/bin/fx shell run-test-suite fuchsia-pkg://fuchsia.com/hello-world-tests#meta/hello-world-bin-test.cm

🎉  Ran 1 tests with 0 failures (use the -v flag to see each test) 🎉
```

The output shows that the `hello-world-bin-test` component is fetched from the
package repository server and the component instance runs the test binary on the
Fuchsia device (the emulator). See
<code>[hello_world.rs](/examples/components/basic/src/hello_world.rs)</code>
for the source code of this test binary.
