# Chromeperf uploading and configuration

This page describes how performance results from Fuchsia builds are
uploaded to Chromeperf, and how Fuchsia's use of Chromeperf is
configured.

Some parts of this are implemented in Google-internal code and are not
described fully here. Googlers can refer to the [Google-internal
version of this documentation][internal-doc] for more details.

## Code path for uploading

The code path for uploading to Chromeperf is somewhat complicated
because it spans several locations:

*   **Builder config:** The file `catapult.star` contains a mapping
    that specifies which builders should upload performance results to
    Chromeperf and allows the names to be remapped. This lives in the
    Google-internal version of `integration.git`, in `infra/config/`.

    *   An entry in `catapult.star` has the effect of setting the
        nested fields `catapult_dashboard_master` and
        `catapult_dashboard_bot` in the properties of the
        corresponding builder. The properties are stored in generated
        files that are checked in to `integration.git`.
    *   This enables uploading to Chromeperf only for CI builds.
    *   This mapping was implemented in
        [fxbug.dev/49592](https://fxbug.dev/49592).

*   **Subbuild:** The Infra recipes code sets various environment
    variables based on the input properties, including
    `CATAPULT_DASHBOARD_MASTER` and `CATAPULT_DASHBOARD_BOT`. See
    [`testing_requests/api.py`](https://fuchsia.googlesource.com/infra/recipes/+/HEAD/recipe_modules/testing_requests/api.py).

    *   This is done in the subbuild (that is, the Swarming task that
        builds Fuchsia). The environment variables are put into the
        shard description.
    *   The recipes code may modify the "master" and "bot" names to
        handle branches (e.g. release branches). Otherwise the names
        are passed through unmodified from the input properties.

*   **Test shard:** In a test shard's Swarming task, the following
    happens:

    *   Each test may generate `*.fuchsiaperf.json` files.
    *   Tests that generate `fuchsiaperf` files pass them to the
        [`performance.dart`][performance.dart] library. That does two
        things with each `fuchsiaperf` file:
        *   It copies the `fuchsiaperf` file to the shard's output
            directory. `fuchsiaperf` files in this location are used
            by the per-build results summary pages (including for
            perfcompare builds).
        *   It runs [`catapult_converter`][catapult_converter] on the
            `*.fuchsiaperf.json` file to produce a `*.catapult_json`
            file, which is also copied to the shard's output
            directory. This file is in the format accepted by
            Chromeperf. This step uses the `CATAPULT_DASHBOARD_*`
            environment variables mentioned above.

*   **Upload step:** A later recipe step picks up the
    `*.catapult_json` files and uploads them to Chromeperf.

    *   This uses an [upload tool] written in Go.
    *   The uploading uses some credentials that are made available to
        CI builds on Fuchsia Infra.
    *   Upload errors are sometimes reported as a failure in this
        recipe step, but are sometimes only reported in Chromeperf's
        logs, which are not publicly visible.

When uploading to Chromeperf is disabled for a builder (which is the
case for all CQ builders and for CI builders not listed in
`catapult.star`), the `catapult_*` input properties are not set, and so
the `CATAPULT_*` environment variables do not get set.
`performance.dart` still runs `catapult_converter`, in order to check
that the conversion succeeds, but it produces `*.catapult_json_disabled`
files rather than `*.catapult_json` files.

## Limitations and hazards

*   **"Push" model:** Chromeperf uploading uses a "push" model. Each
    CI build has the ability to upload results under any builder name,
    test name, etc. within Chromeperf.

    It is important to be careful about getting these uploads right
    because mistakes could potentially produce confusing results in
    the Chromeperf dashboard, and because mistakes cannot easily be
    corrected -- there is no straightforward way to remove bad data
    from Chromeperf's database. Furthermore, Chromeperf's namespaces
    are shared with other projects, including Chrome.

*   **Testability:** There is currently no good way to test uploading
    to Chromeperf. There is no well-defined way to set up a test
    instance of Chromeperf, and Fuchsia developers don't have access
    to the credentials for uploading to the production instance except
    via Fuchsia CI builds. This means that the only way to test
    changes to `catapult_converter`'s output is to land a change and
    check that uploading continues to work in CI.

*   **Duplicates:** If two tests output metrics with the same name
    (i.e. same test name and test suite name), this mistake will not
    be caught. Instead, this will probably appear as two data points
    on the same Chromeperf graph, both linking to the same Fuchsia
    build. This is because each `fuchsiaperf`/`catapult` file is
    processed separately, so there is not currently a suitable step
    for rejecting or merging duplicates.

*   **Alerting:** There is currently no alerting set up to warn if
    Fuchsia's Chromeperf uploads stop working.

## Configuration for regression alerting

Chromeperf has a set of files in Chrome's Google-internal infra repo
for configuring regression alerting. This includes a file for Fuchsia,
`fuchsia-perf.cfg`.


[internal-doc]: <https://goto.google.com/fuchsia-chromeperf-uploading>
[performance.dart]: /sdk/testing/sl4f/client/lib/src/performance.dart
[catapult_converter]: /src/testing/catapult_converter/
[upload tool]: <https://fuchsia.googlesource.com/infra/infra/+/HEAD/cmd/catapult/>
