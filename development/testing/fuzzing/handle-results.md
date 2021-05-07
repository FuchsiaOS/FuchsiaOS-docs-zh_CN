# Handling results found through fuzzing

When [your fuzzer runs](run-a-fuzzer.md), it searches for inputs that crash the program or violate
checked conditions. When the fuzzer finds and reports such a test input, it is evidence of a bug
that needs to be resolved.

Typically, you might use `fx fuzz` when first developing your fuzzer. This can often produce results
immediately. After a fuzzer has been submitted it will be run at scale by
[ClusterFuzz][clusterfuzz], and any results it finds will be filed as bugs.

## Handle results from `fx fuzz` {#fx-fuzz-results}

After [running a fuzzer with `fx fuzz`](run-a-fuzzer.md#run-on-device), the tool can be used to
report any artifacts found by running:

<pre class="devsite-terminal">
fx fuzz check <var>package</var>/<var>fuzzer</var>
</pre>

Additionally, output logs and any results are stored to the output directory. By default, this is:

<pre>
$FUCHSIA_DIR/test_data/fuzzing/<var>package</var>/<var>fuzzer</var>/<var>timestamp</var>
</pre>

A different location can be set passing the `--output` option to `fx fuzz`.

The most recent fuzzer run is symbolically linked to:

<pre>
$FUCHSIA_DIR/test_data/fuzzing/<var>package</var>/<var>fuzzer</var>/latest
</pre>

Crashes and other artifacts will have file name like
`{{ "<var>" }}type-of-finding{{ "</var>" }}.{{ "<var>" }}SHA1-hash-of-input{{ "</var>" }}`. The file
contents will be the input bytes themselves.

For example, examining a crash produced by a [toy example][toy-example] might look like the
following:

<pre class="prettyprint devsite-disable-click-to-copy">
<code class="devsite-terminal">cd test_data/fuzzing/example-fuzzers/toy_example_arbitrary/latest</code>
<code class="devsite-terminal">hd /crash-2fda3f03bb699c8a2151724b64b6e36c3b986aea</code>
00000000  2a 48 49 21 2a 00 08 00  00 00 2a 48 49 00 0a 66  |*HI!*.....*HI..f|
00000010  4a 33 00 0a f9                                    |J3...|
00000015
</pre>

### Reproduce a result {#repro}

You can execute the fuzzer with this input again using `fx fuzz repro`. For example:

<pre class="devsite-terminal devsite-disable-click-to-copy">
fx fuzz repro examples/toy crash-2fda3f03bb699c8a2151724b64b6e36c3b986aea
</pre>

If the result is reproducible, this will produce a symbolized log including a stack trace. The top
of this stack trace is likely to be the error handling by libFuzzer and/or the sanitizer, and the
bottom will likely be the fuzzer engine itself.

For example, the relevant function where the `panic` occurred in the following stack trace is
`_toy_example_arbitrary_lib_rustc_static::toy_example`:

<pre class="prettyprint devsite-disable-click-to-copy">
#0    0x000023c56780a61e in _$LT$std..sys_common..backtrace.._print..DisplayBacktrace$u20$as$u20$core..fmt..Display$GT$::fmt::h510ae2e0fe71c88c <>+0x19161e
#1    0x000023c56783399c in core::fmt::write::hb61ef49191e76a74 <>+0x1ba99c
#2    0x000023c5678009b1 in std::io::Write::write_fmt::h41df81fb2b8460af <>+0x1879b1
#3    0x000023c56780eb92 in std::panicking::default_hook::_$u7b$$u7b$closure$u7d$$u7d$::h4e9a8e3c4f33b3f4 <>+0x195b92
#4    0x000023c56780e87c in std::panicking::default_hook::hd85edcd963c04eae <>+0x19587c
#5    0x000023c56780f271 in std::panicking::rust_panic_with_hook::h8960558cc7e69505 <>+0x196271
#6    0x000023c5677973d5 in std::panicking::begin_panic::h97c6d4cd722282c5 /b/s/w/ir/k/rust/src/libstd/panicking.rs:397 <>+0x11e3d5
#7    0x000023c56777f2d0 in _toy_example_arbitrary_lib_rustc_static::toy_example::h573322211ba71c22 ../../out/default/../../examples/fuzzers/rust/src/lib.rs:22 <>+0x1062d0
#8    0x000023c567780a03 in _toy_example_arbitrary_lib_rustc_static::_::toy_example_arbitrary::hc02c288d17b25ac2 ../../out/default/../../examples/fuzzers/rust/src/lib.rs:35 <>+0x107a03
#9    0x000023c56778136c in LLVMFuzzerTestOneInput ../../out/default/../../examples/fuzzers/rust/src/lib.rs:33 <>+0x10836c
#10   0x000023c56772ab86 in fuzzer::Fuzzer::ExecuteCallback(unsigned char const*, unsigned long) <>+0xb1b86
#11   0x000023c567716ae5 in fuzzer::RunOneTest(fuzzer::Fuzzer*, char const*, unsigned long) ../recipe_cleanup/clangshYTOG/llvm_build_dir/tools/clang/stage2-bins/runtimes/runtimes-x86_64-unknown-fuchsia-bins/compiler-rt/lib/fuzzer/FuzzerDriver.cpp:296 <>+0x9dae5
#12   0x000023c56771c535 in fuzzer::FuzzerDriver(int*, char***, int (*)(unsigned char const*, unsigned long)) <>+0xa3535
#13   0x000023c5677457e3 in main ../recipe_cleanup/clangshYTOG/llvm_build_dir/tools/clang/stage2-bins/runtimes/runtimes-x86_64-unknown-fuchsia-bins/compiler-rt/lib/fuzzer/FuzzerMain.cpp:19 <>+0xcc7e3
#14   0x000041fad9a9243b in start_main ./../../zircon/third_party/ulib/musl/src/env/__libc_start_main.c:112 <libc.so>+0x9343b
</pre>

For unreproducible results, you can still examine symbolized log from the original fuzzer execution
for clues.

### Attach a debugger {#debug}

You may also want to attach a debugger when reproducing fuzzer results. By default, libFuzzer on
Fuchsia creates a [debug exception channel][exception-channel] attached to the fuzzing thread in
order to detect and handle crashes during fuzzing. Only one process may do this per thread, so
debuggers are prevented from attaching.

To prevent `libfuzzer` from creating a debug exception channel, use the `--debug` option with
`fx fuzz`.

For example, to use [zxdb] while reproducing a specific test case:

<pre class="prettyprint devsite-disable-click-to-copy">
<code class="devsite-terminal">fx debug</code>
[zxdb] attach noop-fuzzer
[zxdb] break LLVMFuzzerTestOneInput
</pre>

Now, in a separate terminal, start the fuzzer with your test case:

<pre class="devsite-terminal">
fx fuzz repro --debug zircon_fuzzers/noop-fuzzer testcase_input_file
</pre>

### File fuzzing bugs {#bug-filing}

Note: The bug tracker is currently only open to Googlers.

It may be tempting to immediately fix the bug related to the fuzzer result, especially if the bug is
obvious. No matter how trivial the bug is, please file a bug report!

To file a bug, please use the [Fuzzing Bug template][fuzzing-bug-template]. This ensures you include
certain labels, such as `found-by-fuzzing`, `libfuzzer` and `Sec-TriageMe`. This in turn helps the
security team see where fuzzers are being used and stay aware of any critical issues they are
finding.

Important: As with other potential security issues, bugs should be filed in the component of the
code under test, and _not_ in the `Security` component.

If you encounter problems or shortcomings in the fuzzing framework _itself_, open bugs or
feature requests in the [`Security>libFuzzer` component][security-libfuzzer].

As with all potential security issues, you do not need to wait for triage to begin fixing the bug!
Once fixed, reference the bug number in the commit message.

## Handle bugs from ClusterFuzz {#clusterfuzz-bugs}

ClusterFuzz will file bugs automatically when it finds reproducible fuzzer results. If you are
assigned such a bug, look for the following:

 * The _Detailed Report_ will contain details about the result, including:

   * What type of result it is.
   * Whether it has security implications.
   * What revisions exhibited the behavior.
   * What stack frames appear to uniquely identify the crash.

 * The _Reproducer Testcase_ will link to a fuzzer artifact. You can download this artifact and then
   use it to reproduce the fuzzer result as described [above](#repro).

When you submit a fix so that a fuzzer stops producing an artifact from the input, ClusterFuzz will
automatically close the bug.

## Bugs found by fuzzing {#found-by-fuzzing}

Note: The bug tracker is currently only open to Googlers.

{% dynamic if user.is_googler %}

You can see bugs found in Fuchsia by fuzzing in [Monorail].

You can also see graphs of this information using the Fuchsia fuzzing bug [dashboard].

{% dynamic endif %}

[clusterfuzz]: https://google.github.io/clusterfuzz/
[dashboard]: https://goto.google.com/fuchsia-fuzzing-bugs
[exception-channel]: /docs/concepts/kernel/exceptions.md
[fuzzing-bug-template]: https://bugs.fuchsia.dev/p/fuchsia/issues/entry?template=Fuzzing+Bug
[monorail]: https://goto.google.com/fuchsia-found-by-fuzzing
[security-libfuzzer]: https://bugs.fuchsia.dev/p/fuchsia/issues/list?q=component%3ASecurity%3Elibfuzzer&can=2
[toy-example]: /examples/fuzzers/rust/src/lib.rs
[zxdb]: /docs/development/debugger
