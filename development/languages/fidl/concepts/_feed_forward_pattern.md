In FIDL protocols, using two-way methods with empty responses for flow control
purposes (e.g. `DoSomething(...) -> ();`) has a fundamental drawback for latency
sensitive protocols: either callers wait for each reply before sending the next
message, thereby increasing per-message latency, or they ignore it, which
renders the empty reply itself pointless. For protocols that desire flow control
without these latency costs, a good alternative is the *feed forward pattern*.
In this setup, one or more one-way methods write data to the server, while some
other method (one-way or two-way) is used to "commit" the work and synchronize
between the client and server. This means arbitrary amounts of data can be
transferred as fast as they can be sent, but there is still some amount of flow
control, as the synchronization method forces the client to stop before
proceeding with more work.

<<../examples/canvas/_callout.md>>
<<../examples/canvas/_client_requested_draw_tutorial.md>>
