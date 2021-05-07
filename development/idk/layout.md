# Integrator Development Kit (IDK) layout

The Integrator Development Kit (IDK) archive contains the
Fuchsia-specific libraries, FIDL interfaces, and tools required to start building and running
programs for Fuchsia.

This IDK is the public definition of the Fuchsia platform. It is intentionally
independent from specific build environment and targets integration and engprod teams
integrating Fuchsia into specific developer environments.
For example, it does not contain any build system, favor any
toolchain, or provide standard non-Fuchsia libraries such as for crypto or
graphics.
Instead, it provides metadata accurately describing its various
parts, so that the IDK is processed and augmented with specific tooling
and support libraries to create the end-to-end development experience.

Most developers who wish to build something for Fuchsia should not need to
deal directly with the IDK.
They will instead consume a transformed version of it, for instance within the
development environment and ecosystem supporting a given language runtime.
Maintainers of development environments who wish to add support for Fuchsia are
the main audience for the IDK.
See [Integrating the Core SDK](integrating.md) for a description of how to process this
IDK.

As such, the Fuchsia IDK is the representation of the Fuchsia platform developers'
contract with other developers who work with Fuchsia.
While that contract is absolutely necessary, as this IDK contains the very bits
that are unique to Fuchsia, it is not sufficient and will be complemented by
other "contracts".
The Fuchsia IDK is mirroring the Fuchsia platform in that respect: highly
composable and extensible, with a clear separation of concerns.


## Structure

From this point on, the root of the IDK archive will be referred to as `//`.

### Metadata

Metadata is present throughout this IDK in the form of JSON files.
Every element in this IDK has its own metadata file: for example, a FIDL library
`//fidl/fuchsia.foobar` has its metadata encoded in
`//fidl/fuchsia.foobar/meta.json`.

Every metadata file follows a JSON schema available under `//meta/schemas`: for
example, a FIDL library's metadata file conforms to
`//meta/schemas/fidl_library.json`.
Schemas act as the documentation for the metadata and may be used to facilitate
the IDK ingestion process. See [understanding metadata](understanding_metadata.md).

### Documentation

General documentation is available under `//docs` in the IDK distribution, or
 online at [fuchsia.dev/fuchsia-src/docs/development/idk](/docs/development/idk).
Some individual IDK elements will also provide documentation directly under the
path where they are hosted in the IDK.

### Target prebuilts

Target prebuilts are hosted under `//arch/<architecture>`.
This includes a full-fledged sysroot for each available architecture.

### Source libraries

The IDK contains sources for a large number of FIDL libraries (under
`//fidl`) as well as a few C/C++ libraries (under `//pkg`). See [compiling C/C++](documentation/compilation.md)
for details.

### Host tools

Multiple host-side tools can be found under `//tools`.
This includes tools for building programs, deploying to a device, debugging,
etc...
Some information about how to use these tools can be found under `//docs`.
Specifically:

* [bootserver](documentation/bootserver.md)
* [zxdb](documentation/debugger.md)
* [device-finder](documentation/device_discovery.md)
* [ssh](documentation/ssh.md)
* [logging and symbolizer](documentation/logging.md)
* [package manager](documentation/packages.md)

### Images

`//device` contains metadata describing device configurations matching a given
version of the IDK.
This metadata contains pointers to images that can be paved onto said devices.
See [working with devices](documentation/devices.md) for how to interact with a device
running Fuchsia.
