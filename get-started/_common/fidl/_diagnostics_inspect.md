## Using Inspect

Component Inspection enables Fuchsia components to expose structured diagnostic
information about themselves using the Inspect API. Fuchsia provides this
information through the developer tools and bug reports to assist in diagnosing
issues or monitoring performance.

Components expose inspection metrics as a tree of named **Nodes**, each
containing a set of **Properties** as key/value pairs. Properties support a
variety of numeric, string, and array data types. The component inspector
libraries provide an interface to your component's **root node** where you can
attach additional properties of interest to your application.

![Tree diagram showing how component inspection provides structured metrics
data as a tree of "nodes," where each node can contain one or more key/value
"properties."]
(/get-started/images/fidl/component-inspect.png){: width="583"}

You can retrieve the current set of metrics published to Inspect using the
developer tools:

*   `ffx inspect`: Lets you interactively query the Inspect state using
    component selectors. This is helpful for debugging components during
    development.
*   `ffx target snapshot`: Captures a debug snapshot archive of the entire
    system, which contains the Inspect data in JSON format.

```posix-terminal
ffx inspect show core/foo-example
```

```none {:.devsite-disable-click-to-copy}
core/foo-example:
  metadata:
    filename = fuchsia.inspect.Tree
    component_url = fuchsia-pkg://fuchsia.com/foo-example#meta/foo-example.cm
    timestamp = 55457379176
  payload:
    root:
      version = 1.0
      request_metrics:
        request_count = 3
        error = timeout
```

Note: For more details on using the Inspect API, see
[Fuchsia component inspection](/development/diagnostics/inspect).
