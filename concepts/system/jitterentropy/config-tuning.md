# Jitterentropy: tuning the configuration

The jitterentropy library is written by Stephan Mueller, is available at
<https://github.com/smuellerDD/jitterentropy-library>, and is documented at
<http://www.chronox.de/jent.html>. In Zircon, it's used as a simple entropy
source to seed the system CPRNG.

[The companion document about basic configuration options to jitterentropy](config-basic.md)
describes two options that fundamentally affect how jitterentropy runs. This document describes
instead the numeric parameters that control how fast jitterentropy is and how much entropy it
collects, but without fundamentally altering its principles of operation. It also describes how to
test various parameters and what to look for in the output (e.g. if adding support for a new device,
or to do a more thorough job of optimizing the parameters).

[TOC]

## A rundown of jitterentropy's parameters

The following tunable parameters control how fast jitterentropy runs, and how fast it collects
entropy:

### [`kernel.jitterentropy.ll`](/docs/reference/kernel/kernel_cmdline.md#kernel-jitterentropy-ll-num)

"`ll`" stands for "LFSR loops". Jitterentropy uses a (deliberately inefficient implementation of a)
LFSR to exercise the CPU, as part of its noise generation. The inner loop shifts the LFSR 64 times;
the outer loop repeats `kernel.jitterentropy.ll`-many times.

In my experience, the LFSR code significantly slows down jitterentropy, but doesn't generate very
much entropy. I tested this on RPi3 and qemu-arm64 with qualitatively similar results, but it hasn't
been tested on x86 yet. This is something to consider when tuning: using fewer LFSR loops tends to
lead to better overall performance.

Note that setting `kernel.jitterentropy.ll=0` causes jitterentropy to choose the number of LFSR
loops in a "random-ish" way. As described in [the basic config doc](config-basic.md), I discourage
the use of `kernel.jitterentropy.ll=0`.


### [`kernel.jitterentropy.ml`](/docs/reference/kernel/kernel_cmdline.md#kernel-jitterentropy-ml-num)

"`ml`" stands for "memory access loops". Jitterentropy walks through a moderately large chunk of
RAM, reading and writing each byte. The size of the chunk and access pattern are controlled by the
two parameters below. The memory access loop is repeated `kernel.jitterentropy.ml`-many times.

In my experience, the memory access loops are a good source of raw entropy. Again, I've only tested
this on RPi3 and qemu-arm64 so far.

Much like `kernel.jitterentropy.ll`, if you set `kernel.jitterentropy.ml=0`, then jitterentropy will
choose a "random-ish" value for the memory access loop count. I also discourage this.

### [`kernel.jitterentropy.bs`](/docs/reference/kernel/kernel_cmdline.md#kernel-jitterentropy-bs-num)

"`bs`" stands for "block size". Jitterentropy divides its chunk of RAM into blocks of this size. The
memory access loop starts with byte 0 of block zero, then "byte -1" of block 1 (which is actually
the last byte of block 0), then "byte -2" of block 2 (i.e. the second-to-last byte of block 1), and
so on. This pattern ensures that every byte gets hit, and most accesses go into different blocks.

I have usually tested jitterentropy with `kernel.jitterentropy.bs=64`, based on the size of a cache
line. I haven't tested yet to see whether there's a better option on some/all platforms.

### [`kernel.jitterentropy.bc`](/docs/reference/kernel/kernel_cmdline.md#kernel-jitterentropy-bc-num)

"`bc`" stands for "block count". Jitterentropy uses this many blocks of RAM, each of size
`kernel.jitterentropy.bs`, in its memory access loops.

Since I choose `kernel.jitterentropy.bs=64`, I usually choose `kernel.jitterentropy.bc=1024`.
This means using 64KB of RAM, which is enough to overflow L1 cache.

The [jitterentropy source code](/zircon/third_party/lib/jitterentropy/jitterentropy-base.c#234)
in the comment before `jent_memaccess` suggests choosing the block size and count so that the RAM
used is bigger than L1. Confusingly, the default values in upstream jitterentropy (block size = 32,
block count = 64) aren't big enough to overflow L1.

## Tuning process

The basic idea is simple: on a particular target device, try different values for the parameters.
Collect a large amount of data for each parameter set (ideally around 1MB), then
[run the NIST test suite to analyze the data](/docs/concepts/testing/entropy_quality_tests.md#running-the-nist-test-suite).
Determine which parameters give the best entropy per unit time. The time taken to draw the entropy
samples is logged on the system under test.

One complication is the startup testing built into jitterentropy. This essentially draws and
discards 400 samples, after performing some basic analysis (mostly making sure that the clock is
monotonic and has a high enough resolution and variability). A more accurate test would reboot twice
for each set of parameters: once to collect around 1MB of data for analysis, and a second time to
boot with the "right" amount of entropy (as computed according to the entropy estimate in the first
phase, with appropriate safety margins, etc. See
["Determining the entropy\_per\_1000\_bytes statistic"](#determining-the-entropy_per_1000_bytes-statistic),
below). This second phase of testing simulates a real boot, including the startup tests. After
completing the second phase, choose the parameter set that boots fastest. Of course, each phase of
testing should be repeated a few times to reduce random variations.

## Determining the entropy\_per\_1000\_bytes statistic

The `crypto::entropy::Collector` interface in
[kernel/lib/crypto/include/lib/crypto/entropy/collector.h](/zircon/kernel/lib/crypto/include/lib/crypto/entropy/collector.h)
requires a parameter `entropy_per_1000_bytes` from its instantiations. The value relevant to
jitterentropy is currently hard-coded in
[kernel/lib/crypto/entropy/jitterentropy\_collector.cpp](/zircon/kernel/lib/crypto/entropy/jitterentropy_collector.cc).
This value is meant to measure how much min-entropy is contained in each byte of data produced by
jitterentropy (since the bytes aren't independent and uniformly distributed, this will be less than
8 bits). The "per 1000 bytes" part simply makes it possible to specify fractional amounts of
entropy, like "0.123 bits / byte", without requiring fractional arithmetic (since `float` is
disallowed in kernel code, and fixed-point arithmetic is confusing).

The value should be determined by using the NIST test suite to analyze random data samples, as
described in
[the entropy quality tests document](/docs/concepts/testing/entropy_quality_tests.md#running-the-nist-test-suite).
The test suite produces an estimate of the min-entropy; repeated tests of the same RNG have (in my
experience) varied by a few tenths of a bit (which is pretty significant when entropy values can be
around 0.5 bits per byte of data!). After getting good, consistent results from the test suites,
apply a safety factor (i.e. divide the entropy estimate by 2), and update the value of
`entropy_per_1000_bytes` (don't forget to multiply by 1000).

Note that eventually `entropy_per_1000_bytes` should probably be configured somewhere instead of
hard-coded in jitterentropy\_collector.cpp. Kernel cmdlines or even a preprocessor symbol could work.

## Notes about the testing script

The `scripts/entropy-test/jitterentropy/test-tunable` script automates the practice of looping
through a large test matrix. The downside is that tests run in sequence on a single machine, so (1)
an error will stall the test pipeline so supervision *is* required, and (2) the machine is being
constantly rebooted rather than cold-booted (plus it's a netboot-reboot), which could conceivably
confound the tests. Still, it beats hitting power-off/power-on a thousand times by hand!

Some happy notes:

1. When netbooting, the script leaves bootserver on while waiting for netcp to successfully export
   the data file. If the system hangs, you can power it off and back on, and the existing bootserver
   process will restart the failed test.

2. If the test is going to run (say) 16 combinations of parameters 10 times each, it will go like
   this:

       test # 0: ml = 1   ll = 1  bc = 1  bs = 1
       test # 1: ml = 1   ll = 1  bc = 1  bs = 64
       test # 2: ml = 1   ll = 1  bc = 32 bs = 1
       test # 3: ml = 1   ll = 1  bc = 32 bs = 64
       ...
       test #15: ml = 128 ll = 16 bc = 32 bs = 64
       test #16: ml = 1   ll = 1  bc = 1  bs = 1
       test #17: ml = 1   ll = 1  bc = 1  bs = 64
       ...

   (The output files are numbered starting with 0, so I started with 0 above.)

   So, if test #17 fails, you can delete tests #16 and #17, and re-run 9 more iterations of each
   test. You can at least keep the complete results from the first iteration. In theory, the tests
   could be smarter and also keep the existing result from test #16, but the current shell scripts
   aren't that sophisticated.

The scripts don't do a two-phase process like I suggested in the ["Tuning process"](#tuning-process)
section above. It's certainly possible, but again the existing scripts aren't that sophisticated.

## Open questions

### How much do we trust the low-entropy extreme?

It's *a priori* possible that we maximize entropy per unit time by choosing small parameter values.
Most extreme is of course `ll=1, ml=1, bs=1, bc=1`, but even something like `ll=1, ml=1, bs=64,
bc=32` is an example of what I'm thinking of.  Part of the concern is the variability in the test
suite: if hypothetically the tests are only accurate to within 0.2 bits of entropy per byte, and if
they're reporting 0.15 bits of entropy per byte, what do we make of it? Hopefully running the same
test a few hundred times in a row will reveal a clear modal value, but it's still a little bit risky
to rely on that low estimate to be accurate.

The NIST publication states (line 1302, page 35, second draft) that the estimators "work well when
the entropy-per-sample is greater than 0.1". This is fairly low, so hopefully it isn't an issue in
practice. Still, the fact that there is a lower bound means we should probably leave a fairly
conservative envelope around it.

### How device-dependent is the optimal choice of parameters?

There's evidently a significant difference in the actual "bits of entropy per byte" metric on
different architectures or different hardware. Is it possible that most systems are optimal at
similar parameter values (so that we can just hard-code these values into
`kernel/lib/crypto/entropy/jitterentropy_collector.cpp`? Or, do we need to put the parameters into
MDI or into a preprocessor macro, so that we can use different defaults on a per-platform basis (or
whatever level of granularity is appropriate).

### Can we even record optimal parameters with enough granularity?

I mentioned it above, but one of our targets is "x86", which is what runs on any x86
PC. Naturally, x86 PCs can very quite a bit. Even if we did something like add preprocessor symbols
like `JITTERENTROPY_LL_VALUE` etc. to the build, customized in `kernel/project/target/pc-x86.mk`,
could we pick a good value for *all PCs*?

If not, what are our options?

1. We could store a lookup table based on values accessible at runtime (like the exact CPU model,
   the core memory size, cache line size, etc.). This seems rather unwieldy. Maybe if we could find
   one or two simple properties to key off of, say "CPU core frequency" and "L1 cache size", we
   could make this relatively non-terrible.

2. We could try an adaptive approach: monitor the quality of the entropy stream, and adjust the
   parameters according on the fly. This would take a lot of testing and justification if we want to
   trust it.

3. We could settle for "good enough" parameters on most devices, with the option to tune via kernel
   cmdlines or a similar mechanism. This seems like the most likely outcome to me. I expect that
   "good enough" parameters will be easy to find, and not disruptive enough to justify extreme
   solutions.
