{% set rfcid = "RFC-0028" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

Note: Formerly known as [FTP](../deprecated-ftp-process.md)-028.

_"The Right Stuff"_

## Summary

Annotate required or excluded handle rights in FIDL, specifically:

1. Have a mechanism to **describe rights** on all handle kinded types (e.g.
   `Protocol`, `request<Protocol>`, `handle`)

2. On serialization, **enforce rights filtering**, i.e. the description of
   rights determines a mask, which indicates the set of rights one must
   have.

3. On deserialization, **enforce rights validation** and the set of rights
   provided.

## Motivation

Handle rights are a tool for attenuating privilege within Fuchsia. Prior to this
FTP, FIDL had no way to describe handle rights. Several places with important
restrictions or requirements on handle rights currently relegate such a
description to comments on methods.

Examples of desirable rights restrictions include read-only IPC or VMO handles,
and non-executable VMO handles.

Examples of desirable right requirements include writable IPC handles.

## Design

### Rights Constraints

This proposal adds a new type of constraint to FIDL: a rights annotation. This
can be applied to all handle types in FIDL data structures and method
parameters: handles, protocols, and protocol requests.

Rights constraints MAY be specified on typed handle declarations:
`handle<*subtype*, *RC*>`. They cannot be specified on plain handles because
rights only have meaning within the context of a particular handle subtype. When
rights constraints are not specified on handle declarations, rights will be
forwarded with the same rights (no change to existing behavior).

Example:

`handle<vmo, zx.rights.READ | zx.rights.WRITE>` read and write are required, any
other rights are removed when sending to clients

`handle<vmo>` no rights specified, forward existing rights

At least one right MUST be specified for each right constraint (e.g.
`handle<vmo, >` is illegal).

Rights constraints cannot be specified for server and client protocol endpoints
(`myprotocol` and `request<myprotocol>`). These will always have the rights
`zx.TRANSFER | zx.WAIT | zx.INSPECT | zx.WRITE | zx.READ | zx.SIGNAL | zx.SIGNAL_PEER`
and subtype set to channel. If custom rights are needed (for
instance for an unmovable protocol), `handle<*channel*, *RRC, ORC*>` can be
used. (This may be further refined with the use of templates, and will be
discussed in future FTPs).

Rights constraints MUST follow a "default deny" policy. All rights must be
explicitly listed.

### Syntax

Handle rights constraints are bits-typed expressions that can be specified with
the help of bitwise operators. In the examples above, `|` is the bitwise-OR
operator. The syntax is meant to be general and apply to bits-typed values
elsewhere in the FIDL language as well. In the zx library, there will be a bits
definition zx.rights containing all of the rights.

While a rights specification repeating `zx.rights.[*right*]` is verbose, there are
proposals to shorten it in future FTPs so it is considered out of scope of this
FTP.

### Bindings

Bindings MUST respond to reception and deserialization of a handle with
incorrect required rights by destroying the message and closing the channel with
epitaph `ZX_ERR_ACCESS_DENIED`.

Bindings MUST respond to serialization of a handle with incorrect required
rights by failing to serialize, destroying the message, and disallowing sending.
Bindings MUST close the channel as well with `ZX_ERR_BAD_STATE`.

### Defaults

There will be no defaults for handle rights. The reasoning here is the
following:

* Defaults have unclear behavior that can be inconsistent across object types.
  It isn't transparent to the user what the default rights are for a given
  object.

* Defaults discourage use of fine grained handle rights — it becomes easy to not
  consider rights when writing a FIDL definition.

* If there were defaults, for many object types the most suitable candidates
  would be no rights or maximum rights. A maximum right default would limit the
  effectiveness of the changes in this FTP, but a no right default would have
  the same effect as not having defaults. Neither of which are very helpful.

### Reuse of Handle Declarations

A consequence of not having defaults is that specifying rights can be verbose.
In order to improve this, it will be possible to give a name to an entire handle
declaration with the aliasing features (i.e. *using* keyword).

`using readable_vmo = handle<vmo, zx.READ>`

An alternative to this might be allowing aliases for rights constraints (e.g. an
"`io`" alias for `zx.READ | zx.WRITE`), but this provides a layer of indirection
that obscures the rights, especially if they are used broadly. By allowing
aliasing at the object level, it limits usage to locations where the type is
identical.

### Parameterizability

We want to create generic messages containing channels whose rights constraints
can be parametrized.

For instance, consider `fuchsia.mem.Buffer`, which holds a `handle<vmo>`. It should
be possible to say `fuchsia.mem.Buffer:C` with constraints `C` flowing to
constraining the `handle<vmo>`.

Generalized type aliasing is a start here, and the introduction of templates
will further satisfy this need. While this is out-of-scope for this proposal,
this requirement must be considered for related work.

## Implementation Strategy

Reception of messages should rely on the `zx_channel_read_etc` system call to
provide rights information at the point-of-call. The rights information will be
used by bindings to validate required rights are present and filter out any
additional received rights beyond the required rights.

Sending of messages should rely on `zx_channel_write_etc`, which will decrease
sent rights to the set of specified rights and validate that all required rights
are present, returning `ACCESS_DENIED` if validation fails. The bindings will be
responsible for closing the channel after receiving this response. To match
existing behavior, `ZX_HANDLE_OP_MOVE` will be used on this system call, which is
equivalent to calling `zx_handle_replace` and then `zx_channel_write`. When no
rights are specified `ZX_RIGHT_SAME_RIGHTS` will be used in place of the rights.

An intent to implement doc describing the implementation in detail is in
progress.

## Ergonomics

In the long term this is an ergonomics improvement. Rights documentation and
checking is done in a standard way across FIDL, rather than ad hoc comments and
checks.

## Documentation and Examples

This will require documentation changes to:

* [FIDL language specification]

* [Kernel rights documentation] - how rights can be specified

## Backwards Compatibility

### ABI Compatibility

Rights changes MUST NOT break ABI compatibility.

### Source Compatibility

Rights changes MAY break source compatibility. Breaking source compatibility is
unexpected, however and should be clearly documented if binding authors choose
this path.

### Adding Required Rights

It is backwards compatible on the message recipient side to have an additional
required right, as it only gives more capability. However, on the message sender
side, adding a required right is a backwards incompatible change as the right
now needs to be present before sending.

### Removing Required Rights

It is backwards compatible on the sender side to remove required rights, but
backwards incompatible on the recipient side. The recipient will now not be
receiving a right that it expects.

### Adding Optional Rights

It is backwards compatible to add optional rights.

### Removing Optional Rights

It is backwards compatible on the sender side to remove optional rights, but
backwards incompatible on the recipient side. The recipient will now not be
receiving a right that it previously may have received.

### Ambient assumptions

There are likely to be ambient assumptions that the deployment of this model
will break. For example, clients may be assuming that all VMOs received over a
connection are mappable, even though servers do not intend to provide this
guarantee.

One can see this state as the entire motivation for this FTP: to remove this
pervasive and implicit contracts.

## Performance

Microbenchmarks show that `zx_channel_write_etc` and `zx_channel_read_etc` have
very similar performance. For a 64 byte message with 1 handle, a
`zx_channel_write`/`zx_channel_read` take 962ns while
`zx_channel_write_etc`/`zx_channel_read_etc` take 1000ns.

The handles array needed when reading or writing handles will increase from 256
bytes `(ZX_CHANNEL_MAX_MSG_HANDLES * sizeof(zx_handle_t))` to 1024 bytes
`(ZX_CHANNEL_MAX_MSG_HANDLES * sizeof(zx_handle_info_t))`. Similarly, the array
that is stack allocated when writing handles will increase from 256 bytes to
1280 bytes `(ZX_CHANNEL_MAX_MSG_HANDLES * sizeof(zx_handle_disposition))`.

In order to keep stack allocation to a max of 256 bytes, we will need to heap
allocate handle tables if they are too big (> 16 handles for read or > 12
handles for write). As part of this change, we will look at the combined stack
allocation requirements of both message size and handle tables (we only consider
message size today).

## Security

This is a security improvement. It enables more accurate auditing of our API
surfaces. It moves permissions checks into the bindings, which can be better
reviewed than the same checks across all call sites.

For instance, once rights are fully used, it will be easy to audit all places
transferring ownership of executable VMOs. This would prove difficult today.

## Testing

Each bindings implementation should be unit tested.

The roll out of this feature should also ensure that implementations of FIDL
protocols that are modified to use this feature test the new functionality.

## Drawbacks, Alternatives, and Unknowns

#### Drawback: Public APIs Will Be Verbose

In a certain sense this feature is "noisy". In typical execution of the system,
the missing-rights paths may never be taken. And yet they still take up real
estate in the public system API.

We believe that the cost here is worth the clarity and precision, and that good
use of aliases will cover most needs.

Consider for instance two classes of problems that can be avoided or reduced:

1. Breakage at a distance due to incompatible rights: A failure may occur very
   far from the source (possibly even way downstream in some other process if
   the handle was transferred again).

2. Undocumented assumptions cause compatibility issues: Some clients/servers may
   pass handles with more rights than strictly needed leading to their peers
   assuming they can rely on those rights being present leading to compatibility
   issues should these assumptions prove faulty.

We realize that spelling out rights is annoying but equate it to type
information for the capability.

#### Alternative: Lower and Upper Bounds for Rights

Initial design called for lower and upper bounds on rights (e.g. "without
executable right", or "with writable right"). From a security standpoint, the
line of thinking is that Fuchsia should adopt a "default deny" policy for all
capabilities (and rights).

So if a capability (or right) isn't explicitly mentioned anywhere then it
shouldn't be granted. Component manifests already behave this way for sandboxing
and ideally so should FIDL APIs.

We may want to additionally allow expressing explicitly "optional rights", i.e.
rights that may or may not be provided.

Hence, from a constraint syntax standpoint, i.e. what people type when writing
FIDL APIs, we are gearing towards listing rights, and marking some as optional.

Note that from a constraint semantics standpoint, i.e. what would need to be
expressed in the JSON IR and implemented by bindings, this syntactic change
continues to express lower & upper bounds checks.

#### Mandatory Source Compatibility

In Fuchsia, rights are generally represented as a `uint32` and the value of a
right can change at any time. Because of this, it might seem reasonable to
expect changes in rights values in FIDL to not substantively cause changes in
the generated source code. However, there may be some use cases for breaking
source compatibility - such as generating specific methods (such as write()) if
a given right is present (in this case `zx.rights.WRITE`). Because of this, we
aren't prescribing that rights changes can't break source compatibility.

<!-- xrefs -->
[FIDL language specification]: /docs/reference/fidl/language/language.md
[Kernel rights documentation]: /docs/concepts/kernel/rights.md
