Certain FIDL types can be made *optional* with no change to the wire shape of
their containing message with the addition of the `:optional` constraint.
Further, the `table` layout is always optional, while the `struct` layout never
is. To make a `struct` optional, it must be wrapped in a `box<T>`, thereby
changing the wire shape of its containing message.

| Base type | Optional version | Does optionality change the wire layout? |
|---|---|---|
| `struct {...}` | `box<struct {...}>` | Yes |
| `table {...}` | `table {...}` | No |
| `union {...}` | `union {...}:optional` | No |
| `vector<T>` | `vector<T>:optional` | No |
| `string` | `string:optional` | No |
| `zx.handle` | `zx.handle:optional` | No |
| `client_end:P` | `client_end:<P, optional>` | No |
| `server_end:P` | `server_end:<P, optional>` | No |

All other types (`bits`, `enum`, `array<T, N>`, and the primitive types) cannot
be made optional.

<<../examples/key_value_store/_callout.md>>
<<../examples/key_value_store/_support_trees_tutorial.md>>
