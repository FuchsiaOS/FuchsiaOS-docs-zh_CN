# Zircon thread safety annotations

Zircon code takes advantage of clang's thread safety analysis feature to
document and machine-verify some of our synchronization invariants. These
annotations are checked when building for clang (see
[getting started](/docs/development/kernel/getting_started.md) for instructions on building with
clang).

## How to use

[Clang's documentation](https://clang.llvm.org/docs/ThreadSafetyAnalysis.html)

In Zircon, we provide our own set of macros wrapping the annotations and have
annotated our synchronization primitives. When writing new code involving
synchronization or annotating existing code, in most cases you should use the
thread annotation macros provided by
[<lib/zircon-internal/thread\_annotations.h](/zircon/system/ulib/zircon-internal/include/lib/zircon-internal/thread_annotations.h).
These macros all begin with the prefix `"TA_"` for thread analysis. The most
commonly used ones are:

* `TA_GUARDED(x)` the annotated variable is guarded by the capability (e.g. lock) `x`
* `TA_ACQ(x...)` function acquires all of the mutexes in the set `x` and hold them after returning
* `TA_REL(x...)` function releases all of the mutexes in the set `x`
* `TA_REQ(x...)` function requires that the caller hold all of the mutexes in the set `x`
* `TA_EXCL(x...)` function requires that the caller not be holding any of the mutexes in the set `x`

For example, a class containing a member variable `'int foo_'` protected by a
mutex would be annotated like so:

```
// example.h

class Example {
public:
    // Public function has no locking requirements and thus needs no annotation.
    int IncreaseFoo(int by);

private:
    // This is an internal helper routine that can only be called with |lock_|
    // held. Calling this without holding |lock_| is a compile-time error.
    // Annotations like TA_REQ, TA_ACQ, TA_REL, etc are part of the function's
    // interface and must be on the function declaration, usually in the header,
    // not the definition.
    int IncreaseFooLocked(int by) TA_REQ(lock_);

    // This internal routine requires that both |lock_| and |foo_lock_| be held by the
    // caller.
    int IncreaseFooAndBarLocked(int foo_by, int bar_by) TA_REQ(lock_) TA_REQ(bar_lock_);

    // The TA_GUARDED(lock_) annotation on |foo_| means that |lock_| must be
    // held to read or write from |foo_|.
    int foo_ TA_GUARDED(lock_);

    // |lock_| can be declared after annotations referencing it,
    // if desired.
    Mutex lock_;

    Mutex bar_lock_;
};

// example.cpp

int Example::IncreaseFoo(int by) {
    int new_value;
    {
        AutoLock lock(&lock_);  // fbl::AutoLock is annotated
        new_value = IncreaseFooLocked(by);
    }
    return new_value;
}
```

Note that for annotations, which allow sets of mutex objects, one may either
apply the annotation multiple times, or provided a comma separated list to the
annotation.  In other words, the following two declarations are equivalent.

```
    int IncreaseFooAndBarLocked(int foo_by, int bar_by) TA_REQ(lock_) TA_REQ(bar_lock_);
    int IncreaseFooAndBarLocked(int foo_by, int bar_by) TA_REQ(lock_, bar_lock_);
```

Library code exposed through the sysroot must use the more awkwardly named
macros provided by
[system/public/zircon/compiler.h](/zircon/system/public/zircon/compiler.h) to
avoid collisions with consumers of the sysroot.

## Best practices

Annotations should complement the comments and identifiers to make the code
understandable. Annotations do not replace comments or clear names. Try to
follow these best practices when writing code involving locking:

* Group member variables protected by a lock with the lock. Where it makes
sense, document what is protected by what with a comment in addition to the
annotations. For example when several member variables are protected by one lock
and several are protected by a different lock, a comment is easier to read than
going through each annotation.

* Name functions that require a lock be held with a 'Locked()' suffix. If there
are multiple locks that could be plausibly held to call the function, consider
making the choice clear in the function name. Keep in mind readers of calling
code will not be able to see the annotations.

## Limitations

The thread safety analysis is a purely static check done at compile time and
cannot understand conditionally held locks or locking patterns that span
compilation units in ways not expressible via static annotations. In many
situations, this analysis is still useful but there are situations that the
analysis simply cannot understand. The main escape hatch for disabling analysis
is to add the annotation `TA_NO_THREAD_SAFETY_ANALYSIS` to the function definition
containing the code the analysis is confused by. Other escape mechanisms are
available as well - see the Clang documentation for details. Situations that
require disabling the analysis are likely to be complex for humans to understand
as well as machines and should be accompanied by a comment indicating the
invariants in use.

The thread safety analysis can be defeated in a number of ways, for instance
when using pointers.  For example, when taking the address of a guarded data
member Clang loses track of the guard, e.g. for a foo_ protected by a lock_
a call to `memset(&foo_, 0, sizeof(foo_))` without holding lock_ won't be caught
as a violation.
