# Python testing

To define a host-side python test that can be run by `fx`, CI and CQ:

*   Use the [python_host_test](/build/python/python_host_test.gni) GN template.
*   Ensure some `group("tests")` depends on the `python_host_test` rule,
    and specify the `($host_toolchain)` in the dependency.

[Here](/sdk/cts/build/scripts/BUILD.gn) is an example BUILD.gn.
