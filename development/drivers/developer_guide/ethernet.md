# Ethernet Devices

## Overview

This chapter looks into the details of ethernet drivers, using the Intel driver code
for specific examples.

In order to handle ethernet devices, two distinct parts are involved.
A "top half" driver handles the generic ethernet protocol, and is located in
`//src/connectivity/ethernet/drivers/ethernet/ethernet.c` (yes, three "ethernets" in a row),
and one or more "bottom half" drivers handle the actual devices, located one
directory higher in `//src/connectivity/ethernet/drivers/`**_devicename_**`/`.

Multiple Zircon IPC protocols are used for communication between modules.

> We'll just use the term "protocol" to refer to these.
> Granted, we *are* discussing an Ethernet driver, but since we won't be
> discussing any of the on-wire communications protocols supported by the driver,
> this usage shouldn't result in any confusion.
>


The top half provides a protocol interface that conforms to `ZX_PROTOCOL_ETHERNET_IMPL`.
The bottom half provides a protocol interface that conforms to whatever the
hardware is connected to (for example, this might be `ZX_PROTOCOL_PCI`, for
PCI-based ethernet cards, or `ZX_PROTOCOL_USB_INTERFACE` for USB-based ethernet devices,
and so on).
We'll focus on the PCI version here.

The bottom half drivers all expose a `ZX_PROTOCOL_ETHERNET_IMPL` binding, which is how
the top half finds the bottom halves.

Effectively, the bottom half ethernet driver is responsible for managing the hardware
associated with the ethernet device, and presenting a consistent abstraction of that
hardware for use by the top half.
The top half manages the ethernet interface to the system.

![Figure: Relationship amongst layers in ethernet driver stack](images/ethernet-000-cropped.png)

# Intel PCI-based ethernet

> NOTE: this driver has been removed from the tree in favor of a port of the FreeBSD driver,
> but the discussion here is still relevant for understanding device drivers in Fuchsia.

The Intel ethernet driver can be found in `//src/connectivity/ethernet/drivers/intel-ethernet`,
and consists of the following files:

<dl>
<dt>`ethernet.c`
<dd>The device driver part of the code; handles interface to protocols.
<dt>`ie.c`
<dd>The Intel specific part of the code; knows about the hardware registers on the card.
<dt>`ie-hw.h`
<dd>Contains the manifest constants for all of the control registers.
<dt>`ie.h`
<dd>Common definitions (such as the device context block)
</dl>

This driver not only handles the `ethmac` protocol, but also:

*   finds its device on the PCI bus,
*   attaches to legacy or Message Signaled Interrupts (**MSI**),
*   maps I/O memory, and
*   creates a background IRQ handling thread.

## Binding

The file `ethernet.c` contains the binding information, implemented by the standard
binding macros introduced in the [Simple Drivers](simple.md) chapter:

```c
ZIRCON_DRIVER_BEGIN(intel_ethernet, intel_ethernet_driver_ops, "zircon", "0.1", 11)
    BI_ABORT_IF(NE, BIND_PROTOCOL, ZX_PROTOCOL_PCI),
    BI_ABORT_IF(NE, BIND_PCI_VID, 0x8086),
    BI_MATCH_IF(EQ, BIND_PCI_DID, 0x100E), // Qemu
    BI_MATCH_IF(EQ, BIND_PCI_DID, 0x15A3), // Broadwell
    BI_MATCH_IF(EQ, BIND_PCI_DID, 0x1570), // Skylake
    BI_MATCH_IF(EQ, BIND_PCI_DID, 0x1533), // I210 standalone
    BI_MATCH_IF(EQ, BIND_PCI_DID, 0x1539), // I211-AT
    BI_MATCH_IF(EQ, BIND_PCI_DID, 0x156f), // I219-LM (Dawson Canyon NUC)
    BI_MATCH_IF(EQ, BIND_PCI_DID, 0x15b7), // Skull Canyon NUC
    BI_MATCH_IF(EQ, BIND_PCI_DID, 0x15b8), // I219-V
    BI_MATCH_IF(EQ, BIND_PCI_DID, 0x15d8), // Kaby Lake NUC
ZIRCON_DRIVER_END(intel_ethernet)
```

This ends up binding to ethernet cards that are identified by vendor ID `0x8086` (Intel),
and have any of the listed device IDs (the `BIND_PCI_DID` lines indicate the allowed
hexadecimal device IDs).
It also requires the `ZX_PROTOCOL_PCI` protocol.

Note the sense of the logic here &mdash; the vendor ID is tested with a
"`BI_ABORT_IF(NE`" construct (meaning, "**ABORT IF** the values are **N**ot **E**qual"),
whereas the device IDs are tested with "`BI_MATCH_IF(EQ`" constructs (meaning "**MATCH
IF** the values are **EQ**ual").

Intuitively, you might think that the vendor ID could be tested with a "`BI_MATCH_IF(EQ`"
as well, (looking for vendor `0x8086`), but this would have two major problems.
First, evaluation stops as soon as a condition is true, so that means that **any** device
that had the Intel vendor ID would be considered a "match."
Second, even if the device wasn't an Intel vendor ID, it would open the possibility
of allowing matches to other vendors' devices that had the same device ID as listed.

> The individual tests are evaluated in sequence.
> The first one that's true terminates evaluation, and performs
> the given action (i.e., `ABORT` or `MATCH`).

## More about binding

From the command line, `dm drivers` will display this information.
Here's the relevant portion for the Intel ethernet driver:

```sh
$ dm drivers
<snip>
    Name    : intel_ethernet
    Driver  : /boot/driver/intel-ethernet.so
    Flags   : 0x00000000
    Binding : 11 instructions (88 bytes)
    [1/11]: if (Protocol != 0x70504349) return no-match;
    [2/11]: if (PCI.VID != 0x00008086) return no-match;
    [3/11]: if (PCI.DID == 0x0000100e) return match;
    [4/11]: if (PCI.DID == 0x000015a3) return match;
    [5/11]: if (PCI.DID == 0x00001570) return match;
    [6/11]: if (PCI.DID == 0x00001533) return match;
    [7/11]: if (PCI.DID == 0x00001539) return match;
    [8/11]: if (PCI.DID == 0x0000156f) return match;
    [9/11]: if (PCI.DID == 0x000015b7) return match;
    [10/11]: if (PCI.DID == 0x000015b8) return match;
    [11/11]: if (PCI.DID == 0x000015d8) return match;
```

The `Name` field indicates the name of the driver, given as the first argument to the
`ZIRCON_DRIVER_BEGIN` and `ZIRCON_DRIVER_END` macros.
The `Driver` field indicates the location of the shared object that contains the driver code.

The last section, the binding instructions, corresponds with the `BI_ABORT_IF` and `BI_MATCH_IF`
macro directives.
Note that the first binding instruction compares the field `Protocol` against the hexadecimal
number `0x70504349` &mdash; that "number" is simply the ASCII encoding of the string "`pPCI`",
indicating the PCI protocol (you can see all of the encodings in
`//src/lib/ddk/include/ddk/protodefs.h`)

From the `ZIRCON_DRIVER_BEGIN` macro, the `intel_ethernet_driver_ops`
structure contains the driver operations, in this case just the binding function
**eth_bind()**.

Let's turn our attention to the binding function itself.

## PCI interface

The first part of the binding function deals with the PCI interface.

The Intel ethernet driver is a PCI bus peripheral.
As such, it needs to first query the PCI configuration registers in order to discover
where the BIOS (or other startup program) has located the device in memory
address space, and what interrupt it was assigned.
Second, it needs to initialize the device for use (such as mapping the configuration
registers and attaching to the device's interrupt).

As usual, the binding function allocates and initializes a context block:

```c
static zx_status_t eth_bind(void* ctx, zx_device_t* dev) {
    ethernet_device_t* edev;
    if ((edev = calloc(1, sizeof(ethernet_device_t))) == NULL) {
        return ZX_ERR_NO_MEMORY;
    }
    mtx_init(&edev->lock, mtx_plain);
    mtx_init(&edev->eth.send_lock, mtx_plain);
```

This allocates a zeroed ethernet context block (`ethernet_device_t`).
Then we initialize two mutexes (one for locking the device itself (`edev->lock`), and one
for locking the ethernet send buffers (`edev->eth.send_lock`)).

We'll examine the context block in more detail below.

### PCI protocol operations

The next step fetches the PCI protocol operations pointer (or fails if it can't):

```c
    if (device_get_protocol(dev, ZX_PROTOCOL_PCI, &edev->pci)) {
        printf("no pci protocol\n");
        goto fail;
    }
```

This populates `edev->pci` (of type `pci_protocol_t`) with pointers to functions that
provide PCI protocol services.
Of the many functions available, we use the following subset (listed in order of
use in the binding function):

Function            | Description
--------------------|------------------------------------------------------------------------------
`get_bti`           | Used to get the Bus Transaction Initiator (**[BTI](/docs/reference/kernel_objects/bus_transaction_initiator.md)**) for the device
`query_irq_mode`    | Returns the number of the specific type of IRQ available (MSI or legacy)
`set_irq_mode`      | Requests the specified IRQ mode to be used for the device
`map_interrupt`     | Creates an IRQ handle associated with the device's interrupt
`map_bar`           | Returns a pointer to the Base Address Register (**BAR**) of the PCI device
`enable_bus_master` | Enables / disables bus mastering for the device

> Note that the function names given in the table above are the member names within
> the `pci_protocol_t` structure; throughout the code we'll use the **pci_...()** accessor
> functions to call the protocol ops.

### Fetch the BTI

The first PCI function we call is
**pci_get_bti()**:

```c
    zx_status_t status = pci_get_bti(&edev->pci, 0, &edev->btih);
    if (status != ZX_OK) {
        goto fail;
    }
```

A [BTI](/docs/reference/kernel_objects/bus_transaction_initiator.md)
is used to represent the bus mastering / DMA capability of a device.
It can be used for granting memory access to a device.
The [BTI](/docs/reference/kernel_objects/bus_transaction_initiator.md)
handle is stored in `edev->btih` and is used later to initialize transfer buffers.
The [DMA](/docs/concepts/drivers/driver_development/dma.md) section talks more about this.

### Discover and map interrupts

The interrupt is discovered and mapped next:

```c
    // Query whether we have MSI or Legacy interrupts.
    uint32_t irq_cnt = 0;
    if ((pci_query_irq_mode(&edev->pci, ZX_PCIE_IRQ_MODE_MSI, &irq_cnt) == ZX_OK) &&
        (pci_set_irq_mode(&edev->pci, ZX_PCIE_IRQ_MODE_MSI, 1) == ZX_OK)) {
        printf("eth: using MSI mode\n");
    } else if ((pci_query_irq_mode(&edev->pci, ZX_PCIE_IRQ_MODE_LEGACY, &irq_cnt) == ZX_OK) &&
               (pci_set_irq_mode(&edev->pci, ZX_PCIE_IRQ_MODE_LEGACY, 1) == ZX_OK)) {
        printf("eth: using legacy irq mode\n");
    } else {
        printf("eth: failed to configure irqs\n");
        goto fail;
    }

    zx_status_t r = pci_map_interrupt(&edev->pci, 0, &edev->irqh);
    if (r != ZX_OK) {
        printf("eth: failed to map irq\n");
        goto fail;
    }
```

The **pci_query_irq_mode()**
function determines if the device supports any `MSI` or `LEGACY`
style interrupts, and returns the count (in `irq_cnt`).
We're expecting one interrupt, so we ignore the count and examine just the return status.
If the return status indicates one or more interrupts of that type exist, we set the device to
use that mode.

The **pci_map_interrupt()**
function is then used to bind the hardware interrupt to a handle, stored in `edev->irqh`.

We'll see this handle later, when we look at the interrupt service thread.

### Map PCI BAR

Next up, we map the PCI BAR:

```c
    // map iomem
    uint64_t sz;
    zx_handle_t h;
    void* io;
    r = pci_map_bar(&edev->pci, 0u, ZX_CACHE_POLICY_UNCACHED_DEVICE, &io, &sz, &h);
    if (r != ZX_OK) {
        printf("eth: cannot map io %d\n", h);
        goto fail;
    }
    edev->eth.iobase = (uintptr_t)io;
    edev->ioh = h;

    if ((r = pci_enable_bus_master(&edev->pci, true)) < 0) {
        printf("eth: cannot enable bus master %d\n", r);
        goto fail;
    }
```

The call to **pci_map_bar()** creates a handle to the first BAR
(the `0u` as the second argument
specifies the BAR ID number), which we store into the context block's `ioh` member.
(We also capture the virtual address into `edev->eth.iobase`.)

### Ethernet setup and configuration

At this point, we have access to enough of the device that we can go and set it up:

```c
    if (eth_enable_phy(&edev->eth) != ZX_OK) {
        goto fail;
    }

    if (eth_reset_hw(&edev->eth)) {
        goto fail;
    }
```

The implementation of **eth_enable_phy()** and **eth_reset_hw()**
is in the `ie.c` file.

### DMA buffer setup and hardware configuration

With the device configured, we can now set up the DMA buffers.
Here we see the [BTI](/docs/reference/kernel_objects/bus_transaction_initiator.md)
handle, `edev->btih`, that we set up above, as the 2nd argument to
**io_buffer_init()**:

```c

    r = io_buffer_init(&edev->buffer, edev->btih, ETH_ALLOC, IO_BUFFER_RW | IO_BUFFER_CONTIG);
    if (r < 0) {
        printf("eth: cannot alloc io-buffer %d\n", r);
        goto fail;
    }

    eth_setup_buffers(&edev->eth, io_buffer_virt(&edev->buffer), io_buffer_phys(&edev->buffer));
    eth_init_hw(&edev->eth);
```

The **io_buffer_init()**
function zeroes the buffer, and creates a [VMO](/docs/reference/kernel_objects/vm_object.md)
handle to the [BTI](/docs/reference/kernel_objects/bus_transaction_initiator.md).
The **eth_setup_buffers()** and **eth_init_hw()** functions are defined in the `ie.c` module.

### Final driver binding

The next part binds the device name ("`intel-ethernet`"), context block (`edev`,
allocated above), device operations (`device_ops`, which supports suspend, resume, and release),
and the additional optional protocol ops for ethernet (identified as `ZX_PROTOCOL_ETHERNET_IMPL`
and contained in `ethernet_impl_ops`):

```c
    device_add_args_t args = {
        .version = DEVICE_ADD_ARGS_VERSION,
        .name = "intel-ethernet",
        .ctx = edev,
        .ops = &device_ops,
        .proto_id = ZX_PROTOCOL_ETHERNET_IMPL,
        .proto_ops = &ethernet_impl_ops,
    };

    if (device_add(dev, &args, &edev->zxdev)) {
        goto fail;
    }
```

### Interrupt thread creation

Finally, the background Interrupt Handling Thread (**IHT**), **irq_thread()** is created:

```c
    thrd_create_with_name(&edev->thread, irq_thread, edev, "eth-irq-thread");
    thrd_detach(edev->thread);

    printf("eth: intel-ethernet online\n");

    return ZX_OK;
```

As discussed in the [Interrupts](/docs/concepts/drivers/driver_development/interrupts.md) section,
the IHT handles asynchronous hardware events.
We'll look at the thread itself below.

### Failure handling

In case of failure, the `fail` label is the target of various `goto`s within the code, and is
responsible for cleanup of allocated resources as well as returning a failure code to the caller:

```c
fail:
    io_buffer_release(&edev->buffer);
    if (edev->btih) {
        zx_handle_close(edev->btih);
    }
    if (edev->ioh) {
        pci_enable_bus_master(&edev->pci, false);
        zx_handle_close(edev->irqh);
        zx_handle_close(edev->ioh);
    }
    free(edev);
    return ZX_ERR_NOT_SUPPORTED;
}
```

That concludes the discussion of the binding function.

## The context structure

At this point, we can circle back and take a look at the context structure:

```c
typedef struct ethernet_device {
    ethdev_t        eth;
    mtx_t           lock;
    eth_state       state;
    zx_device_t*    zxdev;
    pci_protocol_t  pci;
    zx_handle_t     ioh;
    zx_handle_t     irqh;
    thrd_t          thread;
    zx_handle_t     btih;
    io_buffer_t     buffer;
    bool            online;

    // callback interface to attached ethernet layer
    ethernet_ifc_t*   ifc;
    void*           cookie;
} ethernet_device_t;
```

It holds all of the context for the ethernet devices.

<!--- How much discussion do we want of the context block members? -->

## Ethernet protocol operations

Recall from the discussion around the binding function
**eth_bind()**
that we bound an `ethernet_impl_protocol_ops_t` structure called
`ethernet_impl_ops` to the driver.
This structure provides the following "bottom-half" ethernet driver protocol operations
for the Intel driver:

```c
static ethernet_impl_protocol_ops_t ethernet_impl_ops = {
    .query = eth_query,
    .stop = eth_stop,
    .start = eth_start,
    .queue_tx = eth_queue_tx,
    .set_param = eth_set_param,
//  .get_bti not supported
};
```

We examine each in turn below.

### Ethernet protocol: **query()**

The **query()** function takes three parameters:
a context block, an options specifier, and a pointer to
an `ethernet_info_t` where the information should be stored.

> Note that at the present time, there are no options defined; therefore, the driver
> should return `ZX_ERR_INVALID_ARGS` in case of a non-zero value.

The `ethernet_info_t` structure is defined as follows (reserved fields omitted for clarity):

```c
typedef struct ethernet_info {
    uint32_t    features;
    uint32_t    mtu;
    uint8_t     mac[ETH_MAC_SIZE];
} ethernet_info_t;
```

The `mtu` field contains the Maximum Transmission Unit (**MTU**) size that the driver
can support.
A common value is `1500`.

The `mac` field contains `ETH_MAC_SIZE` (6 bytes) worth of Media Access Control (**MAC**)
address in big-endian order (that is, for a MAC of `01:23:45:67:89:ab`, the value of
`mac[0]` is `0x01`).

Finally, the `features` field contains a bitmap of available features:

Feature                 | Meaning
------------------------|--------------------------------------------
`ETHERNET_FEATURE_WLAN`   | Device is a wireless network device
`ETHERNET_FEATURE_SYNTH`  | Device is a synthetic network device
`ETHERNET_FEATURE_DMA`    | Driver will be doing DMA to/from the VMO

The Intel driver's **eth_query()** is representative:

```c
static zx_status_t eth_query(void* ctx, uint32_t options, ethernet_info_t* info) {
    ethernet_device_t* edev = ctx;

    if (options) {
        return ZX_ERR_INVALID_ARGS;
    }

    memset(info, 0, sizeof(*info));
    ZX_DEBUG_ASSERT(ETH_TXBUF_SIZE >= ETH_MTU);
    info->mtu = ETH_MTU;
    memcpy(info->mac, edev->eth.mac, sizeof(edev->eth.mac));

    return ZX_OK;
}
```

In that it returns `ZX_ERR_INVALID_ARGS` in case the `options` parameter is non zero,
and otherwise fills the `mtu` and `mac` members.

### Ethernet protocol: **queue_tx()**

The **queue_tx()** function is responsible for taking the `ethernet_netbuf_t` network
buffer and transmitting it.

```c
static zx_status_t eth_queue_tx(void* ctx, uint32_t options, ethernet_netbuf_t* netbuf) {
    ethernet_device_t* edev = ctx;
    if (edev->state != ETH_RUNNING) {
        return ZX_ERR_BAD_STATE;
    }
    return eth_tx(&edev->eth, netbuf->data, netbuf->len);
}
```

The real work for the Intel ethernet driver is done in `ie.c`:

```c
status_t eth_tx(ethdev_t* eth, const void* data, size_t len) {
    if (len > ETH_TXBUF_DSIZE) {
        printf("intel-eth: unsupported packet length %zu\n", len);
        return ZX_ERR_INVALID_ARGS;
    }

    zx_status_t status = ZX_OK;

    mtx_lock(&eth->send_lock);

    reap_tx_buffers(eth);

    // obtain buffer, copy into it, setup descriptor
    framebuf_t *frame = list_remove_head_type(&eth->free_frames, framebuf_t, node);
    if (frame == NULL) {
        status = ZX_ERR_NO_RESOURCES;
        goto out;
    }

    uint32_t n = eth->tx_wr_ptr;
    memcpy(frame->data, data, len);
    // Pad out short packets.
    if (len < 60) {
      memset(frame->data + len, 0, 60 - len);
      len = 60;
    }
    eth->txd[n].addr = frame->phys;
    eth->txd[n].info = IE_TXD_LEN(len) | IE_TXD_EOP | IE_TXD_IFCS | IE_TXD_RS;
    list_add_tail(&eth->busy_frames, &frame->node);

    // inform hw of buffer availability
    n = (n + 1) & (ETH_TXBUF_COUNT - 1);
    eth->tx_wr_ptr = n;
    writel(n, IE_TDT);

out:
    mtx_unlock(&eth->send_lock);
    return status;
}
```

This function performs buffer management and talks to the hardware.
It first locks the mutex, and then finds an available buffer.
This is done by calling **reap_tx_buffers()** to find available buffers,
and then calling the macro **list_remove_head_type()** to try and fetch
a buffer from the head of the list.
If no buffer is available, an error status (`ZX_ERR_NO_RESOURCES`) is set
and the function returns.

Otherwise, the frame data is copied (short frames, less than 60 bytes, are padded
with zeros).

The hardware is kicked with the macro **writel()**, which writes to the
`IE_TDT` register telling it which buffer is available to be written to the ethernet.

At this point, the frame is queued at the chip level, and will be sent shortly.
(The timing depends on if there are other frames queued before this one.)

### Ethernet protocol: **set_param()**

Sets a parameter based on the passed `param` argument and `value` argument.
The Intel driver supports enabling or disabling promiscuous mode, and nothing else:

```c
static zx_status_t eth_set_param(void *ctx, uint32_t param, int32_t value, void* data) {
    ethernet_device_t* edev = ctx;
    zx_status_t status = ZX_OK;

    mtx_lock(&edev->lock);

    switch (param) {
    case ETHERNET_SETPARAM_PROMISC:
        if ((bool)value) {
            eth_start_promisc(&edev->eth);
        } else {
            eth_stop_promisc(&edev->eth);
        }
        status = ZX_OK;
        break;
    default:
        status = ZX_ERR_NOT_SUPPORTED;
    }
    mtx_unlock(&edev->lock);

    return status;
}
```

The following parameters are available:

Parameter                           | Meaning (additional data)
------------------------------------|-------------------------------------------------------------
`ETHERNET_SETPARAM_PROMISC`           | Controls promiscuous mode (bool)
`ETHERNET_SETPARAM_MULTICAST_PROMISC` | Controls multicast promiscuous mode (bool)
`ETHERNET_SETPARAM_MULTICAST_FILTER`  | Sets multicast filtering addresses (count + array)
`ETHERNET_SETPARAM_DUMP_REGS`         | Used for debug, dumps the registers (no additional data)

For multicast filtering, the `value` argument indicates the count of MAC addresses sequentially
presented with the `data` argument. For example, if `value` was `2`, then `data`
would point to two back-to-back MAC addresses (2 x 6 = 12 bytes total).

Note that if a parameter is not supported, the value `ZX_ERR_NOT_SUPPORTED` is returned.

### Ethernet protocol: **start()** and **stop()**

The two functions, **eth_start()** and **eth_stop()** are used to start and stop
the ethernet device:

```c
static void eth_stop(void* ctx) {
    ethernet_device_t* edev = ctx;
    mtx_lock(&edev->lock);
    edev->ifc = NULL;
    mtx_unlock(&edev->lock);
}

static zx_status_t eth_start(void* ctx, ethernet_ifc_t* ifc, void* cookie) {
    ethernet_device_t* edev = ctx;
    zx_status_t status = ZX_OK;

    mtx_lock(&edev->lock);
    if (edev->ifc) {
        status = ZX_ERR_BAD_STATE;
    } else {
        edev->ifc = ifc;
        edev->cookie = cookie;
        edev->ifc->status(edev->cookie, edev->online ? ETHERNET_STATUS_ONLINE : 0);
    }
    mtx_unlock(&edev->lock);

    return status;
}
```

The Intel ethernet driver code shown above is typical; the `ifc` member of the context
block is used as both an indication of status (`NULL` if stopped) and, when running,
it points to a valid interface block.

### Ethernet protocol: **get_bti()**

The Intel ethernet driver doesn't support the optional **get_bti()** callout.

This callout is used to return a handle to the [BTI](/docs/reference/kernel_objects/bus_transaction_initiator.md).
In case the device doesn't support it, it can either leave it out of the `ethernet_impl_protocol_ops_t`
structure (like the Intel ethernet driver does), or it can return `ZX_HANDLE_INVALID`.

If supported, the handle is returned from the function.
Note that the ownership of the handle is *not* transferred; the ethernet driver still
owns the handle.
In particular, the caller must not close the handle.

## Receiving data

The IHT thread created by the binding function waits for data from the ethernet hardware.
When data arrives, it calls **eth_handle_irq()** to process the data.

The portion of the thread in `ethernet.c` is as follows:

```c
static int irq_thread(void* arg) {
    ethernet_device_t* edev = arg;
    for (;;) {
        zx_status_t r;
        r = zx_interrupt_wait(edev->irqh, NULL);
        if (r != ZX_OK) {
            printf("eth: irq wait failed? %d\n", r);
            break;
        }
        mtx_lock(&edev->lock);
        unsigned irq = eth_handle_irq(&edev->eth);
        if (irq & ETH_IRQ_RX) {
            void* data;
            size_t len;

            while (eth_rx(&edev->eth, &data, &len) == ZX_OK) {
                if (edev->ifc && (edev->state == ETH_RUNNING)) {
                    edev->ifc->recv(edev->cookie, data, len, 0);
                }
                eth_rx_ack(&edev->eth);
            }
        }
        if (irq & ETH_IRQ_LSC) {
            bool was_online = edev->online;
            bool online = eth_status_online(&edev->eth);
            zxlogf(DEBUG, "intel-eth: ETH_IRQ_LSC fired: %d->%d", was_online, online);
            if (online != was_online) {
                edev->online = online;
                if (edev->ifc) {
                    edev->ifc->status(edev->cookie, online ? ETHERNET_STATUS_ONLINE : 0);
                }
            }
        }
        mtx_unlock(&edev->lock);
    }
    return 0;
}
```

The thread waits on an interrupt, and, when one occurs, calls **eth_handle_irq()**
to read the interrupt reason register (which also clears the interrupt
indication on the card).

Based on the value read from **eth_handle_irq()**,
there are two major flows in the thread:

1.  the bit `ETH_IRQ_RX` is present &mdash; this indicates data has been
    received by the card,
2.  the bit `ETH_IRQ_LSC` is present &mdash; this indicates a Line Status
    Change (LSC) event has been detected by the card.

If data has been received, the following functions are called:

*   **eth_rx()** &mdash; obtains a pointer to the receive buffer containing the data
*   **eth_rx_ack()** &mdash; acknowledges receipt of the packet by writing to registers on the card


Note that further processing is done by the ethernet device protocol (available through `edev->ifc`):

*   **edev->ifc->recv()** &mdash; processes the received data
*   **edev->ifc->status()** &mdash; processes the status change

In the case of a line status change, **eth_status_online()** is called to handle the event.

```c
status_t eth_rx(ethdev_t* eth, void** data, size_t* len) {
    uint32_t n = eth->rx_rd_ptr;
    uint64_t info = eth->rxd[n].info;

    if (!(info & IE_RXD_DONE)) {
        return ZX_ERR_SHOULD_WAIT;
    }

    // copy out packet
    zx_status_t r = IE_RXD_LEN(info);

    *data = eth->rxb + ETH_RXBUF_SIZE * n;
    *len = r;

    return ZX_OK;
}
```

