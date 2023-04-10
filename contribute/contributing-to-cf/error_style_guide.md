# Component Manager Error Style Guide

## Motivation

Above all errors should be useful. That is their only purpose for existence,
otherwise we would silently fail and leave everyone to wonder what happened.

Errors should provide the information their users need to take appropriate
action. Sometimes that user will be a `match` in code, sometimes it will be a
human looking at a log buffer. Keeping the user in mind is key to making
something useful for them!

Simply put, if you follow the principles below:

*   Errors will become more meaningful
*   Errors will become easier to handle
*   Component manager's codebase will become easier to read
*   More bugs will be avoided
*   Bugs which trigger errors will be easier to fix

## Principles

The principles below are only guidelines, albeit strong ones, about how to make
meaningful and useful errors. If you think there is a scenario where these
guidelines are wrong, use your best judgment, oh, and update this doc.

### <span class="compare-worse">DO NOT</span> use `panic!`, `assert!`, `unwrap(...)` and `expect(...)` carelessly

Component manager is not supposed to crash. If component manager crashes, the
system will reboot and user data can be lost.

There are **very few situations** where crashing is acceptable:

*   Asserting on internal invariants may be appropriate, if violating the
    invariant breaks fundamental assumptions about component manager's behavior
    *   This is potentially unsafe and can expose user data or allow an attacker
        to persist their exploit.
    *   Examples: Component state
        *   [We assert that a component whose child is being destroyed cannot be
            in a New or Unresolved
            state.](https://cs.opensource.google/fuchsia/fuchsia/+/main:src/sys/component_manager/src/model/actions/destroy_child.rs;l=56;drc=108abfa498635ec999f90fc948ec8c4fee89fe1a)
        *   [We assert that a URL scheme was not registered to multiple
            component
            resolvers.](https://cs.opensource.google/fuchsia/fuchsia/+/main:src/sys/component_manager/src/model/resolver.rs;l=50;drc=86305b9bc9cf348f369ac96b566b0153e65182fc)
    *   Examples: Argument validation
        *   [We assert that a static child cannot be given numbered handles or
            dynamic
            offers.](https://cs.opensource.google/fuchsia/fuchsia/+/main:src/sys/component_manager/src/model/component.rs;l=1712;drc=a8e7f1423f5c85ee5cd8d53811e0c92429f46c16)
    *   Examples: Expectations from the rest of the system
        *   [We assert that capabilities like the Hypervisor have the correct
            zircon properties on their
            handles.](https://cs.opensource.google/fuchsia/fuchsia/+/master:src/sys/component_manager/src/builtin/hypervisor_resource.rs;l=109;drc=b986e9b78c456bdde653ee65e73590fe9764dc7b)
        *   [We assert that creating a Zircon DebugLog object should not fail.](https://cs.opensource.google/fuchsia/fuchsia/+/master:src/sys/component_manager/src/builtin_environment.rs;l=565;drc=27c2fc281f0f0dadd9ae2bd113aa49f0f4b5b566)
    *   Examples: Component manager configuration
        *   [We assert that the BootFs filesystem can be initialized.](https://cs.opensource.google/fuchsia/fuchsia/+/master:src/sys/component_manager/src/main.rs;l=104;drc=94c00669e0dcf62e475bad2bb10a1121c538f485)
        *   [We assert that the runtime configuration of component manager was
            successfully loaded from
            BootFs.](https://cs.opensource.google/fuchsia/fuchsia/+/master:src/sys/component_manager/src/main.rs;l=118;drc=94c00669e0dcf62e475bad2bb10a1121c538f485)
*   <code>[fidl::endpoints::create_*](https://cs.opensource.google/fuchsia/fuchsia/+/main:src/lib/fidl/rust/fidl/src/endpoints.rs;l=396;drc=7dd24e78b88fa334fcd31463e97e59147990f759)</code>
    methods are safe to unwrap. They create zircon channels and wait for packets
    on those created channels.
*   Asserting on already-verified invariants is acceptable if it results in
    cleaner code.
    *   Example: Checking that `map.contains_key(key)` is true and then calling
        `map.get(key).unwrap()` or `map[key]`.
    *   When possible, prefer to rewrite the code using `if let` or `match` so
        that the unwrap/panic is not necessary.

Warning: Do not unwrap the channel-to-stream conversion method `into_stream()`
when dealing with channels from untrusted sources. Converting to a stream
implicitly causes
[the executor to wait for the channel to close](https://cs.opensource.google/fuchsia/fuchsia/+/main:src/lib/fuchsia-async/src/handle/zircon/rwhandle.rs;l=80;drc=7dd24e78b88fa334fcd31463e97e59147990f759)
which requires the `ZX_RIGHT_WAIT` permission on the channel.

Warning: Do not panic or unwrap when dealing with input/handles from untrusted
sources (components and their processes).

Note: If you absolutely have to crash component manager, prefer `expect()`
instead of `unwrap()` and provide a lot of detail in the error message.

Note: It is acceptable to panic in tests for component manager.

### <span class="compare-worse">DO NOT</span> use `println!`

While `println!` integrates with debuglog in component manager, the
<code>[tracing log library](https://docs.rs/tracing/latest/tracing/)</code>
offers more features and is used more widely across our codebase. The
<code>tracing</code> library also integrates with debuglog, has convenient
macros and supports structured logging.

Note: It is acceptable to use `println!` in tests for component manager.

This is currently guarded by a
[lint rule](https://cs.opensource.google/fuchsia/fuchsia/+/main:src/sys/component_manager/src/lib.rs;l=10;drc=ec4ab722fea7b52a51176a76b61ca005c2dfdc77).

### <span class="compare-worse">DO NOT</span> use the `anyhow` library

The <code>[anyhow](https://docs.rs/anyhow/latest/anyhow/)</code> library makes
it impossible to build well-structured error types and match on them.

There are no known exceptions to this rule.

### <span class="compare-better">DO</span> use the `thiserror` library to create custom error types

The <code>[thiserror](https://docs.rs/thiserror/latest/thiserror/)</code>
library automates the <code>std::error::Error</code> implementation for custom
error types. Avoid implementing <code>std::error::Error</code> by hand.

There are no known exceptions to this rule.

### <span class="compare-better">DO</span> create a custom error type for a specific feature/action in Component Manager

Custom error types help enumerate all possible error states for your
feature/action. They will not be tied to any other area of component manager and
can be maintained independently of the rest of the codebase.

Note: With a custom error type, the rust compiler can ensure that all error
states specific to your feature are handled. This will avoid the need to match
on unrelated/impossible errors.

### <span class="compare-better">DO</span> consider whether implementing `Into<ModelError>` for your custom error type is necessary

ModelError has many enum variants for different error categories. It may not be
necessary to add your custom error type to ModelError at all.

Note: The rust compiler will not warn you about unused error variants. This is
because component manager is currently structured as a library with public
modules.

Note: The size of a ModelError object is the size of the largest enum variant.
It is in our best interest to keep the size of ModelError small.

Note: If you have to convert to ModelError, do it precisely at the point where a
ModelError is required. Until then, use your own custom error type.

### <span class="compare-worse">DO NOT</span> add precise errors to `ModelError`

Prefer creating a custom error type to represent all errors for your
feature/action, even if you currently have exactly one error. If a precise error
is added directly to ModelError, developers can lose context about where the
error is applicable.

### <span class="compare-worse">DO NOT</span> store generic error types like `CloneableError`

Generic error types make it impossible to build well-structured errors. There is
no idiomatic way to match on these generic types.

The only exception to this rule is if the error comes from an external library
that is using a generic error type. Changing the external library might not be
possible, so storing the generic is acceptable.

Warning: Do not use the `#[from]` property of `thiserror` on generic error
types. This makes it easy to implicitly convert any error into a generic error.
Use `#[source]` instead and do an explicit conversion.

Warning: With generic error types, it is easier to create illogically nested
errors (error type A contains error type B which contains error type A).

### <span class="compare-worse">DO NOT</span> use existing error types for behavior that is “close enough”

When root-causing bugs in component manager, errors that incorrectly describe
the state of the world can make things difficult to debug, especially in
component manager. Prefer creating an error type to precisely describe your
distinct error state. Another option is to add an enum to the existing error
type that provides additional classification.

There are no known exceptions to this rule.

Note: If you are reusing an error type, ensure that the human-readable error
message and any automated responses to the error condition match your
expectations.

### <span class="compare-better">DO</span> consider whether logging is absolutely necessary

One way to think of component manager is as an uber library of sorts. As a
library it is often true that component manager **can not know** if the error is
meaningful. Consider, for example, what would happen if a UDP library logged
every dropped packet.

While there is no universally accepted advice on this matter, there are some
tips that should be followed for component manager:

*   Be conservative with logging.
*   Logs that were created to assist with development/debugging must be removed
    or set to `DEBUG` log level before submitting.
*   Avoid adding logs in hot code paths. This can produce log spam and spam is
    not useful.
*   If an error is also being returned to a client over FIDL with the same
    amount of detail, do not log the error.
*   Conversely, if some amount of detail is being lost when sending errors over
    FIDL, it is acceptable to log the detailed error message.
    *   Also consider why the client isn't getting the detailed error.
*   Log errors where there is no client involved and the error is likely to be
    important in debugging an issue.
*   Respect the logging guidelines set in [RFC-0003][rfc-0003]

### <span class="compare-better">DO</span> add component identifiers to logs and errors

Component manager often produces errors that are related to a specific
component. Ensure that errors include component identifiers for easy debugging.
For logs, use the component-scoped logger or include the component identifier in
the message.

Acceptable component identifiers (in order of preference): monikers, component
URLs, instance IDs

### <span class="compare-worse">DO NOT</span> print `Debug` strings of objects in a log message

The `Debug` trait and the corresponding `{:?}` formatting specifier should only
be used for interactive debugging purposes. Outputting the equivalent of a JSON
object into the logs makes them harder to understand. Prefer printing
human-readable error messages to logs.

There are no known exceptions to this rule.

### ​<span class="compare-worse">DO NOT</span> store a stringified error message

Do not store the `Debug` or `Display` string of an error. String errors have no
reliable structure and are impossible to match on. Always prefer to store the
error as-is.

Some errors in external libraries do not implement required traits like `Clone`
or `PartialEq`. In those **rare situations**, if it is impossible to add those
traits to the external library, it is acceptable to stringify the error message
and store that instead.

Note: [FIDL guidelines][fidl-string-error] make a similar recommendation.

### <span class="compare-worse">DO NOT</span> create functions for each error variant

Creation functions for error variants are unnecessary. They allow implicit
conversions for some types and they also hide field names. Prefer to create the
error type directly and set field names manually.

There are no known exceptions to this rule.

### <span class="compare-better">DO</span> think like a crate author

Component manager is a large crate. We already moved routing into its own crate,
we might do more in the future. Consider what error types you'd use if the
particular section of code you're working on were part of its own crate. What
would reasonable error types be for the logical collection of code in the crate?

[rfc-0003]: /docs/contribute/governance/rfcs/0003_logging.md
[fidl-string-error]: /docs/development/api/fidl.md#avoid-messages-and-descriptions-in-errors
