# Decentralized Product Integration

 * Project lead: mesch@google.com
 * Area(s): Infra, SWD, MOS, Developer

## Problem statement

Fuchsia customers need the ability not just to create Fuchsia software outside
of the Fuchsia repository, but also to integrate [products][glossary.product]
from such software and the Fuchsia platform software outside of the Fuchsia
repository and the currently used global integration build process.

The former is supported by the existing SDK; the latter is currently
"impossible", i.e. it's not done in practice, a theoretical understanding of
how to do it is not fully established and documented, and the tools needed to
apply such a theory in practice may not exist.

## Solution statement

Creation of a "Product Development Kit" (PDK) together with the conventions,
recommendations, and infrastructure that support the use of the kit and that
are applied across multiple repositories that contribute to a product.

As ingredients of this PDK we propose:

 1. To create a set of tools to upload to globally addressable repositories of
    Fuchsia artifacts (mostly packages, but also kernel etc.) from all builds
    (Fuchsia and petals).

 2. To define a naming and metadata convention to reference such artifacts in
    their repository location across build, integration, and deployment
    locations, and to reason about such references with regard to versioning,
    build parameters, etc.

 3. To define a format to describe a product integration able to express the
    configuration of a product outside of global integration, by way of
    referencing the artifacts that contribute to the product using the names
    and metadata above.

 4. To define how to use a set of tools to assemble a product system image and
    OTA update package from such a product integration description.

As the shared and globally addressable repository we propose to use the TUF
implementation by Fuchsia, MOS. The key contribution will be the naming
conventions used to address packages for reference in integration, which are
expected to be applicable to other ways of sharing binary artifacts between
repositories, such as static HTTP servers or CIPD.

The tools will be part of the SDK. Thus, the PDK is not so much a separate kit
to be created, but rather a specific way to complement the SDK with new tools
and new ways to use the new as well as the existing tools.

As a first tangible result to aim for, we propose to support the release of the
workstation product using the PDK from its own public repository to MOS. We
start by creating a "Hello World" product in its own out of tree repository,
and get it to integrate and release it from there. Once this works, we
reverse-merge workstation into the Hello World product.

## Dependencies

### SWD

A novel assembly tool for Fuchsia system images is developed by the SWD team,
to be used for the current in tree assembly of products and to be shipped with
the SDK for out of tree assembly. Once the tool is available in the SDK, we
will use it for assembly, and until then prototype with the existing tools
currently used in global integration.

### MOS

The tools to upload to MOS are developed as part of this effort. They use the
existing MOS APIs in novel ways and exercise the MOS infrastructure at novel
scale. The MOS team supports our efforts.

### Workstation

The workstation team is committed to move workstation off the Fuchsia
repository and will use the PDK to integrate the product in its own repository
and release it to MOS.

## Risks and mitigations

There is no direct risk of interference with existing customers. All existing
processes remain in place. We add new processes in new places, where we don't
expect them to interfere with product development and releases.

As indirect risk, we do propose to use MOS for novel purposes with higher
intensity. This might expose scaling issues with MOS that could interfere with
the ongoing support for releases by existing customers. Mitigating is that none
of the processes we propose to base on MOS/TUF are critical and can be stopped
any time.

[glossary.product]: /docs/glossary/README.md#product
