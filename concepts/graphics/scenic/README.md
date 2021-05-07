# Fuchsia UI services

The Fuchsia tree does not provide a full-fledged end-user UI.  Instead, it provides services that provide a foundation upon which to build secure, performant, multi-process UIs.

These services include:

## Scenic, the Fuchsia graphics engine

Scenic ([doc](scenic.md)) provides a retained-mode scene graph that allows graphical objects from multiple processes to be composed and rendered within a unified lighting environment.

## Input

The input subsystem ([doc](input.md)) is responsible for discovering the available input devices, and allowing clients to register for events from these devices.
