# Testing FIDL protocols

## Prerequisites

This tutorial builds on the [HLCPP getting started tutorials][overview].

## Overview

This tutorial walks through the process of writing a test for the
`Echo.EchoString` method. This tutorial shows you how to use the two utilities
available for testing FIDL protocols implemented in HLCPP:

* The `gtest` test loop fixture `sys::testing::ComponentContextProvider`.
* The `fidl_test_base.h` file provided by the HLCPP bindings

If you want to write the code yourself, delete the following directories:

```
rm -r examples/fidl/hlcpp/testing/*
```

The test will be written in `examples/fidl/hlcpp/testing/main.cc`.

## Set up dependencies

To set up dependencies:

1. Include the libraries that are needed for the test:

  ```cpp
  {%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/hlcpp/testing/main.cc" region_tag="includes" %}
  ```

2. Add a build rule for the test in `examples/fidl/hlcpp/testing/BUILD.gn`:

  ```gn
  {%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/hlcpp/testing/BUILD.gn" %}
  ```

## Create a server implementation

To create a server implementation:

1. Add an implementation for the `Echo` protocol that is tested:

   ```cpp
   {%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/hlcpp/testing/main.cc" region_tag="impl" %}
   ```

   Rather than inheriting from `fuchsia::examples::Echo`, this implementation
   inherits from the [corresponding test base class][test-base]. This means that
   the implementation only needs to override the methods that are being tested
   (in this case, `EchoString`), as well as the `NotImplemented_` method, which
  is called if any of the request handler methods that are not overriden get
  called.

1. Create a test class that wraps the logic of publishing the echo protocol:

   ```cpp
   {%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/hlcpp/testing/main.cc" region_tag="wrapper" %}
   ```

   This is similar to the code that is explained in the
   [server tutorial][server-tut], but the `fidl::Binding` is owned by the class.
   This makes the binding's destructor get called when the class is destroyed.
   This enables the code to publish the echo protocol on each test case given
   a new instance of the test component context.

## Implement the text fixture class

```cpp
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/hlcpp/testing/main.cc" region_tag="fixture" %}
```

The test fixture does the following:

* Holds an instance of a `ComponentContextProvider`. Each test, it uses it to
  create a new test context, and binds the Echo implementation to it using the
  `EchoServerInstance` class.
* Provides a `GetProxy()` method initializes a proxy to the current test
  component context and returns it.

## Add tests

This is an example test that can you can write with the text fixture:

```cpp
{%includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/fidl/hlcpp/testing/main.cc" region_tag="test" %}
```

## Run the test

To run the test:

1. Configure your GN build to include the test:

   ```
   fx set core.x64 --with //examples/fidl/hlcpp/testing

   ```
1. Run the test:

   ```
   fx test -vo example-hlcpp-protocol-test
   ```

You should see the test output indicating a success.

## Summary

* The `gtest::TestLoopFixture` removes the need for boilerplate async loop
  setup code. Each test case can simply call `RunLoopUntilIdle()` instead of
  manually managing an `async::Loop`.
* The `ComponentContextProvider` makes it easy to mock the component context
  during a test. This is useful to e.g. provide specific capabilities to the
  a component.
* The HLCPP bindings test scaffolding provides a test base for each protocol
  class that has a dummy implementation for each method. This allows tests
  to only implement the methods under test.

<!-- xrefs -->
[test-base]: /docs/reference/fidl/bindings/hlcpp-bindings.md#test-scaffolding
[server-tut]: /docs/development/languages/fidl/tutorials/hlcpp/basics/server.md
[overview]: /docs/development/languages/fidl/tutorials/hlcpp/README.md
