# Tracing provider buffering modes

In tracing, the buffering mode is the behavior of a trace provider when
it fills its buffer. If a trace provider's buffer fills while a trace is
running, you may get an incomplete trace.

The behavior of each trace provider is independent of other trace providers.
If one trace provider fills its buffer, other trace providers can still
continue to record trace events into their own buffers until the trace stops.

There are three buffering modes:

* [Oneshot](#oneshot)
* [Circular](#circular)
* [Streaming](#streaming)

For information on the `BufferingMode` for the `fuchsia.tracing.provider`
FIDL, see
[`BufferingMode`](https://fuchsia.dev/reference/fidl/fuchsia.tracing.provider#BufferingMode).

## Oneshot {#oneshot}

In this buffer mode, there is a single durable buffer. If the buffer of the
tracing provider becomes full, then that trace provider stops recording events.

## Circular {#circular}

In this buffer mode, the trace buffer is divided into three pieces, the durable
buffer and two rolling buffers. The durable buffer is for records important
enough that they should not be lost. These include records for thread and
string references.

When you start a trace, the tracing provider writes data to the first rolling
buffer. Once one rolling buffer fills, the tracing provider writes data to
the other rolling buffer.

If the durable buffer fills, then tracing for the tracing provider stops.
This behavior does not affect other tracing providers.

## Streaming {#streaming}

In this buffer mode, the trace buffer is divided into three pieces, the durable
buffer and two rolling buffers. The durable buffer is for records important
enough that they should not be lost. These include records for thread and
string references.

When you start a trace, the tracing provider writes data to the first rolling
buffer. Once one rolling buffer fills, the tracing provider notifies the
trace manager that one rolling buffer is full and writes data to the other
rolling buffer. If the other rolling buffer is not available, then data is
dropped until the rolling buffer becomes available. A rolling buffer becomes
unavailable between the point when it is filled and when the trace manager
reports back that the buffer's contents have been saved.

The possibility of dropped data depends on the rate at which the tracing
provider creates records and the rate at which the trace manager can save
the buffers. Dropped data can result in a partially incomplete trace, but
in most cases this is more desirable than affecting program performance by
waiting for a buffer to be saved.

If the durable buffer fills, then tracing for the tracing provider stops.
This behavior does not affect other tracing providers.

