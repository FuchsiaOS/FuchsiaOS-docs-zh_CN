# FIFO packets

For more information on how FIFO packets work in the Fuchsia tracing system,
see [Fuchsia tracing system](/docs/concepts/kernel/tracing-system.md).

## Format

FIFO packets are fixed size with the following format:

```cpp
typedef struct trace_provider_packet {
    // One of TRACE_PROVIDER_*.
    uint16_t request;

    // For alignment and future concerns, must be zero.
    uint16_t reserved;

    // Optional data for the request.
    // The contents depend on the request.
    // If unused they must be passed as zero.
    uint32_t data32;
    uint64_t data64;
} trace_provider_packet_t;
```

## Defined packets {#defined-packets}

The following FIFO packets are defined for the Fuchsia tracing system:

### TRACE_PROVIDER_STARTED

This packet is sent from trace providers to the trace manager.
Notifies the trace manager that the provider has received the "start tracing"
request and is starting to collect trace data.
The `data32` field of the packet contains the version number of the FIFO
protocol that the provider is using. The value is specified by
`TRACE_PROVIDER_FIFO_PROTOCOL_VERSION` in
[`//zircon/system/ulib/trace-provider/include/lib/trace-provider/provider.h`](/zircon/system/ulib/trace-provider/include/lib/trace-provider/provider.h).
If the trace manager sees a protocol it doesn't understand, it closes
its side of the FIFO and ignores all trace data from the provider.

### TRACE_PROVIDER_SAVE_BUFFER

Note: This request is only used in streaming mode.

This packet is sent from trace providers to the trace manager in streaming
mode. Notifies the trace manager that a buffer is full and needs saving.
The `data32` field contains the wrap count, which is the number of times
writing has switched from one buffer to the next. The buffer that needs saving
is `(data32 & 1)`. The `data64` field contains the offset of the end of
data written to the durable buffer.

Only one buffer save request may be sent at a time. The next one cannot be
sent until `TRACE_PROVIDER_BUFFER_SAVED` is received acknowledging the
previous request.

### TRACE_PROVIDER_BUFFER_SAVED

This packet is sent from the trace manager to trace providers
in streaming mode. Notifies the trace provider that the requested
buffer has been saved. The `data32` and `data64` fields must have
the same values from the originating `TRACE_PROVIDER_SAVE_BUFFER` request.
