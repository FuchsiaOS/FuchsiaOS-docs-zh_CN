An *enum* is a FIDL data type that represents a fixed list of possible
constants, like the suits in a deck of playing cards, or the make of car a user
may select from a dropdown menu. This list of values is then mapped over an
underlying integer type, with each value thereof corresponding to one of the
listed members.

In the example below, a FIDL enum is added in a scenario where enums are a
perfect fit: enumerating the possible error values that may be emitted by a
failed method call. The `ReadError` enum has two members: `NOT_FOUND` is used to
indicate that a search key could not be matched during a read attempt, while
`UNKNOWN` serves as a grab-bag error for all cases that cannot be explicitly
described. Note that this enum is marked `flexible`, allowing it to be easily
evolved with new members in the future.

<<../examples/key_value_store/_callout.md>>
<<../examples/key_value_store/_add_read_item_tutorial.md>>
