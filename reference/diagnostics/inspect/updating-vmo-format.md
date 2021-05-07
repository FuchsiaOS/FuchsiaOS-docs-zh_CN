# Extending the VMO file format

[TOC]

This document describes how to extend the [**Component Inspection File Format**][inspect-vmo]

# Adding a new type

A total of 256 types are possible in the Inspect Format. This section
describes how to add a new type and update all library implementations.

This section describes how to break down your change into multiple
Gerrit changes:

* [Choose type number](#choose-type-number)
* [Update C++ implementation](#update-cpp)
* [Update Validator](#update-validator)
* [Update Rust](#update-rust)
* [Update Dart](#update-dart)

## Choose type number {#choose-type-number}

View the type table in the [Inspect file format][inspect-vmo], and choose
an unused type number.

Update the [documentation][inspect-vmo] for your new type, and then submit
this change for review.

## Update the C++ reference implementation. {#update-cpp}

The examples in this section create a new type called "MyFoo."

Every change from this section goes into a single Gerrit change:

* [Set up](#set-up)
* [Bitfield updates](#bitfield-updates)
* [Type wrapper declaration](#type-decl)
* [State action updates](#state-action)
* [Implement the type wrapper](#type-impl)
* [Implement the type reader](#type-reader)
* [Implement tests](#implement-tests)

### Set up {#set-up}

1. Include tests
  ```
  fx set ... --with //zircon/system/ulib/inspect:tests
  ```
1. Run tests.
  ```
  fx test -od inspect-unittest-package
  ```

### Bitfield updates {#bitfield-updates}

This section describes how to define the bitfields for your new type.

Update [/zircon/system/ulib/inspect/include/lib/inspect/cpp/vmo/block.h][block-header].

1. Change `BlockType` to include your new type. For example: `kMyFoo = #`;

1. If your type needs a new header (typically if it is not a VALUE):

   Define the header bitfields for your type with a struct. For example: `struct
   MyFooBlockFields final : public BlockFields`.

1. If your type needs a new payload (it requires using the second 8
bytes of the block):

   Define the payload bitfields for your type with a struct. For example: `struct
   MyFooBlockPayload final`.

1. If your type contains enums (such as `format`):

   Define a new enum at the top of block.h. For example: `enum class MyFooBlockFormat :
   uint8_t`.

### Type wrapper declaration {#type-decl}

This section describes how to declare a C++ RAII-style wrapper for your new type.

Type wrappers contain indices of blocks that are owned by the type. You
are responsible for implementing operations on those blocks, including
creation and deletion, in [State action updates](#state-action).

Update [/zircon/system/ulib/inspect/include/lib/inspect/cpp/vmo/types.h][types-header].

Determine:

* If you can reuse an existing wrapper depending on the operations you need to
   support:
   1. If you need to support `Add`, `Subtract`, and `Set`: `using MyFoo =
      internal::NumericProperty<T>`, where T is the argument type to those
      operations.
   1. If you need to support `Set`: `using MyFoo = internal::Property<T>`,
      where T is the argument type to `Set`.
   1. If you need to support numeric operations on an array: `using MyFood =
      internal::ArrayProperty<T>`, where `T` is the argument type for slots in
      the array.
   1. If you need to support inserting to a histogram: `using MyFoo =
      internal::{Linear,Expnential}Histogram<T>`, where `T` is the argument
      to `Insert`.
* If you cannot reuse an existing type:
  1. Create a new type wrapper. For example `class MyFoo final`.
  1. Ensure your class has internal::State as a friend class.
     Note: See `class Link` for a copyable starting point.

### State action updates {#state-action}

The `State` class is the actual implementation for all operations on all
types. This section describes how to implement the operations you will
need to complete your wrapper implementation.

Update [/zircon/system/ulib/inspect/include/lib/inspect/cpp/vmo/state.h][state-header]:

1. Update [/zircon/system/ulib/inspect/include/lib/inspect/cpp/vmo/state.h][state-header]:
   1. Add `Create` and `Free methods`. For example: `MyFoo CreateMyFoo(<args>);
      void FreeMyFoo(MyFoo* property);` where args typically includes name,
      parent, and some initial value.
   1. Add methods for each operation you need to support on your type. For
      example, if your type can be `Set`, `void SetMyFoo(MyFoo* property, T)`,
      where `T` is the same type from your update to `types.h`.
1. Update [/zircon/system/ulib/inspect/vmo/state.cc][state-cc]:
  1. Implement your new type's methods. The implementation
     varies between the different types. This section provides a high-level
     overview of what each method must do:
     - `MyFoo CreateMyFoo(Args...)` is responsible for allocating a number of
       blocks, setting their values, and returning them wrapped in a MyFoo. You
       may use a private constructor to create MyFoo from the BlockIndex objects
       it wraps. Various internal helpers exist to simplify this operation. See
       `CreateIntProperty` for an example.
     - `void FreeMyFoo(MyFoo* property)` is responsible for freeing all blocks
        wrapped by the MyFoo. There are sometimes particular ordering requirements
        or updates necessary for freeing blocks. See `InnerFreeValue` for an
        example of how values are freed.
     - Operations, such as `void SetMyFoo(MyFoo* property, T value)` change
       the value of blocks allocated to MyFoo to implement the operation. See
       `SetIntProperty` for an example.
     - Note:
       - Always lock the state before accessing any internal data, using
         `std::lock_guard<std::mutex> lock(mutex_);`
       - Always lock the buffer before making any modifications to blocks,
         using `AutoGenerationIncrement gen(header_, heap_.get());`

### Implement the type wrapper {#type-impl}

This section describes how to implement the wrapper methods declared
previously.

1. Update [/zircon/system/ulib/inspect/vmo/types.cc][types-cc]:
   * If you used an existing templated type, you need to override each
     method for your new base type `T`. For example, if you typed
     `using MyFoo = internal::Property<T>`, you will write:
    ```
template<>
void internal::Property<T>::OPERATION(...) {
  ...
}
    ```
   * If you created your own type, simply create definitions for the methods
     you declared. You need to do the following:
     - Make your constructor call `state_->CreateMyFoo(...);`
     - Make your destructor call `state_->FreeMyFoo(...);`
     - Make your other methods call the corresponding implementation on `State`.
     - Have all of your constructors and methods check that `state_` is not null
       before calling.

### Implement the type reader {#type-reader}

This section describes how to make your new type readable.

1. Update [/zircon/system/ulib/inspect/include/lib/inspect/cpp/hierarchy.h][hierarchy-header]:
1. Based on your type:
   * A value (child of Node):
     1. Update `PropertyFormat` enum with a new number for your type. This
        must be sequential in this specific enum and does not need to match the
        format type ordinal you chose.
     1. Create a new value type. For example, `using MyFooValue =
        internal::Value\<T, static_cast<size_t>(PropertyFormat::kMyFoo)>;`
     1. Update `PropertyValue` variant with the new value. Note: The index in
       `fit::internal::variant` must match the value of `PropertyFormat`.
    * Not a value:
      You need to make your own in-memory representation objects in the
      [/zircon/system/ulib/inspect/include/lib/inspect/cpp/hierarchy.h][hierarchy-header] file.
1. Update the actual reader in [/zircon/system/ulib/inspect/reader.cc][reader-cc]:
   1. Update `InnerScanBlocks` to dispatch your type.
      If you are creating a new Property, you  may only have to add  your `BlockType`.
   1. If you need a custom parser, implement `InnerParseMyFoo`, which takes
      a parent (if needed) and the pointer to the scanned block.

### Implement tests {#implement-tests}

1. Update [/zircon/system/ulib/inspect/tests/state\_unittest.cc][state-unittest-cc] with
tests for your low-level operations.
1. Update [/zircon/system/ulib/inspect/tests/reader\_unittest.cc][reader-unittest-cc] with
tests for your high-level reader implementation.

# Update Validator {#update-validator}

TODO(fxbug.dev/43131)

# Update Rust Library {#update-rust}

TODO(fxbug.dev/43131)

# Update Dart Library {#update-dart}

TODO(fxbug.dev/43131)

<!-- xrefs -->
[block-header]: /zircon/system/ulib/inspect/include/lib/inspect/cpp/vmo/block.h
[hierarchy-header]: /zircon/system/ulib/inspect/include/lib/inspect/cpp/hierarchy.h
[reader-cc]: /zircon/system/ulib/inspect/reader.cc
[reader-unittest-cc]: /zircon/system/ulib/inspect/tests/reader_unittest.cc
[state-cc]: /zircon/system/ulib/inspect/vmo/state.cc
[state-header]: /zircon/system/ulib/inspect/include/lib/inspect/cpp/vmo/state.h
[state-unittest-cc]: /zircon/system/ulib/inspect/tests/state_unittest.cc
[types-cc]: /zircon/system/ulib/inspect/vmo/types.cc
[types-header]: /zircon/system/ulib/inspect/include/lib/inspect/cpp/vmo/types.h
[inspect-vmo]: /docs/reference/diagnostics/inspect/vmo-format.md
