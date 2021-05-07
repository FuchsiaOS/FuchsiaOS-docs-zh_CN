# Add logging to hello world

The following guide discusses adding the [Rust Fuchsia logger library](https://fuchsia-docs.firebaseapp.com/rust/fuchsia_syslog/index.html)
to the existing [hello world example](/examples/hello_world/rust/)
component. In this guide, development takes place within the Fuchsia source tree.

With the Fuchsia logger library, you can interact with log collection
services. You can use Fuchsia's logging tools to log and analyze services
and components that are written to run on Fuchsia.


## Prerequisites {#prerequisites}

*   A hardware device that is set up to run Fuchsia.
    *   The device should be paved and running. If you haven't already
        installed Fuchsia, see [Get Started](/docs/get-started/README.md).
*   Rust installed on your environment.
    *   The Fuchsia build installs a version of Rust that can be used for
        Fuchsia development. If you have already built Fuchsia, you don't need
        to install Rust again.


## Set build to include example {#include-example}

This guide modifies the existing hello world Rust example component.
In order to run that component later, you must set the hello world component with
the `fx` tool.

Run `fx set`, replacing `PRODUCT` and `BOARD` with your chosen product and board.

<pre class="prettyprint">
    fx set <var>PRODUCT</var>.<var>BOARD</var>  --with //examples/hello_world
</pre>

Note:
To see a list of possible products, run: <p><pre class="prettyprint">fx list-products</pre></p>
To see a list of possible boards, run: <p><pre class="prettyprint">fx list-boards</pre></p>

## Edit the component package {#edit-component-package}

When connecting your component to an additional service, you need to
do the following:

1. [Edit the `BUILD.gn`](#edit-the-buildgn).

1. [Edit the source file containing the `main()`](#edit-the-source-file).

1. [Edit the component manifest](#edit-the-component-manifest).

### Edit the BUILD.gn {#edit-the-buildgn}

You can declare your component's dependencies and source files in the `BUILD.gn`.

For more information, see [Introduction to GN](/docs/concepts/build_system/intro.md).

1.  Open  the `BUILD.gn` in your chosen text editor.

    ```
    vi ~/fuchsia/examples/hello_world/rust/BUILD.gn
    ```

1.  Add `"//src/lib/syslog/rust:syslog"` to the dependencies array in the
   `rustc_binary` target, which defines the executable.

    After adding this dependency, the `rustc_binary` in your `BUILD.gn`  should
    look like this:

    ```
    …
    rustc_binary("bin") {
    name = "hello_world_rust"
    with_unit_tests = true
    edition = "2018"

    deps = [
    "//src/lib/syslog/rust:syslog",
    ]
    test_deps = [ "//garnet/public/rust/fuchsia-async" ]
    }
    …
    ```

### Edit the source file {#edit-the-source-file}

The source files are included in the `src` directory of your component's
package. In this guide, the source file is `main.rs`.

1.  Open  the source file, `main.rs`, with your chosen text editor.

    ```
    vi ~/fuchsia/examples/hello_world/rust/src/main.rs
    ```

1.  Add a `use` declaration for the `fuchsia_syslog` crate.

    ```
    use fuchsia_syslog as syslog
    ```

1.  Within `main()`, initialize the `fuchsia_syslog` crate.

    ```
    syslog::init().expect("should not fail");
    ```

1.  Within `main()`, add your log message.

    ```
    syslog::fx_log_info!("{}, log!", greeting());
    ```

    At this point, `main.rs` should look like this:

    ```rust
    use fuchsia_syslog as syslog;

    fn main() {
        syslog::init().expect("should not fail");
        syslog::fx_log_info!("{}, log!", greeting());
        println!("{}, world!", greeting());
    }
    …
    ```

### Edit the component manifest {#edit-the-component-manifest}

1.  See the [following language-agnostic instructions](logging.md#manifest).

1.  Execute a build of the Fuchsia image that contains your modified component
    package.

    ```
    fx build
    ```

### Test logging {#test-logging}

1.  Ensure that `fx serve` is running in a shell tab. If it is not, open
    a shell tab and run `fx serve`.

    ```
    cd ~/fuchsia
    ```

    ```
    fx serve
    ```

1.  In a new shell tab, navigate to your `fuchsia` directory and run `fx log`.

    ```
    cd ~/fuchsia
    ```

    ```
    fx log
    ```

1.  In a new shell tab, navigate to your fuchsia directory and run the
    `hello_world_rust` component:

    ```
    cd ~/fuchsia
    ```

    ```
    fx shell run fuchsia-pkg://fuchsia.com/hello-world-rust#meta/hello-world-rust.cmx
    ":hello-world-rust-component-v1"
    ```

1.  Navigate to the shell tab where you ran `fx log`.

    You should be able to see your logging text, which in this example is
    `Hello log!`.
