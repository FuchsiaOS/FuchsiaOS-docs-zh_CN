# FHO subtool interface

FHO is a set of library interfaces that connects `ffx` with subtools, whether
compiled into the `ffx` cli binary or as separate binaries in the build tree or
SDK. It's designed to allow for a ratcheting forward and backwards compatibility,
allowing for binaries built with FHO to communicate with each other over large
spans of versions.

Inside an ffx subtool (a top level command run with an ffx prefix), FHO is
best thought of as a compatibility layer between the program and a device or
virtual machine running Fuchsia, allowing the tool to ignore all the setup
necessary to connect to the device, establish FIDL channels to services on
the device, and log useful information about the interactions.

## FHO host side (the `ffx` CLI)

In the FHO architecture, the host binary (`ffx`) is responsible for discovering
important information about the environment the subtool is going to be run in.
This potentially includes the following details:

* User and project configuration files and their contents.
* Managing the active daemon and the services it provides.
* Discovery and connection to a target device, including and especially
the default device if one is configured.
* Discovery of the locations of subtools within the project, SDK, or both.
* Discovery of the SDK and validating version constraints between the device,
the SDK, and the subtool.

Most tools should be built as external processes, and discovered through the
SDK or build output directory, but some are still built into the `ffx` binary.
These tools are not necessarily very different from those compiled separately,
but they have more limited compatibility constraints. Still, it is useful to
be able to access some tools even without an SDK present.

Most of the rest of this document is about the separately compiled subtools and
how they're discovered and run.

## FHO tool side

In an ideal world, an FHO-based subtool would simply be a program written for
Fuchsia, but run on the host with the contextual information the `ffx` host
binary provided used to connect to services on the device.

However, tools runnable by `ffx` have a special entry point to allow for
conveying the contextual information necessary for the tool to run correctly.
They also need to have some metadata that `ffx` can use to discover,
verify compatibility, and run them.

### Tool metadata

Tools generally have a JSON metadata file associated with them that might
look like this:

```json
{
  "name": "echo",
  "description": "run echo test against the daemon",
  "requires_fho": 0,
  "fho_details": {
    "FhoVersion0": {}
  }
}
```

This JSON file is intended to provide versioning information to `ffx` as well as
the information that will be output when running `ffx help` or `ffx commands`.
The above file is a "version 0" FHO description, which is the most minimal
starting point for the subtool interface.

### FHO metadata versions

FHO versions will be numbered sequentially. The supported versions of any given
subtool can be determined by a range from the `requires_fho` key to the number
at the end of the highest value entry in the `fho_details` map. A subtool should
be able to be run as any version between those two values.

In the above example, the subtool *only* supports FHO Version 0. If the host
tool does not support version 0, it must ignore that instance of the subtool
in favour of a newer version found elsewhere, or error if no matching version
is found at all.

A host tool will be expected to use the next highest (or equal) entry in
`fho_details` than the highest FHO version it supports. This means, for example,
that if you ran an `ffx` version that natively supported version `3` against the
following metadata:


```json
{
  "name": "echo",
  "description": "run echo test against the daemon",
  "requires_fho": 0,
  "fho_details": {
    "FhoVersion2": { "some_thing": "is_something" },
    "FhoVersion4": { "some_thing_else": "is_something_else" },
  }
}
```

It would be expected that the host tool would pull metadata from `FhoVersion4`
and run the program as an `FhoVersion3` program.

This allows for both incremental and breaking changes to metadata over time,
while allowing the tools themselves to support a wide range of versions.
Incremental changes can simply be added to the current version's metadata field,
older versions of the host tool will ignore them and newer ones will use them.

When a breaking change is necessary for the metadata, a new entry altogether can
be made and prior versions will continue to use the older metadata structure.

It is be permissible for a subtool to include an `FhoVersion0` section,
even if `requires_fho` is higher, to indicate that the tool can be invoked in
that simpler mode directly. An `FhoVersion0` section *should not* contain any
keys.

The current FHO Metadata JSON schema can be found
[in the source tree](/src/developer/ffx/lib/fho/metadata/schema/fho_metadata.json).

## Tool discovery

Currently, subtools are searched for in the following places:

* In the FFX binary itself.
* In a project-specific build output location.
* In the configured SDK's metadata.

### Path-based discovery

If `ffx` has been configured with a build output directory or any other locations
to search for subtools, it may search those locations for binaries matching the
following pattern:

`/ffx-([-_a-zA-Z0-9]/)` or `ffx-toolname` where `toolname` is the subcommand
name that it will be invoked as. Note that it does *not* treat further hyphens
as deeper nestings, ie. `ffx-some-sub-tool` is run as `ffx some-sub-tool` and
not `ffx some sub tool`. Subtools are responsible for their own nesting command
layers, though the FHO library itself may provide helpers for doing so.

These files *must* be accompanied by a file with the same name and a `.json`
extension suffix. This file will be the metadata described in earlier sections,
and the details in that must match the binary being run (for example, the `name`
key at the top level must be the same as `toolname` in the binary and metadata
filenames).

Files that match this pattern without an accompanying metadata file
will not be treated as subtools and will be ignored.

### SDK discovery

In the SDK, tools are discovered through metadata search. The
[SDK](/build/sdk/README.md) manifest is searched for `atoms` of type
[`ffx_tool`](/build/sdk/meta/ffx_tool.json), which points at the SDK-relative
location of the metadata and executable binaries.

To run a subcommand, the SDK entries matching the `ffx_tool` type will be
scanned until a matching name is found. As with path-based search, hyphens in
the name do not create deeper nestings of command names.

A host tool *may* treat the main `ffx` `host_tool` binary from the SDK as a
'default' command to defer to if no subtool is found, invoking it as an FHO
version 0 (which essentially invokes the subtool as if it were `ffx` with only
one subcommand).

## Tool interface

As mentioned above, the metadata declares a versioned interface between the `ffx`
binary and the subtool being run. The intention is that this interface will
become more structured over time, but starts off relatively simple to avoid
breaking any expectations currently built into subtools.

Subtools *may* support earlier versions of the FHO interface than they
represent in their metadata, particularly it's likely that some subtools may
need to continue to implement version 0 even well after `ffx` no longer supports
it, depending on their use in things like build scripts.

### FHO Version 0

FHO Version 0 is the first and simplest version of FHO. Tools that only support
version 0 should not be included in the SDK, but tools included in the SDK may
additionally support version 0.

An FHO Version 0 subtool has a very simple invocation process. It is simply run
with exactly the arguments the calling `ffx` was given, including all arguments
to `ffx` itself. That is, if you invoke:

```sh
> ffx --isolate-dir "/tmp/blah" echo stuff
```

The `ffx` binary you run searches for a matching `ffx-echo` binary, and if
their highest common supported version is zero will invoke it as:

```sh
> ffx-echo --isolate-dir "/tmp/blah" echo stuff
```

The subtool must support all top-level `ffx` arguments known of for the version
of `ffx` it was built with. It will also be run with an environment variable
`FFX_BIN_ENV` that points to the filesystem path of the top level invocation of
`ffx` it was run under (though it should not overwrite it), or that can be run
to re-start processing from the same contextual environment the subtool was run
under.

Otherwise, the child process will be given the same `(stdin,stdout,stderr)`
triple and environment variables the original ffx invocation was run under.

The reasons for this being the version 0 interface are:

* It allows for direct invocation of subtools in the build without having built
the primary `ffx` binary yet, removing unnecessary targets from the build's
critical path.
* It doesn't interfere with existing plugins that expect full
control over the input and output environment they're run in.
* It let the first version of this architecture focus on the more important
aspects of building out the capability.
