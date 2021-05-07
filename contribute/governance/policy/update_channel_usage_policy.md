# Fuchsia Update Channel Usage Policy

This document codifies the policy for using *Update Channel* information in
Fuchsia.

## Background

Fuchsia exposes Get/Set channel information through the
_fuchsia.update.channelcontrol_ FIDL APIs. In order to expose these APIs to a
wider set of clients as part of the Fuchsia SDK, it is important to ensure
client behavior is not modulated based on channel strings. Below is our
policy, which applies to internal clients as well as any other Fuchsia
supported components.

## Policy

Clients looking to obtain approvals for use of Update Channel APIs must conform to the
following requirements:

*   The channel information cannot be used in conditional logic
*   The client software must execute exactly the same code path regardless of channel.
*   The channel information cannot be cached or shared in a way that allows
    unapproved consumption.
    *   This implies that additional clients should not by-pass the policy by
        reading cached channel information.

## Update Channel Use within Fuchsia Platform

Within the Fuchsia “stem” the use of channels MUST conform to the defined
policy. Also, the following properties MUST apply:

*   There should be a single component responsible for the writing of channel
    information. The component should export this capability via a FIDL service to
    other clients. Both read/write capabilities should be exported.
*   Readers of channel information should use the canonical APIs of the
    authoritative component and should not use this information to alter runtime
    behavior.

## Update Channel Use within the IDK

Users of channel information in the IDK for Fuchsia components MUST follow the
defined policy. The following properties MUST apply:

*   Clients MUST only read the channel for the purposes of reporting (via a
    metrics agent), information collecting,  or displaying to a front-end
    interface.
*   There MUST be only a single component to set the channel during runtime.
    Additional clients must communicate via this component to set the channel.
    This is currently the Fuchsia Omaha Client.
