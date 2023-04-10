# Owners

Each file in Fuchsia has a set of owners. These are tracked in files
named `OWNERS`. One of these files is present in the root of the
repository, and many directories have their own `OWNERS` files too.

## Contents

Each `OWNERS` file lists a number of individuals (by their email address) who are
familiar with and can provide code reviews for the contents of that directory.

## Responsibilities

Fuchsia requires changes to have an `Code-Review +2` review, which anyone in the
'OWNERS' file can provide. In addition, many `OWNERS` files
contain a `*` allowing anyone to provide such a `+2`.

## Tools

Gerrit has a "find owners" button that will list all the owners for all the
files modified in a given change. More information on this is available on the
[Gerrit find-owners plugin][find-owners] page.

## Format

Fuchsia uses the [Gerrit file syntax][owners-syntax] for `OWNERS`
files, with the addition of a comment indicating the default Monorail component
to use when filing issues related to the contents of this directory.

Here's an example `OWNERS` file:

```none
# These users are owners
validuser1@example.com
validuser2@example.com

# Users listed elsewhere are also owners
include /path/to/another/OWNERS

# This user is only an owner of the listed file
per-file main.c = validuser3@example.com

# The line below helps automate bug triage
# COMPONENT: TopComponent>SubComponent
```

## Best practices

*   It's important to have at least two individuals in an `OWNERS` file. Having areas
    of Fuchsia with a single owner leads to single points of failure. Having multiple
    owners ensures that knowledge and ownership is shared over areas of Fuchsia.
*   When applicable, `include` owners from another file rather than listing individuals.
    This creates fewer "sources of truth" and makes OWNERS maintenance easier.

[find-owners]: https://gerrit.googlesource.com/plugins/find-owners/+/HEAD/src/main/resources/Documentation/about.md
[owners-syntax]: https://gerrit.googlesource.com/plugins/find-owners/+/HEAD/src/main/resources/Documentation/syntax.md
