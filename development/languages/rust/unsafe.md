# Unsafe code in Rust


`unsafe` is a dangerous but sometimes necessary escape hatch in Rust.
When writing or reviewing `unsafe` code, it's essential that you:
- clearly identify all of the assumptions and invariants required by every
  `unsafe` block;
- ensure that those assumptions are met;
- ensure that those assumptions will *continue* to be met.

In order to ensure that `unsafe` invariants are not broken by future editors,
each usage of `unsafe` must be accompanied by a clear, concise comment
explaining what assumptions are being made.

Where possible, package up unsafety into a single function or module which
provides a safe abstraction to the outside world. FFI calls should usually
be exposed through a safe function whose only purpose is to provide a safe
wrapper around the function in question. These functions should contain
a comment with the following information (if applicable):
- Preconditions (e.g. what are the valid states of the arguments?)
- Failure handling (e.g. what values should be free'd? forgotten? invalidated?)
- Success handling (e.g. what values are created or consumed?)

Example:

```rust
impl Channel {
    /// Write a message to a channel. Wraps the
    /// [zx_channel_write](https://fuchsia.googlesource.com/zircon/+/master/docs/syscalls/channel_write.md)
    /// syscall.
    pub fn write(&self, bytes: &[u8], handles: &mut Vec<Handle>)
            -> Result<(), Status>
    {
        let opts = 0;
        let n_bytes = try!(usize_into_u32(bytes.len()).map_err(|_| Status::OUT_OF_RANGE));
        let n_handles = try!(usize_into_u32(handles.len()).map_err(|_| Status::OUT_OF_RANGE));

        // Requires that `self` contains a currently valid handle or ZX_HANDLE_INVALID.
        // On success, all of the handles in the handles array have been moved.
        // They must be forgotten and not dropped.
        // On error, all handles are still owned by the current process and can be dropped.
        unsafe {
            let status = sys::zx_channel_write(self.raw_handle(), opts, bytes.as_ptr(), n_bytes,
                handles.as_ptr() as *const sys::zx_handle_t, n_handles);
            ok(status)?;
            // Handles were successfully transferred, forget them on sender side
            handles.set_len(0);
            Ok(())
        }
    }
}
```

If `unsafe` code relies on other safe code for correctness, a comment
must be left alongside the corresponding safe code indicating what invariants
it must uphold and why. Invariants that rely upon the behavior of multiple
functions will draw extra scrutiny, and cross-module or cross-crate unsafety
requires even more attention. `unsafe` code that depends on correct behavior of
a third-party crate will likely be rejected, and `unsafe` code that depends
upon the internal representation details of third-party types will _never_ be
accepted.

Finally, `struct` definitions containing `unsafe` types such as `*const`,
`*mut`, or `UnsafeCell` must include a comment explaining the internal
representation invariants of the type. If the `unsafe` type is used to perform
a mutation OR if it aliases with memory inside another type, there should be
an explanation of how it upholds Rust's "aliasing XOR mutation" requirements.
If any `derive`able traits are purposefully omitted for safety reasons, a
comment must be left to prevent future editors from adding the unsafe impls.

The rules above are applied to any additions of `unsafe` code or any
modifications of existing `unsafe` code.

For more discussion on encapsulating `unsafe` invariants, see
[Ralf Jung's "The Scope of Unsafe"][scope-of-unsafe] and
[Niko Matsakis's "Tootsie Pop" model][tootsie-pop].


[scope-of-unsafe]: https://www.ralfj.de/blog/2016/01/09/the-scope-of-unsafe.html
[tootsie-pop]: http://smallcultfollowing.com/babysteps/blog/2016/05/27/the-tootsie-pop-model-for-unsafe-code
