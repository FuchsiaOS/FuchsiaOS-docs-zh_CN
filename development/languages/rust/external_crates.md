# External Rust crates

Fuchsia uses external Rust crates. External Rust crates are placed in
[`//third-party/rust_crates/vendor`][external-vendor]. This set of crates is
based on the dependencies listed in
[`//third_party/rust_crates/Cargo.toml`][external-cargo-toml].

Generally, adding or updating an external crate involves the following:

-  Calculating the dependencies of your external crate.

   Note: Pay attention to transitive dependencies. An external crate may
   depend on other external crates. List all the new crates that end up
   being brought in along with the originally intended crate, within the OSRB
   review.

-  Requesting Open Source Review Board (OSRB) approval.
   - For more information, see either the [Adding an external crate](#adding_an_external_crate)
   or [Updating an external crate](#updating_an_external_crate) section of this
   document.

-  Waiting to be granted OSRB approval.

   Warning: You must receive approval from the OSRB _before_ pushing a commit to
   Gerrit that adds an external crate. Do not request a code review for adding an
   external crate until you have approval from the OSRB.

-  Uploading the change for code review.

## Adding an external crate

If you don't find an existing crate that you want to use,
you may want to add an external crate to Fuchsia.

To add an external crate, do the following:

   1. Change to Fuchsia repository's base directory.

      For example, if your fuchsia directory is `~/fuchsia`, run the following
      command:

      ```posix-terminal
      cd ~/fuchsia
      ```

   1. Add an entry in
      [`third_party/rust_crates/Cargo.toml`][external-cargo-toml]
      for the crate that you want to add.
   1. Run the following command to download your desired crate(s) and calculate
      that crate's dependencies:

      Note: On Linux, you need to install `pkg-config` with your chosen package
      manager prior to running this command.

      ```posix-terminal
      fx update-rustc-third-party
      ```

      `fx update-rustc-third-party` downloads all of the crates listed in
      [`rust_crates/Cargo.toml`][external-cargo-toml] as well as their
      dependencies, places the downloaded crates in the `vendor` directory, and
      then updates `Cargo.toml` and `Cargo.lock`.

      You may need to provide additional configuration in a `[gn.package.<crate>]`
      section inside the `Cargo.toml` file. For crates that use a `build.rs`
      script, this configuration replaces the script, which is intentionally
      unsupported by the build system. This configuration is used by
      `cargo-gnaw`, which generates the GN rules from the Cargo.toml file.
      See [cargo-gnaw's README][cargo-gnaw-readme]
      for more details.

   1. Run the following command to perform a build test:

      ```posix-terminal
      fx set core.x64 && fx build
      ```

   1. Request OSRB approval by doing the following:
      - Create an issue with the
      [Open Source Review Board (OSRB) template](https://bugs.fuchsia.dev/p/fuchsia/issues/entry?template=Open+Source+Review+Board+).
      - In the issue, do the following:
         - Leave the **Owner** field blank.
            - The OSRB team meets regularly to review the issues.
            Please allow about a week for a response.
         - Specify all of the crates that you want to **add** (no need to list
           previously approved crates). Include the crate(s) that you're adding
           as well as the dependency crates identified after running
           `fx update-rustc-third-party`.
         - If there are any files in the source repository that are not included
           when vendored, specify those files in your issue to the OSRB. For
           example, font files that are only used for testing but are excluded
           when the crate is vendored would need to be included in an OSRB issue.

      Note: As part of the OSRB review, you may be asked to import only a subset
      of the files in an external crate. See
      [Importing a subset of files in a crate](#importing_a_subset_of_files_in_a_crate)
      for more information.

   1. If you receive OSRB approval, upload the change to Gerrit for review.
      Include the OSRB Issue ID number in the change.

      Warning: You must receive approval from the OSRB _before_ pushing a
      commit to Gerrit that adds an external crate. Do not request a code
      review for adding an external crate until you have approval from the OSRB.

   1. Add an [OWNER][owners-file]
   of the external rust crate directory as a code reviewer. You must get a
   `Code Review Label +2` from one of the repository's owners.

   1. If you have the ability to submit an approved change to the
      Commit Queue (CQ), [submit your change](https://gerrit-review.googlesource.com/Documentation/intro-user.html#submit)
      to merge that change into [third_party/rust_crates][external-crates].

      If you don't have the ability to submit an approved change, reply to your
      Gerrit change and request that one of the repository owners submit your
      change.

      For more information about the associated actions for each contributor
      role, see [Role matrix](/contribute/community/contributor-roles.md).

## Updating an external crate

Warning: You must receive approval from the OSRB _before_ pushing a commit to
Gerrit if updating an external crate changes the license or pulls in a new crate
as a dependency. Do not request a code review until you have approval from the
OSRB in these circumstances.

To update an external crate, do the following:

   1. Increase the patch number of the crate in
      [`third_party/rust_crates/Cargo.toml`][external-cargo-toml].

   1. Run the following command:

      ```posix-terminal
      fx update-rustc-third-party
      ```

      You may need to update or provide additional configuration
      in `[gn.package.<crate>]` sections inside the Cargo.toml file. For crates
      that use a `build.rs` script this configuration replaces the script,
      which is intentionally unsupported by of the build system. This
      configuration is used by `cargo-gnaw`, which generates the GN rules from
      the `Cargo.toml` file.
      See [cargo-gnaw's README][cargo-gnaw-readme] for more details.

   1. Run the following command to perform a build test:

      ```posix-terminal
      fx set core.x64 && fx build
      ```

   1. Request OSRB approval by doing the following:
      - Create an issue with the
      [Open Source Review Board (OSRB) template](https://bugs.fuchsia.dev/p/fuchsia/issues/entry?template=Open+Source+Review+Board+).
      - In the issue, do the following:
         - Leave the **Owner** field blank.
            - The OSRB team meets regularly to review the issues.
            Please allow about a week for a response.
         - Specify all of the crates that you want to add. Include the crate(s)
         that you're adding as well as the dependency crates identified after
         running `fx update-rustc-third-party`.
         - If there are any files in the source repository that are not included
         when vendored, specify those files in your issue to the OSRB. For
         example, font files that are only used for testing but are excluded
         when the crate is vendored would need to be included in an OSRB issue.

      Note: As part of the OSRB review, you may be asked to import only a subset
      of the files in an external crate. See
      [Importing a subset of files in a crate](#importing_a_subset_of_files_in_a_crate)
      for more information.

   1. If you receive OSRB approval, upload the change for review to
      Gerrit. Include the OSRB Issue ID number in the change.

   1. Add an [OWNER][owners-file]
   of the external rust crate repository as a code reviewer. You must get a
   `Code Review Label +2` from one of the repository's owners.

   1. If you have the ability to submit an approved change to the
      Commit Queue (CQ), [submit your change](https://gerrit-review.googlesource.com/Documentation/intro-user.html#submit)
      to merge that change into [third_party/rust_crates][external-crates].

      If you don't have the ability to submit an approved change, reply to your
      Gerrit change and request that one of the repository owners submit your
      change.

      For more information about the associated actions for each contributor
      role, see [Role matrix](/contribute/community/contributor-roles.md).

## Adding a new mirror

When actively contributing to an upstream repository or
maintaining a long-lived fork of a Fuchsia repository, it can be useful to
import a crate using a full git repository rather than Cargo's vendoring tools.
While this approach is useful, it has significant overhead compared to the
default flow and should be approached with caution.

   Warning: You must receive approval from the OSRB _before_ pushing a commit to
   Gerrit that adds or updates an external crate. Do not request a code
   review for adding or updating an external crate until you have approval
   from the OSRB.

1. Request the addition of a mirror on *fuchsia.googlesource.com*.
1. Add the mirror to the [Jiri manifest][jiri-manifest] for the Rust runtime.
1. Add a patch section for the crate to the workspace.
1. Run the update script.

## Importing a subset of files in a crate

In some cases, you may want to import only a subset of files in a crate. For
example, there may be an optional license in the
external repository that's incompatible with Fuchsia's license requirements.
Here's [an example](https://fuchsia-review.googlesource.com/c/fuchsia/+/369174)
OSRB review in which this happened.

To do this, you'll need to add the crate's files to `/third_party/rust_crates/forks`.

1. Follow the [instructions for adding an external crate](#adding_an_external_crate).
1. After running `fx update-rustc-third-party`, move the downloaded copy of your
   crate from `/third_party/rust_crates/vendor/<my_crate>` to
   `/third_party/rust_crates/forks/<my_crate>`.
1. Make the changes you need to make to the imported files.
1. Add a line to the `[patch.crates-io]` section of
   `/third_party/rust_crates/Cargo.toml` to point to your new crate:

   ```
   [patch.crates-io]
   ...
   my_crate = { path = "forks/<my_crate>" }
   ...
   ```
1. Re-run `fx update-rustc-third-party` and `fx build`.
1. Add a `/third_party/rust_crates/forks/<my_crate>/README.fuchsia` file which matches the format of
   other crates' `README.fuchsia`s there. See [/third_party/rust_crates/forks/README.md] for what it
   should contain.

## Unicode crates

If the project requires importing a new external crate to handle
functionality related to Unicode and internationalization, prefer crates from
the [UNIC project](https://crates.io/crates/unic){: .external} when available.

### Exempted non-UNIC crates

The following non-UNIC crates are already vendored and are exempted:

* `unicode-bidi`
* `unicode-normalization`
* `unicode-segmentation`
* `unicode-width`
* `unicode-xid`

### Rationale for standardization

UNIC crates have distinct advantages over other crates:

* UNIC crates are developed in a single repo, with shared common code and a
  single version scheme.

  * Independently developed crates do not share a common release schedule,
    versioning scheme, or adherence to any particular version of the Unicode
    standard.

* UNIC crates are generated from a consistent set of Unicode data files.

  * Each of the independent crates uses an arbitrary version and subset of
    the data. For example, different crates might have different assumptions
    about whether a particular code point is assigned, what its properties
    are, etc.

* The UNIC project is aiming for comprehensive feature coverage, to be like
  [ICU](http://site.icu-project.org/){: .external} for Rust. If the project
  succeeds, our dependencies on unrelated Unicode crates should be
  reduced over time.

## OWNERS files

`OWNERS` files are maintained for all of the external Rust crates to
indicate who is responsible for their reviews and updates. These files are
generated from a combination of build graph metadata and an explicit
override file.

### Running the tool

The tool discovers which build targets depend on a given crate, which means it
needs the metadata from the completion of a maximal "kitchen sink" build:

1. Include `//bundles/buildbot:core` and `//bundles:kitchen_sink` in your build
2. Run `fx build`
3. Run `fx update-rust-3p-owners --num-threads <NUM_THREADS>`. It's usually a good idea to limit
   the number of threads to 50% of available CPUs (see [#75382] for details).

[#75382]: https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=75382

### Adding overrides

Some crates have more users than can be relied upon to maintain
(see [Bystander effect]{: .external}). Others implement behavior specific to a
domain like security and we would prefer for a specific team to be responsible
for reviews of the code.

In these cases, add an entry to `//third_party/rust_crates/owners.toml` with
the path(s) to other `OWNERS` files to reference, then re-run the tool.
This replaces the reverse-dependency metadata ownership with the overridden
paths.

[Bystander effect]: https://en.wikipedia.org/wiki/Bystander_effect

### Update frequency

A member of the Rust on Fuchsia team is currently responsible for running the
tool on a regular cadence. See [https://fxbug.dev/73348](https://fxbug.dev/73348)
to track the process of automating updates to OWNERS files.

## Overriding locally

It can be useful to override a third party crate if you're contributing upstream
and want to run in-tree builds or tests. That can be achieved with the following
steps.

1. Clone (or symlink) the upstream repository under
   `third_party/rust_crates/forks/<my_crate>`.
1. Add the override to the `[patch.crates-io]` section in
   `third_party/rust_crates/Cargo.toml`.

```
[patch.crates-io]
my_crate = { path = "forks/<my_crate>" }
```
1. You must make sure that the version under the crate's `Cargo.toml` matches
   all references to that crate in `third_party/rust_crates/Cargo.toml`.
1. Run `fx update-rustc-third-party`.

## Troubleshooting
### Broken Config
After running `fx update-rustc-third-party`, if you encounter an error like
this:

```
Generating GN file from /$HOME/fuchsia/third_party/rust_crates/Cargo.toml
Error: GNaw config exists for crates that were not found in the Cargo
build graph:

library crate, package handlebars version 2.0.0-beta.2
library crate, package core-foundation-sys version 0.7.0
library crate, package pulldown-cmark version 0.6.0
library crate, package nix version 0.18.0
```

You can fix this issue commenting out your fuchsia target in `.cargo/config`:

```
[build]
...
target = "x86_64-fuchsia"
```

After commenting, it becomes:

```
[build]
...
# target = "x86_64-fuchsia"
```

This issue is being tracked [upstream](https://github.com/rust-lang/cargo/issues/8462){: .external}.


[external-crates]: /third_party/rust_crates/
[external-cargo-toml]: /third_party/rust_crates/Cargo.toml
[external-vendor]: /third_party/rust_crates/vendor
[cargo-gnaw-readme]: /tools/cargo-gnaw/README.md
[osrb-process]: /contribute/governance/policy/osrb-process.md#process_for_adding_external_code_to_new_repositories
[jiri-manifest]: https://fuchsia.googlesource.com/manifest/+/main/runtimes/rust "Jiri manifest"
[owners-file]: /third_party/rust_crates/OWNERS
