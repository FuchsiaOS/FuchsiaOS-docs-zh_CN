# Inspecting memory in zxdb

Zxdb supports the following commands for inspecting memory:

  * [`aspace`](#aspace_show_mapped_memory_regions)
  * [`mem-analyze`](#mem-analyze_dumps_memory_trying_to_interpret_pointers)
  * [`mem-read` / `x`](#mem-read_dumps_process_memory)
  * [`stack`](#stack_provides_a_low-level_analysis_of_the_stack)
  * [`sym-near`](#sym-near_map_addresses_to_symbols)

## `aspace`: Show mapped memory regions.

The `aspace` command, abbreviated `as`, outputs address space information for the process. In
Fuchsia, virtual memory consists of a hierarchy of [Virtual Memory
Objects](/docs/reference/kernel_objects/vm_object.md) (VMOs).

With no parameters, the `aspace` command shows all VMOs in the process.

```none {:.devsite-disable-click-to-copy}
[zxdb] as
```

When given an address, the `aspace` command shows the VMO hierarchy containing just that address.
This can be useful to determine where an address is in memory, as the names of the VMOs typically
indicate what type of region that is.

```none {:.devsite-disable-click-to-copy}
[zxdb] as 0x10b7f304d28
```

### Understanding the output

In the following example, the `aspace` command details the following about the `0x10b7f304d28`
address:

  * Hierarchy of VMOs that contain the address.
  * The address and size of each VMO.
  * The name of each VMO, which can give clues about their purpose.
      * From the name in this example, you can tell the address is in a stack allocated by pthreads.

```none {:.devsite-disable-click-to-copy}
          Start              End   Size  Name
      0x1000000   0x7ffffffff000   127T  proc:3109
      0x1000000   0x7ffffffff000   127T    root
  0x10b7f104000    0x10b7f305000     2M      useralloc
  0x10b7f105000    0x10b7f305000     2M        pthread_t:0x10c4ea38b00
```

The following are relevant VMO names that could be included in output from the `aspace` command:

  * `initial-thread`: The stack of the startup thread.
  * `pthread_t:0x...`: The stack of a pthread-created thread. The address indicates the memory
    location of the `pthread_t structure for that thread.
  * `*uncompressed-bootfs`: A memory-mapped library coming from bootfs (core system libraries). The
    `libs` command can tell you the library name for that address.
  * `stack: msg of ...`: The startup stack. This very small stack is used only by the dynamic linker
    and loader code.
  * `scudo:*`: Pages allocated with the scudo memory manager. If the process is using scudo, these
    regions are the application heap.
  * `vdso/next`: The built-in library that implements the next system calls.
  * `vdso/stable`: The built-in library that implements the stable system calls.
  * `blob-*`: Mapped library coming from blobfs. The `libs` command can tell you the library name
    for that address.

## `mem-analyze`: Dumps memory, trying to interpret pointers.

This command attempts to interpret memory as pointers and decode what they point to. Addresses with
corresponding symbols are symbolized, while other addresses indicate the name of the
memory-mapping region they fall into (see the [`aspace`](#aspace_show_mapped_memory_regions)
command). It can be useful for dumping unknown memory.

```none {:.devsite-disable-click-to-copy}
[zxdb] ma 0x42ff9c2fdd30
       Address               Data
0x42ff9c2fdd30 0x00000000000015f0
0x42ff9c2fdd38 0x0000000000000008
0x42ff9c2fdd40 0x000042f401a8a730 ▷ ldso
0x42ff9c2fdd48 0x000042f401a8a9f8 ▷ $(dls3.app)
0x42ff9c2fdd50 0x0000000000000053
0x42ff9c2fdd58 0x0000000010469c6b
0x42ff9c2fdd60 0x000042f401a8a9f8 ▷ $(dls3.app)
0x42ff9c2fdd68 0x0000000000000000
0x42ff9c2fdd70 0x000042ff9c2fde70 ▷ inside map "stack: msg of 0x1000"
0x42ff9c2fdd78 0x000042f4015e5548 ▷ dls3 + 0x42b
0x42ff9c2fdd80 0x10469c6b10769c7b
0x42ff9c2fdd88 0x10569c3310469c23
0x42ff9c2fdd90 0x10469c2710469c37
```

See also [`stack`](#stack_provides_a_low-level_analysis_of_the_stack), which is a variant of the
`mem-analyze` command for stack analysis.

## `mem-read`: Dump process memory

The `mem-read` command, abbreviated `x`, provides hex dumps of the given address. You supply
the address and optionally override the default size with the `-s` option.

```none {:.devsite-disable-click-to-copy}
[zxdb] x -s 100 0x42ff9c2fdd30
0x42ff9c2fdd30:  f0 15 00 00 00 00 00 00-08 00 00 00 00 00 00 00  |
0x42ff9c2fdd40:  30 a7 a8 01 f4 42 00 00-f8 a9 a8 01 f4 42 00 00  |0    B       B
0x42ff9c2fdd50:  53 00 00 00 00 00 00 00-6b 9c 46 10 00 00 00 00  |S       k F
0x42ff9c2fdd60:  f8 a9 a8 01 f4 42 00 00-00 00 00 00 00 00 00 00  |     B
0x42ff9c2fdd70:  70 de 2f 9c ff 42 00 00-48 55 5e 01 f4 42 00 00  |p /  B  HU^  B
0x42ff9c2fdd80:  7b 9c 76 10 6b 9c 46 10-23 9c 46 10 33 9c 56 10  |{ v k F # F 3 V 
0x42ff9c2fdd90:  37 9c 46 10
```

You can also supply an expression that evaluates to an address. If the type of the pointer has
a known size, the dump automatically shows that many bytes:

```none {:.devsite-disable-click-to-copy}
[zxdb] x &self->main_waker
0x1605a5d1ed0:  70 1a c8 36 47 04 00 00-68 fe 3d dd 25 01 00 00  |p  6G   h = %
```

## `stack`: Provides a low-level analysis of the stack

The `stack` command analyzes the stack in a similar way to `mem-analyze`. It defaults to the
top of the current thread's stack. The `stack` command attempts to decode addresses present in the
memory region, but it also adds annotations for the known register values and stack base pointers of
the thread.

```none {:.devsite-disable-click-to-copy}
[zxdb] stack
      Address               Data 
0x1605a5d1428 0x000042a352fca11f ◁ rsp. ▷ _zx_port_wait + 0x1f
0x1605a5d1430 0x000001605a5d1460 ◁ frame 1 rsp. ▷ inside map "initial-thread"
0x1605a5d1438 0x000001605a5d1540 ▷ inside map "initial-thread"
0x1605a5d1440 0x7fffffffffffffff
0x1605a5d1448 0x0000044ab6c81800 ▷ inside map "scudo:primary"
0x1605a5d1450 0x000001605a5d14d0 ◁ rbp, frame 1 base. ▷ inside map "initial-thread"
0x1605a5d1458 0x00000125dd3566f5 ▷ fuchsia_zircon_status::Status::ok
0x1605a5d1460 0x0000000000000000 ◁ frame 2 rsp
0x1605a5d1468 0x0000000000000000
0x1605a5d1470 0x0000000000000000
0x1605a5d1478 0x0000000000000000
0x1605a5d1480 0x0000000000000000 ◁ rdx, r14
```

In the notes colum, left-pointing arrows indicate which registers point to that stack location,
while right-pointing arrows indicate where the value of the stack entry points to if it is
interpreted as an address.

## `sym-near`: Map addresses to symbols

The `sym-near` command, abbreviated `sn`, attempts to map an address to a symbol name. Running the
command outputs the name and line information (if available) for the symbol at or preceeding the
address and is most often used to tell what a pointer points to.

```none {:.devsite-disable-click-to-copy}
[zxdb] sym-near 0x125dd3a845e
0x125dd3a845e, power_manager::main() • main.rs:37
```
