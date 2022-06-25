# Contribute to the open source platform

This section is for contributing to the Fuchsia open source platform and
operating system.

## Basic workflows

 - [Get started](get-started/README.md) - If you're new to the Fuchsia
   platform, complete this guide first.
 - [Source code](get-started/get_fuchsia_source.md)
 - [fx workflows](build/fx.md)
 - [Pushing a package](concepts/packages/package_update.md)
 - [Working across different petals](source_code/working_across_petals.md)
 - [Build system](development/build/build_system/index.md)
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
 - [Flutter modules](languages/dart/mods.md) - How to write a graphical module
   using Flutter
 - [New language](languages/new/README.md) - How to bring a new language to Fuchsia

## API

 - [README](development/api/README.md) - Developing APIs for Fuchsia
 - [API Council](contribute/governance/api_council.md) - Definition of the API council
 - [System](development/api/system.md) - Rubric for designing the Zircon System Interface
 - [FIDL API][fidl-api] - Rubric for designing FIDL protocols
 - [FIDL style][fidl-style] - FIDL style rubric
 - [C](development/api/c.md) - Rubric for designing C library interfaces
 - [Tools](development/api/tools.md) - Rubrics for designing developer tools
 - [Devices](development/api/device_interfaces.md) - Rubric for designing device interfaces

## Testing

 - [Debugging workflow](development/debugging/debugging.md)
 - [Fuzz testing with LibFuzzer](development/testing/fuzzing/overview.md)
 - [Test components](concepts/testing/v1_test_component.md)
 - [Test environments](contribute/testing/environments.md)
 - [Testability rubrics](development/testing/testability_rubric.md)
 - [Test flake policy](development/testing/test_flake_policy.md)
 - [Host-target interaction tests](development/testing/host_target_interaction_tests.md)
 - [Testing for Flakiness in CQ](development/testing/testing_for_flakiness_in_cq.md)
 - [Testing a USB Device](development/testing/testing_usb_device.md)

## Tracing

 - [Fuchsia tracing system](concepts/kernel/tracing-system.md)
 - [Tracing guides](development/tracing/README.md)

## Internationalization

 - [Internationalization, localization and input methods](internationalization/README.md)

[fidl-style]: development/languages/fidl/guides/style.md
[fidl-api]: development/api/fidl.md
