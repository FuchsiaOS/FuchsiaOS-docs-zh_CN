
# FIDL internal library zx

The `zx` library is is defined by
[//zircon/vdso/zx_common.fidl](/zircon/vdso/zx_common.fidl). It is included by
GN build rules into invocations of `fidlc` when building fidl code and need not
be explicitly depended upon by every fidl library. If invoking `fidlc` directly,
`zx_common.fidl` would need to be included with a `--files` argument if
necessary.

You can reference this library with the `using` statement:

```fidl
using zx;
```

The types generally correspond to [Zircon System
Types](/docs/concepts/api/system.md#types). For example,
`zx.duration` corresponds to `zx_duration_t`.

> The `CHANNEL_MAX_MSG_BYTES` and `CHANNEL_MAX_MSG_HANDLES`
> are bound at `fidlc` compile time (that is, when the **compiler**
> is compiled) and reflect the constants present at that time.
