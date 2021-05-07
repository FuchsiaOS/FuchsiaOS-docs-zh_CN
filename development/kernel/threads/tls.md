# Thread Local Storage #

The ELF Thread Local Storage ABI (TLS) is a storage model for variables that
allows each thread to have a unique copy of a global variable. This model
is used to implement C++'s `thread_local` storage model. On thread creation the
variable will be given its initial value from the initial TLS image. TLS
variables are for instance useful as buffers in thread safe code or for per
thread book keeping. C style errors like errno or dlerror can also be handled
this way.

TLS variables are much like any other global/static variable. In implementation
their initial data winds up in the `PT_TLS` segment. The `PT_TLS` segment
is inside of a read only `PT_LOAD` segment despite TLS variables being writable.
This segment is then copied into the process for each thread in a unique
writable location. The location the `PT_TLS` segment is copied to is influenced
by the segment's alignment to ensure that the alignment of TLS variables is
respected.

## ABI ##

The actual interface that the compiler, linker, and dynamic linker must adhere
to is actually quite simple despite the details of the implementation being more
complex. The compiler and the linker must emit code and dynamic relocations that
use one of the 4 access models (described in a following section). The dynamic
linker and thread implementation must then set everything up so that this
actually works. Different architectures have different ABIs but they're similar
enough at broad strokes that we can speak about most of them as if there was
just one ABI. This document will assume that either x86-64 or AArch64 is being
used and will point out differences when they occur.

The TLS ABI makes use of a few terms:

  * Thread Pointer: This is a unique address in each thread, generally stored
    in a register. Thread local variables lie at offsets from the thread pointer.
    Thread Pointer will be abbreviated and used as `$tp` in this document. `$tp`
    is what `__builtin_thread_pointer()` returns on AArch64. On AArch64 `$tp`
    is given by a special register named `TPIDR_EL0` that can be accessed using
    `mrs <reg>, TPIDR_EL0`. On `x86_64` the `fs.base` segment base is used and
    can be accessed with `%fs:` and can be loaded from `%fs:0` or `rdfsbase`
    instruction.
  * TLS Segment: This is the image of data in each module and specified by the
    `PT_TLS` program header in each module. Not every module has a `PT_TLS`
    program header and thus not every module has a TLS segment. Each module
    has at most one TLS segment and correspondingly at most one `PT_TLS`
    program header.
  * Static TLS set: This is the sum total of modules that are known to the
    dynamic linker at program start up time. It consists of the main executable
    and every library transitively mentioned by `DT_NEEDED`. Modules that
    require being in the Static TLS set have `DF_STATIC_TLS` set on their
    `DT_FLAGS` entry in their dynamic table (given by the `PT_DYNAMIC` segment).
  * TLS Region: This is a contiguous region of memory unique to each
    thread. `$tp` will point to some point in this region. It contains the
    TLS segment of every module in Static TLS set as well as some
    implementation-private data, which is sometimes called the TCB (Thread
    Control Block). On AArch64 a 16-byte reserved space starting at `$tp` is
    also sometimes called the TCB. We will refer to this space as the "ABI TCB"
    in this doc.
  * TLS Block: This is an individual thread's copy of a TLS segment. There is
    one TLS block per TLS segment per thread.
  * Module ID: The module ID is not statically known except for the main
    executable's module ID which is always 1. Other module's module IDs are
    chosen by the dynamic linker. It's just a unique non-zero ID for each
    module. In theory it could be any non-zero 64-bit value that is unique to
    the module like a hash or something. In practice it's just a simple counter
    that the dynamic linker maintains.
  * The main executable: This is the module that contains the start address. It,
    is also treated in a special way in one of the access models. It always
    has a Module ID of 1. This is the only module that can use fixed offsets
    from `$tp` via the Local Exec model described below.

To comply with the ABI all access models must be supported.

#### Access Models ####

There are 4 access models specified by the ABI:

  * `global-dynamic`
  * `local-dynamic`
  * `initial-exec`
  * `local-exec`

These are the values that can be used for `-ftls-model=...` and
`__attribute__((tls_model("...")))`

Which model is used relates to:

1. Which module is performing the access:
  1. The main executable
  2. A module in the static TLS set
  3. A module that was loaded after startup, e.g. by `dlopen`
2. Which module the variable being accessed is defined in:
  1. Within the same module (i.e. `local-*`)
  2. In a different module (i.e. `global-*`)

* `global-dynamic` Can be used from anywhere, for any variable.
* `local-dynamic` Can be used by any module, for any variable defined in that
  same module.
* `initial-exec` Can be used by any module for any variable defined in the static
  TLS set.
* `local-exec` Can be used by the main executable for variables defined in the
  main executable.

###### Global Dynamic ######

Global dynamic is the most general access format. It is also the slowest.
Any thread-local global variable should be accessible with this method. This
access model *must* be used if a dynamic library accesses a symbol defined in
another module (see exception in section on Initial Exec). Symbols defined
within the executable need not use this access model. The main executable can
also avoid using this access model. This is the default access model when
compiling with `-fPIC` as is the norm for shared libraries.

This access model works by calling a function defined in the dynamic linker.
There are two ways functions might be called, via TLSDESC, or via
`__tls_get_addr`.

In the case of `__tls_get_addr` it is passed the pair of `GOT` entries
associated with this symbol. Specifically it is passed the pointer to the first
and the second entry comes right after it. For a given symbol `S`, the first
entry, denoted `GOT_S[0]`, must contain the Module ID of the module in which
`S` was defined. The second entry, denoted `GOT_S[1]`, must contain offset into
TLS Block, which is the same as the offset of the symbol in the `PT_TLS` segment
of the associated module. The pointer to `S` is then computed using
`__tls_get_addr(GOT_S)`. The implementation of `__tls_get_addr` will be
discussed later.

TLSDESC is an alternative ABI for `global-dynamic` access (and `local-dynamic`)
where a different pair of `GOT` slots are used where the first `GOT` slot
contains a function pointer. The second contains some dynamic linker defined
auxiliary data. This allows the dynamic linker a choice over which function is
called depending on circumstance.

In both cases the calls to these functions must be implemented by a specific
code sequence and a specific set of relocs. This allows the linker to recognize
these accesses and potentially relax them to the `local-dynamic` access model.

(NOTE: The following paragraph contains details about how the compiler upholds
its end of the ABI. Skip this paragraph if you don't care about that.)

For the compiler to emit code for this access model a call needs to be emitted
against `__tls_get_addr` (defined by the dynamic linker) and a reference to the
symbol name. Specifically the compiler the emits code for (minding the
additional relocation needed for the GOT itself) `__tls_get_addr(GOT_S)`. The
linker then emits two dynamic relocations when generating the GOT. On `x86_64`
these are `R_X86_64_DTPMOD` and `R_X86_64_DTPOFF`. On AArch64 these are
`R_AARCH64_DTPMOD` and `R_AARCH64_DTPOFF`. These relocations reference the symbol
regardless of whether or not the module defines a symbol by that name or not.

###### Local Dynamic ######

Local dynamic the same as Global Dynamic but for local symbols. It can be
thought of as a single `global-dynamic` access to the TLS block of this module.
Then because every variable defined in the module is at fixed offsets from the
TLS block the compiler can optimize multiple `global-dynamic` calls into one.
The compiler will relax a `global-dynamic` access to a `local-dynamic` access
whenever the variables are local/static or have hidden visibility. The linker
may sometimes be able to relax some `global-dynamic` accesses to `local-dynamic`
as well.

The following gives an example of how the compiler might emit code for this
access model:

```
static thread_local char buf[buf_cap];
static thread_local size_t buf_size = 0;
while(*str && buf_size < buf_cap) {
  buf[buf_size++] = *str++;
}
```

might be lowered to

```
// GOT_module[0] is the module ID of this module
// GOT_module[1] is just 0
// <X> denotes the offset of X in this module's TLS block
tls = __tls_get_addr(GOT_module)
while(*str && *(size_t*)(tls+<buf_size>) < buf_cap) {
  (char*)(tls+<buf>)[*(size_t*)(tls+<buf_size>)++] = *str++;
}
```

If this code used global dynamic it would have to make at least 2 calls, one to
get the pointer for buf and the other to get the pointer for `buf_size`.

###### Initial Exec ######

This access model can be used anytime the compiler knows the module that the
symbol being accessed is defined in will be loaded in the initial set of
executables rather than opened using `dlopen`. This access model is generally
only used when the main executable is accessing a global symbol with default
visibility. This is because compiling an executable is the only time the
compiler knows that any code generated will be in the initial executable set. If
a DSO is compiled to make thread local accesses use this model then the DSO
cannot be safely opened with `dlopen`. This is acceptable in performance
critical applications and in cases where you know the binary will never be
dlopen-ed such as in the case of libc. Modules compiled/linked this way have
their `DF_STATIC_TLS` flag set.

Initial Exec is the default when compiling without `-fPIC`.

The compiler emits code without even calling `__tls_get_addr` for this access
model. It does so using a single GOT entry, which we'll denote `GOT_s` for symbol
`s`, for which the compiler emits relocations, to ensure that

```
extern thread_local int a;
extern thread_local int b;
int main() {
  return a + b;
}
```

would be lowered to something like the following

```
int main() {
  return *(int*)($tp + GOT[a]) + *(int*)($tp + GOT[b]);
}
```

Note that on x86 architectures `GOT[s]` will actually resolve to a negative
value.

###### Local Exec ######

This is the fastest access model and can only be used if the symbol is in the
first TLS block, which is the TLS block of the main executable. In practice only
the main executable can use this access mode because any shared library can't
(and normally wouldn't need to) know if it is accessing something from the main
executable. The linker will relax `initial-exec` to `local-exec`. The compiler
can't do this without explicit instructions via `-ftls-model` or
`__attribute__((tls_model("...")))` because the compiler cannot know if the
current translation unit is going to be linked into a main executable or a
shared library.

The precise details of how this offset is computed changes a bit
from architecture to architecture.

example code:

```
static thread_local int a;
static thread_local int b;

int main() {
  return a + b;
}
```

would be lowered to

```
int main() {
  return (int*)($tp+TPOFF_a) + (int*)($tp+TPOFF_b));
}
```

On AArch64 `TPOFF_a == max(16, p_align) + <a>` where `p_align` is exactly the
`p_align` field of the main executable's `PT_TLS` segment and `<a>` is the
offset of `a` from the beginning of the main executable's TLS segment.

On `x86_64` `TPOFF_a == -<a>` where `<a>` is the offset of the `a` from the *end*
of the main executable's TLS segment.

The linker is aware of what `TPOFF_X` is for any given `X` and fills in this
value.

## Implementation ##

This section discusses the implementation as it is implemented on Fuchsia. This
said the broad strokes here are widely similar across different libc
implementations including musl and glibc.

The actual implementation of all of this introduces a few more details. Namely
the so-called "DTV" (Dynamic Thread Vector) (denoted `dtv` in this doc), which
indexes TLS blocks by module ID. The following diagram shows what the initial
executable set looks like. In Fuchsia's implementation we actually store a
bunch of meta information in a thread descriptor struct along with the
ABI TCB (denoted `tcb` below). In our implementation we use the first 8 bytes
of this space to point to the DTV. At first `tcb` points to `dtv` as shown in
the below diagrams but after a dlopen this can change.

arm64:

```
*------------------------------------------------------------------------------*
| thread | tcb | X | tls1 | ... | tlsN | ... | tls_cnt | dtv[1] | ... | dtv[N] |
*------------------------------------------------------------------------------*
^         ^         ^             ^            ^
td        tp      dtv[1]       dtv[n+1]       dtv
```

Here `X` has size `min(16, tls_align) - 16` where `tls_align` is the maximum
alignment of all loaded TLS segments from the static TLS set. This is set by
the static linker since the static linker resolves `TPOFF_*` values. This
padding is set that so that if, as required, `$tp` is aligned to main
executable's `PT_TLS` segment's `p_align` value then `tls1 - $tp` will be
`max(16, p_align)`. This ensures that there is always at least a 16 byte space
for the ABI TCB (denoted `tcb` in the diagram above).

x86:

```
*-----------------------------------------------------------------------------*
| tls_cnt | dtv[1] | ... | dtv[N] | ... | tlsN | ... | tls1 | tcb |  thread   |
*-----------------------------------------------------------------------------*
^                                       ^             ^       ^
dtv                                  dtv[n+1]       dtv[1]  tp/td
```

Here `td` denotes the "thread descriptor pointer". In both implementations this
points to the thread descriptor. A subtle point not made apparent in these
diagrams is that `tcb` is actually a member of the thread descriptor struct in
both cases but on AArch64 it is the last member and on `x86_64` it is the first
member.

#### dlopen ####

This picture explains what happens for the initial executables but it doesn't
explain what happens in the `dlopen` case. When `__tls_get_addr` is called it
first checks to see if `tls_cnt` is such that the module ID (given by `GOT_s[0]`
) is within the `dtv`. If it is then it simply looks up `dtv[GOT_s[0]] + GOT_s[1]`
but if it isn't something more complicated happens. See the implementation of
`__tls_get_new` in [dynlink.c](/zircon/third_party/ulib/musl/ldso/dynlink.c).

In a nutshell a sufficiently large space was already allocated for a larger `dtv`
on a call to `dlopen`. It is an invariant of the system that sufficient space
will always exist somewhere already allocated. The larger space is then setup to
be a proper `dtv`. `tcb` is then set to point to this new larger `dtv`. Future
accesses will then use the simpler code path since `tls_cnt` will be large
enough.
