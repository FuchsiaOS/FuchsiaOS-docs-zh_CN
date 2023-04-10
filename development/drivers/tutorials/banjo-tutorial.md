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
This is the file [`//sdk/banjo/fuchsia.hardware.i2cimpl/i2cimpl.fidl`](/sdk/banjo/fuchsia.hardware.i2cimpl/i2c-impl.fidl):

> Note that the line numbers in the code samples throughout this tutorial are not part of the files.

```banjo
[01] // Copyright 2018 The Fuchsia Authors. All rights reserved.
[02] // Use of this source code is governed by a BSD-style license that can be
[03] // found in the LICENSE file.
[04] @available(added=7)
[05] library fuchsia.hardware.i2cimpl;
[06]
[07] using zx;
[08]
[09] const I2C_IMPL_10_BIT_ADDR_MASK uint32 = 0xF000;
[10] /// The maximum number of I2cImplOp's that may be passed to Transact.
[11] const I2C_IMPL_MAX_RW_OPS uint32 = 8;
[12] /// The maximum length of all read or all write transfers in bytes.
[13] const I2C_IMPL_MAX_TOTAL_TRANSFER uint32 = 4096;
[14]
[15] /// See `Transact` below for usage.
[16] type I2cImplOp = struct {
[17]     address uint16;
[18]     @buffer
[19]     @mutable
[20]     data vector<uint8>:MAX;
[21]     is_read bool;
[22]     stop bool;
[23] };
[24]
[25] /// Low-level protocol for i2c drivers.
[26] @transport("Banjo")
[27] @banjo_layout("ddk-protocol")
[28] protocol I2cImpl {
[29]     /// First bus ID that this I2cImpl controls, zero-indexed.
[30]     GetBusBase() -> (struct {
[31]         base uint32;
[32]     });
[33]     /// Number of buses that this I2cImpl supports.
[34]     GetBusCount() -> (struct {
[35]         count uint32;
[36]     });
[37]     GetMaxTransferSize(struct {
[38]         bus_id uint32;
[39]     }) -> (struct {
[40]         s zx.status;
[41]         size uint64;
[42]     });
[43]     /// Sets the bitrate for the i2c bus in KHz units.
[44]     SetBitrate(struct {
[45]         bus_id uint32;
[46]         bitrate uint32;
[47]     }) -> (struct {
[48]         s zx.status;
[49]     });
[50]     /// |Transact| assumes that all ops buf are not null.
[51]     /// |Transact| assumes that all ops length are not zero.
[52]     /// |Transact| assumes that at least the last op has stop set to true.
[53]     Transact(struct {
[54]         bus_id uint32;
[55]         op vector<I2cImplOp>:MAX;
[56]     }) -> (struct {
[57]         status zx.status;
[58]     });
[59] };
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
* `[09]` `[11]` and `[13]` &mdash; these introduce two constants for use by the programmer.
* `[16` .. `23]` &mdash; these define a structure, called `I2cImplOp`, that the programmer
  will then use for transferring data to and from the bus.
* `[26` .. `59]` &mdash; these lines define the interface methods that are provided by
  this Banjo specification; we'll discuss this in greater detail [below](#the_interface).

> Don't be confused by the comments on `[50` .. `52]` (and elsewhere) &mdash; they're
> "flow through" comments that are intended to be emitted into the generated source.
> Any comment that starts with "`///`" (three! slashes) is a "flow through" comment.
> Ordinary comments (that is, "`//`") are intended for the current module.
> This will become clear when we look at the generated code.

## The operation structure

In our I2C sample, the `struct I2cImplOp` structure defines four elements:

Element   | Type              | Use
----------|-------------------|-----------------------------------------------------------------
`address` | `uint16`          | the address of the chip to interact with on the bus
`data`    | `vector<voidptr>` | contains the data sent to, and optionally received from, the bus
`is_read` | `bool`            | flag indicating read functionality desired
`stop`    | `bool`            | flag indicating a stop byte should be sent after the operation

The structure defines the communications area that will be used between the protocol
implementation (the driver) and the protocol user (the program that's using the bus).

## The interface

The more interesting part is the `protocol` specification.

We'll skip the `@transport("Banjo")` (line `[26]`) and `@banjo_layout("ddk-protocol")` (line `[27]`)
attributes for now, but will return to them below, in [Attributes](#attributes).

The `protocol` section defines five interface methods:

* `GetBusBase`
* `GetBusCount`
* `GetMaxTransferSize`
* `SetBitrate`
* `Transact`

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
`$BUILD_DIR/fidling/gen/sdk/banjo/fuchsia.hardware.i2cimpl/fuchsia.hardware.i2cimpl_banjo_c/fuchsia/hardware/i2cimpl/c/banjo.h`

The file is relatively long, so we'll look at it in several parts.

### Boilerplate

The first part has some boilerplate, which we'll show without further comment:

```c
[01] // Copyright 2018 The Fuchsia Authors. All rights reserved.
[02] // Use of this source code is governed by a BSD-style license that can be
[03] // found in the LICENSE file.
[04]
[05] // WARNING: THIS FILE IS MACHINE GENERATED. DO NOT EDIT.
[06] // Generated from the fuchsia.hardware.i2cimpl banjo file
[07]
[08] #pragma once
[09]
[10]
[11] #include <zircon/compiler.h>
[12] #include <zircon/types.h>
[13]
[14] __BEGIN_CDECLS
```

### Forward declarations

Next are forward declarations for our structures and functions:

```c
[16] // Forward declarations
[17] typedef struct i2c_impl_op i2c_impl_op_t;
[18] typedef struct i2c_impl_protocol i2c_impl_protocol_t;
[19] typedef struct i2c_impl_protocol_ops i2c_impl_protocol_ops_t;
...
[26] // Declarations
[27] // See `Transact` below for usage.
[28] struct i2c_impl_op {
[29]     uint16_t address;
[30]     uint8_t* data_buffer;
[31]     size_t data_size;
[32]     bool is_read;
[33]     bool stop;
[34] };
```

Note that lines `[17` .. `19]` only declare types, they don't actually define
structures or prototypes for functions.

Notice how the "flow through" comments (original `.fidl` file line `[15]`, for example)
got emitted into the generated code (line `[27]` above), with one slash stripped off to
make them look like normal comments.

Lines `[28` .. `34`] are, as advertised, an almost direct mapping of the `struct I2cImplOp`
from the `.fidl` file above (lines `[16` .. `23`]).

Astute C programmers will immediately see how the C++ style `vector<voidptr> data` (original
`.fidl` file line `[20]`) is handled in C: it gets converted to a pointer
("`data_buffer`") and a size ("`data_size`").

> As far as the naming goes, the base name is `data` (as given in the `.fidl` file).
> For a vector of `voidptr`, the transpiler appends `_buffer` and `_size` to convert the
> `vector` into a C compatible structure.
> For all other vector types, the transpiler appends `_list` and `_count` instead (for
> code readability).

### Constants

Next, we see our `const uint32` constants converted into `#define` statements:

```c
[20] // The maximum length of all read or all write transfers in bytes.
[21] #define I2C_IMPL_MAX_TOTAL_TRANSFER UINT32_C(4096)
[22] // The maximum number of I2cImplOp's that may be passed to Transact.
[23] #define I2C_IMPL_MAX_RW_OPS UINT32_C(8)
[24] #define I2C_IMPL_10_BIT_ADDR_MASK UINT32_C(0xF000)
```

In the C version, We chose `#define` instead of "passing through" the `const uint32_t`
representation because:

* `#define` statements only exist at compile time, and get inlined at every usage site, whereas
  a `const uint32_t` would get embedded in the binary, and
* `#define` allows for more compile time optimizations (e.g., doing math with the constant value).

The downside is that we don't get type safety, which is why you see the helper macros (like
**UINT32_C()** above); they just cast the constant to the appropriate type.

Note: Adding the `@namespaced` attribute to constant declarations for
Banjo C bindings will cause the variable name to be prefaced by the FIDL
library name. In this example, adding the `@namespaced` attribute to `I2C_IMPL_MAX_RW_OPS`
would cause the variable name to be `fuchsia_hardware_i2c_impl_I2C_IMPL_MAX_RW_OPS`
instead. This may be required to avoid name conflicts with FIDL hlcpp constant
bindings in the same build target.

### Protocol structures

And now we get into the good parts.

```c
[36] struct i2c_impl_protocol_ops {
[37]     uint32_t (*get_bus_base)(void* ctx);
[38]     uint32_t (*get_bus_count)(void* ctx);
[39]     zx_status_t (*get_max_transfer_size)(void* ctx, uint32_t bus_id, uint64_t* out_size);
[40]     zx_status_t (*set_bitrate)(void* ctx, uint32_t bus_id, uint32_t bitrate);
[41]     zx_status_t (*transact)(void* ctx, uint32_t bus_id, const i2c_impl_op_t* op_list, size_t op_count);
[42] };
```

This creates a structure definition that contains the five `protocol` methods that were defined in
the original `.fidl` file at lines `[30]`, `[34]`, `[37]`, `[44]`, and `[43]`.

Notice the name mangling that has occurred &mdash; this is how you can map the
`protocol` method names to the C function pointer names so that you know what
they're called:

Banjo                | C                       | Rule
---------------------|-------------------------|---------------------------------------------------------------
`Transact`           | `transact`              | Convert leading uppercase to lowercase
`GetBusBase`         | `get_bus_base`          | As above, and convert camel-case to underscore-separated style
`GetBusCount`        | `get_bus_count`         | Same as above
`SetBitrate`         | `set_bitrate`           | Same as above
`GetMaxTransferSize` | `get_max_transfer_size` | Same as above

Next, the interface definitions are wrapped in a context-bearing structure:

```c
[45] struct i2c_impl_protocol {
[46]     i2c_impl_protocol_ops_t* ops;
[47]     void* ctx;
[48] };
```

Finally, we see the actual generated code for the five methods:

```c
[53] static inline uint32_t i2c_impl_get_bus_base(const i2c_impl_protocol_t* proto) {
[54]     return proto->ops->get_bus_base(proto->ctx);
[55] }
[56]
[57] // Number of buses that this I2cImpl supports.
[58] static inline uint32_t i2c_impl_get_bus_count(const i2c_impl_protocol_t* proto) {
[59]     return proto->ops->get_bus_count(proto->ctx);
[60] }
[61]
[62] static inline zx_status_t i2c_impl_get_max_transfer_size(const i2c_impl_protocol_t* proto, uint32_t bus_id, uint64_t* out_size) {
[63]     return proto->ops->get_max_transfer_size(proto->ctx, bus_id, out_size);
[64] }
[65]
[66] // Sets the bitrate for the i2c bus in KHz units.
[67] static inline zx_status_t i2c_impl_set_bitrate(const i2c_impl_protocol_t* proto, uint32_t bus_id, uint32_t bitrate) {
[68]     return proto->ops->set_bitrate(proto->ctx, bus_id, bitrate);
[69] }
[70]
[71] // |Transact| assumes that all ops buf are not null.
[72] // |Transact| assumes that all ops length are not zero.
[73] // |Transact| assumes that at least the last op has stop set to true.
[74] static inline zx_status_t i2c_impl_transact(const i2c_impl_protocol_t* proto, uint32_t bus_id, const i2c_impl_op_t* op_list, size_t op_count) {
[75]     return proto->ops->transact(proto->ctx, bus_id, op_list, op_count);
[76] }
```

### Prefixes and paths

Notice how the prefix `i2c_impl_` (from the interface name, `.fidl` file line `[28]`)
got added to the method names; thus, `Transact` became `i2c_impl_transact`, and so on.
This is part of the mapping between `.fidl` names and their C equivalents.

Also, the `library` name (line `[05]` in the `.fidl` file) is transformed into the
include path: so `library fuchsia.hardware.i2cimpl` implies a path of `<fuchsia/hardware/i2cimpl/c/banjo.h>`.

## C++ {#cpp}

The C++ code is slightly more complex than the C version.
Let's take a look.

The Banjo transpiler generates three files:
the first is the C file discussed above, and the other two are under
`$BUILD_DIR/fidling/gen/sdk/banjo/fuchsia.hardware.i2cimpl/fuchsia.hardware.i2cimpl_banjo_c/fuchsia/hardware/i2cimpl/cpp/`

* `i2cimpl.h` &mdash; the file your program should include, and
* `i2cimpl-internal.h` &mdash; an internal file, included by `i2cimpl.h`

The "internal" file contains declarations and assertions, which we can safely skip.

The C++ version of `i2cimpl.h` is fairly long, so we'll look at it in smaller pieces.
Here's an overview "map" of what we'll be looking at, showing the starting line
number of each piece:

Line | Section
-----|----------------------------
1    | [boilerplate](#boilerplate_2)
20   | [auto generated usage comments](#auto-generated_comments)
61   | [class I2cImplProtocol](#the_i2cimplprotocol_mixin_class)
112  | [class I2cImplProtocolClient](#the_i2cimplprotocolclient_wrapper_class)

### Boilerplate

The boilerplate is pretty much what you'd expect:

```c++
[001] // Copyright 2018 The Fuchsia Authors. All rights reserved.
[002] // Use of this source code is governed by a BSD-style license that can be
[003] // found in the LICENSE file.
[004]
[005] // WARNING: THIS FILE IS MACHINE GENERATED. DO NOT EDIT.
[006] // Generated from the fuchsia.hardware.i2cimpl banjo file
[007]
[008] #pragma once
[009]
[010] #include <ddktl/device-internal.h>
[011] #include <fuchsia/hardware/i2cimpl/c/banjo.h>
[012] #include <lib/ddk/device.h>
[013] #include <lib/ddk/driver.h>
[014] #include <zircon/assert.h>
[015] #include <zircon/compiler.h>
[016] #include <zircon/types.h>
[017]
[018] #include "banjo-internal.h"
```

It `#include`s a bunch of DDK and OS headers, including:

* the C version of the header (line `[011]`, which means that everything discussed
  [above in the C section](#a_simple_example) applies here as well), and
* the generated `i2cimpl-internal.h` file (line `[018]`).

Next is the "auto generated usage comments" section; we'll come back to that
[later](#auto-generated_comments) as it will make more sense once we've seen
the actual class declarations.

The two class declarations are wrapped in the DDK namespace:

```c++
[057] namespace ddk {
...
[214] } // namespace ddk
```

### The I2cImplProtocolClient wrapper class

The `I2cImplProtocolClient` class is a simple wrapper around the `i2c_impl_protocol_t`
structure (defined in the C include file, line `[45]`, which we discussed in
[Protocol structures](#protocol_structures), above).

```c++
[112] class I2cImplProtocolClient {
[113] public:
[114]     I2cImplProtocolClient()
[115]         : ops_(nullptr), ctx_(nullptr) {}
[116]     I2cImplProtocolClient(const i2c_impl_protocol_t* proto)
[117]         : ops_(proto->ops), ctx_(proto->ctx) {}
[118]
[119]     I2cImplProtocolClient(zx_device_t* parent) {
[120]         i2c_impl_protocol_t proto;
[121]         if (device_get_protocol(parent, ZX_PROTOCOL_I2C_IMPL, &proto) == ZX_OK) {
[122]             ops_ = proto.ops;
[123]             ctx_ = proto.ctx;
[124]         } else {
[125]             ops_ = nullptr;
[126]             ctx_ = nullptr;
[127]         }
[128]     }
[129]
[130]     I2cImplProtocolClient(zx_device_t* parent, const char* fragment_name) {
[131]         i2c_impl_protocol_t proto;
[132]         if (device_get_fragment_protocol(parent, fragment_name, ZX_PROTOCOL_I2C_IMPL, &proto) == ZX_OK) {
[133]             ops_ = proto.ops;
[134]             ctx_ = proto.ctx;
[135]         } else {
[136]             ops_ = nullptr;
[137]             ctx_ = nullptr;
[138]         }
[139]     }
[140]
[141]     // Create a I2cImplProtocolClient from the given parent device + "fragment".
[142]     //
[143]     // If ZX_OK is returned, the created object will be initialized in |result|.
[144]     static zx_status_t CreateFromDevice(zx_device_t* parent,
[145]                                         I2cImplProtocolClient* result) {
[146]         i2c_impl_protocol_t proto;
[147]         zx_status_t status = device_get_protocol(
[148]                 parent, ZX_PROTOCOL_I2C_IMPL, &proto);
[149]         if (status != ZX_OK) {
[150]             return status;
[151]         }
[152]         *result = I2cImplProtocolClient(&proto);
[153]         return ZX_OK;
[154]     }
[155]
[156]     // Create a I2cImplProtocolClient from the given parent device.
[157]     //
[158]     // If ZX_OK is returned, the created object will be initialized in |result|.
[159]     static zx_status_t CreateFromDevice(zx_device_t* parent, const char* fragment_name,
[160]                                         I2cImplProtocolClient* result) {
[161]         i2c_impl_protocol_t proto;
[162]         zx_status_t status = device_get_fragment_protocol(parent, fragment_name,
[163]                                  ZX_PROTOCOL_I2C_IMPL, &proto);
[164]         if (status != ZX_OK) {
[165]             return status;
[166]         }
[167]         *result = I2cImplProtocolClient(&proto);
[168]         return ZX_OK;
[169]     }
[170]
[171]     void GetProto(i2c_impl_protocol_t* proto) const {
[172]         proto->ctx = ctx_;
[173]         proto->ops = ops_;
[174]     }
[175]     bool is_valid() const {
[176]         return ops_ != nullptr;
[177]     }
[178]     void clear() {
[179]         ctx_ = nullptr;
[180]         ops_ = nullptr;
[181]     }
[182]
[183]     // First bus ID that this I2cImpl controls, zero-indexed.
[184]     uint32_t GetBusBase() const {
[185]         return ops_->get_bus_base(ctx_);
[186]     }
[187]
[188]     // Number of buses that this I2cImpl supports.
[189]     uint32_t GetBusCount() const {
[190]         return ops_->get_bus_count(ctx_);
[191]     }
[192]
[193]     zx_status_t GetMaxTransferSize(uint32_t bus_id, uint64_t* out_size) const {
[194]         return ops_->get_max_transfer_size(ctx_, bus_id, out_size);
[195]     }
[196]
[197]     // Sets the bitrate for the i2c bus in KHz units.
[198]     zx_status_t SetBitrate(uint32_t bus_id, uint32_t bitrate) const {
[199]         return ops_->set_bitrate(ctx_, bus_id, bitrate);
[200]     }
[201]
[202]     // |Transact| assumes that all ops buf are not null.
[203]     // |Transact| assumes that all ops length are not zero.
[204]     // |Transact| assumes that at least the last op has stop set to true.
[205]     zx_status_t Transact(uint32_t bus_id, const i2c_impl_op_t* op_list, size_t op_count) const {
[206]         return ops_->transact(ctx_, bus_id, op_list, op_count);
[207]     }
[208]
[209] private:
[210]     i2c_impl_protocol_ops_t* ops_;
[211]     void* ctx_;
[212] };
```

There are four constructors:

* the default one (`[114]`) that sets `ops_` and `ctx_` to `nullptr`,
* an initializer (`[116]`) that takes a pointer to an `i2c_impl_protocol_t` structure and populates
  the `ops_` and `ctx`_ fields from their namesakes in the structure, and
* an initializer (`[119]`) that extracts the `ops_` and `ctx_` information from a `zx_device_t`.
* an initializer (`[130]`) like above but gets `ops_` and `ctx_` from a device fragment.

The last two constructors are preferred, and can be used like this:

```c++
ddk::I2cImplProtocolClient i2cimpl(parent);
if (!i2cimpl.is_valid()) {
  return ZX_ERR_*; // return an appropriate error
}
```
```c++
ddk::I2cImplProtocolClient i2cimpl(parent, "i2c-impl-fragment");
if (!i2cimpl.is_valid()) {
  return ZX_ERR_*; // return an appropriate error
}
```

Three convenience member functions are provided:

* `[171]` **GetProto()** fetches the `ctx_` and `ops_` members into a protocol structure,
* `[175]` **is_valid()** returns a `bool` indicating if the class has been initialized with
   a protocol, and
* `[178]` **clear()** invalidates the `ctx_` and `ops_` pointers.

Next we find the four member functions that were specified in the `.fidl` file:

* `[138]` **GetBusBase()**, and
* `[138]` **GetBusCount()**, and
* `[138]` **GetMaxTransferSize()**, and
* `[138]` **SetBitrate()**, and
* `[134]` **Transact()**.

These work just liked the four wrapper functions from the C version of the include file &mdash;
that is, they pass their arguments into a call through the respective function pointer.

In fact, compare **i2c_impl_get_max_transfer_size()** from the C version:

```c
[138] zx_status_t GetMaxTransferSize(size_t* out_size) const {
[139]     return ops_->get_max_transfer_size(ctx_, out_size);
[140] }
```

with the C++ version above:

```c++
[138] zx_status_t GetMaxTransferSize(size_t* out_size) const {
[139]     return ops_->get_max_transfer_size(ctx_, out_size);
[140] }
```

As advertised, all that this class does is store the operations and context pointers for
later use, so that the call through the wrapper is more elegant.

> You'll also notice that the C++ wrapper function doesn't have any name mangling &mdash;
> to use a tautology, **GetMaxTransferSize()** is **GetMaxTransferSize()**.

### The I2cImplProtocol mixin class

Ok, that was the easy part.
For this next part, we're going to talk about [mixins](https://en.wikipedia.org/wiki/Mixin)
and [CRTPs &mdash; or Curiously Recurring Template
Patterns](https://en.wikipedia.org/wiki/Curiously_recurring_template_pattern).

Let's understand the "shape" of the class first (comment lines deleted for outlining
purposes):

```c++
[060] template <typename D, typename Base = internal::base_mixin>
[061] class I2cImplProtocol : public Base {
[062] public:
[063]     I2cImplProtocol() {
[064]         internal::CheckI2cImplProtocolSubclass<D>();
[065]         i2c_impl_protocol_ops_.get_bus_base = I2cImplGetBusBase;
[066]         i2c_impl_protocol_ops_.get_bus_count = I2cImplGetBusCount;
[067]         i2c_impl_protocol_ops_.get_max_transfer_size = I2cImplGetMaxTransferSize;
[068]         i2c_impl_protocol_ops_.set_bitrate = I2cImplSetBitrate;
[069]         i2c_impl_protocol_ops_.transact = I2cImplTransact;
[070]
[071]         if constexpr (internal::is_base_proto<Base>::value) {
[072]             auto dev = static_cast<D*>(this);
[073]             // Can only inherit from one base_protocol implementation.
[074]             ZX_ASSERT(dev->ddk_proto_id_ == 0);
[075]             dev->ddk_proto_id_ = ZX_PROTOCOL_I2C_IMPL;
[076]             dev->ddk_proto_ops_ = &i2c_impl_protocol_ops_;
[077]         }
[078]     }
[079]
[080] protected:
[081]     i2c_impl_protocol_ops_t i2c_impl_protocol_ops_ = {};
[082]
[083] private:
...
[085]     static uint32_t I2cImplGetBusBase(void* ctx) {
[086]         auto ret = static_cast<D*>(ctx)->I2cImplGetBusBase();
[087]         return ret;
[088]     }
...
[090]     static uint32_t I2cImplGetBusCount(void* ctx) {
[091]         auto ret = static_cast<D*>(ctx)->I2cImplGetBusCount();
[092]         return ret;
[093]     }
[094]     static zx_status_t I2cImplGetMaxTransferSize(void* ctx, uint32_t bus_id, uint64_t* out_size) {
[095]         auto ret = static_cast<D*>(ctx)->I2cImplGetMaxTransferSize(bus_id, out_size);
[096]         return ret;
[097]     }
...
[099]     static zx_status_t I2cImplSetBitrate(void* ctx, uint32_t bus_id, uint32_t bitrate) {
[100]         auto ret = static_cast<D*>(ctx)->I2cImplSetBitrate(bus_id, bitrate);
[101]         return ret;
[102]     }
...
[106]     static zx_status_t I2cImplTransact(void* ctx, uint32_t bus_id, const i2c_impl_op_t* op_list, size_t op_count) {
[107]         auto ret = static_cast<D*>(ctx)->I2cImplTransact(bus_id, op_list, op_count);
[108]         return ret;
[109]     }
[110] };
```

The `I2CImplProtocol` class inherits from a base class, specified by the second template parameter.
If it's left unspecified, it defaults to `internal::base_mixin`, and no special magic happens.
If, however, the base class is explicitly specified, it should be `ddk::base_protocol`,
in which case additional asserts are added (to double check that only one mixin is the base protocol).
In addition, special DDKTL fields are set to automatically register this protocol as the
base protocol when the driver triggers **DdkAdd()**.

The constructor calls an internal validation function, **CheckI2cImplProtocolSubclass()** `[32]`
(defined in the generated `i2c-impl-internal.h` file), which has several **static_assert()** calls.
The class `D` is expected to implement the five member functions (**I2cImplGetBusBase()**,
**I2cIImplGetBusCount()**, **I2cImplGetMaxTransferSize()**, **I2cImplSetBitrate()**, and
**I2cImplTransact()**) in order for the static methods to work. If they're not provided by `D`, then
the compiler would (in the absence of the static asserts) produce gnarly templating errors. The
static asserts serve to produce diagnostic errors that are understandable by mere humans.

Next, the five pointer-to-function operations members (`get_bus_base`, `get_bus_count`,
`get_max_transfer_size`, `set_bitrate`, and `transact`) are bound (lines `[065` .. `069]`).

Finally, the `constexpr` expression provides a default initialization if required.

### Using the mixin class

The `I2cImplProtocol` class can be used as follows (from
[`//src/devices/i2c/drivers/intel-i2c/intel-i2c-controller.h`](/src/devices/i2c/drivers/intel-i2c/intel-i2c-controller.h)):

```c++
[135] class IntelI2cController : public IntelI2cControllerType,
[136]                            public ddk::I2cImplProtocol<IntelI2cController, ddk::base_protocol> {
[137]  public:
[138]   explicit IntelI2cController(zx_device_t* parent)
[139]       : IntelI2cControllerType(parent), pci_(parent, "pci") {}
[140]
[141]   static zx_status_t Create(void* ctx, zx_device_t* parent);
[142]
[143]   void DdkInit(ddk::InitTxn txn);
...
[170]   uint32_t I2cImplGetBusBase();
[171]   uint32_t I2cImplGetBusCount();
[172]   zx_status_t I2cImplGetMaxTransferSize(const uint32_t bus_id, size_t* out_size);
[173]   zx_status_t I2cImplSetBitrate(const uint32_t bus_id, const uint32_t bitrate);
[174]   zx_status_t I2cImplTransact(const uint32_t bus_id, const i2c_impl_op_t* op_list,
[175]                               const size_t op_count);
[176]
[177]   void DdkUnbind(ddk::UnbindTxn txn);
[178]   void DdkRelease();
[179]
[180]  private:
...
```

Here we see that `class IntelI2cController` inherits from the DDK's `I2cImplProtocol` and provides
itself as the argument to the template &mdash; this is the "mixin" concept.
This causes the `IntelI2cController` type to be substituted for `D` in the template definition
of the class (from the `i2c-impl.h` header file above, lines `[086]`, `[091]`, `[95]`, `[100]`, and
`[107]`).

Taking a look at just the **I2cImplGetMaxTransferSize()** function as an example, it's
effectively as if the source code read:

```c++
[094] static zx_status_t I2cImplGetMaxTransferSize(void* ctx, uint32_t bus_id, uint64_t* out_size) {
[095]     auto ret = static_cast<IntelI2cController*>(ctx)->I2cImplGetMaxTransferSize(bus_id, out_size);
[096]     return ret;
[097] }
```

This ends up eliminating the cast-to-self boilerplate in your code.
This casting is necessary because the type information is erased at the DDK boundary &mdash;
recall that the context `ctx` is a `void *` pointer.

### Auto-generated comments

Banjo automatically generates comments in the include file that basically summarize what we
talked about above:

```c++
[020] // DDK i2cimpl-protocol support
[021] //
[022] // :: Proxies ::
[023] //
[024] // ddk::I2cImplProtocolClient is a simple wrapper around
[025] // i2c_impl_protocol_t. It does not own the pointers passed to it.
[026] //
[027] // :: Mixins ::
[028] //
[029] // ddk::I2cImplProtocol is a mixin class that simplifies writing DDK drivers
[030] // that implement the i2c-impl protocol. It doesn't set the base protocol.
[031] //
[032] // :: Examples ::
[033] //
[034] // // A driver that implements a ZX_PROTOCOL_I2C_IMPL device.
[035] // class I2cImplDevice;
[036] // using I2cImplDeviceType = ddk::Device<I2cImplDevice, /* ddk mixins */>;
[037] //
[038] // class I2cImplDevice : public I2cImplDeviceType,
[039] //                      public ddk::I2cImplProtocol<I2cImplDevice> {
[040] //   public:
[041] //     I2cImplDevice(zx_device_t* parent)
[042] //         : I2cImplDeviceType(parent) {}
[043] //
[044] //     uint32_t I2cImplGetBusBase();
[045] //
[046] //     uint32_t I2cImplGetBusCount();
[047] //
[048] //     zx_status_t I2cImplGetMaxTransferSize(uint32_t bus_id, uint64_t* out_size);
[049] //
[050] //     zx_status_t I2cImplSetBitrate(uint32_t bus_id, uint32_t bitrate);
[051] //
[052] //     zx_status_t I2cImplTransact(uint32_t bus_id, const i2c_impl_op_t* op_list, size_t op_count);
[053] //
[054] //     ...
[055] // };
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
a `@transport("Banjo")` and a `@banjo_layout("ddk-protocol")` attribute.

### The transport attribute

All Banjo protocols must have `@transport("Banjo")` to indicate that Banjo is
being used instead of FIDL.

### The banjo\_layout attribute

The line just before the `protocol` is the `banjo_layout` attribute:

```banjo
[27] @banjo_layout("ddk-protocol")
[28] protocol I2cImpl {
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

### The async attribute

For an example of the `@async` attribute, see the
[`fuchsia.hardware.block`](/sdk/fidl/fuchsia.hardware.block.driver/block.fidl) `Block` protocol.

Within the `protocol` section, we see the `@async` attribute:

```banjo
[254] protocol Block {
...       /// comments (removed)
[268]     @async
```

The `@async` attribute is a way to make protocol messages not be synchronous.
It autogenerates a callback type in which the output arguments are inputs to the callback.
The original method will not have any of the output parameters specified in its signatures.

In the protocol above there is a `Queue` method declared as:

```banjo
[268] @async
[269] Queue(resource struct {
[270]     @in_out
[271]     txn BlockOp;
[272] }) -> (resource struct {
[273]     status zx.status;
[274]     @mutable
[275]     op BlockOp;
[276] });
```

When used (as above) in conjunction with the `@async` attribute, it means that we want Banjo
to invoke a callback function, so that we can handle the output data (the second
`BlockOp` above, representing the data from the block device).

Here's how it works.
We send data to the block device through the first `BlockOp` argument.
Some time later, the block device may generate data in response to our request.
Because we specified `@async`, Banjo generates the functions to take a callback function
as input.

In C, these two lines (from the `block.h` file) are important:

```c
[085] typedef void (*block_queue_callback)(void* ctx, zx_status_t status, block_op_t* op);
...
[211] void (*queue)(void* ctx, block_op_t* txn, block_queue_callback callback, void* cookie);
```

In C++, we have two place where the callback is referenced:

```c++
[113] static void BlockQueue(void* ctx, block_op_t* txn, block_queue_callback callback, void* cookie) {
[114]     static_cast<D*>(ctx)->BlockQueue(txn, callback, cookie);
[115] }
```

and

```c++
[201] void Queue(block_op_t* txn, block_queue_callback callback, void* cookie) const {
[202]     ops_->queue(ctx_, txn, callback, cookie);
[203] }
```

Notice how the C++ is similar to the C: that's because the generated code includes the
C header file as part of the C++ header file.

The transaction callback has the following arguments:

Argument   | Meaning
-----------|----------------------------------------
`ctx`      | the cookie
`status`   | status of the asynchronous response (provided by callee)
`op`       | the data from the transfer

How is this different than just using the `@banjo_layout("ddk-callback")` attribute we
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

> One more caveat with `@async` is that its callback *MUST* be called for each
> protocol method invocation, and the accompanying cookie must be provided.
> Failure to do so will result in undefined behavior (likely a leak, deadlock,
> timeout, or crash).

Although not the case currently, C++ and future language bindings (like Rust)
will provide "future" / "promise" style based APIs in the generated code, built on top of
these callbacks in order to prevent mistakes.

> Ok, one more caveat with `@async` &mdash; the `@async` attribute applies *only*
> to the immediately following method; not any other methods.

### The buffer attribute

This attribute applies to protocol method parameters of the `vector` type to convey that they are
used as buffers. In practice, it only affects the names of the generated parameters.

### The callee\_allocated attribute

When applied to a protocol method output parameter of type `vector`, the attribute conveys the fact
that the contents of the vector should be allocated by the receiver of the method call.

### The derive\_debug attribute (C bindings only)

When applied to an enum declaration, a helper `*_to_str()` function
will be generated for C bindings which returns a `const char*` for each
value of the enum. For example, an enum declared with this attribute such
as

```banjo
@derive_debug
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

### The inner\_pointer attribute

In the context of a protocol input parameter of type `vector`, this attribute turns the contents of
the vector into pointers to objects instead of objects themselves.

### The in\_out attribute

Adding this attribute to a protocol method input parameter makes the parameter mutable, effectively
turning it into an "in-out" parameter.

### The mutable attribute

This attribute should be used to make `struct`/`union` fields of type `vector` or `string` mutable.

### The namespaced attribute

This attribute applies to `const` declarations and makes it so that the C backend prefaces the
constant name with the snake-cased FIDL library name, e.g. `library_name_CONSTANT_K` instead
of `CONSTANT_K`. This attribute may be required to avoid name conflicts with FIDL hlcpp constant
bindings in the same build target.

### The out\_of\_line\_contents attribute

This attribute allows the contents of a `vector` field in a `struct`/`union` to be stored outside
of the container.

### The preserve\_c\_names attribute

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

Test code must include the protocol header with a `-mock` suffix, e.g.
`#include <fuchsia/hardware/gpio/cpp/banjo-mock.h>`.

Consider the following Banjo protocol snippet:

```banjo
[20] @transport("Banjo")
[21] @banjo_layout("ddk-protocol")
[22] protocol Gpio {
 ...
[53]     /// Gets an interrupt object pertaining to a particular GPIO pin.
[54]     GetInterrupt(struct {
[55]         flags uint32;
[56]     }) -> (resource struct {
[57]         s zx.status;
[58]         irq zx.handle:INTERRUPT;
[59]     });
 ...
[82] };
```

Here are the corresponding bits of the mock class generated by Banjo:

```c++
[034] class MockGpio : ddk::GpioProtocol<MockGpio> {
[035] public:
[036]     MockGpio() : proto_{&gpio_protocol_ops_, this} {}
[037]
[038]    virtual ~MockGpio() {}
[039]
[040]     const gpio_protocol_t* GetProto() const { return &proto_; }
 ...
[067]     virtual MockGpio& ExpectGetInterrupt(zx_status_t out_s, uint32_t flags, zx::interrupt out_irq) {
[068]         mock_get_interrupt_.ExpectCall({out_s, std::move(out_irq)}, flags);
[069]         return *this;
[070]     }
 ...
[092]     void VerifyAndClear() {
 ...
[098]         mock_get_interrupt_.VerifyAndClear();
 ...
[103]     }
 ...
[131]     virtual zx_status_t GpioGetInterrupt(uint32_t flags, zx::interrupt* out_irq) {
[132]         std::tuple<zx_status_t, zx::interrupt> ret = mock_get_interrupt_.Call(flags);
[133]         *out_irq = std::move(std::get<1>(ret));
[134]         return std::get<0>(ret);
[135]     }
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
