{% set rfcid = "RFC-0029" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

Note: Formerly known as [FTP](../deprecated-ftp-process.md)-029.

## Summary

We propose:

1. To **increase the size of method ordinals to 64 bits** (up from 32 bits),
   while maintaining the range with the highest order bit set for Fuchsia
   reserved usage;
2. Update the **epitaph ordinal to `0xFFFFFFFFFFFFFFFF`** (from `0xFFFFFFFF`);
3. Packing the increased ordinal size by **using the flags field of the
   transactional header**;
4. And **discontinue the use of the `[FragileBase]` attribute**.

## Motivation

#### Combatting Breakage at a Distance {#breakage-at-a-distance}

We've looked for some time at the issue of breakage at a distance, which the
composition model introduces.

In short, when a `Child` protocol composes a `Parent` protocol, it is
possible for the `Parent` to introduce a method after the fact that clashes
with a method defined in the `Child`.
This change is likely made in the `Parent` without awareness of the breakage
caused "at a distance" in the `Child` protocol.

With the ordinal hashing scheme we have today, there is a roughly 2 in 10,000
chances to incur a collision for protocols on the order of 1,000 methods.
This is quite likely given the large number of protocols (and expected
protocols) to be built and expressed in FIDL.
We've resorted to a temporary annotation `[FragileBase]` to indicate this
problem, and make developers aware of this issue.

By increasing the number of bits dedicated to a method ordinal, we can reduce
the probability of collision to a level satisfactory to consider this issue
of breakage at a distance nonexistent (in practical terms).

#### Sizing of Method Ordinals

The [generalized birthday problem][birthday] well describes the probability
of collisions.
We have a `D = 2^k` possibilities ("days") and are looking for the chances
that amongst **N** methods ("people"), two of these methods happen to collide
("same birthday").

If we consider a threshold of 1 in a million chances as being the largest
collision probability we are willing to tolerate, we end up with a maximum number
of methods of:

*   31 bits: ~60
*   39 bits: ~1k
*   47 bits: ~16k
*   52 bits: ~95k
*   63 bits: ~4.3M

Given the above, 47 bits or above are reasonable choices.
We choose 63 bits for a few additional reasons:

*   In practice, it's safe to have ordinals as numbers in JSON with standard
    parsers (e.g., [in Go](https://play.golang.org/p/Rf523ZjyAK8), or in
    Python).
    (In a v2 of the JSON IR, we plan to wrap ordinals as strings.)
*   With room left for control messages, there is room for other flags
    should these need to be allocated later.
    Today, the only existing control message is the epitaph.

We add a high bit for reserved ordinals hence the resulting size for ordinals
being **64 bits**.

We considered limiting to 52 bits (53 bits total with control messages)
because it is the largest positive integer representable in IEEE754, which
makes it advantageous to represent ordinals in languages that only support
doubles (e.g., JavaScript).
However, those languages would still need to manipulate ordinals to place
them on the wire, and therefore would need a further mechanism to access the
individual bytes composing doubles.

## Design

#### Hash Calculation

The hash calculation introduced in [RFC-0020: Interface Ordinal
Hashing](/docs/contribute/governance/rfcs/0020_interface_ordinal_hashing.md) is slightly altered.
It should both produce a 64 bit number, and the string over which the hash is
calculated is `<library>/<top level declaration>.<member>` (per
[RFC-0043: Documentation Comment Format](/docs/contribute/governance/rfcs/0043_documentation_comment_format.md)).

The hashed ordinal is derived by a SHA-256 hash of:

* *library name* &mdash; encoded as UTF-8; no trailing \0
* "/" &mdash; the forward slash character, ASCII `0x2f`
* *protocol name* &mdash; encoded as UTF-8; no trailing \0
* "." &mdash; the period character, ASCII `0x2e`
* **method name** &mdash; encoded as UTF-8; no trailing \0

For example, the following FIDL declaration:

```fidl
library foo;

protocol Science {
    Hypothesize();
    Investigate();
    Explode();
    Reproduce();
};
```

Will have the following byte patterns used to calculate the ordinal hash:

```
foo/Science.Hypothesize
foo/Science.Investigate
foo/Science.Explode
foo/Science.Reproduce
```

Once the SHA-256 hash is computed, the upper 63 bits of the SHA-256 hash are
extracted.
We extract only 63 bits, since the 64th bit is reserved for system usage.

In pseudo-code:

```
full_hash = sha256(library_name + "/" +
                   protocol_name + "." +
                   method_name)
ordinal = full_hash[0]       |
          full_hash[1] << 8  |
          full_hash[2] << 16 |
          full_hash[3] << 24 |
          full_hash[4] << 32 |
          full_hash[5] << 40 |
          full_hash[6] << 48 |
          full_hash[7] << 56;
ordinal &= 0x7fff ffff ffff ffff; // i.e., 63 ones
```

#### Packing Hash in Header

The transactional message header consists of four 4 bytes fields today:
transaction id, reserved, flags, and the current ordinal.
The reserved field is used by the epitaph's error code.
We therefore propose to increase the ordinal fields from 4 bytes to 8 bytes,
by using the flags field (unused today).

The new definition for the header is:

*   `zx_txid_t txid`, transaction ID (32 bits)
    *   `txid`s with the high bit set are reserved for use by [**zx_channel_call()**][channel_call]
    *   `txid`s with the high bit unset are reserved for use by userspace
    *   See [**zx_channel_call()**][channel_call] for more details on `txid` allocation
*   `uint32 reserved0`, reserved for future use.
*   `uint64 ordinal`
    *   The zero ordinal is invalid.
    *   Ordinals above `0xfffffffffffff` are reserved.

#### JSON IR

No change, and given the choice of max 52 bits for developer defined ordinals,
this will be parsable by standard JSON parsers into a 64 bit floating point
number without loss of precision.

#### Other Ordinals

We have ordinals in tables, and extensible unions.
We are not proposing changing those: in both these cases, there is no breakage
at a distance scenario today (e.g., no extensible union composes other
extensible unions).

## Implementation strategy

Similar strategy to what has been followed for explicit-to-hashed ordinals.

## Ergonomics

No change.

## Documentation and examples

Need to modify wire format.

## Backwards compatibility

Not backwards compatible.

## Performance

No impact expected.
Method and event dispatch must now be done on a 64 bit integer (vs a 32 bit
integer), and this is expected to make no difference.

## Security

No impact on security.
See [Alternative: Identifying Protocol and
Method](#alternate_identifiying-protocol-and-method) for a security
motivated use case whose performance could be improved with another scheme.

## Testing

Unit testing for ordinal calculation.
Follows similar pattern to [RFC-0020: Interface Ordinal Hashing](/docs/contribute/governance/rfcs/0020_interface_ordinal_hashing.md).

## Alternative: Identifying Protocol and Method

We envision sandboxing services that would shield another service from
unauthorized use, i.e., a sort of firewall for FIDL messages.
In building such a sandboxing service, it would be useful to efficiently
reference a set of messages ("the allowed messages").
One could imagine defining this set of messages using protocols.

In this scenario, having two identifiers, one for the protocol, and one for
the method, would be useful (as opposed to the proposed scheme above, which
only provides one identifier).

Let's consider this alternative, where we have:

*   **P** bits for hash of protocol name
*   **M** bits for hash of method name

Hence, the total size of the ordinal would be:

*   **P** + **M** + 1

Since we need to reserve 1 bit for system ordinals.

For instance, with the example library:

```fidl
library alternative;

protocol Parent {
    Get(); // alternative/Parent.Get
}

protocol Child {
    compose Parent;
    Get(); // alternative/Child.Get
}
```

Both "Get" methods would have the same **M** bits (it's the hash of "Get").
However, the **P** bits would differ; one would be the hash of
`alternative/Parent`, whereas the other would be the hash of
`alternative/Child`.

From a feasibility standpoint, using a similar numerical approach to the
above, we have:

*   How many protocols do we expect? On the order of 100k is reasonable.
    *   → would need **P** = 52 bits
*   How many methods do we expect? On the order of 1k is reasonable.
    *   → would need **M** = 39 bits
*   So this scheme would require **92 bits**

As a result, we do not consider this alternative to be feasible.

Additionally, considering the sandboxing use case further, matching against
one protocol identifier is insufficient due to protocol composition (methods
could come from multiple source protocols).
Hence, while an optimized path may benefit from a single identifier, the
general case requires a lookup through some data structure to make this
efficient.

## Addendum: Collision Probabilities

#### Results

```
size | num_methods | r | p(collision)
-----+-------------+---+-------------
  31 |        1000 |   | 0.0002325707643
  39 |        1000 | x | 0.0000009085847943
  47 |        1000 | x | 0.000000003549160959
  52 |        1000 | x | 0.0000000001109112802
  63 |        1000 | x | 0.00000000000005415589852
  31 |       10000 |   | 0.02301183054
  39 |       10000 |   | 0.00009093624028
  47 |       10000 | x | 0.0000003552357776
  52 |       10000 | x | 0.00000001110111996
  63 |       10000 | x | 0.000000000005420468761
  31 |       50000 |   | 0.4412566126
  39 |       50000 |   | 0.002271108402
  47 |       50000 |   | 0.00000888156712
  52 |       50000 | x | 0.0000002775501665
  63 |       50000 | x | 0.000000000135522561
  31 |      100000 |   | 0.9025370676
  39 |      100000 |   | 0.009053622963
  47 |      100000 |   | 0.00003552615045
  52 |      100000 |   | 0.000001110211306
  63 |      100000 | x | 0.0000000005420956651
  31 |     1000000 |   | 1.0
  39 |     1000000 |   | 0.5972719635
  47 |     1000000 |   | 0.003546406718
  52 |     1000000 |   | 0.0001110160287
  63 |     1000000 | x | 0.00000005421005294

size: max. num_methods
31: 66
39: 1049
47: 16777
52: 94906
63: 4294968
```

#### Code

Using [http://mpmath.org](http://mpmath.org) to calculate the various probabilities.

```
from mpmath import mp
mp.dps = 50
mp.pretty = True

# Given n random integers drawn from a discrete uniform distribution with
# range [1,d], what is the probability p(n; d) that at least two numbers are
# the same?
def p_collision(n, d):
    # 1 - ((d - 1) / d) ^ (n * (n - 1) / 2)
    base = mp.fdiv(mp.fsub(d, 1), d)
    expo = mp.fdiv(mp.fmul(n, mp.fsub(n, 1)), 2)
    return mp.fsub(1, mp.power(base, expo))

print("size | num_methods | r | p(collision)")
for num_methods in [1000, 10000, 50000, 100000, 1000000]:
    print("-----+-------------+---+-------------")
    for size in [31, 39, 47, 52, 63]:
        p = p_collision(num_methods, mp.power(2, size))
        # 1 in 1,000,000
        result = " "
        if p < mp.fdiv(1, 1000000):
            result = "x"
        print("%4d | %11d | %s | %s" % (size, num_methods, result, mp.nstr(p, 10, min_fixed = -mp.inf)))

def find_max_num_methods(size):
    low = 1
    target = 1
    upper = 10000000
    while True:
        p = p_collision(target, mp.power(2, size))
        if p < mp.fdiv(1, 1000000):
            low = target
        else:
            upper = target
        target = ((upper - low) / 2) + low
        if upper - low < 2:
            return low

print("size: max. num_methods")
for size in [31, 39, 47, 52, 63]:
    print("%d: %s" % (size, find_max_num_methods(size)))

```

<!-- xrefs -->
[channel_call]: /docs/reference/syscalls/channel_call.md
[birthday]: https://en.wikipedia.org/wiki/Birthday_problem#Generalizations1
