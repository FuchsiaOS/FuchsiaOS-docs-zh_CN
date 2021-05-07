# Architecture Support

Fuchsia supports two ISAs: arm64 and x86-64.

## arm64

Fuchsia supports arm64 (also called AArch64) with no restrictions on
supported microarchitectures.

## x86-64

Fuchsia supports x86-64 (also called IA32e or AMD64), but with some restrictions
on supported microarchitectures.

### Intel

For Intel CPUs, only Broadwell and later are actively supported and will have new features added for them.  Additionally, we will accept patches to keep Nehalem and later booting.

### AMD

AMD CPUs are not actively supported (in particular, we have no active testing on them), but we will accept patches to ensure correct booting on them.
