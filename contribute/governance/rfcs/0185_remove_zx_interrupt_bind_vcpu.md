<!-- Generated with `fx rfc` -->
<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0185" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
{# Fuchsia RFCs use templates to display various fields from _rfcs.yaml. View the #}
{# fully rendered RFCs at https://fuchsia.dev/fuchsia-src/contribute/governance/rfcs #}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Summary

Remove the `zx_interrupt_bind_vcpu` syscall, which is no longer required. The
use case it was built for is no longer one that we want to support. Removing
this syscall will allow us to simplify logic within the kernel and remove
support for guest control of interrupts in the hypervisor.

## Motivation

We originally introduced `zx_interrupt_bind_vcpu` to allow us to bind a physical
interrupt to a VCPU, such that the host would split and end-of-interrupt into
two parts: a priority drop, and a deactivation. The host would continue to issue
a priority drop, but the guest could then signal the interrupt deactiviation.

The goal was to allow a guest to control a physical interrupt for a physical
device, in addition to virtual devices. However, this is something that we no
longer require, and removal of the syscall and supporting infrastructure will
allow us to simplify code and reduce ongoing maintenance.

We have already removed all uses of this syscall from user-space code, and the
only remaining uses are in kernel unit tests.

## Stakeholders

_Facilitator:_

* leannogasawara@google.com

_Reviewers:_

* maniscalco@google.com
* tjdetwiler@google.com
* travisg@google.com
* zarvox@google.com

## Implementation

To implement this, we will make the following changes:

1. Remove C++ bindings from `lib/zx/interrupt.h`.
1. Remove unit tests from `core/interrupt/interrupt-test.cc`.
1. Remove the syscall from `vdso/interrupt.fidl`.
1. Remove logic in the `InterruptDispatcher` and `InterruptEventDispatcher`.
1. Reconfigure GICv2 and GICv3 to disable EOI-mode, and enable one-shot
   end-of-interrupts.
1. Remove all support for physical interrupts from the ARM64 hypervisor.

These changes can be made in a handful of CLs.

## Performance

This proposal may have a very minor impact on interrupt processing, where we can
issue an end-of-interrupt in a single-shot operation. This means we do not have
to deactivate an interrupt, separately from dropping priority.

## Security considerations

This proposal has no impact on security. Arguably, removing this syscall may
improve security, as it reduces the attack surface of the kernel by removing a
syscall that is otherwise not scrutinised.

## Privacy considerations

This proposal has no impact on privacy.

## Testing

This proposal will require we remove all testing related to
`zx_interrupt_bind_vcpu` from the tree, which at present is entirely contained
within kernel unit tests.

Additionally, we should make the change to EOI-mode in a separately CL, so that
in the unlikely event of a breakage, we can quickly revert the CL.

## Documentation

We will need to remove documentation related to `zx_interrupt_bind_vcpu`. At
present, this is contained entirely within the syscall definitions.

## Drawbacks, alternatives, and unknowns

One drawback is if we ever need device pass-through in the future, we may have
to implement this or something similar. However, it's often better to prefer
para-virtualized devices and avoid the complexity and maintenance costs
associated with device pass-through.

## Prior art and references

N/A
