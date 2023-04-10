# Experimental subtools

It's possible to mark subtools as experimental, and prevent them from running
without opting in to the instability they're expected to have. In the legacy
plugin macros you would do this as an argument to the `ffx_plugin`
proc macro. In the new one you do it with a check attribute on the derive macro
for your tool definition.

## When are tools experimental?

Any new tool should start off as experimental. This allows people to use it where
they need it while it's still evolving, and then once it has settled down and
met criteria for inclusion in the SDK can be unmarked as experimental and used
more widely.

## What do I need to do to unmark my tool as experimental

Approval from the tools team and/or the sdk team should probably be sought
before unmarking a tool as experimental, and this should probably happen before
the new tool is included in the SDK.

The Tools team is still working on specific guidance for how and when to move
something out of experimental status, but some good guidelines would include
doing all the things listed in [the main documentation for subtools](../README.md)
and following any additional guidelines from
[RFC-0169](/contribute/governance/rfcs/0169_sdk_tool_compatibility.md).
