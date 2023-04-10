# Run a fuzzer

Once you have [built your fuzzer](build-a-fuzzer.md#fx-set), there are a few ways to execute it.

## Run a fuzzer on a Fuchsia device {#run-on-device}

You can use the `ffx fuzz` tool to run a fuzzer on your local device. This is the recommended way to
run your fuzzer while you are developing it. The tool is useful in that it knows how to find
fuzzing-related files and knows various common [options][options]{:.external}.

Note: `ffx fuzz` is currently an [experimental plugin][experimental_plugin]. It is not yet available
via the SDK and must be enabled before use by running `ffx config set fuzzing true`.

When fuzzing locally, it is often convenient to use the interactive fuzzing shell, which can be
started using `ffx fuzz shell`. This shell can maintain a connection to a fuzzer, and provides
utilities such as tab-completion. It supports a number of commands:

* To list available fuzzers:7
  list [-p <pattern>]
  </pre>
* To connect to a fuzzer, starting it if needed:
  <pre class="devsite-terminal">
  attach <url> [-o <output>]
  </pre>
* To get option(s) from a fuzzer:
  <pre class="devsite-terminal">
  get [<name>]
  </pre>
* To set options on a fuzzer:
  <pre class="devsite-terminal">
  set <name> <value>
  </pre>
* To add an input to a fuzzer's corpus:
  <pre class="devsite-terminal">
  add <input> [--seed]
  </pre>
* To test a specific input with a fuzzer:
  <pre class="devsite-terminal">
  try <input>
  </pre>
* To generate inputs and fuzz the target:
  <pre class="devsite-terminal">
  run [--runs <runs>] [--time <time>]
  </pre>
* To clear extraneous bytes from an error input:
  <pre class="devsite-terminal">
  cleanse <input>
  </pre>
* To reduce the size of an error input:
  <pre class="devsite-terminal">
  minimize <input> [--runs <runs>] [--time <time>]
  </pre>
* To compact the attached fuzzer's corpus:
  <pre class="devsite-terminal">
  merge [--corpus <corpus>]
  </pre>
* To get a fuzzer's execution status:
  <pre class="devsite-terminal">
  status
  </pre>
* To retrieve the attached fuzzer's corpus:
  <pre class="devsite-terminal">
  fetch [--corpus <corpus>] [--seed]
  </pre>
* To disconnect from a fuzzer without stopping it:
  <pre class="devsite-terminal">
  detach
  </pre>
* To stop the attached fuzzer:
  <pre class="devsite-terminal">
  stop
  </pre>
* To disconnect from a fuzzer and exit the shell:
  <pre class="devsite-terminal">
  exit
  </pre>
* To clear the screen:
  <pre class="devsite-terminal">
  clear
  </pre>
* To print the command history for the shell:
  <pre class="devsite-terminal">
  history
  </pre>

Most commands require a fuzzer to be attached to the shell using the fuzzer's
[component URL][glossary.component_url]. Once attached, the fuzzer component remains alive until it
is stopped, either by the `stop` command or by [Test Manager][test_manager]. You can detach from a
fuzzer and reattach to it later.

Note: if `attach` reports that the component cannot be found or cannot be resolved, check that the
component is available to the [component resolver][component-resolvers]. Typically this means
ensuring [`fx serve`][fx-serve] is running or including your fuzzer in the
[base image][package-deployment-options] via `fx set ... --with-base <package>`.

Several of the commands represent long-running workflows. These include the most important command,
`run`, which performs coverage-guided fuzzing. These also include `try`, `cleanse`, `minimize`, and
`merge`.  All of these may execute for an indefinite amount of time.

Commands that take inputs accept both files and hexadecimal values. If the provided input is
ambiguous, `ffx fuzz` will treat it as a hexadecimal value and print a warning. Treatment as a file
can be forced by adding path elements, e.g. `./deadbeef` is a file while `deadbeef` is a value.

Putting this all together, a typical workflow might look like the following, minus the comments:

```sh
attach fuchsia-pkg://fuchsia.com/my-fuzzers#meta/my-fuzzer.cm   # Connect to the fuzzer.
run -t 60m              # Run for up to one hour.
try deadbeef            # Try a specific input, e.g. reproduce an error found by the step above.
merge -c my-local-dir   # Shrink the corpus as much as possible and save it.
stop
exit
```

## Run a fuzzer on a development host {#run-on-host}

You can run host fuzzers built by the Fuchsia build system as host tools, although the extra tooling
of `ffx fuzz` is not supported.  This means you need to manually add any
[libFuzzer options][options]{:external} to your `fx host-tool` invocation.

For example:

<pre class="devsite-terminal">
fx host-tool my_host_fuzzer -runs=1000
</pre>

You can see all available options by using `-help=1`:

<pre class="devsite-terminal">
fx host-tool my_host_fuzzer -help=1
</pre>

## Run a fuzzer on ClusterFuzz {#run-on-clusterfuzz}

This is the easiest and most recommended way to run a fuzzer after initial development. To run your
fuzzer on [ClusterFuzz][clusterfuzz]{:.external}, you simply need to ensure it is a [GN][fuchsia-gn]
dependency of `//bundles/buildbot/core`. Practically, this means including it in your code's "tests"
GN target.

For example:

```
   group("tests") {
     deps = [
       ":existing-unittest-package",
       ":my-fuzzers",
     ]
   }
```

If you are unsure if your fuzzer is included in the dependency graph, you can check using `gn path`.

For example:

<pre class="devsite-terminal">
fx gn path out/default //bundles/buildbot/core <var>path-to-fuzzer</var>
</pre>

For `//examples/fuzzers`, this yields:

<pre>
//bundles/buildbot/core --[public]-->
//examples:examples --[private]-->
//examples/fuzzers:fuzzers

1 non-data path found. It is not public.
</pre>

All fuzzers in that dependency graph will be made available to ClusterFuzz to select and run. If
ClusterFuzz opens bugs, you can use its outputs to reproduce its findings.
See [Handling results found through fuzzing](handle-results.md#clusterfuzz-bugs).

[component-resolvers]: /docs/concepts/components/v2/capabilities/resolvers.md
[clusterfuzz]: https://google.github.io/clusterfuzz/
[experimental_plugin]: /docs/development/tools/ffx/development/plugin-experimental.md
[fuchsia-gn]: /docs/development/build/build_system/intro.md
[fx-serve]: /tools/devshell/serve
[glossary.component_url]: /docs/glossary/README.md#component_url
[options]: https://llvm.org/docs/LibFuzzer.html#options
[package-deployment-options]: /docs/development/build/fx.md#package_deployment_options
[test_manager]: /docs/development/testing/components/test_runner_framework.md#the_test_manager
