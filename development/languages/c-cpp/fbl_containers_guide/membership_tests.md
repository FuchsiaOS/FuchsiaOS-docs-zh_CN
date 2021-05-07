# Testing an object for membership in a container

Because node storage is a property of the objects that exist in containers, the
objects themselves can be examined in order to check to see if the object is
currently contained with in a container or not. This check is always a quick
O(1) operation.

This section of the guide will show you various ways to test whether your object
are currently a member of any of the containers they may currently be contained
in.

## Testing single-container objects using mix-ins {#single-container}

All node state classes provide a public `InContainer()` method, which may be used
to test to see if the node state instance is currently a member of its
container, as do the `Listable` or `Containable` mix-ins that objects typically
derive from. Because the objects derive from the mix-in, they also expose this
method, so testing an object that uses mix-ins, and can only be a member of
single container, for container membership is as simple as calling
`InContainer()` on the object.

```cpp
class Obj : public fbl::SinglyLinkedLisable<Obj*> { /* ... */ };

Obj* the_obj = new Obj();
fbl::SinglyLinkedList<Obj*> the_list;
ASSERT(the_obj->InContainer() == false);

the_list.push_front(the_obj);
ASSERT(the_obj->InContainer() == true);

the_list.clear();
ASSERT(the_obj->InContainer() == false);

delete the_obj;
```

## Multiple-container objects using mix-ins, without `ContainableBaseClasses`

When an object can be a member of multiple containers because it derives from
multiple mix-ins, things can become a bit more complicated. The object itself
now has multiple implementations of `InContainer()`, which it inherited from the
mix-ins, so callers need to be specific about which one they want to call.
While this is certainly possible, the syntax is awkward, it may benefit from
being wrapped up in custom methods on the object. For example:

```cpp
class Obj : public fbl::DoublyLinkedListable<Obj*>,
            public fbl::WAVLTreeContainable<Obj*> {
 public:
   // ...
   bool InList() const { return this->DoublyLinkedListable<Obj*>::InContainer(); }
   bool InTree() const { return this->WAVLTreeContainable<Obj*>::InContainer(); }
   // ...
};

void test(const Obj& obj) {
  bool in_list, in_tree;

  // The hard way
  in_list = obj.DoublyLinkedListable<Obj*>::InContainer();
  in_tree = obj.WAVLTreeContainable<Obj*>::InContainer();

  // The slightly easier way (the class still needs to implement the hard way)
  in_list = obj.InList();
  in_tree = obj.InTree();
}
```

## Multiple container objects using mix-ins, with `ContainableBaseClasses`

Using the `ContainableBaseClasses` for existing in multiple containers
simultaneously can make this a lot easier as it allows tags to be used in order
to select the container that you want to test for membership. `fbl::` provides
a standalone `InContainer` function, which may be used  along with tags to more
easily test for membership. Let's look at the previous example, but this time
using `ContainableBaseClasses`.

```cpp
struct ListTag {};
struct TreeTag {};

class Obj
  : public fbl::ContainableBaseClasses<fbl::TaggedDoublyLinkedListable<Obj*, ListTag>,
                                       fbl::TaggedWAVLTreeContainable<Obj*, TreeTag>> { /* ... */ };

void test(const Obj& obj) {
  bool in_list = fbl::InContainer<ListTag>(obj);
  bool in_tree = fbl::InContainer<TreeTag>(obj);
}
```

## Testing by directly using the `NodeState` object

Finally, in the event that `NodeState` objects and custom traits are being used,
container membership can still be tested, but you need to ask the `NodeState`
instance directly.

```cpp
class Obj {
 public:
  // Obj impl here

  bool InFooList() const { return foo_list_node_.InContainer(); }
  bool InBarList() const { return bar_list_node_.InContainer(); }

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
