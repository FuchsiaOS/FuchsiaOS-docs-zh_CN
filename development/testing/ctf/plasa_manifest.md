# Platform Surface Area Manifest

*Design document*

## Motivation

Provide an exhaustive list of platform surface elements for computing platform
surface test coverage for the CTF effort.

## Glossary

(References to already defined glossary items are typeset in *cursive* type.)

**Platform surface element.**

> A named smallest component of Fuchsia's public [API][api] or [ABI][abi].

**Platform surface area.**

> An exhaustive collection of platform surface elements.

**Platform surface fragment.**

> A subset of the *platform surface area*.

**Platform surface area manifest.**

> A file containing pointers to the *platform surface fragment* files.

## Requirements

1. Uniquely identify and enumerate useful platform surface elements in a given
   platform surface view.

2. Efficiently generate and traverse potentially many surface elements based on
   a platform surface view.

3. Allow reuse of already existing platform surface fragments across different
   platform surface kinds.

4. Allow incremental inclusion of platform surface fragments as they are
   defined and built.

5. Cooperate with our build system.

## Why now?

For the CTF test coverage dashboard to be universally useful, it should show
the covered and uncovered parts of the platform surface.

1. The covered parts of the platform surface allow us to make certain
   correctness and safety guarantees about the underlying platform.

2. The uncovered parts of the platform surface guide CTF test authors to the
   parts of the API surface that are not adequately covered by CTF tests.

The platform surface area manifest complements (1) and fulfills (2).

## Stakeholders

*   [CTF][cts] maintainers
*   CTF test authors

## Design

The platform surface area is described in a collection of platform surface area
fragment files, with a platform surface manifest file which in turn lists the
location of all such files in Fuchsia's build output directory
(`$FUCHSIA_DIR/out/default` or similar).  By convention, the manifest file is
called `manifest.plasa.json`, while the fragment files have names matching the
file path pattern `*.fragment.plasa.json`, as seen in the diagram below.

![The logical structure of the plasa files](plasa_manifest_structure.dot.png "A graph showing the root node marked manifest.plasa.json and two edges one pointing at file_1.fragment.plasa.json, other pointing at file_2.fragment.plasa.json")

A number of reasons influenced the choice of this hub-and-spoke manifest layout:

1. Allows different subsystems to evolve their own custom platform surface area
   formats.

2. Allows generating the platform surface area fragments concurrently. This
   avoids platform-wide build bottlenecks from large file merges.

3. Allows generating the platform surface area manifest file using a
   lightweight [metadata propagation][mp] mechanism inherent in our build system.

4. Allows easy extension of the platform surface area metadata if needed.

[mp]: https://gn.googlesource.com/gn/+/main/docs/reference.md#var_metadata

The tooling needs to be cognizant of this file layout and ensure that all files
are properly handled.  For those tools that require single files, it should be
easy to write a script that will be able to merge the files.

### Manifest file format

The manifest file format is a sequence of JSON-formatted objects.  Each object
contains a reference to a fragment file, and a declaration of the type the
fragment is expected to be. The manifest is a sequence of items as this is a
format that is particularly easy to generate by our build system, while still
being able to describe the content in sufficient detail.

* Sequence of...

  * `kind` (Enum): The kind of the fragment in question. Each distinct kind may
      be interpreted differently. At the time of this writing, the possible
      values are:

    * `api_cc`: denotes the fragment in question is following the C++ API
        fragment format

    * `api_fidl`: denotes the fragment in question is following the FIDL API
        summary format (per [RFC-0076][rfc76])

  * `file` (String): The fully qualified label of the fragment.

#### Sample
```
[
  {
    "file": "//out/workstation_eng.qemu-x64/gen/sdk/lib/fdio/fdio.fragment.plasa.json",
    "kind": "api_cc",
  },
  {
    "file": "//out/workstation_eng.qemu-x64/gen/sdk/lib/stdcompat/stdcompat.fragment.plasa.json",
    "kind": "api_cc",
  },
  {
    "file": "//out/workstation_eng.qemu-x64/gen/sdk/lib/fit/fit.fragment.plasa.json",
    "kind": "api_cc",
  }
]
```
### C++ API fragment file format

The fragment file format is currently a sequence of platform element items.
The format may be expanded as need arises.

* JSON object of...

  * `items` (Sequence) of...

    * `name` (String): The name of the platform surface element.

    * `file` (optional[String]): The file path to the file where the element is
        located.

    * `line` (optional[Integer]): The line of `file` where the element is
        defined.

#### Sample

```
{
  "items": [
        {
            "name": "fit::deferred_action::deferred_action<T>",
            "file": "gen/sdk/lib/fit/../../../../../../sdk/lib/fit/include/lib/fit/defer.h",
            "line": 81
        },
        {
            "name": "fit::deferred_action::operator bool",
            "file": "gen/sdk/lib/fit/../../../../../../sdk/lib/fit/include/lib/fit/defer.h",
            "line": 43
        }
  ]
}
```
## Documentation & examples

We may want to update the CTF documentation to include the existence of the
platform surface manifests.

*   [The rule that generates the manifest][l1].
*   [The template `plasa_artifacts.gni` that generates the fragment files][l2].
*   [Usage of the template `plasa_artifacts.gni` throughout the source tree][l3].

## Backwards compatibility

This is new functionality which does not have a compatibility baseline to
maintain.

## Security & privacy

This design is neutral with respect to security and privacy as follows:

1. All platform surface fragments that are public knowledge are likely not
   subject to security and privacy constraints, by the nature of their public
   visibility.

2. Any private platform surface fragments can be defined in a manifest that is
   outside of the public view.  An overall platform surface manifest can be
   formed by including both the privately and the publicly accessible fragments
   into a single manifest, if it becomes needed.

## Future work

Future work will see more elements of the platform surface covered by the
platform surface area manifest:

1. Represent command line utilities and flags in the manifest.

2. Represent other Plasa elements in the manifest.

[abi]: /docs/concepts/packages/system.md#abi_surfaces
[api]: /docs/contribute/governance/rfcs/0002_platform_versioning.md#implementation
[cts]: /docs/development/testing/ctf/overview.md
[l1]: https://cs.opensource.google/fuchsia/fuchsia/+/main:sdk/BUILD.gn;l=53;drc=0590a9df6e18abbaafb719c42cc2ef59abb7fdb6
[l2]: https://cs.opensource.google/fuchsia/fuchsia/+/main:sdk/ctf/plasa/plasa_artifacts.gni;l=8
[l3]: https://cs.opensource.google/search?q=plasa_artifacts.gni&ss=fuchsia%2Ffuchsia
[rfc76]: /docs/contribute/governance/rfcs/0076_fidl_api_summaries.md

