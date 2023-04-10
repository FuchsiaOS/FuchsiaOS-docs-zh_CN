# Compatibility Tests for Fuchsia

The Compatibility Tests for Fuchsia (CTF) are a set of tests designed to provide
coverage for Application Programming Interface (API) and Application Binary
Interface (ABI) elements (e.g., headers, FIDL files) made available to
developers via a Fuchsia SDK. It was originally proposed as [RFC
0015](/docs/contribute/governance/rfcs/0015_cts.md), and
the project code is located at
[//sdk/ctf](https://fuchsia.googlesource.com/fuchsia/+/refs/heads/main/sdk/ctf/).

## Background, Motivation, and Goals

The CTF exists to determine whether a build of the Fuchsia platform, running on
a particular device, correctly implements (or *is compatible with*) the API and
ABI exposed by a particular Fuchsia SDK.  To put it another way, it demonstrates
that the build correctly implements Fuchsia.

If a system running Fuchsia passes the CTF tests for a particular ABI revision,
then its developers can have confidence that components built for that revision
will work on the system, and that the system is backwards compatible with that
revision.

Each release of the Fuchsia platform is bundled with a set of Software
Development Kits (SDKs), which contain tools, libraries, and headers that allow
developers to target Fuchsia's APIs and ABIs.  We refer to the API and ABI as
Platform Surface Area (or PlaSA).  Each SDK will be paired with a set of CTF
tests that exercise the surface area it exposes.  The tests will be available in
both source and binary form.

CTF tests are not comprehensive.  They cannot guarantee that any component that
is built against one SDK will run against any particular build of Fuchsia.  CTF
must, therefore, be complemented with product-specific testing.

With that in mind, the CTF can then be used to enforce compatibility in the
following ways:

### For platform developers, including those working in fuchsia.git, system integrators, product developers, and device developers

The CTF binary tests can be run against a device running Fuchsia to ensure that
the build on that device is ABI compatible with the CTF's associated SDK.  This
can provide enforcement of backwards compatibility guarantees: if the CTF from a
given SDK passes, that is a strong indicator (although not a conclusive one)
that programs built against that SDK will continue to work.  It can also provide
confidence that software running on the device that is not exercised by in-tree
tests, such as out-of-tree device drivers, does not change the behavior of the
platform.

As a table:


| Run → Against ↓                       | CTF N  | CTF N + 1   |
|---------------------------------------|--------|-------------|
| SDK / Product Build at version N      | A      | B           |
| SDK / Product Build at version N + 1  | C      | A           |

Where:

A = Someone who wants to make sure a product build correctly implements the ABI
revision they claim it does.

B = Someone who wants to make sure that a product build is forward compatible
with the a newer ABI revision.  Fuchsia org doesn't provide this kind of
guarantee.

C = Someone who wants to make sure that a product build is backwards compatible
with an older ABI revision.  Fuchsia org provides this kind of guarantee to
customers whose code targets older SDKs.

### For SDK vendors

The CTF source tests can be built against an SDK to ensure that the SDK is API
compatible with the CTF's associated SDK.  Additionally, CTF contains a suite of
tests for SDK host tools.  These tests can provide confidence that changes to
the SDK do not break developer code and workflows.  For example, we can build
the CTF for API version N against an SDK that contains support for API version
N+1 to see if the SDK has broken API compatibility.  Currently, the only SDK
vendor supported by the CTF project is the Fuchsia organization itself.

As a table:

| Build → Against ↓                     | CTF N  | CTF N + 1   |
|---------------------------------------|--------|-------------|
| SDK at version N                      | A      | B           |
| SDK at version N + 1                  | C      | A           |

Where:

A = Someone who wants to make sure an SDK correctly implements the API level
they claim it does.  This includes Fuchsia org (testing at tip of tree).

B = Someone who wants to make sure that an SDK is forward compatible with the a
newer API level.  Fuchsia org doesn't provide this kind of guarantee.

C = Someone who wants to make sure that an SDK is backwards compatible with an
older API level.  Fuchsia org provides this kind of guarantee to customers whose
code targets older SDKs.
