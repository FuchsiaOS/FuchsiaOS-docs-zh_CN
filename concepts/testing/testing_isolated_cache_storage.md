# Testing Isolated Cache Storage

A component may request that persistent storage be present in its incoming
namespace under `/cache` by using [the `isolated-cache-storage`
feature][cache-feature]. This directory, unlike the storage provided by
`isolated-persistent-storage`, will be cleaned up by the system when disk
pressure is high. This cache cleaning event will walk _every_ component's cache
storage and unlink everything it finds.

Because this cache cleaning event only happens under very specific situations, a
service interface named [`fuchsia.sys.test.CacheControl`][cache-control] is
provided to allow tests to cause cache clearing events. To exercise this
interface tests should cause components under test to populate items in their
`/cache` storage, the test should call the `Clear()` function in
`fuchsia.sys.test.CacheControl`, and then the test should ensure that the
component continues to behave correctly when faced with this unexpected removal
of a file it needs.

Note that components not related to the test will also have their caches cleared
when the `Clear()` function is called. The function clears the cache of every
component on the system.

An example demonstrating test coverage of a user of `isolated-cache-storage` is
available at [`//examples/isolated_cache`][cache-example].

[cache-feature]: /docs/concepts/components/v1/component_manifests.md#sandbox
[cache-example]: /examples/isolated_cache
[cache-control]: /sdk/fidl/fuchsia.sys.test/cache.fidl
