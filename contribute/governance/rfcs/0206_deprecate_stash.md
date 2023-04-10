<!-- Generated with `fx rfc` -->
<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0206" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
{# Fuchsia RFCs use templates to display various fields from _rfcs.yaml. View the #}
{# fully rendered RFCs at https://fuchsia.dev/fuchsia-src/contribute/governance/rfcs #}
<!-- mdformat on -->

## Summary

The stash service is unmaintained and the current implementation does not
provide the properties it was initially created to deliver. This RFC proposes
deprecating stash. We will migrate the clients using stash to Fuchsia's standard
persistent storage capabilities then delete the three components serving the
stash service.

## Motivation

Stash is a simple persistence service originally created for use by platform
components that need to access persistent mutable state early in the boot
process, generally prior to software update.

As discussed in more detail below, three different variants of the stash
protocol exist. All variants use the same [underlying FIDL protocol][fidl_stash]
and are implemented using the same [binary][src_stash_src]. This document uses
the word "stash" to cover all of these variants.

Each time Fuchsia reads persistent mutable state prior to software update is
potentially an opportunity for an attacker to persist their access: If an
attacker had gained full control of a device in a previous power cycle we must
assume they were able to write arbitrary data into the device's persistent
mutable state. A vulnerability in a code path that reads mutable data would
potentially let this attacker-controlled data exploit the system in the next
power cycle. When the read occurs early in the boot process the consequences can
be even more severe; potentially the attacker can prevent a software update from
occurring and therefore prevent the vulnerability from being patched.

Stash was intended to reduce the risk of this scenario by providing a simple and
secure persistent mutable storage for use in early boot. Stash was intended to
have three properties:

1. **Minimal** - Stash should use a significantly smaller and simpler codebase
  than the full storage stack, making the implementation easier to review and
  reducing the risk of bugs.
1. **Easy to use** - Stash should be easy for clients to use, reducing the risk
  of bugs in their integration with persistent mutable state.
1. **Secure** - Since stash exists to improve the security of the system, its
  design and implementation should not introduce new security problems.

Currently stash is failing to provide most of these properties:

1. **Minimal** - Stash is implemented as a FIDL server, on top of a
  serialization layer, on top of the standard filesystem (either fxfs or minfs
  on top of zxcrypt). Stash is no simpler than the standard storage stack. In
  theory it would be possible to migrate to a simpler storage implementation in
  the future without changing the FIDL interface, but there are no plans to do
  this.
1. **Easy to use** - Stash is mainly meeting this objective. Its FIDL interface
  exposes a simple key-value paradigm with a limited number of data types. This
  keeps client code generally simple and reduces the likelihood of bugs in
  client code. The need to handle backwards compatibility, transactional writes,
  and FIDL errors does introduce some complexity and in cases clients have still
  needed to write their own helper libraries, e.g. [wlan][src_wlan_helper].
1. **Secure** - The design of stash assumed component framework would provide
  client identity, letting a FIDL server identify its clients. Component
  framework does not provide client identity and there are no plans to introduce
  it in the foreseeable future. This means different stash clients can read and
  write each other's data and the protocol relies on an honor system to avoid
  this.

Although stash was initially intended to support only "early boot" components
there is no formal definition of an early boot phase on Fuchsia or list of which
components are considered "early boot". Several clients of stash do not start
early in the boot process and none of the existing clients start during
bootstrap.

Since 2020, Fuchsia has managed a complex system of duplication and isolation to
work around the security problem noted above (see
[fxbug.dev/47602][bug_stash_isolation]).

- Three different FIDL stash protocols have been defined: `fuchsia.stash.Store`,
`fuchsia.stash.Store2`, and `fuchsia.stash.SecureStore`.
- Three different stash components exist, each running as a separate process
  and serving a separate protocol.
- Stash clients are carefully assigned to one of the three protocols and the
  clients sharing a channel are assessed to ensure the risk of them reading or
  writing each other's data is acceptable.
- A BUILD visibility list exists to prevent the addition of new stash clients
  without security review.

This situation has led to an ongoing confusion and engineering cost while its
performance and security properties are barely adequate.

## Stakeholders

_Facilitator:_

- hjfreyer

_Reviewers:_

- atait (DHCP)
- brunodalbo (Netstack)
- ecstone (Migration)
- emircan (Scenic)
- jamuraa (Bluetooth)
- nmccracken (WLAN)
- palmer (Security)
- senj (Omaha Client)
- paulfaria (Settings service)

_Consulted:_

- silberst, wittrock, shayba, cgonyeo, erahm, mnck, jfsulliv

_Socialization:_

An early draft of this RFC was developed in collaboration with stakeholders from
all impacted clients.

## Design

We intend to deprecate stash and migrate most existing clients to the ["data"
storage capability][docs_data] used by other components.  Storage capabilities
are accessed using a standard file system API. For most components this
migration will involve using a serialization library to convert the component's
persistent data structures to byte streams that the component then writes to
disk. Reading persistent data will involve reading files from disk then using
the same library to deserialize and populate the component's persistent data
structures.

Scenic is not using stash for the intended purpose (ref
[fxbug.dev/91585][bug_scenic]) and we will migrate their use case to [structured
config developer overrides][rfc_developer_overrides].

It is always desirable to minimize the attack surface of components that start
early in boot and can impact the success of a software update. The networking
and SWD components that currently rely on stash should be prudent in their use
of the data storage capability but we will not maintain a separate set of
storage access requirements for these components. The best practices for
securely persisting data to a storage capability include:

- Minimize the amount of data that is persisted.
- Use narrow and precisely defined data types.
- Use a security-reviewed serialization library to pack and unpack data.

This design can be assessed against the desirable stash properties as follows:

1. **Minimal** - The overall complexity is similar to the status quo:
   a serialization library is still used (although usage moves from a common
   location to each client). Fxfs (or minfs and zxcrypt) is still present. It
   would still be possible to back the storage capability by a simpler
   filesystem in the future although doing so may require changes in the
   clients.
1. **Easy to use** - The ease of use is similar to the status quo: clients must
   interact with a serialization library and perform basic filesystem operations
   instead of managing FIDL connections, transactions, and failures. There are
   several existing implementations of persisting Fuchsia component state using
   a storage capability and these implementations could be reused.
1. **Security** - Security is improved: The design guarantees isolation between
   components. We use existing Fuchsia technologies that have already passed
   security review and are already used in production.

In addition, the design improves several other properties:

- Resource utilization is lower since we remove three component instances
- It is easier to attribute disk utilization to the component using it

## Implementation

If this proposal is accepted we will clearly document the stash component and
all stash protocols as deprecated.

We will then work with each of the seven impacted client components to agree a
plan and approximate timeline for migration. In some cases the team is already
planning to migrate away from stash
(e.g. [fxbug.dev/91403][bug_setui_migration]), in other cases the trusted
platform services team could help with the migration. This RFC does not specify
a deadline for completing the migration.

Components that store critical data in stash will require a stepping stone
release to complete their migration (i.e. a software release that every device
must pass through during its software upgrade process). At this stepping stone
release the component would be able to read its persistent state from either
stash or the storage capability but would write to the storage capability.
Passing through this stepping stone would ensure data is migrated to the storage
capability before the component removes its code to read stash.

As we work with stash clients to plan their migrations we aim to minimize the
number of platform stepping stone releases that are required.

Once all clients of a particular stash protocol have completed their migration,
that protocol and the instance of stash serving it will be deleted. The stash
binary will be deleted along with the last protocol.

## Performance

This proposal will reduce disk, memory, and CPU utilization by deleting three
component instances.

## Security considerations

This proposal increases the security of Fuchsia by guaranteeing that early boot
components can no longer read and write each other's persistent state and by
eliminating code that is currently not maintained.

## Privacy considerations

This proposal does not alter the set of user data that is collected and stored.
There is a small improvement in privacy because a compromised early boot
component could no longer read PII data stored by a different component.

## Testing

Existing end to end tests and integration tests cover the storage and retrieval
of persistent mutable state using stash. These same tests will cover the
storage and retrieval of this state using the basic storage capability. Each
stash client should add integration tests to validate migration of data from
stash to the storage capability.

## Documentation

Existing stash documentation will be deleted once migration is complete.

## Alternatives Considered

### Alternative 1: Use a new "basic" storage capability

The current proposal recommends using the existing "data" storage capability. A
similar solution would have been to create a new "basic" storage exclusively for
use by early boot components.

Using a separate storage capability lets us track components that should
eventually use a simpler storage solution. Components that use the "basic"
storage capability follow a set of best practices intended to reduce complexity,
reduce the risk of bugs, and simplify migration to a future replacement backend.
The best practices may include:

- Use an approved serialization library to pack and unpack data. For example,
  serde for Rust components
- Update the content of files atomically. For example, by writing into a temp
  file then renaming the temp file to the final path
- Do not create subdirectories
- Do not create files larger than X kB
- Do not create more than Y files

Many of these best practices feed into simplifications that we could make in a
replacement filesystem to back the "basic" storage capability. For example, it
would be easier to migrate to a trivial filesystem that did not support
directories if the client did not use directories.

The security team maintains a list of which components are considered "early
boot". Automated tooling verifies that these components only use the "basic"
storage capability and, where practical, verifies that their usage of the file
system is consistent with best practices.

The initial implementation for this solution would be simple: "basic" could be
backed by a new subdirectory on the existing "data" fxfs or minfs partition.
However, there would be significant process and tooling cost to maintain a
consistent definition of "early boot" and build the tooling to enforce data
persistence patterns in these early boot components.

Fuchsia does not plan to implement a simpler filesystem to back a "basic"
storage capability. Without a different filesystem there would be very little
benefit in spending resources to maintain two different sets of clients backed
by the same implementation, hence this solution was not selected.

### Alternative 2: Write a dedicated client library for early mutable state

The current proposal recommends using existing mature libraries to serialize
mutable persistent state. An alternative would be to write a new client library
in each of the target languages (currently Rust and Go, potentially C++ at some
point in the future).

A dedicated client library could easily enforce best practices and could be both
smaller and easier to use than the existing general purpose serialization
libraries like serde. However, designing and implementing these new libraries
would significantly increase the engineering cost of this proposal. The existing
clients use stash for different purposes with different wrappers so a single
client library may not meet the expectations of all these clients. Stash
is currently unstaffed and it is likely that committing to designing and
implementation new client libraries would have delayed migration by several
quarters.

## Prior art and references

- [Stash source code][src_stash_dir]
- [`data` storage cabability][docs_data]

[bug_stash_isolation]: https://fxbug.dev/47602
[bug_scenic]: https://fxbug.dev/91585
[bug_setui_migration]: https://fxbug.dev/91403

[rfc_developer_overrides]: https://fxrev.dev/755286

[fidl_stash]: https://cs.opensource.google/fuchsia/fuchsia/+/main:sdk/fidl/fuchsia.stash/stash.fidl

[docs_data]: /docs/concepts/components/v2/capabilities/storage.md#data

[src_stash_dir]: https://cs.opensource.google/fuchsia/fuchsia/+/main:src/sys/stash/
[src_stash_src]: https://cs.opensource.google/fuchsia/fuchsia/+/main:src/sys/stash/src/
[src_wlan_helper]: https://cs.opensource.google/fuchsia/fuchsia/+/main:src/connectivity/wlan/lib/stash/src/lib.rs