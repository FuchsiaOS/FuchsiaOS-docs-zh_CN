A common aspect of using FIDL on Fuchsia is passing protocols themselves across
protocols. More precisely, many messages include either the client end or
the server end of a channel, where the channel is used to communicate over a
specific protocol. In this case, client end means that the remote end of the
channel implements the specified protocol, whereas server end means that the
remote end is making requests for the specified protocol. An alternate set of
terms for client end and server end are protocol and protocol request, respectively.

This tutorial covers:

* The usage of these client and server ends, both in FIDL and in the HLCPP
  FIDL bindings.
* The request pipelining pattern and its benefits.
