# Ergonomic Inspect

This guide covers the usage of the
[`fuchsia_inspect_derive`](/src/lib/diagnostics/inspect/derive)
library, and assumes that you are familiar with
[Inspect](/docs/development/diagnostics/inspect)
and have basic experience with the
[`fuchsia_inspect`](/src/lib/diagnostics/inspect/rust) library.

## Overview

The `fuchsia_inspect_derive` library provides ergonomic macros, traits and
smart pointers around the `fuchsia_inspect` library, that makes it easier to
integrate inspect with your Rust code base, by:

- Owning source data and inspect data under the same RAII type
- Being idiomatic. First class support for primitives, common interior
  mutability patterns and async.
- Generating repetitive boilerplate code
- Providing a unified way to [attach a type to inspect](#inspect-attaching)
- Supporting gradual integration with existing code bases, both those that
  don't yet support inspect, and the ones that are integrated with
  `fuchsia_inspect` directly.
- Supporting foreign types that lack inspect integration. See
  [`IDebug<T>`](#idebug) for usage and constraints.

At the same time, it preserves the performance and semantics of a manual inspect
integration, by:

- Committing granular inspect tree modifications, where logical leaf nodes are
  updated independently.
- Applying static dispatch only, to avoid additional runtime overhead.
- Not using any additional synchronization primitives.

### Caveats

When you integrate your Rust code base with this library, be aware that:

- The library mirrors the internal type hierarchy of the Rust program. Limited
  structural modifications such as renaming, flattening and omitting fields are
  supported (similar to [Serde][serde-field-attrs]). If the desired inspect tree
  structure is vastly different from the type hierarchy, you should consider
  using `fuchsia_inspect` directly.
- Some features are not yet supported, requiring you to
  [implement `Inspect` manually](#implement-inspect-manually):
  - Lazy nodes, histograms and inspect arrays.
  - `Option<T>` and other enums.
  - Collection types, such as vectors and maps.
  - StringReferences
- The library promotes [custom smart pointers](#iowned), which creates another
  layer of data wrapping.


## Quick start {#quick-start}

This section shows an example where you take an existing data structure and
apply inspect to that structure. Let's start with a simple example, a Yak:

```rust
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/rust-ergonomic/src/main.rs" region_tag="quick_start_before_decl" adjust_indentation="auto" %}
```

Then, consider this construction site:

```rust
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/rust-ergonomic/src/main.rs" region_tag="quick_start_before_init" adjust_indentation="auto" %}
```

Let's make the yak inspectable. In particular:

- Expose the current hair length
- Expose the number of times the Yak has been shaved
- The credit card number should NOT be exposed

Now use `fuchsia_inspect_derive` to make this Yak inspectable:


```rust
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/rust-ergonomic/src/main.rs" region_tag="quick_start_after_decl" adjust_indentation="auto" %}
```

Now, in your main program (or in a unit test), construct the yak and attach it
to the inspect tree:


```rust
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/rust-ergonomic/src/main.rs" region_tag="quick_start_after_init" adjust_indentation="auto" %}
```

Now you have integrated a simple program with Inspect. The rest of this guide
describes the types, traits and macros of this library, and how to apply them to
real world programs.

## Derive `Inspect` {#inspect-derive}

`derive(Inspect)` can be added to any named struct, *but each of its fields
must also implement `Inspect`* (except for `inspect_node` and skipped fields).
The library provides implementations of `Inspect` for several types:

- The [`IOwned` smart pointers](#iowned)
- Many common [interior mutability wrappers](#interior-mutability)
- All inspect properties (`UintProperty`, `StringProperty`, etc) except for
  arrays and histograms
- Other `Inspect` types. [See the section on nesting](#inspect-nesting).

If you add a type that isn't `Inspect`, you get a compiler error:

```rust
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/rust-ergonomic/src/compiler_errors.rs" region_tag="derive_inspect_unwrapped" adjust_indentation="auto" %}
```

### Nested `Inspect` Types {#inspect-nesting}

`Inspect` types can be freely nested, like so:

```rust
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/rust-ergonomic/src/main.rs" region_tag="inspect_nested_decl" adjust_indentation="auto" %}
```

### Fields and Attributes {#inspect-attributes}

All fields, except for skipped fields and `inspect_node`, must implement
`Inspect`, either for `&mut T` or `&T`.

If an `inspect_node` field is present, instances will have its own node in the
inspect tree. It must be a `fuchsia_inspect::Node`:

```rust
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/rust-ergonomic/src/main.rs" region_tag="inspect_node_present_decl" adjust_indentation="auto" %}
```

If `inspect_node` is absent, fields will be attached directly to the parent node
(meaning that the name provided to `with_inspect` will be ignored):

```rust
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/rust-ergonomic/src/main.rs" region_tag="inspect_node_absent_decl" adjust_indentation="auto" %}
```

If your type needs to add or remove nodes or properties dynamically,
it should own an inspect node. The inspect node is needed when
nodes or properties are added or removed after the initial attachment.

`derive(Inspect)` supports the following field attributes:

- `inspect(skip)`: The field is ignored by inspect.
- `inspect(rename = "foo")`: Use a different name. By default, the field name
  is used.
- `inspect(forward)`: Forwards the attachment to an inner `Inspect` type, omitting one
  layer of nesting from the inspect hierarchy. All other fields should not have any inspect
  attributes. The type must NOT have an `inspect_node` field. Useful for wrapper types.
  For example:

```rust
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/rust-ergonomic/src/main.rs" region_tag="inspect_forward_decl" adjust_indentation="auto" %}
```

### Manually Managed Inspect Types

If you are integrating with a code base that uses `fuchsia_inspect` directly,
its types are not be aware of `fuchsia_inspect_derive`. Do not add such
manually managed types as fields to an `Inspect` type directly. Instead,
[implement `Inspect` manually](#implement-inspect-manually) for the type.
Avoid attaching manually outside of the `Inspect` trait,
since attachment in `fuchsia_inspect_derive` occurs after construction.
Attaching in a constructor can silently cause its inspect state to be
absent.

### Attaching to the Inspect Tree {#inspect-attaching}

An inspect type should be attached once, and immediately after instantiation,
using the `with_inspect` extension trait method:

```rust
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/rust-ergonomic/src/main.rs" region_tag="inspect_node_present_init" adjust_indentation="auto" %}
```

If you have a nested `Inspect` structure, you should only attach the top-level
type. The nested types are attached implicitly:

```rust
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/rust-ergonomic/src/main.rs" region_tag="inspect_nested_init" adjust_indentation="auto" %}
```

Note that when a `Yak` is constructed from within a `Stable`, there is no
`with_inspect` call present. Instead, the `Yak` is automatically attached as a
child of the `Stable`. However, you can still attach a `Yak` when it is the
top-level type, such as in the unit tests for `Yak`. This allows you to test any
`Inspect` type in isolation.

You can optionally choose to supply inspect nodes in constructors instead of
explicitly calling `with_inspect` at the construction sites. First, ensure that
the type is NOT nested under another `Inspect` type (as this would cause
duplicate attachments). Sedondly, make sure to document this fact clearly,
so the calling user is aware of your attachment convention.

### Interior mutability {#interior-mutability}

In Rust (and particularly `async` Rust), it is common to use interior
mutability. This library provides `Inspect` implementations for several smart
pointers and locks:

- `std`: `Box`, `Arc`, `Rc`, `RefCell`, `Mutex` and `RwLock`
  - Note that `Cell` does NOT work. Instead, upgrade to a `RefCell`.
- `parking_lot`: `Mutex` and `RwLock`
- `futures`: `Mutex`

Generally, interior mutability within a `derive(Inspect)` type just works:

```diff
#[derive(Inspect)]
struct Stable {
-   yak: Yak,
+   yak: Arc<Mutex<Yak>>,
-   horse: Horse,
+   horse: RefCell<Horse>,
    inspect_node: fuchsia_inspect::Node,
}
```

Make sure to put your smart pointers inside your mutability wrapper:

```diff
struct Yak {
-   coins: IValue<Rc<RwLock<u32>>>,  // Won't compile
+   coins: Rc<RwLock<IValue<u32>>>,  // Correct
}
```

If an inner type is behind a lock, attachment will fail if the lock is
acquired by someone else. Hence, always attach immediately after
instantiation.

### Implement `Inspect` Manually {#implement-inspect-manually}

The `derive(Inspect)` derive-macro generates an
`impl Inspect for &mut T { .. }`. Oftentimes, this works fine, but in
some cases you may need to implement `Inspect` manually. Fortunately,
the `Inspect` trait is quite simple:

```rust
trait Inspect {
    /// Attach self to the inspect tree
    fn iattach(self, parent: &Node, name: AsRef<str>) -> Result<(), AttachError>;
}
```

Do not return an `AttachError` for structural errors in the data.
Instead, report the error using logs or an inspect node.
`AttachError` is reserved for irrecoverable invariant errors that
fail the entire attachment.

## `IOwned` Smart Pointers {#iowned}

Smart pointers may sound scary, but you probably use them everyday already. For
instance, `Arc` and `Box` are smart pointers. They are statically dispatched,
and have first-class support in Rust (through [deref coercion]). This makes them
minimally invasive.

`fuchsia_inspect_derive` comes with a few useful smart pointers that implement
`Inspect` and can be used to wrap primitives, debuggable types, and more. They
all share the same behavior: An `IOwned<T>` smart pointer owns a generic
**source type** `T` and some associated **inspect data**.

Here is a demonstration of the `IOwned` API:

```rust
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/rust-ergonomic/src/main.rs" region_tag="smart_pointers_ivalue" adjust_indentation="auto" %}
```

An `IOwned<T>` smart pointer should not be instantiated directly, but rather one
of its variants:

### `IValue<T>` {#ivalue}

The `IValue<T>` smart pointer wraps a primitive ([or any type `T: Unit`](#unit)).
For example, an `IValue<f32>` is represented as a `DoubleProperty`, and
an `IValue<i16>` is represented as an `IntProperty`.

An `IValue` of a primitive results in the same structure as using a plain
inspect property directly. So, why would you use an `IValue`? If you only
need to write or increment a value, you can use a plain inspect property. If you
also need to read the value, you should use an `IValue`.

### `IDebug<T>` {#idebug}

The `IDebug<T>` smart pointer wraps a debuggable type, and maintains the debug
representation of `T` as a `StringProperty`. This is useful for:

- Foreign types, where adding an inspect implementation is infeasible
- Debugging, to quickly verify some state about your program

Avoid using debug representations in production code, since they come with
the following issues:

- Debug representations are written on every inspect update, which can result in
  unnecessary performance overhead.
- Debug representations can exhaust the space of the inspect VMO, causing
  truncation of the entire inspect state.
- Debug representations cannot be integrated with the privacy pipeline: if any
  PII is exposed as part of the debug string, the entire field must be
  considered PII. Managing your own structured data allows to granularly redact
  fields containing PII.

## The `Unit` Trait {#unit}

The `Unit` trait describes the inspect representation of a type, how to
initialize it, and how to update it. It should be implemented for types that act
as a logical leaf node, and does NOT support per-field updates. This library
provides implementations of `Unit` for most primitives. For example, `u8`,
`u16`, `u32` and `u64` are represented as a `UintProperty`.

### Usage in IValue {#unit-usage}

A `Unit` type should be wrapped in an `IValue<T: Unit>` (see above), for a RAII
managed inspectable type. It is NOT recommended to call methods on `Unit`
directly.

### Derive `Unit` {#unit-derive}

Sometimes a logical `Unit` is a composite type. Unit can be derived for a named
struct, as long as its fields also implement `Unit`. For example:

```rust
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/rust-ergonomic/src/main.rs" region_tag="unit_plain_decl" adjust_indentation="auto" %}
```

`Unit` can be nested, but keep in mind that all fields are still written at
the same time:

```rust
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/rust-ergonomic/src/main.rs" region_tag="unit_nested_decl" adjust_indentation="auto" %}
```

#### Attributes {#unit-attributes}

`derive(Unit)` supports the following field attributes:

- `inspect(skip)`: The field is ignored by inspect.
- `inspect(rename = "foo")`: Use a different name. By default, the field name
  is used.

[deref coercion]:
https://doc.rust-lang.org/1.27.2/book/second-edition/ch15-02-deref.html

[serde-field-attrs]:
https://serde.rs/field-attrs.html
