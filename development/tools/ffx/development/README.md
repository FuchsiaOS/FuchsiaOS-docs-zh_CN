# Developing for FFX

FFX is divided into some areas of responsibility, with core functionality
related to finding and running fuchsia-interacting programs being in the core
program (the [ffx cli](/docs/development/tools/ffx/architecture/cli.md)) and
the actual programs being built as 'subtools' (formerly plugins) that may either
be compiled in with the `ffx` cli, or run as external tools (using the
[FHO subtool interface](/docs/development/tools/ffx/architecture/fho.md)).

In order to add functionality to be accessible through `ffx`, you can write your
own subtools and integrate them into the build.

## Get started with subtools

To get started building a simple subtool (formerly called plugins) for `ffx`, see
[developing an ffx subtool](subtools/getting-started.md).

If you have an existing plugin and want or need to migrate it to the new subtool
interface, take a look at [migrating an existing plugin](subtools/migrating.md).

We also keep a reference implementation of a subtool that does some basic things
called [`ffx-echo`](/src/developer/ffx/tools/echo/src) that should always act as
a canonical source of how to write a subtool.

## New subtool checklist

If this is the start of a larger project, there are some things you should try
to do to make sure your new subtool will work over time:

* Mark your new subtool as [experimental](subtools/experimental.md) to establish
an expectation that it will be changing before stabilization.
* Use a [`Writer`](subtools/writers.md) for all output related to your subtool.
If inclusion in the SDK is expected at some point, make sure that you use a
`MachineWriter` with a concrete serialize-able type and output works correctly
when `--machine json` is passed to `ffx`.
* Properly delineate between user-actionable errors and bugs at the subtool
boundary using the
[`fho::Error` type](/docs/development/tools/ffx/development/subtools/errors.md).
* If the subtool lives in the `ffx` tree, it should have a clear `OWNERS` file
describing both the team that owns it and individuals who can check CLs against
that tool.
* If it lives outside the `ffx` tree, but inside the fuchsia.git repository, it
must have the [`ffx owners file`](/src/developer/ffx/OWNERS) referenced (ie.
`file:/src/developer/ffx/OWNERS` in its OWNERS file) applying to any files at
the api boundary between `ffx` and the subtool so that we can make pro-active
changes to that api boundary over time.

## Legacy plugin references

If you're looking for information about the legacy plugin macro interface, see:

* [Developing an ffx plugin](plugins.md)
* [Experimental plugins](plugin-experimental.md)
* [Plugin internals](plugin-internals.md)
