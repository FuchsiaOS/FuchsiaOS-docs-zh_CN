The `bits` type is FIDL's way of representing the [bit
array](https://en.wikipedia.org/wiki/Bit_array). It is used in cases where a
set of boolean flags is desired. The `bits` array is generally used "over" an
underlying subtype, which controls its bitwidth on the wire.

<<../examples/key_value_store/_callout.md>>
<<../examples/key_value_store/_use_generic_values_tutorial.md>>
