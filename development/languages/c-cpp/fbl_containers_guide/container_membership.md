# Controlling container membership

As noted in the introduction, for objects to exist in an intrusive container,
users must explicitly add storage for the container bookkeeping to the object
itself.  This section will show you the details of how you can control which
container(s) your object are allowed to exist in. It will:

1. Demonstrate the simple case of an object that may exist in a single
   container.
1. Show two ways to allow for membership in multiple containers simultaneously.
1. Show how to take complete manual control of bookkeeping storage in your
   object in the case that advanced requirements need to be satisfied.

## Single container membership using a mix-in.

Typically, choosing to use the default mix-in for a container is the easiest and
proper choice. You have already seen what this looks like for the doubly linked
list in the [Getting Started](getting_started.md) section of this guide. Here
are simple examples of all the default mix-ins.

```cpp
class FooObj : public fbl::SinglyLinkedListable<FooObj*> { /* ... */ };
using StackOfFoos = fbl::SinglyLinkedList<FooObj*>;

class FooObj : public fbl::DoublyLinkedListable<FooObj*> { /* ... */ };
using QueueOfFoos = fbl::DoublyLinkedList<FooObj*>;

class FooObj : public fbl::WAVLTreeContainable<FooObj*> { /* ... */ };
using MapOfIntToFoos = fbl::WAVLTree<int, FooObj*>;

// Hash tables default to singly linked list buckets
class FooObj : public fbl::SinglyLinkedListable<FooObj*> { /* ... */ };
using SLLHashOfIntToFoos = fbl::HashTable<int, FooObj*>;

// But you can use a doubly linked list as well.
class FooObj : public fbl::DoublyLinkedListable<FooObj*> { /* ... */ };
using DLLHashOfIntToFoos = fbl::HashTable<int, FooObj*,
                                          fbl::DoublyLinkedList<FooObj*>>;

```

In each of these examples, a raw pointer to a `FooObj` was used.  If you manage
your object using either `std::unique_ptr` or `fbl::RefPtr` semantics, they may
be substituted as needed provided that the pointer type in the mix in matches
the pointer type of the container.

Each of these objects can exist in a single _type_ of a container, but this does
not bind them to a single _instance_ of a container. For example:

```cpp
class Message : public fbl::DoublyLinkedListable<std::unique_ptr<Message>> { /* ... */ };

class TransmitQueue {
 public:
  // ...

  void SendMessage(Payload payload) {
    // Get a free message to send, or allocate if there are no free messages.
    std::unique_ptr<Message> tx;
    if (fbl::AutoLock lock(&lock_); free_messages_.is_empty()) {
      tx = std::make_unique<Message>();
    } else {
      tx = free_messages_.pop_front();
    }

    tx.PrepareMessage(std::move(payload));

    {
      fbl::AutoLock lock(&lock_);
      tx_pending_messages_.push_back(std::move(tx));
    }
    SignalTxThread();
  }

  // ...
 private:
  fbl::Mutex lock_;
  fbl::DoublyLinkedList<std::unique_ptr<Message>> free_messages_ TA_GUARDED(lock_);
  fbl::DoublyLinkedList<std::unique_ptr<Message>> tx_pending_messages_ TA_GUARDED(lock_);
};
```

`Message` objects in this example can exist in any instance of a
`DoublyLinkedList` of unique pointers to `Messages`, _but only in one of the
instances at a time_. In this example, a free-list of messages is maintained.
When the time comes to send a message:

1. A message is removed from the free list, or a new message is allocated if the
   free list is empty.
2. The payload of the message is prepared.
3. The message is placed into the pending queue.
4. Finally, the worker thread is poked to tell it that it has messages waiting
   in the pending queue to process.

When the worker is finished, it will move the `Message` object back to the free
list where it may be re-used, however that code has not been shown here.

## Multiple container membership using a multiple mix-ins

What if an object needs to exist in multiple containers at the same time? If
the container types themselves are different, then it is possible to simply use
multiple default mix-ins. Simply add the mix-ins to the list of base classes
for your object. For example:

```cpp
class FooObj : public fbl::RefCounted<FooObj>,
               public fbl::DoublyLinkedListable<fbl::RefPtr<FooObj>>,
               public fbl::WAVLTreeContainable<fbl::RefPtr<FooObj>> { /* ... */ };

using UniqueId = uint64_t;
static fbl::WAVLTree<UniqueId, fbl::RefPtr<FooObj>> g_active_foos;
static fbl::DoublyLinkedList<fbl::RefPtr<FooObj>> g_process_pending;

zx_status_t ProcessFooWithId(UniqueId id) {
  if (auto iter = g_active_foos.find(unique_id); iter.IsValid()) {
    if ((*iter).DoublyLinkedListable<fbl::RefPtr<FooObj>>::InContainer()) {
      return ZX_ERR_BAD_STATE;
    }
    g_process_pending.push_back(iter.CopyPointer());
    PokeWorkerThread();
  } else {
    return ZX_ERR_NOT_FOUND;
  }
  return ZX_OK;
}
```

In this example, a tree of all the active `FooObj`s in the system is being
maintained.  The objects in the tree are indexed by their `UniqueId` (which is
just a big integer in this case). There is also a queue of `FooObj`s waiting
to be processed. The `ProcessFooWithId` function attempts to find the Foo with
the specified ID and put a reference to it in the `g_process_pending` queue.

Note that when an object is found in the set of active objects, it is checked to
make sure it is not _already_ in the pending queue before attempting to append
it to the pending queue. `FooObj`s can exist in both the pending queue and the
active set at the same time, but they cannot exist in the pending queue twice.
Attempting to put an object into an instance of `ContainerTypeA` when the
object is _already_ in an instance of `ContainerTypeA` (either the same instance
or a different one) will trigger a ZX_DEBUG_ASSERT if asserts are enabled, or
end up corrupting the program state otherwise. It is *very important* to make
sure that this does not happen. Frequently, invariants in a program ensure that
this can never happen, but if your program lacks such invariants, remember to
check your object to see if it is in a container already or not.

See the section on [testing for container membership](membership_tests.md)
for various ways this can be done. Also note how ugly testing for membership is
in this example. There are nicer ways to do this as you will see in the next
sub-section describing the use of `ContainableBaseClasses`.

One last thing to point out about this example. When it is time to put a
`FooObj` into the pending queue, a new instance of a `fbl::RefPtr` to the object
instance needs to be provided to `push_back`. This can be obtained by calling
the `CopyPointer` method of the iterator, which will invoke the copy constructor
of the underlying pointer type, giving us a new reference to the object.
For raw pointers, this is a no-op. For unique_ptrs, this is illegal
and will fail to compile.

## Multiple container membership using ContainableBaseClasses

What should you do if your object needs to exist in multiple containers of the
same fundamental type at the same time? The easiest thing that can be done is to
make use of `fbl::ContainableBaseClasses`, along with type tags, which can be
used to identify the different containers your object can exist in. Here is a
re-implementation of the previous example, but this time with the addition of
another list that the objects can exist in.

```cpp
struct ActiveTag {};
struct ProcessPendingTag {};
struct OtherListTag {};

class FooObj :
  public fbl::RefCounted<FooObj>,
  public fbl::ContainableBaseClasses<
    fbl::TaggedDoublyLinkedListable<fbl::RefPtr<FooObj>, ProcessPendingTag>,
    fbl::TaggedDoublyLinkedListable<fbl::RefPtr<FooObj>, OtherListTag>,
    fbl::TaggedWAVLTreeContainable<fbl::RefPtr<FooObj>, ActiveTag>> { /* ... */ };

using UniqueId = uint64_t;
static fbl::TaggedWAVLTree<UniqueId, fbl::RefPtr<FooObj>, ActiveTag> g_active_foos;
static fbl::TaggedDoublyLinkedList<fbl::RefPtr<FooObj>, OtherListTag> g_process_pending_foos;

zx_status_t ProcessFooWithId(UniqueId id) {
  if (auto iter = g_active_foos.find(unique_id); iter.IsValid()) {
    if (fbl::InContainer<ProcessPendingTag>(*iter)) {
      return ZX_ERR_BAD_STATE;
    }

    iter->SetPriority(fbl::InContainer<OtherTag>(*iter) ? 20 : 10);

    g_process_pending_foos.push_back(iter.CopyPointer());
  } else {
    return ZX_ERR_NOT_FOUND;
  }
}
```

The example starts by defining 3 different types ("tags") that will be used to
identify the different containers to be used concurrently with `FooObj`s. These
types don't actually _do_ anything, they are simply empty structures. You will
never instantiate any of them. Their purpose is only to be a unique type
that the compiler can use to understand which list type is paired with which
node state.  In this example, the node state held by the
`TaggedDoublyLinkedListable` with the `ProcessPendingTag` is the node state used
by the `g_process_pending_foos` list.

Note that this can make the `InContainer` test easier to read as well. Using tags
allows us to invoke the stand-alone `fbl::InContainer<>` function, passing a
`const&` to an object, and specifying which type of container the object should
be tested for membership in using the tag.

`ContainableBaseClasses` works with any combination of containable mix-ins and
allows objects to exist in an arbitrary number of container types, provided that
each container type has a unique type to serve as its tag.

## Avoid mixing ContainableBaseClasses with default mix-ins

While _technically_ `ContainableBaseClasses` can be used in combination with the
default mix-ins, doing so is not considered best practice and should be avoided.

While there is clearly some extra typing involved in starting to use tags and
`ContainableBaseClases` to manage your object's container membership, once you
have started to do so it is easy to extend the pattern. The consistency of
always using tags with an given object (vs. sometimes using them and sometimes
not) will help with both readability and maintainability, particularly when it
comes to testing for container membership, and when understanding which
container type definitions use which piece of node storage in an object.

So, don't do this:

```cpp
namespace FooTags {
struct SortByBase {};
struct SortBySize {};
}

class Foo :
  public DoublyLinkedListable<Foo*>,  // For the pending processing queue
  public fbl::ContainableBaseClasses<
    public TaggedWAVLTreeContainable<Foo*, FooTags::SortByBase>,
    public TaggedWAVLTreeContainable<Foo*, FooTags::SortBySize>> { /* ... */ };
```

Instead do something more like this:

```cpp
namespace FooTags {
struct PendingProcessing {};
struct SortByBase {};
struct SortBySize {};
}

class Foo :
  public fbl::ContainableBaseClasses<
    public TaggedDoublyLinkedListable<Foo*, FooTags::PendingProcessing>,
    public TaggedWAVLTreeContainable<Foo*, FooTags::SortByBase>,
    public TaggedWAVLTreeContainable<Foo*, FooTags::SortBySize>> { /* ... */ };
```

## Container membership using explicit nodes and custom traits

Finally, there is one last option for controlling container membership for objects.
This option is the lowest level option, and the most work to write, understand,
and maintain. It only should be used in situations where specific technical
requirements force you to do so. Here are some of the reasons that might
justify the use of explicit nodes and custom traits in order to control
container membership for your object.

* Your object must have a C++ standard layout and therefore cannot inherit from
  any of the mix-ins.
* You must have precise control of _where_ in your object the node storage
  exists, and cannot have it end up in the storage allocated by the compiler for
  base classes.
* Your object is part of a complicated class hierarchy, where different levels of
  the hierarchy each need to be containable in different containers. Use of the
  mix-in helpers at the various levels would produce ambiguity and conflicts
  because of inheritance.

Every basic container type has a `NodeState` type associated with it. Not
surprisingly, their names are:

* `SinglyLinkedListNodeState<PtrType>`
* `DoublyLinkedListNodeState<PtrType>`
* `WAVLTreeNodeState<PtrType>`

These are the structures that hold the actual bookkeeping used by the
container's data structure.  In order to make use of them, you will need to:

1. [Add the appropriate instances of the node state types to your object.](#adding-node-state)
2. [Define a trait class, which will be used by containers in order access the
   bookkeeping.](#defining-node-state-trait-accessors)
3. [Define a container type, specifying the appropriate trait class to link the
   container type to the bookkeeping in your class it is supposed to make use
   of.](#defining-container-types)

Here is an example of an object that can exist in two doubly linked lists using
explicit nodes and custom traits:

```cpp
class Obj {
 public:
  // Obj impl here

 private:
  struct FooListTraits {
    static auto& node_state(Obj& obj) {
      return obj.foo_list_node_;
    }
  };

  struct BarListTraits {
    static auto& node_state(Obj& obj) {
      return obj.bar_list_node_;
    }
  };

  friend struct FooListTraits;
  friend struct BarListTraits;

  fbl::DoublyLinkedListNodeState<Obj*> foo_list_node_;
  fbl::DoublyLinkedListNodeState<fbl::RefPtr<Obj>> bar_list_node_;

 public:
  using FooList = fbl::DoublyLinkedListCustomTraits<Obj*, FooListTraits>;
  using BarList = fbl::DoublyLinkedListCustomTraits<fbl::RefPtr<Obj>, BarListTraits>;
};
```

### Adding the node state bookkeeping {#adding-node-state}

These lines declare the storage required for `Obj` to exist in two different
doubly linked lists at the same time.

```cpp
  fbl::DoublyLinkedListNodeState<Obj*> foo_list_node_;
  fbl::DoublyLinkedListNodeState<fbl::RefPtr<Obj>> bar_list_node_;
```

The pointer type used for tracking needs to be specified for the node and needs
to match that of the container.  In this example, `foo_list_node_` is a node
state object that can be used by lists to track their objects using raw
pointers, while `bar_list_node_` is a node state object that can be used by
lists to track their objects using `fbl::RefPtr<>`s. It is best practice to
make these node state objects private members of your class.

### Defining node state trait classes {#defining-node-state-trait-accessors}

These lines declare two "trait" classes used to tell a container type
how to access their associated node bookkeeping.

```cpp
  struct FooListTraits {
    static auto& node_state(Obj& obj) {
      return obj.foo_list_node_;
    }
  };

  struct BarListTraits {
    static auto& node_state(Obj& obj) {
      return obj.bar_list_node_;
    }
  };
```

These classes have no member variable or methods, merely a single static method
named `node_state`, which takes a mutable reference to your object type, and
returns a mutable reference to the proper node state bookkeeping instance in
your object.  These classes will never be instantiated, they only are used to
define, at compile time, the relationship of a type of a container to the proper
unit of bookkeeping storage in the object to be contained.

Note that, in keeping with best practice, our node state instances are private
members of `Obj`.  It is also best practice to make these trait classes private,
but because of the private nature of the node instances, you will need to also
declare the trait classes as friends of your object.  In this example, these
lines take care of that task.

```cpp
  friend struct FooListTraits;
  friend struct BarListTraits;
```

### Defining container type and specifying the node state storage they should use {#defining-container-types}

Finally, you will need to define the container types that may be used with your
object and make those types available to the users of your object.  In this
example, these are the lines that take care of that task.

```cpp
 public:
  using FooList = fbl::DoublyLinkedListCustomTraits<Obj*, FooListTraits>;
  using BarList = fbl::DoublyLinkedListCustomTraits<fbl::RefPtr<Obj>, BarListTraits>;
```

Note that we have used one of the specialized `using` aliases for
`DoublyLinkedList`, specifically `DoublyLinkedListCustomTraits`.  This alias
simple re-arranges the ordering of template parameters so that the second
parameter passed to the list type defines the trait class that will be used by
the list to find the appropriate node state bookkeeping storage.

Both the trait classes and the node state storage are private members of `Obj`,
so it is important that there be a public definition of the container types
available to the user of the object.  This will allow them to say things like
the following.

```cpp
Obj obj_instance;
Obj::FooList list;
list.push_back(&obj_instance);
```
