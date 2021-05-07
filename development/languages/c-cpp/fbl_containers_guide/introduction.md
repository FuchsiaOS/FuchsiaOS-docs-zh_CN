# Introduction to `fbl` intrusive containers

Intrusive containers (or intrusive data structures) are the type of containers
offered by the Fuchsia Base Library (`fbl::`).

This document will:

1. [Define what an intrusive container is.](#what-is-an-intrusive-container)
2. [Compare the differences between intrusive and non-intrusive containers.](#intrusive-vs-not)
3. [Discuss the similarities and differences between `fbl::` and `std::` containers.](#fbl-vs-std)
4. [Introduce some basic terminology that will be used throughout this guide.](#terminology)

## What is an intrusive container? {#what-is-an-intrusive-container}

An intrusive container is a data structure that can be used to hold a
collection of objects where the bookkeeping used to track membership in the
container is stored in the objects themselves instead of holding the object
alongside of the bookkeeping in a structure. To highlight the distinction,
consider a structure that defines a point in a 2-dimensional coordinate system.

```cpp
struct Point {
  float x, y;
};
```

In a traditional non-intrusive implementation of a doubly linked list, a node in
the list might look something like this

```cpp
struct ListNode {
  Point val;
  ListNode *next, *prev;
};
```

While an intrusive version of the same list would be

```cpp
struct Point {
  float x, y;
  Point *next, *prev;
};
```

Initially, these appear to be very similar approaches. The distinction seems
mostly like a question of which object is contained within the other. As time goes
on, however, the differences become more apparent.

Adding a point to a non-intrusive list of points means:

1. Allocating a `ListNode`
1. Copying the `Point` you want to add to the list into the `val` member of the `ListNode`
1. Linking the `next`/`prev` pointers of the `ListNode` into the list.

Removing the point is the same process in reverse.

1. The pointers are unlinked.
1. The contents of the `Point` are copied out to local storage.
1. Finally, the `ListNode` is de-allocated.

When using an intrusive list, the add/remove operations skip the
allocation/deallocation and copying of the `Point`'s members. The `next`/`prev`
pointers, which exist in the point object already, are simply linked into the
list during the add operation and unlinked during the remove. While the
intrusive form does not have to perform any allocations or copy any structures
to add elements to a list, every `Point` in the system is must carry around the
overhead of the next/prev bookkeeping, even while not in a list.

While a small structure or primitive data type might be held by value in a
non-intrusive list, when objects become larger, it becomes more common to track
the object using some appropriate pointer type instead of storing the object by
value. For example

```cpp
struct ListNode {
  std::shared_ptr<LargeObject> ptr;
  ListNode *next, *prev;
};
```

The "value" being stored in this list is an instance of a shared pointer, so
while adding or removing a pointer to a `LargeObject` to a list still requires
management of the allocation used to store the node state, the object itself no
longer needs to be copied when being added to a list. Additionally, this
approach allows a single instance of a `LargeObject` to exist in multiple
containers at the same time as the nodes simply track references to the objects
and not copies of the objects. A simple implementation of an intrusive form of
this would put the `next`/`prev` pointers into `LargeObject` directly implying
that an instance of a `LargeObject` could be a member of exactly 0 or 1
intrusive lists, but not more.

## Intrusive vs. non-intrusive containers {#intrusive-vs-not}

The primary advantage of an intrusive container approach to tracking objects is
the lack of allocations when adding/removing elements to/from a container. In
some (usually specialized) environments, heap interactions can introduce a
potential failure path, which would ideally not exist. Additionally,
interactions with heaps usually involve interactions with locks/mutexes, which
could result in involuntary blocking. This might introduce undesirable timing
indeterminism, or in some cases may even not be an option if the code is running
in a context in which blocking is not allowed.

For example, what happens if code needs to add a bookkeeping structure to a list
during a hard interrupt handler in the kernel if the allocation for the
bookkeeping structure cannot be allocated, or if allocation of the bookkeeping
would require the IRQ handler to block due to lock contention (something that
cannot happen in the exception handler)? By allocating the container
bookkeeping as part of the object itself ahead of time, an intrusive container
approach to this problem allows these issues to be avoided.

In general, intrusive containers can provide performance advantages when the
design of the system allows the implementer to know at compile time all of the
various containers that an object will ever exist in. This does not mean that they
are always the best choice, however, as they come with limitations as well.

Finally, for many types of intrusive containers, holding a reference to an
object implies that you have located that object in all of the container types
it can possible exist in where the same is not true when an object is tracked by
reference in multiple non-intrusive container instances. For example, an object
which could exist in a balanced binary search tree as well as in a doubly linked
list (both intrusive) could be located in O(log n) time in the tree, and then
removed from the list in O(1) time, instead of needing to find the object in the
list in O(n) time before removing it.

A short list of some pros and cons of the two approaches might look something
like this:

Non-intrusive:

- Pros

  - Can be used to track simple objects by value, even primitive data types
  like `int` or `float`

  - Does not require changing the definition of the object itself in order to
  manage it in a different container. This is especially helpful when an object
  `O` is defined in Library A, but a user wishes to create a collection of
  `A::O`s in their own program and cannot reasonably re-define `O`.

  - Objects tracked by reference can easily exist in multiple containers
  independently.

- Cons
  - Requires independent allocation management of the bookkeeping, usually on
  every add or remove operation. This management frequently implies hidden heap
  interactions, which can introduce overhead, timing uncertainty, and additional
  complexity as users need to manage potential failure paths.

  - Finding the location of an object in container A provides no information
  about the location of the object in container B. For example, finding an
  object contained in a `std::map` with an O(log n) operation does not
  help me to locate the same object that is simultaneously contained in a
  `std::deque`. If I want to remove the object from the `std::deque`, I must find
  it there with a separate O(n) operation first.

Intrusive:

- Pros

  - Minimal overhead to join or leave a container, and no chance of failure
  or involuntary yielding due to lock contention as the bookkeeping overhead was
  paid up-front during object creation.

  - Finding an instance of an object implicitly finds the instance of the
  object in all of the containers it can exist in.

  - Knowledge of whether an object is currently container in container type A
  is a property of the object and can always be tested (from the object) in O(1)
  time.

- Cons

  - The ability to be contained is a fundamental property of the object. In
  order to change the number or types of the containers an object may exist in,
  the definition of the object itself must be changed, and all of the code that
  depends on that definition must be re-compiled.

  - Only objects themselves can be tracked. Primitive data types do not have
  any place to store extra bookkeeping information and cannot be tracked in an
  intrusive fashion.

## `fbl::` vs. `std::` {#fbl-vs-std}

The C++ Standard Library offers a large set of implementation agnostic
non-intrusive containers. For example, `std::map` is an ordered associative
container, however there are no requirements that an implementation of the
standard library use any specific data structure in order to implement
`std::map`, only that the implementation chosen provides certain
guarantees for various container operations as specified by the standard, such
as worst case algorithmic complexities, or guarantees around iterator
invalidation.

If you need a non-intrusive container, this is where you should be looking.

While there have been draft proposals, the C++ standard library does _not_
currently offer any intrusive container implementations. Similar to `boost::`,
`fbl::` attempts to fill this gap, but in a much more limited/focused fashion.
The algorithms backing the types of containers types offered by `fbl::` are
explicit and embedded in the names of the container types themselves. The APIs
are inspired by and attempt to emulate those offered by `std::`, however there
are some differences as a result of the fundamental differences between
intrusive and non-intrusive containers. There are currently 3 primary container
types defined by `fbl::` and one composed container type. They are:

* `fbl::SinglyLinkedList<>`
* `fbl::DoublyLinkedList<>`
* `fbl::WAVLTree<>`
* `fbl::HashTable<>`

`fbl::HashTable<>` is considered to be a composed container type as it offers a
fixed bucket count hash table that resolves collision using chaining where the
bucket type may be either `fbl::SinglyLinkedList<>` or
`fbl::DoublyLinkedList<>`. Please refer to [Setting up build
dependencies](getting_started.md#build-dependencies) for details on which files
need to be included in order to use the various container types.

## Terminology {#terminology}

Here are some definitions of terms used throughout this
guide.

* Container : An object that tracks a collection of references to objects using
  a specific algorithm.
* Node or Node Storage : The set of data that provides the bookkeeping that
  allows an object to exist in a specific container type.
* Listable, Containable, Mix-in : An object that can be used as a base class to
  allow an object to be easily stored in a container of a specific type.
* Raw pointer : A non-managed `T*`-style pointer to an object.
