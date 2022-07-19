# Driver sample walkthrough: qemu_edu

The [`qemu_edu`][qemu-edu] driver (in the
[SDK driver samples][sdk-driver-samples] repository) is a good sample to
study if you’re interested in creating a new driver or understanding how drivers
work in a Fuchsia system.

One of the services that the `qemu_edu` driver provides is that it enables
Fuchsia components to calculate a factorial using the computing power of a
device (in fact, a virtual device) in the system.

This development walkthrough guide contains the following sections:

*  [How the driver works](#how_this_driver_works)
*  [Build rules: BUILD.bazel](#build_rules_buildbazel)
*  [Driver component: qemu_edu](#driver_component_qemu_edu)
*  [Tools component: eductl](#tools_component_eductl)

## How this driver works

For the `qemu_edu` driver to work, we first need to launch an instance of the
[Fuchsia emulator][femu] (for instance, using a Fuchsia’s Workstation image).
[QEMU][qemu]{:.external} (which underpins the Fuchsia emulator)
then creates a virtual device named [`edu`][edu-device], which is an educational
device for writing drivers. It is this `edu` device that provides a service that
computes a factorial given an integer.

Soon after the emulator instance starts running, Fuchsia’s
[driver framework][driver-framework] discovers the `edu` device (which is
represented as a [node][nodes]) and looks for a driver that is a good match for
the `edu` device. However, at first the driver framework won’t be able to find
a driver with bind rules that can satisfy the [binding properties][driver-binding]
of the `edu` device. At this point, the `edu` device is left unmatched,
therefore its node unbound.

Without a driver, the unmatched `edu` device cannot provide services to other
components in the system. However, here we have the `qemu_edu` driver, which we
know is suitable for serving the `edu` device. To supply this driver to the
`edu` device in the system, we need to do the following:

1. Build a Fuchsia package that contains the `qemu_edu` driver component.
2. Publish the package to a local Fuchsia package repository.
3. Register the `qemu_edu` driver to the Fuchsia emulator instance.

Note: The [`bazel run`][get-started-drivers] command is configured to run the
sequence of all three actions above.

When a new driver is registered, Fuchsia’s
[driver manager][driver-manager] informs the [driver index][driver-index]
about the arrival of the new driver. The driver index then tries to match this
new `qemu_edu` driver to unbound nodes in the node topology.
We’re lucky this time because the `qemu_edu` driver’s bind rules (see
[`qemu_edu.bind`](#qemu_edubind)) are precisely designed to match the binding
properties of the `edu` device – to be exact, the binding properties of the node
that represents the `edu` device in the topology.

Once they are matched and bound, the driver manager examines the `qemu_edu`
driver’s component manifest (see [`meta/qemu_edu.cml`](#metaqemu_educml))
to learn more about the `qemu_edu` driver. For instance, the driver’s
component manifest describes which binary to run, which runtime to use, and
whether to place the driver in a new [driver host][driver-host]
or in the same driver host as its parent driver. In this case, by default the
driver manager places the `qemu_edu` driver in a new driver host.

The driver host (which runs as a Fuchsia component) calls the `Start()` function
in the driver’s code (see [`qemu_edu.cc`](#qemu_educc)) to initialize the
driver. Once initialized, the `qemu_edu` driver can start providing the `edu`
device’s services to other components in the system.

## Build rules: BUILD.bazel

This [`BUILD.bazel`][build-bazel] file contains the build rules of the
[`qemu_edu` driver component](#driver_component_qemu_edu)
and the [`eductl` "tools" component](#tools_component_eductl). Using this
`BUILD.bazel` file, these two components are built into the same package labeled
`qemu_edu` (see the `fuchsia_package` rule in `BUILD.bazel`).

While the build rules of a driver component mostly look similar to the build
rules of other Fuchsia components, there exist a few rules and attributes
specific to the driver development.

Below is the `load` field shown at the top of the `qemu_edu` driver’s
`BUILD.bazel` file:

```none {:.devsite-disable-click-to-copy}
load(
    "@rules_fuchsia//fuchsia:defs.bzl",
    "fuchsia_cc_binary",
    "fuchsia_component",
    "fuchsia_component_manifest",
    {{ '<strong>' }}"fuchsia_driver_bytecode_bind_rules",{{ '</strong>' }}
    "fuchsia_fidl_library",
    {{ '<strong>' }}"fuchsia_fidl_llcpp_library",{{ '</strong>' }}
    "fuchsia_package",
)
```

Of these entries, the following build rules are specific to the driver
development:

*   [`fuchsia_driver_bytecode_bind_rules`](#fuchsia_driver_bytecode_bind_rules)
*   [`fuchsia_fidl_llcpp_library`](#fuchsia_fidl_llcpp_library)

And the following build rules contain additional entries that are specific to
the driver development:

*   [`cc_binary`](#cc_binary)
*   [`fuchsia_component`](#fuchsia_component)

### fuchsia_driver_bytecode_bind_rules

The `fuchsia_driver_bytecode_bind_rules` rule describes the specifics of the
[bind rules][driver-binding] of a driver. Fuchsia’s driver framework uses a
driver’s bind rules to match the driver to a device in a Fuchsia system.

Below are the `fuchsia_driver_bytecode_bind_rules` attributes of
the `qemu_edu` driver:

```none {:.devsite-disable-click-to-copy}
fuchsia_driver_bytecode_bind_rules(
    name = "bind_bytecode",
    output = "qemu_edu.bindbc",
    rules = "qemu_edu.bind",
    deps = [
        "@fuchsia_sdk//bind/fuchsia.pci",
    ],
)
```

The `rules` attribute above specifies that the [`qemu_edu.bind`](#qemu_edubind)
file contains the bind rules of this driver. The `deps` attribute
specifies the bind libraries to be used with the bind rules specified
in `rules`. To see all available bind libraries supported by Fuchsia,
look in the `//bind` directory in the Fuchsia SDK. (For more information,
see [Bind libraries][bind-libraries].)

### fuchsia_fidl_llcpp_library

The `fuchsia_fidl_llcpp_library` rule describes the specifics of the `llcpp`
(Low-Level C++) FIDL library.

This Low-Level C++ FIDL library provides a collection of low-level C++ FIDL
calls used by drivers in a Fuchsia system. The library is optimized to meet the
needs of low-level systems programming and multi-thread environments for
increased security. (For more information, see
[Comparing C, Low-Level C++, and High-Level C++ Language Bindings][c-family-comparison].)

Notice there are two FIDL library build rules in this `BUILD.bazel` file:

*   `fuchsia_fidl_library()`
*   `fuchsia_fidl_llcpp_library()`

Below are the `qemu_edu` driver’s attributes of these two FIDL library
build rules:

```none {:.devsite-disable-click-to-copy}
fuchsia_fidl_library(
    name = "fuchsia.hardware.qemuedu",
    srcs = [
        "qemu_edu.fidl",
    ],
    library = "fuchsia.hardware.qemuedu",
    visibility = ["//visibility:public"],
)

fuchsia_fidl_llcpp_library(
    name = "fuchsia.hardware.qemuedu_cc",
    library = ":fuchsia.hardware.qemuedu",
    visibility = ["//visibility:public"],
    deps = [
        "@fuchsia_sdk//fidl/zx:zx_llcpp_cc",
        "@fuchsia_sdk//pkg/fidl-llcpp",
    ],
)
```

The generic `fuchsia_fidl_library` rule defines a Fuchsia component’s FIDL
capabilities, which are specified in [`qemu_edu.fidl`](#qemu_edufidl) for the
`qemu_edu` driver. The `fuchsia_fidl_llcpp_library` rule describes additional
dependencies for generating the FIDL code that is based on the Low-Level C++
FIDL library.

### cc_binary

The `cc_binary` rule specifies the source and header files (for instance,
`qemu_edu.cc`, `qemu_edu.h`, and `registers.h`) for building a C++ binary for
the `qemu_edu` driver.

Notice some driver-specific dependencies in this `cc_binary` rule, mainly the
`pci_llcpp` FIDL library for PCI FIDL communication and `hwreg` and `mmio`
packages for working with a device’s hardware registers and MMIO
(Memory-Mapped I/O):

```none {:.devsite-disable-click-to-copy}
cc_binary(
    name = "qemu_edu",
    srcs = [
        "qemu_edu.cc",
        "qemu_edu.h",
        "registers.h",
    ],
    {{ '<strong>' }}linkshared = True,{{ '</strong>' }}
    deps = [
        ":fuchsia.hardware.qemuedu_cc",
        "@fuchsia_sdk//fidl/fuchsia.device.fs:fuchsia.device.fs_llcpp_cc",
        "@fuchsia_sdk//fidl/fuchsia.driver.compat:fuchsia.driver.compat_llcpp_cc",
        {{ '<strong>' }}"@fuchsia_sdk//fidl/fuchsia.hardware.pci:fuchsia.hardware.pci_llcpp_cc",{{ '</strong>' }}
        "@fuchsia_sdk//fidl/zx:zx_cc",
        "@fuchsia_sdk//pkg/driver2-llcpp",
        "@fuchsia_sdk//pkg/driver_runtime_cpp",
        "@fuchsia_sdk//pkg/fidl-llcpp",
        {{ '<strong>' }}"@fuchsia_sdk//pkg/hwreg",{{ '</strong>' }}
        {{ '<strong>' }}"@fuchsia_sdk//pkg/mmio",{{ '</strong>' }}
        "@fuchsia_sdk//pkg/sys_component_llcpp",
        "@fuchsia_sdk//pkg/zx-status",
    ],
)
```

What’s also important for Fuchsia drivers is that the line `linkshared = True,`
must be included in this build rule. This line enables the binary of a Fuchsia
driver to be shared with other processes.

### fuchsia_component

For a driver component, the `fuchsia_component` rule mainly describes where to
place binaries and artifacts within the component’s storage space.

Below are the `fuchsia_component` attributes of the `qemu_edu` driver:

```none {:.devsite-disable-click-to-copy}
fuchsia_component(
    name = "component",
    content = {
        ":qemu_edu": "lib/",
        ":bind_bytecode": "meta/bind/",
    },
    manifest = ":manifest",
)
```

According to these attributes, the content of the driver’s binary
(generated from the `qemu_edu` target) is to be placed in the `lib` directory,
and the content of the bytecode that contains the driver’s bind rules (generated
from the `bind_bytecode` target) is to be placed in the `meta/bind` directory of
the component.

The file placement convention above is recommended for driver components.
The Fuchsia platform does not compress files under a component’s `meta`
directory. So it helps to place files that need to be loaded fast, such as
bytecode containing bind rules, in the `meta` directory.

## Driver component: qemu_edu

The `qemu_edu` component is the `qemu_edu` driver itself.

The `qemu_edu` driver component comprises the following files:

*   Build rules: [`BUILD.bazel`](#build_rules_buildbazel)
*   Component manifests: [`meta/qemu_edu.cml`](#metaqemu_educml)
*   Bind rules: [`qemu_edu.bind`](#qemu_edubind)
*   FIDL interface: [`qemu_edu.fidl`](#qemu_edufidl)
*   Driver component header: [`qemu_edu.h`](#qemu_eduh)
*   Registers header: [`registers.h`](#registersh)
*   Component implementation: [`qemu_edu.cc`](#qemu_educc)

### meta/qemu_edu.cml

The [`meta/qemu_edu.cml`][qemu-edu-cml] file contains the
[component manifest][component-manifests] of the `qemu_edu` component.

Below are the attributes of the component manifest for the `qemu_edu` component:

```none {:.devsite-disable-click-to-copy}
{
    program: {
        runner: 'driver',
        binary: 'lib/libqemu_edu.so',
        bind: 'meta/bind/qemu_edu.bindbc'
    },
    use: [
        {
            protocol: [
              'fuchsia.logger.LogSink',
              'fuchsia.device.fs.Exporter'
            ],
        },
        {
            directory: 'fuchsia.driver.compat.Service-default',
            rights: ['rw*'],
            path: '/fuchsia.driver.compat.Service/default',
        }
    ],
}
```

The `program` field describes which program to start when this component starts
running on the Fuchsia platform. For driver components, the `runner` attribute
needs to be `driver` , which means to use the [driver runtime][driver-runtime].
The `binary` attribute indicates that the `lib/libqemu_edu.so` file contains the
binary of the driver. According to the [`fuchsia_component`](#fuchsia_component)
rule in the `BUILD.bazel` file, the driver’s binary (`libqemu_edu.so`)
is located in the `lib` directory, and the bind rules (`qemu_edu.bindbc`)
is located in the `meta/bind` directory of the component’s storage space.

The `use` field specifies the capabilities that this component uses when it
starts running on the Fuchsia platform.

### qemu_edu.bind

The [`qemu_edu.bind`][qemu-edu-bind] file contains the bind rules for the
`qemu_edu` driver.

A driver’s bind rules are a series of Boolean expressions that the driver must
provide to ensure that it’s matched to the right device running in a system.
These rules are compared against the binding properties of devices
in the system. When a device’s binding properties can satisfy the driver’s bind
rules (thus returning `true` for all conditions in the bind rules), then the
driver is said to be matched to the device. Once they are matched, the driver
can provide the device’s services to other components in the system.

Below are the bind rules of the `qemu_edu` driver:

```none {:.devsite-disable-click-to-copy}
using fuchsia.pci;

fuchsia.BIND_FIDL_PROTOCOL == fuchsia.pci.BIND_FIDL_PROTOCOL.DEVICE;
fuchsia.BIND_PCI_VID == 0x1234;
fuchsia.BIND_PCI_DID == 0x11e8;
```

The line `using fuchsia.pci;` indicates the bind library necessary to generate
the bind rules into bytecode. This bind library is also specified in the
[`fuchsia_driver_bytecode_bind_rules`](#fuchsia_driver_bytecode_bind_rules) rule
in `BUILD.bazel`.

The bind rules for the `qemu_edu` driver say the following:

*   The device is a PCI device.
*   The vendor ID (`BIND_PCI_VID`) needs to be `0x1234`, which is a globally
    unique ID that identifies the vendor.
*   The device ID (`BIND_PCI_DID`) needs to be `0x11e8`, which is a globally
    unique ID that the vendor has assigned to the device.

Note: For more information on the bind rules, see the
[Driver Binding][driver-binding-dfv1],
which was written for the previous version of the driver framework (DFv1),
but most of its content is still relevant to the new driver framework (DFv2).

### qemu_edu.fidl

The [`qemu_edu.fdl`][qemu-edu-fidl] file describes the FIDL interface of the
`qemu_edu` driver.

Below are the two FIDL services that the `qemu_edu` driver provides:

```none {:.devsite-disable-click-to-copy}
library fuchsia.hardware.qemuedu;

protocol Device {
    // Computes the factorial of `input` using the edu device and returns the
    // result.
    ComputeFactorial(struct {
        input uint32;
    }) -> (struct {
        output uint32;
    });
    // Performs a liveness check and return true if the device passes.
    LivenessCheck() -> (struct {
        result bool;
    });
};
```

The line `library fuchsia.hardware.qemuedu` indicates the FIDL library used by
these FIDL services (see [FIDL reference docs][fidl-reference])
This FIDL library needs to match the `library` attribute specified in the
[`fuchsia_fidl_library`](#fuchsia_fidl_llcpp_library) rule in `BUILD.bazel`.

The implementation of the `ComputeFactorial()` and `LivenessCheck()` FIDL
services is in the [`qemu_edu.cc`](#qemu_educc) file.

### qemu_edu.h

The [`qemu_edu.h`][qemu-edu-h] file is the C++ header for the implementation of
the `qemu_edu` component ([`qemu_edu.cc`](#qemu_educc)).

Below are the public C++ function declarations for the `qemu_edu` driver:

```none {:.devsite-disable-click-to-copy}
class QemuEduDriver : public fidl::WireServer<fuchsia_hardware_qemuedu::Device> {
 public:
  QemuEduDriver(async_dispatcher_t* dispatcher,
                fidl::WireSharedClient<fuchsia_driver_framework::Node> node, driver::Namespace ns,
                driver::Logger logger)
      : outgoing_(component::OutgoingDirectory::Create(dispatcher)),
        node_(std::move(node)),
        ns_(std::move(ns)),
        logger_(std::move(logger)) {}
  virtual ~QemuEduDriver() = default;
  {{ '<strong>' }}static constexpr const char* Name() { return "qemu-edu"; }{{ '</strong>' }}
  {{ '<strong>' }}static zx::status<std::unique_ptr<QemuEduDriver>> Start(
      fuchsia_driver_framework::wire::DriverStartArgs& start_args,
      fdf::UnownedDispatcher dispatcher,
      fidl::WireSharedClient<fuchsia_driver_framework::Node> node, driver::Namespace ns,
      driver::Logger logger);{{ '</strong>' }}
  // fuchsia.hardware.qemuedu/Device implementation.
  void ComputeFactorial(ComputeFactorialRequestView request,
                        ComputeFactorialCompleter::Sync& completer);
  void LivenessCheck(LivenessCheckRequestView request, LivenessCheckCompleter::Sync& completer);
  static uint32_t ComputeFactorial(uint32_t input);
...
```

All driver components are required to supply the following two `public` functions:

*   `Name()` – The name returned from this function is often used in
    `qemu_edu.cc` to identify the driver (for instance, to identify the driver’s
    placement in the topology).
*   `Start()` – This is the main function that gets executed as the start hook
    for the driver.

### registers.h

The [`registers.h`][registers-h] file is the header file that describes the
registers of the `edu` device.

Below are the offset values of the registers in the `edu` device
(for which the `qemu_edu` driver provides services):

```none {:.devsite-disable-click-to-copy}
constexpr uint32_t kIdentificationOffset = 0x00;
constexpr uint32_t kLivenessCheckOffset = 0x04;
constexpr uint32_t kFactorialCompoutationOffset = 0x08;
constexpr uint32_t kStatusRegisterOffset = 0x20;
constexpr uint32_t kInterruptStatusRegisterOffset = 0x24;
constexpr uint32_t kInterruptRaiseRegisterOffset = 0x60;
constexpr uint32_t kInterruptAcknowledgeRegisterOffset = 0x64;
constexpr uint32_t kDmaSourceAddressOffset = 0x80;
constexpr uint32_t kDmaDestinationAddressOffset = 0x80;
constexpr uint32_t kDmaTransferCountOffset = 0x90;
constexpr uint32_t kDmaCommandRegisterOffset = 0x98;
```

The `qemu_edu` driver uses these offset values to understand how to read and
write values from the registers in the `edu` device. In general, driver
developers would use their target device’s specification sheet to provide this
offset information.

### qemu_edu.cc

The [`qemu_edu.cc`][qemu-edu-cc] file contains the source code of the `qemu_edu`
component, which is the `qemu_edu` driver itself.

#### Implementing the start hook

A driver is required to provide the implementation of a public `Start()`
function, which is the main function that gets executed as the start hook for
the driver. The code below is the `Start()` function of the `qemu_edu` driver:

```none {:.devsite-disable-click-to-copy}
zx::status<std::unique_ptr<QemuEduDriver>> QemuEduDriver::Start(
    fdf::wire::DriverStartArgs& start_args, fdf::UnownedDispatcher dispatcher,
    fidl::WireSharedClient<fdf::Node> node, driver::Namespace ns, driver::Logger logger) {
  auto driver = std::make_unique<QemuEduDriver>(dispatcher->async_dispatcher(), std::move(node),
                                                std::move(ns), std::move(logger));
  auto result = driver->Run(dispatcher->async_dispatcher(), std::move(start_args.outgoing_dir()));
  if (result.is_error()) {
    return result.take_error();
  }
  return zx::ok(std::move(driver));
}
```

This `Start()` function initializes an instance of the `QemuEduDriver` class
and assigns it to the `driver` variable. The function then calls the private
`Run()` function, which performs most of the initialization tasks.

In the `Run()` function, the first task is to connect to Fuchsia’s device
filesystem (`devfs`), which enables the driver to expose the `edu` device and
its services to other components in the system:

```none {:.devsite-disable-click-to-copy}
auto exporter = ns_.Connect<fuchsia_device_fs::Exporter>();
if (exporter.is_error()) {
  return exporter.take_error();
}
exporter_ = fidl::WireClient(std::move(*exporter), dispatcher);
```

To discover which driver services are available in the system, a non-driver
component would look up the device filesystem (usually mounted to /`dev` in a
component’s storage space) and scan for the directories and files under this
filesystem. Once the non-driver component locates a service, which shows up as
a file in `devfs`, the component can contact the driver by opening this file in
and writing to it (similar to a POSIX system).

After this initial contact, the driver sets up a FIDL connection
(more precisely, by exchanging FIDL protocols in /`svc`) to the non-driver
component. From this point, all communication between the component and the
driver takes place using FIDL calls. (For more information, see
[Driver communication][driver-communication].) The rest of the code in the
`Run()` function implements the setting up and mapping of this `devfs`-to-FIDL
connection.

#### Computing a factorial

Once initialized, the `qemu_edu` driver provides the following two services
to its clients (which include both non-driver components and other drivers):

*   `ComputeFactorial()`: Write an integer to the `edu` device and read the
    value, which is the factorial of the integer, returned from the device.
*   `LivenessCheck()`: Write a “challenge” value to the `edu` device’s register
    and read the value returned from the device to confirm that the device is
    working as expected.

When passing values to the `edu` device, the `qemu_edu` driver uses predefined
offsets in the [`registers.h`](#registersh) file to
determine exactly which memory locations to write the input values
in the device’s registers.

In [Line 133][qemu-edu-cc-line-133] of `qemu_edu.cc`, the `Run()` function
includes the following function call:

```none {:.devsite-disable-click-to-copy}
auto pci_status = MapInterruptAndMmio(std::move(pci_endpoints->client));
```

The ​​`MapInterruptAndMmio()` function (whose implementation starts in
[Line 45][qemu-edu-cc-line-45]) sets up access to the register and extracts
Fuchsia’s [VMO][vmo] (Virtual Memory Object), which enables MMIO operations
(such as reading and writing to the register). And in the same function, the
`SetInterruptMode` function in [Line 82][qemu-edu-cc-line-82] initializes
interrupt calls for the driver.

After reading an integer value from its register, the `edu` device uses its own
factorial function (see [`edu.c`][edu-c-line-314]) and its own (virtual)
computing power to compute a factorial using the input integer retrieved from
the register.

The `ComputeFactorial()` function (starting at
[Line 177][qemu-edu-cc-line-177]) handles the factorial computation by writing
a value into the `edu` device’s register (using an offset) and reading a value
from the register:

```none {:.devsite-disable-click-to-copy}
void QemuEduDriver::ComputeFactorial(ComputeFactorialRequestView request,
                                     ComputeFactorialCompleter::Sync& completer) {
  // Write a value into the factorial register.
  uint32_t input = request->input;
  mmio_->Write32(input, regs::kFactorialCompoutationOffset);
  // Busy wait on the factorial status bit.
  while (true) {
    const auto status = regs::Status::Get().ReadFrom(&*mmio_);
    if (!status.busy())
      break;
  }
  // Return the result.
  uint32_t factorial = mmio_->Read32(regs::kFactorialCompoutationOffset);
  FDF_SLOG(INFO, "Replying with", KV("factorial", factorial));
  completer.Reply(factorial);
}
```

When the [`eductl` “tools” component](#tools_component_eductl) calls
this `ComputeFactorial()` function on the driver (using a FIDL call)
and passes an integer as input, the function writes this input value into the
`edu` device’s register (using the `mmio` object). The function then waits until
the register is ready to read. When the device finishes computing the factorial
and writes its result back into the register, the `while` loop breaks out and
the driver reads the value from the register to obtain the value of the
factorial computation performed by the `edu` device.

To emulate a virtual device that processes requests from the `qemu_edu` driver,
the [`edu`][edu-device] device (which is implemented as part of QEMU) provides
the following functions in [`edu.c`][edu-c]:

*   The `edu_mmio_write` function (in [Line 251][edu-c-line-251])
    writes the input value to the `edu` device’s register and signals the
    factorial worker thread :

    ```none {:.devsite-disable-click-to-copy}
    case 0x08:
        if (atomic_read(&edu->status) & EDU_STATUS_COMPUTING) {
            break;
        }
        /* EDU_STATUS_COMPUTING cannot go 0->1 concurrently, because it is only
         * set in this function and it is under the iothread mutex.
         */
        qemu_mutex_lock(&edu->thr_mutex);
        edu->fact = val;
        atomic_or(&edu->status, EDU_STATUS_COMPUTING);
        qemu_cond_signal(&edu->thr_cond);
        qemu_mutex_unlock(&edu->thr_mutex);
        break;
    ```

*   The factorial thread (see the `edu_fact_thread()` function in
    [Line 314][edu-c-line-314]) wakes up and computes the factorial and stores
    the result in the device’s register.
*   The `edu_mmio_read()` function (in [Line 206][edu-c-line-206]) reads a value
    from the device’s register:

    ```none {:.devsite-disable-click-to-copy}
    case 0x08:
        qemu_mutex_lock(&edu->thr_mutex);
        val = edu->fact;
        qemu_mutex_unlock(&edu->thr_mutex);
        break;
    ```

However, keep in mind that the `edu` device performs operations that are far
more complex than necessary in order to make itself behave like a real hardware
device, such as producing results asynchronously.

## Tools component: eductl

The `eductl` component is a helper component (known as a “tools” component) that
serves as a tool for testing the features of the `qemu_edu` driver while it’s
running in the system.

The `eductl` “tools” component comprises the following files:

*   Build rules: [`BUILD.bazel`](#build_rules_buildbazel)
*   Component manifests: [`meta/eductl.cml`](#metaeductlcml)
*   Component implementation: [`eductl.cc`](#eductlcc)

### meta/eductl.cml

The [`meta/eductl.cml`][eductl-cml] file contains the
[component manifest][component-manifests] of the `eductl` component

Below are the attributes of the component manifest for the `eductl` component:

```none {:.devsite-disable-click-to-copy}
{
    program: {
        runner: 'elf',
        binary: 'bin/eductl',
        forward_stderr_to: "log",
        forward_stdout_to: "log",
        args: [
          'factorial',
          '12',
        ]
    },
    use: [
        {
            directory: "dev",
            rights: [ "rw*" ],
            path: "/dev",
        },
        { protocol: "fuchsia.logger.LogSink" },
    ],
}
```


The `program` field describes which program to start when this component starts
running on the Fuchsia platform. The `runner` attribute specifies that this
component uses the `elf` runtime, which is commonly used to run C++ and Rust
binaries on the Fuchsia platform. The `binary` attribute indicates that the
`bin/eductl` file contains the binary of the program. Lastly, the `args` field
shows that the default input arguments provided to the program is
`factorial` `12`.

The `use` field sets up access to Fuchsia’s device filesystem (`devfs`) for
the component. This setup is required for non-driver components to interact
with drivers in a Fuchsia system. (For more information on the role of the
device filesystem, see [Driver communication][driver-communication].)

### eductl.cc

The [`eductl.cc`][eductl-cc] file contains the source code of the “tools”
component, which enables driver developers to interact with the `qemu_edu`
driver while the driver is running in a Fuchsia system. It’s common for driver
developers to create these tools components for the purpose of testing and
debugging during development. However, in production, the ability to use these
driver-specific tools will be disabled.

Below are the hardcoded device paths that are mapped to the `qemu_edu` device
running in a Fuchsia system:

```none {:.devsite-disable-click-to-copy}
constexpr char kEduDevicePath[] =
    "/dev/sys/platform/platform-passthrough/PCI0/bus/00:06.0_/qemu-edu";
constexpr char kEduDevicePath2[] = "/dev/sys/platform/platform-passthrough/acpi/acpi-KBLT/qemu-edu";
```

Using paths in Fuchsia’s device filesystem (`devfs`), non-driver components,
such as the `eductl` component, can discover services provided by the drivers
running in the system. (For more information, see
[Service discovery (using devfs)][service-discovery].)

Below is the function that uses the device paths to establish a FIDL connection
to the `qemu_edu` driver:

```none {:.devsite-disable-click-to-copy}
fidl::WireSyncClient<fuchsia_hardware_qemuedu::Device> OpenDevice() {
  int device = open(kEduDevicePath, O_RDWR);
  if (device < 0) {
    device = open(kEduDevicePath2, O_RDWR);
  }
  if (device < 0) {
    fprintf(stderr, "Failed to open qemu edu device: %s\n", strerror(errno));
    return {};
  }
  fidl::ClientEnd<fuchsia_hardware_qemuedu::Device> client_end;
  zx_status_t st = fdio_get_service_handle(device, client_end.channel().reset_and_get_address());
  if (st != ZX_OK) {
    fprintf(stderr, "Failed to get service handle: %s\n", zx_status_get_string(st));
    return {};
  }
  return fidl::BindSyncClient(std::move(client_end));
}
```

In this `OpenDevice()` function, the `eductl` component uses the device paths
in `devfs` to contact the `qemu_edu` driver in the system. Once the initial
contact is made, a FIDL connection is established between the `eductl` component
and the `qemu_edu` driver. From this point, all communication between these
two takes place using this FIDL channel.

<!-- Reference links -->

[qemu-edu]: https://fuchsia.googlesource.com/sdk-samples/drivers/+/refs/heads/main/src/qemu_edu/
[sdk-driver-samples]: https://fuchsia.googlesource.com/sdk-samples/drivers/
[edu-device]: https://fuchsia.googlesource.com/third_party/qemu/+/refs/heads/main/docs/specs/edu.txt
[qemu]: https://www.qemu.org/
[driver-framework]: /concepts/drivers/driver_framework.md
[nodes]: /concepts/drivers/drivers_and_nodes.md
[driver-binding]: /concepts/drivers/driver_binding.md
[get-started-drivers]: /get-started/sdk/get-started-with-driver.md
[driver-manager]: /concepts/drivers/driver_framework.md#driver_manager
[driver-index]: /concepts/drivers/driver_framework.md#driver_index
[driver-host]: /concepts/drivers/driver_framework.md#driver_host
[driver-runtime]: /concepts/drivers/driver_framework.md#driver_runtime
[driver-communication]: /concepts/drivers/driver_communication.md
[build-bazel]: https://fuchsia.googlesource.com/sdk-samples/drivers/+/refs/heads/main/src/qemu_edu/BUILD.bazel
[bind-libraries]: /development/drivers/concepts/device_driver_model/driver-binding.md#bind-libraries
[c-family-comparison]: /development/languages/fidl/guides/c-family-comparison.md
[qemu-edu-cml]: https://fuchsia.googlesource.com/sdk-samples/drivers/+/refs/heads/main/src/qemu_edu/meta/qemu_edu.cml
[component-manifests]: /concepts/components/v2/component_manifests.md
[qemu-edu-bind]: https://fuchsia.googlesource.com/sdk-samples/drivers/+/refs/heads/main/src/qemu_edu/qemu_edu.bind
[driver-binding-dfv1]: /development/drivers/concepts/device_driver_model/driver-binding.md
[qemu-edu-fidl]: https://fuchsia.googlesource.com/sdk-samples/drivers/+/refs/heads/main/src/qemu_edu/qemu_edu.fidl
[fidl-reference]: https://fuchsia.dev/reference/fidl
[qemu-edu-h]: https://fuchsia.googlesource.com/sdk-samples/drivers/+/refs/heads/main/src/qemu_edu/qemu_edu.h
[registers-h]: https://fuchsia.googlesource.com/sdk-samples/drivers/+/refs/heads/main/src/qemu_edu/registers.h
[qemu-edu-cc]: https://fuchsia.googlesource.com/sdk-samples/drivers/+/refs/heads/main/src/qemu_edu/qemu_edu.cc
[qemu-edu-cc-line-133]: https://fuchsia.googlesource.com/sdk-samples/drivers/+/refs/heads/main/src/qemu_edu/qemu_edu.cc#133
[qemu-edu-cc-line-45]: https://fuchsia.googlesource.com/sdk-samples/drivers/+/refs/heads/main/src/qemu_edu/qemu_edu.cc#45
[vmo]: /concepts/kernel/concepts.md#shared_memory_virtual_memory_objects_vmos
[qemu-edu-cc-line-82]: https://fuchsia.googlesource.com/sdk-samples/drivers/+/refs/heads/main/src/qemu_edu/qemu_edu.cc#82
[edu-c-line-314]: https://fuchsia.googlesource.com/third_party/qemu/+/refs/heads/main/hw/misc/edu.c#314
[qemu-edu-cc-line-177]: https://fuchsia.googlesource.com/sdk-samples/drivers/+/refs/heads/main/src/qemu_edu/qemu_edu.cc#177
[edu-c]: https://fuchsia.googlesource.com/third_party/qemu/+/refs/heads/main/hw/misc/edu.c
[edu-c-line-251]: https://fuchsia.googlesource.com/third_party/qemu/+/refs/heads/main/hw/misc/edu.c#251
[edu-c-line-314]: https://fuchsia.googlesource.com/third_party/qemu/+/refs/heads/main/hw/misc/edu.c#314
[edu-c-line-206]: https://fuchsia.googlesource.com/third_party/qemu/+/refs/heads/main/hw/misc/edu.c#206
[eductl-cml]: https://fuchsia.googlesource.com/sdk-samples/drivers/+/refs/heads/main/src/qemu_edu/meta/eductl.cml
[eductl-cc]: https://fuchsia.googlesource.com/sdk-samples/drivers/+/refs/heads/main/src/qemu_edu/eductl.cc
[service-discovery]: /concepts/drivers/driver_communication.md#service_discovery_using_devfs
[femu]: /development/sdk/ffx/start-the-fuchsia-emulator.md

