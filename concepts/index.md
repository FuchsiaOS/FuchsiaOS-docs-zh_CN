# About Fuchsia

You can learn more about Fuchsia by diving deeper into the concepts driving its
design. Fuchsia  is still evolving rapidly, but the underlying principles and
values of the system have remained relatively constant throughout the project.

## Principles

The core principles guiding Fuchsia’s design and development are simple, secure,
updatable, and performant.

### [Simple][simple]

Fuchsia makes it easy to create, maintain, and integrate software and hardware
across a wide range of devices.

* Fuchsia prioritizes clarity and minimalism in its architecture and design.
* Fuchsia strives to be easy to understand and easy to maintain.
* Fuchsia provides the core foundations of an operating system—like hardware access, resource
management, and software abstraction—in a coherent way, creating a robust, stable platform for
products.


### [Secure][secure]

Fuchsia has a kernel and software model designed for modern computing.

* A capability-based system fully isolates processes by default, and limits program access to only
the capabilities and resources that have been explicitly granted.
* Software components are distributed as hermetic packages that provide security boundaries and
guarantees.
* Software isolation, or “sandboxing,” is enforced by the operating system, which lowers the
resource costs for security.


### [Updatable][updatable]

As a modular operating system, Fuchsia allows the kernel, drivers, and software
components to be independently updatable.

* Stable ABIs make it possible for the operating system, device drivers, and product software to
remain compatible over longer time horizons. This reduces the maintenance burden on product
developers, and extends the lifespan of devices for consumers.
* Software is delivered in packages that can be updated independently and even delivered on demand,
like the web.
* Google is committed to updating and maintaining Fuchsia over time.


### [Performant][performant]

Fuchsia is designed for real world product requirements and optimized for
performance.

* Fuchsia is a general purpose operating system that enables high performance across a variety of
platforms, architectures, and devices.
* Fuchsia efficiently manages system resources—processors, memory, storage, networking, and power—
to optimize performance.
* Fuchsia meets performance goals in commercial devices currently in production.


[performant]: /concepts/principles/performant.md
[simple]: /concepts/principles/simple.md
[secure]: /concepts/principles/secure.md
[updatable]: /concepts/principles/updatable.md