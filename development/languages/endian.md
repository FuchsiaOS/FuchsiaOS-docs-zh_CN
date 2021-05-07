# Fuchsia endian policy

Although Fuchsia only runs on little endian (LE) CPU architectures,
the Fuchsia project still needs to consider big endian (BE) issues.

This page explains:

 * Where endian issues arise
 * How to handle them
 * Why these choices are made

## Where endian issues arise

### Peripheral hardware

A lot of peripheral hardware defines multi-byte BE values, which must be
converted.

### Legacy formats

Network byte order is BE.
SCSI data structures are BE.

### Big endian CPU execution

Even if Fuchsia never runs on a BE CPU (which it might someday, at least in
theory), some of our code may be ported to a BE CPU.

Any time we define a multi-byte value, we create the possibility that another platform
may want to write or read that value, and our code (which is open source) may be ported to
that platform in order to do this.

## How to handle endian issues in C/C++ code and docs

### If a module is unlikely to run into any endian issues

Many modules do not need to do anything about endian issues; their data will only be interpreted by a single CPU running Fuchsia.

For those that might be ported to other OS's, or whose data might be exported by any channel:

Suggested style in C or C++ is to add

```
#include <endian.h>
...
static_assert(__BYTE_ORDER__ == __ORDER_LITTLE_ENDIAN__);
```

either in every file, or accompanied by a comment explaining which files are
not BE compatible.

(It's OK to not do anything, but better to make it explicit that the code is
not BE compatible.)

### If a module must deal with endian issues

In structures that are inherently endian, it's best to include macros that
"convert" little-endian data to CPU endianness; this is a form of
self-documenting code. Of course big-endian data should always use the macros.

#### For C/C++

Best style is to use the LE16 .. BE64 macros from endian.h, which should be
available everywhere including DDK.

```
#include <endian.h>
...
hw_le_struct.int_field = LE32(program_int);
program_long =  BE64(hw_be_struct.long_field);
```

#### For Rust

To access multi-byte values in a byte buffer, use [this crate](https://docs.rs/byteorder/1.2.7/byteorder/). To convert integer values, use [these methods](https://doc.rust-lang.org/std/primitive.i32.html).
