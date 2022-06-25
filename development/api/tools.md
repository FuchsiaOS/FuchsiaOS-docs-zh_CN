# Developer tool guidelines

This section provides guidelines on *creating* CLI and GUI tools for
Fuchsia.

For information on existing tools, please refer to documentation for those
tools.

## Other topics

- [Command-line tool requirements](cli.md)
    - [CLI --help requirements](cli_help.md)
- GUI Tool requirements (needs writing)

## Packaging a tool for inclusion in the Integrator Development Kit (IDK)

The Integrator Development Kit (IDK) will contain:

  * The tool binary itself.

  * A document in
    [//docs/development/idk/documentation](/docs/development/idk/documentation)
    describing the contract of this tool and how to connect it to the target
    system. The target audience of this document is people writing integration
    scripts rather than being an end-user-friendly “how-to” (debugger example).

## Environment-specific SDKs

The `ffx target list` command abstracts device listing and selection across all
SDK variants. With the right tool design, the extent of integration required
should be to run `ffx target list` to get the address and pass the address to
the tool with other environment-specific flags. In the case of the debugger the
tool-specific code would:

  * Connect to a shell (this should be a primitive provided by the
    environment-specific SDK) on the target and run the `debug_agent`.

  * Run zxdb with the address provided by `ffx target list`, passing any local
    settings files and symbol paths on the command-line.

## Tool requirements

Tools should allow all environment parameters to be passed in via command-line
arguments. Examples include the location of settings files and symbol
locations. This allows different SDKs to be hermetic.

Tools should be written to make writing environment-specific scripts as simple
as possible. For example, the debugger should automatically retry connections
(fxbug.dev/5931) so the current behavior of waiting for the port to be open in
the launch scripts can be removed.

Tool authors are responsible for:

*   Writing the tool with the appropriate interface.
*   Providing documentation on this interface in
    //docs/development/idk/documentation.
*   Currently please reach out to get bugs filed on individual SDKs. We are
    working on a better process for this (fxbug.dev/5908).

