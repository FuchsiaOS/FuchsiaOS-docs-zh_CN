# size()'s order of operation

## Constant order vs linear order `size()`

Unlike `std::` containers, not all `fbl::` containers automatically track the
size specific of a container. Specifically, the associative containers always
track the number of elements in the container, while the sequenced containers do
not by default.

In order to prevent any surprises, which may occur from encountering an
unexpected `O(n)` call to a `size()` method on a container, the API for
sequenced containers does not include a `size()` method. More
accurately, attempting to call size() will trigger a `static_assert` and fail to
compile.

Instead, if the specific number of elements currently in a sequenced container
needs to be known, and an `O(n)` count of the elements is an acceptable price to
pay, then `size_slow()` may be called instead.

On the other hand, if `O(1)` performance is needed in a sequenced container,
users may use a "sized" version of the container instead. The in-memory size of
the container will increase by one `size_t`, and adding/removing elements to and
from the container will pay the slightly higher cost of maintaining the count of
elements in the container. Calling `size()` is now permitted and calling
`size_slow()` now becomes an error.

The simple way to specify that a list should automatically track its size is to
use the `SizedSinglyLinkedList` or `SizedDoublyLinkedList` template aliases when
declaring the container. Alternatively, the template argument can be specified
manually by passing fbl::SizeOrder::Constant or fbl::SizeOrder::N to the
appropriate place in the template arguments.

All `fbl::` containers support an inexpensive `O(1)` `is_empty()` operation
which can be used to simply test whether the container holds _any_ elements or
not.

```cpp
// This is a list with an O(n) size_slow method
fbl::SinglyLinkedList<std::unique_ptr<Obj>> my_stack;
if (my_stack.size() > 50) { /* ... */ }       // not allowed. Compile time error
if (my_stack.size_slow() > 50) { /* ... */ }  // allowed.
if (my_stack.is_empty()) { /* ... */ }        // always allowed.

// This is a list with an O(1) size method
fbl::SizedSinglyLinkedList<std::unique_ptr<Obj>> my_sized_stack;
if (my_sized_stack.size() > 50) { /* ... */ }      // allowed.
if (my_sized_stack.size_slow() > 50) { /* ... */ } // not allowed. Compile time error
if (my_sized_stack.is_empty()) { /* ... */ }       // always allowed.

// A more verbose way to say the same thing
fbl::SinglyLinkedList<std::unique_ptr<Obj>,
                      fbl::DefaultObjectTag,
                      fbl::SizeOrder::Constant> another_sized_stack;
if (another_sized_stack.size() > 50) { /* ... */ }      // allowed.
if (another_sized_stack.size_slow() > 50) { /* ... */ } // not allowed. Compile time error
if (another_sized_stack.is_empty()) { /* ... */ }       // always allowed.

// Both of these work as well.
struct Tag1 {};
struct Tag2 {};
fbl::SinglyLinkedList<std::unique_ptr<Obj>, Tag1,
                      fbl::SizeOrder::Constant> t1_sized_stack;
fbl::TaggedSinglyLinkedList<std::unique_ptr<Obj>, Tag2,
                      fbl::SizeOrder::Constant> t2_sized_stack;
if (t1_sized_stack.size() > 50) { /* ... */ }      // allowed.
if (t2_sized_stack.size() > 50) { /* ... */ }      // allowed.
if (t1_sized_stack.size_slow() > 50) { /* ... */ } // not allowed. Compile time error
if (t2_sized_stack.size_slow() > 50) { /* ... */ } // not allowed. Compile time error
```

