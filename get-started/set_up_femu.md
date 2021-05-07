# Set up and start the Fuchsia emulator (FEMU)

This document describes how to set up and run the Fuchsia emulator (FEMU), including networking
and GPU support setup.

## Prerequisites

To run FEMU, you must have:

 * [Checked out the Fuchsia source and set up some environment variables](/docs/get-started/get_fuchsia_source.md)
 * [Configured and built Fuchsia](/docs/get-started/build_fuchsia.md)

### Building Fuchsia for FEMU

Before you can use FEMU, you need to build Fuchsia using `fx set`, 
specifying a qemu board and supported product. This example uses
`qemu-x64` for the board and `workstation` for the product:

<pre class="prettyprint">
<code class="devsite-terminal">fx set workstation.qemu-x64 --release [--with=...]</code>
<code class="devsite-terminal">fx build</code>
</pre>

Note: More information on supported boards and products is in the
[Fuchsia emulator overview](/docs/concepts/emulator/index.md).

## Configure network

For Fuchsia's ephemeral software to work with FEMU, you need to configure
an IPv6 network.

  * [Linux configuration](#linux-config)
  * [macOS configuration](#mac-config)

### Linux {#linux-config}

To enable networking in FEMU using [tap networking](https://wiki.qemu.org/Documentation/Networking#Tap), run the following commands:

<pre class="prettyprint">
<code class="devsite-terminal">sudo ip tuntap add dev qemu mode tap user $USER</code>
<code class="devsite-terminal">sudo ip link set qemu up</code>
</pre>


### macOS {#mac-config}

[User Networking (SLIRP)](https://wiki.qemu.org/Documentation/Networking#User_Networking_.28SLIRP.29){: .external} is the default networking set up for FEMU on macOS. This networking set up does not support Fuchsia device discovery.

## Start FEMU

The most common way to run FEMU is with networking enabled, using the following commands.

### Linux {#linux-start-femu}

To support device discovery without access to external networks.

```posix-terminal
fx vdl start -N
```

To get access to external networks:

{% dynamic if user.is_googler %}

Note: Command will differ depending on the type of machines you use.

* {Corp}

  To use FEMU on a corp machine, see [go/fuchsia-emulator-corp](http://go/fuchsia-emulator-corp).

* {Non-Corp}

  Note: `FUCHSIA_ROOT` is the path to the Fuchsia checkout on your local machine (ex: `~/fuchsia`).

  ```posix-terminal
  fx vdl start -N -u {{ '<var>' }}FUCHSIA_ROOT{{ '</var>' }}/scripts/start-unsecure-internet.sh
  ```

{% dynamic else %}

Note: `FUCHSIA_ROOT` is the path to the Fuchsia checkout on your local machine (ex: `~/fuchsia`).

```posix-terminal
fx vdl start -N -u {{ '<var>' }}FUCHSIA_ROOT{{ '</var>' }}/scripts/start-unsecure-internet.sh
```
{% dynamic endif %}


Once you run the command, a separate window opens with the title "Fuchsia Emulator". After
the Fuchsia emulator launches successfully, the terminal starts with the SSH console. You
can run shell commands in this window, just like you would on a Fuchsia device.

### macOS {#mac-start-femu}

On macOS, Fuchsia device discovery does not work. However, you can still use `fx` tools such as `fx ssh`.


```posix-terminal
fx vdl start
```

From the output, take note of the instruction on running `fx set-device`, you will need it for the steps below.

Note: When you launch FEMU for the first time on your Mac machine after starting up (ex: after a reboot),
a window pops up asking if you want to allow the process “aemu” to run on your machine.
Click “allow”.

Run `fx set-device` to specify the launched Fuchsia emulator SSH port. For `SSH_PORT`, use the value that the `fx vdl start --host-gpu` command outputted.


```posix-terminal
fx set-device 127.0.0.1:{{ '<var>' }}SSH_PORT{{ '</var>' }}
```

## Additional FEMU options

### Input options

By default FEMU uses multi-touch input. You can add the argument `--pointing-device mouse`
for mouse cursor input instead.

```posix-terminal
fx vdl start --pointing-device mouse
```

### Run FEMU without GUI support

If you don't need graphics or working under the remote workflow, you can run FEMU in headless mode:

```posix-terminal
fx vdl start --headless
```

### Specify GPU used by FEMU

By default, FEMU launcher uses software rendering using [SwiftShader](https://swiftshader.googlesource.com/SwiftShader/). 
To force FEMU to use a specific graphics emulation method, use the parameters `--host-gpu` or `--software-gpu` to the `fx vdl start` command.

These are the valid commands and options:

<table><tbody>
  <tr>
   <th>GPU Emulation method</th>
   <th>Explanation</th>
   <th><code>fx vdl start</code> flag</th>
  </tr>
  <tr>
   <td>Hardware (host GPU)</td>
   <td>Uses the host machine’s GPU directly to perform GPU processing.</td>
   <td><code>fx vdl start --host-gpu</code></td>
  </tr>
  <tr>
   <td>Software (host CPU)</td>
   <td>Uses the host machine’s CPU to simulate GPU processing.</td>
   <td><code>fx vdl start --software-gpu</code></td>
  </tr>
</tbody></table>

### Supported hardware for graphics acceleration {#supported-hardware}

FEMU currently supports a limited set of GPUs on macOS and Linux for
hardware graphics acceleration. FEMU uses a software renderer fallback for unsupported GPUs.

<table>
  <tbody>
    <tr>
      <th>Operating System</th>
      <th>GPU Manufacturer</th>
      <th>OS / Driver Version</th>
    </tr>
    <tr>
      <td>Linux</td>
      <td>Nvidia Quadro</td>
      <td>Nvidia Linux Drivers <a href="https://www.nvidia.com/download/driverResults.aspx/160175/en-us">440.100</a>+</td>
    </tr>
    <tr>
      <td>macOS</td>
      <td><a href="https://support.apple.com/en-us/HT204349#intelhd">Intel HD Graphics</a></td>
      <td>macOS version 10.15+</td>
    </tr>
    <tr>
      <td>macOS</td>
      <td>AMD Radeon Pro</td>
      <td>macOS version 10.15+</td>
    </tr>
  </tbody>
</table>

## Exit FEMU

To exit FEMU, run `dm poweroff` in the FEMU terminal.

## Next steps

 *  To learn more about how FEMU works, see the
    [Fuchsia emulator (FEMU) overview](/docs/concepts/emulator/index.md).
 *  To learn more about Fuchsia device commands and Fuchsia workflows, see
    [Explore Fuchsia](/docs/get-started/explore_fuchsia.md).

