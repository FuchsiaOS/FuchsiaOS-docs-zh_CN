# Flaky test policy

This document codifies the best practice for interacting with test flakes on
Fuchsia.

## Background: What is a flaky test?

A **flaky test** is a test that sometimes passes and sometimes fails, when run
using the exact same revision of the code.

Flaky tests are bad because they:

-   Risk letting real bugs slip past our commit queue (CQ) infrastructure.
-   Devalue otherwise useful tests.
-   Increase the failure rate of CQ, thereby increasing latency for modifying code.

This document is specific to *test flakes*, not infrastructure flakes.

## Requirements: Goals for flaky tests

1.  **Flakes should be removed from the critical path of CQ as quickly as
    possible**.
2.  Since flakes present themselves as a failing test, **flakes should not be
    ignored** once taken off of CQ. They represent a real problem that should be
    fixed.
3.  Tests may flake at any time, and as a consequence, the observer of these
    bugs may not necessarily be the person best equipped to fix it. **The
    process for reporting bugs must be fast, easy, and decoupled from diagnosis
    and patching.**

## Policy

The following provides the expected & recommended lifetime of a flake:

0.  A test flakes in CI or CQ.
1.  Identify: The test is *automatically* identified as a flake.
2.  Track: An issue is *automatically* filed for the identified flake under the Flake component.
3.  Remove: The test is removed from CQ immediately.
4.  Fix: The offending test is fixed offline and re-enabled.

#### Identify

A flake fetching tool is currently in use to identify the vast majority of flakes.

The tool looks for test failures in CQ where the same test succeeded when retried on the same
patch set.

#### Remove

One should prioritize, above all else, removing the test from the commit
queue. This can be achieved in the following ways:

-   If the flake has been prompted by a recent patch: Submitting a revert of a
    patch that triggers this flake.
-   [Disable the test](/docs/development/testing/faq.md#disable-test).

The above mechanisms are recommended because they remove the flaky test and
prevent the commit queue from becoming unreliable. The first option (reverting code)
is preferred, but it is not as easy as the second option (disabling test), which
reduces test coverage. Importantly, neither of these options prevent diagnosis
and fixing of the flake, but they allow it to be processed offline.

It is **not** recommended to attempt to fix the test without first
removing it from CQ. This causes CQ to be unreliable for all other
contributors, which allows additional flakes to compound in the codebase.

#### Fix

At this point, one can take the filed issue, locally re-enable the test, and work on
reproducing the failure. This will enable them to find the root cause, and fix the
issue. Once the issue has been fixed, the bug can be closed, and the test can be
re-enabled. If any reverted patches need to re-land, they can re-land safely.

When fixing a flake, verify the fix by [testing for flakiness in CQ](/docs/development/testing/testing_for_flakiness_in_cq.md).
