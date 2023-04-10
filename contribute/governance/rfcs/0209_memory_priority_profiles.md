<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0209" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
{# Fuchsia RFCs use templates to display various fields from _rfcs.yaml. View the #}
{# fully rendered RFCs at https://fuchsia.dev/fuchsia-src/contribute/governance/rfcs #}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Summary

Use the [profile object](/reference/kernel_objects/profile.md) to instruct
the kernel to minimize page faults and other operations that may increase the
latency of memory accesses by allowing profiles to be applied to VMARs.

## Motivation

Zircon is an over commit system and employs different reclamation systems to
attempt to support these applications. These reclamation methods include
eviction, page table reclamation, zero page scanning, etc and can be at odds
with deadline tasks, as they can add unacceptable amounts of latency to memory
accesses. Potential future reclamation methods, such as page compression, will
also be problematic.

Audio has been a repeated example where any reclamation by the kernel, which
leads to subsequent page faults in the audio threads, can cause audio threads
to miss their deadlines.

The solution up to this point has been special case workarounds in the kernel,
to disable all kinds of reclamation, just for audio related components. However,
this approach is very fragile and does not scale to other users with similar
requirements.

This RFC aims to provide a general mechanism suitable both for audio, to replace
the existing temporary workarounds, as well as any other component.

## Stakeholders

_Facilitator:_

jamesr@google.com

_Reviewers:_

eieio@google.com, rashaeqbal@google.com, tombergan@google.com

_Consulted:_

andresoportus@google.com

_Socialization:_

This problem and proposal has been discussed in docs with the Zircon team and at
the Kernel Evolution Working Group (KEWG).

## API design

To provide control of reclamation to the user the profile object will be
extended in two ways:

1. An additional flag `ZX_PROFILE_INFO_FLAG_MEMORY_PRIORITY` for
`zx_profile_create` will be added, along with a `memory_priority` field.
2. It will be allowable to apply a profile, via `zx_object_set_profile`, to
VMARs.

Initially only two values of the `memory_priority` field will be supported:
`ZX_PRIORITY_DEFAULT` and `ZX_PRIORITY_HIGH`, with high acting to disable all
reclamation permanently. Room is left to support additional levels in the future
to allow for trade offs between minimizing latency and preventing OOMs.

When a profile is applied to a VMAR, assuming that profile has a valid
`memory_priority`, it is immediately applied to that VMAR and all its
subregions, overriding any previously applied profiles.

It is a requirement that the kernel will respect the `ZX_PRIORITY_HIGH`
setting and the kernel MUST disable any dynamic reclamation in the marked VMARs.

The `ZX_PRIORITY_DEFAULT` setting has no particular meaning, and does not have
to be respected. In particular the kernel is allowed, primarily for ease of
implementation, to extend `ZX_PRIORITY_HIGH` requests to wider ranges that
are marked `ZX_PRIORITY_DEFAULT`.

Although over application is always allowed, if an address space goes from
having some `ZX_PRIORITY_HIGH` VMARs to have none, then the kernel SHOULD
return to the same state equivalent to if no profiles had ever been applied.

### Information queries

The `ZX_INFO_KMEM_STATS_EXTENDED` topic of `zx_object_get_info` will be
extended to report an additional field:

```
    // The amount of memory in VMOs that would otherwise be tracked for
    // reclamation, but has had reclamation disabled.
    uint64_t vmo_reclaim_disabled_bytes;
```

### Component usage

User-space components do not typically use `zx_profile_create` etc directly,
rather they invoke a `ProfileProvider` service. Here the `SetProfileByRole`
method of the `ProfileProvider` will be relaxed to accept arbitrary handles,
instead of just threads.

## Kernel design

This section describes how the objects in the kernel would be changed to
accommodate the information from the profile. The goal being to ensure that any
part of the system that might need to know the memory priority in order to make
a decision has efficient access to it. Where possible this design favors doing
work at the point of profile application under the assumption that profile
applications will be infrequent compared to other operations.

### Reduction to boolean

The kernel objects involved in reclamation are:

* `VmAspace` - Controls page table mappings and page table reclamation goes
               through here.
* `VmAddressRegion` - Not currently involved in reclamation, but the creation of
                      all page table mappings happens through this object.
* `VmObject` - Any eviction, or future page reclamation strategies, go through
               the `VmObject`.

Aside from the `VmAddressRegion`, which is the object the profile is applied to,
each of these objects can query, for any of their sub ranges, what VMARs exist
in that range and the applied memory priority.

Having multiple VMARs, with different priorities, apply to a VMO region can be
resolved by taking the highest applied priority.

To avoid repeated long running searches across all VMARs, objects need an
efficient way to know if any memory priority apply to them. For simplicity of
tracking, the initial proposed implementation will upgrade any priority range to
the full object. That is, if any part of a `VmObject` is referenced by a
profile, the entire object will be considered to have that profile.

These two implementation simplifications make efficient memory profile querying
just require propagating the union of booleans through object links.

### Propagation

Propagation of this reclamation disable boolean is based around edge transitions
and counting.

When a VMAR has a profile set there are three outcomes to consider:

1. Reclamation of this VMAR stays the same.
2. Reclamation transitions from disabled to enabled.
3. Reclamation transitions from enabled to disabled.

In the first case no direct propagation happens, but all subregions must still
be traversed and have the profile applied to them. This unconditional traversal
needs to occur as a subregion have had a different profile applied that needs
to be overridden.

In either of the transitions both of the potential reference objects, `VmAspace`
and `VmObject`, need to be updated.

Instead of a single boolean the `VmAspace` and `VmObject` objects will have a
counter of how many objects that refer to them have reclamation disabled.
Determining whether reclamation is disabled for these objects is therefore a
matter of comparing the count with zero.

A `VmObject` may have additional `VmObject` parents that it needs to propagate
the flag to. Since reclamation is controlled by the counter being, or not being
zero, when the counter transitions to or from zero the propagation occurs.

This reference counting propagation strategy ensures that profile changes are as
efficient as possible, with almost no overhead compared to just tracking a
boolean.

### VmObject transitions

In addition to propagating the reclamation flag, `VmObject` needs to perform
actions as part of a transition to update its pages.

When reclamation is disabled, all pages that would otherwise be reclaimable will
be moved to a separate page queue. Being in this queue will both prevent
reclamation, and provide a way to count the pages.

Similarly, when reclamation is enabled, pages need to be moved back to their
default queues so that they become reclamation candidates again. It is an
implementation detail what the age of these pages are considered to be when
placed back into their default queues. On platforms without a hardware accessed
flag it will not be possible to have any age information and so an age will have
to be invented.

## Implementation

The implementation will be landed in a series of steps from the kernel
implementation through successive layers of APIs.

### Kernel implementation

The proposed design of changes to the kernel objects can be fully implemented to
add support for being able to set memory priorities without changing any
behavior. This would be done over multiple CLs and tested with in kernel
unittests.

### Kernel API changes

The `ZX_INFO_KMEM_STATS_EXTENDED` query requires a fairly privileged system
resource and only has a handful of usages, all in tree. Therefore this query can
be modified in a single CL without a multi stage struct evolution.

Update the profile API and documentation and link the profile syscalls to the
previously implemented kernel support. The `zx_profile_create` syscall and its
associated configuration struct, `zx_profile_info_t`, are privileged system
calls and so can similarly be modified in single CLs.

### ProfileProvider changes

Extend the implementation of the `ProfileProvider` to support a way to specify
the memory priority in `.profiles`.

Change the `ProfileProvider` FIDL API to take arbitrary handles instead of just
threads. As this is a relaxation of the API it does not break any backwards
compatibility.

### Media migration

The relevant profiles for media related components will be changed to include a
`memory_priority` of `ZX_PRIORITY_HIGH`, and any media components will then
apply these profiles to their root VMARs in exactly the same way as they apply
them to their threads.

Once the profiles are confirmed to be working the existing hard coded kernel
workarounds can be removed.

## Performance

The proposed kernel design is very close to the temporary workaround used in the
kernel, with the exception that the current temporary method cannot be
un-applied, and is permanent to all involved VMARs and VMOs. Therefore switching
from this method to profiles should be a complete no-op in terms of
functionality and kernel behavior, including CPU and memory usage.

## Security considerations

Usage of profiles, and hence the ability to set memory priorities, is gated by
needing the root job handle. Therefore any security considerations around
denial-of-service attacks are equivalent to existing denial-of-service
possibilities of the `ProfileProvider`.

## Testing

The majority of the testing can be focused into unittests on the respective
kernel and profile provider implementations, with some integration tests to
validate the full usage path.

## Documentation

Documentation around the profile objects, related syscalls and FIDL protocols
will be updated.

## Alternatives

### Property or equivalent on VMARs

Instead of using a profile object, directly setting a property, or equivalent,
on the VMAR object could be used to indicate its priority. This avoids
extensions to the scheduler objects and simplifies user space usage and does not
need involvement of the `ProfileProvider`.

With this approach there is not an inherent way to restrict the ability to set
priorities. Although any component can already allocate arbitrary memory and
perform denial-of-service, this is not necessarily ideal, and disabling
reclamation is potentially tempting as a way to boost performance, potentially
enabling a tragedy of the commons scenario.

Some form of bespoke access control could be designed to solve this problem,
but now the benefits over leveraging profiles has been removed.

### Inference via taint

An alternative to any direct marking by user space is to assume any deadline
thread needs deadline memory accesses and to mark any memory it accesses as high
priority. This requires no changes to user space, but either requires over
marking, with every address space with at least one deadline thread having all
of its mappings marked, or mappings / VMOs are marked once they have been used
touched by a deadline thread.

Over marking is bad as not all deadline threads have the same memory latency
requirements, and not to all of their address spaces. Lazily tainting means
there is no way to pre-fault and so deadline threads may, in the worst case,
always miss their deadlines once to fault items in.

With a lazy marking scheme it is also unclear how items get unmarked.

### Applying memory_priority to thread

Instead of needing to apply profiles to VMARs, the `memory_priority` field could
be interpreted when a profile is applied to a thread, and have the priority
applied to its root VMAR.

Although this simplifies both the `ProfileProvider` protocol, as well as the
syscall interface, it prevents the option for users and the kernel to be more
efficient in the future. An efficient component could organize to have critical
latency sensitive data in one subregion of its address space, and non-critical
data in another region, and just apply a profile to the critical region. This
lets the non-critical data remain a consideration for reclamation, benefiting
memory usage.

### Single priority field

The same `priority` field for thread priority could be reused instead of
introduction the additional `memory_priority` field. This produces a theoretical
saving of having a simpler configuration structure, but now requires different
profile objects to be created if a different memory and scheduler priority are
desired.

### Expanding ALWAYS_NEED hints

There is an existing API for controlling reclamation by using `ALWAYS_NEED` and
`DONT_NEED` hints on a VMO or VMAR. Currently these are only given meaning to
pager backed VMOs, but could be extended to have meaning to anonymous VMOs.

Just extending the semantics to cover anonymous VMOs leaves some gaps:

 * `ALWAYS_NEED` does not guarantee a lack of reclamation, as its a hint.
 * Pages may still need access faults as age is still tracked.
 * Does not disable page table reclamation.
 * Only be applied to existing mappings.

Each of these limitations could be addressed by using an internal implementation
similar to what was described in the main proposal, however the API itself has
two fundamental problems.

The main proposal has a clear way to know when reclamation has been re-enabled
by having a profile applied to all VMARs (or just the root VMAR). With hinting
there is no way to remove `ALWAYS_NEED`, as `DONT_NEED` is stronger than undoing
`ALWAYS_NEED`.

One motivation for the hinting API to be hints, and not promises, is due to a
lack of access control with it being usable on any VMAR or VMO. Job policy or
similar could be used to control what the hints do, but such a mechanism would
also need to be designed.
