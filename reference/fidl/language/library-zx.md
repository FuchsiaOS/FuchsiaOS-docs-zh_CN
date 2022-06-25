# FIDL internal library zx

The `zx` library is is defined in [//zircon/vdso](/zircon/vdso), and can be
depended on in-tree by adding the `//zircon/vdso/zx` target to a  `fidl`
target's `public_deps`. If invoking `fidlc` directly, `zx_common.fidl` would
need to be included with a `--files` argument if necessary.

You can then reference this library with the `using` statement:

```fidl
using zx;
```

The types generally correspond to [Zircon System
Types](development/api/system.md#types). For example,
`zx.duration` corresponds to `zx_duration_t`.

> The `CHANNEL_MAX_MSG_BYTES` and `CHANNEL_MAX_MSG_HANDLES`
> are bound at `fidlc` compile time (that is, when the **compiler**
> is compiled) and reflect the constants present at that time.
