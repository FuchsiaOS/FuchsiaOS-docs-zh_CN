Fuchsia Core Libraries
======================

This document describes the core libraries in the Fuchsia system, starting from
the bottom of the dependency chain.

# Zircon libraries

## libzircon

This library defines the Zircon system ABI.

TODO(kulakowski) Talk about how this is not quite the kernel
syscall interface, since the VDSO abstracts that.

## libzx

libzircon defines C types and function calls acting on those
objects. libzx is a light C++ wrapper around those. It adds type
safety beyond `zx_handle_t`, so that every kernel object type has a
corresponding C++ type, and adds ownership semantics to those
handles. It otherwise takes no opinions around naming or policy.

For more information about libzx, see
[its documentation](/zircon/system/ulib/zx/README.md).

## FBL

Much of Zircon is written in C++, both in kernel and in
userspace. Linking against the C++ standard library is not especially
well suited to this environment (it is too easy to allocate, throw
exceptions, etc., and the library itself is large). There are a number
of useful constructs in the standard library that we would wish to use,
like type traits and unique pointers. However, C++ standard libraries
are not really to be consumed piecemeal like this. So we built a
library that provides similar constructs named fbl. This library
also includes constructs not present in the standard library but
are useful library code for kernel and device driver environments (for
instance, slab allocation).

For more information about FBL,
[read its overview](/development/languages/c-cpp/cxx.md#fbl).

# FXL

FXL is a platform-independent library containing basic C++ building blocks, such
as logging and reference counting. FXL depends on the C++ standard library but
not on any Zircon- or Fuchsia-specific libraries. We build FXL both for target
(Fuchsia) and for host (Linux, Mac) systems.

Generally speaking, we try to use the C++ standard library for basic building
blocks, but in some cases the C++ standard library either doesn't have something
we need (e.g., a featureful logging system) or has a version of what we need
doesn't meet our requirements (e.g., `std::shared_ptr` is twice as large as
`fxl::RefPtr`).
