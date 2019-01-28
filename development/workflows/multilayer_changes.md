<!--# Working on multiple projects-->
# 处理多个项目

## Switching between projects

When you bootstrapped your development environment (see
[getting source][getting-source]), you selected a project. Your development
environment views that project at the latest revision and views the dependencies
at specific revisions in the past.

If you want to switch to working on a different project, either to get the source
code for higher projects in your source tree or to see lower projects at more recent
revisions, you have two choices:

1. You can bootstrap a new development environment for that project using
   [the same instructions you used originally][getting-source].
2. You can modify your existing development environment using the
   `fx set-petal <project>` command. This command edits the `jiri` metadata for
   your source tree to refer to the new project and prints instructions for how to
   actually get the source and build the newly configured project.

## Changes that span projects

Fuchsia is divided into a number of [projects][layers]. Each project views the
previous projects at pinned revisions, which means changes that land in one
project are not immediately visible to the upper projects.

When making a change that spans projects, you need to think about when the
different projects will see the different parts of you change. For example,
suppose you want to change an interface in Zircon and affects clients in Garnet.
When you land your change in Zircon, people building Garnet will not see your
change immediately. Instead, they will start seeing your change once Garnet
updates its revision pin for Zircon.

## Hard and Soft Transitions

This section outlines how to make the breaking changes mentioned above.

### Terminology:

* *D* - A project used in the Fuchsia tree.
* *P* - Another project used in the Fuchsia tree with a direct dependency on `D`.
For example, `D` might be Zircon, and `P` might be Garnet.
* *integration* - The internal integration repository.

### Soft transitions (preferred)

The preferred way to make changes that span multiple projects is to use a
*soft transition*. In a soft transition, you make a change to `D` in such a
way that the interface supports both old and new clients. For example, if you
are replacing a function, you might add the new version and turn the old
function into a wrapper for the new function.

Use the following steps to land a soft transition:

1. Land the change in `D` that introduces the new interface without breaking
   the old interface used by `P`.
1. Wait for the new revision of `D` to roll into the integration repository.
1. Migrate `P` to use the new interface.
1. Wait for the new revision of `P` to roll into the integration repository.
1. Land a cleanup change in `D` to remove the old interface.

### Hard transitions

For some changes, creating a soft transition can be difficult or impossible. For
those changes, you can make a *hard transition*. In a hard transition, you make
a breaking change to `D` and update `P` manually.

Note that to prevent accidental clobbering of the manifest contents, Gerrit is
configured to not automatically rebase changes that edit a manifest file. You
must manually rebase before merging so that your submit is a pure fast-forward.

Making a hard transition is more stressful than making a soft transition because
your change will be preventing other changes in 'D' from becoming available in
dependent projects between steps 1 and 2.

Only Google developers can make hard transitions.  See internal documentation for
instructions.

[getting-source]: /development/source_code/README.md "Getting source"
[layers]: /development/source_code/layers.md "Layers"
