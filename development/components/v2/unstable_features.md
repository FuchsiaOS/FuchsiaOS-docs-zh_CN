# Unstable CML features

Oftentimes when a new CML feature is being developed (e.g. a new capability type), the
Component Framework team wants to experiment with the API or implementation before committing to
supporting the new feature.

For this reason, the CML compiler `cmc` will only allow the use of unstable features in your CML
if you've opted-in to them.

## Opting-in

In order to use an unstable feature, you need to add the `unstable_features` property to your
component build rule.

```gn
fuchsia_component("my-component") {
  manifest = "meta/my-component.cml"
  unstable_features = [ "services" ]  # This component opts-in to the unstable "services" feature.
  deps = [ ... ]
}
```

Use of unstable features are restricted to an allowlist. You must add your component to the
allowlist for the feature at `//tools/cmc/build/unstable_features/BUILD.gn`.
