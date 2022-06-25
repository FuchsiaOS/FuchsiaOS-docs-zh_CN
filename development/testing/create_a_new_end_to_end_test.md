# Create a new end-to-end test

This guide provides instructions on how to create a new end-to-end test using
the [Dart test library](https://pub.dev/packages/test){:.external} and
[SL4F](/docs/development/drivers/concepts/driver_development/sl4f.md).

The guide creates an end-to-end test that prints “Hello world!” in the log of a
Fuchsia device. Once you verify that you can build and run this test from your
machine, use the [resources](#edit-the-test) provided in this guide as reference
to further develop the test.

To create a new end-to-end test, the steps are:

1.  [Prerequisites](#prerequisites).
1.  [Create a new test](#create-a-new-test).
1.  [Build the test](#build-the-test).
1.  [Start the emulator](#start-the-emulator).
1.  [Run the test](#run-the-test).
1.  [Edit the test](#edit-the-test).
1.  [Update and run the test](#update-and-run-the-test).

## 1. Prerequisites {#prerequisites}

This guide requires that you're familiar with the following tasks:

*   [Configure and build a Fuchsia image](/docs/get-started/build_fuchsia.md).
*   [Start the emulator (FEMU)](/docs/get-started/set_up_femu.md).
*   [Run an end-to-end test](/docs/development/testing/run_an_end_to_end_test.md).

## 2. Create a new test {#create-a-new-test}

An end-to-end test needs to have the following directory structure and files:

```none {:.devsite-disable-click-to-copy}
//src/tests/end_to_end/<your_test_directory>
                       ├── test
                       │   └── <test_source_code_files>
                       ├── BUILD.gn
                       ├── OWNERS
                       ├── README.md
                       ├── pubspec.yaml
                       └── analysis_options.yaml
```

The instructions in this section create the minimum directory structure and
files necessary to build and run an end-to-end test.

Do the following:

1.  Go to your Fuchsia directory, for example:

    Note: If your Fuchsia source code is not located in the `~/fuchsia`
    directory, replace `~/fuchsia` with your Fuchsia directory.

    ```posix-terminal
    cd ~/fuchsia
    ```

1.  Create a new branch, for example:

    ```posix-terminal
    git checkout -b create_my_first_e2e_test
    ```

1.  Go to the `//src/tests/end_to_end` directory:

    ```posix-terminal
    cd src/tests/end_to_end
    ```

1.  Create a new test directory called `my_e2e_test_example`, which has a `test`
    directory:

    ```posix-terminal
    mkdir -p my_e2e_test_example/test
    ```

1.  Go to the new test directory:

    ```posix-terminal
    cd my_e2e_test_example
    ```

1.  Use a text editor to create a new `my_new_e2e_test.dart` file in the `test`
    directory, for example:

    ```posix-terminal
    nano test/my_new_e2e_test.dart
    ```

1.  Paste the following code to `my_new_e2e_test.dart`:

    ```
    import 'package:sl4f/sl4f.dart' as sl4f;
    import 'package:test/test.dart';

    void main() {
      sl4f.Sl4f sl4fDriver;

      setUp(() async {
        sl4fDriver = sl4f.Sl4f.fromEnvironment();
        await sl4fDriver.startServer();
      });

      tearDown(() async {
        await sl4fDriver.stopServer();
        sl4fDriver.close();
      });

      test('tests hello world', () async {
        await sl4f.DeviceLog(sl4fDriver).info('Hello world!');
        print('Printed "Hello world!" in the device\'s log.');
      }, timeout: Timeout(Duration(minutes: 1)));
    }
    ```

    The `test()` function in this code prints `Hello world!` in the device's
    log, then the test outputs the `Printed "Hello world!" in the device's log.`
    message on the host machine's screen.

1.  Save the file and exit the text editor.

1.  Use a text editor to create a new `BUILD.gn` file, for example:

    ```posix-terminal
    nano ./BUILD.gn
    ```

1.  Paste the following code to `BUILD.gn`:

    ```
    import("//build/dart/test.gni")

    dart_test("my_new_e2e_test") {
      sources = [ "my_new_e2e_test.dart" ]
      deps = [ "//sdk/testing/sl4f/client", "//third_party/dart-pkg/pub/test", ]
    }

    group("test") {
      testonly = true
      deps = [ ":my_new_e2e_test($host_toolchain)" ]
    }
    ```

    This
    <code>[BUILD.gn](/docs/development/build/build_system/fuchsia_build_system_overview.md#build_targets)</code>
    file defines the <code>test</code> target group to include
    <code>my_new_e2e_test</code>.

1.  Save the file and exit the text editor.

1.  Copy an existing `analysis_options.yaml` file to your test directory, for
    example:

    ```posix-terminal
    cp ../sl4f/analysis_options.yaml .
    ```

    The Dart compiler uses this file to identify compile warnings.

1.  Create empty `pubspec.yaml`, `OWNERS`, and `README.md` files:

    ```posix-terminal
    touch pubspec.yaml OWNERS README.md
    ```

    Some text editors use the `pubspec.yaml` file to recognize that this test is
    a Dart package. Provide the content of `OWNERS` and `README.md` files later
    when you contribue the test to the Fuchsia project.

## 3. Build the test {#build-the-test}

Before you can run an end-to-end test, you first need to configure and build a
Fuchsia image to include the test in the build artifacts:

Note: The examples in this guide use the `workstation` product. End-to-end tests work
with most products except `core`.

1.  Configure your Fuchsia image to include the `my_e2e_test_example` test
    directory and the `test` target group:

    ```posix-terminal
    fx set workstation.qemu-x64 --with //src/tests/end_to_end/my_e2e_test_example:test
    ```

    `//src/tests/end_to_end/my_e2e_test_example` is the path to your new test
    directory. The `test` target group, as defined in the `BUILD.gn` file,
    includes `my_new_e2e_test`.

1.  Build your Fuchsia image:

    ```posix-terminal
    fx build
    ```

    When the `fx build` command completes, your build artifacts now include the
    `my_new_e2e_test` end-to-end test, which you can run from your host machine.

## 4. Start the emulator {#start-the-emulator}

Start the emulator to run your Fuchsia image:

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
    Logging to "{{ '<var>' }}$USER{{ '</var>' }}/.local/share/Fuchsia/ffx/emu/instances/fuchsia-emulator/emulator.log"
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

## 5. Run the test {#run-the-test}

In a new terminal, run the `my_new_e2e_test` end-to-end test:

```posix-terminal
fx test --e2e my_new_e2e_test
```

This test prints the following output:

```none {:.devsite-disable-click-to-copy}
...

00:00 +0: my_new_e2e_test tests hello world

Printed "Hello world!" in the device's log.

00:02 +1: All tests passed!

...
```

To scan the device logs for the `Hello world!` string,
run the following command:

```posix-terminal
ffx log --filter "Hello world!" dump
```

This command only prints the lines that contain `Hello world!` from
the device logs, for example:

```none {:.devsite-disable-click-to-copy}
[sl4f][][I] request id: String(""), name: "logging_facade.LogInfo", args: Object({"message": String("Hello world!")})
[sl4f][][I] Received synchronous request: Sender, MethodId { facade: "logging_facade", method: "LogInfo" }, Object({"message": String("Hello world!")})
[sl4f][][I] "\"Hello world!\""
```

## 6. Edit the test {#edit-the-test}

Edit the `my_new_e2e_test.dart` file to implement your test case.

Use the following resources for writing new tests:

*   The [developer guide](https://pub.dev/packages/test){:.external} for writing
    Dart tests.
*   The source code of existing end-to-end tests, for example:
    *   <code>[screen_is_not_black_test.dart](/src/tests/end_to_end/screen_is_not_black/test/screen_is_not_black_test.dart)</code>
*   The source code of the <code>[sl4f](/src/tests/end_to_end/sl4f/test/)</code>
    end-to-end test, which tests various
    [facades](/docs/development/drivers/concepts/driver_development/sl4f.md#facades-in-sl4f) in
    [SL4F](/docs/development/drivers/concepts/driver_development/sl4f.md). See these tests to understand how
    you may want to invoke some facades for testing certain features of a
    Fuchsia product, for example:
    *   [Audio facade test](/src/tests/end_to_end/sl4f/test/audio_test.dart) -
        Insert and capture audio on the device.
    *   [DeviceLog facade test](/src/tests/end_to_end/sl4f/test/device_log_test.dart) -
        Read and write log messages on the device.
    *   [Performance facade test](/src/tests/end_to_end/sl4f/test/performance_test.dart) -
        Enable collecting performance traces from the device, for instance, CPU
        usage, Flutter frame rate, and kernel counters.
    *   [SetUI facade test](/src/tests/end_to_end/sl4f/test/setui_test.dart) -
        Configure the device's settings, for instance, a network interface
        setting.
    *   [File facade test](/src/tests/end_to_end/sl4f/test/storage_test.dart) -
        Read and write files on the device's storage.

## 7. Update and run the test {#update-and-run-the-test}

After editing the test's source code, use the `fx test --e2e` command to run
the updated version of the test, for example:

```posix-terminal
fx test --e2e my_new_e2e_test
```

When this command detects any changes in the test's source code, the command
automatically rebuilds the test prior to running the test.
