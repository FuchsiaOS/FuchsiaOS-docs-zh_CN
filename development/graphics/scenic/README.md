# Fuchsia UI services

The Fuchsia tree does not provide a full-fledged end-user UI.  Instead, it provides services that provide a foundation upon which to build secure, performant, multi-process UIs.

These services include:

## Scenic, the Fuchsia graphics engine

Scenic ([doc](concepts/ui/scenic/index.md)) provides a retained-mode scene graph that allows graphical objects from multiple processes to be composed and rendered within a unified lighting environment.

