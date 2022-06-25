# Rust Rubric

[TOC]

This document lists conventions to follow when writing Rust in the Fuchsia Source Tree. These conventions are a combination of best practices, project preferences, and some choices made for the sake of consistency.

<!-- TODO add collapsible <details> sections around guideline bodies -->
<!-- TODO inline text of upstream guidelines once fuchsia-specific guidelines settle -->

## Guidelines

### Naming

#### Casing conforms to Rust idioms
See [C-CASE](https://rust-lang.github.io/api-guidelines/naming.html#c-case).

#### Ad-hoc conversions follow `as_`, `to_`, `into_` conventions
See [C-CONV](https://rust-lang.github.io/api-guidelines/naming.html#c-conv).

#### Getter names follow Rust convention

With a few exceptions, the `get_` prefix is not used for getters in Rust code.

See [C-GETTER](https://rust-lang.github.io/api-guidelines/naming.html#c-getter).

#### Methods on collections that produce iterators follow `iter`, `iter_mut`, `into_iter`
See [C-ITER](https://rust-lang.github.io/api-guidelines/naming.html#c-iter).

#### Iterator type names match the methods that produce them
See [C-ITER-TY](https://rust-lang.github.io/api-guidelines/naming.html#c-iter-ty).

#### Names use a consistent word order
See [C-WORD-ORDER](https://rust-lang.github.io/api-guidelines/naming.html#c-word-order).



### Interoperability

#### Types eagerly implement common traits
`Copy`, `Clone`, `Eq`, `PartialEq`, `Ord`, `PartialOrd`, `Hash`, `Debug`, `Display`, `Default` should all be implemented when appropriate.

See [C-COMMON-TRAITS](https://rust-lang.github.io/api-guidelines/interoperability.html#c-common-traits).

#### Conversions use the standard traits `From`, `AsRef`, `AsMut`
See [C-CONV-TRAITS](https://rust-lang.github.io/api-guidelines/interoperability.html#c-conv-traits).

#### Collections implement `FromIterator` and `Extend`
See [C-COLLECT](https://rust-lang.github.io/api-guidelines/interoperability.html#c-collect).

#### Data structures implement Serde's `Serialize`, `Deserialize`
See [C-SERDE](https://rust-lang.github.io/api-guidelines/interoperability.html#c-serde).

#### Types are `Send` and `Sync` where possible
See [C-SEND-SYNC](https://rust-lang.github.io/api-guidelines/interoperability.html#c-send-sync).

#### Error types are meaningful and well-behaved
See [C-GOOD-ERR](https://rust-lang.github.io/api-guidelines/interoperability.html#c-good-err).

#### Binary number types provide `Hex`, `Octal`, `Binary` formatting
See [C-NUM-FMT](https://rust-lang.github.io/api-guidelines/interoperability.html#c-num-fmt).

#### Generic reader/writer functions take `R: Read` and `W: Write` by value
See [C-RW-VALUE](https://rust-lang.github.io/api-guidelines/interoperability.html#c-rw-value).



### Macros

#### Input syntax is evocative of the output
See [C-EVOCATIVE](https://rust-lang.github.io/api-guidelines/macros.html#c-evocative).

#### Macros compose well with attributes
See [C-MACRO-ATTR](https://rust-lang.github.io/api-guidelines/macros.html#c-macro-attr).

#### Item macros work anywhere that items are allowed
See [C-ANYWHERE](https://rust-lang.github.io/api-guidelines/macros.html#c-anywhere).

#### Item macros support visibility specifiers
See [C-MACRO-VIS](https://rust-lang.github.io/api-guidelines/macros.html#c-macro-vis).

#### Type fragments are flexible
See [C-MACRO-TY](https://rust-lang.github.io/api-guidelines/macros.html#c-macro-ty).



### Documentation

#### Crate level docs are thorough and include examples
See [C-CRATE-DOC](https://rust-lang.github.io/api-guidelines/documentation.html#c-crate-doc).

#### All items have a rustdoc example
See [C-EXAMPLE](https://rust-lang.github.io/api-guidelines/documentation.html#c-example).

> Note: this guideline is not reasonable to enforce for targets which build on Fuchsia until
> doctests are supported on Fuchsia targets. See
> [#38215](https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=38215).

#### Examples use `?`, not `try!`, not `unwrap`
See [C-QUESTION-MARK](https://rust-lang.github.io/api-guidelines/documentation.html#c-question-mark).

<!-- TODO how does this interact with avoiding ? in tests? -->

#### Function docs include error, panic, and safety considerations
See [C-FAILURE](https://rust-lang.github.io/api-guidelines/documentation.html#c-failure).

#### Prose contains hyperlinks to relevant things
See [C-LINK](https://rust-lang.github.io/api-guidelines/documentation.html#c-link).

#### Rustdoc does not show unhelpful implementation details
See [C-HIDDEN](https://rust-lang.github.io/api-guidelines/documentation.html#c-hidden).

#### Every `unsafe` block has an accompanying justification

Note: This guideline pertains only to safety documentation when performing unsafe operations. See
[C-FAILURE](https://rust-lang.github.io/api-guidelines/documentation.html#c-failure) for guidelines
on documenting function safety requirements.

Safety justifications should begin with `// SAFETY: ` and explain why the unsafe block is sound.

**Do**

```rust
// SAFETY: <why this unsafe operation's safety requirements are met>
```

**Don't**

```rust
// Safety: <...>
// [SAFETY] <...>
// <...>
// SAFETY: Trust me.
```

Unsafe code should explain why the unsafe block is necessary and why the code contained within the
block is sound. If there are safe alternatives which may appear suitable but cannot be used, the
reason why they cannot be used should be documented as well.

**Do**

```rust
// SAFETY: The `bytes` returned from our string builder are guaranteed to be
// valid UTF-8. We used to call `from_utf8`, but this caused performance issues
// with large inputs.
let s = unsafe { String::from_utf8_unchecked(bytes) };
```

**Don't**

```rust
// SAFETY: We shouldn't have to validate `bytes`, and the safe version is slow.
let s = unsafe { String::from_utf8_unchecked(bytes) };
```

Justifications should directly address the requirements for the operation. It's okay to summarize as
long as all of the requirements are addressed.

**Do**

```rust
// SAFETY: The caller has guaranteed that `ptr` is valid for reads, properly
// aligned, and points to a properly-initialized `T`.
unsafe {
    let x = ptr.read();
}
```

**Do**

```rust
// SAFETY: The caller has guaranteed that `ptr` points to a valid `T`.
unsafe {
    let x = ptr.read();
}
```

**Don't**

```rust
// SAFETY: `ptr` is safe to read.
unsafe {
    let x = ptr.read();
}
```

Safety justifications should address *why* an operation is justified, not just that an operation is
justified.

**Do**

```rust
const BUFFER_LEN: usize = 1024;
fn partially_init(n: usize) -> MaybeUninit<[i32; BUFFER_LEN]> {
    // These asserts ensure our safety conditions are met later on
    const _: () = assert!(BUFFER_LEN <= 1024);
    assert!(n < BUFFER_LEN);

    let mut buffer = MaybeUninit::<[i32; BUFFER_LEN]>::uninit();
    let ptr = buffer.as_mut_ptr().cast::<i32>();
    for i in 0..n {
        // SAFETY:
        // - `ptr` points to the first `i32` of `buffer`.
        // - `buffer` has space for BUFFER_LEN elements and we asserted that
        //   `n < BUFFER_LEN`.
        // - We asserted that `BUFFER_LEN <= 1024`, so `size_of::<i32>() * i`
        //   is at most 4096 which is less than `isize::MAX` and `usize::MAX`.
        let element = unsafe { &mut *ptr.add(i) };
        *element = i as i32;
    }

    buffer
}
```

**Don't**

```rust
const BUFFER_LEN: usize = 1024;
fn partially_init(n: usize) -> MaybeUninit<[i32; BUFFER_LEN]> {
    // Why are these asserts here?
    const _: () = assert!(BUFFER_LEN <= 1024);
    assert!(n < BUFFER_LEN);

    let mut buffer = MaybeUninit::<[i32; BUFFER_LEN]>::uninit();
    let ptr = buffer.as_mut_ptr().cast::<i32>();
    for i in 0..n {
        // SAFETY:
        // - `ptr` is in bounds or one byte past the end of an allocated object.
        // - `ptr + i` is also in bounds.
        // - The computed offset, in bytes, doesn't overflow an `isize`.
        // - The offset being in bounds does not rely on "wrapping around" the
        //   address space.
        let element = unsafe { &mut*ptr.add(i) };
        *element = i as i32;
    }

    buffer
}
```

#### Unsafe traits are documented, unsafe trait impls are justified

Unsafe traits should be documented according to the same guidelines as unsafe functions.

Unsafe trait definitions should document safety considerations (See [C-FAILURE][c-failure]), and
unsafe trait implementations should be justified (See
["Every `unsafe` block has an accompanying justification"][every-unsafe-block]))

[c-failure]: https://rust-lang.github.io/api-guidelines/documentation.html#c-failure
[every-unsafe-block]: #every-unsafe-block-has-an-accompanying-justification

**Do**

```rust
/// A labeler that always returns unique labels.
///
/// # Safety
///
/// Every time `create_unique_label()` is called on the same labeler, it must
/// return a distinct `u32`.
unsafe trait UniqueLabeler {
    /// Returns a new unique label.
    fn create_unique_label(&mut self) -> u32;
}

struct SequentialLabeler {
    next: Option<u32>,
}

// SAFETY: `create_unique_label()` will always return the next sequential label
// or panic if all available labels are exhausted.
unsafe impl UniqueLabeler for SequentialLabeler {
    fn create_unique_label(&mut self) -> u32 {
        if let Some(next) = self.next {
            self.next = (next < u32::MAX).then(|| next + 1);
            next
        } else {
            panic!("sequential unique labels exhausted");
        }
    }
}
```

**Don't**

```rust
/// A labeler that always returns unique labels.
unsafe trait UniqueLabeler {
    /// Returns a new unique label.
    fn create_unique_label(&mut self) -> u32;
}

struct SequentialLabeler {
    next: u32,
}

unsafe impl UniqueLabeler for SequentialLabeler {
    fn create_unique_label(&mut self) -> u32 {
        // This will have correct panicking behavior in debug builds because
        // integer overflow is trapped. In a release build, this will still
        // overflow but our labeler will not panic!
        let result = self.next;
        next += 1;
        result
    }
}
```

#### Unsafe operations are always in an `unsafe` block

Note: This guideline depends on a change to linting behavior and cannot yet be followed
([tracking issue](https://fxbug.dev/94323)). Continue to adhere to other guidelines in this section.

`unsafe` functions are not considered unsafe contexts in Fuchsia. Unsafe operations must always be
located inside an `unsafe` block, even if they are in an `unsafe` function body.

**Do**

```rust
unsafe fn clear_slice(ptr: *mut i32, len: usize) {
    assert!(len.checked_mul(mem::size_of::<i32>()).unwrap() < isize::MAX);

    // SAFETY:
    // - The caller has guaranteed that `ptr` points to `len` consecutive, valid
    //   i32s and that the data at behind `ptr` is not simultaneously accessed
    //   through any other pointer.
    // - We asserted that the total size of the slice is less than isize::MAX.
    let slice = unsafe { slice::from_raw_parts_mut(ptr, len) };
    for x in slice.iter_mut() {
        *x = 0;
    }
}
```

**Don't**

```rust
unsafe fn clear_slice(ptr: *mut i32, len: usize) {
    // We forgot to assert that the length of the slice is less than isize::MAX!
    // If we justified our call to from_raw_parts_mut, we would have been much
    // more likely to remember.

    let slice = slice::from_raw_parts_mut(ptr, len);
    for x in slice.iter_mut() {
        *x = 0;
    }
}
```



### Predictability

#### Smart pointers do not add inherent methods
See [C-SMART-PTR](https://rust-lang.github.io/api-guidelines/predictability.html#c-smart-ptr).

#### Conversions live on the most specific type involved
See [C-CONV-SPECIFIC](https://rust-lang.github.io/api-guidelines/predictability.html#c-conv-specific)

#### Functions with a clear receiver are methods
See [C-METHOD](https://rust-lang.github.io/api-guidelines/predictability.html#c-method).

#### Functions do not take out-parameters
See [C-NO-OUT](https://rust-lang.github.io/api-guidelines/predictability.html#c-no-out).

#### Operator overloads are unsurprising
See [C-OVERLOAD](https://rust-lang.github.io/api-guidelines/predictability.html#c-overload).

#### Only smart pointers implement `Deref` and `DerefMut`
See [C-DEREF](https://rust-lang.github.io/api-guidelines/predictability.html#c-deref).

#### Constructors are static, inherent methods
See [C-CTOR](https://rust-lang.github.io/api-guidelines/predictability.html#c-ctor).



### Flexibility

#### Functions expose intermediate results to avoid duplicate work
See [C-INTERMEDIATE](https://rust-lang.github.io/api-guidelines/flexibility.html#c-intermediate).

#### Caller decides where to copy and place data
See [C-CALLER-CONTROL](https://rust-lang.github.io/api-guidelines/flexibility.html#c-caller-control).

#### Functions minimize assumptions about parameters by using generics
See [C-GENERIC](https://rust-lang.github.io/api-guidelines/flexibility.html#c-generic).

#### Traits are object-safe if they may be useful as a trait object
See [C-OBJECT](https://rust-lang.github.io/api-guidelines/flexibility.html#c-object).



### Type safety

#### Newtypes provide static distinctions
See [C-NEWTYPE](https://rust-lang.github.io/api-guidelines/type-safety.html#c-newtype).

#### Arguments convey meaning through types, not `bool` or `Option`
See [C-CUSTOM-TYPE](https://rust-lang.github.io/api-guidelines/type-safety.html#c-custom-type).

#### Types for a set of flags are `bitflags`, not enums
See [C-BITFLAG](https://rust-lang.github.io/api-guidelines/type-safety.html#c-bitflag).

#### Builders enable construction of complex values
See [C-BUILDER](https://rust-lang.github.io/api-guidelines/type-safety.html#c-builder).



### Dependability

#### Functions validate their arguments
See [C-VALIDATE](https://rust-lang.github.io/api-guidelines/dependability.html#c-validate).

#### Destructors never fail
See [C-DTOR-FAIL](https://rust-lang.github.io/api-guidelines/dependability.html#c-dtor-fail).

#### Destructors that may block have alternatives
See [C-DTOR-BLOCK](https://rust-lang.github.io/api-guidelines/dependability.html#c-dtor-block).



### Debuggability

#### All public types implement `Debug`
See [C-DEBUG](https://rust-lang.github.io/api-guidelines/debuggability.html#c-debug).

#### `Debug` representation is never empty
See [C-DEBUG-NONEMPTY](https://rust-lang.github.io/api-guidelines/debuggability.html#c-debug-nonempty).



### Future Proofing

#### Sealed traits protect against downstream implementations
See [C-SEALED](https://rust-lang.github.io/api-guidelines/future-proofing.html#c-sealed).

#### Structs have private fields
See [C-STRUCT-PRIVATE](https://rust-lang.github.io/api-guidelines/future-proofing.html#c-struct-private).

#### Newtypes encapsulate implementation details
See [C-NEWTYPE-HIDE](https://rust-lang.github.io/api-guidelines/future-proofing.html#c-newtype-hide).

#### Data structures do not duplicate derived trait bounds
See [C-STRUCT-BOUNDS](https://rust-lang.github.io/api-guidelines/future-proofing.html#c-struct-bounds).


## Updating the guidelines

To propose additions or modifications, open a CL and cc
[`fuchsia-rust-api-rubric@google.com`] to ensure it is reviewed. Use any
feedback to iterate on the proposal.

Once feedback has been addressed, any one of the [OWNERS] of this file who is
not the proposal author may act as facilitator and move the proposal to last
call. The facilitator will send an email to [`fuchsia-rust-api-rubric@google.com`]
announcing last call on the proposal. The proposal will be open for feedback
for 7 calendar days.

At the end of the last call period and once relevant concerns have been
discussed or addressed, a facilitator will comment on the CL with a final
decision based on the review feedback and discussion. Any controversial
decisions should be made with adequate public discussion of the relevant issues,
and should include the rationale in the CL comment. The decision outcome should
also be sent to the email thread.

If a proposal is accepted, the facilitator will leave a +2 and the author can
then submit it.

[`fuchsia-rust-api-rubric@google.com`]: mailto:fuchsia-rust-api-rubric@google.com
[OWNERS]: https://cs.opensource.google/fuchsia/fuchsia/+/master:docs/development/api/OWNERS

### Pending Topics

Pending topics are tracked in the [Rust>Rubric][monorail] Monorail component.

[monorail]: https://bugs.fuchsia.dev/p/fuchsia/issues/list?q=component:Rust%3ERubric

## Relationship with upstream Rust API guidelines

This rubric contains most of the [Rust API Guidelines][rust-guidelines], however the following
official guidelines are omitted:

* [C-FEATURE](https://rust-lang.github.io/api-guidelines/naming.html#c-feature) as Fuchsia does not
  currently support features for crates.
* [C-METADATA](https://rust-lang.github.io/api-guidelines/documentation.html#c-metadata) as Fuchsia
  does not maintain internal `Cargo.toml` files.
* [C-HTML-ROOT](https://rust-lang.github.io/api-guidelines/documentation.html#c-html-root) as
  Fuchsia does not currently publish most Rust code to `crates.io`.
* [C-RELNOTES](https://rust-lang.github.io/api-guidelines/documentation.html#c-relnotes) as most
  Rust code in Fuchsia "lives at HEAD".
* [C-STABLE](https://rust-lang.github.io/api-guidelines/necessities.html#c-stable) as Fuchsia does
  not currently publish most Rust code to `crates.io`.
* [C-PERMISSIVE](https://rust-lang.github.io/api-guidelines/necessities.html#c-permissive) as all of
  Fuchsia's Rust code is under the Fuchsia license.

The following Fuchsia-specific guidelines are included:

* [Every `unsafe` block has an accompanying justification][safety-justification-guideline]
* [Unsafe traits are documented, unsafe trait impls are justified][unsafe-traits-guideline]
* [Unsafe operations are always in an `unsafe` block][unsafe-ops-in-unsafe-blocks-guideline]

[safety-justification-guideline]: #every-unsafe-block-has-an-accompanying-justification
[unsafe-traits-guideline]: #unsafe-traits-are-documented-unsafe-trait-impls-are-justified
[unsafe-ops-in-unsafe-blocks-guideline]: #unsafe-operations-are-always-in-an-unsafe-block

[rust-guidelines]: https://rust-lang.github.io/api-guidelines/about.html
