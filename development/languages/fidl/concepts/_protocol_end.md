A *protocol end* represents one endpoint of a channel connection over which some
specified FIDL protocol is spoken. The server side of this connection is the
`server_end`, while the client side is the `client_end`.

Protocol ends have a required constraint, specifying the FIDL protocol being
spoken over the connection. For instance, `client_end:Foo` represents the client
endpoint of a Zircon channel over which all messages exchanged will conform to
methods and events defined in that FIDL protocol, while `server_end:Foo`
represents the opposite endpoint.

<<../examples/key_value_store/_callout.md>>
<<../examples/key_value_store/_add_iterator_tutorial.md>>
