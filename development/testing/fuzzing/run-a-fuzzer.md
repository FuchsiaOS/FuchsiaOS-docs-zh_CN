# Run a fuzzer

Once you have [built your fuzzer](build-a-fuzzer.md#fx-set), there are a few ways to execute it.

## Run a fuzzer on a Fuchsia device {#run-on-device}

You can use the `fx fuzz` tool to run a fuzzer on your local device. This is the recommended way to
run your fuzzer while you are developing it. The tool is useful in that it knows how to find
fuzzing-related files and knows various common [options].

* To see available commands and options:
  <pre class="devsite-terminal">
  fx fuzz help
  </pre>
* To see available fuzzers:
  <pre class="devsite-terminal">
  fx fuzz list
  </pre>
* To start a fuzzer:
  <pre class="devsite-terminal">
  fx fuzz <var>package</var>/<var>fuzzer</var>
  </pre>


`package` and `fuzzer` match those reported by `fx fuzz list`, and may be abbreviated.  For commands
that accept a single fuzzer, e.g. `check`, the abbreviated name must uniquely identify exactly one
fuzzer.

When starting a fuzzer, the tool will echo the command it is invoking, prefixed by `+`.  This can be
useful if you want to manually invoke the fuzzer.

If the fuzzer finds any results, you can use `fx fuzz` to investigate them further. See
[Handling results found through fuzzing](handle-results.md#fx-fuzz-results).

## Run a fuzzer on a host platform {#run-on-host}

You can run host fuzzers built by the Fuchsia build system, although the extra tooling of `fx fuzz`
is not supported.  This means you will need to manually run them and reproduce the results
they find. To build host fuzzers, set `fuzz_host=true` in the `fuzzers_package`[gn fuzzers package].

For example:

```
fuzzers_package("overnet_fuzzers") {
  cpp_fuzzers = [ "packet_protocol:packet_protocol_fuzzer" ]
  fuzz_host = true
}
```

Upon building, you can find the host fuzzers in the host variant output directory. For example, the
fuzzer above would be produced at `//out/default/host_x64-asan-fuzzer`.

## Run a fuzzer on ClusterFuzz {#run-on-clusterfuzz}

This is the easiest and most recommended way to run a fuzzer after initial development. To run your
fuzzer on [ClusterFuzz][clusterfuzz], you simply need to ensure it is a [GN][fuchsia-gn] dependency
of `//bundles/buildbot:core`. Practically, this means including it in your code's "tests" GN target.

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
fx gn path out/default //bundles/buildbot:core <var>path-to-fuzzer</var>
</pre>

For `//examples/fuzzers`, this yields:

<pre>
//bundles/buildbot:core --[public]-->
//examples:examples --[private]-->
//examples/fuzzers:fuzzers

1 non-data path found. It is not public.
</pre>

All fuzzers in that dependency graph will be made available to ClusterFuzz to select and run. If
ClusterFuzz opens bugs, you can use its outputs to reproduce its findings.
See [Handling results found through fuzzing](handle-results.md#clusterfuzz-bugs).

[clusterfuzz]: https://google.github.io/clusterfuzz/
[fuchsia-gn]: /docs/concepts/build_system/intro.md
[options]: https://llvm.org/docs/LibFuzzer.html#options
