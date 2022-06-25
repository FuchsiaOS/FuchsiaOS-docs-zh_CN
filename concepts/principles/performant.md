# Performant

Fuchsia is a commercial operating system designed for real world product
requirements and optimized for performance. Fuchsia efficiently manages system
resources—processors, memory, storage, networking, and power—to optimize
performance across a variety of platforms, architectures, and devices.

## Flexibility and efficiency

**[Design principles][design-principles] prioritize performance**

Fuchsia enables programs to run as fast as the hardware allows. Whether it’s
choosing a [programming language][programming-language] or deciding between
[structs and tables][structs-and-tables], Fuchsia is designed to give developers
flexibility while maximizing efficiency.

## Performance benchmarks

**Every subsystem on Fuchsia is [benchmarked] to evaluate
performance**

Comparing Fuchsia’s overhead to previous builds and other operating systems
ensures that Fuchsia meets performance expectations. While Fuchsia does not
yet achieve its performance goals in all areas, it is an area under active
development.

## A flexible scheduler optimizes the system

**[Fair scheduling][fair-scheduling] gives the system more flexibility**

Increasing the choices available to the system scheduler gives the scheduler the
flexibility to optimize for power, throughput, or latency, as appropriate for
the situation. At any given time, there are more threads in the system that are
ready to do useful work than there would be if threads commonly blocked one another.

## Asynchronous communication

**APIs use [asynchronous communication][async-com] to reduce latency**

Fuchsia makes heavy use of asynchronous communication, which reduces latency by
letting the sender proceed without waiting for the receiver. This is important
for delivering software that can come and go on a device as needed, to account
for network latency.

[design-principles]: contribute/governance/rfcs/0027_you_only_pay_what_you_use.md
[programming-language]: contribute/governance/rfcs/0082_starnix.md
[structs-and-tables]: contribute/governance/rfcs/0047_tables.md#should_i_use_a_struct_or_a_table
[benchmarked]: /src/tests/benchmarks
[fair-scheduling]: concepts/kernel/fair_scheduler.md
[async-com]: concepts/fidl/overview.md#messaging_models