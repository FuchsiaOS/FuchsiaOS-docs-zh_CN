# Fuchsia Wireless Networking

## Introduction

Fuchsia's wireless networking stack intends to provide a compliant non-AP station
implementation of IEEE Std 802.11. It supports hardware with both "full MAC" and
"soft MAC" firmware, in which the MLME layer of the 802.11 spec is implemented
in the firmware and the host OS, respectively.

## High-level architecture

```
                          +------------------+        +------------------+
 Fuchsia service          | Fuchsia netstack |        | Fuchsia Wireless |
                          |                  |        | Network Service  |
                          +------------------+        +------------------+
                              ^                        ^                ^
                              |                        |                |
 fdio/FIDL              ------|------------------------|----------------|-------------------
                              |                        |                |
                              v                        |                v
                          +------------------+         |               +--------------+
                          | Fuchsia ethernet |<--------|-------------->| Fuchsia WLAN |
                          | driver           |         |               | MLME driver  |
 devmgr                   +------------------+         |               +--------------+
                                         ^             |                    ^
                                         |             |                    |
                                         v             v                    v
                                    +-------------------+              +-------------------+
                                    | Driver            |              | Driver            |
                                    | (Full MAC device) |              | (Soft MAC device) |
                                    +-------------------+              +-------------------+
                                                      ^                    ^
                                                       \                  /
 hardware bus                       --------------------\----------------/------------------
 (USB, PCI, etc)                                         \              /
                                                          v            v
                                                     +---------------------+
                                                     | Wireless networking |
 hardware                                            | hardware            |
                                                     +---------------------+
```


## Drivers

A Full MAC driver relies on the firmware in the wireless hardware to implement
the majority of the IEEE 802.11 MLME functions.

A Soft MAC driver implements the basic building blocks of communication with the
wireless hardware in order to allow the Fuchsia MLME driver to execute the IEEE
802.11 MLME functions.

The Fuchsia MLME driver is a hardware-independent layer that provides state
machines for synchronization, authentication, association, and other wireless
networking state. It communicates with a Soft MAC driver to manage the hardware.

## WLAN service

The Fuchsia Wireless Network Service implements the IEEE 802.11 SME functions
and holds state about all the wireless networks that are available in the
current environment. It is the interface to the hardware (via the drivers) used
by components like System UI.

## Relation to the Ethernet stack

Either a Full MAC driver or the Fuchsia WLAN MLME driver will expose an Ethernet
device in devmgr. This device will behave as any other Ethernet device, and will
provide data packets to the rest of the system. TBD: whether to use Ethernet II
frames always, or support 802.2 SNAP frames.

## Interfaces

The Fuchsia Wireless Network Service will communicate with each hardware device
using a channel to the driver, obtained via ioctl. (Eventually this will be
replaced by FIDL.) Messages exchanged over this channel will encode the
request/response for each action, generally following the IEEE 802.11 MLME SAP
definitions.

For Soft MAC devices, the hardware driver and the generic MLME driver will
communicate in-process using a DDK "protocol" for wlan devices. Primitives
exposed through this interface include send, receive, and setting the radio
channel.
