# Entropy quality tests

This document describes how we test the quality of the entropy sources used to
seed the Zircon CPRNG.

[TOC]

## Theoretical concerns

Approximately speaking, it's sometimes easy to tell that a stream of numbers is
not random by recognizing a pattern in it. It's impossible to be sure that the
numbers are truly random. The state of the art seems to be running several
statistical tests on the data, and hoping to detect any exploitable weaknesses.

The problem of testing for randomness gets more difficult when the random
numbers aren't perfectly random (when their distributions aren't uniform, or
when there are some limited correlations between numbers in the sequence). A
stream of non-perfect random numbers still contains some randomness, but it's
hard to determine how random it is.

For our purposes, a good measure of how much randomness is contained in a stream
of non-perfectly random numbers is the min-entropy. This is related to the
Shannon entropy used in information theory, but is always takes a smaller value.
The min-entropy controls how much randomness we can reliably extract from the
entropy source; see, for example
<https://en.wikipedia.org/wiki/Randomness_extractor#Formal_definition_of_extractors>

From a practical standpoint, we can use the test suite described in US NIST
SP800-90B to analyze samples of random from an entropy source. A prototype
implementation for the tests is available from
<https://github.com/usnistgov/SP800-90B_EntropyAssessment>. The suite takes a
sample data file (say, 1MB of random bytes) as input. The nice thing about this
test suite is that it can handle non-perfect RNGs, and it reports an estimate
for how much min-entropy is contained in each byte of the random data stream.

### The importance of testing unprocessed data

After drawing entropy from our entropy sources, we will mix it into the CPRNG in
a "safe" way that basically gets rid of detectable correlations and
distributional imperfections in the raw random byte stream from the entropy
source. This is a very important thing to do when actually generating random
numbers to use, but we must avoid this mixing and processing phase when testing
the entropy source itself.

For a stark example of why it's important to test unprocessed data if we want to
test our actual entropy sources, here's an experiment. It should run on any
modern linux system with OpenSSL installed.

    head -c 1000000 /dev/zero >zero.bin
    openssl enc -aes-256-ctr -in zero.bin -out random.bin -nosalt -k "password"

This takes one million bytes from /dev/zero, encrypts them via AES-256, with a
weak password and no salt (a terrible crypto scheme, of course!). The fact that
the output looks like good random data is a sign that AES is working as
intended, but this demonstrates the risk of estimating entropy content from
processed data: together, /dev/zero and "password" provide ~0 bits of entropy,
but our tests are way more optimistic about the resulting data!

For a more concrete Zircon-related example, consider jitterentropy (the RNG
discussed here: <http://www.chronox.de/jent/doc/CPU-Jitter-NPTRNG.html>).
Jitterentropy draws entropy from variations in CPU timing. The unprocessed data
are how long it took to run a certain block of CPU- and memory-intensive code
(in nanoseconds). Naturally, these time data are not perfectly random: there's
an average value that they center around, with some fluctuations. Each
individual data sample might be several bits (e.g. a 64-bit integer) but only
contribute 1 bit or less of min-entropy.

The full jitterentropy RNG code takes several raw time data samples and
processes them into a single random output (by shifting through a LFSR, among
other things). If we test the processed output, we're seeing apparent randomness
both from the actual timing variations and from the LFSR. We want to focus on
just the timing variation, so we should test the raw time samples. Note that
jitterentropy's built-in processing can be turned on and off via the
`kernel.jitterentropy.raw` cmdline.

## Quality test implementation

As mentioned above, the NIST test suite takes a file full of random bytes as
input. We collect those bytes on a Zircon system (possibly with a thin Fuchsia
layer on top), then usually export them to a more capable workstation to run the
test suite.

## Boot-time tests

Some of our entropy sources are read during boot, before userspace is started.
To test these entropy sources in a realistic environment, we run the tests
during boot. The relevant code is in
`kernel/lib/crypto/entropy/quality\_test.cpp`, but the basic idea is that the
kernel allocates a large static buffer to hold test data during early boot
(before the VMM is up, so before it's possible to allocate a VMO). Later on, the
data is copied into a VMO, and the VMO is passed to userboot and devmgr, where
it's presented as a pseudo-file at `/boot/kernel/debug/entropy.bin`. Userspace
apps can read this file and export the data (by copying to persistent storage or
using the network, for example).

In theory, you should be able to build Zircon with entropy collector testing
enabled using `scripts/entropy-test/make-parallel`, and then you should be able
to run a single boot-time test with the script
`scripts/entropy-test/run-boot-test`. The `run-boot-test` script is mostly
intended to be invoked by other scripts, so it's a little bit rough around the
edges (for example, most of its arguments are passed via command line options
like `-a x86-64`, but many of these "options" are in fact mandatory).

Assuming the `run-boot-test` script succeeds, it should produce two files in the
output directory: `entropy.000000000.bin` and `entropy.000000000.meta`. The
first is the raw data collected from the entropy source, and the second is a
simple text file, where each line is a key-value pair. The keys are single words
matching `/[a-zA-Z0-9_-]+/`, and the values are separated by whitespace matching
`/[ \t]+/`. This file can be pretty easily parsed via `read` in Bash,
`str.split()` in Python, or (with the usual caution about buffer overruns)
`scanf` in C.

In practice, I'm nervous about bit-rot in these scripts, so the next couple
sections document what the scripts are supposed to do, to make it easier to run
the tests manually or fix the scripts if/when they break.

### Boot-time tests: building

Since the boot-time entropy test requires that a large block of memory be
permanently reserved (for the temporary, pre-VMM buffer), we don't usually build
the entropy test mode into the kernel. The tests are enabled by passing the
`ENABLE_ENTROPY_COLLECTOR_TEST` flag at build time, e.g. by adding the line

```
EXTERNAL_DEFINES += ENABLE_ENTROPY_COLLECTOR_TEST=1
```

to `local.mk`. Currently, there's also a build-time constant,
`ENTROPY_COLLECTOR_TEST_MAXLEN`, which (if provided) is the size of the
statically allocated buffer. The default value if unspecified is 1MiB.

### Boot-time tests: configuring

The boot-time tests are controlled via kernel cmdlines. The relevant cmdlines
are `kernel.entropy-test.*`, documented in
[kernel\_cmdline.md](/docs/reference/kernel/kernel_cmdline.md).

Some entropy sources, notably jitterentropy, have parameter values that can be
tweaked via kernel cmdline. Again, see [kernel\_cmdline.md](/docs/reference/kernel/kernel_cmdline.md)
for further details.

### Boot-time tests: running

The boot-time tests will run automatically during boot, as long as the correct
kernel cmdlines are passed (if there are problems with the cmdlines, error
messages will be printed instead). The tests run just before the first stage of
RNG seeding, which happens at LK\_INIT\_LEVEL\_PLATFORM\_EARLY, shortly before
the heap the VMM are brought up. If running a large test, boot will often slow
down noticeably. For example, collecting 128kB of data from jitterentropy on
rpi3 can take around a minute, depending on the parameter values.

## Run-time tests

*TODO(fxbug.dev/24760): discuss actual user-mode test process*

*Current rough ideas: only the kernel can trigger hwrng reads. To test,
userspace issues a kernel command (e.g. `k hwrng test`), with some arguments to
specify the test source and length. The kernel collects random bytes into the
existing VMO-backed pseudo-file at `/boot/kernel/debug/entropy.bin`, assuming
that this is safely writeable. Currently unimplemented; blocked by lack of a
userspace HWRNG driver. Can test the VMO-rewriting mechanism first.*

## Test data export

Test data is saved in `/boot/kernel/debug/entropy.bin` in the Zircon system
under test. So far I've usually exported the data file manually via `netcp`.
Other options include `scp` if you build with the correct Fuchsia packages, or
saving to persistent storage.

## Running the NIST test suite

*Note: the NIST tests aren't actually mirrored in Fuchsia yet. Today, you need
to clone the tests from the repo at
<https://github.com/usnistgov/SP800-90B_EntropyAssessment>.*

The NIST test suite has three entry points (as of the version committed on Oct.
25, 2016): `iid_main.py`, `noniid_main.py`, and `restart.py`. The two "main"
scripts perform the bulk of the work. The `iid_main.py` script is meant for
entropy sources that produce independent, identically distributed data samples.
Most of the testing is to validate the iid condition. Many entropy sources will
not be iid, so the `noniid_main.py` test implements several entropy estimators
that don't require iid data.

Note that the test binaries from the NIST repo are Python scripts without a
shebang line, so you probably need to explicitly call `python` on the command
line when invoking them.

The first two scripts take two arguments, both mandatory: the data file to read,
and the number of significant bits per sample (if less than 8, only the low `N`
bits will be used from each byte). They optionally accept a `-v` flag to produce
verbose output or `-h` for help.

The `noniid_main.py` also optionally accepts a `-u <int>` flag that can reduce
the number of bits below the `N` value passed in the second mandatory argument.
I'm not entirely sure why this flag is provided; it seems functionally
redundant, but passing it does change the verbose output slightly. My best guess
is that this is provided because the noniid Markov test only works on samples of
at most 6 bits, so 7- or 8-bit datasets will be reduced to their low 6 bits for
this test. In contrast, all the iid tests can run on 8-bit samples.

A sample invocation of the `iid_main.py` script:

```
python2 -- $FUCHSIA_DIR/third_party/sp800-90b-entropy-assessment/iid_main.py -v /path/to/datafile.bin 8
```

The `restart.py` script takes the same two arguments, plus a third argument: the
min-entropy estimate returned by a previous run of `iid_main.py` or
`noniid_main.py`. This document doesn't describe restart tests. For now, see
NIST SP800-90B for more details.

## Future directions

### Automation

It would be nice to automate the process of building, configuring, and running a
quality test. As a first step, it should be easy to write a shell script to
perform these steps. Even better would be to use the testing infrastructure to
run entropy collector quality tests this automatically, mostly to reduce bit-rot
in the test code. Failing automation, we have to rely on humans to periodically
run the tests (or to fix the tests when they break).
