# Layer cake deprecation

## Goal & motivation

Once upon a time Fuchsia OS & platform built out of multiple repositories,
designed in a cascade so that changes would flow from the lower-level
repositories to the higher-level repositories. This arrangement was called the
Layer Cake, and was comprised of four layers: Zircon (mostly kernel and
drivers), Garnet (low-level usermode services), Peridot (application-facing
services), and Topaz (application layer).

The distinctions between layers weren't crystal clear, the cost/benefit of this
arrangement wasn't compelling, and so eventually we moved the majority of
Fuchsia OS & platform development into a single repository and established a
new [source code layout][source-code-layout].

However the old model left a lingering mark on our directory structure, as you
can still find top-level directories named after layers.

These holdover-directories only serve to confuse new team members.
Let's move on.

## Technical background

Not much to say here. Move slow and don't break things.

Don't let them eat cake?

## How to help

### Picking a task

The following directories remain in the Fuchsia git repository:

*   `//zircon/`
*   `//garnet/`
*   `//topaz/`

Show them that you mean business by picking one of their subdirectories and
moving it elsewhere - typically to `//src`.

At the time of writing, prime targets for migrations include:

*   `//zircon/system/ulib/`
*   `//zircon/system/utest/`
*   `//garnet/bin/`
*   `//garnet/lib/`

### Doing a task

This mostly involves moving files and renaming some build targets.
9 out of 10 changes will be entirely mechanical.

### Completing a task

Find reviewers via OWNERS, merge your change, and take us one step closer to
deleting layer directories.

## Examples

*   [424156: [fuchsia-cprng] Move to src/lib/zircon/rust](https://fuchsia-review.googlesource.com/c/fuchsia/+/424156)
*   [449714: [build][power] Move power_manager to //src/power](https://fuchsia-review.googlesource.com/c/fuchsia/+/449714)
*   [461096: [thermd][shuffle] Migrate thermd from //garnet to //src](https://fuchsia-review.googlesource.com/c/fuchsia/+/461096)
*   [473957: [sysmem] Move sysmem_connector to src/devices/sysmem/bin](https://fuchsia-review.googlesource.com/c/fuchsia/+/473957)

## Sponsors

Reach out for questions or for status updates:

*   jamesr@google.com

[source-code-layout]: /docs/concepts/source_code/layout.md
