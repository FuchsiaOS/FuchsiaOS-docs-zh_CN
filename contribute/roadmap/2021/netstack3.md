# Netstack3 - A Fuchsia owned rust based netstack

 * Authors: ghanan@google.com, asafi@google.com
 * Project lead: tamird@google.com
 * Area(s): Connectivity

## Problem statement

The current netstack is written in Go which is not an approved language (and is
garbage collected) and is owned by the gVisor team who have different
priorities, use cases and design goals than Fuchsia. The gVisor netstack was
not originally designed to run on real devices, operate as a router or support
dynamic configurations; Fuchsia on the other hand does run on real devices,
operates as a router and depends on dynamic configurations (gVisor originally
didn't expect addresses, routes, network interfaces or their link status to
change at runtime).

## Solution statement

By developing a netstack that the Fuchsia team owns, we can design with
Fuchsia's goals in mind without having to depend on another team and work
around their use cases and restrictions. The Fuchsia Netstack team will design
and implement a Rust-based netstack that achieves functional parity with the
existing netstack, while leveraging the type and memory safety of rust.

## Dependencies

All users and components should be unaware of the netstack switching underneath
them.

## Risks and mitigations

Risks:

 * The netstack is extremely load bearing as all workflows and product use
   cases depend on network connectivity.
 * Interoperability with netstacks on other platforms (Linux, BSD, Windows,
   etc.).
 * Performance risks. We still can't estimate N3 performance to validate we're
   at parity.
 * Posix compatibility.

Mitigations:

 * Use industry standard tools/partners to ensure RFC compliance and
   interoperability (IxANVL, INTACT, NOVUS labs, etc.).
 * Leverage netstack and platform independent tests to test Netstack3 with the
   behaviour of the existing netstack and Linux.
 * Measure performance of the Netstack 3 during its iterative development.
 * Phased migration into Netstack3 with a plan to start running current CI
   workloads as soon as Netstack3 is capable. In addition, transition eng/dev
   targets to Netstack3 before product targets.

The netstack team has been planning for how to reliably test Netstack3 and
ensure parity with the existing netstack to avoid regressions when a switch is
made; the above mitigations are examples of what the team *is currently doing*.
IxANVL and INTACT tooling do not make assumptions about the underlying netstack
implementation and the tests mentioned above can be updated to also run on
Netstack3 once it is ready.
