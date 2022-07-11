# Max out pagination

This document describes the best ways to calculate the size both in terms of
bytes and handles of elements as they are added to a vector. This should be
done in order to maximize the number of elements that can be batched at once
while satisfying the kernel caps on channel writes.

Note: Use the [measure-tape](/tools/fidl/measure-tape/) tool to implement the
techniques described below. In the Fuchsia Source Tree, this tool has direct
build integration.

## Summary

To maximize throughput through a channel, it’s common to batch large responses
as multiple vectors of things, for instance by [using a pagination
API][pagination-api]. Since channels are capped at [64K bytes and 64
handles][channel-byte-and-handle-caps], comes the question of how many elements
can be batched in the vector to max out the capacity (and yet, be just under the
byte size and handle count thresholds).

The key reference document for the following is the [FIDL wire
format][fidl-wire-format] specification.

There are various examples that explain the best ways to max out pagination:

* [Bluetooth `WatchPeers` method](#bluetooth-watchpeers-method)
* [Scenic `Enqueue` method](#scenic-enqueue-method)

## Bluetooth `WatchPeers` method {#bluetooth-watchpeers-method}

Consider the [WatchPeers][bts-watch-peers] method of the
`fuchsia.bluetooth.sys.Access` protocol, defined as:

```fidl
WatchPeers() -> (vector<Peer>:MAX updated, vector<bt.PeerId>:MAX removed);
```

First, a request or response is preceded by a header, i.e. a fixed 16 bytes or
`sizeof(fidl_message_header_t)` as [defined here][fidl-message-header-t].

Each vector has a 16 bytes header `sizeof(fidl_vector_t)`, followed by the
content.

Since `bt.PeerId` is a `struct{uint64}` ([defined here][bt-peer-id]) it is a
fixed 8 bytes, and therefore the `removed` vector’s content is the number of
elements *  8 bytes.

Next, we need to estimate the size of `Peer`, which is [defined as a
table][bts-peer]. Tables are essentially a [vector of envelopes][fidl-table-t],
where each envelope then points to the field content. Estimating the size must
be done in two steps:

1. Determine the largest field ordinal used (a.k.a. `max_set_ordinal`)
2. Determine the size of each present field

The size of `Peer` is then the table header -- i.e. `sizeof(fidl_table_t)`, 16
bytes -- plus the largest set ordinal * envelope header (16 bytes) -- i.e.
`max_set_ordinal * sizeof(fidl_envelope_t)` -- plus the total size of the
content, that is, each present field’s content added.

Fields are relatively easy to size, many are primitives or wrappers thereof,
hence result in 8 bytes (due to padding). The `bt.Address` [field][bt-address]
is also 8 bytes since it’s definition reduces to `struct{uint8;
array<uint8>:6}`. The `string` field is a vector of bytes, i.e.
`sizeof(fidl_vector_t) + len(name)`, and padded to the nearest 8 bytes boundary.

## Scenic `Enqueue` method {#scenic-enqueue-method}

Consider the [Enqueue][scenic-enqueue] method of the
`fuchsia.scenic.Session` protocol, defined as:

```fidl
Enqueue(vector<Command>:MAX cmds);
```

A request or response is preceded by a header, i.e. a fixed 16 bytes or
`sizeof(fidl_message_header_t)` from [zircon/fidl.h][fidl-message-header-t].
Then, the vector has a 16 bytes header `sizeof(fidl_vector_t)`, followed by the
content of the vector, which are the actual commands. As a result, before you
account for the size of each individual command, there is a fixed size of 32
bytes.

A [command][scenic-command] is a [union][fidl-wire-format-union] that has a 24
bytes header (i.e. `sizeof(fidl_xunion_t)`) followed by the content, which is 8
bytes aligned.

The size of a `Command` union content depends on the variant selected. This
example uses the `input` variant of type
[`fuchsia.ui.input.Command`][input-command].

The `input` variant (of the scenic command) is itself a union, which adds
another 24 bytes header, followed by the content of that union, such as a
`send_pointer_input` of type
[`SendPointerInputCmd`][input-send-pointer-input-cmd].

The simplified definition of `SendPointerInputCmd` and all transitively
reachable types through this struct is provided below:

```fidl
type SendPointerInputCmd = struct {
    compositor_id uint32;
    pointer_event PointerEvent;
};

type PointerEvent = struct {
    event_time uint64;
    device_id uint32;
    pointer_id uint32;
    type PointerEventType;
    phase PointerEventPhase;
    x float32;
    y float32;
    radius_major float32;
    radius_minor float32;
    buttons uint32;
};

type PointerEventType = flexible enum {
    // members elided
};

type PointerEventPhase = flexible enum {
    // members elided
};
```

Both enums `PointerEventType` and `PointerEventPhase`
[default][fidl-language-enums] to an underlying representation of `uint32`. You
can reduce the sizing of `SendPointerInputCmd` to the struct:

```fidl
struct {
    uint32;   // 4 bytes, total 4
              // 4 bytes (padding due to increase in alignment), total 8
    uint64;   // 8 bytes, total 16
    uint32;   // 4 bytes, total 20
    uint32;   // 4 bytes, total 24
    uint32;   // 4 bytes, total 28
    uint32;   // 4 bytes, total 32
    float32;  // 4 bytes, total 36
    float32;  // 4 bytes, total 40
    float32;  // 4 bytes, total 44
    float32;  // 4 bytes, total 48
    uint32;   // 4 bytes, total 52
};
```

Therefore, the size of the `SendPointerInputCmd` struct is 52 bytes. For more
information on struct sizing calculation, see [The Lost Art of Structure
Packing][lostart].

Now that you have sized all the pieces of a command, you add the total size:

* Header of `fuchsia.ui.scenic.Command`: 24 bytes, i.e `sizeof(fidl_xunion_t)`
* Content with variant `input`:
  * Header of `fuchsia.ui.input.Command`: 24 bytes, i.e `sizeof(fidl_xunion_t)`
  * Content with variant `set_hard_keyboard_delivery`:
    * Struct `SendPointerInputCmd`: 52 bytes
    * Padding to align to 8 bytes: 4 bytes

This results in a total size of 104 bytes.

<!-- xrefs -->
[lostart]: http://www.catb.org/esr/structure-packing/
[pagination-api]: /development/api/fidl.md#pagination

[fidl-wire-format]: /reference/fidl/language/wire-format
[fidl-wire-format-union]: /reference/fidl/language/wire-format#unions
[fidl-language-enums]: /reference/fidl/language/language.md#enums

[channel-byte-and-handle-caps]: https://fuchsia.googlesource.com/fuchsia/+/b7840e772fccb93be4fff73a9cb83f978095eac2/zircon/system/public/zircon/types.h#296
[fidl-message-header-t]:        https://fuchsia.googlesource.com/fuchsia/+/b7840e772fccb93be4fff73a9cb83f978095eac2/zircon/system/public/zircon/fidl.h#358
[fidl-table-t]:                 https://fuchsia.googlesource.com/fuchsia/+/b7840e772fccb93be4fff73a9cb83f978095eac2/zircon/system/public/zircon/fidl.h#328
[bt-peer-id]:                   https://fuchsia.googlesource.com/fuchsia/+/b7840e772fccb93be4fff73a9cb83f978095eac2/sdk/fidl/fuchsia.bluetooth/id.fidl#13
[bt-address]:                   https://fuchsia.googlesource.com/fuchsia/+/b7840e772fccb93be4fff73a9cb83f978095eac2/sdk/fidl/fuchsia.bluetooth/address.fidl#16
[bts-watch-peers]:              https://fuchsia.googlesource.com/fuchsia/+/b7840e772fccb93be4fff73a9cb83f978095eac2/sdk/fidl/fuchsia.bluetooth.sys/access.fidl#100
[bts-peer]:                     https://fuchsia.googlesource.com/fuchsia/+/b7840e772fccb93be4fff73a9cb83f978095eac2/sdk/fidl/fuchsia.bluetooth.sys/peer.fidl#16
[scenic-enqueue]:               https://fuchsia.googlesource.com/fuchsia/+/b7840e772fccb93be4fff73a9cb83f978095eac2/sdk/fidl/fuchsia.ui.scenic/session.fidl#54
[scenic-command]:               https://fuchsia.googlesource.com/fuchsia/+/b7840e772fccb93be4fff73a9cb83f978095eac2/sdk/fidl/fuchsia.ui.scenic/commands.fidl#12
[input-command]:                https://fuchsia.googlesource.com/fuchsia/+/b7840e772fccb93be4fff73a9cb83f978095eac2/sdk/fidl/fuchsia.ui.input/commands.fidl#7
[input-send-pointer-input-cmd]: https://fuchsia.googlesource.com/fuchsia/+/b7840e772fccb93be4fff73a9cb83f978095eac2/sdk/fidl/fuchsia.ui.input/commands.fidl#25
