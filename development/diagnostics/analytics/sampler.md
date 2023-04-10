# Sampler - Diagnostics Data Forwarder

Sampler forwards [Inspect]
diagnostics data to the [Cobalt] telemetry system.

If your component is instrumented with Inspect, you can use Cobalt telemetry
(fleet monitoring) with just configuration file changes! If you want a
low-barrier entry into fleet monitoring (no new FIDL integrations, no new data
models, no flow control complexity in your component), Sampler is for you!

[Inspect]: /docs/development/diagnostics/inspect/quickstart.md
[Cobalt]: https://fuchsia.dev/reference/fidl/fuchsia.metrics

## What do I get? {#why-sampler}

Sampler offers a reduced complexity approach to getting fleetwide monitoring
into the debugging toolkit of every component.

### Centralized data model {#centralized-data-model}

Many developers find it unwieldy to have to maintain two separate data models,
one for things like on-device or crash report debugging, and one for things
like fleet monitoring.

With Sampler, a single diagnostics data model (Inspect) can serve as the source
for all branches of diagnostics data.

### Avoid complexity of direct integration {#no-direct-integration}

For some components (like drivers), their workload is so system-critical that
finding the "right time" to perform non-essential work like diagnostics
reporting can be a complex undertaking. By centralizing sampling to Sampler we
are able to offload work to a program that can be run during
non-peak-user load. Even for components not providing
system-critical services, integrating with Sampler makes Fuchsia healthier by
allowing for better informed sampling scheduling.

Direct integration with Cobalt requires component authors to consider their flow
control as a client sending requests to the Cobalt on-device forwarder. Flow
control is implicit in Sampler, so you can focus more on uploading common
metric types without adding new service dependencies.

## Getting Started {#getting-started}

Note: This "Getting Started" assumes that you already have Inspect
instrumentation in your component. Please see the
[Inspect Codelab] for more information on Inspect instrumentation.

1.  If you are a V2 component, make sure that you expose your `diagnostics`
    directory to the [Diagnostics platform].
1.  Register the [Cobalt metric]
    you want to transform your Inspect metric into.
1.  Create a Sampler configuration defining the transformation from Inspect to
    Cobalt. [See existing config files].

[Inspect Codelab]: /docs/development/diagnostics/inspect/codelab/README.md
[Cobalt metric]: https://fuchsia.dev/reference/fidl/fuchsia.metrics
[Diagnostics platform]: /docs/reference/diagnostics/inspect/tree.md#archivist
[See existing config files]: https://fuchsia.googlesource.com/fuchsia/+/refs/heads/main/src/diagnostics/config/sampler/

## Sampler Configuration {#configuration}

We will be using the FVM Integration as an example configuration.

```json
{
  "project_id": 3676913920,
  "poll_rate_sec": 3600,
  "metrics": [
    {
      "selector": "bootstrap/driver_manager:root/fvm/partitions/blobfs:total_slices_reserved",
      "metric_id": 3000,
      "metric_type": "Integer",
      "event_codes": [0]
    },
    {
      "selector": "bootstrap/driver_manager:root/fvm/partitions/minfs:total_slices_reserved",
      "metric_id": 3000,
      "metric_type": "Integer",
      "event_codes": [1]
    }
  ]
}
```

The Sampler configuration specifies a top-level project id, which specifies
the Cobalt client that Sampler is sampling and forwarding on the behalf of.
It also specifies a top-level poll_rate_seconds, although in Sampler v1.1 this
poll rate will be migrated on client's behalfs to a per-metric configuration.

Next, the configuration requires a list of metric transformations. In these
configurations, the client provides the selector identifying the relevant
inspect metric, along with the Cobalt metadata needed to forward the inspect
on the client's behalf.

1.  `metric_type` is the type of metric transformation requested.
    *   If your Cobalt metric is of type OCCURRENCE being used to track
        numerical aggregations, use the metric_type [Occurrence].
    *   If your Cobalt metric is of type INTEGER being used to track raw
        integers (e.g. cpu_load, thermal_temp), use the metric_type [Integer].
    *   If your Cobalt metric is of type INTEGER_HISTOGRAM, use the metric_type
        [IntHistogram].
    *   If your Cobalt metric is of type STRING, use the metric_type
        [String].
1.  `metric_id` is the same id used when you register your metric in the Cobalt
    yaml files.
1.  `event_codes` is the list of dimension values specified in your
    registration of the Cobalt metric. In FVM's case, each metric has a single
    dimension value corresponding to its [partition type].

NOTE: The order of the dimension values in the Sampler configuration must
align with the order in which the dimensions were declared in the yaml
definition.

[Occurrence]: https://fuchsia.googlesource.com/fuchsia/+/refs/heads/main/src/diagnostics/lib/sampler-config/src/lib.rs#139
[Integer]: https://fuchsia.googlesource.com/fuchsia/+/refs/heads/main/src/diagnostics/lib/sampler-config/src/lib.rs#141
[IntHistogram]: https://fuchsia.googlesource.com/fuchsia/+/refs/heads/main/src/diagnostics/lib/sampler-config/src/lib.rs#143
[String]: https://fuchsia.googlesource.com/fuchsia/+/refs/heads/main/src/diagnostics/lib/sampler-config/src/lib.rs#145
[partition type]: https://fuchsia-review.googlesource.com/c/cobalt-registry/+/462754/4/fuchsia/local_storage/metrics.yaml
