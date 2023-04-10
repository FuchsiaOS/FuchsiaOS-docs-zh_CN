# Hit Testing

## Overview

A hit is the point of interaction between a pointer event and a graphical view.

Hit testing is the process where Scenic determines which hits should be routed
to which clients.

### Hit regions

Client-submitted graphical content is not in and of itself hittable. Rather, it
is the rectangular hit regions attached to Flatland transforms that enable
content views to receive input events.

To ease the use of the Flatland API for developers, each view has a default hit
region. These hit regions act as normal hit regions but are added implicitly to
the root transform in the SetRootView() call. See the
[default hit regions](#default-hit-regions) section for more information.

### Semantic visibility

There are two types of hit regions Flatland clients may attach to transforms:
semantically visible and semantically invisible hit regions. The property of
semantic visibility can be ignored by most clients, as its only used in rare
accessibility-specific use cases. See the
[semantic visibility](/docs/concepts/ui/input/semantic_visibility.md#semantic-visibility)
page for more information.

## High level overview of algorithm

Given a hit at point (x,y), the View containing the geographically top-most
(topologically *last*) semantically-compatible hit region which intersects (x,y)
receives the hit. Let’s break down the requirements into its component parts.

#### Top most

Given a parent and child transform with content, the child transform is drawn on
top of the parent transform, and is also topologically after the parent.

Given two sibling transforms with the same parent, the child added *last* via
AddChild() is topologically after its sibling.

#### Semantic compatibility

Semantic hit tests do not interact with semantically invisible hit regions.

See the [semantic visibility](/docs/concepts/ui/input/semantic_visibility.md)
page for more information.

#### Overlap

A hit that is on the border of a hit region is considered to be included in that
hit region.

If two or more transforms share a border, the tie-breaker goes to the
topologically last transform.

## Default hit regions {#default-hit-regions}

While a View itself is not by default hittable, API clients tend to expect
content to be interactable when created, so Flatland creates a full screen
default hit region (DHR) for Views when they install their root transform via a
SetRootTransform() call, unless the client manually specifies their own hit
region.

DHRs are maximally sized, so resizing the View to be larger should not lead to a
part of the screen becoming unhittable.

The semantics of default hit regions can be summarized in the following rules:

1.  A DHR is added to the root transform, T, during a SetRootTransform(T) call
    if and only if no SetHitRegions(T) calls were made *prior*.
2.  Calling SetHitRegions(T, hit\_region\_vector) *after* a SetRootTransform(T)
    call overrides the DHR, even if |hit\_region\_vector| is empty.
3.  A DHR is only installed for a given root transform so long as it is the root
    View.
4.  A DHR for a given transform T is unaffected by any hit region operation on
    any other transform, U.

To help illustrate the semantics, let’s consider some examples.

### Scenario 1

```
SetRootTransform(T)
```

End state: T has a DHR.

### Scenario 2

```
SetRootTransform(T)

SetRootTransform(U)
```

End state: T has no hit regions, U has a DHR.

### Scenario 3

#### Scenario 3A

```
SetRootTransform(T)

SetHitRegions(T, {HR1})
```

#### Scenario 3B

```
SetHitRegions(T, {HR1})

SetRootTransform(T)
```

End state for 3A and 3B: T has one hit region, HR1. There is no DHR.

### Scenario 4

```
SetRootTransform(T)

SetHitRegions(T, { {} })
```

End state: T has no hit regions

### Scenario 5

```
SetRootTransform(T)

SetRootTransform(U)

SetHitRegions(T, {HR1, HR2})
```

End state: T has two hit regions, HR1 and HR2. U has a DHR.
