An *anonymous type* is a type whose definition is located inline with its use,
rather than in a standalone, named `type` declaration. There are two benefits to
using anonymous types. First, they prevent excessive namespace pollution,
absolving FIDL authors of the need to name types that are only used once.
Second, they prevent the type from being imported into another FIDL library via
the `using` declaration, as the type cannot be identified by name.

<<../examples/key_value_store/_callout.md>>
<<../examples/key_value_store/_support_trees_tutorial.md>>
