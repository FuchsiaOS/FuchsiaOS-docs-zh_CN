# Invest in Sanitizers

- Project leads: phosek@google.com, shayba@google.com
- Area: Toolchain

## Problem statement

Memory safety bugs continue to be the root cause of high-severity bugs that
affect security. The presence of sanitizers also improves engineering
productivity, by quickly exposing and root-causing difficult bugs. Despite
Fuchsia’s relative strengths in this domain, such as stronger isolation between
different software and a growing investment in memory-safe system programming
languages, memory safety remains a concern on Fuchsia as it is
[on other platforms][queue-hardening-enhancements].

Currently Fuchsia uses several [sanitizers] to detect memory safety bugs:

- [AddressSanitizer][asan] (ASan) detects instances of out-of-bounds access, use
  after free / return / scope, and double free. Relatedly, [kasan] extends this
  to kernel code.
- [LeakSanitizer][lsan] (LSan) detects memory leaks.
- [UndefinedBehaviorSanitizer][ubsan] (UBSan) detects specific issues of relying
  on undefined program behavior.
- Relatedly, [libFuzzer] is supported on Fuchsia to run coverage-directed
  [fuzz testing][fuzz-testing] and detect crashes or issues that are detectable
  by the above sanitizers. There is [ongoing work][rfc-0078] to improve kernel
  syscall fuzzing by making it coverage-directed.
- Lastly, [GWP-ASan], a sampling version of asan, is supported on Fuchsia.
  Work is underway to demonstrate its use in production to detect bugs in the
  field.

These sanitizers cover C/C++ code where memory safety isn’t guaranteed. They
also detect memory bugs in [Rust unsafe blocks][rust-unsafe] and can find
[memory leaks in Rust code][rust-leaking].

These tools have [proven to be effective at finding bugs][fxb-sanitizers]. They
require no effort from the developer when bugs aren’t detected. When bugs are
detected, troubleshooting is relatively easy since sanitizers provide stack
traces for easy root cause analysis and since they exhibit reproducible
behavior.

Efforts in 2020-2021 to roll out all three sanitizers broadly and to fix
pre-existing bugs were successful. These efforts leaned on prior dedicated work
to bring up runtime instrumentation support on Fuchsia. They were staffed
temporarily by volunteers and 20%ers, and have since concluded.

However we continue to see room for improvement. Particularly:

- Some hardware-dependent code isn’t covered by sanitizers, mostly due to
  runtime performance issues and automation gaps.
- Sanitizers for kernel code lag behind those for user space code.
- There exist additional classes of severe bugs that have sanitizer support
  elsewhere but not yet on Fuchsia, particularly for uninitialized reads and a
  variety of thread-safety bugs.
- Bugs that sanitizers detect in the system outside the boundaries of a
  particular test are not root-caused automatically, requiring manual triage to
  assign them to an owner.

The first two issues are particularly concerning at a time when the Fuchsia team
is investing more in device driver development and out-of-tree development, as
well as both of those combined. The other issues are ongoing deficiencies that
lower engineering productivity.

## Solution statement

Increase our investment in sanitizers, building up on previous investments in
LLVM compiler runtime instrumentation support. Specifically:

**Bring up support for more sanitizers**, such as:

- [Hardware-accelerated AddressSanitizer][hwasan] (hwasan) which significantly
  reduces the memory overhead of asan and makes instrumentation viable on
  RAM-constrained devices.
- [ThreadSanitizer][tsan] (TSan) which detects data races, building on
  [prior work][fxr-362634-359540].
- Kernel support for concurrency sanitization (kcsan).
- [MemorySanitizer][msan] (MSan) which detects reads from uninitialized memory.

**Fix long-standing issues with existing sanitizers**, such as:

- [Known UBSan bugs][ubsan-open-project].
- Long-standing lsan issues such as [testing gaps][fxb56628] and
  [races][fxb66819].
- Engage with engineering teams to build a culture of fixing sanitizer bugs. Pay
  down the tech debt of pre-existing sanitizer bugs.

**Investigate opportunities and present a roadmap** for future work, such as:

- Enabling sanitizers on more configurations (beyond qemu) in such a way that
  tracks Fuchsia’s priorities.
- Identifying exactly what code is not exercised under instrumentation (such as
  drivers and other hardware-dependent code), and closing these gaps by priority
  order.
- Measure and quantify the impact of sanitizers on Fuchsia - the magnitude and
  distribution of issues found, their severity, time from detection to repair,
  an inventory of tech debt and a plan to conquer it.
- Research new opportunities, for instance leveraging
  [hardware support for sanitizers][arm-mte] which is already seeing adoption
  [on other platforms][arm-mte-android], or making additional optimizations such
  as inlining viable in instrumented builds.
- Consider growing our investment in sanitizers for Rust, for instance
  overcoming FFI boundaries when checking for memory safety issues or
  introducing a UBSan equivalent for Rust.

## Dependencies

- Sanitizer bringup work will rely on LLVM expertise that is present in the
  Fuchsia Toolchain team and should be extended to more individuals to promote
  team health.
- Hardware-accelerated features such as top-byte ignore (TBI) require kernel
  support across all syscalls that pass pointers.
- Exercising sanitizers on hardware will require lab device provisioning and
  changes to build & test automation capacity and configuration by the EngProd
  team.
- Sanitizer variants and various toggles need to be supported by the Fuchsia
  Build team.

Note that all of the above teams are already committed to sanitizers work in
principle, and meet regularly.

## Risks and mitigations

- Some efforts, particularly bringup, can take a couple of years to demonstrate
  and validate. This requires long term commitment and dedication to the work,
  as well as patience.
- Some expectations on the viability of instrumentation on hardware are
  speculative, assuming positive results that track prior art on other platforms
  and subject to specific and undisclosed target hardware choices.

[arm-mte]: https://community.arm.com/developer/ip-products/processors/b/processors-ip-blog/posts/enhancing-memory-safety
[arm-mte-android]: https://security.googleblog.com/2019/08/adopting-arm-memory-tagging-extension.html
[asan]: https://clang.llvm.org/docs/AddressSanitizer.html
[fuzz-testing]: /contribute/testing/fuzz_testing.md
[fxb-sanitizers]: https://bugs.fuchsia.dev/p/fuchsia/issues/list?q=label%3Aasan%20OR%20label%3Alsan%20OR%20label%3Aubsan%20OR%20label%3Alibfuzzer%20OR%20label%3AClusterFuzz&can=2
[fxb56628]: https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=56628
[fxb66819]: https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=66819
[fxr-362634-359540]: https://fuchsia-review.googlesource.com/q/change:362634+OR+change:359540
[gwp-asan]: https://llvm.org/docs/GwpAsan.html
[hwasan]: https://clang.llvm.org/docs/HardwareAssistedAddressSanitizerDesign.html
[kasan]: https://fuchsia.googlesource.com/fuchsia/+/refs/heads/main/zircon/kernel/lib/instrumentation/asan/README.md
[libfuzzer]: https://llvm.org/docs/LibFuzzer.html
[lsan]: https://clang.llvm.org/docs/LeakSanitizer.html
[msan]: https://clang.llvm.org/docs/MemorySanitizer.html
[queue-hardening-enhancements]: https://security.googleblog.com/2019/05/queue-hardening-enhancements.html
[rfc-0078]: /contribute/governance/rfcs/0078_kernel_coverage_for_fuchsia_fuzzing.md
[rust-leaking]: https://doc.rust-lang.org/nomicon/leaking.html
[rust-unsafe]: https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html
[sanitizers]: /contribute/testing/sanitizers.md
[tsan]: https://clang.llvm.org/docs/ThreadSanitizer.html
[ubsan]: https://clang.llvm.org/docs/UndefinedBehaviorSanitizer.html
[ubsan-open-project]: /contribute/open_projects/cpp/ubsan.md
