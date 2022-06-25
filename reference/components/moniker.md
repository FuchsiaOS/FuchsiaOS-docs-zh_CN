# Component monikers

A [component moniker][glossary.moniker] identifies a specific component instance
in the component tree using a topological path.

This section describes the syntax used for displaying monikers to users.

## Identifiers {#identifiers}

### Instance and Collection Names

Parents assign names to each of their children. Dynamically created children
are arranged by their parent into named collections.

Syntax: Each name is a string of 1 to 100 of the following characters:
`a-z`, `0-9`, `_`, `.`, `-`.

See the [component manifest reference][cml-reference] for more details.

### Instance Identifiers

Instance identifiers ensure the uniqueness of monikers over time whenever a
parent destroys a component instance and creates a new one with the same name.

Syntax: Decimal formatted 32-bit unsigned integer using characters: `0-9`.

## Child Monikers {#child}

Represented by the child's collection name (if any), name, and instance
identifier delimited by `:`.

Syntax: `{name}:{id}` or `{collection}:{name}:{id}`

The following diagram shows an example component topology,
with the children of `alice` labeled with their child monikers.

<br>![Diagram of Child Monikers](images/monikers_child.png)<br>

Examples:

- `carol:0`: child `carol` (instance ID `0`)
- `support:dan:1`: child `dan` (instance ID `1`) in collection `support`

## Relative Monikers {#relative}

Represented by the minimal sequence of child monikers encountered when tracing
upwards from a source to the common ancestor of the source and target and then
downwards to the target.

A relative path begins with `.` and is followed by path segments. ` \ ` denotes
an upwards traversal segment. `/` denotes a downwards traversal segment. There
is no trailing ` \ ` or `/`.

Relative monikers are invertible; a path from source to target can be
transformed into a path from target to the source because information about
both endpoints are fully encoded by the representation.

In contrast, file system paths are not invertible because they use `..`
to denote upwards traversal so some inverse traversal information is missing.

To maintain invertibility, the syntax for denoting paths varies slightly
for upward and downward traversals. A downward path segment is a child moniker
of one of the current component instance's children: `./carol:2`. Conversely,
an upward path segment *is* the child moniker of one of the current component
instances, according to its parent: `.\alice:2/bob:0`.
The reason that the child moniker is explicitly specified in the upward path
(instead of a generic "upward traversal" marker like `..`) is that otherwise the
relative moniker would not be invertible, and would not uniquely identify a
component instance. For downward traversals, the paths don't need to include
the parent's name to be traceable because a child only has *one* parent.
However, for upward traversals the source path can be one of many children of
its parent path.

Syntax: `.\{path from source to ancestor}/{path from ancestor to target}`

The following diagram shows an example component topology, with all relative
monikers that can be derived from the source component `alice` labeled. Note
that `support` is not a component but rather a collection with two
children: `dan` and `jan`.

<br>![Diagram of Relative Monikers](images/monikers_relative.png)<br>

Examples:

- `.`: self - no traversal needed
- `./carol:0`: a child - traverse down `carol:0`
- `./carol:0/sandy:0`: a grandchild - traverse down `carol:0` then down `sandy:0`
- `.\alice:0/bob:0`: a cousin - traverse up `alice:0` then down `bob:0`
- `./support:dan:1`: a child - traverse down into collection child `support:dan:1`

## Absolute Monikers {#absolute}

Represented by the absolute path from the root to the component instance as
a sequence of child monikers.

An absolute path begins with `/` and is followed by downwards traversal path
segments delimited by `/`. There is no trailing `/`.

Syntax: `/{path from root to target}`

The following diagram shows an example component topology, all absolute
monikers that can be derived from the unnamed root component labeled. The root
component is unnamed because it is inherently not the child of any other
component and components are named by their parents, not by components
themselves. Note that `support` is not a component but rather a collection with
two children: `dan` and `jan`.

<br>![Diagram of Absolute Monikers](images/monikers_absolute.png)<br>

Examples:

- `/`: the root itself (it has no name because it has no parent)
- `/alice:0/support:dan:0`: from root traverse down `alice:0` then down `support:dan:0`
- `/alice:0/carol:0`: from root traverse down `alice:0` then down `carol:0`

[glossary.moniker]: glossary/README.md#moniker
[cml-reference]: https://fuchsia.dev/reference/cml
