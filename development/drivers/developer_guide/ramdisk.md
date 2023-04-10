# RAMdisk Device

Caution: This page may contain information that is specific to the legacy
version of the driver framework (DFv1). Also the workflows documented on
this page may only be specific to the Fuchsia source checkout
(`fuchsia.git`) environment.

## Overview

In this section, we'll examine a simplified RAM-disk driver.

This driver introduces:

*   the block protocol's **query()** and **queue()** ops
*   Virtual Memory Address Regions ([VMAR](/reference/kernel_objects/vm_address_region.md)s)
    and Virtual Memory Objects ([VMO](/reference/kernel_objects/vm_object.md)s)

The source is in `//examples/drivers//ramdisk/demo-ramdisk.c`.

As with all drivers, the first thing to look at is how the driver initializes itself:

```c
static zx_status_t ramdisk_driver_bind(void* ctx, zx_device_t* parent) {
    zx_status_t status = ZX_OK;

    // (1) create the device context block
    ramdisk_device_t* ramdev = calloc(1, sizeof((*ramdev)));
    if (ramdev == NULL) {
        return ZX_ERR_NO_MEMORY;
    }

    // (2) create a VMO
    status = zx_vmo_create(RAMDISK_SIZE, 0, &ramdev->vmo);
    if (status != ZX_OK) {
        goto cleanup;
    }

    // (3) map the VMO into our address space
    status = zx_vmar_map(zx_vmar_root_self(), 0, ramdev->vmo, 0, RAMDISK_SIZE,
                         ZX_VM_FLAG_PERM_READ | ZX_VM_FLAG_PERM_WRITE, &ramdev->mapped_addr);
    if (status != ZX_OK) {
        goto cleanup;
    }

    // (4) add the device
    device_add_args_t args = {
        .version = DEVICE_ADD_ARGS_VERSION,
        .name = "demo-ramdisk",
        .ctx = ramdev,
        .ops = &ramdisk_proto,
        .proto_id = ZX_PROTOCOL_BLOCK_IMPL,
        .proto_ops = &block_ops,
    };

    if ((status = device_add(parent, &args, &ramdev->zxdev)) != ZX_OK) {
        ramdisk_release(ramdev);
    }
    return status;

    // (5) clean up after ourselves
cleanup:
    zx_handle_close(ramdev->vmo);
    free(ramdev);
    return status;
}

static zx_driver_ops_t ramdisk_driver_ops = {
    .version = DRIVER_OPS_VERSION,
    .bind = ramdisk_driver_bind,
};

ZIRCON_DRIVER_BEGIN(ramdisk, ramdisk_driver_ops, "zircon", "0.1", 1)
    BI_MATCH_IF(EQ, BIND_PROTOCOL, ZX_PROTOCOL_MISC_PARENT),
ZIRCON_DRIVER_END(ramdisk)

```

At the bottom, you can see that this driver binds to a `ZX_PROTOCOL_MISC_PARENT` type of
protocol, and provides `ramdisk_driver_ops` as the list of operations supported.
This is no different than any of the other drivers we've seen so far.

The binding function, **ramdisk_driver_bind()**, does the following:

1.  Allocates the device context block.
2.  Creates a [VMO](/reference/kernel_objects/vm_object.md).
    The [VMO](/reference/kernel_objects/vm_object.md)
    is a kernel object that represents a chunk of memory.
    In this simplified RAM-disk driver, we're creating a
    [VMO](/reference/kernel_objects/vm_object.md) that's `RAMDISK_SIZE`
    bytes long.
    This chunk of memory **is** the RAM-disk &mdash; that's where the data is stored.
    The [VMO](/reference/kernel_objects/vm_object.md)
    creation call, [**zx_vmo_create()**](/reference/syscalls/vmo_create.md),
    returns the [VMO](/reference/kernel_objects/vm_object.md) handle through
    its third argument, which is a member in our context block.
3.  Maps the [VMO](/reference/kernel_objects/vm_object.md) into our address space with
    [**zx_vmar_map()**](/reference/syscalls/vmar_map.md).
    This function returns a pointer to a
    [VMAR](/reference/kernel_objects/vm_address_region.md)
    that points to the entire
    [VMO](/reference/kernel_objects/vm_object.md) (because
    we specified `RAMDISK_SIZE` as the mapping size argument) and gives us read and
    write access (because of the `ZX_VM_FLAG_PERM_*` flags).
    The pointer is stored in our context block's `mapped_addr` member.
4.  Adds our device with **device_add()**,
    just like all the examples we've seen above.
    The difference here, though is that we see two new members: `proto_id` and
    `proto_ops`.
    These are defined as "optional custom protocol" members.
    As usual, we store the newly created device in the `zxdev` member of our
    context block.
5.  Cleans up resources if there were any problems along the way.

For completeness, here's the context block:

```c
typedef struct ramdisk_device {
    zx_device_t*    zxdev;
    uintptr_t       mapped_addr;
    uint32_t        flags;
    zx_handle_t     vmo;
    bool            dead;
} ramdisk_device_t;
```

The fields are:

Type            | Field         | Description
----------------|---------------|----------------
`zx_device_t*`  | zxdev         | the ramdisk device
`uintptr_t`     | mapped_addr   | address of the [VMAR](/reference/kernel_objects/vm_address_region.md)
`uin32_t`       | flags         | device flags
`zx_handle_t`   | vmo           | a handle to our [VMO](/reference/kernel_objects/vm_object.md)
`bool`          | dead          | indicates if the device is still alive

### Operations

Where this device is different from the others that we've seen, though,
is that the **device_add()**
function adds two sets of operations; the "regular" one, and an
optional "protocol specific" one:

```c
static zx_protocol_device_t ramdisk_proto = {
    .version = DEVICE_OPS_VERSION,
    .message = ramdisk_message,
    .get_size = ramdisk_getsize,
    .unbind = ramdisk_unbind,
    .release = ramdisk_release,
};

static block_protocol_ops_t block_ops = {
    .query = ramdisk_query,
    .queue = ramdisk_queue,
};
```

The `zx_protocol_device_t` one handles control messages (**ramdisk_message()**), device size
queries (**ramdisk_getsize()**), and device cleanups (**ramdisk_unbind()** and
**ramdisk_release()**).

<!--- should I discuss the ioctls, or were they to have been removed as part of the simplification? -->

The `block_protocol_ops_t` one contains protocol operations particular to the
block protocol.
We bound these to the device in the `device_add_args_t` structure (step (4) above through
the `.proto_ops` field.
We also set the `.proto_id` field to `ZX_PROTOCOL_BLOCK_IMPL` &mdash; this is what
identifies this driver as being able to handle block protocol operations.

Let's tackle the trivial functions first:

```c
static zx_off_t ramdisk_getsize(void* ctx) {
    return RAMDISK_SIZE;
}

static void ramdisk_unbind(void* ctx) {
    ramdisk_device_t* ramdev = ctx;
    ramdev->dead = true;
    device_unbind_reply(ramdev->zxdev);
}

static void ramdisk_release(void* ctx) {
    ramdisk_device_t* ramdev = ctx;

    if (ramdev->vmo != ZX_HANDLE_INVALID) {
        zx_vmar_unmap(zx_vmar_root_self(), ramdev->mapped_addr, RAMDISK_SIZE);
        zx_handle_close(ramdev->vmo);
    }
    free(ramdev);
}

static void ramdisk_query(void* ctx, block_info_t* bi, size_t* bopsz) {
    ramdisk_get_info(ctx, bi);
    *bopsz = sizeof(block_op_t);
}
```

**ramdisk_getsize()** is the easiest &mdash; it simply returns the size of the resource, in bytes.
In our simplified RAM-disk driver, this is hardcoded as a `#define` near the top of the file.

Next, **ramdisk_unbind()** and **ramdisk_release()** work together.
When the driver is being shut down, the **ramdisk_unbind()** hook is called.
It sets the `dead` flag to indicate that the driver is shutting down (this is checked
in the **ramdisk_queue()** handler, below).
It's expected that the driver will finish up any I/O operations that are in progress (there
won't be any in our RAM-disk), and it should call
**device_unbind_reply()**
to indicate unbinding is complete.

Sometime after **device_unbind_reply()** is called,
the driver's **ramdisk_release()** will be called.
Here we unmap the [VMAR](/reference/kernel_objects/vm_address_region.md),
with [**zx_vmar_unmap()**](/reference/syscalls/vmar_unmap.md), and close the
[VMO](/reference/kernel_objects/vm_object.md),
with [**zx_handle_close()**](/reference/syscalls/handle_close.md).
As our final act, we release the device context block.
At this point, the device is finished.

### Block Operations

The **ramdisk_query()** function is called by the block protocol in order to get
information about the device.
There's a data structure (the `block_info_t`) that's filled out by the driver:

```c
// from fuchsia/hardware/block/driver/c/banjo.h:
typedef struct {
    uint64_t    block_count;        // The number of blocks in this block device
    uint32_t    block_size;         // The size of a single block
    uint32_t    max_transfer_size;  // Max size in bytes per transfer.
                                    // May be BLOCK_MAX_TRANSFER_UNBOUNDED if there
                                    // is no restriction.
    uint32_t    flags;
    uint32_t    reserved;
} block_info_t;

// our helper function
static void ramdisk_get_info(void* ctx, block_info_t* info) {
    ramdisk_device_t* ramdev = ctx;
    memset(info, 0, sizeof(*info));
    info->block_size = BLOCK_SIZE;
    info->block_count = BLOCK_COUNT;
    // Arbitrarily set, but matches the SATA driver for testing
    info->max_transfer_size = fuchsia_hardware_block::wire::kMaxTransferUnbounded;
    info->flags = ramdev->flags;
}
```

In this simplified driver, the `block_size`, `block_count`, and `max_transfer_size`
fields are hardcoded numbers.

The `flags` member is used to identify if the device is read-only (`FLAG_READONLY`,
otherwise it's read/write), removable (`FLAG_REMOVABLE`, otherwise it's not
removable) or has a bootable partition (`FLAG_BOOTPART`, otherwise it doesn't).

The final value that **ramdisk_query()** returns is the "block operation size" value
through the pointer to `bopsz`.
This is a host-maintained block that's big enough to contain the `block_op_t` *plus*
any additional data the driver wants (appended to the `block_op_t`), like an
extended context block.

### Reading and writing

Finally, it's time to discuss the actual "block" data transfers; that is, how does
data get read from / written to the device?

The second block protocol handler, **ramdisk_queue()**, performs that function.

As you might suspect from the name, it's intended that this hook starts whatever
transfer operation (a read or a write) is requested, but doesn't require that
the operation completes before the hook returns.
This is a little like what we saw in earlier chapters
in the **read()** and **write()** handlers
for devices like `/dev/misc/demo-fifo` &mdash; there, we could either return
data immediately, or put the client to sleep, waking it up later when data (or room
for data) became available.

With **ramdisk_queue()** we get passed a block operations structure that indicates
the expected operation: `BLOCK_OP_READ`, `BLOCK_OP_WRITE`, or `BLOCK_OP_FLUSH`.
The structure also contains additional fields telling us the offset and size of
the transfer (from `//src/lib/ddk/include/ddk/protocol/block.h`):

```c
// simplified from original
struct block_op {
    struct {
        uint32_t    command;    // command and flags
        uint32_t    extra;      // available for temporary use
        zx_handle_t vmo;        // vmo of data to read or write
        uint32_t    length;     // transfer length in blocks (0 is invalid)
        uint64_t    offset_dev; // device offset in blocks
        uint64_t    offset_vmo; // vmo offset in blocks
        uint64_t*   pages;      // optional physical page list
    } rw;

    void (*completion_cb)(block_op_t* block, zx_status_t status);
};
```

The transfer takes place to or from the `vmo` in the structure &mdash; in the case of
a read, we transfer data to the [VMO](/reference/kernel_objects/vm_object.md),
and vice versa for a write.
The `length` indicates the number of *blocks* (not bytes) to transfer, and the
two offset fields, `offset_dev` and `offset_vmo`, indicate the relative offsets (again,
in blocks not bytes) into the device and the [VMO](/reference/kernel_objects/vm_object.md)
of where the transfer should take place.

The implementation is straightforward:

```c
static void ramdisk_queue(void* ctx, block_op_t* bop) {
    ramdisk_device_t* ramdev = ctx;

    // (1) see if we should still be handling requests
    if (ramdev->dead) {
        bop->completion_cb(bop, ZX_ERR_IO_NOT_PRESENT);
        return;
    }

    // (2) what operation are we performing?
    switch ((bop->command &= BLOCK_OP_MASK)) {
    case BLOCK_OP_READ:
    case BLOCK_OP_WRITE: {
        // (3) perform validation common for both
        if ((bop->rw.offset_dev >= BLOCK_COUNT)
            || ((BLOCK_COUNT - bop->rw.offset_dev) < bop->rw.length)
            || bop->rw.length * BLOCK_SIZE > MAX_TRANSFER_BYTES) {
            bop->completion_cb(bop, ZX_ERR_OUT_OF_RANGE);
            return;
        }

        // (4) compute address
        void* addr = (void*) ramdev->mapped_addr + bop->rw.offset_dev * BLOCK_SIZE;
        zx_status_t status;

        // (5) now perform actions specific to each
        if (bop->command == BLOCK_OP_READ) {
            status = zx_vmo_write(bop->rw.vmo, addr, bop->rw.offset_vmo * BLOCK_SIZE,
                                  bop->rw.length * BLOCK_SIZE);
        } else {
            status = zx_vmo_read(bop->rw.vmo, addr, bop->rw.offset_vmo * BLOCK_SIZE,
                                 bop->rw.length * BLOCK_SIZE);
        }

        // (6) indicate completion
        bop->completion_cb(bop, status);
        break;
        }

    case BLOCK_OP_FLUSH:
        bop->completion_cb(bop, ZX_OK);
        break;

    default:
        bop->completion_cb(bop, ZX_ERR_NOT_SUPPORTED);
        break;
    }
}
```

As usual, we establish a context block at the top by casting the `ctx` argument.
The `bop` argument is the "block operation" structure we saw above.
The `command` field indicates what the **ramdisk_queue()** function should do.

In step (1), we check to see if we've set the `dead` flag (**ramdisk_unbind()**
sets it when required).
If so, it means that our device is no longer accepting new requests, so we return
`ZX_ERR_IO_NOT_PRESENT` in order to encourage clients to close the device.

In step (3), we handle some common validation for both read and write &mdash;
neither should allow offsets that exceed the size of the device, nor transfer
more than the maximum transfer size.

Similarly, in step (4) we compute the device address (that is, we establish a
pointer to our [VMAR](/reference/kernel_objects/vm_address_region.md)
that's offset by the appropriate number of blocks as per the request).

In step (5) we perform either a [**zx_vmo_read()**](/reference/syscalls/vmo_read.md)
or a [**zx_vmo_write()**](/reference/syscalls/vmo_write.md), depending
on the command.
This is what transfers data between a pointer within our
[VMAR](/reference/kernel_objects/vm_address_region.md) (`addr`)
and the client's [VMO](/reference/kernel_objects/vm_object.md) (`bop->rw.vmo`).
Notice that in the read case, we *write* to the [VMO](/reference/kernel_objects/vm_object.md),
and in the write case, we *read* from the [VMO](/reference/kernel_objects/vm_object.md).

Finally, in step (6) (and the other two cases), we signal completion with the
`completion` callback in the block ops structure.

The interesting thing about completion is that:

*   it doesn't have to happen right away &mdash; we could have queued this
    operation and signalled completion some time later,
*   it is allowed to be called before this function returns (like we did).

The last point simply means that we are not *forced* to defer completion until
after the queuing function returns.
This allows us to complete the operation directly in the function.
For our trivial RAM-disk example, this makes sense &mdash; we have the ability to
do the data transfer to or from media instantly; no need to defer.

## How is the real one more complicated?

The RAM-disk presented above is somewhat simplified from the "real" RAM-disk
device (present at `//src/devices/block/drivers/ramdisk/ramdisk.c`).

The real one adds the following functionality:

*   dynamic device creation through a new VMO
*   ability to use an existing VMO
*   background thread
*   sleep mode

<!--- how much, if anything, do we want to say about this one? I found the dynamic -->
<!--- device creation of interest, for example... -->

