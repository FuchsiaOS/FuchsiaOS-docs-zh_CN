# Verified Execution

## Background

Fuchsia is designed from the ground up to support and extend strong verified
boot models. In most systems with verified boot, verification only covers code
up to a certain privilege level, and at some point unverified code must execute.
Fuchsia has taken verification into the runtime of the system, hence the name
“verified execution” (abbreviated as VX or FVX). The goal of FVX is simply to
ensure that all executable code is trustworthy.

Unless specified, this document uses *executable code* to refer to machine code
that executes directly on the hardware. Executable code does *not* refer code
that is sandboxed, interpreted, or otherwise subject to a confinement mechanism.

### Scope

This document describes the general principles and phases of verified execution
for an end-user buildtype (unless otherwise specified). Implementation details
and policy decisions that are hardware- or product-specific are out of scope.
This document focuses on the core verification logic and leaves out several
layers of defense-in-depth.

### Security Models

VX considers two security models: the running software model and the verified
boot model.

```
+------------------------------------------------------------------------------+
|                                                                              |
|  +---------------------------------------------+--------------------------+  |
|  |                                             |                          |  |
|  | +------------------+ +--------------------+ | +----------------------+ |  |
|  | | Recovery from    | | Recovery from full | | |  Mitigation against  | |  |
|  | | physical attacks | | attacker control   | | |   vulnerabilities    | |  |
|  | +------------------+ +--------------------+ | +----------------------+ |  |
|  |            Verified boot model              | Running software model | |  |
|  +---------------------------------------------+--------------------------+  |
|                                                                              |
|                            Verified execution                                |
+------------------------------------------------------------------------------+
```

#### The Verified Boot Security Model

There are two aspects to the verified boot security model: recovery from
physical attacks, and recovery from full attacker control. Physical attacks are
largely out of scope for this overview, but suffice it to say that an attacker
with physical access can accomplish strictly less than one with full control.

*Full attacker control (FAC)* is the ability for an attacker to run code of
their choosing as “supervisor” or “kernel” mode (the existence of more powerful
modes of execution is not considered here). The goal of a defender in this
security model is to recover by eliminating untrustworthy states (code and data)
in which attackers could persist control across reboots.

Fuchsia’s response to the verified boot security model is to allow a computer to
recover from any compromise by simply rebooting. At boot, Fuchsia verified
execution recursively guarantees that

1.  all executed code is trustworthy according to an “anchor” (for example, code
    in ROM), and
1.  all data that serves as input to code is trustworthy according to the same
    anchor.

This guarantee allows the system to fully remove untrustworthy states, even in
the presence of arbitrarily bad vulnerabilities.

##### Anti-Rollback

The verified boot model assumes that an attacker can replace mass storage
content with the content of their choosing. Under this model, any security
vulnerability that can be exploited after a reboot can never be patched without
a hardware-based anti-rollback mechanism. Without such a mechanism, an attacker
could flash an older, vulnerable version of the operating system that will pass
verified boot checks and then replay the exploit needed to regain FAC.
Hardware-based anti-rollback prevents this scenario entirely by refusing to boot
an out-of-date version of Fuchsia once the vulnerability has been patched.

#### The Running Software Model

In this model, software runs and attackers try to exploit vulnerabilities in the
software to gain control over the running software, usually by crafting
malicious inputs to the software. In this security model, the aim of a defender
is to solve or mitigate possible vulnerabilities by hardening code against
malicious input.

Fuchsia’s response to the running software model is to restrict execution to
trustworthy code. (What is “trustworthy” will be defined in more detail later.)

### Principles

The following is a non-exhaustive list of general principles guiding the
implementation of Fuchsia verified execution:

-   Code and associated read-only data should be verified against an intentional
    policy. Policies may not be the same for every kind of code, product, or
    build type, but all code should be bound to a policy. A simple example of a
    policy is “code cannot execute unless it is digitally signed by a trusted
    party.”

    -   There is no default or fallback policy.
    -   Policies do not allow one-off, side-loaded code (no “snowflakes”).
    -   Policies rarely allow just-in-time compilation (JITs).

-   Inputs to policy verification and cryptographic schemes should be as simple
    as possible.

    -   Policy verification should not require parsing untrusted complex data.

-   Verified execution can be implemented on a variety of hardware, but strongly
    prefers to have hardware immutable trust anchors and tamper-resistant
    storage (for example, to store a rollback index that can only be modified by
    firmware, not the kernel).

## Phases of Verification

Code and data are deemed trustworthy after verification using a hash-and-sign
paradigm: data is loaded in message+signature pairs and verified using a trusted
public key. Messages contain cryptographic hashes of larger code or data blobs,
which are hashed and verified before treating them as trustworthy.

### Phase Zero: Hardware to First Bootloader

Phase zero comprises the hardware to software transition. There is nothing
verified execution can do against hardware attacks, so for the purpose of this
document, the hardware is assumed to be trusted. In this phase, immutable code
verifies the first bootloader against an immutable trust anchor (for example, a
public key in One-Time Programmable (OTP) memory). The details are
hardware-specific.

### Phase One: First Bootloader to Main Bootloader

Once the first bootloader has been verified, it is trusted to verify and execute
additional software needed to boot the system. This could be a chain of software
images where each one verifies and executes the next, or it could be a tree-like
flow. This phase is also hardware-specific.

### Phase Two: Main Bootloader to Preauthorized Code

The main bootloader is responsible for the verification of preauthorized code,
which refers to all code explicitly approved by a product authority to run as a
core part of that product. Preauthorized code may run on the hardware directly
(i.e. without a sandbox) and includes all the required elements of a Fuchsia
system (for example, Zircon kernel, package management system, drivers). The
bootloader verification may be product-specific, but usually a version of
Android Verified Boot
([AVB](https://android.googlesource.com/platform/external/avb/+/master/README.md)))
is used to verify the Zircon Boot Image
([ZBI](/docs/glossary/README.md#zircon-boot-image)).

#### Delegation from the Main Bootloader

After the bootloader verifies the ZBI, there are two crucial points where
verification responsibility is delegated.

1.  The verified ZBI verifies the package management system. Specifically, the
    ZBI receives from the bootloader a precise description of the package
    management system and enforces all policies which apply to that system.
1.  Once verified, the package management system becomes responsible for the
    verification of all remaining preauthorized code that is not part of the ZBI
    or the package management system.

#### Direct vs Indirect Verification

Direct verification specifies exactly *what* to trust by precisely describing
the software that is permitted to run (for example, using a cryptographic hash
of a package). Indirect verification specifies who to trust and leaves the
*what* up to a delegated authority. Once the package management system takes
over as the VX policy enforcer, it may use indirect verification to allow for
on-the-fly delivery of software without a full system update and reboot. To
maintain the chain of trust, components and configuration involved in the
indirect verification process (ZBI, package management system) must always be
verified directly.

Note that descriptions for direct verification can be arbitrarily complex. If a
signature is attached to a hash of a file that contains more hashes, all of the
hashed content is still directly verified.

### Phase Three: Non-Preauthorized Code

If neither the code nor its signing authority was authorized by the product
owners, that code is not verified so much as it is subject to a static policy.
Some purpose-built devices may not run non-preauthorized code at all. The
enforcement of the static policy is the role of the package management system
and component framework.

#### Strong vs Weak Intention

Non-preauthorized code with strong intention is code from a package that the
user, acting in an administrative role, has explicitly authorized to run in a
particular environment. This type of non-preauthorized code can be run directly
on the machine in an environment that is isolated from preauthorized code.
Non-preauthorized code with weak intention is code that is loaded incidentally
or authorized by users not acting in an administrative role (for example,
JavaScript loaded by a web page). Non-preauthorized code with weak intention
must be sandboxed, interpreted, or otherwise subject to a strict confinement
mechanism.

## Implementation

The following Fuchsia systems are integral to enforcing verified execution
policies.

### Main Bootloader

The main bootloader implementation relies on
[Android Verified Boot](https://android.googlesource.com/platform/external/avb/+/master/README.md)
for verification and kernel rollback protection.

### BlobFS

[BlobFS](/docs/concepts/filesystems/blobfs.md) is a cryptographic,
content-addressed filesystem purpose-built to support verified execution. BlobFS
is the sole storage system for executable code and associated read-only data
(with exceptions for pre-kernel code, the kernel, and its
[bootfs](/docs/concepts/process/userboot.md#bootfs)) ramdisk, all of which are
stored in the ZBI). Each blob in BlobFS is uniquely represented and accessed by
a hash ([Merkle root](/docs/concepts/packages/merkleroot.md)), and a Merkle tree
structure allows for random blob access. It is computationally infeasible for an
attacker to change a blob without changing the hash.

Once BlobFS has been loaded in Phase Two, the method of verifying preauthorized
code is simply to check a signature for each blob hash that is loaded, and to
ensure that the blob content matches the hash. BlobFS performs the content
verification, and the signature checks are delegated to the package management
system.

### Package Management System

The package management system, also known as the software delivery (SWD) stack,
adds a layer over BlobFS to make handling blobs friendlier to programmers. The
SWD stack translates Merkle roots describing packages (specifically, the
[Merkle root of the package’s meta.far](/docs/concepts/packages/package.md#structure-of-a-package))
into human-readable package names. If any package content changes, the Merkle
root of the meta.far will change. For brevity, this document will use “package
hash” to refer to the package’s meta.far Merkle root.

The SWD stack manages a special package called the system image, which includes
a list of hashes for all
[base packages](/docs/concepts/packages/package.md#base-packages). The system
image package is verified by the ZBI when the package management system is
loaded in Phase Two, so all base packages are directly verified; that is, no
signature checks are required after boot. For some purpose-built devices, the
SWD stack can be configured such that only base packages can be loaded into
memory with execute rights, which means the device will only execute directly
verified code.

For indirectly verified code
(“[universe packages](/docs/concepts/packages/package.md#universe-packages)”),
trusted public keys and other metadata about the delegated authority that
provided them are included in a location covered by the verified boot signature
(for example, the ZBI or a base package). Any packages downloaded from such an
authority must be verified, by signature and content hash, every time the
package is loaded (rather than checking the signature once at download time).
“Backstop version” metadata may be included in the directly verified data to
ensure that both directly and indirectly verified packages are covered by
hardware anti-rollback protection.

The SWD stack is also responsible for downloading
[system updates](/docs/concepts/packages/ota.md). It has a number of controls
that prevent any blobs from an in-progress update from being available to the
running system before the device has rebooted into (and thus verified) the
update.

### Component Framework

Packages are the unit of software *distribution*, whereas components are
Fuchsia’s standard unit of software *execution*. One consequence of this
distinction is that most Fuchsia components do not interact directly with the
SWD stack. Instead, the main customer of the SWD package resolver is the
component framework, which contains a number of
[component resolvers](/docs/concepts/components/v2/capabilities/resolvers.md)
that delegate to the SWD stack to load code and data. Some component resolvers
are limited to base packages, and others may allow access to universe packages.
Like other Fuchsia capabilities, these resolvers are routed in component
manifests, which provides additional control over and auditability of their use.
For example, indirectly verified code can be limited to only specific
[environments](/docs/concepts/components/v2/environments.md).
