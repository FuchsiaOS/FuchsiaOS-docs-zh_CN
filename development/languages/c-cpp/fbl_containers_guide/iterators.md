# Iterators

So far, examples that have been presented have shown many uses of `fbl::`
iterators. Iterators in `fbl::` use an API very similar to the API used in the
`std::` containers, so hopefully this will feel very familiar to you.  It is,
however, worth taking a small amount of time to mention the things that all
`fbl::` iterators support, in addition to places where they differ slightly from
`std::` iterators.

## `iterator` and `const_iterator`

As with `std::` iterators, `fbl::` iterators come in two flavors, a non-constant
and a constant version. Operations such as `begin()` or `find()` will return a
simple iterator in the case that the reference to the container the user has is
non-`const`, and a `const_iterator` otherwise. Dereference operations performed
on `const_iterators` give a `const T&` and therefore `const` access to the
underlying object.

## `begin()`/`end()`

Just like in `std::`, the `begin()` method on a container returns an iterator to
the first element in the container while `end()` returns an iterator to an
element one past the last element in the container. Both begin and end will
automatically return a `const_iterator` when called on a `const` reference to
the container, but `cbegin()`/`cend()` may be used in the case that a
`const_iterator` is explicitly desired from a mutable reference to the
container.

## Iterator comparison and default initialized iterators vs. `end()`

Like `std::`, all fbl:: iterators support testing for equality using the `==`
and `!=` operators. Unlike `std::`, none of the iterators have random access
iterator semantics, and the `>`, `>=`, `<`, and `<=` operators are not supported
for any of the `fbl::` containers' iterator types.

In addition, there is a slight internal difference between a default initialized
iterator and an iterator returned from a call to a container's `end()` method.
Semantically, they are the same. Both a default initialized iterator and the
value of `end()` are invalid, so comparison between the two will return `true`.
The bits contained in the two instances, however are different. Always use the
comparison operators when testing for equality between two iterators.

```cpp
fbl::DoublyLinkedList<Obj*> the_list;
fbl::DoublyLinkedList<Obj*>::iterator default_init;
auto end_init = the_list.end();

if (default_init == end_init) { }                            // This comparison is true
if (!memcmp(&default_init, &end_init, sizeof(end_init))) { } // This is not.
```

## Iterator advancement

All iterators support both the pre and post-fix forms of the `++` operator. The
pre-fix form will move the iterator to the next element in the sequence, and
return a copy of the iterator now pointing to the next element. The post-fix
form will move the iterator to the next element in the sequence while returning
a copy of the pre-advanced iterator.

```cpp
// Assuming that the list starts containing objects with values
// "5 7 9", in that order.
fbl::DoublyLinkedList<Obj*> the_list;
auto iter   = the_list.begin();   // iter points to "5".
auto second = iter++;             // iter points to "7", but second points to "5"
auto third  = ++iter;             // iter points to "9", and so does third
++iter;                           // iter is now equal to the_list.end()
```

Iterators for `DoublyLinkedList`s, `HashTable`s with doubly linked list buckets,
and `WAVLTree`s all support bidirectional iteration via the `--` operator, again
in both its pre and post fix forms. `SinglyLinkedList`s and `HashTable`s with
singly linked list buckets do not.

Advancing an iterator past the end of a container gives `container.end()`.
Attempting to advance further is legal, but does not change the value of the
iterator. Backing up a bi-directional iterator that is currently set to
`container.end()` using the `--` operator will produce an iterator that points
to the last element in the list, _however backing up an iterator that has been
default initialized does not_. Instead, executing either `++` or `--` on a
default initialized leaves the iterator in the default initialized state.
Finally, backing up an iterator whose value is equal to `container.begin()` will
produce an iterator whose value is equal to `container.end()`. Subsequent
applications of `--` will walk through the elements in reverse order starting
with the last element.

## Dereferencing iterators

Elements in `fbl::` containers are always objects, therefore they always support
the `->` operator in addition to the unary `*` operator. Both produce either a
`T&` or a `const T&` (which `->` then accesses a member of) based on if the
iterator was a `const_iterator` or not.

It is illegal to attempt to deference an invalid iterator and will either
trigger a `ZX_DEBUG_ASSERT` or undefined behavior, depending on the nature of
the build.

## Creating an iterator from an object using container::make_iterator()

Because of the intrusive nature of the containers, it is possible to create a
container iterator using an existing reference to an object. For example, given
a tree of objects ordered by key, a function that takes an object, and returns a
reference to the object before it, in key sequence, could be written by saying
something like:

```
using ObjectTree = fbl::WAVLTree<uint64_t, fbl::RefPtr<Object>>;

fbl::RefPtr<Object> FetchPrior(ObjectTree& tree, Object& obj) {
  ZX_DEBUG_ASSERT(obj.InContainer());
  auto iter = tree.make_iterator(obj);
  return (--iter).IsValid() ? iter.CopyPointer() : nullptr;
}
```

## iterator::IsValid()

All `fbl::` iterator instances may be tested for validity using the `IsValid`
method of the iterator instance itself. Testing an iterator for validity in this
way is equivalent to testing `iter != container.end()`, however the `IsValid`
approach may produce _slightly_ more efficient code, depending on how smart the
compiler is and how much visibility it has into the implementation of the
container's `end()` method..

## iterator::CopyPointer()

Finally, `fbl::` iterators provide a method called `CopyPointer`, which can be
used to produce a copy of the pointer type being used by the container. For
containers of raw pointers, this is nothing special. It is simply a `T*` copy of the
pointer to the object. In fact, `iter.CopyPointer() == &(*iter)` should always
be true for raw pointers.

`CopyPointer` is not legal for managed pointers with unique semantics. Attempting
to call `CopyPointer` on a container of objects tracked using `std::unique_ptr`
will produce an error.

Finally, when `CopyPointer` is executed on an iterator for a container of
copyable managed pointers, a new copy of the pointer will be produced using the
copy constructor of the pointer type. In other words, it will produce a new
managed reference to the object.

Attempting to call `CopyPointer` on an invalid iterator to a copyable pointer
type will produce `nullptr`.
