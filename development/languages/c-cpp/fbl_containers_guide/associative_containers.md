# Associative containers

In addition to the [sequenced containers](sequenced_containers.md) , `fbl::`
offers two types of "associative" container:

* `HashTable` is an unordered associative container.
* `WAVLTree` is an ordered associative container.

Associative containers associate each object in the container with a key whose
type and properties can be specified by the user. Elements can be found or
removed using this key, and in the case of the `WAVLTree`, the order of
enumeration will be determined by this key and its properties. Since
`HashTable` is an unordered container, the order of enumeration is not
guaranteed to be anything in particular.

Please refer to [Setting up build
dependencies](getting_started.md#build-dependencies) for details on which files
need to be included in order to use the various container types.

This section of the guide will show you how to

1. [Define keys for objects, and teach containers how to access them](#defining-keys)
2. [Control how keys are used by your container using trait classes](#key-traits)
3. [Add elements to your container](#adding-elements)
4. [Remove elements from your container](#removing-elements)
5. [Find elements in your container](#finding-elements)
6. [Handling collisions while adding elements](#handle-collisions)
7. [Update the key of an element while it is in a container](#update-key)

## Defining a key type and providing access to the container {#defining-keys}

Like node state, keys for objects held in associative containers are intrusive
properties of the object itself. They are not static members of the node state,
and do not technically require any extra storage in the object, they simply need
to be computable from the object in a consistent and deterministic fashion. The
easiest way to expose a key to a container is to allow the container to use the
default key traits, and to implement a GetKey function. Here is an example:

```cpp
class Obj : public fbl::WAVLTreeContainable<Obj*>  {
 public:
  explicit Obj(uint32_t awesomeness) : awesomeness_(awesomeness) {}
  uint32_t GetKey() const { return awesomeness_; }

 private:
  const uint32_t awesomeness_;
};

using ObjSortedByAwesomeness = fbl::WAVLTree<uint32_t, Obj*>;
```

Here, a container has been defined, which is keyed using 32 bit unsigned integers,
and which will end up holding a collection of `Obj`s sorted by ascending
awesomeness. There are rules that need to be obeyed.

1. The `GetKey` method needs to be `const`.
2. Whenever an object is in the associative container, the `GetKey` method must
   be consistent. Each time it is called, it needs to return the same value.

In this example, we guarantee that rule 2 is followed by making the
`awesomeness_` member of our object `const`.  While this is an easy way to prevent
mistakes, it is not always a practical approach for some code.

Keys may sometimes need to be mutable, see
[Updating the key of an object while it is in a container](#update-key) for
information on how to properly handle this pattern.

## Controlling key behavior with key traits {#key-traits}

By default, keys need to be copyable types with a defined `==` operator for a
`HashTable`, and defined `<` and `==` operators for a `WAVLTree`. When used in
an ordered container (the `WAVLTree`) the objects will be stored in ascending
order. What if you wanted to take more control of this? What if your type was
not copyable, or if you wanted to sort in descending instead of ascending order?

For this, you can use custom key traits class. As with custom node traits, you
need to define a struct or class that exposes three public static methods, which
can be used to fetch a key from an object, and to compare those keys. Here is
an example:

```cpp
struct Endpoint {
  // Move only, for some reason
  Endpoint() = default;
  Endpoint(const Endpoint&) = delete;
  Endpoint& operator=(const Endpoint&) = delete;
  Endpoint(Endpoint&&) = default;
  Endpoint& operator=(Endpoint&&) = default;

  uint32_t ipv4_addr_{0};
  uint16_t ipv4_port_{0};
};

class Obj : public WAVLTreeContainable<Obj*> {
 public:
  // ...

 private:
  struct EndpointKeyTraits {
    static const Endpoint& GetKey(const Obj& obj) const { return obj.ep_; }
    static bool LessThan(const Endpoint& ep1, const Endpoint& ep2) {
        return (ep1.ipv4_addr_ > ep2.ipv4_addr_) ||
               ((ep1.ipv4_addr_ == ep2.ipv4_addr_) &&
                (ep1.ipv4_port_ > ep2.ipv4_port_));
    }

    static bool EqualTo(const Endpoint& ep1, const Endpoint& ep2) {
        return (ep1.ipv4_addr_ == ep2.ipv4_addr_) &&
               (ep1.ipv4_port_ == ep2.ipv4_port_);
    }
  };

  friend struct EndpointKeyTraits;

  Endpoint ep_;

 public:
  using ByEndpointMap = fbl::WAVLTree<Endpoint, Obj*, EndpointKeyTraits>;
};

Obj::ByEndpointMap objs_by_descending_endpoint_;
```

The traits tell the container both where to find the key, as well as how to
order the keys. In this example, a constant reference to the `Endpoint` in
`GetKey` is returned instead of a copy, avoiding the issue that `Endpoint`s are
move-only.  Additionally, the sense of the `LessThan` operation is inverted,
resulting in a tree that holds objects in descending `Endpoint` order instead
of ascending. The `LessThan` and `EqualTo` operations defined by the user are
required to be deterministic, transitive, and commutative. In other words:

* Performing a comparison operation for the same pair of keys multiple times
  must always return the same results.
* `LessThan(A, B)` and `LessThan(B, C)` implies `LessThan(A, C)`
* `EqualTo(A, B)` and `EqualTo(B, C)` implies `EqualTo(A, C)`
* `EqualTo(A, B)` if-and-only-if `EqualTo(B, A)`
* `LessThan(A, B)` if-and-only-if (not `LessThan(B, A)`)
* not `LessThan(A, B)` if-and-only-if `EqualTo(B, A)` or `LessThan(B, A)`

## Adding elements to an associative container {#adding-elements}

Because associative containers do not maintain a specific sequence defined by a
user's operations, adding elements to an associative container is a more
straight-forward process. Simply call `insert` on the container passing the
appropriate type of pointer. As with the sequenced containers, managed pointers
may either be copied, or moved in order to transfer ownership of the pointer to
the container.

```cpp
class Obj : public fbl::RefCounted<Obj>,
            public fbl::WAVLTreeContainable<fbl::RefPtr<Obj>> {
 public:
  explicit Obj(int val) : val_(val) {}
  int val() const { return val_; }
  int GetKey() const { return val(); }
 private:
  const int val_;
};

fbl::WAVLTree<int, fbl::RefPtr<Obj>> objects;

for (int i = 0; i < 100; ++i) {
  fbl::RefPtr<Obj> obj_ref = fbl::MakeRefPtr<Obj>(i);
  if (i % 2) {
    // For odd numbered elements, move our reference directly into the
    // collection of objects.
    objects.insert(std::move(obj_ref));
  } else {
    // For even number elements, make a copy of our pointer (so, another
    // AddRef), and then give away our reference to someone else.
    objects.insert(obj_ref);
    TakeEvenElementReference(std::move(obj_ref));
  }
}
```

## Removing elements from an associative container {#removing-elements}

As noted earlier, elements in an associative container either have their
enumeration order determined by key (`WAVLTree`), or do not have a guaranteed
enumeration order (`HashTable`). They still have an enumeration order, however,
and therefore a `front()` and a `back()`. Because of this, the methods used to
remove elements from a `fbl::` associative collection are identical to those for
a `DoublyLinkedList`, just without the `erase_next`.

* `pop_front()`
* `pop_back()`
* `erase(Iter or Obj&)`
* `clear()`

As the usage example would be very similar to those of the
[sequenced containers](sequenced_containers.md#removing-elements),
they have been omitted for brevity.

## Finding elements in an associative container {#finding-elements}

Members of associative containers can be located by key when present in the
container using the `find(Key)` method. The computational complexity of the find
operation is O(log n) for the `WAVLTree`. Hashtables use linked list chaining
for conflict resolution, so their find operation is technically an O(n)
operation, however performance can approximate O(1) if there are enough buckets
relative to N, and the hash function is flat.

If no element with the desired key exists in the container, the value of `end()`
for the container will be returned instead.

```cpp
void PrintValueForKey(const SomeAssociativeContainerType& container, KeyType key) {
  auto iter = container.find(key);
  if (iter.IsValid()) {
    cout << "Found val (" << iter->val() << "for Key (" << key << ") in the container.\n";
  } else {
    cout << "Key (" << key << ") was not found in the container.\n";
  }
}
```

Because it is an ordered associative container, `WAVLTree` also supports
`upper_bound(Key)` and `lower_bound(Key)` operations, in keeping with the
`std::` APIs.

* `lower_bound(Key)` find the first element in the container with a key greater
  than or equal to the provided key.
* `upper_bound(Key)` find the first element in the container with a key strictly
  greater than the provided key.

```cpp
// Given a set of objects whose keys are initially
// "1 5 25 67 100"
fbl::WAVLTree<uint32_t, std::unique_ptr<Obj>> objects;
fbl::WAVLTree<uint32_t, std::unique_ptr<Obj>>::iterator iter;

iter = objects.lower_bound(5);    // iter->GetKey() == 5
iter = objects.upper_bound(5);    // iter->GetKey() == 25
iter = objects.lower_bound(26);   // iter->GetKey() == 67
iter = objects.upper_bound(26);   // iter->GetKey() == 67
iter = objects.lower_bound(100);  // iter->GetKey() == 100
iter = objects.upper_bound(100);  // (iter == objects.end()) && !iter.IsValid()
iter = objects.lower_bound(101);  // (iter == objects.end()) && !iter.IsValid()
```

## Handling collisions in associative containers {#handle-collisions}

Like `std::map`, associative containers in `fbl::` require that the set of keys
in the container be unique. No collisions are allowed. If a collision occurs
during an insert operation, it will trigger a `ZX_DEBUG_ASERT` or result in
undefined behavior of debug asserts are not enabled.

So, how can insertion in an environment where collisions _might_ take place be
handled? Two methods are provided that allow us to efficiently control the
behavior of insertion when a collision occurs. They are:

* `insert_or_find`
* `insert_or_replace`

If no collision occurs, then these are both the equivalent of a simple insert.
Otherwise, `insert_or_find` will not perform the insertion and can return an
iterator to the object that your insert collided with if desired.  The function
returns a bool, which indicates a successful insert (no collision) when true.

`insert_or_replace` will simply replace the element in the container and give
back to you a reference to the object you collided with, or a `nullptr` version
of your pointer type in the case of no collision. So, in code this looks like
the following:

```cpp
// Attempt to insert an object into the container, but do nothing in the case of
// a collision.
Obj* ptr = GetAnObj();
container.insert_or_find(ptr);

// Attempt to insert an object into the container.  In the case of collision,
// the reference stored in ptr will not be consumed.  Put it back and log a
// message about the collision.
ContainerType::iterator collided;
std::unique_ptr<Obj> ptr = GetAnObj();
if (!container.insert_or_find(std::move(ptr), &collided)) {
  ASSERT(ptr.get() != nullptr);
  ASSERT(collided.IsValid());
  printf("Collided with obj \"%s\" when trying to insert obj \"%s\"\n",
         collided->name(), ptr->name());
  PutAnObjBack(std::move(ptr));
}

// Attempt to insert an object into the container, replacing whatever was there
// before and letting the reference to the replaced object drop on the ground,
// destroying the object if this was a managed pointer and the last reference to
// the object.
std::unique_ptr<Obj> ptr = GetAnObj();
container.insert_or_replace(std::move(ptr));

// Attempt to insert an object into the container, recovering the object
// replaced in the case of a collision.
std::unique_ptr<Obj> ptr = GetAnObj();
std::unique_ptr<Obj> replaced = container.insert_or_replace(std::move(ptr));
if (replaced != nullptr) {
  // We collided and replaced the element we collided with. Put this object
  // back.
  PutAnObjBack(std::move(replaced));
}
```

## Updating the key of an object while it is in a container {#update-key}

In the [example](#defining-keys) given in the section about how to define the
key used by a container, an object's awesomeness was a constant property of the
object. The value of a key cannot be allowed to change while the key is present
in the container without violating the container's invariants and producing
undefined behavior. A key _can_ change, however while the object is not in its
container without any trouble. Let's update the previous example to allow
mutable awesomeness and see what it would look like to update the awesomeness of
an `Obj` in a container.

```cpp
class Obj : public fbl::WAVLTreeContainable<Obj*>  {
 public:
  explicit Obj(uint32_t awesomeness) : awesomeness_(awesomeness) {}
  uint32_t GetKey() const { return awesomeness; }

  void UpdateAwesomeness(uint32_t awesomeness) {
    ZX_DEBUG_ASSERT(!InContainer());
    awesomeness_ = awesomeness;
  }

 private:
  uint32_t awesomeness_;
};

fbl::WAVLTree<uint32_t, Obj*> all_objs;

void UpdateAwesomeness(Obj& obj, uint32_t new_val) {
  ZX_DEBUG_ASSERT(obj.InContainer());
  all_objs.erase(obj);
  UpdateAwesomeness(new_val);
  all_objs.insert(*obj);
}
```

Note that (to prevent accidents) the example asserts that an object being
updated is not in the container at the time that the value of the key is
changed. `InContainer` is implemented by the object's `WAVLTreeContainable`
mix-in and may be invoked from the `UpdateAwesomeness()` by simply saying
`InContainer()`.

Also that there is no need to find the object in the tree before removing it.
Given a reference to the object to be updated, it is already known
where the object is in the container's internal structure because of the
intrusive nature of the container.

