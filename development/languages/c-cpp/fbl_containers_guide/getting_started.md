# Getting started

This section will show you a simple example of an intrusive container in use,
and spend a small amount of time discussing the specifics of some basic
operations. While subsequent sections of the guide will go into greater detail
on these concepts as well as a number of others, this section will give a basic
demonstration of:

1. [Pointer types](#pointer-types) and how they are used to control objects' lifecycles.
1. Containable "[mix-ins](#mix-ins)" and how they are used to allow an object to exist in a
   container.
1. [How to iterator](#iteration) over the elements in a `fbl::` intrusive container.
1. [How to remove elements](#removing-elements) from a `fbl::` intrusive container.
1. [Setting up build dependencies](#build-dependencies) from a `fbl::` intrusive container.

## A simple example

Let's start with a simple example and go from there.  In this example, an object
will be defined that can exist in a doubly linked list, tracked using
`std::unique_ptr<>`.  A list of these objects will be:

1. Populated
1. Iterated over to print a subset of the objects
1. Iterated over to remove a subset of the objects
1. Explicitly cleaned up

```cpp
#include <stdint.h>
#include <fbl/intrusive_double_list.h>

// An object that holds an immutable int and can exist on a doubly linked list
// managed using a std::unique_ptr
class MyObject : public fbl::DoublyLinkedListable<std::unique_ptr<MyObject>> {
 public:
  explicit MyObject(int val) : val_(val) {}

  int val() const { return val_; }

 private:
  const int val_;
};

extern void TakeThisObjectFromMe(std::unique_ptr<MyObject>);

void DoThings() {
  fbl::DoublyLinkedList<std::unique_ptr<MyObject>> list;

  // Add 100 random integers to our list.
  for (uint32_t i = 0; i < 100; ++i) {
    list.push_back(std::make_unique<MyObject>(rand()));
  }

  // print out any members of the list that are even
  for (const auto& obj : list) {
    if (!(obj.val() % 2)) {
      printf("Even Object %d\n", obj.val());
    }
  }

  // Remove any objects that are divisible by 7 and give them to someone else.
  for (auto iter = list.begin(); iter != list.end(); ) {
    auto consider = iter++;
    if (!(consider->val() % 7)) {
      TakeThisObjectFromMe(list.erase(consider));
    }
  }

  // Destroy the rest of the object by forcing the list to release its unique
  // references to the objects. We could also simply let the list leave the
  // scope of the function, which would do the same.
  list.clear();
}
```

## Pointer types {#pointer-types}

One of the first things to note here, and an important difference between
`std::` containers and `fbl::` containers, is that `fbl::` containers always
track objects using pointers. In particular, they are tracked using a set of
explicitly supported pointer types. When the type of a `fbl::` list is defined,
it is defined as a list of a specific pointer type to an object type. In this
example, a `std::unique_ptr<>` was chosen meaning that the unique reference to
the object may be held locally, or held by the list, but it cannot be held by
both at the same time. There are currently 3 different pointer types that
`fbl::` containers allow:

* `T*` : A raw pointer type with no managed RAII semantics.
* `std::unique_ptr<T, Deleter>` : The standard unique pointer type, either with
  a custom deleter, or with the default deleter.
* `fbl::RefPtr<T>` : A `fbl::` intrusive reference pointer. Basically `fbl::`'s
  intrusive version of a `std::shared_ptr<T>`.

All containers must be told which type of object they hold and which type of
pointer is being used to manage lifetime via the first template argument of the
container type. When managed pointer types are used, containers always take
ownership of a reference when the object is added to the container, and give
back ownership of that reference when the object is removed from the container.
The exception to this rule is `clear()`, which drops all references.

"Raw" pointers have no special ownership semantics, meaning that it is entirely
up to the user to ensure that objects are kept alive while in a container, and
that the are properly cleaned up when removed or cleared from a container.

## Listable/Containable Mix-in classes {#mix-ins}

If the object must have the next/prev pointers stored in it somewhere, where
does this node state live and how does the list find it? There are actually
several ways to control this behavior, however this example demonstrates
simplest possible approach. This is to derive from the `DoublyLinkedList`'s
`Listable` mix-in helper. Just like with the list itself, you need to inform the
mix-in of the object type as well as the pointer type via the first template
argument.  These types need to match the types given to the list definition
itself. By default, a list will go looking for an instance of its `Listable`
mix-in that the object derives from in order to find the node state it uses for
bookkeeping. All of this happens at compile time, there is no runtime overhead
involved in locating the node state in the object.

## Iteration {#iteration}

`fbl::` containers support both ranged-based for loop iteration, as well as the
more classic begin()/end() style of iteration. As with `std::` containers, when
enumerated using a range-based for, an l-value reference to the object will be
returned. It is important to remember to say either `auto&` or `const auto&`
(or even `MyObject&`) in the for loop and not simply `auto`, lest you end up
accidentally triggering the copy constructor of your object.

`fbl::` container iterators also support the standard pre-fix and post-fix forms
of the `++` operator, which you can see being used to optionally remove elements
from the list as you iterate. The post-fix form is used in this example so that
`consider` becomes the element that may chosen for removal, while `iter`
becomes a pointer to the next element in the list that needs to be considered.

Iterators act like unmanaged pointers to the elements while they are in the
list. The underlying object may be accessed in all the standard ways, using
either the `->` operator, or using the `(*iter).val()` style. A raw pointer can
even be fetched to the object by saying `&(*iter)`. Care should be taken with
iterators as they are functionally raw pointers to an object.  It is not advised
that you be store them for any length of time, especially when you are using modern
managed pointers to control the lifecycles of your objects.

## Removing elements {#removing-elements}

In this example, when elements are removed, they are removed using iterator
based erase. The result of the erase operation will be an r-value reference to
the _pointer type_ used by the list. The reference could have been taken back
and held in a local pointer instance, but in this case it was moved directly
into the call of some external function transferring the unique ownership of the
object from the list to the external function in the process.

`clear()` is another way to removes elements from a container, but it will drop
all of the element references, potentially destroying the objects they point to
in the process.

## Setting up build dependencies {#build-dependencies}

In order to make use of `fbl::` containers, users must be sure to both include
the proper header files for the containers they want to use, and to include a
dependency on the `fbl` library in their project's `BUILD.gn` file.

The required include files for each container type are:

| Container type            | Include statement                        |
|---------------------------|------------------------------------------|
| `fbl::SinglyLinkedList<>` | `#include <fbl/intrusive_single_list.h>` |
| `fbl::DoublyLinkedList<>` | `#include <fbl/intrusive_double_list.h>` |
| `fbl::WAVLTree<>`         | `#include <fbl/intrusive_wavl_tree.h>`   |
| `fbl::HashTable<>`        | `#include <fbl/intrusive_hash_table.h>`  |

Users also must add the library `//zircon/system/ulib/fbl` to either the `deps`
or `public_deps` section of their project's `BUILD.gn` file.  The reference
belongs in the `deps` section in the case that the user is writing an
executable, or using `fbl` only in the private portions of their library.  If
`fbl` is being used anywhere in the public headers of a user's library,
`public_deps` should be used instead.
