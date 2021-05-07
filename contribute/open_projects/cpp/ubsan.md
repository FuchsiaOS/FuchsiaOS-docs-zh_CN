# Undefined behavior issues (UBSan)

## Goal & motivation

C/C++ code may be subject to undefined behaviors at runtime. Common causes for
undefined behavior include:

*   Using misaligned or null pointer.
*   Signed integer overflow.
*   Conversion to, from, or between floating-point types that would overflow
    the destination.

To catch these issues at scale, we introduced support for
[Undefined Behavior Sanitizer][ubsan] (UBSan) in Fuchsia. This runtime check was
enabled on CQ by default in 2020, with pre-existing violations suppressed.
Though the majority of issues found were fixed, and new issues are kept from
being introduced to the source tree, there are still
[open UBSan issues][label-ubsan-open] that deserve attention.

## Technical background

Proficiency at C/C++ programming is required.

## How to help

### Picking a task

Review [open UBSan issues][label-ubsan-open] and pick a bug to fix.

Alternatively, look for build targets with suppressions:

```gn
source_set("foo") {
  ...
  # TODO(fxbug.dev/xxxxx): UBSan has found an instance of undefined behavior in this target.
  # Disable UBSan for this target temporarily until it is migrated into CI/CQ.
  public_configs += [ "//build/config:temporarily_disable_ubsan_do_not_use" ]
}
```

You can focus on code that you own, or pick random targets throughout the tree.

### Doing a task

Your first step is to remove the suppression and address the issue. UBSan
provides troubleshooting information by logging a root cause and a stack trace
during a test, when the issue was surfaced at runtime. For instance:

{# Disable variable substition to avoid '{{' being interpreted by the template engine #}
{% verbatim %}

```
[2105.728] 1054084.1055594> ../../src/connectivity/wlan/lib/common/cpp/include/wlan/common/element.h:769:48: runtime error: upcast
[2105.728] 1054084.1055594> of misaligned address 0x245287a88d03 for type 'wlan::SupportedMcsRxMcsHead', which requires 8 byte alignment
[2105.729] 1054084.1055594> 0x245287a88d03: note: pointer points here
[2105.729] 1054084.1055594>  62  fe 01 00 ff 00 00 00 01  00 00 00 00 00 00 00 01  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00
[2105.729] 1054084.1055594>               ^
2020/02/20 21:52:42.559678 WARN: on line 16830: could not find module for 0x22ec07646c96
2020/02/20 21:52:42.559792 WARN: on line 16831: could not find module for 0x22ec076466e9
[2105.734] 1054084.1055594>    #0    0x000022ddedc71a41 in wlan::HtCapabilities::ToDdk() const ../../out/default/../../src/connectivity/wlan/lib/common/cpp/include/wlan/common/element.h:769 <<VMO#1054220=wlantap.so>>+0x1dba41
[2105.734] 1054084.1055594>    #1    0x000022ec07646c96 in <>+0x38c96
[2105.734] 1054084.1055594>    #2    0x000022ec076466e9 in <>+0x386e9
[2105.734] 1054084.1055594>    #3    0x000022ddedc71a41 in wlan::HtCapabilities::ToDdk() const ../../out/default/../../src/connectivity/wlan/lib/common/cpp/include/wlan/common/element.h:769 <<VMO#1054220=wlantap.so>>+0x1dba41
2020/02/20 21:49:12.077658 testrunner attempting to close SSH session due to: failed to run SSH command: Process exited with status 101
2020/02/20 21:49:12.077681 testrunner ERROR: failed to send KILL signal over SSH session: EOF
2020/02/20 21:49:12.077689 testrunner ERROR: failed to close SSH session: EOF
[2105.735] 1054084.1055594> {{2020/02/20 21:49:12.081620 testrunner ERROR: failed to run SSH command: Process exited with status 101
not ok 138 fuchsia-pkg://fuchsia.com/wlan-hw-sim-test#meta/configure_legacy_privacy_off.cmx (4.259382565s)
{bt:4:0x22ddedc6d904}}}
[2105.735] 1054084.1055594>    #5    0x000022ddedc6f173 in wlan::ConvertPhyInfo(wlan_info*, fuchsia::wlan::device::PhyInfo const&) ../../out/default/../../src/connectivity/wlan/testing/wlantap-driver/utils.cc:167 <<VMO#1054220=wlantap.so>>+0x1d9173
[2105.735] 1054084.1055594>    #6    0x000022ddedc9521a in wlan::(anonymous namespace)::WlantapPhy::Query(wlanphy_impl_info*) ../../out/default/../../src/connectivity/wlan/testing/wlantap-driver/wlantap-phy.cc:186 <<VMO#1054220=wlantap.so>>+0x1ff21a
[2105.735] 1054084.1055594>    #7    0x000022ddedc94f90 in wlan::$_0::operator()(void*, wlanphy_impl_info*) const ../../out/default/../../src/connectivity/wlan/testing/wlantap-driver/wlantap-phy.cc:410 <<VMO#1054220=wlantap.so>>+0x1fef90
[2105.735] 1054084.1055594>    #8    0x000022ddedc94e8c in wlan::$_0::__invoke(void*, wlanphy_impl_info*) ../../out/default/../../src/connectivity/wlan/testing/wlantap-driver/wlantap-phy.cc:409 <<VMO#1054220=wlantap.so>>+0x1fee8c
[2105.735] 1054084.1055594>    #9    0x000021a1e2e3d3e4 in wlanphy::Device::Query(fit::function_impl<16ul, false, void (fuchsia::wlan::device::QueryResponse)>) ../../out/default/../../src/connectivity/wlan/drivers/wlanphy/device.cc:260 <<VMO#1055531=wlanphy.so>>+0x1ab3e4
[2105.735] 1054084.1055594>    #10   0x000021a1e2e9e77e in fuchsia::wlan::device::Phy_Stub::Dispatch_(fidl::Message, fidl::internal::PendingResponse) ../../out/default/fidling/gen/sdk/fidl/fuchsia.wlan.device/fuchsia/wlan/device/cpp/fidl.cc:739 <<VMO#1055531=wlanphy.so>>+0x20c77e
[2105.735] 1054084.1055594>    #11   0x000021a1e2fa9a26 in fidl::internal::StubController::OnMessage(fidl::Message) ../../out/default/../../sdk/lib/fidl/cpp/internal/stub_controller.cc:30 <<VMO#1055531=wlanphy.so>>+0x317a26
[2105.735] 1054084.1055594>    #12   0x000021a1e2f916b4 in fidl::internal::MessageReader::ReadAndDispatchMessage(fidl::MessageBuffer*) ../../out/default/../../sdk/lib/fidl/cpp/internal/message_reader.cc:235 <<VMO#1055531=wlanphy.so>>+0x2ff6b4
[2105.735] 1054084.1055594>    #13   0x000021a1e2f91bde in fidl::internal::MessageReader::OnHandleReady(async_dispatcher*, int, zx_packet_signal const*) ../../out/default/../../sdk/lib/fidl/cpp/internal/message_reader.cc:179 <<VMO#1055531=wlanphy.so>>+0x2ffbde
[2105.735] 1054084.1055594>    #14   0x000021a1e2f8f13c in fidl::internal::MessageReader::CallHandler(async_dispatcher*, async_wait*, int, zx_packet_signal const*) ../../out/default/../../sdk/lib/fidl/cpp/internal/message_reader.cc:166 <<VMO#1055531=wlanphy.so>>+0x2fd13c
[2105.735] 1054084.1055594>    #15.1 0x000021a1e3273e31 in async_loop_run_once ../../out/default/../../zircon/system/ulib/async-loop/loop.c:0 <<VMO#1055531=wlanphy.so>>+0x5e1e31
[2105.735] 1054084.1055594>    #15   0x000021a1e3273e31 in async_loop_run ../../out/default/../../zircon/system/ulib/async-loop/loop.c:253 <<VMO#1055531=wlanphy.so>>+0x5e1e31
[2105.735] 1054084.1055594>    #16   0x000021a1e3275b76 in async_loop_run_thread ../../out/default/../../zircon/system/ulib/async-loop/loop.c:799 <<VMO#1055531=wlanphy.so>>+0x5e3b76
[2105.736] 1054084.1055594>    #17   0x000041f10d1f067e in start_c11 ../../out/default.zircon/../../zircon/third_party/ulib/musl/pthread/pthread_create.c:37 <libc.so>+0xaa67e
[2105.736] 1054084.1055594>    #18   0x000041f10d3017ad in thread_trampoline ../../out/default.zircon/../../zircon/system/ulib/runtime/thread.c:93 <libc.so>+0x1bb7ad
[2105.736] 1054084.1055594>
```

{# Re-enable variable substition #}
{% endverbatim %}

If you're picking up a UBSan bug then the bug will already have this information
in it, although depending on when the bug was filed the information might be out
of date. Alternatively you can attempt to reproduce the issue such as by running
the tests with the suppression removed or sending the change to CQ.

If you cannot reproduce the issue and the change with the suppression removed
passes CQ, then proceed to the next section. This can happen if the offending
code was fixed or removed but the author of the change left the suppression/bug
intact.

Additional information about UndefinedBehaviorSanitizer and the available checks
can be found in the [upstream docs][ubsan].

### Completing a task

Tag the cover bug in your change description as follows:

```
Fixed: xxxxx
```

Find reviewers via owners and merge your change.

## Examples

*   [464343: [volume_image] Fix ubsan issue](https://fuchsia-review.googlesource.com/c/fuchsia/+/464343)
*   [461437: \[ubsan\] Fix memcpy from nullptr](https://fuchsia-review.googlesource.com/c/fuchsia/+/461437)
*   [460020: [quickjs] Fix UBSan bugs](https://fuchsia-review.googlesource.com/c/third_party/quickjs/+/460020)
*   [460140: [UBSan][roughtime] Fix unaligned reads, reenable UBSan](https://fuchsia-review.googlesource.com/c/third_party/roughtime/+/460140)

## Sponsors

Reach out for questions or for status updates:

*   <leonardchan@google.com>
*   <phosek@google.com>
*   <mcgrathr@google.com>

[label-ubsan-open]: https://bugs.fuchsia.dev/p/fuchsia/issues/list?q=label%3Aubsan%20is%3Aopen&can=2
[ubsan]: https://clang.llvm.org/docs/UndefinedBehaviorSanitizer.html
