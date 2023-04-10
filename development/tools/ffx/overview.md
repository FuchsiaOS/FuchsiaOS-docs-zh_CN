# The ffx tool

*ffx* provides a unified platform for Fuchsia CLI tools for host target
interactions. It introduces a service oriented interface to many common
development and integration workflow operations that users may wish to
perform against one or more Fuchsia target devices.

It is both a service runtime, and a collection of utilities, and it is
intended for both users and infrastructure integrators alike.

## Getting started for users

Review the user guide for [using the ffx command line tool.](/docs/development/tools/ffx/getting-started.md)

## Getting started for developers

When extending or adding a new plugin for `ffx` it is important to
consider the following:

 - `ffx` is developed in Rust and makes heavy use of rust crates. However,
    crates must be hosted in the [Fuchsia Platform Source Tree](https://fuchsia.googlesource.com).
    This process is detailed in Open Source Review Board (OSRB)
    [process](/docs/contribute/governance/policy/osrb-process.md) document.
    Review existing crates in [`third_party/rust_crates`](/third_party/rust_crates).
 - When extending `ffx`, review the existing command surface by running
   `ffx help` to understand where the new command or tool may fit.
 - When extending an existing command, consider adding a flag or an option.
   However, if the overall workflow enabled does not exist, consider a
   new command or a higher level subgrouping.
 - Consider the dependencies such as which FIDL services, local filesystem
   entries, target filesystem paths, or any manifests/manifest formats
   the tool depends on.
 - Consider how the command interacts with multiple devices. `ffx` provides
   a global `--target` flag that can be passed multiple times to execute
   commands across multiple targets in parallel.
 - Does the command need access to configurations or depend on a specific
   build environment? If so, take advantage of the various configuration
   settings within `ffx` or define new ones for the specific command or
   workflow. Existing configurations can be accessed via `ffx config get`.

A detailed guide with examples is available in [developing for ffx.](/docs/development/tools/ffx/development/plugins.md)

## [CLI](/docs/development/tools/ffx/architecture/cli.md)

The Command Line Interface (CLI) provides the UX for ffx. It is responsible
for:

- Parsing user parameters (CLI Parameters)
- Communicating with the daemon (starting it if necessary)
- Routing parsed parameters and requested FIDL proxies to the proper code path
  for execution

## Daemon

The daemon runs in the background on the host device and manages:

- Target discovery
- Target life cycle management (flashing, provisioning, and package serving)
- Facilitating communication with target devices

## Remote control service

The remote control service runs on target devices and is responsible
for providing access to the FIDL services running on the target.