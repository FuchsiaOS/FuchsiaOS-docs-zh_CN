#Cryptographically Secure Pseudo Random Number Generator

This document describes the design of Zircon's Cryptographically secure pseudo
random number generator (CPRNG), including its algorithm, (re)seeding process,
and entropy sources.

## Introduction

Zircon's built-in CPRNG provides cryptographically secure pseudorandom data in a
non-blocking fashion. User space programs can access them through the
[`zx_cprng_draw()`](/docs/reference/syscalls/cprng_draw.md) syscall.

Zircon's CPRNG only trusts entropy sources directly accessible from within the
kernel because anything outside the kernel such as the drivers, which are
considered userspace programs, cannot be trusted. For the CPRNG to function
properly and securely, at least one of these sources is required. However,
userspace programs may inject additional entropy to CPRNG through the
[`zx_cprng_add_entropy()`](/docs/reference/syscalls/cprng_add_entropy.md)
syscall.


## Algorithms

Zircon's CPRNG is a pseudorandom number generator. Its implementation is located
at `zircon/kernel/lib/crypto`. It supports two operations, `Draw()` and
`AddEntropy()`, which corresponds to the two syscalls mentioned above. The
internal state consists of a 256-bit `key` and a 128-bit `nonce`. The `key` must
be kept secret because the CPRNG output can be reliably predicted with the
knowledge of it. At the beginning, `key` is initialized with some random bytes
(see next section) and `nonce` is initialized as 0.

When the `Draw()` method is called:

1. `nonce` is incremented.

1. The output buffer is encrypted using the ChaCha20 algorithm with `key` and
`nonce`.

Here `nonce` is incremented for every `Draw()` request to ensure different
results. The caller provides a buffer to perform the encryption in-place. Any
existing data in the buffer is used since they does not affect the security
properties.

When there is a `AddEntropy()` request, the `key` is updated by mixing
additional entropy with the old key:

```
k<sub>new</sub> = H(e || k<sub>old</sub>)
```

where `k<sub>old</sub>` and `k<sub>new</sub>` are the old and new `key`,
respectively, `e` is the input bytes, `H` is the SHA256 hash function and `||`
denotes concatenation. The old key is included in the hash to ensure that
callers, e.g. userspace programs that call `zx_cprng_add_entropy()`, cannot
purge the old `key` and replace it with something these programs control.

## Seeding and Reseeding

A call to the `AddEntropy()` method performs the initial seeding of the Zircon
CPRNG. The initial seeding is needed for virtual memory ASLR, so the first call
to the `AddEntropy()` method occurs very early in the boot sequence before the
userspace starts. The initial seeding is required for the CPRNG to function as
the `Draw()` method blocks until enough entropy is added.

After the initial seeding, a thread is created to reseed the CPRNG every 30
seconds by calling the `AddEntropy()` method. This ensures forward secrecy (a
guarantee of secrecy for all the CPRNG's previous output since the last reseed,
even if its internal state is compromised).

## Entropy sources

There are several entropy sources Zircon's CPRNG can utilize for seeding and
reseeding:

* Entropy from kernel cmdline option `kernel.entropy-mixin`, documented in
[kernel\_cmdline.md](/docs/reference/kernel/kernel_cmdline.md).

* Entropy from hardware RNG such as the `RDSEED` instruction on x86 devices and
other hardware specific RNGs.

* [Jitter Entropy](/docs/concepts/system/jitterentropy/README.md)

The kernel cmdline is only used at initial seeding because it is a constant
passed in at boot for one-time use only. The entropy from hardware and jitter
entropy can be used for both initial seeding and reseeding. To ensure the CPRNG
is sufficiently (re)seeded from the selected entropy sources, you can use the
kernel cmdline `kernel.cprng-(re)seed-require.*` options. For more information,
see [kernel_cmdline.md](/docs/reference/kernel/kernel_cmdline.md).

There may be other available entropy sources such as a trusted platform module
(TPM), but we do not currently have a strong framework in place for userspace
programs to securely communicate with the CPRNG subsystem in the kernel.

