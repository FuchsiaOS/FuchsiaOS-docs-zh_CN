FIDL vectors and strings may carry a *size constraint*, which specifies a limit
to how many members the type can contain. In the case of vectors this refers to
the number of elements stored in the vector, while for strings it refers to the
[number of bytes] the string contains.

[number of bytes]: /docs/development/api/fidl.md#string_encoding_string_contents_and_length_bounds)

The use of size constraints is strongly encouraged, since it sets an upper bound
on what would otherwise be an unboundedly large type.

<<../examples/key_value_store/_callout.md>>
<<../examples/key_value_store/_add_iterator_tutorial.md>>
