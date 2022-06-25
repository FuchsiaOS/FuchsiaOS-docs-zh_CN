# Netstack Team's Cultural Tenets

Here are some cultural tenets which have helped keep the members of the [Fuchsia
Netstack team] aligned and productive. Having these values helps us navigate
prioritization and ambiguity.

## Compliance & Compatibility

> Fuchsia's success depends on being **compliant on the wire and compatible on
> the API surface**.

It is our responsibility to ensure RFC compliance and compatibility with adopted
network protocols. Instances of non-compliance are considered bugs except where
explicitly deemed necessary for interoperability with third parties. We have a
bias for citations; prior art and references are good sources of default
decisions.

We actively avoid the trap of "invented here", always designing with a guarantee
for compatibility with known networking APIs. We prefer to use Fuchsia concepts
to empower and improve on common networking concepts and APIs rather than invent
new ones for novelty's sake. We allow reinvention when needed, but compatibility
must always be addressed.

We encode our compliance and compatibility requirements by authoring tests that
run against other platforms as well as Fuchsia. That ensures that Fuchsia's
behavior matches or, alternatively, that the test serves as clear documentation
of any deviation from it.

## Shared Ownership

> Cross-contribution increases diversity of thought, and improves our
> [bus factor]. We reject internal and external siloing.

Networking and its subsystems are an integral part of the Fuchsia platform.
Information silos build walls between those system layers hindering creativity
and progress. We celebrate cross contributions by actively engaging, welcoming,
and encouraging them.

We're good neighbors to our sister and cousin teams, we hop over the ownership
fence to help them out and invite them over to do the same. We encode, publish,
and disseminate the good patterns that we find or come up with.

## Urgency Addressing Disrepair

> Left unattended, all code rots. It's our collective responsibility to actively
> fight against this decay.

We recognize cleanliness begets cleanliness. We believe clean, predictable, and
explicit code improves our engineering velocity. While working on the tree we're
constantly on the lookout for paper cuts and rough edges. When we hit those, we
prefer to fix them immediately. Sometimes the detour proves larger than
expected, but we seek not to lose the signal and document the need to revisit by
filing bugs and leaving breadcrumbs.

It takes a constantly vigilant eye to notice some of these instances. Sometimes
it's a pattern that stops making sense but is still being used; other times it's
a pattern that is not quite a good match in a particular case; often the rough
edge simply becomes clearer in hindsight. When navigating existing code, we're
aware that we at the same time know more (through hindsight) and less (through
not being original authors) than the original authors of the code. That
translates into a constant, yet respectful, challenge to the status quo.

The sense of urgency in addressing disrepair comes from the understanding that
copying existing patterns is always the path of least resistance, so by acting
quickly we prevent the spread of rot. In line with *Shared Ownership*, we apply
these standards to all the code in the Fuchsia tree.

[bus factor]: https://en.wikipedia.org/wiki/Bus_factor
[Fuchsia Netstack team]: /src/connectivity/network/OWNERS
