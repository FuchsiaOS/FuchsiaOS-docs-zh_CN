# Flatland 2D Composition in Scenic

 * Project lead: emircan@google.com
 * Document authors: emircan@google.com, dworsham@google.com
 * Area(s): View system

## Problem statement

Scenic currently provides graphics clients with a 3D API under the
`fuchsia.ui.gfx` namespace (the "gfx api").  This API provides clients with a
scene model like that in a video game or other 3D graphics program.  Drawing
order is handled by Z depth and opacity is handled via alpha blending based on
depth (features like group opacity are impossible).  Unfortunately the gfx API
is no longer suitable for the demands being placed on Scenic, both from a
product standpoint and a performance standpoint.

From the product standpoint, our current customers are fundamentally 2D
products.  There is no concept of depth and draw order is simply that -- the
order that different batches of draw geometry are submitted in.  Because there
is no depth, transparency effects are also dictated by draw order and effects
like group opacity are common.  The "2D" products (via Flutter, Chromium, and
session framework) have to do extra work to resolve the impedance mismatch
between Scenic's 3D scene representation and the 2D representation experienced
by the user.

From the performance standpoint, modern Video Display Controller (VDC) hardware
provides acceleration features such as multiple display planes and hardware
overlays that Scenic would like to leverage in the future to lower power
consumption and GPU usage.  The hardware operates in a strictly 2D paradigm
however, and only understands rectangular layers that can be positioned in X/Y.
Scenic's current 3D API allows and encourages clients to submit content that
doesn't fit into this paradigm and frustrates optimization attempts.

## Solution statement

The Flatland 2D API is a new client API currently in development by the Scenic
team that seeks to address product and performance goals for graphics on
Fuchsia by exposing display-controller-like functionality to clients.  Clients
may only submit 2D layers that are scaled and offset in X/Y.

This will lead to:

 * Scenic exposing a true 2D API that matches existing client expectations more
   closely.
 * Less GPU usage by delegating work to the VDC where possible.
 * A more lightweight Scenic optimized for 2D rectangular layers only.

## Dependencies

Conversion of all existing Scenic clients, in-tree and out

## Risks and mitigations

Risk: All clients in a product must be migrated atomically, because of
user-input concerns

Mitigation: Config-data flags for enabling Flatland across RootPresenter,
Flutter and Chrome so that enabling Flatland is non-disruptive and atomic

Mitigation: More complex input implementation in Scenic that allows clients to
be migrated non-atomically (mixed gfx and flatland scene-graphs)

Mitigation: Will not implement group opacity functionality at first; save for
later
