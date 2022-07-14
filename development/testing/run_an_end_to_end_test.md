# Run an end-to-end test

This guide provides instructions on how to run an end-to-end test for testing a
Fuchsia product.

This guide uses the Fuchsia emulator ([FEMU](/get-started/set_up_femu.md)) to
emulate a device that runs Fuchsia. As for the end-to-end test, the guide uses
the
[`screen_is_not_black`](/src/tests/end_to_end/screen_is_not_black/)
end-to-end test.

The `screen_is_not_black` end-to-end test reboots a device under test
(DUT), waits 100 seconds, and takes a snapshot of the device’s screen. If the
snapshot image is not a black screen, the test concludes that Fuchsia is
successfully up and running on the device after reboot.

To run this end-to-end test, the steps are:

1.  [Prerequisites](#prerequisites).
1.  [Build a Fuchsia image to include the end-to-end test](#build-a-fuchsia-image-to-include-the-end-to-end-test).
1.  [Start the emulator with the Fuchsia image](#start-the-emulator-with-the-fuchsia-image).
1.  [Run the end-to-end test](#run-the-end-to-end-test).

Also, to run any end-to-end test, see the [Appendices](#appendices) section.

## 1. Prerequisites {#prerequisites}

This guide requires that you've completed the following guides:

*   [Download the Fuchsia source code](/get-started/get_fuchsia_source.md).
*   [Start the Fuchsia emulator](/get-started/set_up_femu.md).

## 2. Build a Fuchsia image to include the end-to-end test {#build-a-fuchsia-image-to-include-the-end-to-end-test}

Before you can run the `screen_is_not_black` end-to-end test, you first
need to build your Fuchsia image to include the test in the build artifacts:

Note: The examples in this guide use the `workstation_eng` product. End-to-end tests work with most
products except `core`.

1.  To add the end-to-end test, run the `fx set` command with the following
    `--with` option:

    ```posix-terminal
    fx set workstation_eng.qemu-x64 --with //src/tests/end_to_end/screen_is_not_black
    ```

    `//src/tests/end_to_end/screen_is_not_black` is a test directory in the
    Fuchsia source tree. The
    <code>[BUILD.gn](/src/tests/end_to_end/screen_is_not_black/BUILD.gn)</code>
    file in this directory defines the <code>screen_is_not_black</code> target
    to include the <code>screen_is_not_black</code> end-to-end test in the build
    artifacts.

1.  Build your Fuchsia image:

    ```posix-terminal
    fx build
    ```

    When the `fx build` command completes, the build artifacts now include the
    `screen_is_not_black` end-to-end test, which you can run from your host
    machine.

## 3. Start the emulator with the Fuchsia image {#start-the-emulator-with-the-fuchsia-image}

Start the emulator with your Fuchsia image and run a
[package repository server](/development/build/fx.md#serve-a-build):

Note: The steps in this section assume that you don't have any terminals
currently running FEMU or the `fx serve` command.

1.  Configure an IPv6 network for the emulator:

    Note: This has to be completed once per machine.

    ```posix-terminal
    sudo ip tuntap add dev qemu mode tap user $USER && sudo ip link set qemu up
    ```

1. Configure the upscript:

    Note: If your machine is behind a firewall, you may need to apply some additional
    configuration to allow the emulator to access the network. This is typically
    accomplished by running an "upscript", which sets up the interfaces and firewall
    access rules for the current process. If you're on a corporate network, check
    with your internal networking team to see if they have an existing upscript
    for you to use.
    If you're not behind a firewall, there's still some configuration needed to
    enable tun/tap networking. The example upscript
    at <code>{{ '<var>' }}FUCHSIA_ROOT{{ '</var>' }}/scripts/start-unsecure-internet.sh</code>
    should work for the majority of non-corporate users.

    ```posix-terminal
    ffx config set emu.upscript {{ '<var>' }}PATH_TO_UPSCRIPT{{ '</var>' }}
    ```

    Replace the following:

    * `PATH_TO_UPSCRIPT`: The path to a FEMU network setup script; for example,
    `~/fuchsia/scripts/start-unsecure-internet.sh`.

1. Start the package server

   ```posix-terminal
   fx serve
   ```

1.  Start the emulator:

    ```posix-terminal
    ffx emu start --net tap
    ```

    When startup is complete, the emulator prints the following message and opens
    a shell prompt:

    ```none {:.devsite-disable-click-to-copy}
    Logging to "{{ '<var>' }}$HOME{{ '</var>' }}/.local/share/Fuchsia/ffx/emu/instances/fuchsia-emulator/emulator.log"
    Waiting for Fuchsia to start (up to 60 seconds)........Emulator is ready.
    ```

    1. The `--net` flag requires a value to indicate which kind of
    networking to implement. `--net` has the following possible values:

        - `tap`: Attaches a Tun/Tap interface.
        - `user`: Sets up mapped ports through SLiRP.
        - `none`: Disables networking.
        - `auto`: Checks the host system's capabilities and selects `tap` if it is
            available or `user` if a tap interface is unavailable.
            `auto` is the default.

    `auto` is the default if the flag is not specified on the command line.
    The upscript is automatically executed only if the user selects `tap`
    mode.

    If `auto` is used, the launcher checks for a tap interface on the
    device. If it finds a tap interface, it uses `tap` mode; otherwise it
    uses `user` mode.

1.  Run the `fx set-device` command and select `fuchsia-emulator` (the
    emulator's default device name) to be your device, for example:

    <pre>
    $ fx set-device
    ERROR: Multiple devices found, please pick one from the list:
    1) fuchsia-4407-0bb4-d0eb
    2) fuchsia-emulator
    #? <b>2</b>
    New default device: fuchsia-emulator
    </pre>

## 4. Run the end-to-end test {#run-the-end-to-end-test}

Run the `screen_is_not_black` end-to-end test:

```posix-terminal
fx test --e2e screen_is_not_black
```

When the test passes, this command prints output similar to the
following:

```none {:.devsite-disable-click-to-copy}
Saw a screen that is not black; waiting for 0:00:59.924543 now.

...

[FINE]: Running over ssh: killall sl4f.cmx
01:46 +1: All tests passed!
```

## Appendices {#appendices}

### Run any end-to-end test {#run-any-end-to-end-test}

Use the `fx test --e2e` command to run an end-to-end test from your host
machine:

```posix-terminal
fx test --e2e <TEST_NAME>
```

Some product configurations may include a set of end-to-end tests by default
(see [Examine product configuration files](#examine-product-configuration-files)).
However, if you want to run an end-to-end test that is not part of your
product configuration, configure your Fuchsia image to include the specific
test:

```posix-terminal
fx set <PRODUCT>.<BOARD> --with <TEST_DIRECTORY>:<TARGET>
```

For example, the following commands configure and build your Fuchsia image
with all the end-to-end tests in the
<code>[//src/tests/end_to_end/perf](/src/tests/end_to_end/perf/)</code> test
directory:

```none {:.devsite-disable-click-to-copy}
$ fx set workstation_eng.qemu-x64 --with //src/tests/end_to_end/perf:test
$ fx build
```

Note: Some end-to-end tests are designed to run only on specific product
configurations.

For the list of all available end-to-end tests in the Fuchsia repository, see
the [//src/tests/end\_to\_end](/src/tests/end_to_end/) directory.

### Examine product configuration files {#examine-product-configuration-files}

To find out which end-to-end tests are included in a
specific product configuration, examine product configuration files (`.gni`) in
the Fuchsia repository's <code>[//products][products-dir]</code> directory.

The following example shows the product configurations files in the
`//products` directory:

```none {:.devsite-disable-click-to-copy}
~/fuchsia/products$ ls *.gni
bringup.gni  core.gni  terminal.gni  workstation_eng.gni
```
To see the list of all available product configurations, you can run the
following command:

```posix-terminal
fx list-products
```

Among these product configurations, <code>[terminal][terminal-gni]</code> and
<code>[workstation_eng][workstation-gni]</code> include end-to-end tests by
default. The following example shows the end-to-end tests included
in `terminal.gni`:

```none {:.devsite-disable-click-to-copy}
cache_package_labels += [
  ...
  "//src/tests/end_to_end/bundles:end_to_end_deps",
  "//src/tests/end_to_end/bundles:terminal_end_to_end_deps",
]

...

universe_package_labels += [
  "//src/tests/end_to_end/screen_is_not_black",
  "//src/tests/end_to_end/sl4f:test",
  "//src/tests/end_to_end/perf:test",
  ...
]
```

<!-- Reference links -->

[products-dir]: /products/
[terminal-gni]: /products/terminal.gni
[workstation-gni]: /products/workstation_eng.gni
