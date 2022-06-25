# Banjo tutorial

Caution: This page may contain information that is specific to the legacy
version of the driver framework (DFv1).

Banjo is a "transpiler" (like [FIDL's
`fidlc`](/docs/development/languages/fidl/README.md))
&mdash; a program that converts an interface definition language (**IDL**) into target language
specific files.

This tutorial is structured as follows:

* brief overview of Banjo
* simple example (I2C)
* explanation of generated code from example

There's also a reference section that includes:

* a list of builtin keywords and primitive types.

## Overview

Banjo generates C and C++ code that can be used by both the protocol implementer
and the protocol user.

## A simple example

As a first step, let's take a look at a relatively simple Banjo specification.
This is the file [`//sdk/banjo/fuchsia.hardware.i2c/i2c.fidl`](/sdk/banjo/fuchsia.hardware.i2c/i2c.fidl):

> Note that the line numbers in the code samples throughout this tutorial are not part of the files.

```banjo
[01] // Copyright 2018 The Fuchsia Authors. All rights reserved.
[02] // Use of this source code is governed by a BSD-style license that can be
[03] // found in the LICENSE file.
[04]
[05] library fuchsia.hardware.i2c;
[06]
[07] using zx;
[08]
[09] const uint32 I2C_10_BIT_ADDR_MASK = 0xF000;
[10] const uint32 I2C_MAX_RW_OPS = 8;
[11]
[12] /// See `Transact` below for usage.
[13] struct I2cOp {
[14]     vector<voidptr> data;
[15]     bool is_read;
[16]     bool stop;
[17] };
[18]
[19] [Transport = "Banjo", BanjoLayout = "ddk-protocol"]
[20] protocol I2c {
[21]     /// Writes and reads data on an i2c channel. Up to I2C_MAX_RW_OPS operations can be passed in.
[22]     /// For write ops, i2c_op_t.data points to data to write.  The data to write does not need to be
[23]     /// kept alive after this call.  For read ops, i2c_op_t.data is ignored.  Any combination of reads
[24]     /// and writes can be specified.  At least the last op must have the stop flag set.
[25]     /// The results of the operations are returned asynchronously through the transact_cb.
[26]     /// The cookie parameter can be used to pass your own private data to the transact_cb callback.
[27]     [Async]
[28]     Transact(vector<I2cOp> op) -> (zx.status status, vector<I2cOp> op);
[29]     /// Returns the maximum transfer size for read and write operations on the channel.
[30]     GetMaxTransferSize() -> (zx.status s, usize size);
[31]     GetInterrupt(uint32 flags) -> (zx.status s, handle<interrupt> irq);
[32] };
```

It defines an interface that allows an application to read and write data on an I2C bus.
In the I2C bus, data must first be written to the device in order to solicit
a response.
If a response is desired, the response can be read from the device.
(A response might not be required when setting a write-only register, for example.)

Let's look at the individual components, line-by-line:

* `[05]` &mdash; the `library` directive tells the Banjo compiler what prefix it should
  use on the generated output; think of it as a namespace specifier.
* `[07]` &mdash; the `using` directive tells Banjo to include the `zx` library.
* `[09]` and `[10]` &mdash; these introduce two constants for use by the programmer.
* `[13` .. `17]` &mdash; these define a structure, called `I2cOp`, that the programmer
  will then use for transferring data to and from the bus.
* `[19` .. `32]` &mdash; these lines define the interface methods that are provided by
  this Banjo specification; we'll discuss this in greater detail [below](#the-interface).

> Don't be confused by the comments on `[21` .. `26]` (and elsewhere) &mdash; they're
> "flow through" comments that are intended to be emitted into the generated source.
> Any comment that starts with "`///`" (three! slashes) is a "flow through" comment.
> Ordinary comments (that is, "`//`") are intended for the current module.
> This will become clear when we look at the generated code.

## The operation structure

In our I2C sample, the `struct I2cOp` structure defines three elements:

Element   | Type              | Use
----------|-------------------|-----------------------------------------------------------------
`data`    | `vector<voidptr>` | contains the data sent to, and optionally received from, the bus
`is_read` | `bool`            | flag indicating read functionality desired
`stop`    | `bool`            | flag indicating a stop byte should be sent after the operation

The structure defines the communications area that will be used between the protocol
implementation (the driver) and the protocol user (the program that's using the bus).

## The interface

The more interesting part is the `protocol` specification.

We'll skip the `[Transport = "Banjo", BanjoLayout]` (line `[19]`) and `[Async]` (line `[27]`) attributes for now,
but will return to them below, in [Attributes](#attributes).

The `protocol` section defines three interface methods:

* `Transact`
* `GetMaxTransferSize`
* `GetInterrupt`

Without going into details about their internal operations (this isn't a tutorial on
I2C, after all), let's see how they translate into the target language.
We'll look at the C and C++ implementations separately, using the C description
to include the structure definition that's common to the C++ version as well.

> Currently, generation of C and C++ code is supported, with Rust support planned
> in the future.

## C

The C implementation is relatively straightforward:

* `struct`s and `union`s map almost directly into their C language counterparts.
* `enum`s and constants are generated as `#define` macros.
* `protocol`s are generated as two `struct`s:
    * a function table, and
    * a struct with pointers to the function table and a context.
* Some helper functions are also generated.

The C version is generated into
`$BUILD_DIR/banjoing/gen/fuchisia/hardware/i2c/c/banjo.h`,
where _TARGET_ is the target architecture, e.g., `arm64`.

The file is relatively long, so we'll look at it in several parts.

### Boilerplate

The first part has some boilerplate, which we'll show without further comment:

```c
[01] // Copyright 2018 The Fuchsia Authors. All rights reserved.
[02] // Use of this source code is governed by a BSD-style license that can be
[03] // found in the LICENSE file.
[04]
[05] // WARNING: THIS FILE IS MACHINE GENERATED. DO NOT EDIT.
[06] //          MODIFY sdk/banjo/fuchsia.hardware.i2c/i2c.banjo INSTEAD.
[07]
[08] #pragma once
[09]
[10] #include <zircon/compiler.h>
[11] #include <zircon/types.h>
[12]
[13] __BEGIN_CDECLS
```

### Forward declarations

Next are forward declarations for our structures and functions:

```c
[15] // Forward declarations
[16]
[17] typedef struct i2c_op i2c_op_t;
[18] typedef struct i2c_protocol i2c_protocol_t;
[19] typedef void (*i2c_transact_callback)(void* ctx, zx_status_t status, const i2c_op_t* op_list, size_t op_count);
[20]
[21] // Declarations
[22]
[23] // See `Transact` below for usage.
[24] struct i2c_op {
[25]     const void* data_buffer;
[26]     size_t data_size;
[27]     bool is_read;
[28]     bool stop;
[29] };
```

Note that lines `[17` .. `19]` only declare types, they don't actually define
structures or prototypes for functions.

Notice how the "flow through" comments (original `.banjo` file line `[12]`, for example)
got emitted into the generated code (line `[23]` above), with one slash stripped off to
make them look like normal comments.

Lines `[24` .. `29`] are, as advertised, an almost direct mapping of the `struct I2cOp`
from the `.banjo` file above (lines `[13` .. `17`]).

Astute C programmers will immediately see how the C++ style `vector<voidptr> data` (original
`.banjo` file line `[14]`) is handled in C: it gets converted to a pointer
("`data_buffer`") and a size ("`data_size`").

> As far as the naming goes, the base name is `data` (as given in the `.banjo` file).
> For a vector of `voidptr`, the transpiler appends `_buffer` and `_size` to convert the
> `vector` into a C compatible structure.
> For all other vector types, the transpiler appends `_list` and `_count` instead (for
> code readability).

### Constants

Next, we see our `const uint32` constants converted into `#define` statements:

```c
[31] #define I2C_MAX_RW_OPS UINT32_C(8)
[32]
[33] #define I2C_10_BIT_ADDR_MASK UINT32_C(0xF000)
```

In the C version, We chose `#define` instead of "passing through" the `const uint32_t`
representation because:

* `#define` statements only exist at compile time, and get inlined at every usage site, whereas
  a `const uint32_t` would get embedded in the binary, and
* `#define` allows for more compile time optimizations (e.g., doing math with the constant value).

The downside is that we don't get type safety, which is why you see the helper macros (like
**UINT32_C()** above); they just cast the constant to the appropriate type.

Note: Adding the `[Namespaced]` attribute to constant declarations for
Banjo C bindings will cause the variable name to be prefaced by the FIDL
library name. In this example, adding the `[Namespaced]` attribute to `I2C_MAX_RW_OPS`
would cause the variable name to be `fuchsia_hardware_i2c_I2C_MAX_RW_OPS`
instead. This may be required to avoid name conflicts with FIDL hlcpp constant
bindings in the same build target.

### Protocol structures

And now we get into the good parts.

```c
[35] typedef struct i2c_protocol_ops {
[36]     void (*transact)(void* ctx, const i2c_op_t* op_list, size_t op_count, i2c_transact_callback callback, void* cookie);
[37]     zx_status_t (*get_max_transfer_size)(void* ctx, size_t* out_size);
[38]     zx_status_t (*get_interrupt)(void* ctx, uint32_t flags, zx_handle_t* out_irq);
[39] } i2c_protocol_ops_t;
```

This `typedef` creates a structure definition that contains the three `protocol` methods
that were defined in the original `.banjo` file at lines `[28]`, `[30]` and `[31]`.

Notice the name mangling that has occurred &mdash; this is how you can map the
`protocol` method names to the C function pointer names so that you know what
they're called:

Banjo                | C                       | Rule
---------------------|-------------------------|---------------------------------------------------------------
`Transact`           | `transact`              | Convert leading uppercase to lowercase
`GetMaxTransferSize` | `get_max_transfer_size` | As above, and convert camel-case to underscore-separated style
`GetInterrupt`       | `get_interrupt`         | Same as above

Next, the interface definitions are wrapped in a context-bearing structure:

```c
[41] struct i2c_protocol {
[42]     i2c_protocol_ops_t* ops;
[43]     void* ctx;
[44] };
```

And now the "flow-through" comments (`.banjo` file, lines `[21` .. `26]`)
suddenly make way more sense!

```c
[46] // Writes and reads data on an i2c channel. Up to I2C_MAX_RW_OPS operations can be passed in.
[47] // For write ops, i2c_op_t.data points to data to write.  The data to write does not need to be
[48] // kept alive after this call.  For read ops, i2c_op_t.data is ignored.  Any combination of reads
[49] // and writes can be specified.  At least the last op must have the stop flag set.
[50] // The results of the operations are returned asynchronously through the transact_cb.
[51] // The cookie parameter can be used to pass your own private data to the transact_cb callback.
```

Finally, we see the actual generated code for the three methods:

```c
[52] static inline void i2c_transact(const i2c_protocol_t* proto, const i2c_op_t* op_list, size_t op_count, i2c_transact_callback callback, void* cookie) {
[53]     proto->ops->transact(proto->ctx, op_list, op_count, callback, cookie);
[54] }
[55] // Returns the maximum transfer size for read and write operations on the channel.
[56] static inline zx_status_t i2c_get_max_transfer_size(const i2c_protocol_t* proto, size_t* out_size) {
[57]     return proto->ops->get_max_transfer_size(proto->ctx, out_size);
[58] }
[59] static inline zx_status_t i2c_get_interrupt(const i2c_protocol_t* proto, uint32_t flags, zx_handle_t* out_irq) {
[60]     return proto->ops->get_interrupt(proto->ctx, flags, out_irq);
[61] }
```

### Prefixes and paths

Notice how the prefix `i2c_` (from the interface name, `.banjo` file line `[20]`)
got added to the method names; thus, `Transact` became `i2c_transact`, and so on.
This is part of the mapping between `.banjo` names and their C equivalents.

Also, the `library` name (line `[05]` in the `.banjo` file) is transformed into the
include path: so `library fuchsia.hardware.i2c` implies a path of `<fuchsia/hardware/i2c/c/banjo.h>`.

## C++

The C++ code is slightly more complex than the C version.
Let's take a look.

The Banjo transpiler generates three files:
the first is the C file discussed above, and the other two are under
`$BUILD_DIR/banjoing/gen/fuchsia/hardware/i2c/cpp/banjo.h`:

* `i2c.h` &mdash; the file your program should include, and
* `i2c-internal.h` &mdash; an internal file, included by `i2c.h`

As usual, _TARGET_ is the build target architecture (e.g., `x64`).

The "internal" file contains declarations and assertions, which we can safely skip.

The C++ version of `i2c.h` is fairly long, so we'll look at it in smaller pieces.
Here's an overview "map" of what we'll be looking at, showing the starting line
number of each piece:

Line | Section
--------------|----------------------------
1    | [boilerplate](#a-simple-example-c-boilerplate-2)
20   | [auto generated usage comments](#auto_generated-comments)
55   | [class I2cProtocol](#the-i2cprotocol-mixin-class)
99   | [class I2cProtocolClient](#the-i2cprotocolclient-wrapper-class)

### Boilerplate

The boilerplate is pretty much what you'd expect:

```c++
[001] // Copyright 2018 The Fuchsia Authors. All rights reserved.
[002] // Use of this source code is governed by a BSD-style license that can be
[003] // found in the LICENSE file.
[004]
[005] // WARNING: THIS FILE IS MACHINE GENERATED. DO NOT EDIT.
[006] //          MODIFY sdk/banjo/fuchsia.hardware.i2c/i2c.banjo INSTEAD.
[007]
[008] #pragma once
[009]
[010] #include <ddk/driver.h>
[011] #include <fuchsia/hardware/i2c/c/banjo.h>
[012] #include <ddktl/device-internal.h>
[013] #include <zircon/assert.h>
[014] #include <zircon/compiler.h>
[015] #include <zircon/types.h>
[016] #include <lib/zx/interrupt.h>
[017]
[018] #include "i2c-internal.h"
```

It `#include`s a bunch of DDK and OS headers, including:

* the C version of the header (line `[011]`, which means that everything discussed
  [above in the C section](#a-simple-example-c-1) applies here as well), and
* the generated `i2c-internal.h` file (line `[018]`).

Next is the "auto generated usage comments" section; we'll come back to that
[later](#auto_generated-comments) as it will make more sense once we've seen
the actual class declarations.

The two class declarations are wrapped in the DDK namespace:

```c++
[053] namespace ddk {
...
[150] } // namespace ddk
```

### The I2cProtocolClient wrapper class

The `I2cProtocolClient` class is a simple wrapper around the `i2c_protocol_t`
structure (defined in the C include file, line `[41]`, which we discussed in
[Protocol structures](#protocol-structures), above).

```c++
[099] class I2cProtocolClient {
[100] public:
[101]     I2cProtocolClient()
[102]         : ops_(nullptr), ctx_(nullptr) {}
[103]     I2cProtocolClient(const i2c_protocol_t* proto)
[104]         : ops_(proto->ops), ctx_(proto->ctx) {}
[105]
[106]     I2cProtocolClient(zx_device_t* parent) {
[107]         i2c_protocol_t proto;
[108]         if (device_get_protocol(parent, ZX_PROTOCOL_I2C, &proto) == ZX_OK) {
[109]             ops_ = proto.ops;
[110]             ctx_ = proto.ctx;
[111]         } else {
[112]             ops_ = nullptr;
[113]             ctx_ = nullptr;
[114]         }
[115]     }
[116]
[117]     void GetProto(i2c_protocol_t* proto) const {
[118]         proto->ctx = ctx_;
[119]         proto->ops = ops_;
[120]     }
[121]     bool is_valid() const {
[122]         return ops_ != nullptr;
[123]     }
[124]     void clear() {
[125]         ctx_ = nullptr;
[126]         ops_ = nullptr;
[127]     }
[128]     // Writes and reads data on an i2c channel. Up to I2C_MAX_RW_OPS operations can be passed in.
[129]     // For write ops, i2c_op_t.data points to data to write.  The data to write does not need to be
[130]     // kept alive after this call.  For read ops, i2c_op_t.data is ignored.  Any combination of reads
[131]     // and writes can be specified.  At least the last op must have the stop flag set.
[132]     // The results of the operations are returned asynchronously through the transact_cb.
[133]     // The cookie parameter can be used to pass your own private data to the transact_cb callback.
[134]     void Transact(const i2c_op_t* op_list, size_t op_count, i2c_transact_callback callback, void* cookie) const {
[135]         ops_->transact(ctx_, op_list, op_count, callback, cookie);
[136]     }
[137]     // Returns the maximum transfer size for read and write operations on the channel.
[138]     zx_status_t GetMaxTransferSize(size_t* out_size) const {
[139]         return ops_->get_max_transfer_size(ctx_, out_size);
[140]     }
[141]     zx_status_t GetInterrupt(uint32_t flags, zx::interrupt* out_irq) const {
[142]         return ops_->get_interrupt(ctx_, flags, out_irq->reset_and_get_address());
[143]     }
[144]
[145] private:
[146]     i2c_protocol_ops_t* ops_;
[147]     void* ctx_;
[148] };
```

There are three constructors:

* the default one (`[101]`) that sets `ops_` and `ctx_` to `nullptr`,
* an initializer (`[103]`) that takes a pointer to an `i2c_protocol_t` structure and populates
  the `ops_` and `ctx`_ fields from their namesakes in the structure, and
* another initializer (`[106]`) that extracts the `ops`_ and `ctx_` information from
  a `zx_device_t`.

The last constructor is the preferred one, and can be used like this:

```c++
ddk::I2cProtocolClient i2c(parent);
if (!i2c.is_valid()) {
  return ZX_ERR_*; // return an appropriate error
}
```

Three convenience member functions are provided:

* `[117]` **GetProto()** fetches the `ctx_` and `ops_` members into a protocol structure,
* `[121]` **is_valid()** returns a `bool` indicating if the class has been initialized with
   a protocol, and
* `[124]` **clear()** invalidates the `ctx_` and `ops_` pointers.

Next we find the three member functions that were specified in the `.banjo` file:

* `[134]` **Transact()**,
* `[138]` **GetMaxTransferSize()**, and
* `[141]` **GetInterrupt()**.

These work just liked the three wrapper functions from the C version of the include file &mdash;
that is, they pass their arguments into a call through the respective function pointer.

In fact, compare **i2c_get_max_transfer_size()** from the C version:

```c
[56] static inline zx_status_t i2c_get_max_transfer_size(const i2c_protocol_t* proto, size_t* out_size) {
[57]     return proto->ops->get_max_transfer_size(proto->ctx, out_size);
[58] }
```

with the C++ version above:

```c++
[138] zx_status_t GetMaxTransferSize(size_t* out_size) const {
[139]   return ops_->get_max_transfer_size(ctx_, out_size);
[140] }
```

As advertised, all that this class does is store the operations and context pointers for
later use, so that the call through the wrapper is more elegant.

> You'll also notice that the C++ wrapper function doesn't have any name mangling &mdash;
> to use a tautology, **GetMaxTransferSize()** is **GetMaxTransferSize()**.

### The I2cProtocol mixin class

Ok, that was the easy part.
For this next part, we're going to talk about [mixins](https://en.wikipedia.org/wiki/Mixin)
and [CRTPs &mdash; or Curiously Recurring Template
Patterns](https://en.wikipedia.org/wiki/Curiously_recurring_template_pattern).

Let's understand the "shape" of the class first (comment lines deleted for outlining
purposes):

```c++
[055] template <typename D, typename Base = internal::base_mixin>
[056] class I2cProtocol : public Base {
[057] public:
[058]     I2cProtocol() {
[059]         internal::CheckI2cProtocolSubclass<D>();
[060]         i2c_protocol_ops_.transact = I2cTransact;
[061]         i2c_protocol_ops_.get_max_transfer_size = I2cGetMaxTransferSize;
[062]         i2c_protocol_ops_.get_interrupt = I2cGetInterrupt;
[063]
[064]         if constexpr (internal::is_base_proto<Base>::value) {
[065]             auto dev = static_cast<D*>(this);
[066]             // Can only inherit from one base_protocol implementation.
[067]             ZX_ASSERT(dev->ddk_proto_id_ == 0);
[068]             dev->ddk_proto_id_ = ZX_PROTOCOL_I2C;
[069]             dev->ddk_proto_ops_ = &i2c_protocol_ops_;
[070]         }
[071]     }
[072]
[073] protected:
[074]     i2c_protocol_ops_t i2c_protocol_ops_ = {};
[075]
[076] private:
...
[083]     static void I2cTransact(void* ctx, const i2c_op_t* op_list, size_t op_count, i2c_transact_callback callback, void* cookie) {
[084]         static_cast<D*>(ctx)->I2cTransact(op_list, op_count, callback, cookie);
[085]     }
...
[087]     static zx_status_t I2cGetMaxTransferSize(void* ctx, size_t* out_size) {
[088]         auto ret = static_cast<D*>(ctx)->I2cGetMaxTransferSize(out_size);
[089]         return ret;
[090]     }
[091]     static zx_status_t I2cGetInterrupt(void* ctx, uint32_t flags, zx_handle_t* out_irq) {
[092]         zx::interrupt out_irq2;
[093]         auto ret = static_cast<D*>(ctx)->I2cGetInterrupt(flags, &out_irq2);
[094]         *out_irq = out_irq2.release();
[095]         return ret;
[096]     }
[097] };
```

The `I2CProtocol` class inherits from a base class, specified by the second template parameter.
If it's left unspecified, it defaults to `internal::base_mixin`, and no special magic happens.
If, however, the base class is explicitly specified, it should be `ddk::base_protocol`,
in which case additional asserts are added (to double check that only one mixin is the base protocol).
In addition, special DDKTL fields are set to automatically register this protocol as the
base protocol when the driver triggers **DdkAdd()**.

The constructor calls an internal validation function, **CheckI2cProtocolSubclass()** `[059]`
(defined in the generated `i2c-internal.h` file), which has several **static_assert()** calls.
The class `D` is expected to implement the three member functions (**I2cTransact()**,
**I2cGetMaxTransferSize()**, and **I2cGetInterrupt()**) in order for the static methods to work.
If they're not provided by `D`, then the compiler would (in the absence of the static
asserts) produce gnarly templating errors.
The static asserts serve to produce diagnostic errors that are understandable by mere humans.

Next, the three pointer-to-function operations members (`transact`,
`get_max_transfer_size`, and `get_interrupt`) are bound (lines `[060` .. `062]`).

Finally, the `constexpr` expression provides a default initialization if required.

### Using the mixin class

The `I2cProtocol` class can be used as follows (from
[`//src/devices/bus/drivers/platform/platform-proxy.h`](/src/devices/bus/drivers/platform/platform-proxy.h)):

```c++
[01] class ProxyI2c : public ddk::I2cProtocol<ProxyI2c> {
[02] public:
[03]     explicit ProxyI2c(uint32_t device_id, uint32_t index, fbl::RefPtr<PlatformProxy> proxy)
[04]         : device_id_(device_id), index_(index), proxy_(proxy) {}
[05]
[06]     // I2C protocol implementation.
[07]     void I2cTransact(const i2c_op_t* ops, size_t cnt, i2c_transact_callback transact_cb,
[08]                      void* cookie);
[09]     zx_status_t I2cGetMaxTransferSize(size_t* out_size);
[10]     zx_status_t I2cGetInterrupt(uint32_t flags, zx::interrupt* out_irq);
[11]
[12]     void GetProtocol(i2c_protocol_t* proto) {
[13]         proto->ops = &i2c_protocol_ops_;
[14]         proto->ctx = this;
[15]     }
[16]
[17] private:
[18]     uint32_t device_id_;
[19]     uint32_t index_;
[20]     fbl::RefPtr<PlatformProxy> proxy_;
[21] };
```

Here we see that `class ProxyI2c` inherits from the DDK's `I2cProtocol` and provides
itself as the argument to the template &mdash; this is the "mixin" concept.
This causes the `ProxyI2c` type to be substituted for `D` in the template definition
of the class (from the `i2c.h` header file above, lines `[084]`, `[088]`, and `[093]`).

Taking a look at just the **I2cGetMaxTransferSize()** function as an example, it's
effectively as if the source code read:

```c++
[087] static zx_status_t I2cGetMaxTransferSize(void* ctx, size_t* out_size) {
[088]     auto ret = static_cast<ProxyI2c*>(ctx)->I2cGetMaxTransferSize(out_size);
[089]     return ret;
[090] }
```

This ends up eliminating the cast-to-self boilerplate in your code.
This casting is necessary because the type information is erased at the DDK boundary &mdash;
recall that the context `ctx` is a `void *` pointer.

### Auto-generated comments

Banjo automatically generates comments in the include file that basically summarize what we
talked about above:

```c++
[020] // DDK i2c-protocol support
[021] //
[022] // :: Proxies ::
[023] //
[024] // ddk::I2cProtocolClient is a simple wrapper around
[025] // i2c_protocol_t. It does not own the pointers passed to it
[026] //
[027] // :: Mixins ::
[028] //
[029] // ddk::I2cProtocol is a mixin class that simplifies writing DDK drivers
[030] // that implement the i2c protocol. It doesn't set the base protocol.
[031] //
[032] // :: Examples ::
[033] //
[034] // // A driver that implements a ZX_PROTOCOL_I2C device.
[035] // class I2cDevice;
[036] // using I2cDeviceType = ddk::Device<I2cDevice, /* ddk mixins */>;
[037] //
[038] // class I2cDevice : public I2cDeviceType,
[039] //                   public ddk::I2cProtocol<I2cDevice> {
[040] //   public:
[041] //     I2cDevice(zx_device_t* parent)
[042] //         : I2cDeviceType(parent) {}
[043] //
[044] //     void I2cTransact(const i2c_op_t* op_list, size_t op_count, i2c_transact_callback callback, void* cookie);
[045] //
[046] //     zx_status_t I2cGetMaxTransferSize(size_t* out_size);
[047] //
[048] //     zx_status_t I2cGetInterrupt(uint32_t flags, zx::interrupt* out_irq);
[049] //
[050] //     ...
[051] // };
```

# Using Banjo

<!--
> Suraj says:
>> We also need something in-between a FIDL tutorial and a driver writing tutorial,
>> in order to describe banjo usage.
>> Basically, writing a simple protocol, and then describing a driver that emits
>> it, and another driver that binds on top of it and makes use of that protocol.
>> If it makes sense, the existing driver writing tutorial could just be modified
>> to have more fleshed out details on banjo usage.
>> I think the current driver tutorial is focused on C usage as well, and getting
>> a C++ version (using ddktl) would probably bring the most value [this is
>> already on my work queue, "Tutorial on using ddktl (C++ DDK wrappers)" -RK].
-->
Now that we've seen the generated code for the I2C driver, let's take a look
at how we would use it.

> @@@ to be completed

# Reference

> @@@ This is where we should list all builtin keywords and primitive types

## Attributes

Recall from the example above that the `protocol` section had two attributes;
a `[Transport = "Banjo", BanjoLayout]` and an `[Async]` attribute.

### The BanjoLayout attribute

The line just before the `protocol` is the `[Transport = "Banjo", BanjoLayout]` attribute:

```banjo
[19] [Transport = "Banjo", BanjoLayout = "ddk-protocol"]
[20] protocol I2c {
```

The attribute applies to the next item; so in this case, the entire `protocol`.
Only one layout is allowed per interface.

There are in fact 3 `BanjoLayout` attribute types currently supported:

* `ddk-protocol`
* `ddk-interface`
* `ddk-callback`

In order to understand how these layout types work, let's assume we have two drivers,
`A` and `B`.
Driver `A` spawns a device, which `B` then attaches to, (making `B` a child of `A`).

If `B` then queries the DDK for its parent's "protocol" through **device_get_protocol()**,
it'll get a `ddk-protocol`.
A `ddk-protocol` is a set of callbacks that a parent provides to its child.

One of the protocol functions can be to register a "reverse-protocol", whereby
the child provides a set of callbacks for the parent to trigger instead.
This is a `ddk-interface`.

From a code generation perspective, these two (`ddk-protocol` and `ddk-interface`)
look almost identical, except for some slight naming differences (`ddk-protocol`
automatically appends the word "protocol" to the end of generated structs / classes,
whereas `ddk-interface` doesn't).

`ddk-callback` is a slight optimization over `ddk-interface`, and is used when an
interface has just one single function.
Instead of generating two structures, like:

```c
struct interface {
   void* ctx;
   inteface_function_ptr_table* callbacks;
};

struct interface_function_ptr_table {
   void (*one_function)(...);
}
```

a `ddk-callback` will generate a single structure with the function pointer inlined:

```c
struct callback {
  void* ctx;
  void (*one_function)(...);
};
```

### The Async attribute

Within the `protocol` section, we see another attribute: the `[Async]` attribute:

```banjo
[20] protocol I2c {
...      /// comments (removed)
[27]     [Async]
```

The `[Async]` attribute is a way to make protocol messages not be synchronous.
It autogenerates a callback type in which the output arguments are inputs to the callback.
The original method will not have any of the output parameters specified in its signatures.

Recall from the example above that we had a `Transact` method:

```banjo
[27] [Async]
[28] Transact(vector<I2cOp> op) -> (zx.status status, vector<I2cOp> op);
```

When used (as above) in conjunction with the `[Async]` attribute, it means that we want Banjo
to invoke a callback function, so that we can handle the output data (the second
`vector<I2cOp>` above, representing the data from the I2C bus).

Here's how it works.
We send data to the I2C bus through the first `vector<I2cOp>` argument.
Some time later, the I2C bus may generate data in response to our request.
Because we specified `[Async]`, Banjo generates the functions to take a callback function
as input.

In C, these two lines (from the `i2c.h` file) are important:

```c
[19] typedef void (*i2c_transact_callback)(void* ctx, zx_status_t status, const i2c_op_t* op_list, size_t op_count);
...
[36] void (*transact)(void* ctx, const i2c_op_t* op_list, size_t op_count, i2c_transact_callback callback, void* cookie);
```

In C++, we have two place where the callback is referenced:

```c++
[083] static void I2cTransact(void* ctx, const i2c_op_t* op_list, size_t op_count, i2c_transact_callback callback, void* cookie) {
[084]     static_cast<D*>(ctx)->I2cTransact(op_list, op_count, callback, cookie);
[085] }
```

and

```c++
[134] void Transact(const i2c_op_t* op_list, size_t op_count, i2c_transact_callback callback, void* cookie) const {
[135]     ops_->transact(ctx_, op_list, op_count, callback, cookie);
[136] }
```

Notice how the C++ is similar to the C: that's because the generated code includes the
C header file as part of the C++ header file.

The transaction callback has the following arguments:

Argument   | Meaning
-----------|----------------------------------------
`ctx`      | the cookie
`status`   | status of the asynchronous response (provided by callee)
`op_list`  | the data from the transfer
`op_count` | the number of elements in the transfer

How is this different than just using the `ddk-callback` `[Transport = "Banjo", BanjoLayout]` attribute we
discussed above?

First, there's no `struct` with the callback and cookie value in it, they're inlined
as arguments instead.

Second, the callback provided is a "one time use" function.
That is to say, it should be called once, and only once, for each invocation of the
protocol method it was supplied to.
For contrast, a method provided by a `ddk-callback` is a "register once, call
many times" type of function (similar to `ddk-interface` and `ddk-protocol`).
For this reason, `ddk-callback` and `ddk-interface` structures usually have
paired **register()** and **unregister()** calls in order to tell the parent device
when it should stop calling those callbacks.

> One more caveat with `[Async]` is that its callback *MUST* be called for each
> protocol method invocation, and the accompanying cookie must be provided.
> Failure to do so will result in undefined behavior (likely a leak, deadlock,
> timeout, or crash).

Although not the case currently, C++ and future language bindings (like Rust)
will provide "future" / "promise" style based APIs in the generated code, built on top of
these callbacks in order to prevent mistakes.

> Ok, one more caveat with `[Async]` &mdash; the `[Async]` attribute applies *only*
> to the immediately following method; not any other methods.

### The Buffer attribute

This attribute applies to protocol method parameters of the `vector` type to convey that they are
used as buffers. In practice, it only affects the names of the generated parameters.

### The CalleeAllocated attribute

When applied to a protocol method output parameter of type `vector`, the attribute conveys the fact
that the contents of the vector should be allocated by the receiver of the method call.

### The DeriveDebug attribute (C bindings only)

When applied to an enum declaration, a helper `*_to_str()` function
will be generated for C bindings which returns a `const char*` for each
value of the enum. For example, an enum declared with this attribute such
as

```banjo
[DeriveDebug]
enum ExampleEnum {
    VAL_ONE = 1;
    VAL_TWO = 2;
};
```

will result in the following generated definition.

```c
#ifndef FUNC_EXAMPLE_ENUM_TO_STR_
#define FUNC_EXAMPLE_ENUM_TO_STR_
static inline const char* example_enum_to_str(example_enum_t value) {
  switch (value) {
    case EXAMPLE_ENUM_VAL_ONE:
      return "EXAMPLE_ENUM_VAL_ONE";
    case EXAMPLE_ENUM_VAL_TWO:
      return "EXAMPLE_ENUM_VAL_TWO";
  }
  return "UNKNOWN";
}
#endif
```

### The InnerPointer attribute

In the context of a protocol input parameter of type `vector`, this attribute turns the contents of
the vector into pointers to objects instead of objects themselves.

### The InOut attribute

Adding this attribute to a protocol method input parameter makes the parameter mutable, effectively
turning it into an "in-out" parameter.

### The Mutable attribute

This attribute should be used to make `struct`/`union` fields of type `vector` or `string` mutable.

### The Namespaced attribute

This attribute applies to `const` declarations and makes it so that the C backend prefaces the
constant name with the snake-cased FIDL library name, e.g. `library_name_CONSTANT_K` instead
of `CONSTANT_K`. This attribute may be required to avoid name conflicts with FIDL hlcpp constant
bindings in the same build target.

### The OutOfLineContents attribute

This attribute allows the contents of a `vector` field in a `struct`/`union` to be stored outside
of the container.

### The PreserveCNames attribute

This attribute applies to `struct` declarations and makes it so that their fields' names remain
unchanged when run through the C backend.

# Banjo Mocks

Banjo generates a C++ mock class for each protocol. This mock can be passed to protocol users in
tests.

## Building

Tests in Zircon get the mock headers automatically. Tests outsize of Zircon must depend on the
protocol target with a `_mock` suffix, e.g.
`//sdk/banjo/fuchsia.hardware.gpio:fuchsia.hardware.gpio_banjo_cpp_mock`.

## Using the mocks

Test code must include the protocol header with a `mock/` prefix, e.g.
`#include <fuchsia/hardware/gpio/cpp/banjo-mock.h>`.

Consider the following Banjo protocol snippet:

```banjo
[021] [Transport = "Banjo", BanjoLayout = "ddk-protocol"]
[022] protocol Gpio {
 ...
[034]     /// Gets an interrupt object pertaining to a particular GPIO pin.
[035]     GetInterrupt(uint32 flags) -> (zx.status s, handle<interrupt> irq);
 ...
[040] };
```

Here are the corresponding bits of the mock class generated by Banjo:

```c++
[034] class MockGpio : ddk::GpioProtocol<MockGpio> {
[035] public:
[036]     MockGpio() : proto_{&gpio_protocol_ops_, this} {}
[037]
[038]     const gpio_protocol_t* GetProto() const { return &proto_; }
 ...
[065]     virtual MockGpio& ExpectGetInterrupt(zx_status_t out_s, uint32_t flags, zx::interrupt out_irq) {
[066]         mock_get_interrupt_.ExpectCall({out_s, std::move(out_irq)}, flags);
[067]         return *this;
[068]     }
 ...
[080]     void VerifyAndClear() {
 ...
[086]         mock_get_interrupt_.VerifyAndClear();
 ...
[089]     }
 ...
[117]     virtual zx_status_t GpioGetInterrupt(uint32_t flags, zx::interrupt* out_irq) {
[118]         std::tuple<zx_status_t, zx::interrupt> ret = mock_get_interrupt_.Call(flags);
[119]         *out_irq = std::move(std::get<1>(ret));
[120]         return std::get<0>(ret);
[121]     }
```

The MockGpio class implements the GPIO protocol. `ExpectGetInterrupt`
is used to set expectations on how `GpioGetInterrupt` is called. `GetProto` is used to get the
`gpio_protocol_t` that can be passed to the code under test. This code will call `GpioGetInterrupt`
which will ensure that it got called with the correct arguments and will return the value specified
by `ExpectGetInterrupt`. Finally, the test can call `VerifyAndClear` to verify that all expectations
were satisfied. Here is an example test using this mock:

```c++
TEST(SomeTest, SomeTestCase) {
    ddk::MockGpio gpio;

    zx::interrupt interrupt;
    gpio.ExpectGetInterrupt(ZX_OK, 0, zx::move(interrupt))
        .ExpectGetInterrupt(ZX_ERR_INTERNAL, 100, zx::interrupt());

    CodeUnderTest dut(gpio.GetProto());
    EXPECT_OK(dut.DoSomething());

    ASSERT_NO_FATAL_FAILURE(gpio.VerifyAndClear());
}
```

### Equality operator overrides

Tests using Banjo mocks with structure types will have to define equality operator overrides. For
example, for a struct type `some_struct_type` the test will have to define a function with the
signature

```c++
bool operator==(const some_struct_type& lhs, const some_struct_type& rhs);
```

in the top-level namespace.

### Custom mocks

It is expected that some tests may need to alter the default mock behavior. To help with this, all
expectation and protocol methods are `virtual`, and all `MockFunction` members are `protected`.

### Async methods

The Banjo mocks issue callbacks from all async methods by default.
