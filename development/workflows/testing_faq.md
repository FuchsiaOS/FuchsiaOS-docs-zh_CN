# Testing: Questions and Answers

You are encouraged to add your own questions (and answers) here!

[TOC]

## Q: How do I define a new unit test?

A: Use language-appropriate constructs, like GTest for C++. You can define a new
file if need be, such as:

(in a BUILD.gn file)
```code
executable("unittests") {
  output_name = "scenic_unittests"
  testonly = true
  sources = ["some_test.cc"],
  deps = [":some_dep"],
}
```

## Q: What ensures it is run?

A: An unbroken chain of dependencies that roll up to a config file under
`//<layer>/packages/tests/`, such as
[`//garnet/packages/tests/`](https://fuchsia.googlesource.com/garnet/+/master/packages/tests/).

For example:

`//garnet/lib/ui/scenic/tests:unittests`

is an executable, listed under the "tests" stanza of

`//garnet/bin/ui:scenic_tests`

which is a package, which is itself listed in the "packages" stanza of

`//garnet/packages/tests/scenic`

a file that defines what test binaries go into a system image.

Think of it as a blueprint file: a (transitive) manifest that details which
tests to try build and run.

Typically, one just adds a new test to an existing binary, or a new test binary to an existing package.

## Q: How do I run this unit test on a QEMU instance?

A: Start a QEMU instance on your workstation, and then *manually* invoke the unit test binary.

First, start QEMU with `fx run`.

In the QEMU shell, run `/system/test/scenic_unittests`. The filename is taken
from the value of "output_name" from the executable's build rule. All test
binaries live either in the `/system/test` directory if they are built with
`deprecated_system_image = true`, or in `/pkgfs/packages/<package_name>/0/test`
if they are defined in a [package](package_update.md).

Note Well! The files are loaded into the QEMU instance at startup. So after
rebuilding a test, you'll need to shutdown and re-start the QEMU instance to see
the rebuilt test. To exit QEMU, `dm shutdown`.

## Q: How do I run this unit test on my development device?

A: Either manual invocation, like in QEMU, **or** `fx run-test` to a running device.

Note that the booted device may not contain your binary at startup, but `fx
run-test` will build the test binary, ship it over to the device, and run it,
while piping the output back to your workstation terminal. Slick!

Make sure your device is running (hit Ctrl-D to boot an existing image) and
connected to your workstation.
If your target device is a QEMU emulator, then it needs networking to receive
the test package. Start your emulator with `fx run -N`.

From your workstation, `fx run-test scenic_unittests`. The argument to
`run-test` is the name of the test program as defined in `BUILD.gn`, for
instance:
`tests = [ { name = "scenic_unittests" } ]`.

You can automatically rebuild, install, and run your tests on every source file
change with `fx -i`. For instance: `fx -i run-test scenic_unittests`.

## Q: Where are the test results captured?

A: The output is directed to your terminal.

There does exist a way to write test output into files (including a summary JSON
file), which is how CQ bots collect the test output for automated runs.

## Q: How to disable a test? How to find and run disabled tests?

A: To temporary prevent a particular test to being run as part of a test suite,
you can mark it as disabled. Disabled tests are defined by having their name
prefixed with `DISABLED_`. One way to find them is therefore simply `git grep
DISABLED_`.

If running the test outputs `YOU HAVE 1 DISABLED TEST`, you can also pass the
following flags to find out which test is disabled:
`/system/test/scenic_unittests --gtest_list_tests --gtest_filter=*DISABLED_*`.

To force-run disabled tests:
`/system/test/scenic_unittests --gtest_also_run_disabled_tests`.

Alternatively, you may also want to disable a full test *binary* within a
package containing several test binaries. To do this, edit the `BUILD.gn` as
follows:
`tests = [ { name = "scenic_unittests", disabled = true } ]`.
As a result, `scenic_unittests` will be put in a `disabled` sub-directory of
`/system/test` or `/pkgfs/packages/<package_name>/0/test`, and will not be run
by the CQ system.

## Q: How do I run a bunch of tests automatically? How do I ensure all dependencies are tested?

A: Upload your patch to Gerrit and do a CQ dry run.

## Q: How do I run this unit test in a CQ dry run?

A: Clicking on CQ dry run (aka +1) will take a properly defined unit test and
run it on multiple bots, one for each build target (*x86-64* versus *arm64*, *release*
versus *debug*). Each job will have an output page showing all the tests that
ran.
