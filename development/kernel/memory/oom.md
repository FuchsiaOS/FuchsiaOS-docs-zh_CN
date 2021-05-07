# Out-of-memory (OOM) system

This file contains information about the systems that watch for and respond to
out-of-memory (OOM) events.

[TOC]

## Behavior

When the system runs out of memory and the kernel OOM thread is running, you
should see a series of log messages like:

```
OOM: 5915.8M free (+0B) / 8072.4M total
OOM: oom_lowmem(shortfall_bytes=524288) called
OOM: Process mapped committed bytes:
OOM:   proc  1043  397M 'bin/devmgr'
OOM:   proc  2107   88M 'driver_host:pci#1:8086:1916'
OOM:   proc  1297   12M 'virtual-console'
OOM:   proc  3496   17M 'netstack'
OOM:   proc  4157  170M 'flutter:userpicker_device_shell'
OOM:   proc 28708  353M 'flutter:armadillo_user_... (+3)'
OOM:   proc 31584    9M 'dart:weather_agent'
OOM:   proc 32093   14M 'dart:mi_dashboard.dartx'
OOM: Finding a job to kill...
OOM:   (skip) job  57930 'story-8cf82cb9f742d9ecc77f1d449'
OOM:   (skip) job  37434 'story-10293ae401bc0358b3ce52d2a'
OOM:   *KILL* job  29254 'agent'
OOM:        + proc 32093  run 'dart:mi_dashboard.dartx'
OOM:        = 1 running procs (1 total), 0 jobs
OOM:   (next) job  29247 'agent'
OOM:   (next) job  29240 'agent'
OOM:   (next) job  29233 'agent'
```

The first line shows the current state of system memory:

```
OOM: 45.8M free (-12.4M) / 8072.4M total
```

The next section prints a list of processes that are consuming large amounts of
memory, in no particular order:

```
OOM: Process mapped committed bytes:
OOM:   proc  1043  397M 'bin/devmgr'
OOM:   proc  2107   88M 'driver_host:pci#1:8086:1916'
OOM:   proc  1297   12M 'virtual-console'
OOM:   ...
             ^koid  ^mem
```

The next section shows the walk through the ranked job list, printing skipped
jobs (which don't have killable process descendants), the job that will be
killed, and the next jobs on the chopping block:

```
OOM: Finding a job to kill...
OOM:   (skip) job  57930 'story-8cf82cb9f742d9ecc77f1d449'
OOM:   (skip) job  37434 'story-10293ae401bc0358b3ce52d2a'
OOM:   *KILL* job  29254 'agent'
OOM:        + proc 32093  run 'dart:mi_dashboard.dartx'
OOM:        = 1 running procs (1 total), 0 jobs
OOM:   (next) job  29247 'agent'
OOM:   (next) job  29240 'agent'
OOM:   (next) job  29233 'agent'

                   ^koid ^name
```

The `*KILL*` entry will also show all process descendants of the to-be-killed
job.

## Components

### Kernel OOM thread

A kernel thread that periodically checks the amount of free memory in the
system, and kills a job if the free amount is too low (below the "redline").

Use `k oom info` to see the state of the OOM thread (on the kernel console):

```
$ k oom info
OOM info:
  running: true
  printing: false
  simulating lowmem: false
  sleep duration: 1000ms
  redline: 50M (52428800 bytes)
```

The redline, sleep duration, and auto-start values are controlled by
`kernel.oom.*` [kernel commandline flags](/docs/reference/kernel/kernel_cmdline.md).

The thread can be started with `k oom start` and stopped with `k oom stop`.

`k oom print` will toggle a flag that prints the current free and total memory
every time the thread wakes up.

`k oom lowmem` will trigger a false low-memory event the next time the thread
wakes up, potentially killing a job.

### OOM-ranker driver

TODO(dbort/maniscalco): Implement and document.
