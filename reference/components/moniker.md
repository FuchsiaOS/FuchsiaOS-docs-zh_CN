# Component monikers

A [component moniker][glossary.moniker] identifies a specific component instance
in the component tree using a topological path.

This section describes the syntax used for displaying monikers to users.

## Identifiers {#identifiers}

### Instance and Collection Names

Parents assign names to each of their children. Dynamically created children
are arranged by their parent into named collections.

Syntax: Each name follows the regex `[-_.a-z0-9]{1,100}`. That is, a string of
1-100 of the following characters: `a-z`, `0-9`, `_`, `.`, `-`.

See the [component manifest reference][cml-reference] for more details.

## Child Monikers {#child}

Represented by the child's collection name (if any) and the child's name
delimited by `:`.

Syntax: `{name}` or `{collection}:{name}`

Examples:

- `carol`
- `support:dan` - The collection `support` with the child `dan`.

## Relative Monikers {#relative}

Represented by the minimal sequence of child monikers encountered when tracing
downwards to the target.

A relative path begins with `.` and alternates path segments (`/`) and
component/collection names. Relative monikers do not support upward traversal
(i.e. `..`) (from child to parent).

Syntax: `./{path from ancestor to target}`

Examples:

- `.` - self - no traversal needed
- `./carol` - a child - traverse down `carol`
- `./carol/sandy` - a grandchild - traverse down `carol` then down `sandy`
- `./support:dan` - a child - traverse down into collection child `support:dan`

## Absolute Monikers {#absolute}

Represented by the absolute path from the root to the component as a sequence of
child monikers.

An absolute path begins with `/` and is followed by downwards traversal path
segments delimited by `/`. There is no trailing `/`.

Syntax: `/{path from root to target}`

Examples:

- `/` - the root itself (it has no name because it has no parent)
- `/alice/support:dan` - from root traverse down `alice` then down `support:dan`
- `/alice/carol` - from root traverse down `alice` then down `carol`

[glossary.moniker]: /docs/glossary/README.md#moniker
[cml-reference]: https://fuchsia.dev/reference/cml
