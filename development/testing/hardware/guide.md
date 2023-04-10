{% import 'fuchsia-src/_common/_doc_widgets.md' as widgets %}

# Hardware testing guide

This guide shows you ways to test the hardware subsystems of a
physical Fuchsia device.

Experimental: All of the tools and workflows mentioned in this
guide are experimental and can change at anytime.

## Intended audience of this guide {: #audience }

This guide assumes that you're familiar with hardware and
low-level software development.

## Overview {: #overview }

All of the Fuchsia hardware testing workflows in this guide assume
that your **Fuchsia device** is connected to a **test host**. The
test host is a laptop or desktop running macOS, Linux, or Windows.
To test a hardware subsystem of your Fuchsia device you run a command
like `adb shell gpioutil list` on the test host. The first part of the command,
`adb shell`, is the **transport system** that handles communication between
your test host and Fuchsia device. The second part of the command,
`gpioutil list`, is the **testing tool** that actually exercises or queries
a hardware subsystem on your Fuchsia device.

[sequence]: /development/testing/hardware/sequence.png

![A sequence diagram of the `adb shell gpioutil list` workflow.][sequence]

<!--
https://sequencediagram.org diagram source code:

```
title Hardware Testing Workflow Example

participant "Test Host" as host
participant "Fuchsia Device" as target

host->target: adb shell gpioutil list
target->target: gpioutil::ListGpios()
target->host: [gpio-14] VIM3_ETH_MAC_INTR\n[gpio-15] VIM3_J4_PIN_39\n...
```
-->

## Supported setups {: #supported-setups }

The OS of your test host determines what transport systems you
can use. For example, currently only `adb` is supported
on Windows. The following table shows what combinations of test
host OS and transport system are supported:

<table>
  <thead>
    <tr>
      <th>Test Host OS</th>
      <th><code>ffx</code></th>
      <th><code>adb</code></th>
      <th>UART</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>macOS</td>
      <td><span style="color:green" class="material-icons">check_circle</span></td>
      <td><span style="color:green" class="material-icons">check_circle</span></td>
      <td><span style="color:green" class="material-icons">check_circle</span></td>
    </tr>
    <tr>
      <td>Linux</td>
      <td><span style="color:green" class="material-icons">check_circle</span></td>
      <td><span style="color:green" class="material-icons">check_circle</span></td>
      <td><span style="color:green" class="material-icons">check_circle</span></td>
    </tr>
    <tr>
      <td>Windows</td>
      <td><span style="color:red" class="material-icons">cancel</span></td>
      <td><span style="color:green" class="material-icons">check_circle</span></td>
      <td><span style="color:green" class="material-icons">check_circle</span></td>
    </tr>
  </tbody>
</table>

The transport system determines what kind of physical connection
you can use between your test host and target device. For example,
`adb` requires a USB connection between the test host and Fuchsia
device. The hardware layout of your Fuchsia device also determines
what transport system you can use. For example, if your Fuchsia device
doesn't expose GND, TX, and RX pins, then you can't use UART.

## Configuring Fuchsia builds for hardware testing {: #build }

This section provides guidance for people who need to build a Fuchsia
image with hardware testing tools enabled from source. If you already have a
Fuchsia image with the tools you need then you can skip this section.

### Building Fuchsia from source {: #build-basics }

If you've never built Fuchsia from source and need help with the
basic workflows, see the following tutorials:

1. [Download the source code](/get-started/get_fuchsia_source.md)
1. [Configure and build Fuchsia](/get-started/build_fuchsia.md)

### Including hardware testing tools in your build {: #include }

If you try running a hardware testing tool (like `gpioutil` for example)
and get a `not found` error, it probably means that the hardware
testing tool was not enabled when your Fuchsia image was built.
Example:

```
-v: 1: gpioutil: not found
```

[`fx set`]: /reference/tools/fx/cmd/set.md

To fix this, try adding `--with-base //bundles/tools` to your [`fx set`]
call:

```posix-terminal
fx set {{"<var>"}}PRODUCT{{"</var>"}}.{{"<var>"}}BOARD{{"</var>"}} --with-base '//bundles/tools'
```

[base packages]: /concepts/packages/package.md#base-packages
[`//bundles/tools/BUILD.gn`]: /bundles/tools/BUILD.gn

The `--with-base` option adds all of the dependencies listed in
[`//bundles/tools/BUILD.gn`] as [base packages] in your Fuchsia image.
Most of Fuchsia's hardware testing tools are in this bundle.

[product]: /development/build/build_system/boards_and_products.md#products
[board]: /development/build/build_system/boards_and_products.md#boards

Caution: {{"<var>"}}PRODUCT{{"</var>"}} and {{"<var>"}}BOARD{{"</var>"}}
are placeholder values. You need to replace them with real [product]
and [board] names. Run `fx list-products` and `fx list-boards` to
see which products and boards are available in your Fuchsia checkout.
You should also double-check that you are using the correct product
and board. If you're using the wrong product or board, that could
also explain why a hardware testing tool is missing.

#### Include a single tool directly {: #include-single }

If the hardware testing tool you need is not listed in [`//bundles/tools/BUILD.gn`]
try setting the `--with-base` value to the path to the hardware testing
tool's `BUILD.gn` file. For example, the `gpioutil` build file is located at
`//src/devices/gpio/bin/gpioutil/BUILD.gn`. You can directly include `gpioutil`
as a base package with the following `fx set` command:

```posix-terminal
fx set {{"<var>"}}PRODUCT{{"</var>"}}.{{"<var>"}}BOARD{{"</var>"}} --with-base '//src/devices/gpio/bin/gpioutil'
```

You can repeat the `--with-base` option as many times as needed:

```posix-terminal
fx set {{"<var>"}}PRODUCT{{"</var>"}}.{{"<var>"}}BOARD{{"</var>"}} \
    --with-base '//src/devices/gpio/bin/gpioutil' \
    --with-base '//src/media/audio/tools/audio-driver-ctl'
```

#### Include adb {: #include-adb }

To enable `adb` in your Fuchsia image, add the following options
to your `fx set` command:

```posix-terminal
fx set {{"<var>"}}PRODUCT{{"</var>"}}.{{"<var>"}}BOARD{{"</var>"}} \
    --args 'dev_kernel_cmdline=["driver.usb.peripheral=adb"]' \
    --args 'dev_bootfs_labels=["//src/developer/adb:drivers"]' \
    --with-base '//src/developer/adb:adb' \
    --args 'core_realm_shards+=["//src/developer/adb:core_shards"]'
```

Use `driver.usb.peripheral=cdc_adb` rather than `driver.usb.peripheral=adb`
to enable networking and `adb` interfaces simultaneously.

[`//src/developer/adb/README.md`]: /src/developer/adb/README.md

See [`//src/developer/adb/README.md`] for more information.

## Running commands {: #transport }

As explained in [Overview](#overview), you send commands from
your test host to your Fuchsia device over a transport system
like `ffx`, `adb`, or UART. This section provides more detail
about how to run commands with each transport system.

### A key difference between transport systems: capabilities {: #capabilities }

The underlying {{ widgets.glossary_simple ('component', 'component')}}
that powers each transport system affects what features are available
to you. Different components expose different
{{ widgets.glossary_simple ('capability', 'capabilities')}}.

[`console-launcher`]: /src/bringup/bin/console-launcher/README.md

The capabilities available in [`ffx`](#ffx) are determined by
the component you're exploring. The capabilities available in
[`adb`](#adb) and [UART](#uart) are determined by [`console-launcher`],
which is the underlying component that powers those shells.

### ffx {: #ffx }

[`ffx`](/development/tools/ffx/overview.md) is Fuchsia's main tool
for host-target interactions.

To start an interactive shell:

```posix-terminal
ffx component explore {{"<var>"}}COMPONENT{{"</var>"}} -l namespace
```

Caution: {{"<var>"}}COMPONENT{{"</var>"}} is a placeholder value.
You need to replace it with the name of a real diagnostic component.

To run single commands (`gpioutil list` for example):

```posix-terminal
ffx component explore {{"<var>"}}COMPONENT{{"</var>"}} -l namespace -c 'gpioutil list'
```

### UART {: #uart }

Most Fuchsia {{ widgets.glossary_simple ('board', 'boards')}} support
3-pin (TX, RX, GND) UART communication. They usually expect the
typical [115200 8N1](https://en.wikipedia.org/wiki/8-N-1)
UART configuration. To send commands from your test host to your
Fuchsia device you'll need a serial console like `minicom` or
[`fx serial`](/reference/tools/fx/cmd/serial).

### adb {: #adb }

[`adb`]: https://developer.android.com/studio/command-line/adb

If your Fuchsia image [included `adb`](#include-adb) then you can send
commands from your test host to your Fuchsia device with [`adb`].

To start an interactive shell:

```posix-terminal
adb shell
```

To run single commands (`gpioutil list` for example):

```posix-terminal
adb shell 'gpioutil list'
```

[RFC-0200]: /contribute/governance/rfcs/0200_support_adb_protocol_and_interface_for_hardware_testing.md

Fuchsia only has partial support for `adb` by design. See [RFC-0200] for
details on what's supported and what's not. See
[`//src/developer/adb/README.md`] for more guidance on using `adb` with Fuchsia.

Tip: If you have access to an {{ widgets.glossary_simple ('in-tree', 'in-tree')}}
Fuchsia checkout there's a copy of `adb` that you can use at
`//prebuilt/starnix/internal/android-image-amd64/bin/adb`.

## List of hardware testing tools {: #tools }

The following table points you to the specific tools that you
can use to test out the various hardware subsystems on your
Fuchsia device.

<table class="alternating-odd-rows">
  <thead>
    <tr>
      <th>Hardware subsystem</th>
      <th>Tools</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>Audio</td>
      <td>
        <ul>
          <li><a href="/reference/tools/hardware/audio-driver-ctl.md"><code>audio-driver-ctl</code></a></li>
        </ul>
      </td>
    </tr>
    <tr>
      <td>GPIO</td>
      <td>
        <ul>
          <li><a href="/reference/tools/hardware/gpioutil.md"><code>gpioutil</code></a></li>
        </ul>
      </td>
    </tr>
    <tr>
      <td>I2C</td>
      <td>
        <ul>
          <li><a href="/reference/tools/hardware/i2cutil.md"><code>i2cutil</code></a></li>
        </ul>
      </td>
    </tr>
    <tr>
      <td>Lights</td>
      <td>
        <ul>
          <li><a href="/reference/tools/hardware/lights-cli.md"><code>lights-cli</code></a></li>
        </ul>
      </td>
    </tr>
  </tbody>
</table>
