{% set rfcid = "RFC-0035" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

Note: Formerly known as [FTP](../deprecated-ftp-process.md)-035.

## Rejection rationale

Messages without responses (be it events, or fire-and-forget calls) have a
transaction id set to 0, and therefore could not be distinguished using the
proposed scheme.

Some uses leverage [**zx_channel_call()**][zx_channel_call], which assigns
transaction id in the kernel, and waits for a reply.
(This pattern allows concurrent callers to rely on kernel synchronization,
avoiding a user space lock for transaction id assignment.)
Again, the proposed scheme would not be able to distinguish those.

It's expected that kernel tracing support provide the telemetry sought by
this review, and there is a preference to improve this mechanism rather
than push this in FIDL bindings.

Finally, in the SDK dependency pecking order, using FIDL and using FIDL
bindings is very close to the top due to the pervasive use of FIDL on Fuchsia.
Including telemetry and metrics in bindings would therefore raise such
concerns to that order, which is not something we are comfortable with.
Some build and opt-in trickery would be conceivable, and would need to be
part of a future proposal.

## Summary

Adding tracing events to our FIDL bindings enables end-to-end flows across
processes on Fuchsia without hand-rolling custom IDs.

## Motivation

There are an abundance of hacks to enable flow-events across process
boundaries on Fuchsia.
We can automate most of these in a way that doesn't complicate our API
surfaces and requires less manual work.

## Design

A standard attribute for Fuchsia FIDL functions that adds flow begin/end
events to their respectively generated bindings.

The attribute sets the category for the tracing and uses the protocol
function for the name.
Tracing on Fuchsia only supports one category at this time, so while the
attribute could potentially contain **N** categories, we expect only one to be
used and will use the first in the list.

```fidl
protocol Example {
    [Trace = "CATEGORY"]
    ExampleFn(bool test) -> (bool status);
};
```

The unique cross-process ID is the ordinal ID, transaction ID (contained
within every message) and an ID for the transport mechanism (for zircon
channels: the koid of the sending process channel handle, and the related
koid of the receiving process handle) hashed together with a
non-cryptographic hash.

#### Example Stable Trace IDs for FIDL over zx channels

We propose to combine a few identifiers:

*   The **koid of the server end of a FIDL channel**, this can be
    obtained with [**zx_object_get_info()**][zx_object_get_info],
    topic [`ZX_INFO_HANDLE_BASIC`][ZX_INFO_HANDLE_BASIC], using the
    koid on the server side, and the related koid on the client side;
*   The **method ordinal** of the transactional message (note: this is
    [currently a uint32](/docs/contribute/governance/rfcs/0020_interface_ordinal_hashing.md) hashed value, and will soon evolve
    to be a uint64 hashed value, see [RFC-0029](/docs/contribute/governance/rfcs/0029_increasing_method_ordinals.md));
*   Lastly, the **transactional ID** of the [transactional message][wformat-transactional].

How these three identifiers are assembled should strive to reduce possible
trace ID collision, in the following priority:

1. Between two distinct messages, with the **same ordinal**, and between the
   **same client and server**;
2. Between two distinct messages, with **different ordinals**, and between the
   **same client and server**;
3. Between two distinct messages, with **different ordinals**, and between
   **different client and server**.

Currently, koid assignment is mostly sequential.
As a result, the lowest bits of koids will have more entropy than the
highest bits.
Similarly, transaction IDs are sequentially assigned, hence offer more
entropy in the lowest bits.
Method ordinals are cryptographically hashed, and despite the highest bit
being reserved for system usage, it is safe to assume that all bits have
the same entropy.

As a result, a reasonable algorithm given current conditions is to OR:

*   `koid & OxFFFF << 48`
*   `ordinal & 0xFFFFFFFF << 16`
*   `transaction ID & 0xFF << 0`

A trace duration is also started on the receive side of the FIDL bindings.
With languages like C++/Rust this is scoped using RAII and allows the
event to be stitched with another flow event.

## Ergonomics

This makes our tracing system much easier to use, which is a huge win for
our infrastructure as well.

## Documentation and examples

The documentation should be updated to show how to add traces (as outlined
above).

## Backwards compatibility

This change is API compatible and ABI compatible.

## Performance

This will have a small cost when the tracing category is disabled, less
than 5 nanoseconds per dje@google.com testing (on a NUC).
We can also strip the tracing annotations from the IR assuming more
performance is required.

## Security

No Security implications.

## Drawbacks, alternatives, and unknowns

### Alternatives

#### Kernel Trace Mechanisms

Leverage existing ktrace flow events on channel read and channel write.

In the absence of this feature, it would be possible to attempt to
accomplish this by using existing ktrace flow events on the underlying
channel reads and writes.
This is undesirable however, as the channels are common to all FIDL
interfaces, meaning that only one category may be specified.
This means that in order to actually be used, users have to enable the
channel read and channel write categories, meaning that all channel read
and write events (rather than just the ones being used for the FIDL
interface of interest) would be present.
This results in harder to read trace viewer output, unnecessary ktrace
buffer usage, and also relies on FIDL implementation detail.

<!-- xrefs -->
[zx_object_get_info]: /docs/reference/syscalls/object_get_info.md
[ZX_INFO_HANDLE_BASIC]: /docs/reference/syscalls/object_get_info.md#zx_info_handle_basic
[wformat-transactional]: /docs/reference/fidl/language/wire-format/README.md#Transactional-Messages
[zx_channel_call]: /docs/reference/syscalls/channel_call.md
