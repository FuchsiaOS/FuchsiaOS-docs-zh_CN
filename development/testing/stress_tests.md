# Stress tests

## The need for stress tests

When writing a stateful system that interacts with other processes/components, it is possible to
introduce bugs that may not manifest until the system is put under load. These bugs arise from
incorrect assumptions, off-by-one errors or subtle timing issues. The errors are hard to detect in
advance because they are the result of complex, stateful interactions between layers of the system.

Stress tests reveal errors in these interactions by generating large workloads for the system
concurrently. If a workload produces an unexpected error, developers can root-cause the issue,
fix it and write a test that covers that specific interaction.

Our infrastructure also has a dedicated builder `core.qemu-x64-stress` for running stress tests.
This builder will run each stress test for a maximum of 22 hours.

Note: Stress tests are currently restricted to the `QEMU` device type, since they run for long
periods of time.

## Client Libraries

### Rust stress test library

We offer a Rust Client Library for writing stress tests. A guide to writing tests using that library
can be found [here](rust_stress_test_library.md).

#### Concepts

Developers can write stress tests that create an environment and produce actors
that indefinitely run randomized workloads on that environment.

##### Actor

An actor's responsibility is to perform operations on an environment. An actor should perform
operations that stress the environment. It can do this in one of two ways:

* Interacting cooperatively with the system-under-test (workload generation).

* Intentionally breaking the system-under-test (failure simulation).

##### Actor Runner

An Actor Runner is a thin wrapper on an Actor and is responsible for running the actor indefinitely.
Runners are created for each actor by the environment and are run on individual threads.
A runner repeatedly instructs its actor to perform a single operation and return the result.

##### Environment

An environment's responsibility is to:

* Store global state needed for the test.

* Provide the exit criteria for the test.

* Provide actors that run for the entire duration of the test.

* Reset global state when requested by an actor.

A stress test creates exactly one environment that lives for the entire duration of the test. A test
writer must define an environment and provide it to the stress test framework.

An environment can store global objects that outlive every instance of the system-under-test.
For example, if a filesystem test operates on a block device backed by a VMO, store that VMO in the
environment, so that even if an actor crashes the block device, the VMO remains intact and can be
used to reset the state.

The environment provides multiple runners to the framework, each containing an actor
that can perform operations.

On reset, an environment must update global state and the connections of each actor.