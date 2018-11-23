# Development

This document is a top-level entry point to all of Fuchsia documentation related
to **developing** Fuchsia and software running on Fuchsia.

## Developer workflow

This sections describes the workflows and tools for building, running, testing
and debugging Fuchsia and programs running on Fuchsia.

 - [Getting started](../getting_started.md) - **start here**. This document
   covers getting the source, building and running Fuchsia.
 - [Source code](source_code/README.md)
 - [Multiple device setup](workflows/multi_device.md)
 - [Pushing a package](workflows/package_update.md)
 - [Changes that span layers](workflows/multilayer_changes.md)
 - [Debugging](workflows/debugging.md)
 - [Tracing][tracing]
 - [Trace-based Benchmarking][trace_based_benchmarking]
 - [LibFuzzer-based fuzzing](workflows/libfuzzer.md)
 - [Build system](build/README.md)
 - [Workflow FAQ](workflows/workflow_faq.md)
 - [Testing FAQ](workflows/testing_faq.md)

## Languages

 - [C/C++](languages/c-cpp/README.md)
 - [Dart](languages/dart/README.md)
 - [FIDL](languages/fidl/README.md)
 - [Go](languages/go/README.md)
 - [Rust](languages/rust/README.md)
 - [Flutter modules](languages/dart/mods.md) - how to write a graphical module
   using Flutter

## API

 - [System](api/system.md) - Rubric for designing the Zircon System Interface
 - [FIDL](api/fidl.md) - Rubric for designing FIDL protocols
 - [C](api/c.md) - Rubric for designing C library interfaces

## SDK

 - [SDK](sdk/README.md) - information about developing the Fuchsia SDK

## Hardware

This section covers Fuchsia development hardware targets.

 - [Acer Switch Alpha 12][acer_12]
 - [Intel NUC][intel_nuc] (also [this](hardware/developing_on_nuc.md))
 - [Pixelbook](hardware/pixelbook.md)

## Conventions

This section covers Fuchsia-wide conventions and best practices.

 - [Layers](source_code/layers.md) - the Fuchsia layer cake, ie. how Fuchsia
   subsystems are split into a stack of layers
 - [Repository structure](source_code/layer_repository_structure.md) - standard way
   of organizing code within a Fuchsia layer repository
 - [Documentation standards](/best-practices/documentation_standards.md)

## Miscellaneous

 - [CTU analysis in Zircon](workflows/ctu_analysis.md)
 - [Persistent disks in QEMU](workflows/qemu_persistent_disk.md)


[acer_12]: https://fuchsia.googlesource.com/zircon/+/master/docs/targets/acer12.md "Acer 12"
[intel_nuc]: https://fuchsia.googlesource.com/zircon/+/master/docs/targets/nuc.md "Intel NUC"
[pixelbook]: hardware/pixelbook.md "Pixelbook"
[tracing]: https://fuchsia.googlesource.com/garnet/+/master/docs/tracing_usage_guide.md
[trace_based_benchmarking]: benchmarking/trace_based_benchmarking.md
