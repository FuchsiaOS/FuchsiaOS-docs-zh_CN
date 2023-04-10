Events are FIDL calls initiated from the server. Because these calls have no
built-in client-side response, they are not flow controlled: a server could
queue up a very large number of such calls, and flood the client. One solution
to this problem is the *throttled event pattern*. This pattern involves adding a
client-called FIDL method to serve as an acknowledgement point for one or more
events to sync to.

Servers should refrain from sending more of the throttled events (the exact
semantics here are specific to the implementing protocol) until they have
received the next acknowledgement call from the client. Similarly, clients
should close the connection the if servers send more throttled events than
allowed before the client has acknowledged them. These restrictions are not
built into the FIDL runtime, and require some manual implementation on the part
of client/server implementers to ensure correct behavior.

<<../examples/canvas/_callout.md>>
<<../examples/canvas/_client_requested_draw_tutorial.md>>
