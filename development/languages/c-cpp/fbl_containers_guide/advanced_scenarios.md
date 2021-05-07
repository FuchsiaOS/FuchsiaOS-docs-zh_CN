# Advanced scenarios

By default, most of the behavior of intrusive containers in `fbl::` are designed
to provide as much safety as possible at compile time, typically by disallowing
use patterns that might easily lead to mistakes at compile time.

That said, there are certain advanced scenarios where a user may choose to
bypass these compile time safeties in order to deliberately permit certain
behaviors.  Whenever you choose to use one of these options, make sure that you
have thought carefully about the implications of taking the safety off, and be
sure that you are using the functionality in a safe way.

This section of the guide will show you how to:

1. [Use `fbl::NodeOptions` to opt into advanced behaviors](#node-options)
2. [Control the copy/move-ability of objects while they are in a container.](#copy-move-behavior)
3. [Permit an object tracked by `unique_ptr` to be contained in multiple container types](#multiple-unique)
4. [Clear a container of raw pointers in O(1) time](#clear-unsafe)
5. [Remove an object from a container without a reference to the container](#direct-remove)

## Controlling advanced options with `fbl::NodeOptions` {#node-options}

In order to control some advanced options at compile time, node state objects
(as well as their associated mix-ins) can take a bit-flag style constant, which
can be used to change specific behaviors. Options may be combined using the `|`
operator. By default, options are the second template parameter of either a
`NodeState` or `Listable`/`Containable` type, and the third template parameter
of a `TaggedListable`/`TaggedContainable` mix-in. These options always default
to `fbl::NodeOptions::None`. The syntax looks like this:

```cpp
class SimpleObject :
  public fbl::DoublyLinkedListable<SimpleObject*, fbl::NodeOption::OptionFoo> { /* ... */ };

class MoreComplexObject :
  public fbl::ContainableBaseClasses<
    fbl::TaggedSinglyLinkedListable<MoreComplex*, Tag1, fbl::NodeOption::OptionBar>,
    fbl::TaggedWAVLTreeContainable <MoreComplex*, Tag2,
                                    fbl::NodeOption::OptionA | fbl::NodeOption::OptionB>> {
  // ...
};

class ExplicitNodesObject {
 public:
  // ...

 private:
  // ...
  static constexpr fbl::NodeOptions kOptions = fbl::NodeOption::OptionX |
                                               fbl::NodeOption::OptionY |
                                               fbl::NodeOption::OptionZ;
  fbl::WAVLTreeNodeState<ExplicitNodesObject*, kOptions> wavl_node_state_;
};
```

## Controlling the copy/move behavior of objects while they are in a container {#copy-move-behavior}

Copying or moving node state while its object is in a container is not a legal
operation can cannot be permitted. Consider the following:

```cpp
fbl::DoublyLinkedList<Obj*> the_list;
ASSERT(!the_list.is_empty());

Obj the_obj;
the_list.insert_after(the_list.begin());

Obj another_obj{the_obj};

Obj yet_another_object;
the_obj = yet_another_object;
```

`the_obj` exists in the list after the first node. If you were to allow the node
state to be copied into `another_obj` via the default copy constructor, you would
have two objects with two copies of the bookkeeping. `another_obj` would
incorrectly think that it was in the container, and will now attempt to assert
when destroyed.

Worse, if you attempt to remove the object by calling
`the_list.erase(another_object)`, you are attempting to erase an object from a
container in an incoherent state. In this case, the prev and next pointers of
another object point to the first object in the list, the object that used to
be after `begin()` at the start of the example, but the next pointer of
`*begin()` is pointing to `the_obj`, and likewise for the prev pointer of the
next object in the sequence. While the specific behavior will vary on the type
of container and the specific implementation of erase, what is going to happen
is undefined behavior and cannot end well.

Finally, when the example code assigns the newly stack constructed
`yet_another_object` to `the_obj`, if the node state data were to be copied from
`yet_another_object`, `the_obj` would suddenly think it is not in the list even
though the objects on either side of it held pointers to it.

Any way you look at it, permitting the node state data to be copied will corrupt
the container's structure and cannot be allowed to happen, and the same goes for
the definition of most move operations.

To prevent mistakes like this, the default behavior of `fbl::` node state
objects is to disallow copy construction/assignment as well as move
construction/assignment. Any attempt to invoke the copy/move
constructor/assignment operator will result in a `static_assert` and a failure
to compile.

What about when objects are not in a container? Shouldn't copy/move be
permitted then? The short answer is sure, but in the interest of safety, it is
considered to be allowed only if the code author has opted into the behavior.
In order to opt in, you may use the following `NodeOptions` with their mix-in
or node storage types.

* `fbl::NodeOptions::AllowCopy`
* `fbl::NodeOptions::AllowMove`
* `fbl::NodeOptions::AllowCopyAndMove`

Setting `AllowCopy` will permit copy (l-value) construction and assignment,
while setting `AllowMove` will permit move (r-value) construction and
assignment. `AllowCopyAndMove` is simply shorthand for the two of them
combined.

During the operation itself, the node state object will `ZX_DEBUG_ASSERT` that
neither the source object is not in a container for construction and that
neither the source nor the destination object exist in containers during
assignment. Regardless of whether or not `ZX_DEBUG_ASERT`s are enabled, the
source and destination objects' states will *never* be modified.

For example:

```cpp
struct Point : fbl::DoublyLinkedListable<std::unique_ptr<Point>,
                                         fbl::NodeOptions::AllowCopy> {
  float x, y;
};

fbl::DoublyLinkedList<std::unique_ptr<Point>> all_points;

void AddCopy(const Point& pt) {
  // If pt is in a list, this will assert. If asserts are off, pt will remain
  // where it is and new_pt will not start life in a container.
  auto new_pt = std::make_unique<Point>(pt);
  all_points.push_back(std::move(new_pt));
}
```

So, what if you want to permit copying or moving of an object even while it is
_in_ a container? For example, what if you wanted to clone your list of points
making use of your copy constructor in order to do so? Users may opt into this
behavior as well by passing appropriate combinations of the following:

* `fbl::NodeOptions::AllowCopyFromContainer`
* `fbl::NodeOptions::AllowMoveFromContainer`
* `fbl::NodeOptions::AllowCopyAndMoveFromContainer`

The behavior described above remains the same; node states will never be
changed. New objects will start outside of any container and source objects will
remain wherever they are. The only difference between the `FromContainer`
version of this option vs the non-`FromContainer` version is that the
`FromContainer` version will never assert. So, you could clone your list of
points with the following.

```cpp
struct Point : fbl::DoublyLinkedListable<std::unique_ptr<Point>,
                                         fbl::NodeOptions::AllowCopyFromContainer> {
  float x, y;
};

using PointList = fbl::DoublyLinkedList<std::unique_ptr<Point>>;

PointList CloneList(const PointList& list) {
  PointList ret;
  for (const auto& point : list) {
    ret.push_back(std::make_unique<Point>(point));
  }
  return ret;
}
```

## Allowing objects tracked by `unique_ptr` to exist in multiple containers {#multiple-unique}

Usually, it would be a mistake to define an object that can exist in
multiple containers concurrently, while tracking those objects in their
containers using `unique_ptr` semantics. In theory, it should be impossible
for two different containers to track the same object at the same time,
each using something like a `unique_ptr` as this would violate the uniqueness of
the pointer.

To assist in preventing any mistakes here, `ContainableBaseClasses` will not
permit the use of a `std::unique_ptr` pointer type for any of the specified
mix-ins, unless the length of the list of containable base classes is exactly 1.

```cpp
struct Tag1 {};
struct Tag2 {};

// This is legal
class Obj : public fbl::ContainableBaseClasses<
  fbl::TaggedSinglyLinkedListable<std::unique_ptr<Obj>, Tag1>> { /* ... */ };

// This is not
class Obj : public fbl::ContainableBaseClasses<
  fbl::TaggedSinglyLinkedListable<std::unique_ptr<Obj>, Tag1>,
  fbl::TaggedSinglyLinkedListable<std::unique_ptr<Obj>, Tag2>> { /* ... */ };

// Neither is this
class Obj : public fbl::ContainableBaseClasses<
  fbl::TaggedSinglyLinkedListable<std::unique_ptr<Obj>, Tag1>,
  fbl::TaggedSinglyLinkedListable<Obj*, Tag2>> { /* ... */ };
```

There are, however, legitimate uses for types that can exist in multiple
containers, which are managed using `std::unique_ptr`s.

First, you may have a situation where an object can exist in two different types
of data structure (perhaps a list and a tree), but never the same data structure
at the same time. If the uses of the structure are completely disjoint, you may
wish to relax the default restriction.

A second reason you might want to allow this is because you have an object whose
life is tracked by a container using `std::unique_ptr`, but for which you want
to allow objects to exist in containers on a temporary basis in order to more
easily implement some sort of algorithm. Perhaps a set of objects needs to be
filtered into a temporary list and then passed to a function that will operate
on the filtered set.  Or, perhaps they need to be placed into a temporary
WAVLTree with a custom sorting/keys in order to check for duplicates.

Whatever the reason, you can permit this behavior by passing the
`AllowMultiContainerUptr` option to the node state types that you use. Here is
an example for the disjoint container use case:

```cpp
struct FreeObjTag {};
struct ActiveObjTag {};
class Obj : public fbl::ContainableBaseClasses<
  fbl::TaggedSinglyLinkedListable<std::unique_ptr<Obj>, FreeObjTag,
                                  fbl::NodeOptions::AllowMultiContainerUptr>,
  fbl::TaggedWAVLTreeContainable<std::unique_ptr<Obj>, ActiveObjTag,
                                 fbl::NodeOptions::AllowMultiContainerUptr>> {
 public:
  using FreeStack = fbl::TaggedSinglyLinkedList<std::unique_ptr<Obj>, FreeObjTag>;
  using ActiveSet = fbl::TaggedWAVLTree<UniqueId, std::unique_ptr<Obj>, ActiveObjTag>;

  // ...
  UniqueId GetKey() const { return unique_id_; }
  void AssignId(UniqueId id) {
    ZX_DEBUG_ASSERT(!fbl::InContainer<ActiveObjTag>(*this));
    unique_id_ = id;
  }
  // ...

 private:
  // ...
};

fbl::Mutex obj_lock_;
Obj::FreeStack free_objects_ TA_GUARDED(obj_lock_);
Obj::ActiveSet active_objects_ TA_GUARDED(obj_lock_);

zx_status_t ActivateObject(UniqueId id) {
  fbl::AutoLock lock(&obj_lock_);

  if (free_objects_.is_empty()) {
    return ZX_ERR_NO_MEMORY;
  }

  auto ptr = free_objects_.pop_front();
  ptr.AssignId(id);
  active_objects_.insert(std::move(ptr));
  return ZX_OK;
}
```

## Allowing O(1) clearing of containers with `clear_unsafe()` {#clear-unsafe}

As noted in the [Lifecycle checks](#lifecycle-checks) section, containers of
unmanaged pointers may not destruct with objects still in them, and objects
cannot destruct while they think that they are still in a container. Either
behavior is considered to be an error and will trigger an assert in a debug
build.

What if you had a situation where you didn't care that your objects still
thought that they were in a container when they destructed? Perhaps you
allocated a contiguous slab of memory and carved it up into object that you
then placed onto a free list. If you want to free your slab of memory, and you
know that all of the objects have been returned to the free list, then why
bother walking the list to zero out all of the linked list bookkeeping? This
would just be wasted work.

You can bypass these checks and skip the mandatory O(N) unlinking of the list by
using the `AllowClearUnsafe` NodeOption on your objects. When used, the asserts
present in the node state object are skipped, and a method on the container
called `clear_unsafe()` becomes available for use. `clear_unsafe()` will simply
reset the container to its original empty state making no effort to clean up
the objects' node states. This is a simple O(1) operation. Attempting to call
`clear_unsafe()` on a container that uses a node state object without this flag
will trigger a `static_assert`. Here is a simple example of what this would
look like:

```cpp
class SlabObject :
  public fbl::SinglyLinkedListable<SlabObject*,
                                   fbl::NodeOptions::AllowClearUnsafe> { /* ... */ };

static fbl::Mutex slab_lock;
static SlabObject* slab_memory TA_GUARDED(slab_lock) = nullptr;
static fbl::SizedSinglyLinkedList<SlabObject*> TA_GUARDED(slab_lock) free_objects;

static constexpr size_t kSlabSize = (1 << 20);   // One MB of objects
static constexpr size_t kSlabCount = kSlabSize / sizeof(SlabObject);

zx_status_t InitSlab() {
  fbl::AutoLock lock(&slab_lock);
  if ((slab_memory != nullptr) || !free_objects.is_empty()) {
    return ZX_ERR_BAD_STATE;
  }

  fbl::AllocChecker ac;
  slab_memory = new (&ac) SlabObject[kSlabCount];
  if (!ac.check()) {
    return ZX_ERR_NO_MEMORY;
  }

  for (size_t i = 0; i < kSlabCount; ++i) {
    free_objects.push_front(slab_memory + i);
  }

  return ZX_OK;
}

SlabObject* GetFreeObj() {
  fbl::AutoLock lock(&slab_lock);
  return !free_objects.is_empty() ? free_objects.pop_front() : nullptr;
}

void ReturnObj(SlabObject* obj) {
  fbl::AutoLock lock(&slab_lock);
  ZX_DEBUG_ASSERT(obj != nullptr);
  free_objects.push_front(obj);
}

zx_status_t DeinitSlab() {
  fbl::AutoLock lock(&slab_lock);

  // If not all of our objects have returned, or if we don't have any slab
  // memory allocated, then we cannot de-init our slab.
  if ((slab_memory == nullptr) || (free_objects.size() != kSlabCount)) {
    return ZX_ERR_BAD_STATE;
  }

  // Great, reset the free list with clear unsafe. This basically just sets the
  // head pointer to nullptr.
  free_objects.clear_unsafe();

  // Now give our memory back. Since our objects are flagged with
  // AllowClearUnsafe, node state destructors do nothing. Provided that
  // SlabObject destructors do nothing, this delete should just return memory to
  // the heap and not need to call N destructors.
  delete[] free_objects;
  free_objects = nullptr;

  return ZX_OK;
}
```

## Directly removing objects from whatever container instance they are in. {#direct-remove}

In general, even though it is sometimes possible, it is not considered best
practice to design code that needs to remove objects directly from a container
without having a reference to the container itself. As a design principle, users
of intrusive containers should always be aware of which container types and
instances objects exist in at all times. Still, sometimes direct removal might
be the easiest and best option available.

Containers that track size by default might require an O(n) traversal of the
data structure in order to find the container to update the bookkeeping if nodes
are removed from the container without knowledge of the container instance.
Therefore, these containers do not support direct removal. Other container
types, such as a `SinglyLinkedList`, simply cannot do this as they lack a
back-pointer to their previous node.

It is, however,  possible for an unsized doubly linked list to support direct
node removal. To enable this, add the `AllowRemoveFromContainer` to the node
state's `NodeOption`s. When enabled, node state structures will have a
`RemoveFromContainer()` method available. Calling RemoveFromContainer is
identical to calling InContainer. It may be called directly from the object if
there is no ambiguity, using explicit types to select the container to remove
from when inheritance produces ambiguity, or using the top level
`fbl::RemoveFromContaier<Tag>(obj_ref)` call when using the
`ContainableBaseClasses` helper. See [`InContainer()`](membership_tests.md#single-container)

Consider the following use case. You have a bunch of jobs that need to be
processed by several stages of a pipeline. The pipeline stages each have a
queue of pending work that threads take jobs from, process, and then queue
to the next stage of a pipeline.

If you want to cancel a job while it is in flight, how do you easily know which
pipeline stage it is in? One answer might be that you don't need to, as long as
you can directly remove it from the processing stage it is currently in. This
might end up looking like the following:

```cpp
struct PipelineTag {};
struct ActiveTag {};

fbl::Mutex pipeline_lock;

class Job : public fbl::RefCounted<Job>,
            public fbl::ContainableBaseClasses<
              fbl::TaggedDoublyLinkedListable<fbl::RefPtr<Job>, PipelineTag,
                                              fbl::NodeOptions::AllowRemoveFromContainer>,
              fbl::TaggedWAVLTreeContainable<fbl::RefPtr<Job>, ActiveTag>> {
 public:
  // ...
  UniqueId GetKey() const { return unique_id_; }
  bool is_canceled() const TA_REQ(pipeline_lock) { return cancel_flag_; }
  void set_canceled() TA_REQ(pipeline_lock) { cancel_flag_ = true; }
  // ...
 private:
  bool cancel_flag_ TA_GUARDED(pipeline_lock) = false;
};

using PipelineQueue = fbl::TaggedDoublyLinkedList<fbl::RefPtr<Job>, PipelineTag>;
std::array<PipelineQueue, 10> pipeline_stages TA_GUARDED(pipeline_lock);
fbl::TaggedWAVLTree<fbl::RefPtr<Job>, ActiveTag> active_jobs TA_GUARDED(pipeline_lock);

zx_status_t QueueJob(fbl::RefPtr<Job> job) {
  ZX_DEBUG_ASSERT(job != nullptr);
  {
    fbl::AutoLock lock(&pipeline_lock);

    // Can't queue a job for processing if it is already being processed.
    if (fbl::InContainer<ActiveTag>(*job)) {
      return ZX_ERR_BAD_STATE;
    }

    // If we are not in the active set, then we had better not be in any of the
    // pipeline stages.
    ZX_DEBUG_ASSERT(!fbl::InContainer<PipelineTag>(*job));

    // Put the job into the active set and into the first pipeline stage.
    active_jobs.insert(job);
    pipeline_stages[0].push_back(std::move(job));
  }

  SignalPipelineStage(0);
}

void WorkThread(size_t pipeline_stage) {
  ZX_DEBUG_ASSERT(pipeline_stage < pipeline_stages.size());
  PipelineQueue& our_stage = pipeline_stages[pipeline_stage];
  PipelineQueue* next_stage = ((pipeline_stage + 1) < pipeline_stages.size())
                            ? (pipeline_stages + pipeline_stage + 1)
                            : nullptr;

  while (!QuitTime()) {
    fbl::RefPtr<Job> job;
    {
      // If there is work in our stage, take it out and get to work.
      fbl::AutoLock lock(&pipeline_lock);
      if (!our_stage.is_empty()) {
        job = our_stage.pop_front();
      }
    }

    // Did we not find a job? Just wait for something to do then.
    if (job == nullptr) {
      WaitForPipelineStageWorkOrQuit(pipeline_stage);
      continue;
    }

    // Do the job.
    ProcessJob(job, pipeline_stage);

    // If the job was canceled or reached the end of the pipeline, we will call
    // a handler to take care of it once we are out of the lock.
    void(*handler)(fbl::RefPtr<Job>) = nullptr;
    {
      fbl::AutoLock lock(&pipeline_lock);

      if (job->is_canceled()) {
        // Handle job cancellation if it was flagged for cancel while we were
        // working. No need to take it out of the active set, the cancel
        // operation should have already done that for us.
        ZX_DEBUG_ASSERT(!fbl::InContainer<ActiveTag>(*job));
        handler = HandleCanceledJob;
      } else if (next_stage != nullptr) {
        // Queue to the next stage if there is one.
        next_stage->push_back(std::move(job));
        signal_next_stage = true;
      } else {
        // End of pipeline. This job is finished, remember to take it out of
        // the active set.
        ZX_DEBUG_ASSERT(fbl::InContainer<ActiveTag>(*job));
        active_jobs.erase(*job);
        handler = HandleFinishedJob;
      }
    }

    // Now that we are out of the lock, either signal the next stage so that it
    // knows that it might have some work, or call the chosen handler on the job.
    if (handler) {
      ZX_DEBUG_ASERT(job != nullptr);
      handler(std::move(job));
    } else {
      SignalPipelineStage(pipeline_stage + 1);
    }
  }
}

zx_status_t CancelJob(UniqueId id) {
  fbl::RefPtr<Job> canceled_job;
  {
    fbl::AutoLock lock(&pipeline_lock);

    // Is there an active job with the provided ID?
    auto iter = active_jobs.find(id);
    if (!iter.IsValid()) {
      return ZX_ERR_NOT_FOUND;
    }

    // No matter what, the job is no longer active. Take its reference back from
    // the active job set.
    fbl::RefPtr<Job> job = active_jobs.erase(iter);

    // Flag the job as canceled.
    job->set_canceled();

    // If the job is in a pipeline stage, then no thread is currently working on
    // it. We can just pull it out of whatever stage we are in and we are done.
    if (fbl::InContainer<PipelineTag>(*job)) {
      canceled_job = fbl::RemoveFromContainer<PipelineTag>(*job);
    }
  }

  // Now that we are out of the lock, if we were the ones to pull the job out of
  // the pipeline, we should hand it over to the cancel handler.
  HandleCanceledJob(std::move(canceled_job));
  return ZX_OK;
}
```
