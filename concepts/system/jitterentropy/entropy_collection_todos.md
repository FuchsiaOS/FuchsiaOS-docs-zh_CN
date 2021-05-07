# Entropy collection TODOs

I'm writing this at the end of my internship to record some of the things I didn't get to.

[TOC]

## Proper use of RdRand

On x86, `RdRand` reads from a deterministic CPRNG (which is seeded from a hardware entropy source).
The newer `RdSeed` instruction reads from the underlying entropy source directly (well, with some
post-processing). Currently, we prefer to use `RdSeed` but if that isn't available we fall back on
`RdRand`. However, we just draw random bits directly from `RdRand`, in contravention of the Intel
HWRNG guide
([online here](https://software.intel.com/en-us/articles/intel-digital-random-number-generator-drng-software-implementation-guide);
see section 4.2.5 "Guaranteeing DBRG Reseeding"). We should fix that.

## Reseeding the CPRNG during runtime

My hacky virtio driver will reseed the CPRNG on qemu (on a five minute recurring timer). I think
that's the only entropy source that is currently used to reseed after system startup.

As a start, we should be able to use the entropy sources built into the kernel (RdRand and
jitterentropy). Just running these on a periodic timer would improve our reseeding story. Note that
once every 5 minutes is probably more often than we need.

We've talked about reseeding more often if large amounts of data have been drawn from the CPRNG (on
the order of 2^48 bits, I think).

## Monitoring entropy sources

Entropy sources can potentially fail, either totally or partially.

Total failures like "the device was unplugged" or "the device is not responding to I/O" will
hopefully be reported by the hardware layer.

Partial failures, where the device returns data but with less entropy than expected, are scarier. We
should run simple health tests to try to detect partial failures. See for example the continuous
health tests in NIST SP800-90B, section 4.4. The health tests there are pretty simple and require
minimal resources. They do require storing some statistics about recent entropy source outputs,
which presents some security risk.

The NIST SP also suggests (well, requires, but I'm not aware of any immediate plans for
certification) running startup tests. The NIST startup tests involve running the continuous tests
over at least 4096 samples (see section 4.3 #12), after which these samples may be reused to seed
the CPRNG.

Once monitoring is in place, we need to decide how to respond to entropy source failures. If one of
six different entropy sources fails, we might treat that as a minor hardware failure that gets
logged. If the system has only one entropy source and it fails, we need to take more drastic action
(on the order of shutting off the CPRNG or halting the system).

## Userspace RNG drivers

Once DDK settles down, we should add to and improve our RNG drivers. Currently, there are two
RNG-related drivers: TPM and virtio-rng.

An important requirement is to restrict access to the `zx_cprng_add_entropy` syscall, via a Resource
or similar mechanism. We should also use this to differentiate between the devices providing
entropy, for monitoring purposes. It would also be nice if the kernel can send start/stop signals to
the drivers through this Resource.

Here are some currently unused entropy sources to consider:

- There's an existing TPM driver, which calls `cprng_add_entropy` in its `bind()` callback. We
  should add support for TPM 2.0, for better coverage.

- There are plenty of commercially available hardware RNGs, often connecting over USB. We could add
  drivers for those, but it probably makes sense to expect third party drivers instead.

- There's also apparently a hardware RNG built into the SoC in Raspberry Pis, according to
  [the Raspberry Pi forums](https://www.raspberrypi.org/forums/viewtopic.php?f=29&t=19334&p=273944#p273944).
  In general we could check other specific targets (i.e. not "pc-x86-64") for hardware RNGs and wire
  those up. If we're lucky, many of these will be accessible from the kernel for use during or
  immediately after boot.

- Finally, we could record entropy from hardware IRQs, especially for hard disks, network cards,
  input devices, and other classic entropy sources. This won't be anywhere near as fast as a
  dedicated hardware RNG, but it's attractive since a few lines of code added in the right places in
  our driver stack should enable entropy collection from a wide variety of very common devices.

## Jitterentropy

### Replace the noise-generating functions by assembly, and remove '-O0'

Right now, jitterentropy is compiled at optimization level `-O0` (as per the author's
documentation). The reason is the two noise-generating functions: `jent_lfsr_time` and
`jent_memaccess`. We should replace these C functions by assembly code (probably by compiling with
flags `-S -O0`), then compile the rest of jitterentropy with optimizations enabled. After this, we
should re-test to make sure our entropy estimates remain accurate.

### Test jitterentropy more thoroughly

I've been testing on the same handful of physical devices. We should test jitterentropy on a few
other PCs, RPis, etc.

### Test jitterentropy at runtime

Right now, jitterentropy only runs (and was only tested) during the single-core part of the boot
sequence. We should test jitterentropy during SMP runtime, and consider whether we need to (say)
disable interrupts or pin ourselves to a CPU inside jitterentropy.

### More tuning

See [the tuning doc](/docs/concepts/system/jitterentropy/config-tuning.md). The current universally hard-coded parameters
seem to be decent, so this probably isn't incredibly urgent. Still, since jitterentropy is on the
critical path for every single boot and since it will run during runtime as well (hopefully soon!),
it's probably worth optimizing at some point.

We should probably at least tune jitterentropy on a per-architecture basis, and ideally per-target.
Note that right now, the `entropy_per_1000_bytes` statistic in
`kernel/lib/crypto/entropy/jitterentropy_collector.cpp` is hard-coded and not arch/target dependent.
That should probably also be configurable.

## Cloning the NIST test suite

We may want to clone the NIST test suite into Fuchsia third\_party. This would help us to automate
the testing and analysis of our entropy sources (Jitterentropy in particular).

