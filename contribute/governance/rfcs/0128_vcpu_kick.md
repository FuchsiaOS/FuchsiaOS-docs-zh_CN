<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0128" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Summary

We propose adding a new syscall, `zx_vcpu_kick`, that can cause a running VCPU
to exit to the host and return to user-space. Additionally, we propose renaming
`zx_vcpu_resume` to `zx_vcpu_enter` in order to have a consistent nomenclature.

## Motivation

When a VCPU is running, it may be blocked indefinitely within a call to
`zx_vcpu_enter`. This causes problems for a virtual machine manager, as to
cleanly shutdown a VCPU, a virtual machine manager needs the syscall to return
control to the calling thread, so that it can safely free all of the related
resources.

Furthermore, for the purposes of simplifying integration testing, it is
convenient to have a method of forcing a VCPU to exit from a guest. That way a
test can be written more concisely and precisely.

## Stakeholders

_Facilitator:_ cpu

_Reviewers:_ adanis, tamird, jamesr

_Consulted:_ dgreenaway, brunodalbo

_Socialization:_ The design was discussed in a chat thread, as part of the
solution to a problem that the Connectivity team were facing with the netemul
tests.

## Design

This proposal consists of the following changes to the Zircon syscall interface:

  1. The addition of `zx_vcpu_kick`
  1. The renaming of `zx_vcpu_resume` to `zx_vcpu_enter`

Where `zx_vcpu_enter` and `zx_vcpu_kick` will be defined as:

```fidl
/// Enter a VCPU, and start or continue execution.
/// Rights: handle must be of type ZX_OBJ_TYPE_VCPU and have ZX_RIGHT_EXECUTE.
// @blocking
zx_status_t zx_vcpu_enter(
    zx_handle_t handle,
    uint32_t options,
    zx_port_packet_t* packet
);

/// Exit from the current or next call to |vcpu_enter|.
/// Rights: handle must be of type ZX_OBJ_TYPE_VCPU and have ZX_RIGHT_EXECUTE.
zx_status_t zx_vcpu_kick(
    zx_handle_t handle
);
```

When `zx_vcpu_kick` is called on a VCPU handle, any currently running call to
`zx_vcpu_enter` on that same handle will return `ZX_ERR_CANCELED`. Furthermore,
if `zx_vcpu_enter` was not running at the time `zx_vcpu_kick` was called, the
next call to `zx_vcpu_enter` will immediately return `ZX_ERR_CANCELED`. This
allows a virtual machine manager to call `zx_vcpu_kick` and be guaranteed that
`zx_vcpu_enter` will return `ZX_ERR_CANCELED` next. Conversely, this means that
if `zx_vcpu_enter` has not returned `ZX_ERR_CANCELED` yet, it will only do so
once, no matter how many times `zx_vcpu_kick` is called.

`ZX_ERR_CANCELED` was chosen as the status to return, so that the cause of exit
would be easily distinguishable to a virtual machine manager. This allows a
virtual machine manager to use that status to gracefully stop a VCPU without any
additional state management within the virtual machine manager. When it sees
that `zx_vcpu_enter` has returned `ZX_ERR_CANCELED`, it can close the VCPU
handle and free any other associated resources.

Furthermore, the behaviour of `zx_vcpu_kick` is to stop the VCPU, but not to
terminate it. This means that a virtual machine manager may resume execution of
the VCPU after `zx_vcpu_enter` returns `ZX_ERR_CANCELED` by calling
`zx_vcpu_enter` again. Other than handling the return value and bypassing any
packet handling, the virtual machine manager does not need to do anything
special and can immediately call `zx_vcpu_enter` to resume execution.

## Implementation

Within the hypervisor, `zx_vcpu_enter` is modelled after `zx_port_wait`. It has
an almost identical API, with the exception of a deadline. It is meant to wait
for a guest packet to arrive, and then return to user-space with the packet.
Unlike `zx_thread_start`, it is not meant to be a way to create a new thread of
execution, rather to transform the current thread of execution.

Having said that, we do not believe that a token based approach — such as that
found in `zx_task_suspend` — would work well to interrupt execution of a VCPU. A
token based approach does not fit the model of VCPU execution, which requires a
constant back and forth between user-space and the kernel.

Instead, we propose that `zx_vcpu_kick` simply cause `zx_vcpu_enter` to return
to user-space with an error code suggesting that it was interrupted. Within the
hypervisor, `zx_vcpu_kick` will be implemented very similarly to
`zx_vcpu_interrupt`. It will:

  1. Set state. For `zx_vcpu_kick`, this state is an atomic bool used to
     indicate that the VCPU should return when it exits the guest. This variable
     can be queried through the use of `zx_object_get_info` with a topic of
     `ZX_INFO_VCPU` and the corresponding `zx_info_vcpu_t` type.
  2. Check if the VCPU is currently running, and if so, it IPIs the physical CPU
     that the VCPU is currently running on. This forces the VCPU to exit the
     guest to service the interrupt, and thereby allows it to service the
     request to return.

The atomic bool tracks whether `zx_vcpu_kick` has been called and that the VCPU
should return `ZX_ERR_CANCELED` when it no longer has any outstanding
information to return to user-space. This means that if there is an outstanding
guest packet for user-space, it will be returned successfully before we return
`ZX_ERR_CANCELED` on a subsequent call to `zx_vcpu_enter`.

This approach makes `zx_vcpu_enter` behave similarly to a `zx_port_wait` that
encountered a timeout.

The implementation will be carried out in a single CL, as all known users of the
hypervisor are contained within the Fuchsia repository. This change will include
updates to documentation and language bindings for syscalls, as well as changes
to the hypervisor and virtual machine manager.

## Performance

There is no known impact on performance, other than to cause a guest to exit
from execution. As the primary use case for this is to gracefully terminate
execution of a VCPU, this should have no practical impact.

## Ergonomics

Ergonomic considerations for `zx_vcpu_enter` and `zx_vcpu_kick` were discussed
in the [design](#design) section.

## Backwards Compatibility

All known users of the hypervisor are contained within the Fuchsia repository,
therefore it is feasible to both introduce `zx_vcpu_kick` and to rename
`zx_vcpu_resume` to `zx_vcpu_enter`.

## Security considerations

A virtual machine manager using `zx_vcpu_kick` should be audited to ensure that
any use of `zx_vcpu_enter` takes into account the `ZX_ERR_CANCELED` return
status, and that it must ignore the returned `PortPacket` if it intends to
resume operation of the VCPU. If it does not ignore the `PortPacket`, it will
operate on invalid data that has been zeroed — which is true for any error
status returned by `zx_vcpu_enter`.

## Privacy considerations

This proposal has no impact on privacy.

## Testing

Once the implementation CL has landed, we can enable the ASAN bots for the
virtual machine manager tests, and prove that we can shutdown cleanly without
ASAN failures.

## Documentation

We will need to add additional documentation for the `zx_vcpu_kick` syscall, and
we will need to expand the documentation for the `zx_vcpu_enter` syscall to
include the new return status and `PortPacket` handling recommendations.

## Drawbacks, alternatives, and unknowns

The drawback of this proposal is the introduction of an additional syscall. It
was originally proposed that we could reuse either `zx_task_kill` or
`zx_handle_close`, but both come with their own drawbacks.

To specialise `zx_task_kill` to work with VCPUs, we would have to consider a
VCPU to be a task, and therefore make all task-related syscalls work with VCPUs.
If we only made `zx_task_kill` work with VCPUs and none of the other
task-related syscalls, it would be incongruous.

Instead, we could rely on the semantics of `zx_handle_close` and add an
`on_zero_handles` handler to the `VcpuDispatcher`. This would allow us to
terminate the VCPU if the handle count dropped to zero. However, this has two
immediate drawbacks: we can no longer resume the VCPU after it has been stopped,
and we invalidate the VCPU handle. Invalidation of the VCPU handle is
particularly problematic, as it is an inherently racey operation. Another thread
may be in the process of using the VCPU handle to inject and interrupt when the
handle is invalidated, and therefore cause unintended errors to occur.

Another option would be to introduce a more generic version of this syscall, for
example `zx_thread_cancel`, where any blocking operation on a given thread
immediately returns `ZX_ERR_CANCELED`. This would allow us to extend the syscall
to future use cases that use a similar model to VCPUs.

## Prior art and references

A similar operation is present in Apple's Hypervisor Framework:
[https://developer.apple.com/documentation/hypervisor/1441468-hv_vcpu_interrupt](https://developer.apple.com/documentation/hypervisor/1441468-hv_vcpu_interrupt)

As well as in Linux's KVM (Kernel-based Virtual Machine):
[https://www.kernel.org/doc/html/latest/virt/kvm/vcpu-requests.html#vcpu-kicks](https://www.kernel.org/doc/html/latest/virt/kvm/vcpu-requests.html#vcpu-kicks)
