# Start the Fuchsia emulator

This guide provides instructions on how to set up and launch the
Fuchsia emulator (FEMU) on your machine.

The steps are:

1. [Prerequisites](#prerequisites).
1. [Build Fuchsia for FEMU](#build-fuchsia-for-femu).
1. [Enable VM acceleration (Optional)](#enable-vm-acceleration).
1. [Start FEMU](#start-femu).
1. [Discover FEMU](#discover-femu).

## 1. Prerequisites {#prerequisites}

Running FEMU requires that you've completed the following guides:

 * [Download the Fuchsia source code][get-fuchsia-source]
 * [Configure and build Fuchsia][build-fuchsia]

## 2. Build Fuchsia for FEMU {#build-fuchsia-for-femu}

To run FEMU, you first need to build a Fuchsia system image that supports
the emulator environment. This guide uses `qemu-x64` for the board
and `workstation` for the product as an example.

To build a FEMU Fuchsia image, do the following:

1. Set the Fuchsia build configuration:

   ```posix-terminal
   fx set workstation.qemu-x64 --release
   ```

2. Build Fuchsia:

   ```posix-terminal
   fx build
   ```

For more information on supported boards and products, see the
[Fuchsia emulator (FEMU)][femu-overview] overview page.

## 3. Enable VM acceleration (Optional) {#enable-vm-acceleration}

(**Linux only**) Most Linux machines support VM acceleration through
KVM, which greatly improves the performance and usability of the emulator.

If KVM is available on your machine, update your group permission to
enable KVM.

* {Linux}

  To enable KVM on your machine, do the following:

  Note: You only need to do this once per machine.

  1.  Add yourself to the `kvm` group on your machine:

      ```posix-terminal
      sudo usermod -a -G kvm ${USER}
      ```

  1.  Log out of all desktop sessions to your machine and then log in again.

  1.  To verify that KVM is configured correctly, run the following command:

      ```posix-terminal
      if [[ -r /dev/kvm ]] && grep '^flags' /proc/cpuinfo | grep -qE 'vmx|svm'; then echo 'KVM is working'; else echo 'KVM not working'; fi
      ```

      Verify that this command prints the following line:

      ```none {:.devsite-disable-click-to-copy}
      KVM is working
      ```

      If you see `KVM not working`, you may need to reboot your machine for
      the permission change to take effect.

* {macOS}

  No additional setup is required for macOS.

  Instead of KVM, the Fuchsia emulator on macOS uses the
  [Hypervisor framework][hypervisor-framework]{: .external}.

## 4. Start FEMU {#start-femu}


### Start the package server

Prior to starting the emulator, start the package server.

To start the the package server, run the following command:

  ```posix-terminal
  fx serve
  ```
Note: Alternatively you can background the `fx serve` process.

### Start the emulator

To start the emulator on your Linux machine, do the following:

* {Linux}

  1. Configure the upscript by running the following command:

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
      ffx config set emu.upscript {{ '<var>' }}FUCHSIA_ROOT{{ '</var>' }}/scripts/start-unsecure-internet.sh
      ```
      * `start-unsecure-internet.sh` is an example upscript.
      * `FUCHSIA_ROOT` is the path to your Fuchsia directory.

  1. Start FEMU

      1. To start the emulator with access to external networks, run the
         following command:

          ```posix-terminal
          ffx emu start --net tap
          ```

          * `--net` specifies the networking mode for the emulator. `--net tap`
          attaches to a Tun/Tap interface.

      1. To start the emulator without access to external networks, run
         the following command:

          ```posix-terminal
          ffx emu start --net none
          ```

    Starting the emulator opens a new window with the
    title **Fuchsia Emulator**. When the emulator is finished booting, you are
    returned to the command prompt, and the emulator runs in the background.

* {macOS}

  To start FEMU on macOS, do the following:

  1. Start FEMU:

     ```posix-terminal
     ffx emu start
     ```

     If you launch FEMU for the first time on your macOS (including after a reboot),
     a window pops up asking if you want to allow the process `aemu` to run on your
     machine. Click **Allow**.

     This command opens a new window with the title **Fuchsia Emulator**.
     When the emulator is finished booting, you are returned to the command
     prompt, and the emulator runs in the background.

  2. (Optional) If you need to specify the launched Fuchsia emulator, you can
     run the `fx set-device` command in the same terminal:

     ```posix-terminal
     fx set-device {{ '<var>' }}NAME{{ '</var>' }}
     ```

     Replace the following:

     * `NAME`: Use the desired value from the `ffx emu list` or `ffx target list`
       command's output. `fuchsia-emulator` is the default value.

## 5. Discover FEMU {#discover-femu}

To discover the Fuchsia emulator as a running Fuchsia device, run the
following command:

```posix-terminal
ffx target list
```

This command prints output similar to the following:

```none {:.devsite-disable-click-to-copy}
$ ffx target list
NAME                      SERIAL       TYPE                    STATE      ADDRS/IP                            RCS
fuchsia-emulator    <unknown>    workstation.qemu-x64    Product    [fe80::866a:a5ea:cd9e:69f6%qemu]    N
```

`fuchsia-emulator` is the default node name of the Fuchsia emulator.

The output of `ffx target list` is influenced by the `--net` option in the
following ways:

   * `--net none` disables networking, which causes the device to not be
   discoverable when running `ffx target list`.
   * `--net tap` and `--net user` allow the device to be discoverable
   when running `ffx target list`.


## Next steps

To learn more about Fuchsia device commands and Fuchsia workflows, see
[Explore Fuchsia][explore-fuchsia].

## Appendices

This section provides additional FEMU options.

### See all available flags

To see a full list of the emulator's supported flags:

```posix-terminal
ffx emu start --help
```

### Run FEMU without GUI support

If you don't need graphics or working under the remote workflow,
you can run FEMU in headless mode:

```posix-terminal
ffx emu start --headless
```

### Specify GPU used by FEMU

By default, the FEMU launcher uses software rendering using
[SwiftShader][swiftshader]{: .external}. To force FEMU to use a specific
graphics emulation method, use the parameters `--gpu host` or
`--gpu guest` with the `ffx emu start` command.

These are the valid commands and options:

<table><tbody>
  <tr>
   <th>GPU Emulation method</th>
   <th>Explanation</th>
   <th>Flag</th>
  </tr>
  <tr>
   <td>Hardware (host GPU)</td>
   <td>Uses the host machine's GPU directly to perform GPU processing.</td>
   <td><code>ffx emu start --gpu host</code></td>
  </tr>
  <tr>
   <td>Software (host CPU)</td>
   <td>Uses the host machine's CPU to simulate GPU processing.</td>
   <td><code>ffx emu start --gpu guest</code></td>
  </tr>
  <tr>
   <td>SwiftShader</td>
   <td>Uses SwiftShader libraries to simulate GPU processing.</td>
   <td><code>ffx emu start --gpu swiftshader_indirect</code></td>
  </tr>
  <tr>
   <td>Auto</td>
   <td>Resolves to <code>host</code> if there is a hardware GPU available or
       <code>swiftshader_indirect</code> if there isn't a hardware GPU available.
       <code>auto</code> is the current default.</td>
   <td><code>ffx emu start --gpu auto</code></td>
  </tr>
</tbody></table>

### Reboot FEMU {#reboot-femu}

To reboot FEMU, run the following `ffx` command:

```posix-terminal
ffx target reboot
```

### Stop FEMU {#stop-femu}

To stop FEMU, run the following `ffx` command:

```posix-terminal
ffx emu stop
```

### Configure IPv6 network {#configure-ipv6-network}

This section provides instructions on how to configure an IPv6 network
for FEMU on Linux machine using [TUN/TAP][tuntap]{: .external}.

* {Linux}

  Note: This has to be completed once per machine.

  To enable networking in FEMU using
  [tap networking][tap-networking]{: .external}, do the following:

  1. Set up `tuntap`:

     ```posix-terminal
     sudo ip tuntap add dev qemu mode tap user $USER
     ```

  1. Enable the network for `qemu`:

     ```posix-terminal
     sudo ip link set qemu up
     ```

* {macOS}

  No additional IPv6 network setup is required for macOS.

  [User Networking (SLIRP)][slirp]{: .external} is the default network setup
  for FEMU on macOS – while this setup does not support Fuchsia device
  discovery, you can still use `fx` tools (for example,`fx ssh`) to
  interact with your FEMU instance.

<!-- Reference links -->

[get-fuchsia-source]: /docs/get-started/get_fuchsia_source.md
[build-fuchsia]: /docs/get-started/build_fuchsia.md
[femu-overview]: /docs/development/build/emulator.md
[hypervisor-framework]: https://developer.apple.com/documentation/hypervisor
[explore-fuchsia]: /docs/get-started/explore_fuchsia.md
[swiftshader]: https://swiftshader.googlesource.com/SwiftShader/
[tuntap]: https://en.wikipedia.org/wiki/TUN/TAP
[tap-networking]: https://wiki.qemu.org/Documentation/Networking#Tap
[slirp]: https://wiki.qemu.org/Documentation/Networking#User_Networking_.28SLIRP.29
