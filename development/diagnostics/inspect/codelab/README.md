# Inspect codelab

Contributors: cphoenix@, crjohns@, miguelfrde@

In this codelab, Rust, Dart, and C++ programmers will learn to use the Inspect
library to publish diagnostic information from their programs, and use Inspect
information to debug their programs.

## What is Inspect?

Inspect allows Fuchsia Components to expose structured, hierarchical
information about their current state.

Learn more about Inspect in the [Fuchsia Component Inspection](/docs/development/diagnostics/inspect/README.md)
documentation.

### What benefits does Inspect provide?

Component Inspection supports many use cases, including:

* Debugging

  View Inspect data from your running component to identify
  problems. For example, you can learn if your component is currently
  connected to a dependency.

* Monitoring system health

  Inspect data provides insight into overall system state. For example,
  you can learn why your system is not connected to the internet.

* Gathering usage or performance statistics

  You can read Inspect data from multiple components at the same time
  to understand system performance. For example, you can see the list
  of incoming connections to your component along with your component's
  memory usage.

### What kind of information can I store in Inspect?

You determine the structure and content of the data you expose in
Inspect. Some examples include:

* The number of open WiFi connections.
* The number of requests that the program has served.
* The number of errors that a parser has encountered.
* The contents of a data structure.


## Codelab

In this codelab, you're going to modify programs to output Inspect data.
You will learn:

* How to include the Inspect libraries.

* How to initialize Inspect in your components.

* How to solve real bugs by writing and reading Inspect data.

* How to read Inspect data to verify that your program is doing what you want.

## What you’ll need

* Basic knowledge of Rust, Dart, or C++.
* Access to a Fuchsia source tree you can execute build commands in.

## Get Started

- [Inspect Codelab](codelab.md)
