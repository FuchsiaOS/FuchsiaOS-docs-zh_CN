# Fuchsia open source licensing policies

This document describes open source licenses and related policies in the Fuchsia project.

## Fuchsia project code

All Fuchsia project code is hosted on
[https://fuchsia.googlesource.com/](https://fuchsia.googlesource.com/). Fuchsia
project file headers will list `Copyright <year_of_file_creation> The Fuchsia Authors`.

## Licenses

Fuchsia is subject to multiple licenses:

*   The Fuchsia kernel is released under the following MIT-style license:
    [/zircon/kernel/LICENSE](/zircon/kernel/LICENSE).
*   All Fuchsia user space components are released under a BSD-style license:
    [/LICENSE](/LICENSE) or an Apache 2.0 license:
    [https://fuchsia.googlesource.com/infra/+/main/LICENSE](https://fuchsia.googlesource.com/infra/+/main/LICENSE).
*   All code that is BSD-licensed has an additional IP grant:
    [/PATENTS](/PATENTS).

Any code that has a different copyright holder or that is released under a
different license is considered external code per this policy and must adhere to
the external code policies in this document.

## External code

All external code hosted on
[https://fuchsia.googlesource.com/](https://fuchsia.googlesource.com/) must be
open source code released under a license from the
[approved licenses list](#approved-licenses), unless otherwise listed in the
[specific exceptions](#specific-exceptions) or
[proprietary exceptions](#proprietary-exceptions) sections. All external code
must be hosted in a repository whose name is prefixed with `third_party` or
within a directory named `third_party` within one of Fuchsia’s existing
repositories. If the code is hosted in its own repository, it must be mapped to
a path including a directory named `third_party` when checked out as part of the
Fuchsia Platform source tree. The set of licenses permitted for a particular
portion of code depends on the usage of that code - see below for a list of
approved licenses for production targets and development targets.

All code used in a Fuchsia build must be hosted on a Gerrit server run by
Google. In almost all cases, the code must be hosted on
[https://fuchsia.googlesource.com/](https://fuchsia.googlesource.com/).

All source code must be downloaded when running `jiri update`. No build steps
may download additional source code. Code from package management systems, such
as packages from Dart’s Pub or crates from Rust’s Cargo, must be vendored into
[https://fuchsia.googlesource.com/](https://fuchsia.googlesource.com/) and
comply with the same license requirements as any other code in the project.

## Licenses and tracking

Refer to
[What is a License?](https://opensource.google/docs/using/license/){:.external}
for an explanation of what an open source license is and why it is important.
All projects hosted on
[https://fuchsia.googlesource.com/](https://fuchsia.googlesource.com/) must be
released under an approved license and contain the full license text alongside
the code. Simply stating the license by reference - i.e. “BSD3” - is not
sufficient, the actual text must be included. In most cases, the project should
be an entire repository and the license text should be at the top level of the
repository in a file named LICENSE, COPYRIGHT, or similar. In rare cases where
the Fuchsia project needs to host multiple logical projects in a single
repository, for example in Fuchsia’s Dart pub vendor repository, each project
must be in its own directory with the license text for that project and a
top-level file in the repository must explain the set of licenses used by the
subdirectories.

To facilitate tracking, in addition to the license text, each project containing
external code must contain a README file containing information about the
project. The README must be named `README.fuchsia` and contain at least the
following information:

```
Name: common name for the project

URL: web site of upstream project

LICENSE: short description of license
```

It’s also recommended, but not required, that the `README.fuchsia` files
describe what version of the upstream project is being used and what kinds of
modifications, if any, were made to port to Fuchsia. The short description of
the license should be a
[Software Package Data Exchange (SPDX) license identifier](https://spdx.org/licenses/){:.external}
that matches the license but there can be more text in cases where more
elaboration on the license is required. Fuchsia project code, that is, code that
The Fuchsia Authors hold copyright for and code that is released under the
Fuchsia project’s standard license - does not require this file.

## Categories of code and allowed licenses

### Production target {:#production-target}

This section applies to all code that runs in a production Fuchsia-based device
in use by an end user. In this document, “production target” is defined as a
production Fuchsia-based device in use by an end user. “Production target”
includes the kernel, drivers, system services, frameworks, programs, etc running
on the device regardless of how they are deployed. Code is permitted in this
target if it is released under one of the following licenses and that license
only. If there are additional restrictions, such as an IP grant or other
additional clause, then the license approval does not suffice for that portion
of code.

#### Approved licenses {:#approved-licenses}

The following are the approved licenses for production target devices:

*   BSD 3-clause license, specifically the text at [/LICENSE](/LICENSE)

*   BSD 2-clause license, specifically the text at
    [https://opensource.org/licenses/BSD-2-Clause](https://opensource.org/licenses/BSD-2-Clause)

*   MIT license, specifically the text at
    [/zircon/kernel/LICENSE](/zircon/kernel/LICENSE)

*   X11 license, specifically the text at
    [/third_party/github.com/intel/libva/va/x11/va_dri2.c](https://fuchsia.googlesource.com/third_party/github.com/intel/libva/+/6e86b4fb4dafa123b1e31821f61da88f10cfbe91/va/x11/va_dri2.c)

*   Zlib license, specifically the text at
    [https://fuchsia.googlesource.com/third\_party/zlib/+/main/README#85](https://fuchsia.googlesource.com/third_party/zlib/+/main/README#85)

*   Libpng license, specifically the text at
    [https://fuchsia.googlesource.com/third\_party/libpng/+/main/LICENSE](https://fuchsia.googlesource.com/third_party/libpng/+/main/LICENSE)

*   Boost license 1.0, specifically the text at
    [https://fuchsia.googlesource.com/third\_party/asio/+/main/asio/LICENSE\_1\_0.txt](https://fuchsia.googlesource.com/third_party/asio/+/main/asio/LICENSE_1_0.txt)

*   OpenSSL license, specifically the text at
    [https://fuchsia.googlesource.com/third\_party/boringssl/+/upstream/master/LICENSE](https://fuchsia.googlesource.com/third_party/boringssl/+/upstream/master/LICENSE)

*   FreeType project license, specifically the text at
    [https://fuchsia.googlesource.com/third\_party/freetype2/+/main/docs/FTL.TXT](https://fuchsia.googlesource.com/third_party/freetype2/+/main/docs/FTL.TXT)

*   Apache 2.0 license, specifically the text at
    [https://fuchsia.googlesource.com/third\_party/flatbuffers/+/main/LICENSE.txt](https://fuchsia.googlesource.com/third_party/flatbuffers/+/main/LICENSE.txt)

*   Independent JPEG Group License (IJG), specifically the text at
    [https://fuchsia.googlesource.com/third\_party/iccjpeg/+/main/LICENSE](https://fuchsia.googlesource.com/third_party/iccjpeg/+/main/LICENSE)

*   ICU license, specifically the text at
    [https://fuchsia.googlesource.com/third\_party/icu/+/main/LICENSE](https://fuchsia.googlesource.com/third_party/icu/+/main/LICENSE)

*   Curl license, specifically the text at
    [https://fuchsia.googlesource.com/third\_party/curl/+/main/COPYING](https://fuchsia.googlesource.com/third_party/curl/+/main/COPYING)

*   University of Illinois / NCSA Open Source License (NCSA), specifically the
    text at
    [https://fuchsia.googlesource.com/third\_party/clang/+/main/LICENSE.TXT](https://fuchsia.googlesource.com/third_party/clang/+/main/LICENSE.TXT)

*   ISC license, specifically the text at
    [https://fuchsia.googlesource.com/third\_party/boringssl/+/upstream/master/LICENSE#143](https://fuchsia.googlesource.com/third_party/boringssl/+/upstream/master/LICENSE#143)

*   IBM-Pibs license, specifically the text at
    [https://github.com/u-boot/u-boot/blob/master/Licenses/ibm-pibs.txt](https://github.com/u-boot/u-boot/blob/master/Licenses/ibm-pibs.txt)

*   R8a779x\_usb3 license, specifically the text at
    [https://github.com/u-boot/u-boot/blob/master/Licenses/r8a779x\_usb3.txt](https://github.com/u-boot/u-boot/blob/master/Licenses/r8a779x_usb3.txt)

*   Creative Commons Attribution 3.0 Unported license at
    [https://creativecommons.org/licenses/by/3.0/legalcode](https://creativecommons.org/licenses/by/3.0/legalcode)

*   Creative Commons Attribution 4.0 International license at
    [https://creativecommons.org/licenses/by/4.0/legalcode](https://creativecommons.org/licenses/by/4.0/legalcode)

Code not under one of these licenses and not explicitly granted an exemption is
not permitted in the production target.

Licenses in the
[restricted](https://opensource.google/docs/thirdparty/licenses/#restricted){:.external}
or
[reciprocal](https://opensource.google/docs/thirdparty/licenses/#reciprocal){:.external}
categories will not be approved for use in Fuchsia.

#### Specific exceptions {:#specific-exceptions}

The following repositories have been granted specific exemptions for production
target devices:

*   [https://fuchsia.googlesource.com/third\_party/llvm/](https://fuchsia.googlesource.com/third_party/llvm/)
*   [https://git.kernel.org/pub/scm/linux/kernel/git/firmware/linux-firmware.git/tree/LICENCE.iwlwifi\_firmware](https://git.kernel.org/pub/scm/linux/kernel/git/firmware/linux-firmware.git/tree/LICENCE.iwlwifi_firmware)

These exemptions apply only to these specific repositories, and do not apply to
anything else no matter how similar they may seem.

#### Proprietary exceptions {:#proprietary-exceptions}

Under exceptional circumstances, when partners will not provide certain
technology under open source licenses, Fuchsia may only be able to provide
compiled binaries to the public under more restrictive license terms.

Any proprietary libraries that fall under this exception, such as several
compatible drivers, will be separate and clearly marked as proprietary materials
with the relevant license terms.

Proprietary license exceptions are disfavored in the Fuchsia ecosystem and
exceptions will only be made when technology substantially improves Fuchsia
functionality or interoperability substantially, no adequate open source
alternatives exist, and the code or binaries can be separated from Fuchsia’s
open source repositories. No third parties are entitled to an exception from the
Fuchsia licensing policy.

### Development target {:#development-target}

This section applies to all code that is used by developers building things for
Fuchsia including tools, debuggers, utilities, and examples. All licenses
permitted for production targets are permitted for the development target. In
this document, “development target” is defined as a non-production Fuchsia-based
device in use by a Fuchsia developer and not an end user.

#### Approved licenses

Additionally, the following licenses are permitted for the development target:

*   GNU General Public License v2.0 (GPL 2.0), specifically the text at
    [https://fuchsia.googlesource.com/third\_party/gdb/+/main/COPYING](https://fuchsia.googlesource.com/third_party/gdb/+/main/COPYING)

*   GNU Library General Public License 2.0 (LGPL 2.0), specifically the text at
    [https://spdx.org/licenses/LGPL-2.0.html#licenseText](https://spdx.org/licenses/LGPL-2.0.html#licenseText)

*   GNU Lesser General Public License 2.1 (LGPL 2.1), specifically the text at
    [https://spdx.org/licenses/LGPL-2.1.html#licenseText](https://spdx.org/licenses/LGPL-2.1.html#licenseText)

*   Open Font License 1.1 (OFL 1.1), specifically the text at
    [https://github.com/u-boot/u-boot/blob/master/Licenses/OFL.txt](https://github.com/u-boot/u-boot/blob/master/Licenses/OFL.txt)

#### Hosting development artifacts

To host an artifact (a binary or tarball) on
[Google storage](https://cloud.google.com/storage/){:.external} for development
purposes you must do the following:

*   Verify all transitive dependencies are under approved licenses.
*   Verify the exact source of all dependencies are hosted on
    [https://fuchsia.googlesource.com/](https://fuchsia.googlesource.com/).
    *   If some components are hosted elsewhere, contact the
        [Open Source Review Board (OSRB)](/docs/contribute/governance/policy/osrb-process.md)
        to check that the hosting arrangement satisfies the requirements of the
        Fuchsia project.
*   Produce a file containing the license text of the license of the binary and
    all transitive dependencies. Serve this file with the artifact (i.e., in a
    tar).

## Modifying external code

The process for modifying external code is the same as for modifying Fuchsia
project code. Be sure to keep the appropriate `README.fuchsia` files up-to-date
with a high level description of changes from upstream. Do not modify any
existing copyright notice or license file when changing external code.

## Support contacts

### Add new external code

For information on adding new external code, see
[Open Source Review Board (OSRB) Process](/docs/contribute/governance/policy/osrb-process.md).

### Questions

If you have a question about Fuchsia’s external policies or how these policies
relate to the Fuchsia project, email
[external-code@fuchsia.dev](https://groups.google.com/a/fuchsia.dev/g/external-code).
