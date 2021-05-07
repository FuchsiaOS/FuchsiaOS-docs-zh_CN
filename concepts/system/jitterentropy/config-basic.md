# Jitterentropy: basic configuration

The jitterentropy library is written by Stephan Mueller, is available at
<https://github.com/smuellerDD/jitterentropy-library>, and is documented at
<http://www.chronox.de/jent.html>. In Zircon, it's used as a simple entropy
source to seed the system CPRNG.

This document describes and analyzes two (independent) configuration options of
jitterentropy:

1. Whether to use a variable, pseudorandom number of iterations in the noise
   generating functions.
2. Whether to post-process the raw noise samples with jitterentropy's internal
   processing routines.

I consider these basic configuration options because the affect the basic
process that jitterentropy uses. I'm contrasting them to tunable parameters
(like the precise value used for loop counts if they are not chosen
pseudorandomly, or the size of the scratch memory used internal by
jitterentropy), since the tunable parameters don't greatly affect the means by
which jitterentropy collects entropy, just the amount it collects and the time
it takes.

My full conclusions are at the end of this document, but in summary I think that
we should avoid both choosing pseudorandom iteration numbers and using the
jitterentropy post-processed data.

[TOC]

## Brief explanation of jitterentropy

The author's documentation is available in HTML form at
<http://www.chronox.de/jent/doc/CPU-Jitter-NPTRNG.html>, or in PDF form at
<http://www.chronox.de/jent/doc/CPU-Jitter-NPTRNG.pdf>. In brief, the library
collects random bits from variations in CPU instruction timing.

Jitterentropy maintains a random state, in the form of a 64-bit number that is
affected by many of the jitterentropy functions, and ultimately is used as the
output randomness.

There are two noise sources, both of which are blocks of relatively slow-running
code whose precise runtime is measured (using a system clock, requiring roughly
nanosecond resolution). The precise time to complete these blocks of code will
vary. We test these times to ensure that they are unpredictable; while we can't
be perfectly certain that they are, the test results (including the results
below) are encouraging. Note however that the purpose of this document is not to
justify our estimates for the min-entropy in jitterentropy samples, but rather
to discuss the two configuration options listed above.

The first of the code blocks used as a noise source is a CPU-intensive LFSR
loop, implemented in
[the `jent_lfsr_time` function](/zircon/third_party/lib/jitterentropy/jitterentropy-base.c#185).
The number of times the LFSR logic is repeated is controlled by the
`kernel.jitterentropy.ll` cmdline ("`ll`" stands for "LFSR loops"). If `ll = 0`,
a pseudorandom count is used, and otherwise the value of `ll` is used.
Looking at the source code, the outer loop repeats according to the `ll`
parameter.  The inner loop advances an LFSR by 64 steps, each time XOR-ing in
one bit from the most recent time sample. Passing the time sample through the
LFSR this way serves as a processing step, generally tending to whiten the
random timesteps. As described in the
[entropy quality testing doc](/docs/concepts/testing/entropy_quality_tests.md), it's important to
skip this processing when testing the entropic content of the CPU time
variations.  It's also the case that enabling the processing increases the
entropy estimates by a suspicious amount in some cases (see
[the "Effects of processing the raw samples" section](#effects-of-processing-the-raw-samples)).

The second noise source is a memory access loop, in
[the `jent_memaccess` function](/zircon/third_party/lib/jitterentropy/jitterentropy-base.c#261).
The memory access loop is repeated according to the `kernel.jitterentropy.ml`
cmdline ("`ml`" for "memory loops"), where again a value of 0 activates the
pseudorandom loop count, and any non-zero value overrides the pseudorandom
count. Each iteration of the actual memory access loop both reads and writes a
relatively large chunk of memory, divided into `kernel.jitterentropy.bc`-many
blocks of size `kernel.jitterentropy.bs` bytes each. The default values when I
wrote the current document are `bc = 1024` and `bs = 64`; up-to-date defaults
should be documented in
[the cmdline document](/docs/reference/kernel/kernel_cmdline.md). For comparison, the defaults in
the jitterentropy source code are `bc = 64` and `bs = 32`,
[defined here](/zircon/third_party/lib/jitterentropy/include/lib/jitterentropy/jitterentropy.h#79).
Per the comment above the `jent_memaccess` function, the total memory size
should be larger than the L1 cache size of the target machine. Confusingly,
`bc = 64` and `bs = 32` produce a memory size of 2048 bytes, which is much
smaller than even most L1 caches (I couldn't find any CPU with more than 0 bytes
but less than 4KB of L1). Using `bs = 64` and `bc = 1024` result in 64KB of
memory, which is usually enough to overflow L1 data caches.

### Option 1: Pseudorandom loop counts

Jitterentropy was originally designed so that the two noise generating functions
run a pseudorandom number of times. Specifically,
[the `jent_loop_shuffle` function](/zircon/third_party/lib/jitterentropy/jitterentropy-base.c#125)
mixes together (1) the time read from the high-resolution clock and (2)
jitterentropy's internal random state in order to decide how many times to run
the noise sources.

We added the ability to override these pseudorandom loop counts, and tested
jitterentropy's performance both with and without the override. The results are
discussed in more depth in
[the "Effects of pseudorandom loop counts" section](#effects-of-pseudorandom-loop-counts),
but in summary: the statistical tests suggested that the pseudorandom loop
counts increased the entropy far more than expected.  This makes me mistrust
these higher entropy counts, so I recommend using the lower estimates and
preferring deterministic loop counts to pseudorandom.

### Jitterentropy's random data processing

As mentioned above, jitterentropy can process its random data, which makes the
data look "more random".  Specifically, the processing should decrease (and
ideally remove) the deviation of the random data from the uniform distribution,
and reduce (ideally, remove) any intercorrelations between random bytes.

The main function of interest for generating processed samples is
[`jent_gen_entropy`](/zircon/third_party/lib/jitterentropy/jitterentropy-base.c#462),
which is called in a loop by
[`jent_read_entropy`](/zircon/third_party/lib/jitterentropy/jitterentropy-base.c#544)
to produce an arbitrarily large number of random bytes.
In essence, `jent_gen_entropy` calls the noise functions in a loop 64 times.
Each of the 64 invocations of `jent_lfsr_time` mixes the noisy time measurement
into the jitterentropy random state.

After these 64 iterations, the random state is optionally "stirred" in
[`jent_stir_pool`](/zircon/third_party/lib/jitterentropy/jitterentropy-base.c#403)
by XOR-ing with a "mixer" value, itself dependent on the jitterentropy random
state. As noted in the source code, this operation cannot increase or decrease
the entropy in the pool (since XOR is bijective), but it can potentially improve
the statistical appearance of the random state.

In principle, invoking the noise source functions 64 times should produce 64
times as much entropy, up to the maximum 64 bits that the random state can hold.
This assumes that the mixing operation in `jent_lfsr_time` is cryptographically
sound. I'm not an expert in cryptanalysis, but a LFSR itself is not a
cryptographically secure RNG, since 64 successive bits reveal the entire state
of a 64-bit LFSR, after which all past and future values can be easily
computed. I am not sure that the jitterentropy scheme &mdash; XOR-ing the time
measurement into the "bottom" of the LFSR as the LFSR is shifted &mdash; is more
secure. Without careful cryptographic examination of this scheme (which for all
I know may exist, but the I did not see it mentioned in the jitterentropy
documentation), I would lean towards using unprocessed samples, and mixing them
into our system entropy pool in a known-good way (e.g. SHA-2, as we do now).

That said, I did run the NIST test suite against processed data samples. My
results are in
[the "Effects of processing the raw samples" section](#effects-of-processing-the-raw-samples))
below.

## Testing process

The procedure for running entropy source quality tests is documented in
[the entropy quality tests document](/docs/concepts/testing/entropy_quality_tests.md).

These preliminary results were gathered on a Zircon debug build on Raspberry Pi
3, built from commit 18358de5e90a012cb1e042efae83f5ea264d1502 in the now-obsolete project: https://fuchsia.googlesource.com/zircon/+/a1a80a6a7d
"\[virtio]\[entropy] Basic virtio-rng driver". The following flags were set in
my `local.mk` file when building:

```
ENABLE_ENTROPY_COLLECTOR_TEST=1
ENTROPY_COLLECTOR_TEST_MAXLEN=1048576
```

I ran the boot-time tests after netbooting the debug kernel on the Pi with the
following kernel cmdline, varying the values of `$ML`, `$LL`, and `$RAW`:

```
kernel.entropy-test.src=jitterentropy
kernel.jitterentropy.bs=64
kernel.jitterentropy.bc=1024
kernel.jitterentropy.ml=$ML
kernel.jitterentropy.ll=$LL
kernel.jitterentropy.raw=$RAW
```

## Test results and analysis

### Effects of pseudorandom loop counts

#### Raw Data

Following the logic in the jitterentropy source code (search for
[`MAX_FOLD_LOOP_BIT`](/zircon/third_party/lib/jitterentropy/jitterentropy-base.c#191)
and
[`MAX_ACC_LOOP_BIT`](/zircon/third_party/lib/jitterentropy/jitterentropy-base.c#265))
the pseudorandom loop counts vary within these ranges:

```
ml: 1 .. 128 (inclusive)
ll: 1 .. 16 (inclusive)
```

I have included the overall min-entropy estimate from the NIST suite in this
table, as well as two contributing estimates: the compression estimate and the
Markov estimate. The NIST min-entropy estimate is the minimum of 10 different
estimates, including these two. The compression estimate is generally the
smallest for jitterentropy raw samples with deterministic loop counts, and the
Markov estimate is generally smallest for jitterentropy with other
configurations.

| `ml`              | `ll`             | min-entropy (bits / byte) | Compression estimate | Markov estimate |
|:-----------------:|:----------------:|:-------------------------:|:--------------------:|:---------------:|
| random (1 .. 128) | random (1 .. 16) | 5.77                      | 6.84                 | 5.77            |
| 128               | 16               | 1.62                      | 1.62                 | 3.60            |
| 1                 | 1                | 0.20                      | 0.20                 | 0.84            |


In other words, varying the loop counts pseudorandomly increased the min-entropy
estimate for raw samples by 4.15 bits (or 250%), compared to the deterministic
version that always used the maximum values from the pseudorandom ranges.

#### Analysis

The pseudorandom loop count values are determined by adding one extra time
sample per noise function. First, these time samples are not independent of the
noise function time measurements, since the gaps between the loop count time
samples correspond predictably to the noise function time measurements. As a
result it would be highly questionable to assume that they increase the
min-entropy of the output data at all.  Second, it is absurd to imagine that the
loop count time samples were somehow about 250% as random as the noise function
time measurements, since both rely on the same noise source, except that the
very first loop count time samples maybe get a small boost from the random
amount of time needed to boot the system enough to run the test.

Consequently, I suspect that what happened is that the pseudorandom loop counts
were enough to "fool" the particular suite of statistical tests and
predictor-based tests in the NIST suite, but that a predictor test written with
specific knowledge of how the jitterentropy pseudorandom loop counts are derived
could in fact predict the output with far better accuracy. I think the "true"
min-entropy in the pseudorandom loop count test, against an adversary that's
specifically targeting our code, is within the bounds of the two deterministic
tests, i.e. between about 0.20 and 1.62 bits per byte.

Using pseudorandom counts forces us to make an additional decision: do we
conservatively estimate the actual entropy content at 0.20 bits per byte (as if
the pseudorandom count function always chose `ml = 1` and `ll = 1`)? Or do we
chose an average entropy content (there is probably a more intelligent averaging
technique than to compute (1.62 + 0.20) / 2 = 0.91 bits / byte, but that will
serve for the purpose of this discussion) and risk the pseudorandom loop counts
occasionally causing us to undershoot this average entropy content? If we are
too conservative, we will spend more time collecting entropy than is needed; if
we are too optimistic, we might have a security vulnerability. Ultimately, this
forces a trade-off between security (which prefers conservative entropy
estimates) and efficiency (which prefers optimistic entropy estimates).

### Effects of processing the raw samples

#### Raw Data

I repeated the three tests reported above, but with jitterentropy's internal
processing turned on (with `kernel.jitterentropy.raw = false` instead of the
default value `true`). For convenience, the tables below include both the raw
sample results (copied from above) in the top three rows, and the processed
results (newly added) in the bottom three rows.

| `ml`              | `ll`             | raw   | min-entropy (bits / byte) | Compression estimate | Markov estimate |
|:-----------------:|:----------------:|:-----:|:-------------------------:|:--------------------:|:---------------:|
| random (1 .. 128) | random (1 .. 16) | true  | 5.77                      | 6.84                 | 5.77            |
| 128               | 16               | true  | 1.62                      | 1.62                 | 3.60            |
| 1                 | 1                | true  | 0.20                      | 0.20                 | 0.84            |

| `ml`              | `ll`             | raw   | min-entropy (bits / byte) | Compression estimate | Markov estimate |
|:-----------------:|:----------------:|:-----:|:-------------------------:|:--------------------:|:---------------:|
| random (1 .. 128) | random (1 .. 16) | false | 5.79                      | 6.59                 | 5.79            |
| 128               | 16               | false | 5.78                      | 6.97                 | 5.78            |
| 1                 | 1                | false | 5.77                      | 6.71                 | 5.77            |

#### Analysis

The post-processing min-entropy estimates are all essentially equal (up to
slight variations easily explained by randomness), and also equal to the
min-entropy estimate for raw samples with pseudorandom loop counts.

Recall that jitterentropy's processed entropy is formed from 64 separate random
data samples, mixed together in a 64-bit internal state buffer. Each of the raw
samples corresponds to a sample in the `raw = true` table. In particular, it's
absurd to think that combining 64 samples with `ml = 1` and `ll = 1` then
processing these could produce (5.77 \* 8) = 46.2 bits of entropy per 8 bytes of
processed output, since that would imply (46.2 / 64) = 0.72 bits of entropy per
unprocessed sample as opposed to the measured value of 0.20 bits.

This argument applies against the `ml = 1`, `ll = 1`, `raw = false` measurement,
but does *not* apply to `ml = 128`, `ll = 16`, `raw = false`. In particular,
combining 64 raw samples with `ml = 128` and `ll = 16` could in principle
collect (1.64 \* 64 / 8) = 13.1 bits of entropy per processed byte, except that
of course there is a hard limit at 8 bits per byte.

Interestingly, the minimal entropy estimator switches from the compression
estimate to the Markov estimator. My theory is that the additional "confusion"
from post-processing is enough to "fool" the compression estimate. If there is a
cryptographic vulnerability in the jitterentropy processing routine, it may be
possible to write a similar estimator that reveals a significantly smaller
min-entropy. If we use the general-purpose tests to decide how many raw samples
to collect in order to have 256 of min-entropy, but an adversary uses a targeted
attack, then (relative to this targeted attack) our system may have less entropy
in its entropy pool than we expect. This is a security vulnerability.

If there is a very bad weakness in the jitterentropy processing routine, it may
in fact be reducing the "true" entropy in jitterentropy's internal pool. The
arithmetical argument regarding `ml = 1` and `ll = 1` shows that we can't trust
the NIST test suite to accurately measure the actual min-entropy in the
processed data, so it is possible that the processing actually reduces
min-entropy and our tools just can't detect the loss. This would exacerbate the
vulnerability described in the previous paragraph.

## Conclusions

Jitterentropy's pseudorandom loop counts are of questionable benefit at best,
and if used they force us to make a security/efficiency trade-off. Unless we can
show convincing evidence that the pseudorandom times really do drastically
increase entropy estimates rather than just defeating the NIST test suite, we
should use deterministic loop counts, ideally tuned for performance on a
per-target basis.

Jitterentropy's processing is also questionable, since (to my knowledge) it
hasn't been subjected to enough cryptographic analysis and testing to be
trusted. Furthermore, we can't directly measure the min-entropy in the
post-processed data via the NIST test suite, so if there is a cryptographic
vulnerability we can't easily detect it. I think we should instead rely on the
entropy mixing code in the Zircon CPRNG (based on SHA-2), and leave
jitterentropy's processing disabled.

## TODOs

1. Repeat the tests reported above against different versions of Zircon, and
   ensure that the entropy estimates remain consistent.
2. Repeat the tests on different platforms and targets (note: x86 targets don't
   currently have access to a system clock during early boot, so the early boot
   entropy tests and early boot CPRNG seeding don't yet support jitterentropy on
   x86).
3. Automate the process of running the tests and generating the reports in this
   document. Specifically, the tests should compare:

   - pseudorandom loop counts versus various deterministic loop count values
   - raw samples versus processed data
