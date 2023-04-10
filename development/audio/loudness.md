# Loudness

The loudness of a given audio stream on Fuchsia is a product of five factors:

* [Usage of the stream](#usage-of-stream)
* [System-global volume setting for that Usage](#usage-volume)
* [System-global gain adjustment for that Usage](#usage-gain-adjustment)
* [Stream-local gain adjustment for the stream](#stream-gain-adjustment)
* [Mute state](#mute-state)

## Usages {#usage-of-stream}

[Usages](https://fuchsia.dev/reference/fidl/fuchsia.media#Usage) are a hint to the system about
the meaning of the audio to a user. Two supported Usages are `MEDIA`, for media content such as
music and videos, and `INTERRUPTION` for things like alarms that interrupt the user's task. Each
Usage has a separate volume control.

Audio streams such as `AudioRenderers` and `AudioCapturers` must be tagged with a Usage.

## Usage Volume {#usage-volume}

Volume is a floating point value between 0 and 1, where 0 is muted and 1 is
the maximum volume for the stream.

Some examples:

* If `MEDIA` is set to 0.5 volume and `INTERRUPTION` to 1.0, a stream tagged as
  `INTERRUPTION` would play back at half its loudness if played back instead as a `MEDIA` stream.
* If `INTERRUPTION` is set to 0 volume, all streams tagged as `INTERRUPTION` are
  inaudible to the user.

## Usage Gain Adjustment {#usage-gain-adjustment}

To realize the stream, the Fuchsia audio subsystem must translate volume settings to gain in dbfs
for each output device. Since devices have different ranges of gain and different mappings from
volume to gain, this translation may result in a different value for each output device.

After this translation, the usage's gain adjustment is applied. The gain adjustment is a
persistent setting in units of gain dbfs.

This is useful to enforce deltas between two usages when they are at the same volume.

For example, if `MEDIA` and `INTERRUPTION` are both set to 0.7 volume, but `MEDIA` has a gain
adjustment of -10db, a stream tagged as `INTERRUPTION` would not be as loud as if it is played back
tagged as `MEDIA`.

## Stream Gain Adjustment {#stream-gain-adjustment}

Another gain adjustment can be applied, directly to the stream. This gain adjustment value is local.

For example, if two `AudioRenderer`s exist on the system and one has a gain adjustment of -5db, the
other, if unmodified, still has a no-op gain adjustment of 0db.

## Mute State {#mute-state}

A stream, or usage, may be muted. When a stream is muted, it is not output to the user. When a
usage is muted, no stream tagged with that usage is output to the user.

During mute, other settings such as volume and gain are retained; muted is not the same as 0 volume
because the volume may be changed while muted but the stream remains inaudible.

When unmuted, streams will resume output at their previous loudness settings if those were not
modified during mute.

## Gain Adjustment Considerations

Volume is not a precisely linear mapping to loudness but it is relatively close for the human ear.

Gain is logarithmic, so a gain adjustment has a significantly different effect on loudness
depending on the current loudness of the stream to which it is applied.
