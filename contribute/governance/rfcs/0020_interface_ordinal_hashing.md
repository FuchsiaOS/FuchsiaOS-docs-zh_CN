{% set rfcid = "RFC-0020" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

Note: Formerly known as [FTP](../deprecated-ftp-process.md)-020.

_"60% of the time, it's the answer to an interview question"_

## Summary

We propose removing the programmer's ability to manually
specify the ordinal for interface methods [[1]](#Footnote1).
Instead, the compiler generates the ordinal based on a hash of the
fully-qualified method name, i.e. the library name, interface name & method name.
Method renames will be ABI-compatible via a new `Selector` attribute (see
[below](#the-selector-attribute-method-renaming)).

We specifically restrict this FTP to propose ordinal hashing for interfaces only;
not enums, tables nor extensible unions.
We believe the use-cases for those structures are different enough that they need further
investigation and a different FTP.

## Example

Currently, a FIDL author would write:

```fidl
library foo;

interface Science {
    1: Hypothesize();
    2: Investigate();
    3: Explode();
    4: Reproduce();
};
```

This FTP would enable the ordinal indexes to be dropped:

```fidl
interface Science {
    Hypothesize();  // look, no ordinals!
    Investigate();
    Explode();
    Reproduce();
};
```

Under-the-hood, the compiler effectively generates ordinals that look like this:

```fidl
interface Science {
    // ordinal = SHA-256 of the fully-qualified method name,
    // i.e. "foo.Science/MethodName", truncated to 32 bits
    0xf0b6ede8: Hypothesize();
    0x1c50e6df: Investigate();
    0xff408f25: Explode();
    0x0c2a400e: Reproduce();
};
```

## Motivation

- Manually specifying ordinals is largely mechanical.
  It's less work to write an interface if you don't need to think about them.
- If a good hash is used, it's extremely unlikely that hashing will result in ordinals clashing,
  which is an improvement on humans manually writing ordinals (particularly if interface
  inheritance is used).
  See the [Ordinal Clashing section below](#Ordinal Clashing & Conflict Resolution]
  for more information.
- Programmers must currently ensure that ordinals for different methods do not clash.
  This is easy for interfaces with few methods, but if an interface has many
  methods, this can become non-trivial.
  There are different coding styles and schools of thought for ordinal numbering,
  which leads to inconsistent coding style.
    - Most interfaces start at 1 and go upwards.
    - However, some authors prefer grouping different interface methods in ranges
      (e.g., 1-10, 100-110, etc).
    - Removing manually-numbered ordinals also removes this inconsistent style,
      and removes the need for the author to make the decision about which style to use.
- Interface inheritance may lead to unanticipated clashing of ordinals.
  Two attempts have been made so far to solve this:
    - FTP-010 (**rejected**) proposed an `OrdinalRange` attribute so that interface
      inheritance could be more predictable; it was rejected.
    - `FragileBase` [[2]](#Footnote2) is the current stop-gap solution, but doesn't solve the core
      problem of ensuring that ordinals don't clash.
    - If ordinals are hashed and the interface and library name are used to compute
      the hash, hashing ordinals will not result in clashing ordinals, which solves
      the interface inheritance problem (outside of extremely rare hash collisions).

## Design

### Hash

The hashed ordinal is derived by a [SHA-256](https://en.wikipedia.org/wiki/SHA-2) hash of:

    library name (encoded as UTF-8; no trailing \0)
    ".", ASCII 0x2e
    interface name (encoded as UTF-8; no trailing \0)
    "/", ASCII 0x2f
    method name (encoded as UTF-8; no trailing \0)

For example, the following FIDL declaration:

```fidl
library foo;

interface Science {
    Hypothesize();
    Investigate();
    Explode();
    Reproduce();
};
```

will have the following byte patterns used to calculate the ordinal hash:

    foo.Science/Hypothesize
    foo.Science/Investigate
    foo.Science/Explode
    foo.Science/Reproduce

The `.` and `/` separators are used since `fidlc` already outputs fully-qualified
method names in this format (c.f. [`fidlc`'s **NameName()** method](https://fuchsia-review.googlesource.com/c/fuchsia/+/HEAD/tools/fidl/fidlc/include/fidl/names.h)).

Once the SHA-256 hash is computed:

1. The upper 32 bits of the SHA-256 hash are extracted (e.g., `echo -n foo.Science.Hypothesize | shasum -a 256 | head -c8`)
2. The upper bit is set to 0, resulting in an effective 31-bit hash value that's
   zero-padded to 32 bits.
   (31 bits are used since the FIDL wire format reserves the most significant bit in the 32-bit ordinal.)

In pseudo-code:

```c
full_hash = sha256(library_name + "." + interface_name + "/" + method_name)
ordinal = full_hash[0] |
        full_hash[1] << 8 |
        full_hash[2] << 16 |
        full_hash[3] << 24;
ordinal &= 0x7fffffff;
```

###  The Selector Attribute & Method Renaming

We define a `Selector` attribute that will be used by the compiler to compute
the hashed ordinal instead of using the method name.
If a method name does not have the `Selector` attribute, the method name will
be used as the `Selector`.
(The interface and library names are still used in the hash computation.)

`Selector` can be used to rename a method without breaking ABI compatibility,
which was one advantage of manually-specified ordinals.
For example, if we wish to rename the `Investigate` method to `Experiment` in
the `Science` interface, we can write:

```fidl
interface Science {
    [Selector="Investigate"] Experiment();
};
```

We allow the `Selector` attribute on methods only.
Renaming libraries is considered rare, and preserving ABI-compatibility in
this situation is not a high priority.
Similarly for renaming an interface.
Additionally, the interaction of a renamed interface with a `Discoverable`
attribute would be confusing: which name is the discoverable one?

### Ordinal Clashing & Conflict Resolution

If a hashed ordinal results in a clash or conflict with another hashed ordinal
in the same interface, the compiler will emit an error, and rely on a human
to specify a [`Selector`](#the-selector-attribute-method-renaming)
attribute to resolve the conflict [[3]](#Footnote3).

For example, if the method name `Hypothesize` conflicts with the method
name `Investigate`, we could add `Selector` to `Hypothesize` to avoid the conflict:

```fidl
interface Science {
    [Selector="Hypothesize_"] Hypothesize();
    Investigate();  // should no longer conflict with Hypothesize_
};
```

We will update the
[FIDL API rubric](/docs/concepts/api/fidl.md)
to recommend appending "_" to the method name for the `Selector` to resolve clashes.
`fidlc` will also suggest this fix.

Note that ordinals are only required to be unique per-interface, similarly to
manually-specified ordinals.
If we wish ordinals to be unique across all interfaces, that should be
proposed in another FTP.

Back-of-the-envelope calculations show that, with 31 bits and 100 methods on
an interface, the chance of collision is .0003%, so we expect hash collisions
to be extremely rare.

### Selector Bikeshed

There were other suggestions for `Selector`:

- `WireName` (`abarth`)
- `OriginalName` (`ctiller`)
- `Salt` (`abarth`; slightly different since it suggested adding a compiler-specified
  salt instead of an alternate name)
- `OrdinalName`

We chose `Selector` since we believe it more closely reflects the intent of
the attribute than either WireName or OriginalName.

We chose to have the programmer specify the ordinal name, rather
than the ordinal index, for several reasons:

- requiring the index would be more tedious (e.g. copy-and-paste of the raw
  SHA-256 hash value in case of a conflict),
- specifying the ordinal name enables ABI-compatible method renaming, and
- specifying the name instead of the index arguably keeps the abstraction level
  the same in the programmer's head as they write the interface, rather than
  going one level of abstraction lower, requiring them to think about ordinals.

### The Zero Ordinal

Zero is an [invalid ordinal][transactional-messages].
If a method name hashes to zero, the compiler will treat it as a hash conflict
and require the user to specify a `Selector` that does not hash to zero.

We considered having `fidlc` automatically re-hash the name by deterministically
transforming it, but felt that:

- any such algorithm would be non-obvious, and
- the zero case is extremely rare,

and that therefore this approach didn't warrant complicating both the ergonomics and the compiler implementation.

### Events

This FTP also covers events, which are considered a subset of methods by the FIDL language docs
[[4]](#Footnote4).

### Compiler & Bindings Changes

We believe that only `fidlc` needs to be modified to support ordinal hashing;
code generation back-ends do not need to be modified.
This is because `fidlc` computes the ordinals and emits them in the JSON IR to back-ends.

Bindings do not need to change.

## Implementation strategy

We intend to implement this in distinct phases:

1. Add code to fidlc to compute hashes.
2. Add support for attributes to libraries.
3. Broadcast intent-to-change to fuchsia eng so they are aware of potential issues.
    a. Propose that manual ordinals will be deprecated on a certain date, when
       we expect the next step is completed.
4. In the same CL:
    a. Modify the FIDL grammar's interface-method rule to make ordinals optional;
       see below for more details.
    b. Ignore manually-specified ordinals, and use the hashed ordinal for the
       ordinal name passed to code-generation back-ends.
    c. Manually fix any existing hash collisions by adding the `Selector` attribute.
5. Test the changes over two weeks to ensure there's no production problems.
    a. New FIDL interfaces written in this time should not use ordinals.
    b. Manual ordinals are regarded as deprecated, though fidlc will not emit
       warnings about this.
    c. Work with teams to ensure no manually-specified ordinals remain in interfaces.
    c. At the end of the two weeks, update the FIDL formatter to remove ordinals,
       and mass-apply it to the entire Fuchsia tree.
6. Remove support for manually-specified ordinals.

The above is a
[soft transition](/docs/development/source_code/working_across_petals.md#soft-transitions-preferred);
changing `fidlc` to use the hashed ordinals (step 4b) should not break the rollers,
since rollers build from a single version of the entire tree.

In [jeremymanson@google.com's implementation of this FTP](https://fuchsia-review.googlesource.com/c/zircon/+/227623),
he chose to prefer a manually-specified ordinal over a hashed ordinal, which
diverges from step 4b above.
This keeps all existing interfaces that use manually-specified ordinals ABI-compatible,
and only uses hashed ordinals when ordinals aren't specified.

## Ergonomics

Advantages:

- Writing interfaces should be simpler.

Disadvantages:

- Programmers will need to understand a new attribute, `Selector`, that serves
  two purposes: renaming and conflict resolution.
- It may not be apparent that changing a method name breaks ABI compatibility,
  which wasn't the case with programmer-specified ordinals.
  User education (e.g. better documentation) can ameliorate this.
    - Note that other component systems, such as COM and Objective-C, typically
      also break ABI compatibility when interface methods are renamed.
      So, this behavior probably familiar to developers who have used similar systems.
- The loss of manual control over ordinal numbers may result in less debuggability
  in unusual cases where e.g. multiple FIDL interfaces are being used on the same Zircon channel.

Note that the authors originated this FTP largely for ergonomics reasons.

## Documentation and examples

We expect to make changes to the FIDL attributes, grammar, language and wire format docs.
The API readability rubric doc should also be updated, as noted in the
[`Selector` section](#the-selector-attribute-method-renaming).

## Backwards compatibility

- Hashed ordinals are ABI-incompatible with manually specified ordinals, by design.
  We expect this to be a non-issue, since
    - the `fidlc` change is binary (either hashed xor manual ordinals will be used), and
    - `fidlc` is used to build the entire tree, so
    - all parts of the tree will consistently use the chosen ordinal scheme.
- Hashed ordinals are **API (source)-compatible**.
  Existing source files will be kept compatible; manual ordinals will be
  deprecated (see Implementation Strategy).
- Errors will occur if two different builds of fidlc (i.e. two different builds
  of the Platform Source Tree) are used, and FIDL interfaces are used to
  communicate across machines.
  The authors know of no current uses of this, so this should not be an issue.

## Performance

We expect a negligible slowdown to `fidlc`, as it now has to hash all method
names in an interface to compute them.

We expect an insignificant runtime performance impact.
Compilers may have generated jump tables for manually-specified ordinals
that were previously small and contiguous, which will become binary searches
through a sparse ordinal space when hashed ordinals are used.
The same mechanism may also impact binary size in an insignificant fashion.
(Table-driven dispatch will likely ameliorate both the size and speed concerns.)

## Security

We do not expect runtime security issues, since ordinal hashing has no runtime
changes except changing the ordinal values sent over-the-wire.

The use of a cryptographic hash (SHA-256) may lead some to believe the hash needs
to be cryptographically strong; we do not believe there are security issues since:

- the FIDL compiler will check for hash collisions at compile time and require
  human input to resolve them, and
- we use SHA-256 not for cryptographic purposes, but because we'd like a hash
  that is extremely unlikely to result in collisions.
  CRC-32 (or even **strlen()**) would work too, but would probably result in more
  collisions, which would simply be inconvenient.

Truncation of the SHA-256 hash may also concern some, but again, we do not
believe there are security issues since the FIDL compiler statically checks for hash collisions
[[5]](#Footnote5).

## Testing
ianloic@google.com has analyzed existing FIDL interfaces and determined
that there are zero hash collisions.

We'll carefully consider how to test the case of an actual hash collision, since
artificially generating hash collisions with a good hash is difficult (by design).

Otherwise, the typical battery of unit tests, CQ tests,
[compatibility tests](/src/tests/fidl/compatibility/README.md)
and manual testing should suffice to ensure that ordinal hashing is robust.

## Drawbacks, alternatives, and unknowns

This FTP intentionally only addresses ordinal hashing for interfaces.
It does not propose changes to the manually-enumerated ordinals for enums,
tables nor extensible unions.

Perfect hashing was suggested by jeffbrown@google.com, and was considered.
The FTP authors are not very familiar with perfect hashing schemes, but believe
that the addition of extra methods over time would change the hashes of existing
methods and therefore break ABI compatibility, making perfect hashing unsuitable.
Dynamic perfect hashing may be possible, but raises the same question of
changing hashes, and is also less well-known and more complicated than standard
hashing, which doesn't warrant further investigation.

Another approach to removing manual ordinals is to send the full method name
across the wire, which is done in many (most?) other RPC systems (see [References] below).
This has runtime performance implications that arguably conflict with FIDL's intended use-cases.

We considered being able to specify the hash used so it can be changed later,
if SHA-256 ended up having problems that another hash would solve.
This design is common in security applications, where a widely-used cryptographic
hash [will have vulnerabilities](http://valerieaurora.org/hash.html) discovered later.
However, specifying the hash would likely require changes to the wire format,
and require all language bindings to implement code to select hash algorithms,
significantly complicating both compiler and bindings code.
We did not think that trade-off was worthwhile.
We recognize that
[`git` also took this attitude toward SHA-1](https://lwn.net/Articles/715716/),
and is now somewhat back-tracking on the decision, but think we think our use case is
different enough to justify hard-coding the hash algorithm.

## Explorations

- A space-efficient means of identifying a method could lead to an efficient
  first-class representation of a method, making methods first-class.
    - This could, e.g., enable methods to be used as arguments in FIDL calls, or
      have a FIDL method return another method as a result.
      There are arguably existing use cases for this already, where methods return
      an interface with a single method as a proxy for returning an actual method.
- The proposed 31 bits of hashing could be expanded to, e.g., 64/128/53 bits;
  SHA-256 provides lots o'bits.
- Rename `ordinal` to `selector`, which is an existing concept that serves the same
  purpose in other languages and component systems.
- It may be worth distinguishing between method name and interface name, so we
  have the two distinct pieces of data.
  This enables referring to the interface name uniquely, and the method name uniquely.
  We probably need more than 32 bits for this.
- As mentioned above, enums, tables, and extensible unions are out-of-scope.
  That said, we do think this FTP could apply to them.
  Initial thoughts:
    - We're unsure whether enums would want this feature.
      The simpler and standardized consecutive integer numbering seems sufficient.
    - This could probably be applied as-is to extensible unions.
    - Tables would need a different wire format to adopt ordinal hashing, since
      ordinals currently need to be contiguous due to the packed representation.
- FIDL currently reserves the ordinal upper bit, and explicitly says in docs that
  a range of the upper bits is intended for use as control flow etc.
  The authors think that one of the reasons for this may also have to do with
  clashing ordinals.
  Do we want to revisit this?
    - Expanding the ordinal space to 64 bits (mentioned above) will largely solve this.
    - abarth@google.com suggested on the Fuchsia IPC chat room to reserve only `0xFFFFxxxx`.
- We could include the method's argument types in the calculated hash, which
  would support method overloading if we'd like that in the future.
    - jeffbrown@google.com mentions that hashing full method signatures may limit
      opportunities for interface extension, and that overloading maps poorly
      onto many programming languages.
- Since ordinal hashing should resolve ordinals clashing when interface inheritance
  is used, the [FragileBase] attribute could also be removed.
    - Code Search shows ~9 uses of `FragileBase`.
- The authors were concerned that an interface that has evolved significantly
  over time may become hard to read if many methods have `Selector` attributes on them.
    - One approach to solving this is to adopt something similar to
      [Objective-C categories](https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/ProgrammingWithObjectiveC/CustomizingExistingClasses/CustomizingExistingClasses.html#//apple_ref/doc/uid/TP40011210-CH6-SW2)
      or [C# partial classes](https://docs.microsoft.com/en-us/dotnet/csharp/programming-guide/classes-and-structs/partial-classes-and-methods),
      where an already-existing declared interface can be "extended" to have attributes added to it in a separate declaration.

## Prior art and references

Interestingly, we do not know of any other method dispatch or RPC system that
uses a hash of the method name to identify which method to call.

Most RPC systems call methods by name (e.g. gRPC/Protobuf service, Thrift, D-Bus).
For in-process method calls, Objective-C uses a guaranteed-unique char* pointer
value, called a selector, to identify the method that should be called on a class.
The Objective-C runtime can map selectors to stringified method names and vice versa.
For out-of-process method calls, Objective-C distributed objects uses the method
name for invocation.
COM directly uses the C++ vtable for in-process calls, and therefore depends
on ABI and compiler support to support method dispatch.
apang@google.com suggested ordinal hashing for tables in ctiller@google.com's Phickle proposal.
ianloic@google.com and apang@google.com met on Thu 2018/10/18 to whiteboard this.

--------------------------------------------------------------------------------------------

##### Footnote1

Mojo/FIDL1 also didn't require the programmer to specify ordinals;
instead, they were sequentially generated (similarly to FlatBuffers's
implicit tag numbering for table fields).

##### Footnote2

Previously, you could create a FIDL interface that inherited from
whichever other FIDL interface you liked.
However, the interface and the superinterface share the same ordinal space,
which means if you added a method to an interface you might break a
subinterface in some other, far away library.

There are several proposals kicking around FIDL-land for resolving the
inheritance / ordinal collision problem, but until we figured out how we
want to solve this problem, we've switched the default for interfaces to
forbid inheritance.
An interface can still opt in to allowing subinterfaces using the
`[FragileBase]` attribute.

If you run into this issue, the compiler should print out an error message
with a brief explanation.
I (abarth@google.com) have added the `[FragileBase]` attribute
everywhere we use FIDL interface inheritance in the Platform Source Tree
(hopefully!).

Please let me know if you have any questions or run into any trouble.
--abarth@google.com

##### Footnote3

We do not believe that there'll be sufficient ordinal clashes to warrant any
extra implementation and cognitive complexity added by automatic conflict resolution.
We can revisit this decision without breaking backward-compatibility if data
shows that ordinal clashing becomes problematic.

##### Footnote4

If only results are declared, the method is referred to as an event.
It then defines an unsolicited message from the server.

##### Footnote5

jln@google.com writes, "Yes it's ok to truncate SHA-2 and no, it doesn't matter where you truncate."

<!-- xrefs -->
[transactional-messages]: /docs/reference/fidl/language/wire-format/README.md#transactional-messages
