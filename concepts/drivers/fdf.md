# Fuchsia Driver Framework

## Overview
Fuchsia Driver Framework (FDF) is a collection of libraries, tools, metadata and images that enables
driver writers to develop, test and distribute drivers targeting Fuchsia. This is aimed to provide a
stable ABI allowing driver developers to potentially write a driver once and use it on multiple
versions of the Fuchsia kernel and platform.

At the moment, the FDF is composed of a driver manager, driver host, core library (libdriver),
FIDL interfaces, banjo interfaces and guidelines to develop drivers for Fuchsia.
FDF is constantly evolving and yet to achieve ABI stability.

## Driver manager

Driver manager is a binary maintained and developed as part of FDF. It is responsible to
load drivers and manage devices on all platforms. This is one of the initial process to be started
on device bootup. It finds driver packages in pre-configured paths, tries to match a
driver for every device by running the driver's bind program, and manages the device lifecycle.
It hosts a virtual filesystem named as Device Filesystem (`devfs`), that provides
uniform access to all devices from userspace services/components external to the drivers. `devfs`
is mounted under `/dev` and contains virtual files that eventually route to interfaces
implemented by the devices.

## Driver host

Driver host is a binary that is launched by driver manager to host one or more drivers. It
facilitates sandboxing of drivers.
