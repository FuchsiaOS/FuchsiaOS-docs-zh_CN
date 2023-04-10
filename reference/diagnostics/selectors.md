# Diagnostics Selectors

[TOC]


## Overview {#overview}

The diagnostics platform within Fuchsia contains multiple services. Each of these services (such
as data exfiltration, metric polling, and error-state analysis) share a common need to describe
the specific properties of their diagnostics data.

We have created a domain specific language (DSL), called Diagnostics Selectors, which provides
the ability to describe diagnostics properties exposed by components. Selectors are designed to
act on diagnostics schemas, and are used whenever:

1. Diagnostics data is encoded in a "data hierarchy” in which named nodes host both child nodes
   and named diagnostics properties.
1. Diagnostics hierarchies are attributable to specific components which expose that data, by
   their monikers.

Diagnostics Selectors have the following high-level syntax:

```
<component_selector>:<hierarchy_path_selector>:<property_selector>
```

The three parts of the syntax above serve to progressively index the Fuchsia diagnostics data
source:

- `component_selector`: Specify the producer of the diagnostics data in terms of the producer's
  [moniker][moniker].
- `hierarchy_path_selector`: Specifies a path through the data hierarchy exposed (by the producer)
  to a specific node of interest.
- `property_selector`: Specifies specific properties of interest on the node you specified in the
  hierarchy.

Some tools that are built on top of the diagnostics platform (such as [Triage][triage] and
[Detect][detect]) need to differentiate between data types with their selectors. These tools extend
the selector DSL with an additional syntax [specifying data type][datatype-fidl]:

```
INSPECT|LOG|LIFECYCLE:<component_selector>:<hierarchy_path_selector>:<property_selector>
```

Each of these parts of the syntax are described in detail below.


### As Files

#### Comments

Diagnostics selectors can be written into cfg files in a flat directory. Within these
files, comments can be written using `//`. For example,

```
// a comment
core/sessions/foo

core2/session:foo  // inline comment
```

## Component selector {#component-selector}

### Syntax {#syntax}

The component selector defines a pattern which describes the relative moniker of one or more
components in the component topology. The component selector is a collection of forward-slash
(`/`) delimited strings describing the path from a root component to the component of interest.

The following component topology is used to demonstrate the component selector syntax:

![A visual tree representation for the selectors explained below][topology-example-img]


Given this topology, consider the following component selector:

```
core/sessions/foo
```

This component selector unambiguously identifies the component named `foo`
relative to its grandparent `core`.

Each segment (the sections delimited by forward slashes) of the component selector
describes exactly one "level” of the component topology.

Component selectors segments can contain only characters that are accepted in a component
moniker. Each component selector segment is an
[instance name or a collection name with an instance name][instance_and_collection_names] which
has a restricted set of allowed characters and length. The colon `:` in between a collection
name and instance name must be escaped with a backslash (<code>\\</code>).


### Wildcarding {#wildcarding}

Component selectors support wildcarding, which will match a single "level" of a component
selector. Consider the below component\_selector, as applied to the above example topology:

```
core/other_comp/*
```
This selector matches all components on the system which are running under the parent named
`other_comp`, which is itself running under a parent `core`. These are their monikers:

- `core/other_comp/foo`
- `core/other_comp/bar`

Wildcards can also be used as string-completion regular-expressions for a single "level” of a
component selector. Consider the following component selector, relative to the above topology
figure:


```
core/*_comp
```

This matches all components on the system that are running under a parent called `core`,
and which names end in `_runner`. These include the following monikers:

- `core/some_comp`
- `core/other_comp`

Component selectors may end in the recursive wildcard `**`, which matches all components
under the given realm:

```
core/**
```

This matches all components on the system running under `core` or any subrealm of `core`.

The component selector `**` alone matches all components on the system.

## Hierarchy path selector {#hierarchy-path-selector}

### Syntax {#syntax}

The hierarchy path selector defines a pattern which describes a path through a structured data
hierarchy, to one or many named nodes. The syntax of this sub-selector is identical to that of
the component selector, since they both describe paths through a tree of named nodes.

Consider the following JSON-encoding of a diagnostics data hierarchy. In this case, the hierarchy
comes from Inspect.

```
"root": {
    "reverser_service": {
        "connection-0x0": {
            "request_count": 1,
        },
        connection_validity: {
            "is_valid": true
        },
        "connection_count": 1,
        "connection_validity": "connection_xyz"

    },
    "version": "part1"
}
```

Note: The example hierarchy contains a node and property with the same
name, `connection_validity`, under the same parent. This situation should
be avoided in practice so that the hierarchy can be represented in JSON
(where child keys are unique). We include this example to illustrate
the difference between node and property selection below.

Given this data hierarchy, consider the following hierarchy path selector:

```
root/reverser_service/connection-0x0
```

This hierarchy path selector unambiguously describes a path from the root of the data hierarchy
to a specific node within the data hierarchy.

Each segment (the sections delimited by forward slashes) of the selector describes exactly one
level, or node, of the data hierarchy. Hierarchy path selector segments may contain any characters,
however if a segment needs to contain asterisk (`*`), forward slashes (`/`),
back slashes (<code>\\</code>), whitespace (tabs `\t` or ` `), or colons (`:`) they must be escaped.

One thing to note that is unique to hierarchy path selectors and not component selectors, is the
case in which a given node shares both a child and property of the same name. Consider the
following selector:

```
root/reverser_service/connection_validity
```

This path hierarchy selector describes the path from root to the
`connection_validity` node. It is completely unrelated to the
`connection_validity` property on the `reverser_service` node, which
can be selected using a [Property Selector](#property_selector):
`root/reverser_service:connection_validity`.

### Wildcarding {#wildcarding}

Hierarchy path selectors support wildcarding, which will match a single "level" of a component
selector. The following example will match all nodes in the data hierarchy which are children of
a node reverser\_service under root:

```
root/reverser_service/*
```

Wildcards can also be used as string-completion regular-expressions for a single "level” of a
component selector. The following example will match all nodes under `reverse_service` that start
with `connection-`.


```
core/reverser_service/connection-*
```


In the example above, the only matching node is `connection-0x0`, but if more connection nodes
existed, they’d match as well.


## Property selector {#property_selector}

### Syntax  {#syntax}

The  property selector is the simplest of all sub-selectors. It defines a pattern to match a
single string, which is a property name on a diagnostics hierarchy. All properties in diagnostics
hierarchies have string names. Omitting the property selector is effectively a
[hierarchy path selector](#hierarchy-path-selector)

Like the previous selector segments, if you wish to match against asterisks (`*`),
forward slashes (`/`), back slashes (<code>\\</code>), whitespace (tabs `\t` or ` `),
or colons (`:`) they must be escaped with a backslash (<code>\\</code>).

### Wildcarding {#wildcarding}

Wildcards can be used to match entire property strings, or can be used as string-completion globs.

```
eg: abc will match any string with the exact name "abc".
eg: a\* will match any string with the exact name "a*".
eg: a\\* will match any that starts with exactly "a\".
eg: a* will match any string that starts with "a".
eg: a*b will match any string that starts with a and ends with b.
eg: a*b*c will match any string that starts with a and ends with c, with b in the middle.
```



## Full Selector Examples {#full-selector-examples}

The following selector will select data from any `echo.cmx` instance on the system that exists in
any realm that itself is under `realm1`. The data retrieved will be the `active_connections`
property on the node at `root/a/b/c`.


```
root/realm1/*/echo.cmx:root/a/b/c:active_connections
```


The following selector will select inspect data from any `echo.cmx` instance on the system that
exists in any realm that itself is directly under `realm1`. The inspect data will be the
`memory_usage property` on the node at `a/b/c/d`.


```
root/realm1/echo.cmx:a/b/c/d:*
```


[datatype-fidl]: https://fuchsia.dev/reference/fidl/fuchsia.diagnostics#DataType
[detect]: /src/diagnostics/config/triage/detect
[instance_and_collection_names]: /docs/reference/components/moniker.md#identifiers
[moniker]: /docs/reference/components/moniker.md
[topology-example-img]: selectors-example.png
[triage]: /src/diagnostics/config/triage
