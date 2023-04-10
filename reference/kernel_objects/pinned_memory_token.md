# Pinned Memory Token

## NAME

pinned_memory_token - Representation of a device DMA grant

## SYNOPSIS

Pinned Memory Tokens (PMTs) represent an outstanding access grant to a device
for performing DMA.

## DESCRIPTION

PMTs are obtained by [pinning memory with a BTI object](/docs/reference/syscalls/bti_pin.md).
It is valid for the device associated with the BTI to access the memory represented
by the PMT for as long as the PMT object is around.  When the PMT object is
destroyed, either via [`zx_handle_close()`], [`zx_pmt_unpin()`], or process
termination, access to the represented memory becomes illegal (this is
enforced by hardware on systems with the capability to do so, such as IOMMUs).

If a PMT object is destroyed by means other than [`zx_pmt_unpin()`], the
underlying memory is *quarantined*.  See
[bus_transaction_initiator](bus_transaction_initiator.md) for more details.

## SEE ALSO

 - [bus_transaction_initiator](bus_transaction_initiator.md) - Bus Transaction Initiators

## SYSCALLS

 - [`zx_bti_pin()`] - pin memory and grant access to it to the BTI
 - [`zx_pmt_unpin()`] - revoke access and unpin memory

[`zx_bti_pin()`]: /docs/reference/syscalls/bti_pin.md
[`zx_handle_close()`]: /docs/reference/syscalls/handle_close.md
[`zx_pmt_unpin()`]: /docs/reference/syscalls/pmt_unpin.md
