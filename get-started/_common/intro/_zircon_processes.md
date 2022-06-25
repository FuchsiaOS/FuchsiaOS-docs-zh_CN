## Jobs, processes and threads

Zircon exposes three main kernel objects for running code:

* [Thread](/docs/reference/kernel_objects/thread.md):
  Thread of execution within a given address space.
* [Process](/docs/reference/kernel_objects/process.md):
  Set of executable instructions run in a private, isolated address space.
* [Job](/docs/reference/kernel_objects/job.md):
  Group of related processes and jobs. All jobs form a single rooted tree.

![Tree diagram illustrating Fuchsia's process hierarchy. Processes are
grouped into jobs, which are ultimately owned by the Root Job.]
(/docs/get-started/images/intro/processes-jobs.png){: width="549"}

Processes form the basis for system capabilities. Each process is granted a set
of capabilities through the various handles it holds.

Fuchsia software may or may not run within the confines of a single process.
Jobs allow "applications" that are composed of more than one process to be
controlled as a single entity.
