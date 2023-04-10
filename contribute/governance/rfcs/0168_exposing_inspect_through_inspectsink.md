<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0168" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
{# Fuchsia RFCs use templates to display various fields from _rfcs.yaml. View the #}
{# fully rendered RFCs at https://fuchsia.dev/fuchsia-src/contribute/governance/rfcs #}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Summary

This RFC introduces the `fuchsia.diagnostics.InspectSink` protocol that allows components
to expose Inspect data. As a result, the current mechanism (the `DirectoryReady` event) can now
be removed.

## Motivation

Inspect today is exposed through a mechanism that is not aligned with regular component framework
capability routing. Today a component exposes a `diagnostics` directory in its outgoing namespace
to framework.

The `DirectoryReady` event was marked deprecated in RFC-121 as part of the goal of publishing event
capabilities in the SDK. However, we don’t yet have a good solution to how components will be able
to expose Inspect data if this directory doesn’t exist. Additionally, Flutter on Fuchsia
wants to remove our reliance on these file system abstractions and simplify their implementation.
Rethinking how components expose Inspect will allow them to remove significant
runtime-complexity and provide advantages to developer ergonomics.

## Stakeholders

_Facilitator:_ leannogasawara@google.com

_Reviewers:_

- crjohns@google.com
- geb@google.com
- shayba@google.com


_Consulted:_

- dworsham@google.com
- surajmalhotra@google.com
- zarvox@google.com


_Socialization:_ this RFC was previously socialized in the form of a Google Docs document among
Diagnostics, Component Framework, Flutter and others.

## Design

### Background

Time ago, the Archivist used to ingest Inspect data through the `/hub`. Its codebase was structured
as directory watchers on the hub that would begin tracking Inspect data of a component the moment
the `out/diagnostics` directory appeared (it used to be named different at the time). This
was eventually removed and done through events (in both appmgr and component manager).
Also, at the time, tests that read Inspect data, did so by reading directly from the `/hub`.

The Archivist currently relies on the `DirectoryReady` event for two things:

- Ingesting Inspect data from components (components `expose /diagnostics to framework`).
- Attributing Inspect data to the source component.

The `DirectoryReady` event tackles both of these points. However, we already have
`CapabilityRequested` for (2) and the author believes we could tackle (1) with a regular
protocol (similar to `fuchsia.logger.LogSink`) bringing additional advantages.

Solving attribution is a non-goal of this RFC. That’s left as future work for removing the
`CapabilityRequested` event which is currently used for `LogSink` and `DebugData`.


### InspectSink

This RFC introduces the `InspectSink` protocol that allows the Archivist to ingest Inspect data
from a component. This protocol is defined as follows:

```
library fuchsia.diagnostics;

using zx;

@discoverable
protocol InspectSink {
    /// Publishes a handle to the `fuchsia.inspect.Tree` protocol that the server can use to read
    /// Inspect data, including lazy nodes.
    Publish(struct {
        tree fuchsia.inspect.Tree;
        root zx.handle:<VMO, zx.rights.BASIC | zx.rights.READ | zx.rights.MAP, optional>;
    });

    /// Publishes a read handle to the inspect VMO  that the component asynchronously updates.
    /// The server can read Inspect using the Inspect reader algorithm [1]. A component using this
    /// method to publish Inspect won't be able to expose lazy nodes.
    ///
    /// [1]: /docs/reference/platform-spec/diagnostics/inspect-vmo-format.md#reader_algorithm
    PublishVmo(struct {
        name string;
        root zx.handle:<VMO, zx.rights.BASIC | zx.rights.READ | zx.rights.MAP>;
    });
}
```

The main method is `Publish`, most components will be using it as `fuchsia.inspect.Tree` is the
standard way of exposing Inspect. However, we provide a mechanism that allows components to publish
only an Inspect VMO. This is needed for a few reasons:

- A component doesn't need to be a server (no reason to run an async loop, just consuming
  capabilities, not serving anything) if it doesn't need to be one, but should still be able to
  expose Inspect.
- Until drivers are migrated to use `fuchsia.inspect.Tree` ([issue][tree-llcpp-bug]), they will
  continue exposing VMOs.
- Until the Inspect Dart library supports `fuchsia.inspect.Tree`, it will need to continue exposing
  VMOs.

Just like `fuchsia.logger.LogSink`, this protocol will be served by the Archivist and routed
to components.

This protocol has several advantages over serving an `out/diagnostics` directory:

1. Alignment with standard component protocol routing:

   - Components just use the protocol `fuchsia.inspect.InspectSink` instead of doing
   `expose /diagnostics to framework`, similar to how components export logs and traces today.
   - The Archivist can receive connections to it through `CapabilityRequested`, maintaining
   attribution.

1. Inspect data can be made available before starting the component async loop.

   - A directory is backed by the `fuchsia.io.Directory` and most components do not serve their
   `out` directory until starting their async loop. By using this protocol, snapshots can contain
   the Inspect data of components that haven’t started their async loop yet but have already written
   Inspect data.

   - The same applies to the `fuchsia.inspect.Tree` protocol. A component’s Inspect data won’t be
   available in snapshots until a component starts serving this protocol, which won’t happen in
   most cases until a component starts its async loop. By using the protocol proposed above we
   can immediately provide the Archivist with both the root VMO (so at least this one is
   included in snapshots) and a handle to `fuchsia.inspect.Tree` for future requests.

1. No more issues with runners and file system implementations.

   - We currently have code in appmgr and component manager that special cases the scenario of a
   component not serving `out/diagnostics` on time, which is the case for the Flutter runner.
   That runner serves the `out/` directory first and then fills it. Ideally we would just use
   directory watchers, but not all VFS implementations have watchers implemented (notably our
   VFS implementation in the C++ SDK used by Flutter) and we cannot rely on future runners
   using complete `fuchsia.io.Directory` implementations.

This change can be done under the hood by changing the `inspect/client.shard.cml` to use the
new protocol instead of exposing the diagnostics directory to the framework. The file
`inspect/client.shard.cml`  is offered through the SDK, and is used by all components that expose
Inspect anywhere.

This proposal is intended for v2 components. We’ll still be handling `out/diagnostics` for v1
components for compatibility purposes. With the ongoing migration to v2, the author doesn’t
believe it’s worth spending time changing how v1 is working. The goal is to make this change under
the hood, so components being migrated to v2 don't require code changes on how they expose inspect.


## Implementation

The standard LSC process will be followed.

Particular steps are:

1. Implement the new protocol `InspectSink` in the Archivist.
1. Identify components using the `inspect/client.shard.cml` and route `fuchsia.inspect.InspectSink`
   to them from the Archivist. Today this introduces the problem of having to fully enumerate all
   such components in cml definitions, however we have a solution that has already been socialized
   and will be covered in a follow-up RFC.
1. Change `inspect/client.shard.cml` to use this protocol together with exposing the `/diagnostics`
   directory to the framework, thus allowing for a soft transition.
1. Support connecting to `InspectSink` instead of exposing a `diagnostics` directory in the Inspect
   libraries.
1. Remove the implementation of `DirectoryReady` in Component manager and remove it from the
  `fuchsia.sys2.Event` definition once all components have been transitioned, and all prebuilts
  have been refreshed. We'll rely on CTF tests and the support window we have to know when this
  can be done.


## Performance

Performance should have a positive improvement as we no longer have to traverse directories or
create the `DirectoryReady` event, we just rely on a single protocol.

## Security considerations

This change aligns with component framework security properties, in particular the principle
of least privilege and the principle of hierarchical isolation.

## Privacy considerations

No changes regarding privacy. Inspect data continues to go through the regular privacy
pipelines.

## Testing

The implementation of this RFC will be tested in the following ways:

- Unit tested in libraries connecting to InspectSink.
- Unit tested and integration tested in the Archivist, which serves InspectSink.
- [CTF tests][cts] to detect compatibility changes.

## Documentation

[Inspect discovery and hosting][inspect-discovery] will be updated.

The goal is to make this change under the hood, in the libraries that write inspect. However, If we
end up requiring code changes when components migrate to v2, we'll reflect those changes in the
[diagnostics section of the migration guide][migration-diagnostics].

## Drawbacks, alternatives, and unknowns

### A single protocol for both logs and Inspect

We could have a DiagnosticsSink which allows components to connect Inspect and logs in a single
place.

- _Pros_: One single protocol to route instead of 2.
- _Cons_: If for some reason we need to treat logs or Inspect differently in the future, it might be
  easier to do it if we maintain separate protocols than if we have the same protocol for the two
  of them.

We believe the con makes this alternative very unattractive as logs and Inspect are different
things, so it makes sense to route them separately.

## Prior art and references

N/A

[cts]: /docs/development/testing/ctf/overview.md
[inspect-discovery]: /docs/reference/diagnostics/inspect/tree.md
[migration-diagnostics]: /docs/development/components/v2/migration/diagnostics.md
[tree-llcpp-bug]: https://fxbug.dev/95806
