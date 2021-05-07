# Download the Fuchsia IDK

You can download the Fuchsia Integrator Developer Kit (IDK) using the links below. Please be aware that
Fuchsia is under active development and its API surface is subject to frequent
changes. The Fuchsia IDK is produced continuously as Fuchsia is developed.

Because the [Fuchsia System Interface](/docs/concepts/system/abi/system.md) is changing, you will
need to run software built using a particular version of the IDK on a Fuchsia
system with a matching version. The [IDK](#core) contains a matching system
image appropriate for running in [Qemu](#qemu).

## Integrator Developer Kit (IDK) {#core}

The Integrator Developer Kit (IDK) is independent of any specific build system or development environment.
The IDK contains metadata that can be used by an [IDK backend](README.md#backend) to
generate an SDK for a specific build system.

* [Linux](https://chrome-infra-packages.appspot.com/p/fuchsia/sdk/core/linux-amd64/+/latest)
* [MacOS](https://chrome-infra-packages.appspot.com/p/fuchsia/sdk/core/mac-amd64/+/latest)

## Qemu

A distribution of [Qemu](https://www.qemu.org/) that has been tested to work
with Fuchsia system images contained in the IDK.

* [Linux (amd64)](https://chrome-infra-packages.appspot.com/p/fuchsia/qemu/linux-amd64/+/latest)
* [Linux (arm64)](https://chrome-infra-packages.appspot.com/p/fuchsia/qemu/linux-arm64/+/latest)
* [MacOS (amd64)](https://chrome-infra-packages.appspot.com/p/fuchsia/qemu/mac-amd64/+/latest)
