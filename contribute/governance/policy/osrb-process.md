# Open Source Review Board (OSRB) process

This document details the processes for adding external code to the
[Fuchsia Platform Source Tree](https://fuchsia.googlesource.com/).
For more information on the definition of external code,
see [Fuchsia Open Source Licensing Policies](/docs/contribute/governance/policy/open-source-licensing-policies.md).

## Overview

Any external code that is hosted within the [Fuchsia Platform Source Tree](https://fuchsia.googlesource.com/)
, must be compliant with [Fuchsia Open Source Licensing Policies](/docs/contribute/governance/policy/open-source-licensing-policies.md)
at all times.

## Process for adding external code to new repositories

To host external code within a new repository that does not exist yet, or does
not yet contain any code, submit an issue in Monorail using the
[Open Source Review Board (OSRB) template](https://bugs.fuchsia.dev/p/fuchsia/issues/entry?template=Open+Source+Review+Board+).

This issue lets Fuchsia’s OSRB review the code that you want to add, to ensure
that the code that you want to add is in compliance with [Fuchsia Open Source Licensing Policies](/docs/contribute/governance/policy/open-source-licensing-policies.md).

Warning: You must receive approval from the OSRB _before_ pushing a commit to Gerrit that adds external code to new repositories. Do not request a code review for adding external code to new repositories until you have approval from the OSRB.

If the request is approved, a member of the OSRB communicates the next steps
to the requester.

## Process for adding external code to repositories with existing external code

To add external code to an existing Fuchsia repository, create an issue in
Monorail using the [Open Source Review Board (OSRB) template](https://bugs.fuchsia.dev/p/fuchsia/issues/entry?template=Open+Source+Review+Board+).

This Monorail issue lets Fuchsia’s OSRB review the code that you want to add
to ensure that the code that you want to add is in compliance with
[Fuchsia Open Source Licensing Policies](/docs/contribute/governance/policy/open-source-licensing-policies.md).

Warning: You must receive approval from the OSRB _before_ pushing a commit to Gerrit that adds external code to repositories with existing external code. Do not request a code review for adding external code to repositories with existing external code until you have approval from the OSRB.

If the request is approved, a member of the OSRB communicates the next steps
to the requester.

## Questions about adding external code

If you are unsure if the external code that you want to add to the
[Fuchsia Platform Source Tree](https://fuchsia.googlesource.com/) should be in
a new repository or an existing repository, email [external-code@fuchsia.dev](https://groups.google.com/a/fuchsia.dev/g/external-code).
In your email, include answers to the following questions:

  * How many files is the code that you’re trying to import?
  * Do you want the code that you’re importing to track upstream?

## Process for modifying the stated Name, URL, License, or Usage of existing repositories

To modify the Name, URL, License, or Usage of an existing Fuchsia repository,
create an issue in Monorail using the [Open Source Review Board (OSRB) template](https://bugs.fuchsia.dev/p/fuchsia/issues/entry?template=Open+Source+Review+Board+).

Warning: You must receive approval from the OSRB _before_ pushing a commit to Gerrit that modifies the stated Name, URL, License, or Usage of existing repositories. Do not request a code review for  modifying the stated Name, URL, License, or Usage of existing repositories until you have approval from the OSRB.

If the request is approved, a member of the OSRB communicates the next steps
to the requester.

