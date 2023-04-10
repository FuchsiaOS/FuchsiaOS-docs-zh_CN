# Inspect codelab

This document contains the codelab for Inspect in C++, Dart and Rust.

The code is available at:

* [//examples/diagnostics/inspect/codelab/cpp][inspect-cpp-codelab].
* [//examples/diagnostics/inspect/codelab/rust][inspect-rust-codelab].
* [//examples/diagnostics/inspect/codelab/dart][inspect-dart-codelab].

This codelab is organized into several parts, each with their own
subdirectory. The starting point for the codelab is part 1,
and the code for each part contains the solution for the previous parts.

* [C++ Part 1][cpp-part1]
* [Rust Part 1][rust-part1]
* [Dart Part 1][dart-part1]

Note: For Rust we also have an ergonomic library with a higher level API:
[fuchsia-inspect-derive][fuchsia-inspect-derive]. However, it's recommended to understand the
concepts explained in this codelab before using that other library.

When working on this codelab, you may continue adding your solutions to
"part\_1", or you may skip around by building on the existing solutions.

## Prerequisites

Set up your development environment.

This codelab assumes you have completed [Getting Started](/docs/get-started/README.md) and have:

1. A checked out and built Fuchsia tree.
2. A device or emulator (`ffx emu`) that runs Fuchsia.
3. A workstation to serve components (`fx serve`) to your Fuchsia device or emulator.

To build and run the examples in this codelab, add the following arguments
to your `fx set` invocation:

* {C++}

   Note: Replace `core.x64` with your preferred product and board configuration.

   ```
   fx set core.x64 \
   --with //examples/diagnostics/inspect/codelab/cpp \
   --with //examples/diagnostics/inspect/codelab/cpp:tests
   ```

* {Rust}

   Note: Replace `core.x64` with your preferred product and board configuration.

   ```
   fx set core.x64 \
   --with //examples/diagnostics/inspect/codelab/rust \
   --with //examples/diagnostics/inspect/codelab/rust:tests
   ```

* {Dart}

   Note: Replace `workstation.x64` with your preferred product and board configuration.

   ```
   fx set core.x64
       --with //examples/diagnostics/inspect/codelab/dart \
       --with //examples/diagnostics/inspect/codelab/dart:tests \
       --with-base //src/dart \
       --args='core_realm_shards += [ "//src/dart:dart_runner_core_shard" ]'
   ```

## Part 1: A buggy component

There is a component that serves a protocol called [Reverser][fidl-reverser]:

```fidl
{% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/fidl/reverser.test.fidl" region_tag="reverser_fidl" adjust_indentation="auto" %}
```

This protocol has a single method, called `Reverse`, that simply reverses
any string passed to it. An implementation of the protocol is provided,
but it has a critical bug. The bug makes clients who attempt to call
the `Reverse` method see that their call hangs indefinitely. It is up to
you to fix this bug.

### Run the component

There is a client application that will launch the Reverser component and send the rest of its
command line arguments as strings to Reverse:


1. See usage

   Depending on the part of the codelab you wish to run, you'd launch the
   `client_i` component, where `i` is a number in range [1, 5]. For example, to
   launch the client talking to the reverser from part 2 of the codelab:

   * {C++}

      ```
      ffx component run /core/ffx-laboratory:client_part_2 fuchsia-pkg://fuchsia.com/inspect_cpp_codelab#meta/client_part_2.cm
      ```

   * {Rust}

      ```
      ffx component run /core/ffx-laboratory:client_part_2 fuchsia-pkg://fuchsia.com/inspect_rust_codelab#meta/client_part_2.cm
      ```

   * {Dart}

      ```
      ffx component run /core/ffx-laboratory:client_part_2 fuchsia-pkg://fuchsia.com/inspect_dart_codelab#meta/client_part_2.cm
      ```

2. Run part 1 code, and reverse the string "Hello"

   * {C++}

      <!-- TODO(fxbug.dev/88383): this applies to all languages once Dart supports args -->
      To specify just the single string "Hello" modify the `program.args` section of
      the [common.shard.cml][cpp-common-cml], build and run the following:

      ```
      ffx component run /core/ffx-laboratory:client_part_1 fuchsia-pkg://fuchsia.com/inspect_cpp_codelab#meta/client_part_1.cm
      ```

      To see the command output take a look at the logs:

      ```
      ffx log --tags inspect_cpp_codelab
      ```

      This command prints some output containing errors.

   * {Rust}

      <!-- TODO(fxbug.dev/88383): this applies to all languages once Dart supports args -->
      To specify just the single string "Hello" modify the `program.args` section of
      the [common.shard.cml][cpp-common-cml], build and run the following:

      ```
      ffx component run /core/ffx-laboratory:client_part_1 fuchsia-pkg://fuchsia.com/inspect_rust_codelab#meta/client_part_1.cm
      ```

      To see the command output take a look at the logs:

      ```
      ffx log --tags inspect_rust_codelab
      ```

      We see in the logs that the component got the "Hello" as input, but we
      don't see the correct reversed output.

   * {Dart}

      <!-- TODO(fxbug.dev/88383): this applies to all languages once Dart supports args -->
      To specify just the string "Hello" modify the `args` variable in the
      [client main][dart-client-main].

      ```
      ffx component run /core/ffx-laboratory:client_part_1 fuchsia-pkg://fuchsia.com/inspect_dart_codelab#meta/client_part_1.cm
      ```

      To see the command output take a look at the logs:

      ```
      ffx log --tags inspect_dart_codelab
      ```

      We see in the logs that the component got the "Hello" as input, but we
      don't see the reversed output.

   As you can see in the log the reverser doesn't work properly.

3. Try running the client with more arguments:

   * {C++}

      <!-- TODO(fxbug.dev/88383): this applies to all languages once Dart supports args -->
      Add the string "World" to the `program.args` section of the
      [common.shard.cml][cpp-common-cml]:

      ```json5
      {
          program: {
              args: [
                  "Hello",
                  "World",
              ],
          },
      }
      ```

      Build and run the following:

      ```
      ffx component run --recreate /core/ffx-laboratory:client_part_1 fuchsia-pkg://fuchsia.com/inspect_cpp_codelab#meta/client_part_1.cm
      ```

   * {Rust}

      <!-- TODO(fxbug.dev/88383): this applies to all languages once Dart supports args -->
      Add the string "World" to the `program.args` section of the
      [common.shard.cml][rust-common-cml]:

      ```json5
      {
          program: {
              args: [
                  "Hello",
                  "World",
              ],
          },
      }
      ```

      Build and run the following:

      ```
      ffx component run --recreate /core/ffx-laboratory:client_part_1 fuchsia-pkg://fuchsia.com/inspect_rust_codelab#meta/client_part_1.cm
      ```

   * {Dart}

      <!-- TODO(fxbug.dev/88383): this applies to all languages once Dart supports args -->
      Add the string "World" to the `args` list in the [client main][dart-client-main].

      ```json5
      final args = ["Hello", "World"];
      ```

      Build and run the following:

      ```
      ffx component run --recreate /core/ffx-laboratory:client_part_1 fuchsia-pkg://fuchsia.com/inspect_dart_codelab#meta/client_part_1.cm
      ```

   We can see that the component printed the first input, but we don't see the
   expected output and also no second input.

You are now ready to look through the code to troubleshoot the issue.

### Look through the code

Now that you can reproduce the problem, take a look at what the client is doing:

* {C++}

   In the [client main][cpp-client-main]:

   ```cpp
   {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/cpp/client/main.cc" region_tag="reverse_loop" adjust_indentation="auto" %}
   ```

* {Rust}

   In the [client main][rust-client-main]:

   ```rust
   {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/rust/client/src/main.rs" region_tag="reverse_loop" adjust_indentation="auto" %}
   ```

* {Dart}

  In the [client main][dart-client-main]:

  ```dart
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/dart/client/lib/main.dart" region_tag="reverse_loop" adjust_indentation="auto" %}
  ```


In this code snippet, the client calls the `Reverse` method but never
seems to get a response. There doesn't seem to be an error message
or output.

Take a look at the server code for this part of the
codelab. There is a lot of standard component setup:

* {C++}

   In the [part 1 main][cpp-part1-main]:

   - Logging initialization

     ```cpp
     {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/cpp/part_1/main.cc" region_tag="init_logger" adjust_indentation="auto" %}
     ```

   - Creating an asynchronous executor

     ```cpp
     {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/cpp/part_1/main.cc" region_tag="async_executor" adjust_indentation="auto" %}
     ```

   - Serving a public service

     ```cpp
     {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/cpp/part_1/main.cc" region_tag="serve_outgoing" adjust_indentation="auto" %}
     ```

* {Rust}

   In the [part 1 main][rust-part1-main]:

   - Logging initialization

     ```rust
     {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/rust/part_1/src/main.rs" region_tag="init_logger" adjust_indentation="auto" %}
     ```

   - ServiceFs initialization

     ```rust
     {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/rust/part_1/src/main.rs" region_tag="servicefs_init" adjust_indentation="auto" %}
     ```

   - ServiceFs collection

     ```rust
     {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/rust/part_1/src/main.rs" region_tag="servicefs_collect" adjust_indentation="auto" %}
     ```

   - Serving a public service

     ```rust
     {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/rust/part_1/src/main.rs" region_tag="serve_service" adjust_indentation="auto" %}
     ```

* {Dart}

   In the [part 1 main][dart-part1-main]:

   - Logging initialization

     ```dart
     {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/dart/part_1/lib/main.dart" region_tag="init_logger" adjust_indentation="auto" %}
     ```

   - Serving a public service

     ```dart
     {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/dart/part_1/lib/main.dart" region_tag="serve_service" adjust_indentation="auto" %}
     ```

See what the reverser definition is:

* {C++}

   In [reverser.h][cpp-part1-reverser-h]:

   ```cpp
   {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/cpp/part_1/reverser.h" region_tag="reverser_h" adjust_indentation="auto" %}
   ```

   This class implements the `Reverser` protocol. A helper method called
   `CreateDefaultHandler` constructs an `InterfaceRequestHandler` that
   creates new `Reverser`s for incoming requests.

* {Rust}

   In [reverser.rs][rust-part1-reverser]:

   ```rust
   {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/rust/part_1/src/reverser.rs" region_tag="reverser_def" adjust_indentation="auto" %}
   ```

   This struct serves the `Reverser` protocol. The `ReverserServerFactory` (will make more sense
   later) constructs a `ReverserServer` when a new connection to `Reverser` is established.

- {Dart}

   In [reverser.dart][dart-part1-reverser]:

   ```dart
   {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/dart/part_1/lib/reverser.dart" region_tag="reverser_impl" adjust_indentation="auto" %}
   ```

   This class implements the `Reverser` protocol. A helper method called `getDefaultBinder` returns
   a closure that creates new `Reverser`s for incoming requests.


### Add Inspect

Now that you know the code structure, you can start to instrument the
code with Inspect to find the problem.

Note: [Inspect](/docs/development/diagnostics/inspect/README.md) is a powerful instrumentation feature for
Fuchsia Components. You can expose structured information about the component's state to diagnose
the problem.

You may have previously debugged programs by printing or logging. While
this is often effective, asynchronous Components that run persistently
often output numerous logs about their internal state over time. This
codelab shows how Inspect provides snapshots of your component's current
state without needing to dig through logs.

1. Include Inspect dependencies:

   * {C++}

      In [BUILD.gn][cpp-part1-build]:

      ```
      {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/cpp/part_2/BUILD.gn" region_tag="part_1_solution_build_dep" adjust_indentation="auto" %}
      ```

   * {Rust}

      In [BUILD.gn][rust-part1-build] in `deps` under `rustc_binary("bin")`:

      ```
      {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/rust/part_2/BUILD.gn" region_tag="part_1_solution_build_dep" adjust_indentation="auto" %}
      ```

   * {Dart}

     In [BUILD.gn][dart-part1-build] in `deps` under `dart_library("lib")` and
     `dart_component("bin")`:

     ```
     {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/dart/part_2/BUILD.gn" region_tag="part_1_solution_build_dep" adjust_indentation="auto" %}
     ```

2. Initialize Inspect:

   * {C++}

      In [main.cc][cpp-part1-main]:

      ```cpp
      {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/cpp/part_2/main.cc" region_tag="part_1_include_inspect" adjust_indentation="auto" %}
      {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/cpp/part_2/main.cc" region_tag="part_1_init_inspect" adjust_indentation="auto" %}
      ```


   * {Rust}

      In [main.rs][rust-part1-main]:

      ```rust
      {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/rust/part_2/src/main.rs" region_tag="part_1_use_inspect" adjust_indentation="auto" %}
      {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/rust/part_2/src/main.rs" region_tag="part_1_serve_inspect" adjust_indentation="auto" %}
      ```

   * {Dart}

      In [main.dart][dart-part1-main]:

      ```dart
      {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/dart/part_2/lib/main.dart" region_tag="part_1_import_inspect" adjust_indentation="auto" %}
      {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/dart/part_2/lib/main.dart" region_tag="part_1_init_inspect" adjust_indentation="auto" %}
      ```

   You are now using Inspect.

3. Add a simple "version" property to show which version is running:

   * {C++}

      ```cpp
      {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/cpp/part_2/main.cc" region_tag="part_1_write_version" adjust_indentation="auto" %}
      ```

      This snippet does the following:

      1. Obtain the "root" node of the Inspect hierarchy.

         The Inspect hierarchy for your component consists of a tree of Nodes,
         each of which contains any number of properties.

      2. Create a new property using `CreateString`.

         This adds a new `StringProperty` on the root. This `StringProperty`
         is called "version", and its value is "part2". We're going to set our
         property to "part1".

      3. Emplace the new property in the inspector.

         The lifetime of a property is tied to an object returned by `Create`,
         and destroying the object causes the property to disappear. The
         optional third parameter emplaces the new property in `inspector`
         rather than return it.  As a result, the new property lives as long
         as the inspector itself (the entire execution of the component).

   * {Rust}

     ```rust
     {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/rust/part_2/src/main.rs" region_tag="part_1_write_version" adjust_indentation="auto" %}
     ```

     This snippet does the following:

     1. Obtain the "root" node of the Inspect hierarchy.

        The Inspect hierarchy for your component consists of a tree of Nodes,
        each of which contains any number of properties.

     2. Create a new property using `record_string`.

        This adds a new `StringProperty` on the root. This `StringProperty`
        is called "version", and its value is "part2". We're going to set our
        property to "part1".

     3. It records it in the root node.

        The usual way of creating properties is through `create_*` methods on nodes. The lifetime of
        a property created with these methods is tied to the object returned and destroying the
        object causes the property to disappear. The library provides convenience methods `record_*`
        that perform creation of a property and tie the property lifetime to the node on which the
        method was called. As a result, the new property lives as long as the node itself (in this
        case, as long as the root node, so the entire execution of the component).

   * {Dart}

     ```dart
     {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/dart/part_2/lib/main.dart" region_tag="part_1_write_version" adjust_indentation="auto" %}
     ```

     This snippet does the following:

     1. Obtain the "root" node of the Inspect hierarchy.

        The Inspect hierarchy for your component consists of a tree of Nodes,
        each of which contains any number of properties.

     2. Create a new property using `stringProperty(...).setValue(...)`.

        This adds a new `StringProperty` on the root. This `StringProperty`
        is called "version", and its value is "part2". We're going to set our
        property to "part1".

     3. It records it in the root node.

        The lifetime of a property is tied to the lifetime of the node where it was created (in this
        case root, so the lifetime of the component). To delete the property one would have to call
        `delete()` on it.


### Reading Inspect data

Now that you have added Inspect to your component, you can read what it says:

1. Rebuild and update the target system

   ```
   fx build && fx ota
   ```

2. Run the client:

   * {C++}

      ```
      ffx component run --recreate /core/ffx-laboratory:client_part_1 fuchsia-pkg://fuchsia.com/inspect_cpp_codelab#meta/client_part_1.cm
      ffx log --tags inspect_cpp_codelab
      ```

   * {Rust}

      ```
      ffx component run --recreate /core/ffx-laboratory:client_part_1 fuchsia-pkg://fuchsia.com/inspect_rust_codelab#meta/client_part_1.cm
      ffx log --tags inspect_rust_codelab
      ```

   * {Dart}

      ```
      ffx component run --recreate /core/ffx-laboratory:client_part_1 fuchsia-pkg://fuchsia.com/inspect_dart_codelab#meta/client_part_1.cm
      ffx log --tags inspect_dart_codelab
      ```

3. Use `ffx inspect` to view your output:

   ```
   ffx inspect show
   ```

   This dumps all of the Inspect data for the entire system, which may be a lot of data.

4. Since `ffx inspect` supports glob matching, run:

   * {C++}

      ```
      $ ffx inspect show 'core/ffx-laboratory\:client_part_1/reverser'
      # or `ffx inspect show --manifest inspect_cpp_codelab`
      metadata:
        filename = fuchsia.inspect.Tree
        component_url = fuchsia-pkg://fuchsia.com/inspect_cpp_codelab#meta/part_1.cm
        timestamp = 4728864898476
      payload:
        root:
          version = part1
      ```

   * {Rust}

      ```
      $ ffx inspect show 'core/ffx-laboratory\:client_part_1/reverser'
      # or `ffx inspect show --manifest inspect_rust_codelab`
      metadata:
        filename = fuchsia.inspect.Tree
        component_url = fuchsia-pkg://fuchsia.com/inspect_rust_codelab#meta/part_1.cm
        timestamp = 4728864898476
      payload:
        root:
          version = part1
      ```

   * {Dart}

      ```
      $ ffx inspect show 'core/session-manager/session\:session/workstation_session/ffx-laboratory\:part_1/reverser'
      # or `ffx inspect show --manifest inspect_dart_codelab`
      metadata:
        filename = root.inspect
        component_url = fuchsia-pkg://fuchsia.com/inspect_dart_codelab#meta/part_1.cm
        timestamp = 4728864898476
      payload:
        root:
          version = part1
      ```

5. You can also view the output as JSON:

   * {C++}

      ```
      $ ffx --machine json-pretty inspect show 'core/ffx-laboratory\:client_part_1/reverser'
      [
        {
          "data_source": "Inspect",
          "metadata": {
            "errors": null,
            "filename": "fuchsia.inspect.Tree",
            "component_url": "fuchsia-pkg://fuchsia.com/inspect_pp_codelab#meta/part_1.cm",
            "timestamp": 5031116776282
          },
          "moniker": "core/ffx-laboratory\\:client_part_5/reverser",
          "payload": {
            "root": {
              "version": "part1",
          },
          "version": 1
        }
      ]
      ```

   * {Rust}

      ```
      $ ffx --machine json-pretty inspect show 'core/ffx-laboratory\:client_part_1/reverser'
      [
        {
          "data_source": "Inspect",
          "metadata": {
            "errors": null,
            "filename": "fuchsia.inspect.Tree",
            "component_url": "fuchsia-pkg://fuchsia.com/inspect_rust_codelab#meta/part_1.cm",
            "timestamp": 5031116776282
          },
          "moniker": "core/ffx-laboratory\\:client_part_5/reverser",
          "payload": {
            "root": {
              "version": "part1",
          },
          "version": 1
        }
      ]
      ```

   * {Dart}

      ```
      $ ffx inspect --machine json-pretty show 'core/session-manager/session\:session/workstation_session/ffx-laboratory\:part_1/reverser'
      [
        {
          "data_source": "Inspect",
          "metadata": {
            "errors": null,
            "filename": "root.inspect",
            "component_url": "fuchsia-pkg://fuchsia.com/inspect_dart_codelab#meta/part_1.cm",
            "timestamp": 5031116776282
          },
          "moniker": "core/session-manager/session\:session/workstation_session/ffx-laboratory\:part_1/reverser",
          "payload": {
            "root": {
              "version": "part1",
          },
          "version": 1
        }
      ]
      ```

### Instrumenting the code to find the bug

Now that you have initialized Inspect and know how to read data, you
are ready to instrument your code and uncover the bug.

The previous output shows you how the component is actually running
and that the component is not hanging completely. Otherwise the Inspect
read would hang.

Add new information per-connection to observe if the connection
is even being handled by your component.

1. Add a new child to your root node to contain statistics about the `reverser` service:

   * {C++}

      ```cpp
      {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/cpp/part_2/main.cc" region_tag="part_1_new_child" adjust_indentation="auto" %}
      ```

   * {Rust}


      ```rust
      {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/rust/part_2/src/main.rs" region_tag="part_1_new_child" adjust_indentation="auto" %}
      ```

   * {Dart}


      ```dart
      {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/dart/part_2/lib/main.dart" region_tag="part_1_new_child" adjust_indentation="auto" %}
      ```

2. Update your server to accept this node:

   * {C++}

      Update the definition of `CreateDefaultHandler` in [reverser.h][cpp-part1-reverser-h]
      and [reverser.cc][cpp-part1-reverser-cc]:

      ```cpp
      {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/cpp/part_2/reverser.h" region_tag="part_1_include" adjust_indentation="auto" %}
      {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/cpp/part_2/reverser.cc" region_tag="part_1_update_server" adjust_indentation="auto" %}
      ```

   * {Rust}

      Update `ReverserServerFactory::new` to accept this node in [reverser.rs][rust-part1-reverser]:

      ```rust
      {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/rust/part_2/src/reverser.rs" region_tag="part_1_use" adjust_indentation="auto" %}
      {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/rust/part_2/src/reverser.rs" region_tag="part_1_update_reverser" adjust_indentation="auto" %}
      ```

   * {Dart}

      Update the definition of `getDefaultBinder` in [reverser.dart][dart-part1-reverser]:

      ```dart
      {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/dart/part_2/lib/reverser.dart" region_tag="part_1_import" adjust_indentation="auto" %}
      {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/dart/part_2/lib/reverser.dart" region_tag="part_1_update_reverser" adjust_indentation="auto" %}
      ```

3. Add a property to keep track of the number of connections:

   Note: Nesting related data under a child is a powerful feature of Inspect.

   * {C++}

      ```cpp
      {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/cpp/part_2/reverser.cc" region_tag="part_1_add_connection_count" adjust_indentation="auto" %}
      ```

     Note: `node` is moved into the handler so that it is not dropped and
     deleted from the output.

   * {Rust}

      ```rust
      {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/rust/part_3/src/reverser.rs" region_tag="part_1_add_connection_count" adjust_indentation="auto" %}
      ```

     Note: `node` is moved into the handler so that it is not dropped and
     deleted from the output.

     Note: `node` is kept in ReverserServerFactory so that it is not dropped and deleted from the
     output together with all the nodes and properties nested under it. If the compiler shows an
     error about dead code, try renaming `node` to `_node`, since the node needs to continue
     existing despite not being read. In the following steps, the example calls `self.node`, so the compiler
     will stop complaining!

   * {Dart}

      ```dart
      {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/dart/part_2/lib/reverser.dart" region_tag="part_1_add_connection_count" adjust_indentation="auto" %}
      ```

   This snippet demonstrates creating a new `UintProperty` (containing a 64
   bit unsigned int) called `connection_count` and setting it to 0. In the handler
   (which runs for each connection), the property is incremented by 1.

4. Rebuild, re-run your component and then run `ffx inspect`:

   * {C++}

      ```
      $ ffx --machine json-pretty inspect show --manifest inspect_cpp_codelab
      ```

   * {Rust}

      ```
      $ ffx --machine json-pretty inspect show --manifest inspect_rust_codelab
      ```

   * {Dart}

      ```
      $ ffx --machine json-pretty inspect show --manifest inspect_dart_codelab
      ```

   You should now see:

   ```
   ...
   "payload": {
     "root": {
       "version": "part1",
       "reverser_service": {
         "connection_count": 1,
       }
     }
   }
   ```

The output above demonstrates that the client successfully connected
to the service, so the hanging problem must be caused by the Reverser
implementation itself. In particular, it will be helpful to know:

1. If the connection is still open while the client is hanging.

2. If the `Reverse` method was called.


**Exercise**: Create a child node for each connection, and record
"request\_count" inside the Reverser.

- *Hint*: There is a utility function for generating unique names:

   * {C++}

      ```cpp
      {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/cpp/part_2/reverser.cc" region_tag="part_1_connection_child" adjust_indentation="auto" %}
      ```

   * {Rust}

      ```rust
      {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/rust/part_2/src/reverser.rs" region_tag="part_1_connection_child" adjust_indentation="auto" %}
      ```

   * {Dart}

      ```dart
      {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/dart/part_2/lib/reverser.dart" region_tag="part_1_connection_child" adjust_indentation="auto" %}
      ```

   This will create unique names starting with "connection".


* {C++}

   *Hint*: You will find it helpful to create a constructor for Reverser
   that takes `inspect::Node`. [Part 3](#part-3) of this codelab explains why this is
   a useful pattern.

* {Rust}

   *Hint*: You will find it helpful to create a constructor for `ReverserServer`
   that takes `inspect::Node` for the same reason as we did for `ReverserServerFactory`.

* {Dart}

   *Hint*: You will find it helpful to create a constructor for `ReverserImpl`
   that takes `inspect.Node`. [Part 3](#part-3) of this codelab explains why this is
   a useful pattern.

- *Hint*: You will need to create a member on Reverser to hold the
`request_count` property. Its type will be `inspect::UintProperty`.

- *Follow up*: Does request count give you all of the information you
need? Add `response_count` as well.

- *Advanced*: Can you add a count of *all* requests on *all*
connections? The Reverser objects must share some state. You may find
it helpful to refactor arguments to Reverser into a separate struct
(See solution in [part 2](#part-2) for this approach).

After completing this exercise and running `ffx inspect`, you should see something like this:

```
...
"payload": {
  "root": {
    "version": "part1",
    "reverser_service": {
      "connection_count": 1,
      "connection0": {
        "request_count": 1,
      }
    }
  }
}
```

The output above shows that the connection is still open and it received one request.

* {C++}

   If you added "response\_count" as well, you may have noticed the bug.
   The `Reverse` method receives a `callback`, but it is never called with the value of `output`.

* {Rust}

   If you added "response\_count" as well, you may have noticed the bug.
   The `Reverse` method receives a `responder`, but it is never called with the value of `result`.

* {Dart}

   If you added "response\_count" as well, you may have noticed the bug.
   The `reverse` method receives never returns the value of `result`.


1. Send the response:

   * {C++}

      ```cpp
      // At the end of Reverser::Reverse
      {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/cpp/part_2/reverser.cc" region_tag="part_1_callback" adjust_indentation="auto" %}
      ```

   * {Rust}

      ```rust
      {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/rust/part_2/src/reverser.rs" region_tag="part_1_respond" adjust_indentation="auto" %}
      ```

   * {Dart}

      ```dart
      {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/dart/part_2/lib/reverser.dart" region_tag="part_1_result" adjust_indentation="auto" %}
      ```

2. Run the client again:

   * {C++}

      ```
      ffx component run --recreate /core/ffx-laboratory:client_part_1 fuchsia-pkg://fuchsia.com/inspect_cpp_codelab#meta/client_part_1.cm
      Creating component instance: client_part_1

      ffx log --tags inspect_cpp_codelab
      [00039.129068][39163][39165][inspect_cpp_codelab, client] INFO: Input: Hello
      [00039.194151][39163][39165][inspect_cpp_codelab, client] INFO: Output: olleH
      [00039.194170][39163][39165][inspect_cpp_codelab, client] INFO: Input: World
      [00039.194402][39163][39165][inspect_cpp_codelab, client] INFO: Output: dlroW
      [00039.194407][39163][39165][inspect_cpp_codelab, client] INFO: Done reversing! Please use `ffx component stop`
      ```

   * {Rust}

      ```
      ffx component run --recreate /core/ffx-laboratory:client_part_1 fuchsia-pkg://fuchsia.com/inspect_rust_codelab#meta/client_part_1.cm
      Creating component instance: client_part_1

      ffx log --tags inspect_rust_codelab
      [00039.129068][39163][39165][inspect_rust_codelab, client] INFO: Input: Hello
      [00039.194151][39163][39165][inspect_rust_codelab, client] INFO: Output: olleH
      [00039.194170][39163][39165][inspect_rust_codelab, client] INFO: Input: World
      [00039.194402][39163][39165][inspect_rust_codelab, client] INFO: Output: dlroW
      [00039.194407][39163][39165][inspect_rust_codelab, client] INFO: Done reversing! Please use `ffx component stop`
      ```

   * {Dart}

      ```
      ffx component run --recreate /core/ffx-laboratory:client_part_1 fuchsia-pkg://fuchsia.com/inspect_dart_codelab#meta/client_part_1.cm
      ffx log --tags inspect_dart_codelab
      [00039.129068][39163][39165][inspect_rust_codelab, client] INFO: Input: Hello
      [00039.194151][39163][39165][inspect_rust_codelab, client] INFO: Output: olleH
      [00039.194170][39163][39165][inspect_rust_codelab, client] INFO: Input: World
      [00039.194402][39163][39165][inspect_rust_codelab, client] INFO: Output: dlroW
      [00039.194407][39163][39165][inspect_rust_codelab, client] INFO: Done reversing! Please use `ffx component stop`
      ```

The component continues to run until you execute `ffx component stop`. As long as the component runs
you can run `ffx inspect` and observe your output.

This concludes part 1. You may commit your changes so far:

```
git commit -am "solution to part 1"
```

## Part 2: Diagnosing inter-component problems {#part-2}

Note: All links and examples in this section refer to "part\_2" code. If
you are following along, you may continue using "part\_1."

You received a bug report. The "FizzBuzz" team is saying they
are not receiving data from your component.

In addition to serving the Reverser protocol, the component also reaches
out to the "FizzBuzz" service and prints the response:

* {C++}

   ```cpp
   {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/cpp/part_1/main.cc" region_tag="fizzbuzz_connect" adjust_indentation="auto" %}
   ```

* {Rust}

   ```rust
   {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/rust/part_1/src/main.rs" region_tag="fizzbuzz_connect" adjust_indentation="auto" %}
   ```

* {Dart}

   ```dart
   {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/dart/part_1/lib/main.dart" region_tag="connect_fizzbuzz" adjust_indentation="auto" %}
   ```

If you see the logs, you will see that this log is never printed.

* {C++}

   ```cpp
   ffx log --tags inspect_cpp_codelab
   ```

* {Rust}

   ```rust
   ffx log --tags inspect_rust_codelab
   ```

* {Dart}

   ```dart
   ffx log --tags inspect_dart_codelab
   ```

You will need to diagnose and solve this problem.

### Diagnose the issue with Inspect

1. Run the component to see what is happening:

   Note: Replace 2 with 1 if you are continuing from part 1.

   * {C++}

      ```
      ffx component run /core/ffx-laboratory:client_part_2 fuchsia-pkg://fuchsia.com/inspect_cpp_codelab#meta/client_part_2.cm
      ```

   * {Rust}

      ```
      ffx component run /core/ffx-laboratory:client_part_2 fuchsia-pkg://fuchsia.com/inspect_rust_codelab#meta/client_part_2.cm
      ```

   * {Dart}

      ```
      ffx component run /core/ffx-laboratory:client_part_2 fuchsia-pkg://fuchsia.com/inspect_dart_codelab#meta/client_part_2.cm
      ```

   Fortunately the FizzBuzz team instrumented their component using Inspect.

2. Read the FizzBuzz Inspect data using `ffx inspect` as before, you get:

   ```
   "payload": {
       "root": {
           "fizzbuzz_service": {
               "closed_connection_count": 0,
               "incoming_connection_count": 0,
               "request_count": 0,
               ...
   ```

   This output confirms that FizzBuzz is not receiving any connections.

3. Add Inspect to identify the problem:

   * {C++}

      ```cpp
      {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/cpp/part_2/main.cc" region_tag="instrument_fizzbuzz" adjust_indentation="auto" %}
      ```

   * {Rust}

      ```rust
      {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/rust/part_2/src/main.rs" region_tag="instrument_fizzbuzz" adjust_indentation="auto" %}
      ```

   * {Dart}

      ```dart
      {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/dart/part_2/lib/main.dart" region_tag="instrument_fizzbuzz" adjust_indentation="auto" %}
      ```

**Exercise**: Add Inspect to the FizzBuzz connection to identify the problem

- *Hint*: Use the snippet above as a starting point, it provides an
error handler for the connection attempt.

* {C++}

   *Follow up*: Can you store the status somewhere? You can convert it
   to a string using `zx_status_get_string(status)`.

   *Advanced*: `inspector` has a method called `Health()` that announces
   overall health status in a special location. Since our service is not
   healthy unless it can connect to FizzBuzz, can you incorporate this:

     ```cpp
     /*
     "fuchsia.inspect.Health": {
         "status": "STARTING_UP"
     }
     */
     inspector.Health().StartingUp();

     /*
     "fuchsia.inspect.Health": {
         "status": "OK"
     }
     */
     inspector.Health().Ok();

     /*
     "fuchsia.inspect.Health": {
         "status": "UNHEALTHY",
         "message": "Something went wrong!"
     }
     */
     inspector.Health().Unhealthy("Something went wrong!");
     ```

* {Rust}

   *Advanced*: `fuchsia_inspect::component` has a function called `health()` that returns an object
   that announces overall health status in a special location (a node child of the root of the
   inspect tree). Since our service is not healthy unless it can connect to FizzBuzz, can
   you incorporate this:

   ```rust
   /*
   "fuchsia.inspect.Health": {
       "status": "STARTING_UP"
   }
   */
   fuchsia_inspect::component::health().set_starting_up();

   /*
   "fuchsia.inspect.Health": {
       "status": "OK"
   }
   */
   fuchsia_inspect::component::health().set_ok();

   /*
   "fuchsia.inspect.Health": {
       "status": "UNHEALTHY",
       "message": "Something went wrong!"
   }
   */
   fuchsia_inspect::component::health().set_unhealthy("something went wrong!");
   ```

* {Dart}

   *Advanced*: `fuchsia_inspect::Inspect` has a getter called `health` that returns an object
   that announces overall health status in a special location (a node child of the root of the
   inspect tree). Since our service is not healthy unless it can connect to FizzBuzz, can
   you incorporate this:

   ```dart
   /*
   "fuchsia.inspect.Health": {
       "status": "STARTING_UP"
   }
   */
   inspect.Inspect().health.setStartingUp();

   /*
   "fuchsia.inspect.Health": {
       "status": "OK"
   }
   */
   inspect.Inspect().health.setOk();

   /*
   "fuchsia.inspect.Health": {
       "status": "UNHEALTHY",
       "message": "Something went wrong!"
   }
   */
   inspect.Inspect().health.setUnhealthy('Something went wrong!');
   ```

Once you complete this exercise, you should see that the connection
error handler is being called with a "not found" error. Inspect
output showed that FizzBuzz is running, so maybe something is
misconfigured. Unfortunately not everything uses Inspect (yet!) so
look at the logs:

* {C++}

   ```
   $ ffx log --filter FizzBuzz
   ...
   ...  No capability available at path /svc/fuchsia.examples.inspect.FizzBuzz
   for component /core/ffx-laboratory:client_part_2/reverser, verify the
   component has the proper `use` declaration. ...
   ```

* {Rust}

   ```
   $ ffx log --filter FizzBuzz
   ...
   ... No capability available at path /svc/fuchsia.examples.inspect.FizzBuzz
   for component /core/ffx-laboratory:client_part_2/reverser, verify the
   component has the proper `use` declaration. ...
   ```

* {Dart}

   ```
   $ ffx log --filter FizzBuzz
   [106.395][reverser][][W] No capability available at path /svc/fuchsia.examples.inspect.FizzBuzz
   for component /core/session-manager/session:session/workstation_session/ffx-laboratory:part_1/reverser,
   verify the component has the proper `use` declaration.
   ```

Sandboxing errors are a common pitfall that are sometimes difficult to uncover.

Note: While you could have looked at the logs from the beginning to find
the problem, the log output for the system can be extremely verbose. The
particular log that you are looking for was a kernel log from the framework,
which is additionally difficult to test for.

Looking at part2 meta, you can see it is missing the service:

* {C++}

    Add a `use` entry for `Fizzbuzz` to [part_2/meta][cpp-part2-meta]
    ```
    use: [
        { protocol: "fuchsia.examples.inspect.FizzBuzz" },
    ],
    ```

* {Rust}

    Add a `use` entry for `Fizzbuzz` to [part_2/meta][rust-part2-meta]
    ```
    use: [
        { protocol: "fuchsia.examples.inspect.FizzBuzz" },
    ],
    ```

* {Dart}

    Add a `use` entry for `Fizzbuzz` to [part_2/meta][dart-part2-meta]
    ```
    use: [
        { protocol: "fuchsia.examples.inspect.FizzBuzz" },
    ],
    ```

After you added "fuchsia.examples.inspect.FizzBuzz", rebuild,
and run again. You should now see FizzBuzz in the logs and an OK status:

* {C++}

   ```
   $ ffx log --tags inspect_cpp_codelab
   [inspect_cpp_codelab, part2] INFO: main.cc(57): Got FizzBuzz: 1 2 Fizz
   4 Buzz Fizz 7 8 Fizz Buzz 11 Fizz 13 14 FizzBuzz 16 17 Fizz 19 Buzz Fizz
   22 23 Fizz Buzz 26 Fizz 28 29 FizzBuzz
   ```

* {Rust}

   ```
   $ ffx log --tags inspect_rust_codelab
   [inspect_rust_codelab, part2] INFO: main.rs(52): Got FizzBuzz: 1 2 Fizz
   4 Buzz Fizz 7 8 Fizz Buzz 11 Fizz 13 14 FizzBuzz 16 17 Fizz 19 Buzz Fizz
   22 23 Fizz Buzz 26 Fizz 28 29 FizzBuzz
   ```

* {Dart}

   ```
   $ ffx log --tags inspect_dart_codelab
   [inspect_dart_codelab, part2] INFO: main.dart(35): Got FizzBuzz: 1 2 Fizz
   4 Buzz Fizz 7 8 Fizz Buzz 11 Fizz 13 14 FizzBuzz 16 17 Fizz 19 Buzz Fizz
   22 23 Fizz Buzz 26 Fizz 28 29 FizzBuzz
   ```

This concludes Part 2.

You can now commit your solution:

```
git commit -am "solution for part 2"
```

## Part 3: Unit Testing for Inspect {#part-3}

Note: All links and examples in this section refer to "part\_3" code. If
you are following along, you may continue using the part you started with.

All code on Fuchsia should be tested, and this applies to Inspect data as well.

While Inspect data is not *required* to be tested in general, you
need to test Inspect data that is depended upon by other tools such as
Triage or Feedback.

Reverser has a basic unit test. Run it:

* {C++}

   The unit tests is located in [reverser\_unittests.cc][cpp-part3-unittest].

   ```
   fx test inspect_cpp_codelab_unittests
   ```

* {Rust}

   The unit test is located in [reverser.rs > mod tests][rust-part3-unittest].

   ```
   fx test inspect_rust_codelab_unittests
   ```

* {Dart}

   The unit test is located in [reverser\_test.dart][dart-part3-unittest].

   ```
   fx test inspect_dart_codelab_unittests
   ```

Note: This runs unit tests for all parts of this codelab.

The unit test ensures that Reverser works properly (and doesn't hang!), but it does
not check that the Inspect output is as expected.

Note: If you are following along from part\_1, you will need to uncomment
some lines in the part_1 unit test and pass default values for the Inspect properties to your
Reverser.

Passing Nodes into constructors is a form of [Dependency
Injection](https://en.wikipedia.org/wiki/Dependency_injection), which
allows you to pass in test versions of dependencies to check their state.

The code to open a Reverser looks like the following:

* {C++}

   ```cpp
   {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/cpp/part_3/reverser_unittests.cc" region_tag="open_reverser" adjust_indentation="auto" %}
   // Alternatively
   binding_set_.AddBinding(std::make_unique<Reverser>(inspect::Node()),
                           ptr.NewRequest());
   ```

* {Rust}

   ```rust
   {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/rust/part_2/src/reverser.rs" region_tag="open_reverser" adjust_indentation="auto" %}
   ```

* {Dart}

   ```dart
   {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/dart/part_3/test/reverser_test.dart" region_tag="open_reverser" adjust_indentation="auto" %}
   ```

A default version of the Inspect Node is passed into the Reverser. This
allows the reverser code to run properly in tests, but it does not
support asserting on Inspect output.


* {C++}

   **Exercise**: Change `OpenReverser` to take the dependency for Reverser
   as an argument and use it when constructing Reverser.

   - *Hint*: Create an `inspect::Inspector` in the test function. You can
   get the root using `inspector.GetRoot()`.

   - *Hint*: You will need to create a child on the root to pass in to `OpenReverser`.

* {Rust}

   **Exercise**: Change `open_reverser` to take the dependency for a `ReverserServerFactory`
   as an argument and use it when constructing Reverser.

   - *Hint*: Create a `fuchsia_inspect::Inspector` in the test function. You can
     get the root using `inspector.root()`.

   - *Note*: Do not use `component::inspector()` directly in your tests, this creates a static
     inspector that will be alive in all your tests and can lead to flakes or unexpected behaviors.
     For unit tests, alwas prefer to use a new `fuchsia_inspect::Inspector`

   - *Hint*: You will need to create a child on the root to pass in to `ReverserServerFactory::new`.

* {Dart}

   **Exercise**: Change `openReverser` to take the dependency for an `inspect.Node`
   as an argument and use it when constructing Reverser.

   - *Hint*: Use `inspect.Inspect.forTesting` and `FakeVmoHolder` to create
     an Inspect object without fuchsia dependencies to run your test on host.

   - *Hint*: You will need to create a child on the root to pass in to `openReverser`.


**Follow up**: Create multiple reverser connections and test them independently.

Following this exercise, your unit test will set real values in an
Inspect hierarchy.

Add code to test the output in Inspect:

* {C++}

   ```cpp
   {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/cpp/part_4/reverser_unittests.cc" region_tag="include_testing" adjust_indentation="auto" %}
   {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/cpp/part_4/reverser_unittests.cc" region_tag="get_hierarchy" adjust_indentation="auto" %}
   ```

   Note: If you use the LazyNode or LazyValues features, you will need to
   use inspect::ReadFromInspector and run the returned fpromise::promise to
   completion. See the solution to this part for an example.

   The snippet above reads the underlying virtual memory object (VMO)
   containing Inspect data and parses it into a readable hierarchy.

   You can now read individual properties and children as follows:

   ```cpp
   {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/cpp/part_4/reverser_unittests.cc" region_tag="assertions" adjust_indentation="auto" %}
   ```

* {Rust}

   ```rust
   {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/rust/part_4/src/reverser.rs" region_tag="include_testing" adjust_indentation="auto" %}
   {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/rust/part_4/src/reverser.rs" region_tag="test_inspector" adjust_indentation="auto" %}
   {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/rust/part_4/src/reverser.rs" region_tag="assert_tree" adjust_indentation="auto" %}
   ```

* {Dart}

   ```dart
   {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/dart/part_4/test/reverser_test.dart" region_tag="reverser_test" adjust_indentation="auto" %}
   ```

   The `VmoMatcher` is a convenient utility for testing inspect integrations. It allows to assert
   existing properties and children and missing ones, among other features.

The snippets above read a snapshot from the underlying virtual memory object (VMO)
containing Inspect data and parses it into a readable hierarchy.

**Exercise**: Add assertions for the rest of your Inspect data.

This concludes Part 3.

You may commit your changes:

```
git commit -am "solution to part 3"
```


## Part 4: Integration Testing for Inspect

Note: All links and examples in this section refer to "part\_4" code. If
you are following along, you may continue using the part you started with.

[Integration testing](https://en.wikipedia.org/wiki/Integration_testing)
is an important part of the software development workflow for
Fuchsia. Integration tests allow you to observe the behavior of your
actual component when it runs on the system.

### Running integration tests

You can run the integration tests for the codelab as follows:

* {C++}

   ```
   $ fx test inspect_cpp_codelab_integration_tests
   ```

* {Rust}

   ```
   $ fx test inspect_rust_codelab_integration_tests
   ```

* {Dart}

   ```
   $ fx test inspect_dart_codelab_integration_tests
   ```

Note: This runs integration tests for all parts of this codelab.

### View the code

Look at how the integration test is setup:

1. View the component manifest for the integration test:

   * {C++}

     Find the component manifest (cml) in [part_4/meta][cpp-part4-integration-meta]

   * {Rust}

     Find the component manifest (cml) in [part_4/meta][rust-part4-integration-meta]

   * {Dart}

     Find the component manifest (cml) in [part_4/meta][dart-part4-integration-meta]


```json5
{
   ...
   use: [
       { protocol: "fuchsia.diagnostics.ArchiveAccessor" },
   ]
}
```

This file uses the protocol `fuchsia.diagnostics.ArchiveAccessor` from parent. This protocol
is available to all tests to enable to read diagnostics about all components under the test
realm.

2. Look at the integration test itself. The individual test cases are fairly straightforward:

   * {C++}

      Locate the integration test in [part4/tests/integration_test.cc][cpp-part4-integration].

      ```cpp
      {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/cpp/part_4/tests/integration_test.cc" region_tag="integration_test" adjust_indentation="auto" %}
      ```

      `StartComponentAndConnect` is responsible for creating a new test
      environment and starting the codelab component inside of it. The
      `include_fizzbuzz_service` option instructs the method to optionally
      include FizzBuzz. This feature tests that your Inspect output is as
      expected in case it fails to connect to FizzBuzz as in Part 2.

   * {Rust}

      Locate the integration test in [part4/tests/integration_test.rs][rust-part4-integration].

      ```rust
      {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/rust/part_4/tests/integration_test.rs" region_tag="integration_test" adjust_indentation="auto" %}
      ```

      `IntegrationTest::start` is responsible for creating a new test
      environment and starting the codelab component inside of it. The
      `include_fizzbuzz` option instructs the method to optionally
      launch the FizzBuzz component. This feature tests that your Inspect
      output is as expected in case it fails to connect to FizzBuzz as in Part 2.

   * {Dart}

      Locate the integration test in [part_4/test/integration_test.dart][dart-part4-integration].

      ```dart
      {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/dart/part_4/test/integration_test.dart" region_tag="integration_test" adjust_indentation="auto" %}
      ```

      `IntegrationTest.create` is responsible for creating a new test environment.
      `connectToReverser` launches the reverser component and optionally launches the
      FizzBuzz component. This feature tests that the Inspect output is as expected in case it fails
      to connect to FizzBuzz as in Part 2.

3. Add the following method to your test fixture to read from the ArchiveAccessor service:

   * {C++}

     ```cpp
     {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/cpp/part_5/tests/integration_test.cc" region_tag="include_json" adjust_indentation="auto" %}
     {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/cpp/part_5/tests/integration_test.cc" region_tag="get_inspect" adjust_indentation="auto" %}
     ```

   * {Rust}

     ```rust
     {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/rust/part_5/tests/integration_test.rs" region_tag="include_test_stuff" adjust_indentation="auto" %}
     {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/rust/part_5/tests/integration_test.rs" region_tag="get_inspect" adjust_indentation="auto" %}
     ```

   * {Dart}

     ```dart
     {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/dart/testing/lib/integration_test.dart" region_tag="include_test_stuff" adjust_indentation="auto" %}
     {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/dart/testing/lib/integration_test.dart" region_tag="get_inspect" adjust_indentation="auto" %}
     ```


4. **Exercise**. Use the returned data in your tests and add assertions to the returned data:

   * {C++}

      ```cpp
      {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/cpp/part_5/tests/integration_test.cc" region_tag="parse_result" adjust_indentation="auto" %}
      ```

      Add assertions on the returned JSON data.

      - *Hint*: It may help to print the JSON output to view the schema.

      - *Hint*: You can read values by path as follows:

      - *Hint*: You can `EXPECT_EQ` by passing in the expected value as a rapidjson::Value:
        `rapidjson::Value("OK")`.

      ```cpp
      {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/cpp/part_5/tests/integration_test.cc" region_tag="hint_get_value" adjust_indentation="auto" %}
      ```

   * {Rust}

      ```rust
      {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/rust/part_5/tests/integration_test.rs" region_tag="result_hierarchy" adjust_indentation="auto" %}
      ```

      Add assertions on the returned `DiagnosticsHierarchy`.

      - *Hint*: It may help to print the JSON output to view the schema.

   * {Dart}

      ```dart
      {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/codelab/dart/part_5/test/integration_test.dart" region_tag="result_hierarchy" adjust_indentation="auto" %}
      ```

      Add assertions on the returned Map data.

      - *Hint*: It may help to print the JSON output to view the schema.


Your integration test will now ensure your inspect output is correct.

This concludes Part 4.

You may commit your solution:

```
git commit -am "solution to part 4"
```

## Part 5: Feedback Selectors

This section is under construction.

- TODO: Writing a feedback selector and adding tests to your integration test.

- TODO: Selectors for Feedback and other pipelines

[fidl-fizzbuzz]: /examples/diagnostics/inspect/codelab/fidl/fizzbuzz.test.fidl
[fidl-reverser]: /examples/diagnostics/inspect/codelab/fidl/reverser.test.fidl

[inspect-cpp-codelab]: /examples/diagnostics/inspect/codelab/cpp
[cpp-common-cml]: /examples/diagnostics/inspect/codelab/cpp/client/meta/common.shard.cml
[cpp-part1]: /examples/diagnostics/inspect/codelab/cpp/part_1
[cpp-part1-main]: /examples/diagnostics/inspect/codelab/cpp/part_1/main.cc
[cpp-part1-reverser-h]: /examples/diagnostics/inspect/codelab/cpp/part_1/reverser.h
[cpp-part1-reverser-cc]: /examples/diagnostics/inspect/codelab/cpp/part_1/reverser.cc
[cpp-part1-build]: /examples/diagnostics/inspect/codelab/cpp/part_1/BUILD.gn
[cpp-client-main]: /examples/diagnostics/inspect/codelab/cpp/client/main.cc#118
[cpp-part2-meta]: /examples/diagnostics/inspect/codelab/cpp/part_2/meta/integration_test.cml
[cpp-part3-unittest]: /examples/diagnostics/inspect/codelab/cpp/part_3/reverser_unittests.cc
[cpp-part4-integration]: /examples/diagnostics/inspect/codelab/cpp/part_4/tests/integration_test.cc
[cpp-part4-integration-meta]: /examples/diagnostics/inspect/codelab/cpp/part_4/meta/integration_test.cml

[inspect-rust-codelab]: /examples/diagnostics/inspect/codelab/rust
[rust-common-cml]: /examples/diagnostics/inspect/codelab/rust/client/meta/common.shard.cml
[rust-part1]: /examples/diagnostics/inspect/codelab/rust/part_1
[rust-part1-main]: /examples/diagnostics/inspect/codelab/rust/part_1/src/main.rs
[rust-part1-reverser]: /examples/diagnostics/inspect/codelab/rust/part_1/src/reverser.rs
[rust-part1-build]: /examples/diagnostics/inspect/codelab/rust/part_1/BUILD.gn
[rust-client-main]: /examples/diagnostics/inspect/codelab/rust/client/src/main.rs#41
[rust-part2-meta]: /examples/diagnostics/inspect/codelab/rust/part_2/meta/part_2.cml
[rust-part3-unittest]: /examples/diagnostics/inspect/codelab/rust/part_3/src/reverser.rs#99
[rust-part4-integration]: /examples/diagnostics/inspect/codelab/rust/part_4/tests/integration_test.rs
[rust-part4-integration-meta]: /examples/diagnostics/inspect/codelab/rust/part_4/meta/integration_test.cml

[inspect-dart-codelab]: /examples/diagnostics/inspect/codelab/dart
[dart-part1]: /examples/diagnostics/inspect/codelab/dart/part_1
[dart-part1-main]: /examples/diagnostics/inspect/codelab/dart/part_1/lib/main.dart
[dart-part1-reverser]: /examples/diagnostics/inspect/codelab/dart/part_1/lib/reverser.dart
[dart-part1-build]: /examples/diagnostics/inspect/codelab/dart/part_1/BUILD.gn
[dart-client-main]: /examples/diagnostics/inspect/codelab/dart/client/lib/main.dart
[dart-part2-meta]: /examples/diagnostics/inspect/codelab/dart/part_2/meta/part_2.cml
[dart-part3-unittest]: /examples/diagnostics/inspect/codelab/dart/part_3/test/reverser_test.dart
[dart-part4-integration]: /examples/diagnostics/inspect/codelab/dart/part_4/test/integration_test.dart
[dart-part4-integration-meta]: /examples/diagnostics/inspect/codelab/dart/part_4/meta/integration_tests.cml

[fuchsia-inspect-derive]: /docs/development/languages/rust/ergonomic_inspect.md
