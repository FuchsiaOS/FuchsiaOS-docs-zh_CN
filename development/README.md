# Development guides

This document is a top-level entry point to all of Fuchsia documentation related
to developing Fuchsia and software running on Fuchsia.

## Developer workflow

This sections describes the workflows and tools for building, running, testing
and debugging Fuchsia and programs running on Fuchsia.

 - [Getting started](/docs/get-started/README.md) - This document
   covers getting the source, building and running Fuchsia.
 - [Source code](/docs/get-started/get_fuchsia_source.md)
 - [fx workflows](build/fx.md)
 - [Pushing a package](/docs/concepts/packages/package_update.md)
 - [Working across different petals](source_code/working_across_petals.md)
 - [Build system](/docs/concepts/build_system/index.md)
 - [Workflow tips and FAQ](source_code/workflow_tips_and_faq.md)
 - [Testing FAQ](testing/faq.md)

## Languages

 - [README](languages/README.md) - Language usage in Fuchsia
 - [C/C++](languages/c-cpp/README.md)
 - [Dart](languages/dart/README.md)
 - [FIDL](languages/fidl/README.md)
 - [Go](languages/go/README.md)
 - [Rust](languages/rust/README.md)
 - [Python](languages/python/README.md)
 - [Flutter modules](languages/dart/mods.md) - how to write a graphical module
   using Flutter
 - [New language](languages/new/README.md) - how to bring a new language to Fuchsia

## API

 - [README](/docs/concepts/api/README.md) - Developing APIs for Fuchsia
 - [API Council](/docs/contribute/governance/api_council.md) - Definition of the API council
 - [System](/docs/concepts/api/system.md) - Rubric for designing the Zircon System Interface
 - [FIDL API][fidl-api] - Rubric for designing FIDL protocols
 - [FIDL style][fidl-style] - FIDL style rubric
 - [C](/docs/concepts/api/c.md) - Rubric for designing C library interfaces
 - [Tools](/docs/concepts/api/tools.md) - Rubrics for designing developer tools
 - [Devices](/docs/concepts/api/device_interfaces.md) - Rubric for designing device interfaces

## ABI

 - [System](/docs/concepts/system/abi/system.md) - Describes scope of the binary-stable Fuchsia System Interface

## SDK

 - [SDK](idk/README.md) - information about developing the Fuchsia SDK

## Hardware

This section covers Fuchsia development hardware targets.

 - [Acer Switch Alpha 12][acer_12]
 - [Intel NUC][intel-nuc]
 - [Pixelbook][pixelbook]
 - [Toulouse][toulouse]
 - [Khadas VIM2][khadas-vim]
 - [iMX8M EVK][imx8mevk]

## Drivers

This section covers developing drivers on Fuchsia.

 - [Getting started][drivers-start]

## Testing

 - [Debugging workflow](/docs/development/debugging/debugging.md)
 - [Fuzz testing with LibFuzzer](/docs/development/testing/fuzzing/overview.md)
 - [Test components](/docs/concepts/testing/v1_test_component.md)
 - [Test environments](/docs/concepts/testing/environments.md)
 - [Testability rubrics](/docs/concepts/testing/testability_rubric.md)
 - [Test flake policy](/docs/concepts/testing/test_flake_policy.md)
 - [Testing Isolated Cache Storage](/docs/concepts/testing/testing_isolated_cache_storage.md)
 - [Host-target interaction tests](/docs/development/testing/host_target_interaction_tests.md)
 - [Testing for Flakiness in CQ](/docs/development/testing/testing_for_flakiness_in_cq.md)


## Conventions

This section covers Fuchsia-wide conventions and best practices.

 - [Documentation standards](/docs/contribute/docs/documentation-standards.md)
 - [Endian policy](/docs/development/languages/endian.md)

## Tracing

 - [Fuchsia tracing system](/docs/concepts/tracing/README.md)
 - [Tracing guides](/docs/development/tracing/README.md)

## Internationalization

 - [Internationalization, localization and input methods](internationalization/README.md)

## Miscellaneous

 - [CTU analysis in Zircon](kernel/ctu_analysis.md)
 - [Packet capture](debugging/packet_capture.md)
 - [Editor configurations](/docs/development/editors/README.md)
 - [Using the Fuchsia Installer](/docs/development/hardware/installer.md)
 - [Enable verbose logging for input events](/docs/development/components/v1/verbose_logging.md)

[acer_12]: /docs/development/hardware/acer12.md "Acer 12"
[pixelbook]: /docs/development/hardware/pixelbook.md "Pixelbook"
[toulouse]: /docs/development/hardware/toulouse.md "Toulouse"
[khadas-vim]: /docs/development/hardware/khadas-vim.md "Khadas VIM2"
[imx8mevk]: /docs/development/hardware/imx8mevk.md "iMX8M EVK"
[intel-nuc]: /docs/development/hardware/intel_nuc.md "Intel NUC"
[fidl-style]: /docs/development/languages/fidl/guides/style.md
[fidl-api]: /docs/concepts/api/fidl.md
[drivers-start]: /docs/development/drivers/developer_guide/driver-development.md
