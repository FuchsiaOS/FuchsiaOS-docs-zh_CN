# Updating the VMO file format

This document describes how to update or extend the [Component Inspection File
Format][inspect-file-format].

While extending the format, it is imperative that you don't break any existing functionality, in
particular, the Inspect readers and validation tests. However, packing all changes into a single
change can be overwhelming to review. In general, there should be 5-9 changes or steps in a chain
to alter the VMO format:

1. [(Possibly not applicable) Choose a type number by updating the VMO Format Docs](#choose-type).
1. [Update the Rust reader.](#update-rust-reader)
1. [Update the C++ reader.](#update-cpp-reader)
1. [Update the Rust writer.](#update-rust-writer)
1. [Update the C++ writer.](#update-cpp-writer)
1. [(Potentially) Update the validator.](#update-validator-tests)
1. [Update the Dart writer.](#update-dart)
1. Update the documentation.
1. (Not a change)(Optional) Send a feature announcement.

## Choosing a type number {#choose-type}

View the type table in the [Inspect file format][inspect-file-format] and choose an
available type number. There are a total of 256 possible types in the current specification.

In order to reserve the new type, update the [Inspect file format][inspect-file-format].

## Implementation

It is cumbersome to test a reader or writer without the other and difficult to get the proper
block-level API without both a reader and writer.

To test a reader and writer:

1. Pick a language and design and implement the feature entirely in that language, using unit
   tests to model actual usage of the API.

1. Split the changes into separate reader and writer changes, stacking the writer on top of
   the reader.

   At this point, tests in the reader that rely on the writer API are probably broken
   in the reader change.

1. Rebase into that change and rewrite the tests (keeping around the original version of the tests)
   using lower level functionality.

   Typically, you can put all changes to block-code in the reader change, making it possible but
   messy to write tests for the reader API.

1. The test will be ugly. Rebase into the writer change and remove the modified tests, replacing
   them with the original tests that are written in terms of the high level API.

1. Use the two changes for reference and duplicate them in the second language.

   It's much easier now that you aren't designing an API without actually using it.
   Ideally, you would also update the Dart library at this point.
   There is no Dart reader, so changes in the Rust/C++ writers won't break the Dart library;
   however, the ability to read a feature can't be removed from the Rust and C++ libraries unless
   the Dart one is updated.

1. (Optional) Depending on the contents of your change, you may have to update the
   [validator][validator-tests] tests as you go, since changes to the existing format of existing
   blocks will likely break them.

The following sections outline how to put together each change, but in practice, take these as hints
for designing the whole system cohesively before splitting the design as described above.

## Update the Rust implementation

The examples in this section create a new type called `MyFoo`.

### Set up

1. Include tests:

```
fx set core.x64 --with //src/lib/diagnostics:tests
```

1. Run tests:

   Note: The `inspect-format-tests` are the VMO block format tests, and `fuchsia-inspect-tests` are
   the core library tests.

```
fx test inspect-format-tests fuchsia-inspect-tests
```

### Reader change {#update-rust-reader}

#### Bitfield updates

1. If defining a new Block Type, update [the `BlockType` enum][block-type-rs].

   Note: This must be the same as the VMO format docs and the C++ implementation.
1. Update the methods and functions defined for `BlockType`.

1. If changing the fields in an existing block or creating a new `BlockType`, update
   [the bitfield layout][bitfield-rs].

1. Run `inspect-format-tests` to verify changes compile:

```
fx test inspect-format-tests
```

1. Update [the block definition][block-rs] to include methods for reading and writing the new
   fields. It is appropriate to include write-functionality at this time in the block library;
   it is next to impossible to write a reader test without it.

   Note: Use `#[cfg(test)]` if the function is currently test-only and only used within the
   current crate. Otherwise, use `#[doc(hidden)]` to elide the documentation from public view
   for now. This will keep it out of the crate public API.

1. Write block tests that exercise the new functionality.

1. Write tests that make assertions on the first 8-16 bytes of the block.

   Note: The slice should be formatted as little endian.

   Typically this means writing the expected contents as a `&[u8]` that
   contains hex values and asserting its equivalence to the buffer the block is using as a
   container.

#### Update the reader

You can find the reader code in [mod.rs][reader-mod-rs].
The tests in this change will probably be tricky, because the high-level API writer doesn't exist
yet.

### Writer change {#update-rust-writer}

#### State

The primary changes here will be in [the `State` functionality][state-rs].
This is where blocks can be allocated and converted into the new type.
If all you're doing is modifying an existing block, this is likely the only place you need
to make changes.

#### Creating a new value type

There is a [types directory][types-dir] where you can add a new file for your type.

1. Create the new type in the file created. Use an existing type as an example. Types always
   have access to internal `State`. Use this to create the necessary
   methods on your new type, calling into the methods created in [`State`][state-rs].

1. Add a method to [`Node`][node-rs] for creating the new type.

1. Ensure that your type has RAII semantics in the VMO. If your type is
   a value, this is probably done automatically by the boilerplate copied from an existing
   type in step 1.

Finally, go back and update the tests from the Reader change to use the new API!

## Update the C++ implementation

The examples in this section create a new type called `MyFoo`.

As noted above, this section should be two changes in Gerrit.

### Set up

1. Include tests:

Note: You can also use Fuchsia products other than `core.x64`.

```
fx set core.x64 --with //zircon/system/ulib/inspect:tests
```

1. Run tests.

```
fx test inspect-cpp-unittest
```

### Reader change {#update-cpp-reader}

#### Bitfield updates

This section describes how to define the bitfields for your new type.

Update [the block definition][block-header].

1. Change `BlockType` to include your new type. For example: `kMyFoo = #;`

   Note: This must be the same as the VMO format docs and the Rust implementation.

1. If your type needs a new header (typically if it is not a `VALUE`),
   define the header bitfields for your type with a struct. For example:
   `struct MyFooBlockFields final : public BlockFields`.

1. If your type needs a new payload (it requires using the second 8 bytes of the block),
   define the payload bitfields for your type with a struct. For example:
   `struct MyFooBlockPayload final`.

1. If your type contains enums (such as format),
   define a new enum at the top of block.h. For example: `enum class MyFooBlockFormat : uint8_t`.

#### Implement the type reader

This section describes how to make your new type readable.

Update [the Inspect hierarchy][hierarchy-header] based on your type:

* {A value (child of `Node`)}
    1. Update `PropertyFormat` enum with a new number for your type. This must be sequential in
        this specific enum and does not need to match the format type ordinal you chose.

    1. Create a new value type. For example,
        `using MyFooValue = internal::Value<T, static_cast<size_t>(PropertyFormat::kMyFoo)>;`

    1. Update `PropertyValue` variant with the new value.
        Note: The index in `fit::internal::variant` must match the value of `PropertyFormat`.

* {Not a value}
    1. You need to make your own in-memory representation objects in the
        [hierarchy][hierarchy-header] file.

1. Update the [actual reader][reader-cc].

1. Update `InnerScanBlocks` to dispatch your type. If you are creating a new `Property`, you may
   only have to add your `BlockType`.

1. If you need a custom parser, implement `InnerParseMyFoo`,
   which takes a parent (if needed) and the pointer to the scanned block.

### Writer change {#update-cpp-writer}

#### Type wrapper declaration

This section describes how to declare a C++ RAII-style wrapper for your new type.

Type wrappers contain indices of blocks that are owned by the type. You are responsible for
implementing operations on those blocks, including creation and deletion, in
[State action updates](#state-action).

Update [the writer types definition][types-header].

Determine if you can reuse an existing wrapper or if you need a bespoke type:

* {Reuse}
    1. If you need to support Add, Subtract, and Set: `using MyFoo = internal::NumericProperty<T>`,
       where `T` is the argument type to those operations.

    1. If you need to support Set: `using MyFoo = internal::Property<T>`, where `T` is the argument
       type to Set.

    1. If you need to support numeric operations on an array:
       `using MyFood = internal::ArrayProperty<T>`, where `T` is the argument type for slots in the
        array.

    1. If you need to support inserting to a histogram:
       `using MyFoo = internal::{Linear,Exponential}Histogram<T>`, where `T` is the argument to
       Insert.

* {Bespoke}
    1. Create a new type wrapper. For example `class MyFoo final`.

    1. Ensure your class has `internal::State` as a friend class. Note: See `class Link` for a
       copyable starting point.

#### State action updates {#state-action}

The `State` class is the actual implementation for all operations on all types. This section
describes how to implement the operations you will need to complete your wrapper implementation.

1. Update [`State` header][state-header]:
    1. Add Create and Free methods. For example:
       `MyFoo CreateMyFoo(<args>); void FreeMyFoo(MyFoo* property);` where `args` typically
       includes name, parent, and some initial value.

    1. Add methods for each operation you need to support on your type. For example, if your type
       can be Set, `void SetMyFoo(MyFoo* property, T)`, where `T` is the same type from your
       update to types.h.

1. Update [`State`][state-cc]:

   Note: Always lock the state before accessing any internal data, using
   `std::lock_guard<std::mutex> lock(mutex_);`.

   Note: Always lock the buffer before making any modifications to blocks, using
   `AutoGenerationIncrement gen(header_, heap_.get());`.

    1. Implement your new type's methods. The implementation varies between the different types.
       This section provides a high-level overview of what each method must do:

        * `MyFoo CreateMyFoo(Args...)` is responsible for allocating a number of blocks, setting
          their values, and returning them wrapped in a `MyFoo`. You may use a private constructor
          to create `MyFoo` from the `BlockIndex` objects it wraps. Various internal helpers exist
          to simplify this operation. See `CreateIntProperty` for an example.

        * `void FreeMyFoo(MyFoo* property)` is responsible for freeing all blocks wrapped by the
          `MyFoo`. There are sometimes particular ordering requirements or updates necessary for
          freeing blocks. See `InnerFreeValue` for an example of how values are freed.

        * Operations, such as `void SetMyFoo(MyFoo* property, T value)` change the value of blocks
          allocated to `MyFoo` to implement the operation. See `SetIntProperty` for an example.


#### Implement the type wrapper

This section describes how to implement the wrapper methods declared previously.

1. Update [the writer type definitions][types-cc]:
    * If you used an existing templated type, you need to override each method for your new base
      type `T`. For example, if you typed `using MyFoo = internal::Property<T>`, you will write:
      `template<> void internal::Property<T>::OPERATION(...) { ... }`

    * If you created your own type, simply create definitions for the methods you declared. You need
      to do the following:
        * Make your constructor call `state_->CreateMyFoo(...);`

        * Make your destructor call `state_->FreeMyFoo(...);`

        * Make your other methods call the corresponding implementation on State.

        * Have all your constructors and methods check that `state_` is not null before calling.

#### Implement tests

1. Update [state unit tests][state-unittest-cc] with tests for your low-level operations.

1. Update [reader unit tests][reader-unittest-cc] with tests for your high-level reader
   implementation.

## Update Dart {#update-dart}

{% comment %}
TODO([fxbug.dev/43131][update-this-doc-bug])
{% endcomment %}

## Add to validator tests {#update-validator-tests}

{% comment %}
TODO([fxbug.dev/43131][update-this-doc-bug])
{% endcomment %}

## Example change chain

1. [C++ Reader](https://fuchsia-review.googlesource.com/c/fuchsia/+/603056/6)
1. [Rust Reader](https://fuchsia-review.googlesource.com/c/fuchsia/+/599946/14)
1. [Rust Writer](https://fuchsia-review.googlesource.com/c/fuchsia/+/599947)

{% comment %}
1. TODO([fxbug.dev/43131][update-this-doc-bug]): add C++ writer example
1. TODO([fxbug.dev/43131][update-this-doc-bug]): add validator changes
1. TODO([fxbug.dev/43131][update-this-doc-bug]): add documentation changes
{% endcomment %}


<!-- xrefs -->
[update-this-doc-bug]: https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=43131
[inspect-file-format]: /docs/reference/platform-spec/diagnostics/inspect-vmo-format.md
[bitfield-rs]: /src/lib/diagnostics/inspect/format/rust/src/bitfields.rs
[block-type-rs]: /src/lib/diagnostics/inspect/format/rust/src/block_type.rs
[block-rs]: /src/lib/diagnostics/inspect/format/rust/src/block.rs
[reader-mod-rs]: /src/lib/diagnostics/inspect/rust/src/reader/mod.rs
[state-rs]: /src/lib/diagnostics/inspect/rust/src/writer/state.rs?q=state.rs
[types-dir]: /src/lib/diagnostics/inspect/rust/src/writer/types/
[node-rs]: /src/lib/diagnostics/inspect/rust/src/writer/types/node.rs
[block-header]: /zircon/system/ulib/inspect/include/lib/inspect/cpp/vmo/block.h
[hierarchy-header]: /zircon/system/ulib/inspect/include/lib/inspect/cpp/hierarchy.h
[reader-cc]: /zircon/system/ulib/inspect/reader.cc
[reader-unittest-cc]: /zircon/system/ulib/inspect/tests/reader_unittest.cc
[state-cc]: /zircon/system/ulib/inspect/vmo/state.cc
[state-header]: /zircon/system/ulib/inspect/include/lib/inspect/cpp/vmo/state.h
[state-unittest-cc]: /zircon/system/ulib/inspect/tests/state_unittest.cc
[types-cc]: /zircon/system/ulib/inspect/vmo/types.cc
[types-header]: /zircon/system/ulib/inspect/include/lib/inspect/cpp/vmo/types.h
[inspect-vmo]: /docs/reference/platform-spec/diagnostics/inspect-vmo-format.md
[validator-tests]: /src/diagnostics/validator/inspect/src
