# Life of a handle in FIDL

This page gives a step-by-step explanation of how FIDL transfers a [Zircon
handle] from one process to another. In particular, it focuses on the various
meanings of "handle rights" and how handle rights are validated.

## Scenario

Consider a simple client and server communicating over the following protocol:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/fuchsia.examples.docs/life_of_a_handle.test.fidl" region_tag="protocol" %}
```

Suppose that we remove `zx.rights.WRITE` from the handle rights, but only
recompile the server, not the client. What happens when the client creates a VMO
and passes it to `Method`?

## Diagram

In this scenario, the client is acting as **sender** and the server is acting as
**receiver**. We use those terms below because this is what matters for the
purposes of transferring handles. If the method returned a handle, then the same
steps would apply with the roles reversed.

[See below](#explanation) for a detailed explanation of the diagram.

![Diagram of sending a handle over FIDL](images/life-of-a-handle.svg)

## Explanation {#explanation}

1. **User code (sender)**
    * Assume the sender obtains the VMO using the [`zx_vmo_create`] syscall. The
      returned handle `h1` has default rights for a VMO: `DUPLICATE`,
      `TRANSFER`, `READ`, `WRITE`, `MAP`, `GET_PROPERTY`, and `SET_PROPERTY`.
    * Call `Method`, passing `h1` to it.

1. **FIDL bindings (sender)**
    * Wrap `h1` in a **handle disposition** specifying the rights from the FIDL
      type: `MAP`, `READ`, and `WRITE`. This tells the kernel what rights to
      provide when transferring `h1`. The bindings don't know what rights `h1`
      _actually_ has. (They don't know for sure that it references a VMO either,
      but unlike rights this is usually represented in the static type system,
      making it hard to pass the wrong handle type by accident.)

      ```c
      zx_handle_disposition{
          .operation = ZX_HANDLE_OP_MOVE,
          .handle = h1,
          .type = ZX_OBJ_TYPE_VMO,
          .rights = ZX_RIGHT_MAP | ZX_RIGHT_READ | ZX_RIGHT_WRITE,
      }
      ```

    * Invoke the [`zx_channel_write_etc`] syscall (or similar).

1. **Kernel**
    * Ensure that `h1` exists in the sender process's handle table.
    * Ensure that `h1` refers to a VMO.
    * Ensure that `h1` has (at least) the rights `MAP`, `READ`, and `WRITE`.
    * Restrict the rights to only include `MAP`, `READ`, and `WRITE`, removing
      the rights `DUPLICATE`, `TRANSFER`, `GET_PROPERTY`, and `SET_PROPERTY`.
      We'll refer to this restricted handle as `h2`. This is equivalent to
      invoking the [`zx_handle_replace`] syscall.
    * Enqueue the message with `h2` instead of `h1`.

1. **FIDL bindings (receiver)**
    * Invoke the [`zx_channel_read_etc`] syscall (or similar).
    * Unwrap `h2` from the returned **handle info**. Unlike the handle
      disposition, the handle info stores the handle's _actual_ type and rights
      as reported by the kernel.

      ```c
      zx_handle_info{
          .handle = h2,
          .type = ZX_OBJ_TYPE_VMO,
          .rights = ZX_RIGHT_MAP | ZX_RIGHT_READ | ZX_RIGHT_WRITE,
      }
      ```

    * Get the expected type and rights from the FIDL type: `MAP` and `READ`.
    * Ensure that `h2` has (at least) those rights.
    <!-- TODO(fxbug.dev/89504): In the future, the following point will only
    apply to debug mode. Explain that here once that is the case. -->
    * Since `h2` has the unexpected right `WRITE`, invoke the
      [`zx_handle_replace`] syscall to get a new handle `h3` that only has the
      rights `MAP` and `READ`.

1. **User code (receiver)**
    * Service `Method` using the `h` argument, which is `h3`.

[`zx_channel_read_etc`]: /reference/syscalls/channel_read_etc.md
[`zx_channel_write_etc`]: /reference/syscalls/channel_write_etc.md
[`zx_handle_replace`]: /reference/syscalls/handle_replace.md
[`zx_vmo_create`]: /reference/syscalls/vmo_create.md
[Zircon handle]: /concepts/kernel/handles.md
