# Systems that use Fuchsia performance test results

This page lists the systems that consume results from Fuchsia's
performance tests.

*   **[Chromeperf]** (the Chrome Performance Dashboard): Chromeperf
    maintains a database of performance test results. Fuchsia Infra
    uploads results to that database.

    Chromeperf has three main features that are used by the Fuchsia
    project:

    *   **Post-commit regression alerting**: Chromeperf can detect
        performance regressions and report them by filing bugs in
        Fuchsia's Monorail issue tracker. Chromeperf is only able to
        detect regressions post-commit, i.e. after the CL that causes
        a regression has landed. Chromeperf only uses performance
        results data from CI builds and not from CQ builds.

        An example of a regression alert issue filed by Chromeperf is
        [fxbug.dev/114180](https://fxbug.dev/114180).

    *   **Web UI for graphing performance results**: Chromeperf's
        graphs show results by revision/time.

    *   **BigQuery database**: Chromeperf publishes BigQuery tables
        containing performance results data and details of regressions
        that Chromeperf has detected. This can be useful for doing
        further analysis and graphing of the data. These tables are
        not publicly accessible at the moment.

    Chromeperf is used by the Chrome project and is open
    source. However, currently all the results uploaded to Chromeperf
    from Fuchsia builds are not made publicly visible.

    Chromeperf is sometimes referred to as the "Catapult Performance
    Dashboard" in the Fuchsia codebase, because its code lives in the
    [Catapult project Git repo][catapult-repo].

    Chromeperf uses a "push" model: Fuchsia builds have a step that
    uploads performance results to Chromeperf.

    See [Chromeperf uploading and
    configuration](chromeperf_uploading_config.md) for a description
    of the upload code path and configuration.

*   **Per-build results summary pages**: Each Fuchsia build that runs
    performance tests successfully contains a link to a table of
    results from those tests. See "summary of perf test results" ->
    "stdout" on the build page.

*   **[Perfcompare try builder](perfcompare.md)**: Perfcompare is an
    optional try builder that can be used to measure the performance
    impact of a CL before landing it. It runs performance tests with
    and without the CL applied and compares their results to check for
    performance improvements and regressions.

*   **Culprit finder tools**: This system graphs and analyzes results
    from both CI and CQ builds. It is used as part of the performance
    regression triage process for identifying which CL introduced each
    regression.

    This system is currently Google-internal. Googlers can refer to
    the [Google-internal documentation][culprit-finder] for more
    details.

    This system uses a "pull" model: It enumerates Fuchsia builds
    using Buildbucket and fetches data from them.


[Chromeperf]: https://chromeperf.appspot.com/
[catapult-repo]: https://chromium.googlesource.com/catapult/
[culprit-finder]: https://goto.google.com/fuchsia-performance-culprit-finder
