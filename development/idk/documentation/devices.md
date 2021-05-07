# Working with target devices

## Connecting to a device

Fuchsia target devices must be connected to a host device via a network link.
SSH is the protocol for communications over that link, as described in
[this document](ssh.md).

### Getting the device address

Getting the Fuchsia device address can be done using mDNS. Methods for device
discovery are outlined in [this document](device_discovery.md)

## Flashing a device

In order to flash a device, start a [bootserver](bootserver.md) on the host and
restart the device into its bootloader.

## Installing software onto a device

The unit of installation on Fuchsia is a package.
For information on how to push packages to a Fuchsia device, see the
[this document](packages.md).

## Getting logs from a device

In order to retrieve logs from a device, open a shell on the device and run the
`log_listener` command, which provides various filtering knobs. See
[this page](logging.md) for more details.
