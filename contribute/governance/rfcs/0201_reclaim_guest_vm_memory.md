<!-- Generated with `fx rfc` -->
<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0201" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
{# Fuchsia RFCs use templates to display various fields from _rfcs.yaml. View the #}
{# fully rendered RFCs at https://fuchsia.dev/fuchsia-src/contribute/governance/rfcs #}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Summary

Allow the host to reclaim memory used by the guest.

## Motivation

Running a memory hungry application in the guest and starting a memory hungry
application in the the host may cause OOM even if guest memory is no longer in
use.

The reasons for this are twofold.

1. We currently don't allow the host to automatically take memory from the
   guest.
2. We do not support a way for the guest to proactively tell the host "Here is a
 list of unused memory pages, feel free to use them if you need to".

Example user journey

1. Launch a termina or debian guest
2. Start a memory hungry application in the guest
3. Quit the memory hungry application in the guest
4. Start a memory hungry application in the host
5. Observe the host hitting OOM 💥
    * [Issue 100401: core dump and reboot after
	supertuxkart](https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=100401&cursor=fuchsia%3A98019)
	* [Issue 88066: Memory is not reclaimed when closing
	Quake](https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=88066&cursor=fuchsia%3A98019)

## Background

When OS boot, it asks the hardware (hypervisor in a case of the guest) how much
physical memory there is. If the host told the guest it has 4GiB of RAM, the
guest will know that it can allocate a million or so 4KiB pages.

The way to do it is to introduce a level of mapping between what the guest OS
considers a physical address (it's called a guest physical address) and the
actual physical address (which is called host physical address). As a reminder,
the first level of address translation is mapping between a guest virtual memory
address and the guest physical address. In other words, the guest virtual
address is first translated to the guest physical address by the hardware which
does the translation using page tables managed by the guest OS, and then to the
host physical address - the latter translation is controlled by the page tables
managed by the hypervisor.

The guest handles its own page faults that occur when accessing guest virtual to
guest physical addresses that have not been mapped. The hypervisor handles page
faults for guest-physical to host-physical translations.

We are using paged memory which means when hypervisor gives the guest X GiB of
RAM it does not allocate any memory up-front, but only when those pages are
actually required. Required in this case means the guest attempts to access the
page. From the hypervisor point of view, allocated physical memory pages belong
to the guest and cannot be used for host processes.

As a result, the guest may end up with a lot of allocated physical memory pages
which are not being used while the host can be low on memory to run its own
processes.

Below we'll talk about 2 ways to make the guest memory available to the host:

* Inflate virtio balloon in the guest
* Use of the [free page reporting] in the virtio balloon

### Virtio-balloon

The host can tell the guest virtio-balloon driver to `inflate` the balloon to
certain size by configuring the target balloon size. Inflating the balloon means
the guest will reserve a required amount of memory pages and report
guest-physical addresses of allocated memory pages back to the host. From this
point the host can decommit reported memory pages and use physical memory which
was backing out reported memory.

Guest can start using the memory again at any time if virtio-balloon negotiated
VIRTIO_BALLOON_F_DEFLATE_ON_OOM. See the Virtio Spec 5.5.6.1. Currently we
enable VIRTIO_BALLOON_F_DEFLATE_ON_OOM.

Host can allow guest to re-use pages from the balloon by reducing the target
size of the balloon. The guest may re-use pages previously given to the ballon
if the configured balloon size is less than the actual number of pages in the
balloon. If the guest wants to use memory again it will `deflate` the balloon,
letting the host know that the guest will use a range of physical guest
addresses in the future. After the deflate when the guest access pages that have
been removed from the balloon, it will hit a guest physical to host physical
page fault and hypervisor will allocate a new physical memory page for the guest
to use.

See [Virtio Balloon. slides] and [Virtio Balloon. video] for a detailed
explanation of the virtio-balloon core functionality

### Free page reporting

In 2020 virtio-ballon has received a new feature called free page reporting.

Free page reporting adds a way for the guest to report free memory pages back to
the host. The guest does so by adding 4MiB sized (Linux implementation constant)
free pages to the free page report and sending report to the host. The guest
guarantees not to reuse any free pages until the host acknowledges the report.

When the host receives a free page report, it decommits memory pages making them
available for host applications and acks report back to the guest. At this point
the guest may reuse pages that were previously free'd and acknowledged. If the
guest decides to re-use the page, the host detects a guest physical to host
physical page fauls and allocates a new physical page to fulfill the guest
request.

See [Free page reporting: by Alexander Duyck ( Intel ). Slide 10] for a detailed
explanation.

## Stakeholders

_Facilitator:_

- cpu@google.com

_Reviewers:_

- abdulla@google.com
- dahastin@google.com
- tjdetwiler@google.com

_Consulted:_

- cwd@google.com


_Socialization:_

This RFC went through a review with the Virtualization team. Approach was
discussed with cwd@google.com who is solving a similar albeit larger problem for
ChromeOS.

## Design

### Goals

* Reclaim the memory for the host after running an application inside the guest.
* Minimize the performance impact of memory reclaim on the guest and the host.

### Non-Goals

* Balance heavy memory usage across multiple guests.
* Dynamically prioritize applications when the guest and the host compete for
  memory.
* Support memory reclaim for the Fuchsia guest.

### Success criteria

* Fuchsia is not hitting OOM in the user journey described in the Motivation
  section.
* Host available memory gets back roughly to where it was prior to running a
memory hungry app in the guest.
* Guest can continue running memory hungry applications before and after the
memory reclaim.
* Guest page cache is not affected unless the host is under low memory pressure.
* The number of the host OOMs while at least one guest is running is
  significantly down.

### Approach

* Use the [free page reporting] feature in virtio_balloon to reclaim free memory
for the host usage.
* Inflate ballon on LOW and CRITICAL host memory pressure events to flush out
  the guest page cache and reclaime fragmented memory pages.
* Proof of concept confirmed that free page reporting does reclaim memory as
  expected.

## Implementation

### Use the free page reporting

We'll use [Free page reporting] feature to report and reclaim *all* free memory
  to the host.

*All* being anything of order [PAGE_REPORTING_MIN_ORDER] or higher (defined in
 Linux kernel to 4MiB).

Use of free page reporting will reclaim *most* of the memory over the next 30
seconds. Reported free page size is 2MiB or 4MiB, some amount of memory
fragmentation is expected. On the guest side free page reporting is staggered
over time to minimize the performance impact.

Linux-5.15 will report most free memory over 30 seconds in 2MiB and 4MiB blocks.

Free page reporting won't evict Linux page cache which could be a problem if the
guest is running IO intensive workloads. See [Linux Page Cache Basics] for
information about the page cache in Linux.

This might change in the future when our Linux guest images start using MGLRU
See the "Drawbacks, alternatives and unknown" section on MGLRU.

Not trashing page cache arbitrarily is a good thing, it exists for a reason. The
host should take memory from the guest page cache only when the host actually
needs it. This means we'll need to provide a way to reclaim the guest memory
being used for the page cache when the host is under memory pressure.

### Inflate guest balloon on host memory pressure

Second change is to use the [memorypressure provider] to inflate the balloon on
WARNING and CRITICAL host memory levels.

Inflating the balloon achieves two goals:

* Evict the guest page cache which is important if the guest is doing intensive
  file IO and filling up a page cache.
* Reclaim most of the fragmented memory pages because balloon inflation is done
  in 4KiB granularity while free page reporting is using 4MiB.

Inflation volume will be proportional to the available guest memory. Balloon
will be inflated to 90% of the available guest memory on WARNING and CRITICAL
host memory events. We have to inflate both on WARNING and CRITICAL events in
case free memory sharply goes down from NORMAL to CRITICAL. The balloon will be
deflated to 0% when host memory pressure is back to NORMAL.

We want to avoid constantly inflating and deflating ballon when the host is
under memory pressure. Balloon inflation has a performance cost for both the
guest and the host. On top of it [Constant balloon resizes cause many TLB
shootdowns] according to Intel.

To prevent balloon size bouncing back and forth we'll throttle balloon inflation
operations to 1 inflate per X seconds. X to be configured during teamfood
testing. Initial value will be 1 minute. There is a potential to have more
timeouts, such as a timeout to deflate balloon if host stays in memory pressure
WARNING for too long. Additional timeouts can be added based on the teamfood
testing telemetry.


## Performance

Implementing memory reclaim would improve the the host memory performance when
user is running memory hungry applications in the guest. The host would have
more memory available to work with instead of resorting to memory compression
and other CPU expensive ways to get memory while guest has available memory to
reclaim.

Free page report operation is staggered over 30 seconds in Linux implementation
to reduce the performance impact on the guest. We expect 1%-2% performance
impact in memory intensive guest workloads. See [Free page reporting
benchmarks].

Inflating the ballon when the host is low on memory might add extra load on both
the host and the guest.

We'll need to measure the number of the guest "TLB shootdown" interrupts when
the host is operating under memory pressure with and without memory reclaim
enabled.

Benchmarks:

* [Linux stress tool]
* One of the graphics benchmarks such as [Uningine]

Metrics to capture

* General metrics reported by the benchmark to detect a performance regression
* Available memory in the guest and in the host before and after the benchmark
* TLB shootdown interrupts in the guest

## Security considerations

Reclaimed free pages are zero'ed as part of decommit operation, same as ballon
inflate. This will prevent the guest information leaking to the host and other
guests.

## Testing

The bulk of the work will be done by unit tests and 2 integration tests. One
integration tests will cover the free page reporting memory reclaim and another
will cover the interaction of the guest page cache and the balloon inflate.

The user journey described in the motivation section will be tested manually.
User journey reliance on a guest launch makes it non hermetic. If we had an
automated end-to-end virtualization test it could be extended to cover this
scenario. We don't think it is practical to take a dependency on building an
automated end-to-end virtualization test for this RFC.

## Drawbacks, alternatives, and unknowns

### Multigenerational LRU Framework

Multigenerational LRU Framework aka MGLRU is a memory improvement feature for
the Linux kernel. The current page reclaim in Linux is too expensive in terms of
CPU usage and it often makes poor choices about what to evict. MGLRU aims to
make better choices than the current Linux kernel page reclaim code and to do so
more efficiently. Numbers from Google engineers were cold start times reduced by
up to 16% while enjoying fewer low-memory kills, Chrome OS saw upwards of 59%
fewer out-of-memory kills and 96% fewer low-memory tab discards in its browser,
and server results have been very promising too.

See [PATCH v14 00/14 Multi-Gen LRU Framework] for more details.

Termina 5.15 which we currently use does not have MGLRU patches. MGLRU is not
available in upstream. Termina kernel developers are waiting for MGLRU to get
accepted to the upstream to backport ot Termina 5.15

MGLRU API could be used to drive the free page reporting logic. It's worth
investigating using MGLRU API to improve the free page reporting performance in
Linux kernel. If successful this can be proposed to be merged upstream. This
would be an optimisation of the existing free page reporting solution in Linux
kernel.

There is no dependency on MGLRU in allowing the host to reclaim the guest
memory.

### Constant balloon resizing alternative

Originally suggested way to performance a memory reclaim, hence the name memory
daemon.

This is the approach currently used by the ChromeOS. It has a number of
drawbacks

* Need to choose polling interval.
* Intel has reported there is a perf overhead from even small balloon
  adjustments. See [Constant balloon resizes cause many TLB shootdowns]
* In order for this to work balloon has to be inflated to about 90% of the guest
  memory all the time.
* Guest apps can allocate quickly and get killed by the OOM daemon before the
next polling interval

### ChromeOS approach

ChromeOS is currently working on the next iteration of Responsive
virtio-balloon. The idea in the nutshell is to use a low memory killer in both
the guest and the host to adjust the balloon size instead of killing the
application. There are also plans to use MGLRU to guide the balloon size
adjustment logic.

ChromeOS is solving much a harder and a different problem:

* ChromeOS has multiple guests and the host which all compete for memory
* ChromeOS and all guests have MGLRU and low memory killer daemons to hook up
  to.
* Choosing the right balloon size is a hard problem to solve for all workloads.
    * We'll have to define a set of heuristics to guide the balloon inflate /
    deflate logic
    * It's important to have data from the device fleet to build heuristics and
     analyze their effectiveness.
  * Even more importantly, balloon size has to be adjusted quickly if you are
    using LMKD to trigger the balloon inflate/deflate otherwise OOM killer will
    kill the app while balloon is being inflated/deflated

Fuchsia virtualization doesn't have a large device fleet to collect statistics
from. The problem we are trying to solve is much simpler. Fuchsia doesn't have
 the OOM killer to hook up to.

Fuchsia host does allow running multiple guests (Termina, Debian, Fuchsia)
simultaneously. Currently this is a not a main use case, typically users and
tests run a single guest. This might change once we start using more powerful
hardware. Proposed solution would work for multiple oversubscribed guests if
guests do not use all the available memory. E.g. idle Debian guest and active
 Termina guest.

Problem space gets much bigger if we have to support multiple guests which do
try to use all the available memory. Deciding which guest application or which
guest is more important should be a product policy. We should focus on exposing
the right tools to the product but at a platform level we do not want to be
prescriptive about how low memory is handled.

We will go with the simpler and more predictable solution to solve the problem
at hand while adding data collection to analyze OOMs and see if we need to add
more complex heuristics.

### Sharing the host and the guest page cache with DAX and virtio-fs

[DAX mapping] allows the guest to directly access the file contents from the
hosts caches and thus avoids duplication between the guest and host. Adding
virtio-fs support with DAX, enabling page cache sharing and adding page cache
discard on memory pressure is a alternative solution to the virtio balloon
inflation to clear the guest page cache. Granular page cache control would be
better then the blanket page cache eviction because of the balloon inflation. We
can discard old page cache while keeping the new one to alleviate memory
pressure without affecting host/guest performance too much.

## Prior art and references

* [Free page reporting]
* [Memory Reclaim in the Windows Subsystem for Linux 2]
* [PATCH v14 00/14 Multi-Gen LRU Framework]
* [Virtio mem in Linux]
* [Balloon pressuring page cache]
* [PAGE_REPORTING_MIN_ORDER]
* [Constant balloon resizes cause many TLB shootdowns]
* [Virtio Balloon. slides]
* [Virtio Balloon. video]
* [Linux Page Cache Basics]
* [Virtual memory]
* [Free page reporting benchmarks]
* [DAX mapping]
* [memorypressure provider]

[Free page reporting]: https://patchwork.kernel.org/project/linux-mm/list/?series=181011&state=%2A&archive=both
[Memory Reclaim in the Windows Subsystem for Linux 2]: https://devblogs.microsoft.com/commandline/memory-reclaim-in-the-windows-subsystem-for-linux-2/
[PATCH v14 00/14 Multi-Gen LRU Framework]: https://lore.kernel.org/lkml/20220815071332.627393-1-yuzhao@google.com/T/
[Virtio mem in Linux]: https://lwn.net/Articles/813638/
[Balloon pressuring page cache]: https://lists.linuxfoundation.org/pipermail/virtualization/2020-February/045245.html
[PAGE_REPORTING_MIN_ORDER]:https://lore.kernel.org/all/20210625014710.42954-1-gshan@redhat.com/T/
[Constant balloon resizes cause many TLB shootdowns]: https://buganizer.corp.google.com/issues/202098603#comment26
[Virtio Balloon. slides]:https://static.sched.com/hosted_files/kvmforum2020/8e/KVM%20Forum%202020%20Virtio-%28balloon%20pmem%20mem%29%20Managing%20Guest%20Memory.pdf
[Virtio Balloon. video]: https://youtu.be/Fq47WCCm-HM?t=199
[Free page reporting: by Alexander Duyck ( Intel ). Slide 10]: https://static.sched.com/hosted_files/kvmforum2020/8e/KVM%20Forum%202020%20Virtio-%28balloon%20pmem%20mem%29%20Managing%20Guest%20Memory.pdf)
[Linux Page Cache Basics]: https://www.thomas-krenn.com/en/wiki/Linux_Page_Cache_Basics
[Virtual memory]: https://en.wikipedia.org/wiki/Virtual_memory
[Linux stress tool]: https://linux.die.net/man/1/stress
[Uningine]: https://benchmark.unigine.com/
[Free page reporting benchmarks]: https://lwn.net/Articles/808807/
[DAX mapping]: https://lwn.net/Articles/828371/
[memorypressure provider]: https://fuchsia.dev/reference/fidl/fuchsia.memorypressure
