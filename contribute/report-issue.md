# Report an issue

Filing issues is a great way to contribute to the Fuchsia project.
You can file an issue using Monorail, Google’s issue tracking tool for open
source projects.

Note: You need a Google account to file an issue in Monorail.

## When to file an issue

Monorail issues are used to track and suggest the following
types of changes:

  + Minor changes or suggestions to Fuchsia's code and documentation.
  + Reporting security issues
  + Proposing open source third-party code to be included in Fuchsia

For proposing code changes that would affect Fuchsia significantly, see
[Propose significant code changes](#significant-code-changes).

As an open source contributor, you can file a issue with the
following templates:

<table>
  <tr>
   <td><strong>Template</strong>
   </td>
   <td><strong>Purpose</strong>
   </td>
  </tr>
  <tr>
   <td>Fuchsia Default
   </td>
   <td>Alert the Fuchsia team that there is a bug related to using or building Fuchsia.
   </td>
  </tr>
  <tr>
   <td>Fuchsia Security external bug report
   </td>
   <td>Report a security issue related to Fuchsia.
   </td>
  </tr>
  <tr>
   <td>Fuchsia.dev Documentation
   </td>
   <td>Alert the Documentation team that there is an issue related to fuchsia.dev.
   </td>
  </tr>
  <tr>
   <td>Open Source Review Board
   </td>
   <td>Propose open source external code to be included in Fuchsia. Review the <a href="/docs/contribute/governance/policy/osrb-process.md">Open Source Review Board (OSRB) process</a>
     before filing an issue with the Open Source Review Board (OSRB).
   </td>
  </tr>
  <tr>
   <td>Report Community Abuse
   </td>
   <td>Alert Community Managers about any violations of the <a href="/CODE_OF_CONDUCT.md">Code of Conduct</a>
 that you may have experienced in the Fuchsia community.
   </td>
  </tr>
</table>

## How to file an issue

To file an issue in Fuchsia, do the following:

1. Go to [https://bugs.fuchsia.dev](https://bugs.fuchsia.dev).
1. Click **New Issue.**
1. Select one of the following templates from the **Template** dropdown:
    1. Fuchsia Default
    1. Fuchsia Security external bug report
    1. Fuchsia.dev Documentation
    1. Open Source Review Board
    1. Report Community Abuse
1. Complete the questions associated with the template you selected.
1. Click **Submit Issue**.

## Issue reporting guidelines

When you create an issue, include the following:

*   A description of the issue that you’re experiencing, including the
    expected behavior.
*   The steps necessary to reproduce the issue.
*   (Optional) Screenshots
*   (Optional) If a bug can be identified through a unit test,
    then create a simplified test and attach it to the issue.

## Issue resolution

After you have filed an issue, a team of triagers will route your issue to the
appropriate team. That team is responsible for prioritizing, assigning, and
responding to your issue.

## Propose significant code changes {:#significant-code-changes}

There are some instances where filing an issue would not be the best course of
action. Here are a few examples of alternative courses of action:

<table>
  <tr>
   <td><strong>Scenario</strong>
   </td>
   <td><strong>Process</strong>
   </td>
  </tr>
  <tr>
   <td>Proposing a change that would significantly affect the Fuchsia API.
   </td>
   <td>Create an API Design Document. For more information, see <a href="/docs/contribute/governance/api_council.md">Fuchsia API Council Charter</a>.
   </td>
  </tr>
  <tr>
   <td>Proposing a change that would affect a large part of the Fuchsia codebase or the technical direction of the Fuchsia project.
   </td>
   <td>Propose a Request for Comments (RFC). For more information on RFCs, see <a href="/docs/contribute/governance/rfcs/create_rfc.md">Create an RFC</a>.
   </td>
  </tr>
</table>
