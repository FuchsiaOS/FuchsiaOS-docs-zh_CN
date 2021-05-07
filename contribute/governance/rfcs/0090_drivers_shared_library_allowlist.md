{% set rfcid = "RFC-0090" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }} - {{ rfc.title }}
<!-- *** DO NOT EDIT ABOVE THIS LINE -->

## Summary

This RFC proposes that the Driver Framework adds an explicit allowlist of shared libraries that
drivers are allowed to depend on. This allowlist will be checked at build time in the Fuchsia tree,
and at run-time in the Driver Manager. Drivers that depend on shared libraries that are not
in the allowlist will not be run.

This RFC is formalizing a longstanding informal agreement between driver authors and the
Driver Framework. Driver authors are currently aware that they should not depend on extraneous
shared libraries, but there is no explicit shared library allowlist in place today.

## Problem Statement and Motivation

Drivers in Fuchsia are implemented as shared libraries. Drivers are loaded into a Driver Host.
A Driver Host represents a single process that can hold multiple drivers. A driver itself can
rely on other shared libraries, which also need to be loaded into the Driver Host.

Because drivers and their shared libraries all exist in the same process, the shared libraries
can conflict in subtle and undefined ways. A driver that is built out-of-tree may attempt to link
against a newer or older version of the same shared library that another driver is using. Multiple
implementations of the same symbol in two shared libraries makes symbol resolution
non-deterministic. Additionally, global variables in shared libraries can be seen as unanticipated
shared state between drivers in the same Driver Host. Without an allowlist, drivers may start to
communicate through an unsupported shared library. The Driver Framework needs some way to stop
drivers from using unsupported shared libraries for both security and correctness reasons.

The solution proposed by this RFC is to have a build-time and run-time shared library allowlist
for drivers. Drivers will not be able to link against shared libraries that are not on the
allowlist. All shared libraries on this allowlist will be provided by the Fuchsia platform instead
of being provided by individual drivers.

The long-term solution to this problem is to have a namespaced dynamic linker. With such a linker
each driver would have its own “namespace” inside of a process, and the driver’s shared libraries
would not conflict. If the Driver Framework has a namespaced dynamic linker, then it no longer needs
the shared library allowlist.

## Why Now?

The Driver Framework is implementing two new features that require a strict allowlist: out of tree
drivers and driver packages. Both features load drivers from new locations, which means it becomes
ambiguous for how the system will handle a driver's shared libraries. Having an explicit policy
that a driver's shared libraries will be loaded from the Fuchsia platform reduces system ambiguity
and ensures that each driver gets the same version of a shared library at runtime. Having an allowlist
of shared libraries is a pre-requisite to keeping all drivers' shared libraries in the Fuchsia
image.

## Design

The Driver Framework will maintain a list of shared libraries that drivers are allowed to link
against. The initial contents of this list will be all of the shared libraries that drivers
currently link against. Adding or removing libraries to and from the list is possible but requires
permission from the Driver Framework team.

All shared libraries on this allowlist will be provided by the Fuchsia platform. Many of these
shared libraries will exist as bootfs items in the ZBI at /boot/lib/ so they can be loaded by
drivers at early boot. The loader service for drivers will only load shared libraries from the
allowlist and it will not load libraries provided by individual drivers.

Drivers will not be able to link against shared libraries that are not on the
allowlist. The Driver Framework will create a build-time error if a driver attempts to link against
a shared library that is not on the list. Driver Manager will log run-time errors if a loaded
driver attempts to access a shared library that is not on the list.

### Criteria considered for new shared libraries

The Driver Framework team will have the ability to add new shared libraries to the allowlist as
the team sees fit. The following item will have to be taken into consideration for any new
shared library:

* Size of the library. If the shared library is going to be stored in the ZBI there must be space.
  This is less important if the library will live in storage.

## Implementation

The allowlist will be generated from the list of shared libraries currently being used by drivers.
Then the mechanism for checking the allowlist at build time will be implemented. After that, the
mechanism for checking the allowlist at run time will be implemented. No driver changes
will be required because the initial list will hold all shared libraries that drivers currently use.

The run-time implementation can be done by using a custom fuchsia.ldsvc.Loader implementation in
the Driver Framework. This implementation will not need architectural changes to other areas of the
codebase.

## Performance

This will have no impact on performance.

## Security considerations

This proposal has a positive impact on security. It makes drivers' shared library use more
auditable and it reduces the chances of undefined behavior because of shared library conflicts.

## Privacy considerations

This will likely have a positive impact on privacy because it reduces the surface of shared state
between drivers.

## Testing

The build time check will be implemented first, which means the main test is that the tree builds
correctly. There are currently no out of tree drivers, so if the tree builds correctly then all
drivers conform to the shared library allowlist.

Once the build time check is implemented, the run time check will be implemented in anticipation
of supporting future out of tree drivers. This will have integration tests.

## Documentation

This RFC serves as documentation of our decision to create the allowlist.

Details about the allowlist will be added to the driver tutorials on the fuchsia.dev website.
This will include a process for requesting and approving changes to the allowlist.

## Drawbacks, alternatives, and unknowns

One drawback is that this allowlist is not needed if we had a namespaced dynamic linker.
When that linker exists, we will be able to re-assess and possibly remove the allowlist.

The allowlist limits the number of shared libraries that a driver can link against, but it may
not go far enough towards making sure the shared libraries are the same version. Out of tree drivers
may attempt to build against a newer version of a shared library that is on the allowlist, but then
fail at runtime when the system uses the older version of the library from bootfs. Ensuring that
out of tree drivers build against the same version may be necessary followup work after the
allowlist is implemented.

Another drawback is that driver owners may respond to the shared library allowlist by statically
linking their libraries. Using statically linked libraries will increase the code size of drivers.
This is an unfortunate but necessary side effect to this approach. As mentioned elsewhere, the
long term solution to this problem is a namespaced dynamic linker, which will allow drivers to
use shared library without an allowlist.

## Prior art and references

The Zircon build originally had a shared library allowlist for drivers. This allowlist was removed
during the Build Unification to simplify the unification process. It was always intended to be
re-implemented, but it was not a priority until now.

The Driver Manager currently has a runtime implementation for a shared library allowlist that has
never been turned on. It is currently enabled with the kernel commandline option
`devmgr.devhost.strict-linking`. As part of the effort to implement the allowlist, this
implementation will be updated.
