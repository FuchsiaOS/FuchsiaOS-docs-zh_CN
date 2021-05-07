# Runtime Lock Validation in Zircon and Fuchsia

## Introduction

Lock validation is a technique for checking the consistency of locking behavior
in a program to find potential deadlock hazards. This document discusses
relevant aspects of the static and dynamic approaches to lock validation and
presents the foundation for the runtime lock validation library available in
Zircon and Fuchsia.

## Background

Lock validation may be performed either statically or dynamically. The following
summarizes the important differences between static and dynamic approaches to
lock validation:

* When the validation is performed: compile time vs. run time.
* How effective the validator is at finding potential problems.
* What level of involvement is required by the programmer.
* The overhead cost of the validation itself.

### Static Validation

Static validation is typically performed at compile time by analyzing the call
graphs produced by the compiler or other source-level processor. With this
approach it is necessary to instrument the code and locking primitives with
annotations to inform the validator about which types represent locks and which
rules to apply (or not) to the code that uses the lock types.

The benefits of static validation include early detection of issues at build
time, deterministic validation results, and zero runtime overhead. This
combination of properties make it attractive to always enable static validation,
ensuring that locking issues are often caught before code makes it into the
build, without impacting the performance of the build artifacts.

Static validation also has some down sides. One problem is that static
validation requires correct, consistent application of a variety of annotations
to both locks and code to provide useful results. This can cause maintenance
issues unless diligent code review standards are implemented. Another issue is
that static validation has limited visibility and can be fooled by conditional
paths, dynamic dispatch, move semantics, and lock dependencies that span
compilation units.

### Dynamic Validation

Dynamic validation is performed online at runtime by observing the relationships
between locks as the code executes. With this approach it is generally
sufficient to instrument just the locking primitives and acquire/release
operations to provide the information required for validation.

The benefits of dynamic validation include simpler instrumentation (from the
user's perspective) and potentially greater visibility into the actual runtime
behavior of the program. This makes dynamic validation useful in large code
bases, where it may not be possible for static validation to see the full
set of possible lock interactions.

The main downsides of dynamic validation are runtime overhead and execution
coverage requirements. Because dynamic validation must track lock interactions
at runtime, each acquire and release incurs a non-zero execution cost to update
tracking data, in addition to the memory overhead of the tracking data itself.
Runtime tracking also has the consequence that code paths that are not executed
cannot be analyzed by the validator. This may increase the burden on the
developer and QA to ensure sufficient execution coverage if that is not already
a project requirement.


### Locking Ordering Invariant

The job of the lock validator is to determine whether or not the lock invariants
of the program hold. The primary invariant is the order between two or more
locks: all paths in a program that acquire two or more locks must do so in an
order consistent with every other path involving two or more of the same locks to
avoid the potential for deadlock. Environments that deal with hardware
interrupts, such as embedded systems and kernels, have an additional ordering
invariant to avoid interrupt-induced deadlocks. These invariants are illustrated
in the following subsections.

##### Basic Inversion

The simplest form of inversion occurs when a program has two locks that are
both acquired sequentially with inconsistent orders in different paths.

For example, a program with the locks **A** and **B** and code paths
**P<sub>1</sub>** and **P<sub>2</sub>** and the following behavior has the
potential for deadlock:

Path **P<sub>1</sub>** acquires and releases the locks in the sequence:

1. Acquire(**A**)
2. Acquire(**B**)
3. Release(**B**)
4. Release(**A**)

Path **P<sub>2</sub>** acquires and releases the locks in the inverted sequence:

1. Acquire(**B**)
2. Acquire(**A**)
3. Release(**A**)
4. Release(**B**)

With the right interleaving, perhaps due to both paths executing concurrently
on different threads, a deadlock occurs when path **P<sub>1</sub>** holds lock
**A** and blocks waiting for lock **B**, while path **P<sub>2</sub>** holds lock
**B** and blocks waiting for lock **A**.

##### Circular Dependency

Inversion may also occur between more than two locks and paths. This kind of
inversion is much harder to recognize through manual inspection because each
pair of locks involved may appear to be correctly ordered in every path involving
just the pairs, and yet a potential deadlock may still exist given overall
ordering of the locks.

For example, a program with the locks **A**, **B**, and **C**; paths
**P<sub>1</sub>**, **P<sub>2</sub>**, and **P<sub>3</sub>**; with the following
behavior has the potential for deadlock:

Path **P<sub>1</sub>** acquires and releases the locks in the sequence:

1. Acquire(**A**)
2. Acquire(**B**)
3. Release(**B**)
4. Release(**A**)

Path **P<sub>2</sub>** acquires and releases the locks in the sequence:

1. Acquire(**B**)
2. Acquire(**C**)
3. Release(**C**)
4. Release(**B**)

Path **P<sub>3</sub>** acquires and releases the locks in the sequence:

1. Acquire(**C**)
2. Acquire(**A**)
3. Release(**A**)
4. Release(**C**)

With the right interleaving of paths **P<sub>1</sub>**, **P<sub>2</sub>**,
and **P<sub>3</sub>** a deadlock occurs as each path acquires the lock at the
first step and waits for the lock at the second step. In practice this situation
may be compounded by the existence of many different paths that produce the same
pairwise lock sequences.

##### IRQ-Safe Ordering

In systems that deal with hardware interrupts the ordering between irq-safe and
non-irq-safe locks is critical: a non-irq-safe lock must never be acquired while
holding an irq-safe lock to prevent indirect lock inversions. Irq-safe locks
preserve ordering between irq and non-irq context; a consistent order of two or
more irq-safe locks is guaranteed to be safe for paths running in both irq and
non-irq context. The same is not true for non-irq-safe locks. The reason for this
is that non-irq-safe locks permit irq handlers to effectively insert the locks
acquired by the handler at arbitrary points in the interrupted task's lock
sequences.

For example, a system with non-irq-safe lock **A** and irq-safe lock
**B<sub>irq</sub>**; paths **P<sub>1</sub>**, **P<sub>2</sub>**, and irq path
**P<sub>irq</sub>**; with the following behavior has the potential for deadlock:

Path **P<sub>1</sub>** on **CPU1** acquires and releases the lock in sequence:

1. Acquire(**A**)
2. _**P<sub>irq</sub>** interrupts here on **CPU1**_
3. Release(**A**)

Path **P<sub>irq</sub>** on **CPU1** acquires and releases the lock in sequence:

1. Acquire(**B<sub>irq</sub>**)
2. Release(**B<sub>irq</sub>**)

Path **P<sub>2</sub>** on **CPU2** acquires and releases the locks in sequence:

1. Acquire(**B<sub>irq</sub>**)
2. Acquire(**A**)
3. Release(**A**)
4. Release(**B<sub>irq</sub>**)

With the right interleaving of paths **P<sub>1</sub>**, **P<sub>2</sub>**, and
**P<sub>irq</sub>** a deadlock occurs as **P<sub>irq</sub>** attempts to acquire
**B<sub>irq</sub>** while **P<sub>2</sub>** holds **B<sub>irq</sub>** and blocks
waiting for **A**. This is an indirect lock inversion: **P<sub>irq</sub>**
effectively inserts an acquire/release sequence of **B<sub>irq</sub>** in the
middle of the acquire/release sequence of **A** in path **P<sub>1</sub>**, which
is inconsistent with the lock sequence for the same locks in path
**P<sub>2</sub>**.

### Performing Validation

The invariants discussed in the previous section can be validated using a finite
directed graph. The directed graph tracks the identity and order of locks as the
analysis traverses the code paths. Such a graph can be built either by traversing
the call graphs generated by a compiler or source-level processor (static
analysis) or by observing the ordering of locks during program execution (dynamic
analysis). This section introduce the process in abstract terms that apply to
either approach, in preparation for developing a concrete dynamic analysis
strategy later on.

In the most general terms, building a directed graph from a code path requires
maintaining a list of actively held locks as the path is traversed: a node
representing a lock is added to the list whenever the lock is acquired and
removed from the list whenever the lock is released. In addition to maintaining
the active list, a directed edge is added to the graph from a vertex representing
the newly acquired lock to each vertex representing a lock already in the list.

#### Basic Inversion Example

This section illustrates a directed graph approach to detect a basic two-lock
inversion.

Recall from the earlier example a program with the locks **A** and **B**; code
paths **P<sub>1</sub>** and **P<sub>2</sub>**; and the following behavior:

Path **P<sub>1</sub>** acquires and releases the locks in the sequence:

1. Acquire(**A**)
2. Acquire(**B**)
3. Release(**B**)
4. Release(**A**)

Path **P<sub>2</sub>** acquires and releases the locks in the inverted sequence:

1. Acquire(**B**)
2. Acquire(**A**)
3. Release(**A**)
4. Release(**B**)

##### Analysis of Path **P<sub>1</sub>**

Starting with path **P<sub>1</sub>** we define and update the directed graph.

Let **L<sub>1</sub>** be the ordered _active_ list of locks held by path
**P<sub>1</sub>**.

Let **G** = (**V**, **E**) be the directed graph, having the set of vertices
**V** representing observed locks and the set of directed edges between vertices
**E**.

Initial state:

| **L<sub>1</sub>** | **V** | **E** |
|-------------------|-------|-------|
| ()                | {}    | {}    |

After **P<sub>1</sub>** step 1:

| **L<sub>1</sub>** | **V**   | **E** |
|-------------------|---------|-------|
| (**A**)           | {**A**} | {}    |

This step adds lock **A** to the active list and introduces a vertex for the
same lock to the directed graph. Since there are no other locks in the active
list no edges are added.

After **P<sub>1</sub>** step 2:

| **L<sub>1</sub>** | **V**          | **E**            |
|-------------------|----------------|------------------|
| (**A**, **B**)    | {**A**, **B**} | {(**B**, **A**)} |

This step adds lock **B** to the active list and also introduces a corresponding
vertex to the graph. This time the active list does contain a lock, so an edge
from the new lock to the existing lock is added to the graph. This edge
represents that lock **B** now _depends_ on lock **A** preceding it in any
other path that involves both locks.

After **P<sub>1</sub>** step 3:

| **L<sub>1</sub>** | **V**          | **E**            |
|-------------------|----------------|------------------|
| (**A**)           | {**A**, **B**} | {(**B**, **A**)} |

Lock **B** is removed from the active list. No updates to the graph.

After **P<sub>1</sub>** step 4:

| **L<sub>1</sub>** | **V**          | **E**            |
|-------------------|----------------|------------------|
| ()                | {**A**, **B**} | {(**B**, **A**)} |

Lock **A** is removed from the active list. No updates to the graph.

##### Analysis of Path **P<sub>2</sub>**

Let **L<sub>2</sub>** be the active list of locks held by **P<sub>2</sub>**.

Initial state:

| **L<sub>2</sub>** | **V**          | **E**            |
|-------------------|----------------|------------------|
| ()                | {**A**, **B**} | {(**B**, **A**)} |

In this case the initial state is the final state from path **P<sub>1</sub>**.

After **P<sub>2</sub>** step 1:

| **L<sub>2</sub>** | **V**          | **E**            |
|-------------------|----------------|------------------|
| (**B**)           | {**A**, **B**} | {(**B**, **A**)} |

This step adds lock **B** to the active list. As there are no other locks in the
active list no edges are added to the graph. Since **B** already has a vertex in
the graph there is also no change to **V**.

After **P<sub>2</sub>** step 2:

| **L<sub>2</sub>** | **V**          | **E**                            |
|-------------------|----------------|----------------------------------|
| (**B**, **A**)    | {**A**, **B**} | {(**B**, **A**), (**A**, **B**)} |

This step adds lock **A** to the active list. Since this lock already has a
vertex there is no change to **V**. However, because there is a lock in the
active list an edge from the new lock to the existing lock is added to the
graph. With this new edge the graph now forms a cycle between vertices **A** and
**B**, indicating that ordering between these locks is not consistent between
the two paths considered thus far and that a potential deadlock exists.

#### Circular Dependency Example

This section illustrates a directed graph approach to detect a circular
dependency inversion using previously discussed example from the invariants
section. This illustration is somewhat abbreviated due to the similarity to the
previous illustration.

Consider a program with the locks **A**, **B**, and **C** and paths
**P<sub>1</sub>**, **P<sub>2</sub>**, and **P<sub>3</sub>** and the following
behavior:

Path **P<sub>1</sub>** acquires and releases the locks in the sequence:

1. Acquire(**A**)
2. Acquire(**B**)
3. Release(**B**)
4. Release(**A**)

Path **P<sub>2</sub>** acquires and releases the locks in the sequence:

1. Acquire(**B**)
2. Acquire(**C**)
3. Release(**C**)
4. Release(**B**)

Path **P<sub>3</sub>** acquires and releases the locks in the sequence:

1. Acquire(**C**)
2. Acquire(**A**)
3. Release(**A**)
4. Release(**C**)

##### Analysis of Path **P<sub>1</sub>**

Let **L<sub>1</sub>** be the ordered _active_ list of locks held by path
**P<sub>1</sub>**.

Let **G** = (**V**, **E**) be the directed graph, having the set of vertices
**V** representing observed locks and the set of directed edges between vertices
**E**.

Initial state:

| **L<sub>1</sub>** | **V** | **E** |
|-------------------|-------|-------|
| ()                | {}    | {}    |

After **P<sub>1</sub>** step 1:

| **L<sub>1</sub>** | **V**   | **E** |
|-------------------|---------|-------|
| (**A**)           | {**A**} | {}    |


After **P<sub>1</sub>** step 2:

| **L<sub>1</sub>** | **V**          | **E**            |
|-------------------|----------------|------------------|
| (**A**, **B**)    | {**A**, **B**} | {(**B**, **A**)} |


After **P<sub>1</sub>** step 3:

| **L<sub>1</sub>** | **V**          | **E**            |
|-------------------|----------------|------------------|
| (**A**)           | {**A**, **B**} | {(**B**, **A**)} |


After **P<sub>1</sub>** step 4:

| **L<sub>1</sub>** | **V**          | **E**            |
|-------------------|----------------|------------------|
| ()                | {**A**, **B**} | {(**B**, **A**)} |

##### Analysis of Path **P<sub>2</sub>**

Let **L<sub>2</sub>** be the ordered _active_ list of locks held by path
**P<sub>2</sub>**.

Initial state:

| **L<sub>2</sub>** | **V**          | **E**            |
|-------------------|----------------|------------------|
| ()                | {**A**, **B**} | {(**B**, **A**)} |


After **P<sub>2</sub>** step 1:

| **L<sub>2</sub>** | **V**          | **E**            |
|-------------------|----------------|------------------|
| (**B**)           | {**A**, **B**} | {(**B**, **A**)} |

After **P<sub>2</sub>** step 2:

| **L<sub>2</sub>** | **V**                 | **E**                            |
|-------------------|-----------------------|----------------------------------|
| (**B**, **C**)    | {**A**, **B**, **C**} | {(**B**, **A**), (**C**, **B**)} |

This step adds lock **C** to the active list and also introduces a corresponding
vertex to the graph. The active list contains the lock **B**, so an edge is added
from **C** to **B**.

After **P<sub>2</sub>** step 3:

| **L<sub>2</sub>** | **V**                 | **E**                            |
|-------------------|-----------------------|----------------------------------|
| (**B**)           | {**A**, **B**, **C**} | {(**B**, **A**), (**C**, **B**)} |


After **P<sub>2</sub>** step 4:

| **L<sub>2</sub>** | **V**                 | **E**                            |
|-------------------|-----------------------|----------------------------------|
| ()                | {**A**, **B**, **C**} | {(**B**, **A**), (**C**, **B**)} |

##### Analysis of Path **P<sub>3</sub>**

Let **L<sub>3</sub>** be the ordered _active_ list of locks held by path
**P<sub>3</sub>**.

Initial state:

| **L<sub>3</sub>** | **V**                 | **E**                            |
|-------------------|-----------------------|----------------------------------|
| ()                | {**A**, **B**, **C**} | {(**B**, **A**), (**C**, **B**)} |


After **P<sub>3</sub>** step 1:

| **L<sub>3</sub>** | **V**                 | **E**                            |
|-------------------|-----------------------|----------------------------------|
| (**C**)           | {**A**, **B**, **C**} | {(**B**, **A**), (**C**, **B**)} |

After **P<sub>3</sub>** step 2:

| **L<sub>3</sub>** | **V**                 | **E**                                            |
|-------------------|-----------------------|--------------------------------------------------|
| (**C**, **A**)    | {**A**, **B**, **C**} | {(**B**, **A**), (**C**, **B**), (**A**, **C**)} |

This step adds lock **A** to the active list. The active list contains the lock
**C**, so an edge is added from **A** to **C**. With this new edge the graph now
forms a cycle in the vertices (**A**, **B**, **C**), indicating a circular
dependency and the potential for deadlock if paths **P<sub>1</sub>**,
**P<sub>2</sub>**, and **P<sub>3</sub>** are interleaved in the right way.

#### IRQ-Safe Ordering Example

This section illustrates a directed graph approach to detect irq-safe order
violations using the previously discussed example from the invariants section.

Recall the example system with non-irq-safe lock **A** and irq-safe lock
**B<sub>irq</sub>**; paths **P<sub>1</sub>**, **P<sub>2</sub>**, and irq path
**P<sub>irq</sub>**; with the following behavior:

Path **P<sub>1</sub>** acquires and releases the lock in sequence:

1. Acquire(**A**)
2. Release(**A**)

Path **P<sub>irq</sub>** acquires and releases the lock in sequence:

1. Acquire(**B<sub>irq</sub>**)
2. Release(**B<sub>irq</sub>**)

Path **P<sub>2</sub>** acquires and releases the locks in sequence:

1. Acquire(**B<sub>irq</sub>**)
2. Acquire(**A**)
3. Release(**A**)
4. Release(**B<sub>irq</sub>**)

##### Analysis of Path **P<sub>1</sub>**

Let **L<sub>1</sub>** be the ordered _active_ list of locks held by path
**P<sub>1</sub>**.

Let **G** = (**V**, **E**) be the directed graph, having the set of vertices
**V** representing observed locks and the set of directed edges between vertices
**E**.

Initial state:

| **L<sub>1</sub>** | **V** | **E** |
|-------------------|-------|-------|
| ()                | {}    | {}    |

After **P<sub>1</sub>** step 1:

| **L<sub>1</sub>** | **V**   | **E** |
|-------------------|---------|-------|
| (**A**)           | {**A**} | {}    |

After **P<sub>1</sub>** step 2:

| **L<sub>1</sub>** | **V**   | **E** |
|-------------------|---------|-------|
| ()                | {**A**} | {}    |

##### Analysis of Path **P<sub>irq</sub>**

Let **L<sub>irq</sub>** be the ordered _active_ list of locks held by path
**P<sub>irq</sub>**.

Initial state:

| **L<sub>irq</sub>** | **V**   | **E** |
|---------------------|---------|-------|
| ()                  | {**A**} | {}    |

After **P<sub>irq</sub>** step 1:

| **L<sub>irq</sub>**   | **V**                        | **E** |
|-----------------------|------------------------------|-------|
| (**B<sub>irq</sub>**) | {**A**, **B<sub>irq</sub>**} | {}    |

After **P<sub>irq</sub>** step 2:

| **L<sub>irq</sub>** | **V**                        | **E** |
|---------------------|------------------------------|-------|
| ()                  | {**A**, **B<sub>irq</sub>**} | {}    |

##### Analysis of Path **P<sub>irq</sub>**

Let **L<sub>2</sub>** be the ordered _active_ list of locks held by path
**P<sub>2</sub>**.

Initial state:

| **L<sub>2</sub>** | **V**   | **E** |
|-------------------|---------|-------|
| ()                | {**A**} | {}    |

After **P<sub>2</sub>** step 1:

| **L<sub>2</sub>**     | **V**                        | **E** |
|-----------------------|------------------------------|-------|
| (**B<sub>irq</sub>**) | {**A**, **B<sub>irq</sub>**} | {}    |

After **P<sub>2</sub>** step 2:

| **L<sub>2</sub>**            | **V**                        | **E**                          |
|------------------------------|------------------------------|--------------------------------|
| (**B<sub>irq</sub>**, **A**) | {**A**, **B<sub>irq</sub>**} | {(**A**, **B<sub>irq</sub>**)} |

This step adds lock **A** to the active list. The active list contains lock
**B<sub>irq</sub>**, so an edge is added from **A** to **B<sub>irq</sub>**.
Because this is an edge from a non-irq-safe lock to an irq-safe lock the irq-safe
ordering invariant is violated and a potential deadlock exists.

## From Theory to Implementation

This section develops a concrete strategy to implement a directed graph
validator, based on the analysis techniques of the previous section.

The implementation strategy has the following goals:

1. Avoid dynamic allocation if possible.
2. Minimize the overhead of validation.
3. Support environments that manage hardware interrupts.

### Removing Redundancy with Lock Classes

In the analysis earlier in this document, locks are considered abstractly with
the implication that the tracked objects are individual instances of locks.
While tracking individual instances produces correct results, it has several
consequences that might be avoided:

1. Tracking structures must be dynamically adjusted as lock instances come into
   and out of existence, possibly requiring dynamic allocation or other
   per-instance data storage.
2. The graph contains redundant information when multiple instances of locks are
   used identically by the same code paths.
3. Relatedly, it may take longer to identify violations by locks that serve
   identical functions, but have not yet individually propagated through all of
   the necessary code paths.

A key observation is that locks that serve identical functions should follow the
same ordering rules, regardless of the number of instances.

Consider the following types with lock members and an operation that mutates
both types:

```C++
struct Foo {
    Mutex lock;
    int data; GUARDED_BY(lock);
};

struct Bar {
    Mutex lock;
    int data; GUARDED_BY(lock);
};

void Swap(Foo* foo, Bar* bar) {
    foo->lock.Acquire();
    bar->lock.Acquire();

    int temp = foo->data;
    foo->data = bar->data;
    bar->data = temp;

    bar->Release();
    foo->Release();
}
```

Since operation `Swap` may operate on any instance of `Foo` and any instance of
`Bar` it follows that `Swap` establishes an order between the locks of all
instances of `Foo` and `Bar`; failure to apply this order consistently in other
parts of a program could result in a deadlock when the same instances of `Foo`
and `Bar` are locked concurrently in different orders.

Note that it is possible to intentionally or unintentionally segregate different
collections of `Foo` and `Bar` such that instances locked in different orders
never overlap. This is still dangerous however, because seemingly innocuous
changes to the inputs, structure, or timing of the program could defeat the
segregation and introduce a potential deadlock. This problem can be avoided
entirely by treating all instances of `Foo` and `Bar` equivalently and applying
the same ordering rules throughout the program.

Ensuring universal ordering throughout the program can be achieved by tracking
classes of locks instead of lock instances: each lock member in each type
represents a unique lock class. The relationships between each lock class can
be tracked and analyzed using the same directed graph techniques as with
individual locks.

Tracking lock classes has the following benefits:

1. Statically allocated memory: because all lock classes are known at compile
   time, tracking structures can be allocated up front as static global data.
2. Elimination of redundant graph nodes: locks in the same class use the same
   tracking structures.
3. Faster detection of invariant violations: violations are detected when
   lock class orders are inconsistent, even if the individual instances involved
   have never been used together.

#### Additional Ordering Rules

Tracking lock classes introduces additional ordering considerations when locking
multiple locks of the same class. Because individual instances are not tracked
it is necessary to take additional steps to ensure consistency when multiple
locks of the same class must be acquired at the same time.

##### Externally Ordered Locks

Nesting locks of the same class is necessary when a hierarchical or other
ordered data structure has locks in each node and more than one per-node lock
must be held at a time. In this situation the data structure or access pattern
must provide a stable ordering that is used to guarantee ordering of the locks.

Validation of nestable lock classes requires only that the external order is
recorded in the active locks list for each nestable lock and compared when new
locks of the same class are added to the list. A consequence of this design is
that other lock classes may not be interspersed between nested locks of the
same class, only wholly before or after a collection of nested locks.

For example, non-nestable lock classes **A** and **B**, and nestable lock class
**N** may be interspersed like this:

**A**, **N<sub>0</sub>**, **N<sub>1</sub>**, ... **N<sub>n</sub>**, **B**

But not like this:

**A**, **N<sub>0</sub>**, **B**, **N<sub>1</sub>**, ... **N<sub>n</sub>** or
**A**, **N<sub>0</sub>**, **N<sub>1</sub>**, **B**, ... **N<sub>n</sub>** or
... etc

In most situations this is a reasonable constraint, as interspersing other locks
within a nested structure with arbitrary depth is likely to result in inversions
as the structure is updated at runtime. On the other hand, in situations where
nesting is bounded to a few levels it may be more effective to define separate
lock classes for each level instead of using a nested class -- in this case
other locks may be allowed at a specific level following normal lock ordering
rules.

##### Address Ordering

It is difficult to generalize lock ordering between locks of the same class
without an externally provided order when the locks are acquired at different
times. It is possible however, to provide an ordering guarantee when acquiring
multiple locks at the same time, without temporal separation. In this situation
the locks may be ordered by address, guaranteeing that any path that acquires
the same set locks produces a consistent locking order.

For example, consider an operation **F**(**S<sub>a</sub>**, **S<sub>b</sub>**)
that operates on two instances of structure **S**, each with a lock of class
**L** and, as part of the operation **F** must lock both locks.

If instance **S<usb>0</sub>** is ordered in memory before instance
**S<sub>1</sub>** then the locks have the same relative ordering as their
containing instances. We can consider the locks to have the subclasses
**L<sub>0</sub>** and **L<sub>1</sub>** respectively.

A lock ordering problem arises if we perform the operation with different
orders:

**F**(**S<sub>0</sub>**, **S<sub>1</sub>**) and
**F**(**S<sub>1</sub>**, **S<sub>0</sub>**)

Without intervention these produce the inverted lock sequences:

**L<sub>0</sub>**, **L<sub>1</sub>** and **L<sub>1</sub>**, **L<sub>0</sub>**

Since **F** has simultaneous access to both locks at the same time, it is
possible to order the locks by address, resulting in a consistent lock
sequence regardless of the original order of the arguments.

Now suppose we add two more lock classes to the sequence: class **A** acquired
before operation **F** and class **B** acquired after operation **F**. The
resulting lock sequence is:

**A**, **L<sub>0</sub>**, **L<sub>1</sub>**, **B**

Note that this looks similar to the nested lock class sequence diagram in the
previous section. It is in fact the same situation, only the ordering of locks
is provided by address rather than an external order. This means that the same
bookkeeping in the active threads list can be used for both situations.

#### Lock Class Tracking Data Structure

This section discusses implementation details for tracking lock classes and
concrete processing techniques to detect potential deadlocks.

Each lock class has a statically allocated node in the directed graph
representing all locks belonging to that class. Each node has the following data
structures:

##### Lock-Free, Wait-Free Hash Set

Each lock class node has a hash set that tracks the edges from the lock class to
the lock classes ordered before it.

**TODO**: Add implementation details of the hash set.

##### Lock-Free, Wait-Free Disjoint Set Structures

Each lock class node has a parent pointer used to track nodes that are connected
in cycles in the directed graph. This permits reporting cycles that have been
previously by the loop detection algorithm without fully re-traversing the graph.

**TODO**: Add implementation details of the disjoint set structure.

##### Thread-Local Lock List

Each thread maintains a thread-local list of the locks it currently holds.

**TODO**: Add implementation details of the thread-local lock list.

##### Loop Detection Thread

Whenever a new edge is added to the directed graph, the loop detection thread is
triggered to traverse the graph to find circular dependencies involving more than
two locks. Tarjan's strongly connected sets algorithm is an efficient choice,
with worst case complexity of **O**(|**E**| + |**V**|). This algorithm is stable
even when traversing a graph that is updated concurrently by other threads.

**TODO**: Add implementation details of the loop detection thread.

## References

1. Clang static [thread safety analysis](https://clang.llvm.org/docs/ThreadSafetyAnalysis.html).
2. LLVM runtime [thread sanitizer](https://github.com/google/sanitizers/wiki/ThreadSanitizerDeadlockDetector).
3. Linux Kernel [lockdep subsystem](https://www.kernel.org/doc/Documentation/locking/lockdep-design.txt).

