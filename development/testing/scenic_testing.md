# Testing Scenic and Escher

## Testability

Information about testability:

* All changes within Fuchsia need to adhere to the [Testability rubric](/docs/concepts/testing/testability_rubric.md).
* See also: [Test environments](/docs/concepts/testing/environments.md)

## Scenic test packages

You can specify packages in these ways:

* Everything:

  ```
  --with //bundles:tests
  ```

* Individually, if you want to build less packages:

  ```
  --with //src/ui:tests
  --with //src/ui/lib/escher:escher_tests
  ```

## Scenic and CQ

Tests are automatically run before submission by trybots for every change in
Fuchsia. See the `fuchsia-x64-release` and `fuchsia-x64-asan` bots on
[https://ci.chromium.org/p/fuchsia/builders](https://ci.chromium.org/p/fuchsia/builders).

### Add a new test suite to existing CQ tests

Most of the Scenic and Escher tests are written in
[Googletest](https://github.com/google/googletest/).
If you need to add test suites or test cases, you need to create test source files and add
them as a source of the test executables in corresponding `BUILD.gn` files.

### Adding a test package to CQ

To add a test package to CQ, you need to:

* Create test components;
* Reference the test package;
* Specify test environments.

#### Create test components {#create-test-components}

To add a new test package to CQ, you need to create a test component including source files,
metadata and BUILD.gn file. See
[Test component](/docs/concepts/testing/v1_test_component.md).
You can include multiple test executables in one single test package, and all of them will be
executed if you run `fx test <test_package>` on host.

Examples of test packages:

- **scenic_tests:** [//src/ui/scenic:scenic_tests](/src/ui/scenic/BUILD.gn)
- **escher_tests:** [//src/ui/lib/escher:escher_tests](/src/ui/lib/escher/BUILD.gn)

#### Reference the test package {#reference-test-package}

To ensure the test is run on CQ, it requires an unbroken chain of dependencies that roll up to your
`fx set` command's available packages (expandable using the `--with` flag), typically
going through the all target of `//garnet/packages/tests:all`.

You need to make sure that there is a dependency chain from `//garnet/packages/tests:all` to
your test package. For more information, see
[Testing FAQ documentation](/docs/development/testing/faq.md#q_what-ensures-it-is-run).

#### Specify test environments {#specify-test-environments}

To ensure that the test is run on CQ, you also need to specify a
[test environment](/docs/concepts/testing/environments.md)
for each test executable in the package inside the test's `BUILD.gn` file.

Generally the environment is set to `environments = basic_envs`.
This specifies the test should be run on both QEMU (for arm64), FEMU and NUC (for x64), and using
both debug and release builds. For running on other environments, refer to
[Test environments](/docs/concepts/testing/environments.md).

Reference the test package transitively. For example, the packages above are
referenced by `//garnet/packages/tests:all` through `//garnet/packages/tests:scenic`.

## Unit tests and integration tests

To run tests locally during development:

### Running on device/emulator

Some of these tests require the test Scenic to connect to the real display controller.

Run `fx shell killall scenic.cmx` to kill an active instance of Scenic.

* Run all Scenic tests:

  From host workstation, ensure `fx serve` is running, then:

  ```
  fx test scenic_tests escher_tests
  ```

  Or from Fuchsia target device:

  ```
  runtests --names gfx_apptests,gfx_unittests,escher_unittests,input_unittests,a11y_manager_apptests
  ```

* Run a specific test binary:

  From host workstation, ensure `fx serve` is running, then use the following
  command to run the `gfx_unittests` test component from the `scenic_tests`
  package:

  ```
  fx test gfx_unittests
  ```

  Or from Fuchsia target device:

  ```
  runtests --names gfx_unittests
  ```

* Run a single test within a component:

  From host workstation, ensure `fx serve` is running, then:

  ```
  fx test gfx_unittests -- --gtest_filter=HostImageTest.FindResource
  ```

  Or from Fuchsia target device:

  ```
  runtests --names gfx_unittests -- --gtest_filter=HostImageTest.FindResource
  ```

  See more documentation about the [glob pattern for the filter arg](https://github.com/google/googletest/blob/HEAD/docs/advanced.md).

* Run a specific component

  From your host workstation:

  ```
  fx test fuchsia-pkg://fuchsia.com/scenic_tests#meta/gfx_unittests.cmx
  ```

  Note: `gfx_unittests.cmx` can be swapped for [any test component](/src/ui/scenic/BUILD.gn) . There is also fuzzy matching!

* Pixel tests

  If you get an error connecting to a display controller, first kill all UI services.

  From your host workstation, run:

  ```
  fx shell "killall basemgr.cmx; killall root_presenter.cmx; killall scenic.cmx; killall tiles.cmx; killall present_view"
  ```

  Then run the pixel tests:

  ```
  fx test fuchsia-pkg://fuchsia.com/scenic_tests#meta/gfx_pixeltests.cmx
  ```

  Alternatively, run:

  ```
  fx test gfx_pixeltests
  ```

### Host tests

  * `fx test --host` runs all host tests, but you probably only want to run Escher tests.
  *  Escher: To run `escher_unittests` locally on Linux: follow instructions in
     [Escher documentation](/docs/concepts/graphics/escher/building.md).

## Manual UI tests

After making big Scenic changes, you may want to do manual UI testing by running some example
applications to see if there is any regression.

### UI Examples

There are some examples available:

* **bouncing_ball**
  - Basic example of creating `ViewProvider` and `View`, and creating a UI by using
    Scenic commands.
  - **Source:** [`//src/ui/examples/bouncing_ball`](/src/ui/examples/bouncing_ball)
  - **Build dependency:** `//src/ui/examples/bouncing_ball`
  - **Package URI:** `fuchsia-pkg://fuchsia.com/bouncing_ball#meta/bouncing_ball.cmx`

* **spinning_square_view**
  - Example that shows the use of [`BaseView`](/src/lib/ui/base_view).
  - **Source:** [`//src/ui/examples/spinning_square_view`](/src/ui/examples/spinning_square_view)
  - **Build dependency:** `//src/ui/examples/spinning_square_view`
  - **Package URI:** `fuchsia-pkg://fuchsia.com/spinning_square_view#meta/spinning_square_view.cmx`

* **spinning_square_rs**
  - An example written in Rust that shows the use of
    [Carnelian](/src/lib/ui/carnelian/README.md),
    a prototype framework for writing Fuchsia modules in Rust.
  - **Source:** [`//src/lib/ui/carnelian/examples/spinning_square.rs`](/src/lib/ui/carnelian/examples/spinning_square.rs)
  - **Build dependency:** `//src/lib/ui/carnelian:spinning-square-rs`
  - **Package URI:** `fuchsia-pkg://fuchsia.com/spinning-square-rs#meta/spinning-square-rs.cmx`

* **spinning_cube**
  - An example written in Dart and Flutter, showing how to create a Flutter app in Fuchsia.
  - **Source:** [`//src/experiences/examples/spinning_cube`](https://fuchsia.googlesource.com/experiences/+/main/examples/spinning_cube)
  - **Build dependency:** `//src/experiences/examples/spinning_cube:spinning-cube`
  - **Package URI:** `fuchsia-pkg://fuchsia.com/spinning-cube#meta/spinning_cube.cmx`

* **simplest_app**
  - An application that changes background color with every user touch input, which uses root
    presenter for its implementation of `BaseView`. It tracks input callbacks from Scenic and draws
    elements using `scenic::Material`.
  - **Source:** [`//src/ui/examples/simplest_app`](/src/ui/examples/simplest_app)
  - **Build dependency:** `//src/ui/examples/simplest_app`
  - **Package URI:** `fuchsia-pkg://fuchsia.com/simplest_app#meta/simplest_app.cmx`

* **yuv_to_image_pipe**
  - An application that updates the scene using an ImagePipe.
  - **Source:** [`//src/ui/examples/yuv_to_image_pipe`](/src/ui/examples/yuv_to_image_pipe)
  - **Build dependency:** `//src/ui/examples/yuv_to_image_pipe`
  - **Package URI:** `fuchsia-pkg://fuchsia.com/yuv_to_image_pipe#meta/yuv_to_image_pipe.cmx`

To run these applications, you need to include the following dependency in your `fx set`
configuration:

```shell
fx set terminal.x64 --with "//src/ui/examples,//src/lib/ui/carnelian:examples,//src/experiences/examples/spinning_cube:spinning-cube"
```

You can replace the product with `workstation.x64` as well; you can also use an all-in-one bundle
`--with //bundles:kitchen_sink` to replace the `--with` argument above so that you can include all
tests, tools and examples without any extra dependency.

### Running UI examples

#### Running in shell

You can launch the stories (modules) in any shell you are in:

* In Ermine shell, you can run modules by typing in the package name (e.g. `simplest_app`, or
  `spinning-cube`) in the [ASK] bar to run modules.

#### Running a module standalone

You can also launch a module standalone without any existing shells, this works for configurations
where there's no existing shell (e.g. in `terminal` or `core` products).

* Use command `present_view <mod_name>` command to launch a module standalone.

  From your host workstations, run:

  ```shell
  fx shell "present_view fuchsia-pkg://fuchsia.com/spinning_square_view#meta/spinning_square_view.cmx"
  ```

  to present the `View` provided in `spinning_square_view` package.

  The `present_view` command also allows command auto completion if there is only one package
  matching the `<mod_name>`; so the command above is equivalent to

  ```shell
  fx shell present_view spinning_square_view
  ```

  Note: If this doesn't work, you may need to run `fx shell "killall scenic.cmx; killall root_presenter.cmx"`
  from your host workstation to kill the existing Scenic session.

* You can also use package `tiles` to create a tiled view and add or delete modules to the tile.

  * In order to use `tiles`, first you need to ensure that you already have
    `//src/ui/tools/tiles` and `//src/ui/tools/tiles_ctl` included in your `--with`
    argument of `fx set` command.

    Or you can use `--with //src/ui/tools` or `--with //bundles:tools` (huge
    build may be expected) where `tiles` related packages will be included as
    well.

  * To display the tiles view, from your host workstations, run:

    ```shell
    fx shell "tiles_ctl start"
    ```

    This runs `tiles.cmx` package as a daemon.

  * From your host workstation, run the following command to add tiles:

    ```shell
    fx shell "tiles_ctl add fuchsia-pkg://fuchsia.com/spinning_square_view#meta/spinning_square_view.cmx"
    ```

    You can see outputs like:

    ```shell
    Tile added with key 1
    ```

    You can add as many shells as you want by running `tiles_ctl add` command.

    Note: `tiles_ctl` doesn’t support command auto completion so you have to type in the complete
    URI for the packages, which you can get by running `locate <search string>`.

  * `tiles_ctl remove <key>` command removes existing tile by the given key value. For example,
    you can run this command on host to remove the spinning square view we created previously:

    ```shell
    fx shell tiles_ctl remove 1
    ```

  * `tiles_ctl quit` command kills the `tiles` executable and all associated views.

<!-- Reference links -->

[run_fuchsia_tests]: /docs/development/testing/run_fuchsia_tests.md
