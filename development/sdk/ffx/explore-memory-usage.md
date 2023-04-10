# Measure the memory usage on a device

The `ffx profile memory` command can measure the RAM (Random Access Memory) usage of
a Fuchsia system.

## Concepts

The `ffx profile memory` command evaluates how much memory is used by VMOs
([Virtual Memory Objects][vmo]) in a Fuchsia system. Unlike Linux's `ps` command,
this command evaluates all VMOs whether they are mapped or not.

Below is an example of a process's memory usage captured by `ffx profile memory`:

```none {:.devsite-disable-click-to-copy}
Process name: core.cm
Process koid: 26387
Private:      268 KiB
PSS:          650.72 KiB (Proportional Set Size)
Total:        4.73 MiB (Private + Shared unscaled)
                              Private       Scaled        Total
    [scudo]                    60 KiB       60 KiB       60 KiB
    [data]                     56 KiB       56 KiB       56 KiB
    [relro]                    56 KiB       56 KiB       56 KiB
    bss:blob-e243d9eb          56 KiB       56 KiB       56 KiB
    [stacks]                   28 KiB       28 KiB       28 KiB
    AllocatorRingBuffer         4 KiB        4 KiB        4 KiB
    [libraries]                 4 KiB        4 KiB        4 KiB
    stack: msg of 0x1000        4 KiB        4 KiB        4 KiB
    [blobs]                       0 B   382.22 KiB     4.41 MiB     (shared)
    vdso/next                     0 B        338 B       40 KiB     (shared)
    vdso/stable                   0 B        170 B       20 KiB     (shared)
```

In Fuchsia, memory can be shared among processes because multiple processes can have
handles to the same VMO. Thus in some cases, it is useful to distinguish between private
memory and shared memory. To help distinguish, `ffx profile memory` reports the memory
usage of a process in 3 distinct, but overlapping, categories:

* `Private` is the total size of VMOs and their [child VMOs][child-vmos]
  that are retained exclusively by this process.
* `Scaled` is the total size of VMOs and their child VMOs that are retained by several
  processes. The cost of these VMOs is shared evenly among the retaining processes. For
  example, 500 KiB shared by 5 processes will add 100 KiB to each of the 5 processes.
* `Total` is the total size of all VMOs and their child VMOs that are retained by this
  process, which include VMOs that are shared with other processes.

Some VMOs have names attached to them. Based on the name, it's often possible to have
an idea of what the VMO is used for. For example, if the name of a VMO starts with
`scudo`, it is likely used by the [Scudo allocator][scudo]{:.external}. The names allow
`ffx profile memory` to categorize the VMOs of a given process into probable sources.
The list of categories include:

* `[scudo]`: VMOs used by Scudo, Fuchsia's default memory allocator.
* `[stacks]`: VMOs used to store the stacks of the process's threads.
* `[blobs]`: VMOs handed out by blobFS. These may include child VMOs that have been
   modified.
* `[relro]`: VMOs containing the relocated read-only section of binaries.
* `[data]`: VMOs containing the data segment of binaries.
* `[unnamed]`: VMOs without names.

VMOs with names that do not belong to any of the built-in categories are displayed as
their own categories.

Under the hood, the `ffx profile memory` command uses the `memory_monitor` component to
capture the memory information of all VMOs in the system.

## Measure the memory usage over a time interval {:#measure-the-memory-usage-over-a-time-interval}

To track the memory usage over a specific time interval, run the following command:

```posix-terminal
ffx profile memory --interval {{ '<var>' }}SECONDS{{ '</var>' }}
```

Replace <var>SECONDS</var> with a time interval in seconds.

The example command below checks the memory usage of the target Fuchsia device
every 3 seconds until the command is terminated (usually by pressing `CTRL+C`
in the terminal):

```none {:.devsite-disable-click-to-copy}
$ ffx profile memory --csv --interval 3
```

Notice that the example command prints the output in the CSV format (`--csv`).
For debugging purposes, to obtain the raw data exported by the `memory_monitor`
component, you can run the command with the `--debug-json` option.

<!-- Reference links -->

[vmo]: /reference/kernel_objects/vm_object.md
[child-vmos]: /reference/syscalls/vmo_create_child.md
[scudo]: https://llvm.org/docs/ScudoHardenedAllocator.html
