# Report a security issue

Filing a security issue is a great way to contribute to the Fuchsia project.
Security issue reports that relate to Fuchsia may be eligible for reward
payments under the Android and Google Devices Security Reward Program.

For more information on the program's details and eligibility, see
[Android and Google Devices Security Reward Program](https://bughunters.google.com/about/rules/6171833274204160/android-and-google-devices-security-reward-program-rules) and
[Google Bug Hunters - Fuchsia](https://bughunters.google.com/report/targets/189296641).

Note: You need a Google account to report a bug to the Program.

## Report a new security issue

To report a security issue in Fuchsia, use the [Google Bug Hunters reporting
form](https://bughunters.google.com/report/vrp) and provide the details of your
issue.

###  Complete the issue template

Please include the following information in your issue description.

#### Bug or vulnerability details

Provide a brief explanation of the security issue, including any of the
following:

* The location of that issue in [the
  code](https://cs.opensource.google/fuchsia/fuchsia/+/main:) or in the user
  interface.
* The category of the security issue. Categories include buffer overflow,
  phishing, broken cryptography, authorization bypass, etc.
* The types of assets affected. Asset types include browser cookies, control
  of a device, personally-identifiable information, etc.

#### Version information

Provide any version information associated with your security issue, for
example:

* fuchsia.git revision number
* Fuchsia version number
* Product name (for example, Nest Hub)

#### Steps to reproduce / proof-of-concept

Provide a demonstration or list of steps needed to reproduce the security issue.

Demonstration information can include the following:

* Source code
* Address Sanitizer dumps
* Debugger output
* Example inputs, for example, a JPEG file that crashes the JPEG handler

Minimize the proof-of-concept files and attach them directly to the issue, not
within zip or other archive formats.

Be sure to remove any content not required to demonstrate the issue, including
any personal or confidential information.

#### Credit information for Common Vulnerabilities and Exposures (CVE) and/or Release Notes

Published security issues are publicly visible. For example, a security issue
can be published as a CVE or as a part of the release notes. If you'd like to be
credited for your discovery, provide a one-line description stating how you'd
like to be publicly credited. You can use your name, a pseudonym, or you can
remain anonymous.

## Issue resolution

The Fuchsia Security Team triages incoming issues and assigns those issues to
the appropriate team. The assigned team can then prioritize, assign, and respond
to the issue with guidance from Fuchsia Security.
