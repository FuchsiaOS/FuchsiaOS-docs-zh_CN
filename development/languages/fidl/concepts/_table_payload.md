A *table payload* is a FIDL method payload that uses the `table` layout. The
top-level type used as the method payload must use one of `struct`, `table`, or
`union` as its layout. Notably, some generated bindings "flatten" the arguments
passed to `struct` method payloads, such that each member is itself treated as a
function argument in the calling signature. Payloads that use `table` or `union`
never do this, and always pass a single argument, called `payload`, instead.

<<../examples/key_value_store/_callout.md>>
<<../examples/key_value_store/_use_generic_values_tutorial.md>>
