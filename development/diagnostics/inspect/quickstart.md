# Inspect Quick Start

[TOC]

This document is a guide on how to get started with [Component
Inspection](README.md) depending on your needs:

**I want to learn more about Inspect concepts**

Read the [README](README.md).

**I want to know how to use the iquery tool**

Read the [iquery manual][iquery]. For detailed examples of usage,
see the [Codelab](codelab/codelab.md).

This document provides a simplified example of iquery below.

**I have an existing or new component, and I want to support inspection.**

Continue reading this document.

## Languages

See below for the quick start guide in your language of choice.

* {C++}

  #### Setup
  This section assumes you are writing an asynchronous component and that
  some part of your component (typically `main.cc`) looks like this:

  ```cpp
  async::Loop loop(&kAsyncLoopConfigAttachToCurrentThread);
  auto component_context = sys::ComponentContext::CreateAndServeOutgoingDirectory();
  // ...
  loop.Run();
  ```

  This sets up an async loop, creates a `ComponentContext` wrapping handles
  provided by the runtime, and then runs that loop following some other
  initialization work.

  **Add the following include**

  ```cpp
  #include <lib/sys/inspect/cpp/component.h>
  ```

  **Change your initialization code to look like the following:**

  ```cpp
  async::Loop loop(&kAsyncLoopConfigAttachToCurrentThread);
  auto component_context = sys::ComponentContext::CreateAndServeOutgoingDirectory();
  auto inspector =
        std::make_unique<sys::ComponentInspector>(component_context.get());
  inspect::Node& root_node = inspector->root();
  // ...
  loop.Run();
  ```

  You are now using Inspect! To add some data and see it in action, try adding the following:

  ```cpp
  // Important: Make sure to hold on to hello_world_property and don't let it go out of scope.
  auto hello_world_property = root_node.CreateStringProperty("hello", "world");
  ```

  See [Viewing Inspect Data](#viewing-inspect-data) below to view what you are now exporting.

  See [Supported Data Types](#supported-data-types) for a full list of data types you can try.

  Want to test your Inspect integration? Include
  [lib/inspect/testing/cpp/inspect.h][testing-header]
  in your unit test for a full set of matchers. See [this example][vmo-example] of how it is used.

  [testing-header]: /sdk/lib/inspect/testing
  [vmo-example]: /src/sys/appmgr/integration_tests/inspect/test-vmo.cc

  Read on to learn how Inspect is meant to be used in C++.

  #### Dynamic Value Support

  The C++ library has two property creators for lazily inflating Inspect
  trees at read-time: `CreateLazyNode` and `CreateLazyValues`.

  Both of these methods take a callback returning a promise for an
  inspect::Inspector, the only difference is how the dynamic values are
  stored in the tree.

  `root->CreateLazyNode(name, callback)` creates a child node of
  `root` with the given `name`. The `callback` returns a promise for an
  `inspect::Inspector` whose root node is spliced into the parent hierarchy
  when read. The example below shows that a child called "lazy" exists with
  the string property "version" and has an additional child that is called
  "lazy."

  `root->CreateLazyValues(name, callback)` works like `root->CreateLazyNode(name,
  callback)`, except all properties and child nodes on the promised root node are
  added directly as values
  to the original `root`. In the second output of this example, the internal
  lazy nodes do not appear and their values are flattened into properties on
  `root`.

  ```cpp
  root->CreateLazy{Node,Values}("lazy", [] {
    Inspector a;
    a.GetRoot().CreateString("version", "1.0", &a);
    a.GetRoot().CreateLazy{Node,Values}("lazy", [] {
      Inspector b;
      b.GetRoot().CreateInt("value", 10, &b);
      return fit::make_ok_promise(std::move(b));
    }, &a);

    return fit::make_ok_promise(std::move(a));
  });
  ```

  Output (CreateLazyNode):

  ```
  root:
    lazy:
      version = "1.0"
      lazy:
        val = 10
  ```

  Output (CreateLazyValues):

  ```
  root:
    val = 10
    version = "1.0"
  ```

  Warning: It is the developer's responsibility to ensure that names
  flattened from multiple lazy value nodes do not conflict. If they do,
  output behavior is undefined.

  The return value of `CreateLazy{Node,Values}` is a `LazyNode` that owns
  the passed callback.  The callback is never called once the `LazyNode` is
  destroyed. If you destroy a `LazyNode` concurrently with the execution of
  a callback, the destroy operation is blocked until the callback returns
  its promise.

  If you want to dynamically expose properties on `this`, you may simply
  write the following:

  ```cpp
  class Employee {
    public:
      Employee(inspect::Node node) : node_(std::move(node)) {
        calls_ = node_.CreateInt("calls", 0);

        // Create a lazy node that populates values on its parent
        // dynamically.
        // Note: The callback will never be called after the LazyNode is
        // destroyed, so it is safe to capture "this."
        lazy_ = node_.CreateLazyValues("lazy", [this] {
          // Create a new Inspector and put any data in it you want.
          inspect::Inspector inspector;

          // Keep track of the number of times this callback is executed.
          // This is safe because the callback is executed without locking
          // any state in the parent node.
          calls_.Add(1);

          // ERROR: You cannot modify the LazyNode from the callback. Doing
          // so may deadlock!
          // lazy_ = ...

          // The value is set to the result of calling a method on "this".
          inspector.GetRoot().CreateInt("performance_score",
                                        this->CalculatePerformance(), &inspector);

          // Callbacks return a fit::promise<Inspector>, so return a result
          // promise containing the value we created.
          // You can alternatively return a promise that is completed by
          // some asynchronous task.
          return fit::make_ok_promise(std::move(inspector));
        });
      }

    private:
      inspect::Node node_;
      inspect::IntProperty calls_;
      inspect::LazyNode lazy_;
  };
  ```

  #### C++ Library Concepts

  Now that you have a `root_node` you may start building your
  hierarchy. This section describes some important concepts and patterns
  to help you get started.

  * A Node may have any number of key/value pairs called **Properties**.
  * The key for a Value is always a UTF-8 string, the value may be one of the supported types below.
  * A Node may have any number of children, which are also Nodes.

  The code above gives you access to a single node named
  "root". `hello_world_property` is a Property that contains a string value
  (aptly called a **StringProperty**).

  * Values and Nodes are created under a parent Node.

  Class `Node` has creator methods for every type of
  supported value. `hello_world_property` was created using
  `CreateStringProperty`. You could create a child under the root node
  by calling `root_node.CreateChild("child name")`. Note that names must
  always be UTF-8 strings.

  * Values and Nodes have strict ownership semantics.

  `hello_world_property` owns the Property. When it is destroyed (goes
  out of scope) the underlying Property is deleted and no longer present
  in your component's Inspect output. This is true for child Nodes as well.

  If you are creating a value that doesn't need to be modified, use a
  [`ValueList`](/zircon/system/ulib/inspect/include/lib/inspect/cpp/value_list.h)
  to keep them alive until they are no longer needed.

  * Inspection is best-effort.

  Due to space limitations, the Inspect library may be unable to satisfy
  a `Create` request. This error is not surfaced to your code: you will
  receive a Node/Property object for which the methods are no-ops.

  * Pattern: Pass in child Nodes to child objects.

  It is useful to add an `inspect::Node` argument to the constructors
  for your own classes. The parent object, which should own its own
  `inspect::Node`, may then pass in the result of `CreateChild(...)`
  to its children when they are constructed:

  ```cpp
  class Child {
    public:
      Child(inspect::Node my_node) : my_node_(std::move(my_node)) {
        // Create a string that doesn't change, and emplace it in the ValueList
        my_node_.CreateString("version", "1.0", &values_);
        // Create metrics and properties on my_node_.
      }

    private:
      inspect::Node my_node_;
      inspect::StringProperty some_property_;
      inspect::ValueList values_;
      // ... more properties and metrics
  };

  class Parent {
    public:
      // ...

      void AddChild() {
        // Note: inspect::UniqueName returns a globally unique name with the specified prefix.
        children_.emplace_back(my_node_.CreateChild(inspect::UniqueName("child-")));
      }

    private:
      std::vector<Child> children_;
      inspect::Node my_node_;
  };
  ```

* {Rust}

  #### Setup

  This section assumes you are writing an asynchronous component and that some
  part of your component (typically main.rs) looks similar to this:

  ```rust
  #[fasync::run_singlethreaded]
  async fn main() -> Result<(), Error> {
    ...
    let mut fs = ServiceFs::new();
    ...
    Ok(())
  }
  ```

  Add the following to your initialization code:

  ```rust
  // This creates the root of an inspect tree.
  let inspector = component::inspector();

  // This serves the inspect Tree to the default path for reading at the standard
  // location "/diagnostics/fuchsia.inspect.Tree".
  inspector.serve(&mut fs)?;

  // This will give you a reference to the root node of the inspect tree.
  let root = inspector.root();
  ```

  Don't forget to `use fuchsia_inspect::component;`!

  Now you can use inspect! For example try the following:

  ```rust
  let hello_world_property = root.create_string("hello", "world!");
  ```

  See [this example](/examples/diagnostics/inspect/rust/src/main.rs) for further
  learning of other types offered by the API.

  To test your inspect code, you can use `assert_data_tree`:

  ```rust

  let inspector = component::inspector();
  let root = inspector.root();
  let child = root.create_child("child1");
  child.record_double("some_property_name", 1.0);
  child.record_string("another_property", "example");
  let children = inspector.create_child("children");

  assert_data_tree!(inspector, root: {
    child1: {
      some_property_name: 1.0,
      another_property: "example",
      children: {},
    }
  });
  ```

  Learn more about [testing](/src/lib/diagnostics/hierarchy/rust/src/testing.rs) inspect.

  See [the docs](https://fuchsia-docs.firebaseapp.com/rust/fuchsia_inspect/index.html)
  to learn about other methods offered by the Rust API.

  See [Viewing Inspect Data](#viewing-inspect-data) below to view what you are now exporting.

  See [Supported Data Types](#supported-data-types) for a full list of data types you can try.

  #### Rust Library Concepts

  Refer to [C++ Library Concepts](#c_library-concepts), as similar concepts
  apply in Rust.

  The Rust library provides two ways of managing nodes and properties: creation and recording.

  With the `create_*` methods, the ownership of the property or node object to the caller.
  When the returned object is dropped, it is removed. For example:

  ```rust
  {
      let property = root.create_int("name", 1);
  }
  ```

  In this example, the property went out of scope so a drop on the property is
  called. Readers won't see this property.

  With the `record_*` methods, the lifetime of the node the method is
  called on is entangled with the resulting property. When the node the method was called
  is deleted, the recorded property is deleted.

  ```rust
  {
      let node = root.create_child("name");
      {
        node.record_uint(2); // no return
      }
      // The uint property will still be visible to readers.
  }
  ```

  In this example, neither the `name` node nor the uint property is visible to readers
  after `node` is dropped.

  #### Dynamic Value Support

  Refer to [C++ Dynamic Value Support](#dynamic-value-support), as similar
  concepts apply in Rust.

  Example:

  ```rust
  root.create_lazy_{child,values}("lazy", [] {
      async move {
          let inspector = Inspector::new();
          inspector.root().record_string("version", "1.0");
          inspector.root().record_lazy_{node,values}("lazy", [] {
              let inspector = Inspector::new();
              inspector.root().record_int("value", 10);
              // `_value`'s drop is called when the function returns, so it will be removed.
              // For these situations `record_` is provided.
              let _value = inspector.root().create_int("gone", 2);
              Ok(inspector)
          });
          Ok(inspector)
      }
      .boxed()
  });

  Output (create_lazy_node):
  root:
    lazy:
      version = "1.0"
      lazy:
        val = 10

  Output (create_lazy_values):
  root:
    val = 10
    version = "1.0"
  ```


* {Dart}

  This example obtains and adds several data types and nested children to the
  root Inspect node.

  `BUILD.gn`:

  ```dart
  flutter_app("inspect_mod") {
  [...]
    deps = [
      [...]
      "//topaz/public/dart/fuchsia_inspect",
      [...]
    ]
  [...]

  ```

  ```dart {highlight="lines:6"}
  import 'package:fuchsia_inspect/inspect.dart' as inspect;
  [...]
  class RootIntentHandler extends IntentHandler {
    @override
    void handleIntent(Intent intent) {
      var inspectNode = inspect.Inspect().root;
      runApp(InspectExampleApp(inspectNode));
    }
  }
  ```

  `inspect_example_app.dart`:

  ```dart {highlight="lines:4,7-10,16"}
  import 'package:fuchsia_inspect/inspect.dart' as inspect;

  class InspectExampleApp extends StatelessWidget {
    final inspect.Node _inspectNode;

    InspectExampleApp(this._inspectNode) {
      _inspectNode.stringProperty('greeting').setValue('Hello World');
      _inspectNode.doubleProperty('double down')..setValue(1.23)..add(2);
      _inspectNode.intProperty('interesting')..setValue(123)..subtract(5);
      _inspectNode.byteDataProperty('bytes').setValue(ByteData(4)..setUint32(0, 0x01020304));
    }
    @override
    Widget build(BuildContext context) {
      return MaterialApp(
        home: _InspectHomePage(
            inspectNode: _inspectNode.child('home-page')),
        [...]
    }
  ```

  You can call `delete()` on a Node or Property when you're done with it.
  Deleting a node deletes everything under it.

  `delete()` can also be triggered by a Future completing or broadcast Stream
  closing:

  ```dart
  var answerFuture = _answerFinder.getTheAnswer();
  var wait = _inspectNode.stringProperty('waiting')..setValue('for a hint');
  answerFuture.whenComplete(wait.delete);

  stream.listen((_) {}, onDone: node.delete);

  // FIDL proxies contain a future that completes when the connection closes:
  final _proxy = my_fidl_import.MyServiceProxy();
  _proxy.ctrl.whenClosed.whenComplete(node.delete);
  ```

## Viewing Inspect Data

  You can use the `[iquery]` tool to view the Inspect data you
  exported from your component by looking through the Hub.

  This section assumes you have SSH access to your running Fuchsia system and
  that you started running your component. We will use the name
  `my_component.cmx` as a placeholder for the name of your component.

### Find your Inspect endpoint

  Try the following:

  ```sh
  # This prints all component selectors (monikers in v2 or realm paths in v1) available.
  $ iquery list

  # This prints all files containing inspect under /hub (this won't show v2 components)
  $ iquery list-files /hub

  # This filters the above list to only print your component.
  $ iquery list-files /hub | grep my_component.cmx
  $ iquery list | grep my_component.cmx  # TODO(fxbug.dev/45458): allow to pass a manifest filter
  ```

  Under the listed directories you will see some paths including
  "system\_objects." This Inspect data is placed there by the Component Runtime
  itself.

  Your component's endpoint will be listed as `<path>/my_component.cmx/<id>/out/diagnostics/root.inspect`.

  Note: If you followed [Dynamic Value Support](#dynamic-value-support) above,
  "root.inspect" will be missing.

### Read your Inspect data

  Use the moniker that `list` printed above, and run:

  ```sh
  $ iquery show 'realm/my_component.cmx'
  ```

  You can also spcify a node/property using selectors:

  ```sh
  $ iquery show 'realm/my_component.cmx:root/path/to/some:property'
  ```

  Navigate to the `out/` directory that was printed above, and run:

  ```sh
  $ iquery --recursive fuchsia.inspect.Tree
  # Or root.inspect if you are exposing a vmo file directly
  # (this is not the case if using inspect libs directly)
  ```

  Or `root.inspect` instead of `fuchsia.inspect.Tree`. Note that this is not the
  case in general (except on Dart). If you are using the standard inspect libs it'll be the Tree
  protocol.

  This will print out the following if you followed the suggested steps above:

  ```sh
  root:
    hello = world
  ```

## Supported Data Types

 Type | Description | Notes
  -----|-------------|-------
    IntProperty | A metric containing a signed 64-bit integer. | All Languages
    UIntProperty | A metric containing an unsigned 64-bit integer. | Not supported in Dart
    DoubleProperty | A metric containing a double floating-point number. | All Languages
    BoolProperty | A metric containing a double floating-point number. | All Languages
    {Int,Double,UInt}Array | An array of metric types, includes typed wrappers for various histograms. | Same language support as base metric type
    StringProperty | A property with a UTF-8 string value. | All Languages
    ByteVectorProperty | A property with an arbitrary byte value. | All Languages
    Node | A node under which metrics, properties, and more nodes may be nested. | All Languages
    LazyNode | Instantiates a complete tree of Nodes dynamically. | C++, Rust

[iquery]: /docs/reference/diagnostics/consumers/iquery.md
