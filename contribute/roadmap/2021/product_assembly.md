# Scaling Product Assembly and Configuration

Project lead: aaronwood@google.com
Area(s): Build, Developer

## Problem statement

Product assembly, the process of creating an "image" out of the built software
and configuration data in the Fuchsia platform (fuchsia.git) and the product's
own repo(s), is currently restricted to a compilation-time operation that's
part of fuchsia.git.  It can only be done "in-tree" after the compilation steps
for all software are completed.

There has been a continual explosion of product configurations, across multiple
dimensions (`_eng`, `_eng_arrested`, `_user`, `_userdebug`, LSDi, etc), to
allow for the ability to name each of the configurations that developers,
testers, and customers need.

Product Owners and Developers still cannot easily express the combination that
they wish to build/run at any given time.

The majority of these definitions are due to the combinatorial expansion of the
following dimensions:

 * Base Product that will ship
 * Components Version: stable, latest, developer
 * Product Type: `eng`, `eng_arrested`, `userdebug`, `user`

Through the use of explicit package URLs to components in other packages, the
component topology itself is crystalized within the `fuchsia-pkg://` namespace
on the device, as each package's full URL is listed directly in its parent's
cml:

 * root.cml
 * core.cml

To change which package's component is being used for the implementation of
some protocol, some other package's contents have to change to reference that
different package url.

Fuchsia developers and release teams must manage all of our partners' products
in tree, and it's impossible for out of tree product owners to release or
update products on their own schedules.  This creates significant load on the
Fuchsia organization and adds friction with the product owners.

## Solution statement

To address this, we propose to create a set of tools that can run out-of-tree,
which combine the notion of assembly and configuration into parts of the same
process:

 * assembly is the process of specifying:
   * which software pieces are to be used
   * why they relate to each other the way that they do (topology of a single
     component)
   * how they are to be configured
   * when they can be reconfigured (and by who)

 * configuration is the providing of:
   * the data that each component needs to properly operate
   * the CFv1 sysmgr configuration of the product
   * the overall CFv2 topology of the product
   * kernel boot arguments

These are two halves of the same process: how to assemble and configure Fuchsia
for a given product.

To allow for more controlled use and configuration of Fuchsia, we propose to
introduce the concept of "sub-assemblies".  Each sub-assembly defines a set of
software and files, in a fragment of topology, with configuration points and
values, with explicit documentation for what configuration can be modified
during the assembly process.

When this is complete:

 * The Fuchsia Platform defines sub-assemblies that products can use to choose
   what platform behaviors they want to assemble when doing this out-of-tree
assembly
   * One example is the software delivery stack sub-assembly that contains the
     `omaha-client` (userdebug/user builds) vs. the one that uses
     `system-update-checker` (eng builds).
 * Customer products can be assembled entirely out-of-tree, without using
   `fuchsia.git` directly
 * Multiple products variants can be assembled out of the same set of compiled
   software components.

## Dependencies

Migration of customer products to use sub-assemblies.  This will require
consulting with yaar@ and other stakeholders to ensure that we've appropriately
captured the right aspects of them.

Migration of the Fuchsia platform to use sub-assemblies.  This will require
consulting with the various subsystem teams to ensure that we've appropriately
captured the right variations.

To create out-of-tree images, certain tools (fvm, blobfs, minfs, zbi, avbtool)
will need to be ported to a format that can be used out-of-tree (static library
for linking with Rust, or entirely ported to Rust).

## Risks and mitigations

The main risks are:

 * New tools don't produce the same output as the existing ones
   * Mitigation: Run them both in parallel in CI/CQ, validating the new tools
     produce the same output, and then switch to using the new tools.

 * In-Tree and Out-of-Tree tools could diverge in capabilities
   * Mitigation: Use the Out-of-Tree tools as the in-tree tools, so we don't
     have two sets of tools to maintain.

 * Sub-Assembly Schema design could become a place of seeking perfection.
   * Mitigation: All schemas will be versioned with the express intent of
     revising them during the process, focusing on pragmatic solutions over
     perfectly modeling the system.

 * Large migration effort (products and platform)
   * Mitigation: This can be done in phases, starting with sets of
     components/labels and working up to fully-defined sub-assemblies.
   * Mitigation: Implementation plan is centered around a measured approach
     without boiling the ocean.
