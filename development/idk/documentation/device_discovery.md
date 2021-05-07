# `device-finder`

`device-finder` is the command line tool for device discovery. It uses mDNS to
find Fuchsia devices.

Currently only Linux is supported. For Mac users see the section
[for Mac Users](#for_mac_users).

## For Linux Users

### Finding all Devices

The simplest way to get all the devices on the network by their address is to
run

```
$ ./device-finder list
192.168.42.156
```

This will give you the addresses of all Fuchsia devices on the network. If you'd
like to get their hostnames as well as their addresses, you can include the
`-full` flag.

### Finding devices by hostname

If you'd like to find your device by its unique hostname (e.g.
`lunch-feta-stool-woozy`) you can use the `resolve` command:

```
$ ./device-finder resolve lunch-feta-stool-woozy
192.168.42.156
```

### Finding the Interface Connected to the Device

To find the interface connected to the device, include the `-local` flag to
either the `list` command or the `resolve` command, which will give you the
address that the Fuchsia device can use to connect to your host.

## For Mac Users

For those on Mac hosts, you can use the included `dns-sd` command to find your
device. Here's an example command along with the output you should see when a
device is on your network:

```
$ dns-sd -B _fuchsia._udp .
Browsing for _fuchsia._udp
DATE: ---Fri 14 Dec 2018---
15:28:21.447  ...STARTING...
Timestamp     A/R    Flags  if Domain       Service Type   Instance Name
15:28:21.448  Add        2   7 local.       _fuchsia._udp. quake-agile-lurk-even
```

Mac does not support the equivalent of a `local` flag as described above in the
`device-finder` docs.

## For additional help

For additional help using any of the `device-finder` subcommands, run the subcommand with the `-help` flag.
