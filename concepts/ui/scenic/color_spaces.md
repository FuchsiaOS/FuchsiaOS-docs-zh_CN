# Color spaces
The colorspace of a pixel determines how the numerical values of the pixel are interpreted. This is
different than the pixel format which specifies how the different channels and their bits are
arranged in memory.

## Linear vs sRGB color spaces
The two colorspaces we'll focus on are the linear and sRGB ones. Both can have the same pixel
format, say BGRA32, which means that there each blue, green, red, and alpha channel each have 8
bits. The way these bits are interpreted, however, can differ.

In a linear colorspace, a doubling of a value leads to a doubling of the intensity. Adding two
colors together becomes as simple as adding the values of each channel together, which is
straightforward.

sRGB on the other hand is non-linear, meaning that doubling any value will *not* double the
intensity of that value.

### Rationale
Why even have non-linear colorspaces, if the linear one is so intuitive?

The simple answer is that the human eye does not perceive light linearly. It is better able to
discern differences in light in low-light environments than bright environents, for instance.
Therefore non-linear colorspaces such as sRGB have the ability to better express images as we would
see them in real life.

### What this means for you
The Screenshot protocol outputs images in the sRGB colorspace. This means that if you are using it
for non-saturated pixel comparison tests, you should convert the screenshot from sRGB into a linear
colorspace. Non-saturated pixels means one or more channels are non-0 or non-1.

If you are using the image to create GPU resources, however, this conversion is unnecessary and you
can use the output as VK\_FORMAT\_\*\_SRGB.
