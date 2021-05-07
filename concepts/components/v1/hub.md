# Hub (Components v1)

<<../_v1_banner.md>>

## What do we mean by hub?

The hub is a portal for introspection.  It enables tools to access detailed
structural information about realms and component instances at runtime,
such as their names, job and process ids, and published services.

## Organization

The hub is organized as a tree of directories and files that describe
individual realms and component instances at runtime.

The hub’s structure is mostly **read-only**.  It is not possible to
create, rename, delete, or otherwise modify directories and files that
form the structure of the hub itself.  However, the **outgoing**
directories of component instances may include mutable directories,
files, and services that clients can access via the hub.

The hub's structure is **observable**.  Clients can watch the filesystem
to observe changes such as realms being created or destroyed as indicated
by directories being added or removed.

The hub's structure is **scope constrained**.  Successively deeper levels
of the hub's directory tree are scoped to successively more specific objects.
For example, having opened a directory representing a realm, a client
can obtain information about the realm itself, its child realms, and
its component instances, but it cannot obtain any information about
the realm's parent.  This structure makes it easier to constrain the
parts of the hub particular clients can access.

## Schema

_Note: this document is describing the hub provided by `appmgr`. Hub directories
provided by component manager have a different schema._

The hub is organized as follows:

**\<realm name\>/\<realm id\>/**: realm directory
> A read-only directory containing information about a realm.  The root realm’s
> realm directory is typically mounted at /hub in the root development shell.

**\<realm id\>/name**: realm name
> A read-only file containing the name of the realm, in UTF-8 without padding or
> terminators.

**\<realm id\>/job-id**: realm’s job id
> A read-only file containing the koid of the realm’s job, in decimal ASCII
> without padding or terminators.

**\<realm id\>/svc**: realm’s services
> Contains all the services that are available in this realm. ls command
> will only show the services that were directly created in this realm.

**\<realm id\>/r/**: child realm list
> A read-only directory containing a list of child realms.

**\<realm id\>/r/\<child realm name\>/\<child realm id\>/**: child realm directory
> A read-only directory containing information about a child realm.

**\<realm id\>/c/**: component instance list
> A read-only directory containing a list of component instances.

**\<realm id\>/c/\<component name\>/\<component instance id\>/**: component instance directory
> A read-only directory containing information about a component.

**\<component instance id\>/name**: component’s short name
> A read-only file containing just the name of the component, in UTF-8 without
> padding or terminators.

**\<component instance id\>/args**: component’s original command-line arguments
> A read-only file containing the component’s original command-line arguments,
> in UTF-8 without padding or terminators.

**\<component instance id\>/url**: component’s url
> A read-only file containing the component’s url.

**\<component instance id\>/job-id**: component’s job id
> A read-only file containing the koid of the component’s job, in decimal ASCII
> without padding or terminators. Multiple component instances may coexist
> within the same job. Components may also create new jobs of their own, which
> are not reflected here.

**\<component instance id\>/process-id**: component’s process id
> A read-only file containing the koid of the component’s process, in decimal
> ASCII without padding or terminators. Multiple component instances may
> coexist within the same process. Components may also create new processes of
> their own, which are not reflected here.

**\<component instance id\>/system\_objects**: system-level component inspection
> A directory tree exposing objects conforming to the [Inspect API](/docs/reference/diagnostics/inspect/README.md).
> This directory tree is managed by the system to expose system-level
> information about the components.

**\<component instance id\>/in/**: component's incoming namespace
> A directory tree exposing objects that have been offered to the component by
> its parent or are ambiently offered by the Component Framework.

**\<component instance id\>/in/svc**: component's incoming services directory
> A directory containing the services that are available to the component
> (either from its parent or from the Component Framework).
>
> This maps to `/svc` in the component's own namespace.

**\<component instance id\>/out/**: component’s out directory
> A directory containing objects that the component has exported, such as its
> services.  May be absent if the component exports nothing.  May contain
> read-write objects.

**\<component instance id\>/out/svc**: component’s exported public object directory
> A directory containing objects that the component has exported to its host,
> such as its services.  May contain read-write objects.

**\<component instance id\>/out/ctrl**: component’s exported control object directory
> A directory containing objects that the component has offered to the realm
> manager for lifecycle control.  May contain read-write objects.

**\<component instance id\>/out/debug**: component’s exported debug object directory
> A directory containing objects that the component has published for debugging
> purposes, such as introspection files and services.  May contain read-write
> objects.

**\<component instance id\>/out/diagnostics**: component’s diagnostics data
> A directory tree exposing objects conforming to the [Inspect API](/docs/development/diagnostics/inspect/README.md).
> This directory tree is exposed by the component itself to allow inspection
> of component-specific data.

**\<component instance id\>/c/**: sub component instance list
> A read-only directory containing a list of sub component instances. This is
> only generated for runner components.
