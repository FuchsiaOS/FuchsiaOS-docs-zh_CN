A FIDL *resource type* is a type that is intended to transitively carry handles.
Because FIDL handles are unique references to capabilities, any type that
contains one inherits this behavior: it too cannot be copied. In this way,
resourceness is infectious: if a value type becomes a resource type, all types
that transitively include it must do so as well.

<<../examples/key_value_store/_callout.md>>
<<../examples/key_value_store/_add_iterator_tutorial.md>>
