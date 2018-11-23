# SDK

This folder contains information about developing the Fuchsia SDK.


## What belongs in an SDK?

By default, a piece of code in the Fuchsia tree cannot be added to any SDK:
participation is a strictly opt-in decision. Additionally, this decision is
encoded locally within the code's build file. This was done for multiple
reasons:
1. developers modifying the code need to be aware of the potential impact on
   external customers as early as possible;
1. publishing that code to an SDK may require extra input from the developers to
   inform the build system about how to properly include that code in an SDK;
1. knowing whether the code may be included in an SDK or not allows the build
   system to perform extra checks on that code to ensure conformity with SDK
   standards.

In order to be made available in SDKs, a piece of code must follow a set of
[standards and guidelines](standards.md).


## Infrastructure

The SDK creation pipeline consists of two pieces:
1. the backend, which uses the build system to generate a tarball containing
   compiled artifacts, source files, and metadata;
1. the frontend, which applies transformations to that tarball and turn into
   e.g. an SDK distribution.

### Backend

The backend really is just a specialized use of the build system. In other
words, running the SDK backend amounts to passing the right set of arguments to
the Fuchsia build system, which in turn produces an archive with a
[set layout](layout.md).
The inner workings of the backend are described [here][backend].

### Frontend

The term frontend is used to describe any process that ingests a Fuchsia SDK
archive and applies transformations to it.

[In the Fuchsia tree][frontends], frontends are used to generate SDK
distributions, e.g. a Bazel-ready workspace.

Frontends may also be used to adapt a Fuchsia SDK archive for consumption in a
particular development environment by for example generating build files for a
given build system. The presence of extensive metadata in the archive itself
allows for this kind of processing.


## Recipes

### Generating an SDK archive

Build packages for SDK definitions are located under `//<layer>/packages/sdk`.
Use the normal build process using this build package, adding an extra GN build
argument: `build_sdk_archives=true`. The resulting archive will be available
under `//out/<build-type>/sdk/archive/<sdk-name>.tar.gz`.

### Adding content to an SDK

The first step is to make that content available to SDKs. This is done by using
a set of templates listed in the [backend documentation][backend].
The next step is to add that content to an existing SDK definition. For a target
`//path/to/my:super_target`, this is accomplished by making the implicit
`//path/to/my:super_target_sdk` target a dependency of the `sdk` target.

### Producing an SDK distribution

This is done by running a frontend. See the [frontend documentation][frontends]
for more details.


[backend]: https://fuchsia.googlesource.com/build/+/master/sdk/README.md
[frontends]: https://fuchsia.googlesource.com/scripts/+/master/sdk/README.md
