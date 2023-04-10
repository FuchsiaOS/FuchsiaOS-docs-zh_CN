*Named payloads* are `struct`, `table`, or `union` types that are used directly
as method request or response payloads. These can be used in cases where a
method payload would otherwise be repeated or is already a named type used
elsewhere in the FIDL file.

In the following example, the added `ReadItem` method is notably different from
the existing `WriteItem`, using an already existing named type as the payload,
rather than a repetitive inline definition.

<<../examples/key_value_store/_callout.md>>
<<../examples/key_value_store/_add_read_item_tutorial.md>>
