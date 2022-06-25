# Inspect Quickstart

This quickstart guides you through the basics of using
[Component Inspection][overview]. You will learn how to integrate Inspect into
your component using the language-specific libraries and review the data using
[`ffx inspect`][ffx-inspect].

For a more detailed walkthrough of Inspect concepts, see the
[Inspect codelab](codelab/codelab.md).

## Project setup

See below for the quick start guide in your language of choice:

* {C++}

  This section assumes you are writing an asynchronous component and that
  some part of your component (typically `main.cc`) looks like this:

  ```cpp
  async::Loop loop(&kAsyncLoopConfigAttachToCurrentThread);
  auto context_ = sys::ComponentContext::CreateAndServeOutgoingDirectory();
  // ...
  loop.Run();
  ```

  This sets up an async loop, creates a `ComponentContext` wrapping handles
  provided by the runtime, and then runs that loop following some other
  initialization work.

  **Add the Inspect library dependencies to your `BUILD.gn` file:**

  ```gn
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/cpp/BUILD.gn" region_tag="inspect_libs" adjust_indentation="auto" %}
  ```

  **Add the following includes:**

  ```cpp
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/cpp/example_server_app.h" region_tag="inspect_imports" adjust_indentation="auto" %}
  ```

  **Add the following code to initialize Inspect:**

  ```cpp
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/cpp/example_server_app.cc" region_tag="initialization" adjust_indentation="auto" %}
  ```

  You are now using Inspect! Create properties in the Inspect tree by attaching
  them to the root node:

  ```cpp
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/cpp/example_server_app.cc" region_tag="properties" adjust_indentation="auto" %}
  ```

  Note: For a complete working example, see
  [//examples/diagnostics/inspect/cpp](/examples/diagnostics/inspect/cpp).

  See [Supported Data Types](#supported-types) for a full list of data
  types you can try.

  #### Health checks

  The health check subsystem provides a standardized inspection metric for
  component health. You can use the health node to report the overall status
  of your component:

  ```cpp
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/cpp/example_server_app.cc" region_tag="health_check" adjust_indentation="auto" %}
  ```

  Note: For more details on health metrics, see [Health check][health-check].

  #### Testing

  To test your inspect code, you can use
  [//sdklib/inspect/testing/cpp/inspect.h](/sdk/lib/inspect/testing):

  ```cpp
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/cpp/example_unittests.cc" region_tag="test_imports" adjust_indentation="auto" %}
  ```

  This library includes a full set of matchers to validate the contents of the
  Inspect tree.

  ```cpp
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/cpp/example_unittests.cc" region_tag="inspect_test" adjust_indentation="auto" %}
  ```

* {Rust}

  This section assumes you are writing an asynchronous component and that some
  part of your component (typically `main.rs`) looks similar to this:

  ```rust
  async fn main() -> Result<(), Error> {
    // ...
    let mut service_fs = ServiceFs::new();
    // ...
    service_fs.take_and_serve_directory_handle().unwrap();
    service_fs.collect::<()>().await;
    Ok(())
  }
  ```

  **Add the Inspect library dependencies to your `BUILD.gn` file:**

  ```gn
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/rust/BUILD.gn" region_tag="inspect_libs" adjust_indentation="auto" %}
  ```

  **Add the following code to initialize Inspect:**

  ```rust
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/rust/src/echo_server.rs" region_tag="initialization" adjust_indentation="auto" %}
  ```

  You are now using Inspect! Create properties in the Inspect tree by attaching
  them to the root node:

  ```rust
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/rust/src/echo_server.rs" region_tag="properties" adjust_indentation="auto" %}
  ```

  Note: For a complete working example, see
  [//examples/diagnostics/inspect/rust](/examples/diagnostics/inspect/rust).

  See [Supported Data Types](#supported-types) for a full list of data
  types you can try.

  #### Health checks

  The health check subsystem provides a standardized inspection metric for
  component health. You can use the health node to report the overall status
  of your component:

  ```rust
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/rust/src/echo_server.rs" region_tag="health_check" adjust_indentation="auto" %}
  ```

  Note: For more details on health metrics, see [Health check][health-check].

  #### Testing

  To test your Inspect code, you can use `assert_data_tree` to validate the
  contents of the Inspect tree:

  ```rust
  {% includecode gerrit_repo="fuchsia/fuchsia" gerrit_path="examples/diagnostics/inspect/rust/src/echo_server.rs" region_tag="inspect_test" adjust_indentation="auto" %}
  ```

  Note: To learn more about the Rust library, see the complete reference for
  [`fuchsia_inspect`](https://fuchsia-docs.firebaseapp.com/rust/fuchsia_inspect/index.html).


## Inspect Libraries {#inspect-libraries}

<!-- TODO(fxbug.dev/84444): Replace code snippets with examples -->

Now that you have a `root_node` you may start building your
hierarchy. This section describes some important concepts and patterns
to help you get started.

* A Node may have any number of key/value pairs called **Properties**.
* The key for a Value is always a UTF-8 string, the value may be one of the
  [supported types](#supported-types) below.
* A Node may have any number of children, which are also Nodes.

* {C++}

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
  [`ValueList`](/zircon/system/ulib/inspect/include/lib/inspect/cpp/vmo/types.h)
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

  Refer to [C++ Library Concepts](#c++), as similar concepts apply in Rust.

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

### Dynamic values

This section describes support in the Inspect libraries for nodes that are
inflated lazily at read-time. The methods accept a callback function instead of
a value. The callback function is invoked when the property value is read.

* {C++}

  The C++ library has two property creators for dynamic values:
  `CreateLazyNode` and `CreateLazyValues`.

  Both of these methods take a callback returning a promise for an
  `inspect::Inspector`, the only difference is how the dynamic values are
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
      return fpromise::make_ok_promise(std::move(b));
    }, &a);

    return fpromise::make_ok_promise(std::move(a));
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

          // Callbacks return a fpromise::promise<Inspector>, so return a result
          // promise containing the value we created.
          // You can alternatively return a promise that is completed by
          // some asynchronous task.
          return fpromise::make_ok_promise(std::move(inspector));
        });
      }

    private:
      inspect::Node node_;
      inspect::IntProperty calls_;
      inspect::LazyNode lazy_;
  };
  ```

* {Rust}

  Refer to [C++ Dynamic Value Support](#c++), as similar concepts apply in Rust.

  Example:

  ```rust
  root.create_lazy_{child,values}("lazy", [] {
      async move {
          let inspector = Inspector::new();
          inspector.root().record_string("version", "1.0");
          inspector.root().record_lazy_{node,values}("lazy", || {
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

### String references {#string-reference}

* {C++}

  You can use `inspect::StringReference` to reduce the memory footprint
  of an Inspect hierarchy that has a lot of repeated data. For instance,

  ```cpp
  using inspect::Inspector;

  Inspector inspector;

  for (int i = 0; i < 100; i++) {
    inspector.GetRoot().CreateChild("child", &inspector);
  }
  ```

  Will include 100 copies of the string `"child"` in your inspect
  output.

  Alternatively,

  ```cpp
  using inspect::Inspector;
  using inspect::StringReference;

  namespace {
    const StringReference kChild("child");
  }

  Inspector inspector;
  for (int i = 0; i < 100; i++) {
    inspector.GetRoot().CreateChild(kChild, &inspector)
  }
  ```

  Will generate only one copy of `"child"` which is referenced 100 times.

  This pattern is recommended anywhere a global constant key would be used.

* {Rust}

  You can use `fuchsia_inspect::StringReference` to reduce the memory footprint
  of an Inspect hierarchy that has a lot of repeated data. For instance,

  ```rust
  use fuchsia_inspect::Inspector;

  let inspector = Inspector::new();
  for _ in 0..100 {
    inspector.root().record_child("child");
  }
  ```

  Will generate 100 copies of `"child"`.

  Alternatively,

  ```rust
  use fuchsia_inspect::{Inspector, StringReference}

  lazy_static! {
    static ref CHILD: StringReference<'static> = "child".into();
  }

  let inspector = Inspector::new();
  for _ in 0..100 {
    inspector.root().record_child(&*CHILD);
  }
  ```

  Will generate only 1 copy of `"child"` which is referenced 100 times.

  This pattern is recommended anytime a global constant key would be used.

  Note: It isn't necessary for the `StringReference` to be static.
  It can also take a `String`, owning it.

## Viewing Inspect Data {#view-inspect-data}

You can use the [`ffx inspect`][ffx-inspect] command to view the Inspect data
you exported from your component.

This section assumes you have SSH access to your running Fuchsia system and
that you started running your component. We will use the name
`my_component.cmx` as a placeholder for the name of your component.

### Find your Inspect endpoint

The command below prints all available components that expose inspect:

```posix-terminal
ffx inspect list
```

The command below prints all `fuchsia.diagnostics.ArchiveAccessor` paths:

```posix-terminal
ffx inspect list-accessors
```
Your component's endpoint is listed as
`<path>/my_component.cmx/<id>/out/diagnostics/fuchsia.inspect.Tree`.
However, in some languages (without dynamic value support) and in drivers,
the data is placed in VMO files instead. In that case, the endpoint is listed
as `<path>/my_component.cmx/<id>/out/diagnostics/root.inspect`.

An accessor path listed by `ffx inspect list-accessors` can later be used by
`ffx inspect show` and `ffx inspect selectors` using the `--accessor-path` flag.

The command below prints all available selectors for a component
(for example, `my_component.cmx`):

```posix-terminal
ffx inspect selectors my_component.cmx
```

### Read your Inspect data

The command below prints the inspect hierarchies of all components
running in the system:

```posix-terminal
ffx inspect show
```

Using the output from `ffx inspect list`, you can specify a
single component (for example, `my_component.cmx`) as input to
`ffx inspect show`:

```posix-terminal
ffx inspect show my_component.cmx
```

Or specify multiple components (for example, `core/font_provider`
and `my_component.cmx`):

```posix-terminal
ffx inspect show core/font_provider my_component.cmx
```

You can also specify a node and property value (for example,
`my_component.cmx:root.inspect)` from `ffx inspect selectors`
as input to `ffx inspect show`:

```posix-terminal
ffx inspect show my_component.cmx:root
```

This will print out the following if you followed the suggested steps above:

```none {:.devsite-disable-click-to-copy}
root:
  hello = world
```

## Supported Data Types {#supported-types}

 Type | Description | Notes
  -----|-------------|-------
    IntProperty | A metric containing a signed 64-bit integer. | All Languages
    UIntProperty | A metric containing an unsigned 64-bit integer. | Not supported in Dart
    DoubleProperty | A metric containing a double floating-point number. | All Languages
    BoolProperty | A metric containing a double floating-point number. | All Languages
    {Int,Double,Uint}Array | An array of metric types, includes typed wrappers for various histograms. | Same language support as base metric type
    StringArray | An array of strings. Represented as a [StringReference](#string-reference). | Not supported in Dart.
    StringProperty | A property with a UTF-8 string value. | All Languages
    ByteVectorProperty | A property with an arbitrary byte value. | All Languages
    Node | A node under which metrics, properties, and more nodes may be nested. | All Languages
    LazyNode | Instantiates a complete tree of Nodes dynamically. | C++, Rust

<!-- Reference links -->

[ffx-inspect]: https://fuchsia.dev/reference/tools/sdk/ffx.md#inspect
[health-check]: /docs/development/diagnostics/inspect/health.md
[overview]: /docs/development/diagnostics/inspect/README.md
