# Host-target interaction tests

For the purposes of this doc "target" is the machine running a version of
Fuchsia being tested, and "host" is the machine running some stable OS. A
host-target interaction test runs on the host and interacts with the target.

## Declaration in GN

To declare a host-target interaction test in GN:

1.  Declare a test executable.
2.  Specify one or more [environments](/docs/concepts/testing/environments.md)
    that include Fuchsia (target) devices.
3.  Add a dependency between a
    [tests bundle](/docs/concepts/build_system/bundles.md) and the test
    executable, *specifying `host_toolchain`*.

For example:

```gn
# Doesn't have to be go_test. This is just an example.
go_test("an_hti_test") {
    ...
    # Declares that Fuchsia should be running in an emulator before this test
    # starts on the host.
    environments = [emu_env]
    ...
}

# This should be included in the transitive deps of some tests bundle.
group("tests") {
    testonly = true
    # Anything that depends on ":tests" will build the test as a host test.
    public_deps = [":an_hti_test($host_toolchain)"]
}
```

## Host test API

The continuous integration infrastructure (AKA "infra") and `fx test` start the
target and then invoke the test on the host.

[SL4F](/docs/concepts/testing/sl4f.md) is one way for the host to interact with
the target. The SL4F host libraries take responsibility for establishing a
connection with the target.

Tests that don't want to use SL4F can parse these environment variables and
handle their own communication with the host:

*   `FUCHSIA_IPV4_ADDR`: IPv4 address.
*   `FUCHSIA_IPV6_ADDR`: IPv6 address.
*   `FUCHSIA_SSH_KEY`: SSH key file path.
