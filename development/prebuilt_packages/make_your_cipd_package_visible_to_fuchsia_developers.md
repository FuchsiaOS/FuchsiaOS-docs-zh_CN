# Make your CIPD package visible to Fuchsia developers

After
[uploading a prebuilt package to CIPD](/docs/development/prebuilt_packages/publish_prebuilt_packages_to_cipd.md),
to make it available to Fuchsia developers, you need to update a Jiri manifest
listed under `${FUCHSIA_DIR}/integration/`.

Do the following:

*   Locate the manifest file in `${FUCHSIA_DIR}/integration/`.

*   Determine the Jiri checkout path. This should be under
    `${FUCHSIA_DIR}/prebuilt/` with a path reflecting the CIPD one.

    For example, `${FUCHSIA_DIR}/prebuilt/third_party/ninja/linux-x64/` is used
    for the package at CIPD path `fuchsia/third_party/ninja/linux-amd64` (notice
    that they use different conventions for the CPU architecture).

*   Add or update the Jiri manifest entry appropriately (see other existing
    entries for examples).

    For instance, the following entry is for the QEMU emulator binary:

    <pre>
    &lt;package name="fuchsia/third_party/qemu/${platform}"
             version="git_revision:533bf2563d0213a7e002d9fcde75519d30ffa70f,1"
             platforms="linux-amd64,linux-arm64,mac-amd64"
             path="prebuilt/third_party/qemu/&#0123{.OS}}-&#0123{.Arch}}"/&gt;
    </pre>

    Notice the following in the example above:

    *   The `name` attribute points to the CIPD package path where `${platform}`
        expands to a string matching the CIPD-compatible host platform name (for
        example, `linux-amd64`).

    *   The `version` attribute points to a CIPD tag for the package's revision
        to download.

    *   The `path` attribute points to the checkout path under `${FUCHSIA_DIR}`.
        This attribute uses a special pattern that expands to a
        Fuchsia-compatible system and CPU architecture names for developers or
        infrastructure bot machines (for example, `linux-x64`).

    *   The `platforms` attribute is optional and restricts the list of
        supported build platforms (Jiri doesn't download this specific package
        on unlisted systems).

*   If necessary, update the Jiri lock files for the Fuchsia project.

*   Create a new branch under `integration` (or whatever directory contains your
    Jiri manifest for prebuilts) that includes your manifest change (and
    `jiri.lock` changes if required) and upload it to Gerrit for review.

