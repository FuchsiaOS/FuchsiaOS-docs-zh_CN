# ffx workflows

[`ffx`][ffx-reference] is a CLI (command line interface) tool that supports
common Fuchsia development workflows, such as interacting with Fuchsia
target devices and managing [Fuchsia components][fuchsia-components] and
[Fuchsia packages][fuchsia-packages].

## Table of contents

* Devices

  * [Start the Fuchsia emulator][start-femu]
  * [Flash a Fuchsia image on a device][flash-device]
  * [Create SSH keys for Fuchsia devices][create-ssh-keys]
  * [View device information][view-device-info]
  * [View device logs][view-device-logs]

* Components

  * [View component information][view-component-info]
  * [Start a component during development][start-a-component]
  * [Copy files to and from a component][copy-files]

* Packages

  * [Create a Fuchsia package repository][create-a-package-repo]
  * [Update Fuchsia packages on a device][update-packages]

* Debugging

  * [Register debug symbols][register-symbols]
  * [Symbolize logs][symbolize-logs]
  * [Start the Fuchsia debugger][start-zxdb]
  * [Monitor FIDL messages on a device][monitor-fidl]

* Performance

  * [Record traces for performance analysis][record-traces]

<!-- Reference links -->

[ffx-reference]: https://fuchsia.dev/reference/tools/sdk/ffx
[fuchsia-components]: concepts/components/v2/README.md
[fuchsia-packages]: concepts/packages/package.md
[start-femu]: start-the-fuchsia-emulator.md
[view-device-info]: view-device-information.md
[flash-device]: flash-a-device.md
[create-ssh-keys]: create-ssh-keys-for-devices.md
[view-device-logs]: view-device-logs.md
[view-component-info]: view-component-information.md
[start-a-component]: start-a-component-during-development.md
[copy-files]: copy-files-to-and-from-a-component.md
[create-a-package-repo]: create-a-package-repository.md
[update-packages]: update-packages-on-a-device.md
[register-symbols]: register-debug-symbols.md
[symbolize-logs]: symbolize-logs.md
[start-zxdb]: start-the-fuchsia-debugger.md
[monitor-fidl]: monitor-fidl-messages-on-a-device.md
[record-traces]: record-traces.md
