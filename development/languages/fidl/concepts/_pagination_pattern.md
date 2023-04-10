When sending a list of items that can potentially get very large, a useful
strategy is to break that list up over multiple calls via the *pagination
pattern*. Using pagination allows for more granular synchronization of work
between the sender and receiver: instead of pummeling the receiving party with a
massive list, the sender emits a few items at a time and waits for feedback that
the message has been processed before continuing.

In FIDL terms, this means that rather than sending a single large `vector<T>`,
FIDL authors should instead convert it to an acked message of `vector<T>:N` to
ensure that the page size and flow control is part of the FIDL contract.

<<../examples/canvas/_callout.md>>
<<../examples/key_value_store/_add_iterator_tutorial.md>>
