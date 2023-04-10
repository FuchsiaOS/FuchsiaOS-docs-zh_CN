<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0133" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

## Summary

A set of long-term goals for the Software Delivery area.

## Motivation

The Fuchsia Software Delivery system needs to prioritize the requirements of our
customers and will continue to do so, but we also need to set long-term
direction to keep in mind as we do design. Even though we don't have explicit
customer requirements for many of these goals, they are nonetheless goals that
we should design for and keep in mind as we evolve the system. This list may
change over time, and that's fine. However, this represents the desired state of
the world today.

## Stakeholders

_Facilitator:_

hjfreyer@google.com

_Reviewers:_

* abarth@google.com - FEC
* amathes@google.com - Product
* computerdruid@google.com - SWD
* ampearce@google.com - Security

_Consulted:_

* kitlane@google.com
* mckillop@google.com
* hjfreyer@google.com
* Software Delivery team

_Socialization:_

This RFC went through a review in doc form with the Software Delivery team, as
well as with the contributors.

## Design

### Goals

1. **Prioritize customer requirements**: We shouldn't be afraid to revisit
   existing implementations or change strategy if we have a compelling customer
   need, as long as we don't preclude long-term goals.
2. **One update protocol in production**: All devices in production use the same
   update stack and protocol for platform [OTA][glossary.ota]s as we recommend
   to our customers for updating other software.
3. **Platform and non-platform updates have the same features**: Platform OTAs
   and updates for independent modules have the same capabilities (channels,
   staged rollouts, stepping stones, etc.), and it should be possible for OSS
   users to use these capabilities.
4. **Minimize monoliths**: Over time, we intend most parts of the platform to be
   independently updatable.
5. **Provide recovery options for attacks which can modify persistent state**:
   The SWD stack should be able to perform an update automatically at boot with
   minimum dependence on mutable data and other software in order to maximize
   recovery potential from serious vulnerabilities, including ones which give an
   attacker the ability to modify persistent state.
6. **Update reliability is paramount**: We should prefer updating software as
   long as we can prove that it doesn't compromise security requirements. We
   should prioritize simple and reliable code, even at the cost of reasonable
   performance impact; we aim to create the simplest system that can perform
   updates correctly, securely, and that meets other performance requirements.
   We should bias towards designs that fail safely, and allow another chance at
   an update, rather than possibly corrupting state for another attempt.
7. **[Product owners][glossary.product-owner] decide policy for their
   [products][glossary.product]**: The platform defines what software update
   policies are expressible and recommends initial policy. The product owner can
   choose their configuration of policies, and change their policy choices at
   any time, even when devices are in the field.
8. **All software is verified against an intentional policy**: All software is
   checked against a policy defined for that type of software. These policies
   could include provenance checking for executable code, integrity checking,
   sandboxing, etc. No software can be run without a predefined policy regime
   under which to run it.
9. **Software source policy is a product concern**
    1. The platform does not restrict who can publish software, or what
       approvals are required. Centralized vs. decentralized software sources
       are a product policy, not a platform policy. As long as software is
       published with appropriate metadata like a publisher signature, the
       platform should be capable of running it.
    2. The platform will create mechanisms for enforcing software policy, but
       the decision of what software sources are available is up to product
       owners. Product owners can restrict available software sources through
       policy, but the platform will not.
    3. The platform should be capable of producing products that can install
       arbitrary software, including the ability for a user to allow software
       from arbitrary publishers to run on their device.

## Implementation

We'll link to this RFC from the Software Delivery [documentation][swd-readme],
and keep it in mind during design processes for the Software Delivery area.

## Performance

No performance impact.

## Security considerations

The implementation of this RFC has no immediate security impact. Long term, we
expect substantial collaboration with the security team as we execute on these
goals, as the SWD update mechanism is our primary method to secure
vulnerabilities in the field.

## Privacy considerations

The implementation of this RFC has no privacy impact. Long term, we expect
substantial collaboration with the privacy team as we execute on these goals.

## Testing

The implementation of this RFC has no testing impact.

## Documentation

We'll link to this RFC from Software Delivery [README][swd-readme].

## Drawbacks, alternatives, and unknowns

There are nearly limitless goals we could set for the Software Delivery area.
These are the goals we feel represent both the current state of the stack and
our desired state most accurately.

There are many unknowns, for instance how we'll integrate with any possible
third party publishers of software. Once we have customer requirements for
possible features, we'll be better able to reason about those unknowns.

## Prior art and references

Very little has previously been published about the SWD roadmap. However, many
previous iterations of package management and software supply chain security
exist. Here is a selection of prior art:

* [Debian software repositories](https://wiki.debian.org/DebianRepository)
* [Debian package signing](https://www.debian.org/doc/manuals/securing-debian-manual/deb-pack-sign.en.html)
* [Omaha protocol](https://github.com/google/omaha/blob/ebc25b2b3d77eed3d9a122bcfd89a66f6f192e4b/doc/ServerProtocolV3.md)
* [The Update Framework specification](https://theupdateframework.github.io/specification/latest/)

[glossary.ota]: /docs/glossary/README.md#ota
[glossary.product]: /docs/glossary/README.md#product
[glossary.product-owner]: /docs/glossary/README.md#product-owner
[swd-readme]: /src/sys/pkg/README.md
