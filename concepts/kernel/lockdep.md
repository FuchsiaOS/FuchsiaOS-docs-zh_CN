# Runtime Lock Validation in Zircon

## Introduction

Zircon integrates a runtime lock validator to diagnose inconsistent lock
ordering that could lead to deadlocks. This document discusses how the
validator is integrated, how to enable and tune the validator at build time,
and what output the validator produces.

The theory of operation for the validator itself can be found in the
[design document](lockdep-design.md).

## Enabling the Lock Validator

Lock validation is disabled by default. **When disabled the lock instrumentation
is transparent, acting as a zero-overhead wrapper for the underlying locking
primitives**.

The validator is enabled at compile time by setting the GN build argument
`enable_lock_dep` to true. As of this writing logic for this variable is
handled by [zircon/kernel/BUILD.gn](/zircon/kernel/BUILD.gn).

You can set this variable in your GN invocation like this:

```
fx set <your build options> --args 'enable_lock_dep = true'
```

When the lock validator is enabled a set of global lock-free, wait-free data
structures are generated to track the relationships between the instrumented
locks. The acquire/release operations of the locks are augmented to update
these data structures.

## Lock Instrumentation

The current incarnation of the runtime lock validator requires manually
instrumenting each lock in kernel with a wrapper type. The wrapper type provides
the context the validator needs to properly identify the lock and generate a
global tracking structure for locks with the same context or role.

The kernel defines utility macros for this purpose in `kernel/spinlock.h` and
`kernel/mutex.h`.

### Member Locks

A type with a lock member like this:

```C++
#include <kernel/mutex.h>

class MyType {
public:
	// ...
private:
	mutable fbl::Mutex lock_;
	// ...
};
```

May be instrumented like this:

```C++
#include <kernel/mutex.h>

class MyType {
public:
	// ...
private:
	mutable DECLARE_MUTEX(MyType) lock_;
	// ...
};
```

Note that the containing type is passed to the macro
`DECLARE_MUTEX(containing_type)`. This type provides the context the validator
needs to distinguish locks that are members of `MyType` from locks that are
members of other types.

The macro `DECLARE_SPINLOCK(containing_type)` provides similar support for
instrumenting `SpinLock` members.

For those who are curious, the macro in the example above expands to this type
expression: `::lockdep::LockDep<containing_type, fbl::Mutex, __LINE__>`. This
expression results in a unique instantiation of the `lockdep::LockDep<>` type,
both across different containing types, and within a containing type where
there is more than one mutex.

### Global Locks

Global locks are instrumented using a singleton-type pattern. The kernel defines
utility macros for this purpose in `kernel/mutex.h` and `kernel/spinlock.h`.

In Zircon global locks are typically defined either at global/namespace scope or
within another type as a static member.

example.h:

```C++
#include <kernel/mutex.h>

extern fbl::Mutex a_global_lock;

class MyType {
public:
	// ...
private:
	static fbl::Mutex all_objects_lock_;
};
```

example.cpp:

```C++
#include "example.h"

fbl::Mutex a_global_lock;

fbl::Mutex MyType::all_objects_lock_;
```

The instrumentation simplifies declaring locks by declaring singleton types that
may be used in either scope and handles ODR-use automatically.

example.h:

```
#include <kernel/mutex.h>

DECLARE_SINGLETON_MUTEX(AGlobalLock);

class MyType {
public:
	// ...
private:
	DECLARE_SINGLETON_MUTEX(AllObjectsLock);
};
```

These macro invocations declare new singleton types, `AGlobalLock` and
`MyType::AllObjectsLock` respectively. These types have a static `Get()` method
that returns the underlying global lock with all of the necessary
instrumentation. Note that there is no need to separately define storage for the
locks, this is handled automatically by the supporting template types.

The macro `DECLARE_SINGLETON_SPINLOCK(name)` provides similar support for
declaring a global `SpinLock`.

### Lock Guards

Instrumented locks are acquired and released using the scoped capability types
`Guard` and `GuardMultiple`. In the kernel these types are defined in
`kernel/lockdep.h`.

The operation of `Guard` for simple mutexes is similar to `AutoLock`:

```C++
#include <kernel/mutex.h>

class MyType {
public:
	// ...

	int GetData() const {
		Guard<fbl::Mutex> guard{&lock_};
		return data_;
	}

	int DoSomething() {
		Guard<fbl::Mutex> guard{&lock_};
		int data_copy = data_;
		guard.Release();

		return DoWorkUnlocked(data_copy);
	}

private:
	mutable DECLARE_MUTEX(MyType) lock_;
	int data_{0} TA_GUARDED(lock_);
};
```

`SpinLock` types require an additional template argument to `Guard` to select
one of a few possible options when acquiring the lock: `IrqSave`, `NoIrqSave`,
and `TryLockNoIrqSave`. Omitting one of these type tags results in a
compile-time error.

```C++
#include <kernel/spinlock.h>

class MyType {
public:
	// ...

	int GetData() const {
		Guard<SpinLock, IrqSave> guard{&lock_};
		return data_;
	}

	void DoSomethingInIrqContext() {
		Guard<SpinLock, NoIrqSave> guard{&lock_};
		// ...
	}

	bool TryToDoSomethingInIrqContext() {
		if (Guard<SpinLock, TryLockNoIrqSave> guard{&lock_}) {
			// ...
			return true;
		}
		return false;
	}

private:
	mutable DECLARE_SPINLOCK(MyType) lock_;
	int data_{0} TA_GUARDED(lock_);
};
```

Instrumented global locks work similarly:

```C++
#include <kernel/mutex.h>
#include <fbl/intrusive_double_list.h>

class MyType : public fbl::DoublyLinkedListable<MyType> {
public:
	// ...

	void AddToList(MyType* object) {
		Guard<fbl::Mutex> guard{AllObjectsLock::Get()};
		all_objects_list_.push_back(*object);
	}

private:
	DECLARE_SINGLETON_MUTEX(AllObjectsLock);
	fbl::DoublyLinkedList<MyType> all_objects_list_ TA_GUARDED(AllObjectsLock::Get());
};
```

Note that instrumented locks do not have manual `Acquire()` and `Release()`
methods; using a `Guard` is the only way to acquire the locks directly. There
are two important reasons for this:

1. Manual acquire/release operations are more error prone than guard, plus
   manual release when necessary.
2. When lock validation is enabled the guard provides the storage that the
   validator uses to account for actively held locks. This approach permits
   temporary storage of validator state on the stack only for the duration the
   lock is held, which corresponds with the use patterns of guard objects.
   Without this approach the tracking data would either have to be stored with
   each lock instance, increasing memory use even when locks are not held, or
   stored in heap allocated memory. Neither of these alternatives is desirable.

In rare circumstances the underlying lock may be accessed using the `lock()`
accessor of the instrumented lock. This should be done with care as manipulating
the underlying lock directly may result inconsistency between the state of the
lock and the state the lock validator; at best this may lead to missing a lock
order warning and at worst may lead to a deadlock. **You have been warned!**

## Clang Static Analysis and Instrumented Locks

The lock instrumentation is designed to interoperate with Clang static lock
analysis. In general usage, an instrumented lock may be used as a "mutex"
capability and specified in any of the static lock annotations.

There are two special cases that need some extra attention:

1. Returning pointers or references to capabilities.
2. Unlocking a guard passed by reference.

### Pointers and References to Capabilities

When returning a lock by pointer or reference it may be convenient or necessary
to use a uniform type. Recall from earlier that instrumented locks are wrapped
in a type that captures the containing type, the underlying lock type, and the
line number to disambiguate locks belonging to different types
(`::lockdep::LockDep<Class, Locktype, Index>`). This can lead to difficulty when
returning a lock from a uniform (virtual) interface (e.g. kernel
`Dispatcher::get_lock()`).

Fortunately there is a straightforward solution: every instrumented lock is also
a subclass of `::lockdep::Lock<LockType>` (or simply `Lock<LockType>` in the
kernel). This type only depends on the underlying `LockType`, not the context in
which the instrumented lock is declared, making it convenient to use as a
pointer or reference type to refer to an instrumented lock more generically.
This type may be used in type annotations as well.

The following illustrates the pattern, which is similar to that employed by the
kernel `Dispatcher` types.

```C++
#include <kernel/mutex.h>


struct LockableInterface {
	virtual ~LockableInterface() {}
	virtual Lock<fbl::Mutex>* get_lock() = 0;
	virtual void DoSomethingLocked() TA_REQ(get_lock()) = 0;
};

class A : public LockableInterface {
public:
	Lock<fbl::Mutex>* get_lock() override { return &lock_; }
	void DoSomethingLocked() override {
		data_++;
	}
	void DoSomething() {
		Guard<fbl::Mutex> guard{get_lock()};
		DoSomethingLocked();
		// ...
	}
private:
	mutable DECLARE_MUTEX(A) lock_;
	int data_ TA_GUARDED(get_lock());
};

class B : public LockableInterface {
public:
	Lock<fbl::Mutex>* get_lock() override { return &lock_; }
	void DoSomethingLocked() override {
		// ...
	}
	void DoSomething() {
		Guard<fbl::Mutex> guard{get_lock()};
		DoSomethingLocked();
		// ...
	}
private:
	mutable DECLARE_MUTEX(B) lock_;
	char data_[32] TA_GUARDED(get_lock());
};
```

Note that the type of `A::lock_` is
`::lockdep::LockDep<A, fbl::Mutex, __LINE__>` and the type of `B::lock_` is
`::lockdep::LockDep<B, fbl::Mutex, __LINE__>`. However, both of these types are
subclasses of `Lock<fbl::Mutex>`, so we can treat them uniformly as this type in
pointer and reference expressions.

While this is very convenient, a limitation in Clang static analysis prevents it
from understanding that `LockableInterface::get_lock()` is equivalent to
`A::lock_` or `B::lock_`, even in their local contexts. For this reason is it
necessary to use `get_lock()` in all of the lock annotations.

### Unlocking a Guard Passed by Reference

In very rare circumstances it is useful to release a `Guard` instance held in
a function from a callee of the function.

**TODO(eieio): Complete documentation of this feature.**

## Lock Validation Errors

The lock validator detects and reports two broad classes of violations:

1. Pair-wise violations reported at the point of acquisition.
2. Multi-lock cycles reported asynchronously by a dedicated loop detection
   thread.

### Violations Reported at Acquisition

When a violation is detected at the point of lock acquisition the validator
produces a message like the following in the kernel log:

{# Disable variable substition to avoid {{ being interpreted by the template engine #}
{% verbatim %}

```
[00002.668] 01032:01039> ZIRCON KERNEL OOPS
[00002.668] 01032:01039> Lock validation failed for thread 0xffff000001e53598 pid 1032 tid 1039 (userboot:userboot):
[00002.668] 01032:01039> Reason: Out Of Order
[00002.668] 01032:01039> Bad lock: name=lockdep::LockClass<SoloDispatcher<ThreadDispatcher, 316111>, Mutex, 282, (lockdep::LockFlags)0> order=0
[00002.668] 01032:01039> Conflict: name=lockdep::LockClass<SoloDispatcher<ProcessDispatcher, 447439>, Mutex, 282, (lockdep::LockFlags)0> order=0
[00002.668] 01032:01039> {{{module:0:kernel:elf:0bf16acb54de1ceef7ffb6ee4449c6aafc0ab392}}}
[00002.668] 01032:01039> {{{mmap:0xffffffff10000000:0x1ae1f0:load:0:rx:0xffffffff00000000}}}
[00002.668] 01032:01039> {{{mmap:0xffffffff101af000:0x49000:load:0:r:0xffffffff001af000}}}
[00002.668] 01032:01039> {{{mmap:0xffffffff101f8000:0x1dc8:load:0:rw:0xffffffff001f8000}}}
[00002.668] 01032:01039> {{{mmap:0xffffffff10200000:0x76000:load:0:rw:0xffffffff00200000}}}
[00002.668] 01032:01039> {{{bt:0:0xffffffff10088574}}}
[00002.668] 01032:01039> {{{bt:1:0xffffffff1008f324}}}
[00002.668] 01032:01039> {{{bt:2:0xffffffff10162860}}}
[00002.668] 01032:01039> {{{bt:3:0xffffffff101711e0}}}
[00002.668] 01032:01039> {{{bt:4:0xffffffff100edae0}}}
```

{# Re-enable variable substition #}
{% endverbatim %}

The error is informational and non-fatal. The first line identifies the thread
and process where the kernel lock violation occurred. The next line identifies
the type of violation. The next two lines identify which locks were found to be
inconsistent with previous observations: the "Bad lock" is the lock that is
about to be acquired, while "Conflict" is a lock that is already held by the
current context and is the point of inconsistency with the lock that is about to
be acquired. All of the lines following this are part of the stack trace leading
up to the bad lock.

### Multi-Lock Cycles

Circular dependencies between three or more locks are detected with a dedicated
loop detection thread. Because this detection happens in a separate context from
the lock operations that caused the cycle a stack trace is not provided.

Reports from the loop detection thread look like this:

```
[00002.000] 00000.00000> ZIRCON KERNEL OOPS
[00002.000] 00000.00000> Circular lock dependency detected:
[00002.000] 00000.00000>   lockdep::LockClass<VmObject, fbl::Mutex, 249, (lockdep::LockFlags)0>
[00002.000] 00000.00000>   lockdep::LockClass<VmAspace, fbl::Mutex, 198, (lockdep::LockFlags)0>
[00002.000] 00000.00000>   lockdep::LockClass<SoloDispatcher<VmObjectDispatcher>, fbl::Mutex, 362, (lockdep::LockFlags)0>
[00002.000] 00000.00000>   lockdep::LockClass<SoloDispatcher<PortDispatcher>, fbl::Mutex, 362, (lockdep::LockFlags)0>
```

Each of the locks involved in the cycle are reported in a group. Frequently only
two of the circularly-dependent locks are acquired by a single thread at any
given time, making manual detection difficult or impossible. However, the
potential for deadlock between three or more threads is real and should be
addressed for long-term system stability.

## Kernel Commands

When the lock validator is enabled the following kernel commands are available:

* `k lockdep dump` - dumps the dependency graph and connected sets (loops) for
  all instrumented locks.
* `k lockdep loop` - triggers a loop detection pass and reports any loops found
  to the kernel log.
