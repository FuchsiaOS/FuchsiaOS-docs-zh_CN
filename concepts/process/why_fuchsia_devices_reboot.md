# Why Fuchsia devices reboot

This document lists why a Fuchsia device may reboot. Some are self-explanatory
while others require some additional context.

Outline:

-   [Terminology](#terminology)
-   [Reboot reasons listed](#reboot-reasons)
-   [Where to find reboot reasons](#where-to-find)

## Terminology {#terminology}

### Ungraceful reboot

An ungraceful reboot is a reboot that is initiated by either the kernel in
response to an error, such as a kernel panic, or performed by the hardware
without software intervention, such as a hardware watchdog timeout.

### Graceful reboot

A graceful reboot is a reboot that is initiated by a userspace process. The
process may initiate the reboot in response to an error, like when a device’s
temperature is too high, but Fuchsia should have the opportunity to undergo an
orderly shutdown.

## Reboot reasons listed {#reboot-reasons}

### Kernel panic

If the kernel is unable to recover from an internal error, that error is
considered fatal and the system will reboot.

### The system runs out of memory

If the kernel detects that the amount of free physical memory falls below a
threshold, the system will reboot. The kernel does not kill processes to try to
reclaim memory before rebooting, meaning a single process could cause a
system-wide shortage of memory and force the device to reboot.

### Cold boot

If a device loses power for long enough between when it is shut down and it
boots back up, the system will determine this to be a cold boot.

### Brownout

A device browns out when its voltage dips below an acceptable threshold. This
should only occur when there is an issue with a device’s power supply or its
power related hardware.

### Hardware watchdog timeout

Zircon sets up a hardware watchdog timer that will reboot the device if it is
not reset within a specified period of time.

### Software watchdog timeout

A software watchdog timer may reboot the device if someone sets one up.

### Brief loss of power

If a device loses power for a short period of time, like when a user unplugs a
device and rapidly plugs it back in, it may be unable to determine that the
reboot was cold and will consider the reboot a result of a brief power loss. It
is important to note that there is *not* a quantitative measure of what brief is
and is hardware dependent.

### User request

A user or a component acting on behalf of a user, such as SL4F or RCS,
determines a reboot is necessary.

### System update

A component responsible for system updates must update a package, or multiple
packages, that cannot be updated ephemerally. These packages are canonically
know as base packages.

### Retry system update

A component responsible for system updates fails to apply an update, so the
device reboots to try again (or possibly revert the update).

### ZBI swap

If the Zircon boot image is swapped, the device reboots to apply the change.

### High temperature

A component responsible for power management detects that a device's temperature
is too high and the system cannot adequately reduce the device's temperature by
throttling the CPU or reducing the audio volume.

### Session failure

If the session manager is unable to restart a crashed session or a session
determines it has failed in an unrecoverable manner, the device reboots.

### Sysmgr failure

If the system manager for legacy components (sysmgr) crashes, the device
reboots.

### Critical component failure

If a component marked `reboot_on_terminate` crashed, the device reboots.

### Factory data reset

Following a data reset to the factory defaults, the device reboots.

### Root job termination

If the userspace root job is terminated, e.g., because one of its critical
processes crashes, the device reboots.

### Generic graceful

The platform can know whether the reboot was graceful, but cannot distinguish
between a software update, a user request or some higher-level component
detecting the device as overheating. All the platform knows is that the reboot
was graceful.

### Unknown

There are some scenarios in which the platform cannot determine the specific
reboot reason nor can it determine if the reboot was graceful or ungraceful.

## Where to find reboot reasons {#where-to-find}

Fuchsia exposes the reason a device last (re)booted through
[FIDL](/sdk/fidl/fuchsia.feedback/last_reboot_info.fidl) and tracks it on Cobalt
and the crash server.

#### Culprits

Reboots that at are the result of an error in a specific component have crash
signatures that attribute that component as the cause of the reboot. They follow
a general pattern of combining the reboot reason and the component deemed
responsible for the reboot, a.k.a the culprit.

Reboot reason                | **FIDL**                     | **Cobalt event**           | **Crash signature**
:--------------------------- | :--------------------------- | :------------------------- | :------------------
Kernel panic                 | `KERNEL_PANIC`               | `KernelPanic`              | Function responsible for the crash, exactly like a userspace crash report
System running out of memory | `SYSTEM_OUT_OF_MEMORY`       | `SystemOutOfMemory`        | `fuchsia-oom` or `fuchsia-oom-$CULPRIT`
Cold boot                    | `COLD`                       | `Cold`                     | N/A\*
Brownout                     | `BROWNOUT`                   | `Brownout`                 | `fuchsia-brownout`
Hardware watchdog timeout    | `HARDWARE_WATCHDOG_TIMEOUT`  | `HardwareWatchdogTimeout`  | `fuchsia-hw-watchdog-timeout`
Software watchdog timeout    | `SOFTWARE_WATCHDOG_TIMEOUT`  | `SoftwareWatchdogTimeout`  | `fuchsia-sw-watchdog-timeout`
Brief power loss             | `BRIEF POWER LOSS`           | `BriefPowerLoss`           | `fuchsia-brief-power-loss`
User request                 | `USER_REQUEST`               | `UserRequest`              | N/A\*
System update                | `SYSTEM_UPDATE`              | `SystemUpdate`             | N/A\*
Retry system update          | `RETRY_SYSTEM_UPDATE`        | `RetrySystemUpdate`        | `fuchsia-retry-system-update`
ZBI swap                     | `ZBI_SWAP`                   | `ZbiSwap`                  | N/A\*
High temperature             | `HIGH_TEMPERATURE`           | `HighTemperature`          | `fuchsia-reboot-high-temperature`
Session failure              | `SESSION_FAILURE`            | `SessionFailure`           | `fuchsia-session-failure`
Sysmgr failure               | `SYSMGR_FAILURE`             | `SysmgrFailure`            | `fuchsia-sysmgr-failure`
Critical component failure   | `CRITICAL_COMPONENT_FAILURE` | `CriticalComponentFailure` | `fuchsia-critical-component-failure` or `fuchsia-reboot-$CULPRIT-terminated`
Factory data reset           | `FACTORY_DATA_RESET`         | `FactoryDataReset`         | N/A\*
Root job termination         | `ROOT_JOB_TERMINATION        | `RootJobTermination`       | `fuchsia-root-job-termination` or `fuchsia-reboot-$CULPRIT-terminated`
Generic graceful             | *graceful* field set to true | `GenericGraceful`          | `fuchsia-undetermined-userspace-reboot`
Unknown                      | *graceful* field not set     | `Unknown`                  | `fuchsia-reboot-log-not-parseable`

\* Not a crash. \
