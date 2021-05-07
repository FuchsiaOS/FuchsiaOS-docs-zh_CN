# Compile-Time Object Collections Discussion

This document covers active discussion about building compile-time collections
of objects in C++. The following use cases are examples of where compile-time
collections are useful:

* StringRef - A type that supports building a compile-time collection of string
  labels with associated unique numeric ids for tracing purposes.
* LockClass - A type that supports building a compile-time collection of state
  objects for runtime lock validation purposes.

The following sections discuss common and unique requirements of each use case,
the current challenges with the implementations, and proposed solutions.

## StringRef

StringRef is a type that implements the concept of string references. A string
reference is a mapping from a numeric id to a character string. Using the
mapping makes more economical use of the trace buffer: an (id, string)
pair is emitted once in a tracing session and then subsequent events may refer
to the string by id instead of including the full character sequence inline.

The following is a simple example of `StringRef` in action:

```C++
#include <lib/ktrace.h>

template <typename Op, typename... Args>
inline DoSomething(Op&& op, Args&&... args) {
    ktrace_probe(TraceAlways, TraceContext::Thread, "DoSomething"_stringref);
    // ...
}
```

Here the string literal operator `_stringref` returns an instance of `StringRef`
that provides the facility to map the string "DoSomething" to a numeric id
used by the tracing function.

### Requirements

* Emit each (id, string) mapping at most once per tracing session, prior to any
  trace event that references the id. Ideally the full set of mappings is
  emitted at once at the start of a tracing session to avoid the overhead of
  emitting mappings in the middle of timing sensitive code however, this is not
  a hard requirement.
* A dense id space is desirable so that down stream processing code can use a
  linear pre-allocated array or vector to implement the id-to-string lookup,
  however this is not a hard requirement.
* Some method to unique duplicate string references, since trace calls must be
  supported in template and inline functions and methods.

### Current Implementation

The following is an outline of the current `StringRef` implementation.

```C++
struct StringRef {
    static constexpr int kInvalidId = -1;

    const char* string{nullptr};
    ktl::atomic<int> id{kInvalidId};
    StringRef* next{nullptr};

    int GetId() {
        const int ref_id = id.load(ktl::memory_order_relaxed);
        return ref_id == kInvalidId ? Register(this) : ref_id;
    }

    // Returns the head of the global string ref linked list.
    static StringRef* head() { return head_.load(ktl::memory_order_acquire); }

private:
    // Registers a string ref in the global linked list.
    static int Register(StringRef* string_ref);

    static ktl::atomic<int> id_counter_;
    static ktl::atomic<StringRef*> head_;
};

// Returns an instance of StringRef that corresponds to the given string
// literal.
template <typename T, T... chars>
inline StringRef* operator""_stringref() {
    static const char storage[] = {chars..., '\0'};
    static StringRef string_ref{storage};
    return &string_ref;
}
```

## LockClass

LockClass is a type that captures information about a lock that is common to all
instances of the lock (e.g. its containing type if it is a struct/class member,
the type of the underlying lock primitive, flags describing its behavior). The
LockClass type is used by the runtime lock validator to determine which ordering
rules apply to each lock and to locate the per-lock-class tracking structure
used to record ordering observations.

The following is a simplified example of LockClass in action:

```C++
struct Foo {
    LockClass<Foo, fbl::Mutex> lock;
    // ...
};

struct Bar {
    LockClass<Bar, fbl::Mutex> lock;
};
```

### Requirements

* Ability to iterate the tracking state for all instantiations of LockClass, for
  cycle detection and error reporting purposes.
* Some method to unique duplicate tracking state, since instantiations of
  LockClass may be visible from multiple compilation units, depending on how the
  containing types (e.g. Foo and Bar) are used.
* A dense id space is desirable so that down stream processing code can simplify
  id storage however, this is not a hard requirement.

### Current Implementation

The following is a simplified implementation of LockClass:

```C++
template <typename ContainingType, typename LockType>
class LockClass {
    // ...
private:
    static LockClassState lock_class_state_;
};
```

Each instantiation of `LockClass` creates a unique instance of `LockClassState`
to track the online lock order observations related to locks of class
(`ContainingType`, `LockType`). The current implementation of `LockClassState`
constructs a linked list of all instances in a global ctor to support the
iteration requirement.

## Compile-Time Array Solution

One way to address the requirements of both types is to build a compile-time
array of de-duplicated static instances, using COMDAT sections and groups. This
completely removes the need to build linked lists of objects at init time or
runtime and supports all of the requirements for each type.

For example:

```C++
// Defined in the linker script mark the beginning and end of the section:
// .data.lock_class_state_table.
extern "C" LockClassState __start_lock_class_state_table[];
extern "C" LockClassState __end_lock_class_state_table[];

template <typename ContainingType, typename LockClass>
class LockClass {
    // ...
private:
    static LockClassState lock_class_state_ __SECTION(".data.lock_class_state_table");
};

// Defined in the linker script to make the beginning and end of the section:
// .rodata.string_ref_table.
extern "C" StringRef __start_string_ref_table[];
extern "C" StringRef __end_string_ref_table[];

struct StringRef {
    const char* const string;
    size_t GetId() const {
        return static_cast<size_t>(this - __start_string_ref_table);
    }
};

template <typename T, T... chars>
inline StringRef* operator""_stringref() {
    static const char storage[] = {chars..., '\0'};
    static StringRef string_ref __SECTION(".rodata.string_ref_table") {storage};
    return &string_ref;
}
```

The challenge with this approach is that GCC does not properly handle the
section attribute on static members of template types or functions. However,
Clang does handle the section attribute correctly for these types.
