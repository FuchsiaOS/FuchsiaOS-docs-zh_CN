# Fuzzing in Fuchsia

Fuzzing or fuzz testing is style of testing that stochastically generates inputs to targeted
interfaces in order to automatically find defects and/or vulnerabilities.

You can learn more details about:

 * [Fuzzing in general][background]
 * [How to write a fuzzer](write-a-fuzzer.md)
 * [How to configure and build a package of fuzzers](build-a-fuzzer.md)
 * [How to run a fuzzer](run-a-fuzzer.md)
 * [What to do with bugs and other results produced by fuzzing](handle-results.md)
 * [How to improve your fuzzer](improve-a-fuzzer.md)
 * [Experimental efforts to fuzz FIDL](fidl-fuzzing.md)

You can find complete examples under [`//examples/fuzzers`](/examples/fuzzers).

[background]: /docs/concepts/testing/fuzz_testing.md
