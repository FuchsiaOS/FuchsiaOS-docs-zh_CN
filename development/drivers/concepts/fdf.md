# Fuchsia Driver Framework

Caution: This page may contain information that is specific to the legacy
version of the driver framework (DFv1).

## Overview
Fuchsia Driver Framework (FDF) is a collection of libraries, tools, metadata and images that enables
driver writers to develop, test and distribute drivers targeting Fuchsia. It aims to provide a
stable ABI allowing driver developers to potentially write a driver once and use it on multiple
versions of the Fuchsia kernel and platform.

At the moment, the FDF is composed of a driver manager, driver host, core library (libdriver),
FIDL interfaces, banjo interfaces and guidelines to develop drivers for Fuchsia.
FDF is constantly evolving and has yet to achieve ABI stability.

## Driver manager

Driver manager is a binary maintained and developed as part of FDF. It is responsible for
loading drivers and managing devices on all platforms. This is one of the initial process to be started
on device bootup. It finds driver packages in pre-configured paths, tries to match a
driver for every device by running the driver's bind rules, and manages the device lifecycle.
It hosts a virtual filesystem named as Device Filesystem (`devfs`), that provides
uniform access to all devices from userspace services/components external to the drivers. `devfs`
is mounted under `/dev` and contains virtual files that eventually route to interfaces
implemented by the devices.

## Driver host

Driver host is a binary that is launched by driver manager to host one or more drivers. It
facilitates sandboxing of drivers.
