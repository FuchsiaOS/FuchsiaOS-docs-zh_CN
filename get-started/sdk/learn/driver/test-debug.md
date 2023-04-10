# Test and debug the driver

Fuchsia supports step-through debugging of components using the Fuchsia debugger
(`zxdb`). The debugger attaches to the host process where a component is running,
and allows the developer to set breakpoints and step through code execution.
The Test Runner Framework enables developers to write tests that exercise driver
components.

In this section, you'll use the Fuchsia debugger (`zxdb`) to inspect a running
driver and build a test component to exercise the driver's functionality.

Note: For complete details on `zxdb` and the Test Runner Framework, see
[The Fuchsia Debugger][fuchsia-debugger] and
[Testing with Components][component-testing].

## Connect the debugger

To connect the Fuchsia debugger to the driver component, you'll need to
determine the PID of the host process. Use the `ffx driver list-hosts` command
to discover the PID of the host process where the driver is loaded:

```posix-terminal
ffx driver list-hosts
```

The command outputs a list similar to the following. Locate the driver host
where the `qemu_edu` driver is listed:

```none {:.devsite-disable-click-to-copy}
Driver Host: 5053
    fuchsia-boot:///#meta/block.core.cm
    fuchsia-boot:///#meta/bus-pci.cm
    fuchsia-boot:///#meta/cpu-trace.cm
    fuchsia-boot:///#meta/fvm.cm
    fuchsia-boot:///#meta/hid.cm
    fuchsia-boot:///#meta/netdevice-migration.cm
    fuchsia-boot:///#meta/network-device.cm
    fuchsia-boot:///#meta/platform-bus-x86.cm
    fuchsia-boot:///#meta/platform-bus.cm
    fuchsia-boot:///#meta/ramdisk.cm
    fuchsia-boot:///#meta/sysmem.cm
    fuchsia-boot:///#meta/virtio_block.cm
    fuchsia-boot:///#meta/virtio_ethernet.cm
    fuchsia-pkg://fuchsia.com/virtual_audio#meta/virtual_audio_driver.cm

Driver Host: 7774
    fuchsia-boot:///#meta/intel-rtc.cm

Driver Host: 7855
    fuchsia-boot:///#meta/pc-ps2.cm

{{ '<strong>' }}Driver Host: 44887 {{ '</strong>' }}
{{ '<strong>' }}    fuchsia-pkg://bazel.pkg.component/qemu_edu#meta/qemu_edu.cm {{ '</strong>' }}
```

Make a note of the PID for the `qemu_edu` driver host.
In the above example, the PID is 44887.

Start the Fuchsia debugger with `ffx debug connect`:

```posix-terminal
ffx debug connect
```

Once the debugger connects to the target device, attach to the `qemu_edu` driver
host from the `zxdb` prompt:

<pre class="devsite-click-to-copy">
<span class="no-select">[zxdb] </span>attach <var label="driver host">HOST_PID</var>
</pre>

Replace `HOST_PID` with the PID of the driver host identified in the previous
step. For example:

```none {:.devsite-disable-click-to-copy}
[zxdb] attach 44887
```

Set a breakpoint in the driver's `ComputeFactorial` function:

<pre class="devsite-click-to-copy">
<span class="no-select">[zxdb] </span>break QemuEduServer::ComputeFactorial
</pre>

The command prints output similar to the following to indicate where the
breakpoint is set:

```none {:.devsite-disable-click-to-copy}
[zxdb] break QemuEduServer::ComputeFactorial
Created Breakpoint 1 @ QemuEduServer::ComputeFactorial
   47 void QemuEduServer::ComputeFactorial(ComputeFactorialRequestView request,
 ◉ 48                                      ComputeFactorialCompleter::Sync& completer) {
   49   auto edu_device = device_.lock();
```

## Step through the driver function

In a separate terminal, run the `eductl` tool again:

```posix-terminal
bazel run //fuchsia-codelab/qemu_edu/tools:pkg.eductl_tool -- fact 12
```

In the `zxdb` terminal, verify that the debugger has hit the breakpoint in the driver's
`ComputeFactorial` function. For example:

```none {:.devsite-disable-click-to-copy}
🛑 thread 2 on bp 1 qemu_edu::QemuEduServer::ComputeFactorial(qemu_edu::QemuEduServer*, fidl::WireServer<fuchsia_examples_qemuedu::Device>::ComputeFactorialRequestView, fidl::Completer<fidl::internal::WireCompleterBase<fuchsia_examples_qemuedu::Device::ComputeFactorial> >::Sync&) • qemu_edu.cc:144
   46 // Driver Service: Compute factorial on the edu device
   47 void QemuEduServer::ComputeFactorial(ComputeFactorialRequestView request,
 ▶ 48                                      ComputeFactorialCompleter::Sync& completer) {
   49   auto edu_device = device_.lock();
   50   if (!edu_device) {
```

Use the `list` command at the `zxdb` prompt to show where execution is currently
paused:

<pre class="devsite-click-to-copy">
<span class="no-select">[zxdb] </span>list
</pre>

The command prints output similar to the following:

```none {:.devsite-disable-click-to-copy}
   46 // Driver Service: Compute factorial on the edu device
   47 void QemuEduServer::ComputeFactorial(ComputeFactorialRequestView request,
 ▶ 48                                      ComputeFactorialCompleter::Sync& completer) {
   49   auto edu_device = device_.lock();
   50   if (!edu_device) {
   51     FDF_LOG(ERROR, "Unable to access device resources.");
   52     completer.ReplyError(ZX_ERR_BAD_STATE);
   53     return;
   54   }
   55
   56   uint32_t input = request->input;
   57
   58   edu_device->ComputeFactorial(input);
```

Step into the `ComputeFactorial` function using the `next` command:

<pre class="devsite-click-to-copy">
<span class="no-select">[zxdb] </span>next
</pre>

Print the contents of the request passed into the function:

<pre class="devsite-click-to-copy">
<span class="no-select">[zxdb] </span>print request
</pre>

The command prints output containing the factorial input value:

```none {:.devsite-disable-click-to-copy}
(*)0x747c1f2e98 ➔ {input = 12}
```

Exit the debugger session and disconnect:

<pre class="devsite-click-to-copy">
<span class="no-select">[zxdb] </span>exit
</pre>

## Create a new system test component

In this section, you'll create a new test component that exercises the exposed
functions of the `qemu_edu` driver.

Create a new project directory in your Bazel workspace for a new test component:

```posix-terminal
mkdir -p fuchsia-codelab/qemu_edu/tests
```

After you complete this section, the project should have the following directory
structure:

```none {:.devsite-disable-click-to-copy}
//fuchsia-codelab/qemu_edu/tests
                  |- BUILD.bazel
                  |- meta
                  |   |- qemu_edu_system_test.cml
                  |- qemu_edu_system_test.cc
```

Create the `qemu_edu/tests/BUILD.bazel` file and add the following statement to
include the necessary build rules from the Fuchsia SDK:

`qemu_edu/tests/BUILD.bazel`:

```bazel
{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/tests/BUILD.bazel" region_tag="imports" adjust_indentation="auto" %}
```

Create a new `qemu_edu/tests/meta/qemu_edu_system_test.cml` component manifest
file to the project with the following contents:

`qemu_edu/tests/meta/qemu_edu_system_test.cml`:

```json5
{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/tests/meta/qemu_edu_system_test.cml" region_tag="example_snippet" adjust_indentation="auto" %}

```

Similar to `eductl`, the test component discovers and accesses the driver using
the `dev` directory capability. This component also includes the
`elf_test_runner.shard.cml`, which enables it to run using the Test Runner
Framework.

Create a new `qemu_edu/tests/qemu_edu_system_test.cc` file with the following
contents to implement the tests:

`qemu_edu/tests/qemu_edu_system_test.cc`:

```cpp
{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/tests/qemu_edu_system_test.cc" region_tag="imports" adjust_indentation="auto" %}

{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/tests/qemu_edu_system_test.cc" region_tag="main_body" adjust_indentation="auto" %}

```

Each test case opens the device driver and exercises one of its exposed
functions.

Add the following new rules to the project's build configuration to build the
test component into a Fuchsia test package:

`qemu_edu/tests/BUILD.bazel`:

{% set build_bazel_snippet %}
{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/tests/BUILD.bazel" region_tag="binary" adjust_indentation="auto" %}

{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/tests/BUILD.bazel" region_tag="component" adjust_indentation="auto" %}

{% includecode gerrit_repo="fuchsia/sdk-samples/drivers" gerrit_path="src/qemu_edu/tests/BUILD.bazel" region_tag="package" adjust_indentation="auto" %}
{% endset %}

```bazel
{{ build_bazel_snippet|replace("//src/qemu_edu","//fuchsia-codelab/qemu_edu")|trim() }}
```

## Run the system test

Use the `bazel run` command to build and execute the test component target:

```posix-terminal
bazel run //fuchsia-codelab/qemu_edu/tests:pkg.component
```

The `bazel run` command performs the following steps:

1.  Build the component and package.
1.  Publish the package to a local package repository.
1.  Register the package repository with the target device.
1.  Use `ffx test run` to execute the component's test suite.

Verify that all the tests pass successfully:

```none {:.devsite-disable-click-to-copy}
Running test 'fuchsia-pkg://bazel.test.pkg.system.test.component/qemu_edu_system_test#meta/qemu_edu_system_test.cm'
[RUNNING]	main
[stdout - main]
Running main() from gmock_main.cc
[==========] Running 2 tests from 1 test suite.
[----------] Global test environment set-up.
[----------] 2 tests from QemuEduSystemTest
[ RUN      ] QemuEduSystemTest.LivenessCheck
[       OK ] QemuEduSystemTest.LivenessCheck (4 ms)
[ RUN      ] QemuEduSystemTest.ComputeFactorial
[       OK ] QemuEduSystemTest.ComputeFactorial (4 ms)
[----------] 2 tests from QemuEduSystemTest (9 ms total)

[----------] Global test environment tear-down
[==========] 2 tests from 1 test suite ran. (9 ms total)
[  PASSED  ] 2 tests.
[PASSED]	main
```

## What's Next?

Congratulations! You've successfully debugged and added tests to your Fuchsia
driver.

Now that you have experienced the basics of developing drivers on Fuchsia, take
your knowledge to the next level and dive deeper with the:

<a class="button button-primary"
    href="/concepts/drivers">Driver concepts</a>

<!-- Reference links -->

[component-testing]: /development/testing/components/README.md
[fuchsia-debugger]: /development/debugger/README.md
