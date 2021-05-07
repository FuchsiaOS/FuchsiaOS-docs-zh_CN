# Defining a Stable Driver Runtime

 * Project lead: surajmalhotra@google.com
 * Status: Approved
 * Area(s): Devices

## Problem statement

Banjo is an interface definition language (IDL) used to express interfaces used
between drivers. It is a derivative of FIDL, with a forked syntax from 2018.
While the syntax is similar, unlike FIDL, banjo was designed for synchronous
in-process communication, and the resulting codegen amounts to a very barebones
struct of function pointers, associated with a context pointer.

A non-exhaustive list of problems with banjo include:

 * The generated code for banjo lacks a strategy for interface and type
   evolution. This is a critical requirement for interface stability.
 * Since early 2019, banjo has been largely in maintenance mode, and has fallen
   behind FIDL in terms of ergonomics and features. Understanding how to write
   banjo syntax has become confusing because the Fuchsia project has relied on
   FIDL documentation to address the current gaps in banjo's features and
   ergonomics.
 * Banjo is optimized to be low overhead, placing a great deal of burden on
   driver authors to figure out how to move state onto the heap
   or handle an operation asynchronously. There is a great deal of boilerplate
   involved with manual serialization logic required to do so.
 * There are no strict requirements on how driver authors may invoke banjo
   protocol methods, nor any guarantees on which context their own protocol
   methods may be invoked, leading to unnecessary spawning of threads in order
   to achieve safety (avoiding deadlocks).
 * Banjo types are incompatible with FIDL types often leading to much
   boilerplate when shifting to out of process communication.

## Solution statement

We aim to solve these problems by evolving banjo into something better. The
three key features of the new transport will be:

 1. A forced layer of indirection between drivers, to allow a runtime to
    mediate driver-to-driver communications within the same process
 2. Migration away from C structs towards types built with evolution in mind.
 3. Enforcement of a threading model which is well defined

We are expecting to find a solution with the following characteristics:

 * Shift all communication between drivers to be message oriented, utilizing
   the FIDL wire format between drivers.
 * Allow drivers to make synchronous calls into other drivers.
   - With the caveat it is only allowed on threads owned by the driver.
 * Share threads between drivers
   - With the caveat that all communication on shared threads must be
     asynchronous.
 * Allow drivers to never deal with re-entrance or synchronization if they
   don't opt-in (allowing them to avoid locks altogether).
 * Allow for zero copy and zero serialization / deserialization between
   drivers.

We reserve the right to change our minds depending on the benchmark results of
early prototypes. If we cannot outperform mechanisms provided by the kernel, we
will need to try alternative designs. We also need to prove out our assumptions
that the mechanisms provided by the kernel are insufficient for our needs.

We will try to track progress towards a new banjo with the following
milestones:

 1. Update banjo syntax to match fidl syntax, use fidlc as the frontend, and
    implement a custom backend which generates output equivalent to what banjoc
    generates today.
    1. This allows us to avoid maintenance burden and future syntax drift.
 2. Architect a threading model for drivers that we want to design around.
 3. Decide on metrics/benchmarks to judge any forthcoming designs.
 4. Run experiments to see if we can meet required benchmarks with newer
    transport.
 5. Implement new fidl backend and driver runtime.
    1. We will likely start by creating a variant of LLCPP fidl bindings which
       targets new transport.
 6. Repeat the following steps for each driver stack in a loop:
    1. Migrate drivers which are co-resident in the same driver host over to
       the new threading model, utilizing existing banjo transport.
    2. Migrate drivers which are co-resident in the same driver host over to
       the new in-process FIDL transport.

## Dependencies

We will likely need to work with the FIDL team to allow LLCPP bindings to be
abstracted away from zircon channels and ports to allow us to repurpose the
bindings mostly as-is on a new transport with minimal user visible differences.
We don't anticipate any changes necessary to the frontend IDL, but changes to
FIDL IR may be necessary.

Additionally, migrating 300+ drivers will take a lot of effort and time, and
will require various teams throughout the organization to be involved to ensure
nothing breaks.

## Risks and mitigations

A major change like this has long-term implications on performance
characteristics of our system by inducing additional overhead. Luckily, we have
built in some evolutionary support directly into our framework's architecture
to enable us to move towards another technology if the solution we build is
unable to meet future needs. We can do this by implementing new component
runners and having drivers target the new runner, which may have a different
driver runtime. Switching every driver over to the new driver runner will
likely be impractical, however, so we will end up needing to maintain both in
parallel, which has costs of its own. As such, we really want to get this
approach mostly right to avoid needing to take this course.

Switching drivers to a new threading model also is a large cost to pay, and may
induce new bugs along the way. Many drivers lack tests. Additionally, for
drivers that do have tests, unit tests may also lose their validity after the
switch and may have to be rewritten alongside the transition. We have written a
great deal of our driver tests as integration tests which should continue to be
valid even after migration without any changes. We will continue to try to
invest in more integration tests and e2e tests prior to migration to prevent
introduction of new bugs.

Estimating the migration timeline for the migration is another large risk. It
is hard to accurately estimate the cost here without having built a replacement
and trialed migration on at least one driver. We will need to continually be
cognizant of the cost as we implement our design, and automate as much of the
migration as possible.
