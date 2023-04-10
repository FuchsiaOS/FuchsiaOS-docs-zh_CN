# Clock Transformations

## Overview

[Clock objects](/docs/reference/kernel_objects/clock.md) represent functions which map all of the
points on a reference clock timeline to all of the points on the clock object's
timeline. Over all time, this function is represented as a
[piecewise linear function](https://en.wikipedia.org/wiki/Piecewise_linear_function).
Each segment of this function is a one dimensional
[affine transformation](https://en.wikipedia.org/wiki/Affine_transformation)
which relates the reference timeline to the clock's timeline.

Clock objects store only the most recent segment of the transformation at any
given time, not the entire history.

## Definition of the Affine Transformation

A segment of the piecewise linear function is stored using four numbers.

 + The offset on the reference timeline _R<sub>off_</sub> (64 bits)
 + The offset on the clock timeline _C<sub>off_</sub> (64 bits)
 + The ratio of the reference to clock rate (_R<sub>rate_</sub>/_C<sub>rate_</sub>) (32/32 bits)

Given a reference time _r_, the function to apply the most recent segment of the
transformation, _C(r)_ is given as

_C(r) = (((r - R<sub>off</sub>) * C<sub>rate</sub>) / R<sub>rate</sub>) + C<sub>off</sub>_

Given a clock time _c_, the inverse of the _C_ may be used to compute the
corresponding time on the reference timeline _r_.

_C<sup>-1</sup>(c) = r = (((c - C<sub>off</sub>) * R<sub>rate</sub>) / C<sub>rate</sub>) + R<sub>off</sub>_

Care should be taken to avoid overflow when scaling the offset values. It is
recommended to store the intermediate result of multiplication in 96 bits before
dividing back down to something which will fit in 64 bits.
