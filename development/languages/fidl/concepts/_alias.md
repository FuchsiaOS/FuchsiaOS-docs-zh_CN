An `alias` is a FIDL declaration that assigns a new name to an existing type.
This has several benefits:

- Using `alias` ensures that there is a single source of truth for the concept
  the aliased type represents.
- It provides a way to name things, especially constrained types.
- Disparate uses of the now-aliased type may be linked as being instances of the
  same concept.

It is important to note that aliases do not carry through to the generated
bindings code at the moment. In other words, the name assigned to an `alias`
declaration will never appear as a declaration name in the generated FIDL code.

In this example, adding an `alias` for `Key` allows us to avoid repetition with
a bespoke name, while also making clear to the reader that both the `key` value
on the `Item` type and the `key` used in the `ReadItem` request struct are
purposefully, and not merely coincidentally, the same thing.

<<../examples/key_value_store/_callout.md>>
<<../examples/key_value_store/_add_read_item_tutorial.md>>
