# Errors in FFX subtools

## The FHO `Error` type vs. `ffx_error`

At the subtool boundary, we have an error type defined by `ffx` libraries as
a way to indicate to the main `ffx` tool whether the error was something that
should be reported as a bug or not.

In the older plugin system, this was indicated by using the `ffx_error` or
`ffx_bail` macros to indicate that the error should be reported directly to the
user, with any additional details shunted to the logs. Any other `anyhow` error
was treated as a `BUG` with a big warning and info on where to look in the logs.

Though that still works, and the new error type properly absorbs this
information from `anyhow` errors, it's not really recommended to continue doing
things this way. It's confusing and unclear and when people learn how to write
tools from other tools they often don't see why these macros are being used
the way they are.

## Transforming errors into `fho::Error`

Unlike `anyhow`, fho's error type doesn't try to absorb any error directly.
Instead, you should use methods from the `FfxContext` trait to indicate what
how you want the error to be processed.

If you want the error to be treated as a user error, you can use `user_message`
or `with_user_message` to add the user-visible context to it (while preserving
the error chain for diagnostics):

```rust
process(filename).with_user_message(|| format!("Failed to process {filename}"))?;
```

If, instead, you want it to be treated as a bug because it shouldn't really
happen and the user is unlikely to be able to take any simple action to resolve
it:

```rust
do_thing().bug()?;
```

This forces a useful conscienciousness about how your errors are presented to
the user.

## When to use what error type

It's not necessary, or probably very desirable, to thread this error type
through your entire codebase. It's likely that you will continue to use
`anyhow` or, preferably, `thiserror` in your library code and lower level parts
of your tool. You can then translate these errors into `ffx`-compatible errors.

Keeping a boundary around your library code where
you decide how to present errors. It's a good idea to try to keep everything
that relates to user interaction shallow to this layer and pass what you need
into the deeper layers so you can make better decisions here.

Super-trivial example of that flow:

```rust
let stuff = get_user_input().user_message("Your input made no sense!")?;
stuff.act_on_it().bug()?;
```
