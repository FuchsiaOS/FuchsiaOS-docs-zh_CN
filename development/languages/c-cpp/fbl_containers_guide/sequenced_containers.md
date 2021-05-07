# Sequenced containers

`fbl::` offers two main families of containers, [sequenced containers](sequenced_containers.md)
and [associative containers](associative_containers.md)

Sequenced containers are containers where the enumeration order of elements is
determined by by how a user specifically added and removed elements to and from
the container. `fbl::` defines two types of sequenced containers.

* `SinglyLinkedList` is a sequenced container that supports forward-only
  iteration.
* `DoublyLinkedList` is a sequenced container that supports bi-directional
  iteration.

The main differences between how sequenced containers and associative containers
are used comes down to how elements are added to and removed from the container.
This section of the guide will show you how you can add and remove elements to
and from sequenced containers.

Please refer to [Setting up build
dependencies](getting_started.md#build-dependencies) for details on which files
need to be included in order to use the various container types.

## Adding elements to a sequenced container {#adding-elements}

`SinglyLinkedList` provides two methods for adding elements to the container.
They are:

1. [`push_front(Ptr)`](#pushing-elements)
2. [`insert_after(Iter, Ptr)`](#inserting-elements)

`DoublyLinkedList` supports these methods as well, but also adds the following
methods.

1. [`push_back(Ptr)`](#pushing-elements)
2. [`insert(Iter, Ptr)`](#inserting-elements)
3. [`splice(Iter, List)`](#splicing-sequences)

As always, it is an error to attempt to add an element to a container if the
node state used by that container type is already a member of a container.
When using managed pointer types, users may either give a container its own
reference to an object by providing the pointer instance by value, or may
transfer their reference to the container using `std::move`.

### Pushing elements into a sequenced container {#pushing-elements}

The push methods behave as expected, adding a new element and making it the new
front or or back element in the sequence (in other words, either the first or
the last element in the enumeration order). For example:

```cpp
struct Tag1 {};
struct Tag2 {};
class Obj : public fbl::RefCounted<Obj>,
            public fbl::ContainableBaseClasses<
              fbl::TaggedDoulbyLinkedListable<fbl::RefPtr<Obj>, Tag1>,
              fbl::TaggedDoulbyLinkedListable<fbl::RefPtr<Obj>, Tag2>
            > {
 public:
  explicit Obj(int val) : val_(val) {}
  int val() const { return val_; }
 private:
  const int val_;
};

TaggedDoulbyLinkedList<fbl::RefPtr<Obj>, Tag1> stack_like;
TaggedDoulbyLinkedList<fbl::RefPtr<Obj>, Tag2> queue_like;

for (int i = 0; i < 5; ++i) {
  fbl::RefPtr<Obj> obj_ref = fbl::AdoptRef(new Obj(i));
  stack_like.push_front(obj_ref);            // Copy our reference
  queue_like.push_back(std::move(obj_ref));  // Transfer our reference
}

// Prints "4 3 2 1 0 "
for (const auto& obj : stack_like) { printf("%d ", obj.val()); }
printf("\n");

// Prints "0 1 2 3 4 "
for (const auto& obj : queue_like) { printf("%d ", obj.val()); }
printf("\n");
```

### Inserting elements into a sequenced container {#inserting-elements}

`insert` and `insert_after` both insert an element into the container in a
position either immediately before (`insert`) or immediately after
(`insert_after`) the iterator. Either `begin()` or `end()` may be provided as
the iterator for `insert`, which is functionally equivalent to saying simply
`push_front` or `push_back`. It is an error to call `insert_after` with an
iterator that does not reference an element, therefor `insert_after` will only
accept a container's `begin()` when the container is non-empty, and will never
accept `end()`. Continuing the previous example:

```cpp
queue_like.insert(queue_like.begin(), fbl::MakeRefCounted<Obj>(100));
queue_like.insert(queue_like.end(), fbl::MakeRefCounted<Obj>(500));
for (auto iter = queue_like.begin(), iter != queue_like.end(); ++iter) {
  if (iter->val() == 2) {
    queue_like.insert(iter, fbl::MakeRefCounted<Obj>(200));
    queue_like.insert_after(iter, fbl::MakeRefCounted<Obj>(300));
    break;
  }
}

// Prints "100 0 1 200 2 300 3 4 500 "
for (const auto& obj : queue_like) { printf("%d ", obj.val()); }
printf("\n");
```

### Combining sequenced containers using `splice` {#splicing-sequences}

Finally, `splice` will take the contents of a provided list and splice them into
a position immediately before an iterator in another list. After finishing, the
source list will be empty, having transferred all of its elements to the
destination list. The source and destination lists *must* be different list
instances, but must also be the same type of list (e.g. they must use the same
node storage). `begin()` and `end()` are both valid targets in the destination
list. The former will prepend the elements from the source to the destination,
while the latter will append. Finishing the previous example:

```cpp
TaggedDoulbyLinkedList<fbl::RefPtr<Obj>, Tag2> tmp;

tmp.push_front(fbl::MakeRefCounted<Obj>(-1));
tmp.push_front(fbl::MakeRefCounted<Obj>(-2));
queue_like.splice(queue_like.begin(), tmp);

tmp.push_back(fbl::MakeRefCounted<Obj>(1000));
tmp.push_back(fbl::MakeRefCounted<Obj>(2000));
queue_like.splice(queue_like.end(), tmp);

tmp.push_back(fbl::MakeRefCounted<Obj>(50));
tmp.push_back(fbl::MakeRefCounted<Obj>(60));
for (auto iter = queue_like.begin(), iter != queue_like.end(); ++iter) {
  if (iter->val() == 300) {
    queue_like.splice(iter, tmp);
    break;
  }
}

// Prints "-2 -1 100 0 1 200 2 50 60 300 3 4 500 1000 2000 "
for (const auto& obj : queue_like) { printf("%d ", obj.val()); }
printf("\n");
```

## Removing elements from a sequenced container {#removing-elements}

`SinglyLinkedList` provides three methods for removing elements from a container.
They are:

* `pop_front()`
* `erase_next(Iter)`
* `clear()`

`DoublyLinkedList` supports these methods as well, but also adds the following
methods.

* `pop_back(Ptr)`
* `erase(Iter or Obj&)`

With the exception of `clear()`, all of these methods return a pointer of the
container's pointer type to the user, returning the user's reference to the
object (when using managed pointers) to the user in the process. In the event
that there is no element at the specified position, `nullptr` is returned
instead. In the specific case of `erase_next`, it is illegal to pass an invalid
iterator. The iterator *must* refer to at least some element in the container.
Finally, the erase operation works with either a iterator to an element, which
is a member of the list, or with a `T&` style reference to the object itself.
Objects do not have to be discovered using iterators in order to be directly
erased.

Continuing from the example stared in the previous section:

```cpp
// Remove the object with val "-2" and hold a reference to it in |removed|.
auto removed = queue_like.pop_front();

// Remove the object with val "2000" and drop the reference, allowing the object
// to destruct.
queue_like.pop_back();

// Begin refers to the "-1" element, so erase_next will remove the "100" element
queue_like.erase_next(queue_like.begin());

// remove all of the elements in the list that are not in ascending order,
// relative to the previous element. Hold a reference to element 200 as we pass
// it.
fbl::RefPtr<Obj> e200;
for (auto iter = queue_like.begin(); iter.IsValid(); ) {
  auto tmp = iter++;

  if (iter->IsValid() && (tmp->val() > iter->val())) {
    queue_like.erase(iter);
    iter = tmp;
  } else if (tmp->val() == 200) {
    e200 = tmp.CopyPointer();
  }
}

// List is now "-1 0 1 200 300 500 1000". Remove 200 from the list using the
// object reference we held instead of an iterator.
queue_like.erase(*e200);

// Finally, just clear the list.
queue_like.clear();
```
