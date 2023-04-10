An *infallible two way method* is a FIDL method that cannot return an error
value. The only possible failure modes are if the underlying
[channel][channel-doc-link] has a failure (like failing to connect to one of
the endopints).

<<../examples/calculator/_callout.md>>
<<../examples/calculator/_baseline_tutorial.md>>

[channel-doc-link]: /docs/reference/kernel_objects/channel.md