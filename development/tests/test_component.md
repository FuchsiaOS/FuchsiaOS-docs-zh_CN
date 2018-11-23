# Test Component

## Create a test component

### BUILD.gn

```gn
import("//build/test/test_package.gni")

executable("my_test_bin") {
  testonly = true
  output_name = "my_test"

  sources = [
    "my_test.cc",
  ]
}

test_package("my_test_pkg") {
  deps = [
    ":my_test_bin",
  ]

  tests = [
    {
      name = "my_test"
    },
  ]
}
```

`test_package` will expect that there is a corresponding cmx file in the `meta`
folder. So for above example there should be a `my_test.cmx` file in `meta/`.

### meta/my\_test.cmx

```json
{
    "program": {
        "binary": "test/my_test"
    },
    "sandbox": {
        "services": [...]
    }
}
```

## Run test

There are multiple ways to run tests on your local machine

```bash
runtests /pkgfs/packages/my_test_pkg/test/
```
This command will run all the tests in the path you provided.

```bash
run_test_component my_test
```

This command will search for `my_test.cmx` files and if a unique test is found
it will run it.

```bash
run_test_component fuchsia-pkg://fuchsia.com/my_test_pkg#meta/my_test.cmx
```

The URL passed to `run_test_component` represents a unique component url.

## Run external services
All test components are started in a new hermetic environment. By default, this
environment only contains a few basic services, such as
`fuchsia.sys.Environment` and `fuchsia.sys.Launcher`. To inject additional
services, you can add a `injected-services` clause to the manifest file's facets:

```json
"facets": {
  "fuchsia.test": {
    "injected-services": {
        "service_name1": "component_url1",
        "service_name2": "component_url2"
    }
  }
}
```

`run_test_component` will start `component_url1` and `component_url2` and the
test will have access to `service_name1` and `service_name2`.

### Network access
Currently we cannot run an instance of netstack inside a hermetic environment,
because it conflicts with the real netstack.  If your test needs to talk to
netstack, it may only talk to the real netstack outside the test environment. To
enable this workaround you need to allow some system services:

```json
"facets": {
  "fuchsia.test": {
    "system-services": [
      "fuchsia.netstack.Netstack",
      "fuchsia.net.LegacySocketProvider",
      "fuchsia.net.Connectivity",
      "fuchsia.net.stack.Stack"
    ]
  }
}
```

Depending on your use case you can include one or more of the services above.
However, we do not allow any other services.

This option would be deprecated once we fix CP-144.




