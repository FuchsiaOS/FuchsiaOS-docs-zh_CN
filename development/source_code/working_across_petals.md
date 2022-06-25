# Make changes across different petals

Because it is not possible to atomically commit a change to multiple Git
repositories at once, developers must carefully coordinate changes that
affect multiple petals.

For example, an API or ABI change in the Fuchsia tree that affects callers
in Topaz or Experiences requires either a
[soft transition](#soft-transitions) (preferred) or [hard transition](#hard-transitions).

## Terminology

*  `D` - A project used in the Fuchsia tree.
*  `P` - Another project used in the Fuchsia tree with a direct dependency on `D`.
For example, `D` might be Fuchsia, and `P` might be Topaz or Experiences.
*  *integration* - The internal integration repository.

## Soft transitions {#soft-transitions}

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

Generally, updates to FIDL protocols can be achieved via a soft transition.

## Hard transitions {#hard-transitions}

For some changes, creating a soft transition can be difficult or impossible. For
those changes, you can make a *hard transition*. In a hard transition, you make
a breaking change to `D` and update `P` manually.

Note that to prevent accidental clobbering of the manifest contents, Gerrit is
configured to not automatically rebase changes that edit a manifest file. You
must manually rebase before merging so that your submit is a pure fast-forward.

Making a hard transition is more stressful than making a soft transition because
your change will be preventing other changes in `D` from becoming available in
dependent projects between steps 1 and 2.

Only Google developers can make hard transitions. See internal documentation for
instructions.

[getting-source]: /docs/get-started/get_fuchsia_source.md "Getting source"
