The *acknowledgement pattern* is a simple method of flow-control for methods
that would otherwise be one way calls. Rather than leaving the method as a one
way call, it is instead turned into the a two way call with an absent response,
colloquially known as an *ack*. The ack's only reason for existence is to inform
the sended that the message has been received, which the sender can use to make
decisions about how to proceed.

The cost of this acknowledgement is added chatter over the channel. This pattern
can also result in degraded performance if the client waits for the
acknowledgement before proceeding with the next call.

<<../examples/canvas/_callout.md>>
<<../examples/canvas/_add_line_metered_tutorial.md>>
