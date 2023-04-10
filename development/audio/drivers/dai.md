# Digital Audio Interface (DAI)

Caution: This page may contain information that is specific to the legacy
version of the driver framework (DFv1).

The DAI interface is a FIDL protocol exposed by DAI drivers. The DAI interface
controls the configuration of a DAI link for audio data transfer.

## Notation and terminology

-   All indices start from 0.
-   Vectors of n elements are represented as `<x0,x1,...,xn-1>`, for example a
    vector with two elements 5 and 6 as `<5,6>`.
-   Vectors can be nested, i.e. `<<5,6>,<7,8>>` represents a vector with 2
    vectors in it.

| Term         | Definition                                                     |
| ----------   | -------------------------------------------------------------- |
| DAI          | Digital Audio Interface. Interface between audio HW, for       :
:              : instance a TDM or PDM link between controllers and codecs.     :
| Frame Sync   | A DAI signal that marks frame boundaries, a.k.a. LRCLK, SYNC.  |
| Sclk         | A DAI signal used to mark the data line(s) bits transferring,  :
:              : a.k.a. SCK, BCLK.                                              :
| Mclk         | Master clock, a DAI signal sometimes needed to provide a clock |
:              : to DAIs. Sometimes Sclk is used as the Mclk (or Mclk is        :
:              : derived from the Sclk within the DAI).                         :
| Frame        | The representation of a single moment in time across data,     :
:              : frame sync and sclk in the DAI.                                :
| Frame format | A frame's data, frame sync and sclk arrangement, e.g. location |
:              : of the frame sync w.r.t. samples in the data line(s).          :
| Slot         | Within a frame, the bits reserved for a sample. A slot may be  |
:              : bigger than needed to hold the samples, e.g. 32 bits slot      :
:              : holding 24 or 16 bits samples.                                 :
| Channel      | A single source or destination of audio samples, usually       |
:              : to be rendered by a single speaker or captured by a single     :
:              : microphone. Every frame will contain samples in a fixed number :
:              : of slots for the same fixed number of channels.                :
| Sample       | A digital representation of sound taken at a particular time.  |

## Basic operation

The DAI client is responsible for configuring the DAI. The driver providing the
DAI interface (from here on the DAI driver) advertises supported formats and
allows the creation of a Ring Buffer for audio sample transferring.

Note that the DAI drivers are expected to perform their own shutdown, just like
any other driver (see [FDF](/docs/development/drivers/concepts/getting_started.md)).

## Protocol definition

The DAI protocol is defined in FIDL at
[dai.fidl](/sdk/fidl/fuchsia.hardware.audio/dai.fidl) and
[dai_format.fidl](/sdk/fidl/fuchsia.hardware.audio/dai_format.fidl).

Because the FDF does not currently provide a way to directly get a FIDL channel
for communication, we define a way to get a channel through
[Banjo](/docs/development/drivers/tutorials/banjo-tutorial.md) at
[fuchsia.hardware.audio](/sdk/banjo/fuchsia.hardware.audio/audio.fidl).

Direct connection to a DAI protocol server is provided by
[dai_connector.fidl](/sdk/fidl/fuchsia.hardware.audio/dai_connector.fidl).

### Reset {#reset}

A DAI can be reset by a client at any time by issuing the `Reset` function.

### GetInfo {#get-info}

The `GetInfo` function retrieves information from the DAI including:

1.  The manufacturer name.
1.  The product name.

### GetDaiFormats {#get-dai-formats}

The `GetDaiFormats` function allows the DAI driver to list its supported formats
for the DAI. The supported formats may include multiple sample formats, rates,
etc. DAI driver provides the supported formats and their clients mandate which
format is to be used in the `CreateRingBuffer` function.

The DAI driver replies with a vector of `DaiSupportedFormats`, where each
`DaiSupportedFormats` includes:

1.  A vector of number of channels. This lists the number of channels supported
    by the DAI, for example `<2,4,6,8>`. A stereo DAI reports a vector with
    one element `<2>`. Note that a DAI that takes one channel and inputs/outputs
    its contents in all its inputs/outputs (e.g. 2 for a stereo amplifier) would
    report a vector with one element `<1>`, if it supports either one or two
    input channels, it would report a vector with two elements `<1,2>`.
2.  A vector of sample formats. DAI sample formats, e.g. `PCM_SIGNED`.
3.  A vector of frame formats, For example `I2S` or `TDM1`, or the `CUSTOM`
    option where `DaiFrameFormatCustom` specifies each parameter of the frame
    configuration individually, e.g. `frame_sync_size` and `sclk_on_raising`.
4.  A vector of rates. Frame rates, for example 44100, 48000, and 96000.
5.  A number of bits per slot. Number of bits in each slot in the DAI,
    e.g. 32 bits per slot.
6.  A vector of bits per sample. Sample widths, e.g. 24 bits per sample.

Within a single `DaiSupportedFormats`, any combination of provided parameters is
supported.

When not all combinations supported by the DAI can be described with one
`DaiSupportedFormats`, the DAI returns more than one `DaiSupportedFormats` in
the returned vector. For example, if one `DaiSupportedFormats` allows for 32
bits samples at 48KHz, and 16 bits samples at 96KHz, but not 32 bits samples at
96KHz, then the DAI will reply with 2 `DaiSupportedFormats`:
`<<32bits>,<48KHz>>` and `<<16bits>,<96KHz>>`. For simplicity, this example
ignores parameters other than rate and bits per sample. In the case where the
DAI supports either 16 or 32 bits samples at either 48 or 96KHz, the DAI would
reply with 1 `DaiSupportedFormats`: `<<16bits,32bits>,<48KHz,96KHz>>`.

Additionally, it is assumed that bits per sample is always smaller or equal to
bits per slot. Hence, a DAI can report
`<<16bits_per_slot,32bits_per_slot>,<16bits_per_sample,32bits_per_sample>>`
and this does not imply that it is reporting that 32 bits per sample on 16 bits
samples is valid, it specifies only the 3 valid combinations:

1.  16 bits slot with 16 bits samples
2.  32 bits slot with 32 bits samples
3.  32 bits slot with 16 bits samples

### GetRingBufferFormats {#get-ring-buffer-formats}

Same as the streaming interface `GetSupportedFormats` function, see
[Audio Streaming Interface](streaming.md).

### CreateRingBuffer {#create-ring-buffer}

The client specifies both the format to use in the DAI and the format to use in
the ring buffer as part of the creating of the ring buffer in the
`CreateRingBuffer` function.

The DAI format parameter specifies:

1.  A number of channels. This is the number of channels in the DAI
    (for instance number of channels on a TDM bus, i.e. "on the wire").
2.  A channels to use bitmask. These are the channels in the DAI to be used for
    data transfer. For example to specify that both channels in an I2S DAI this
    must be 3.
3.  A sample format.
4.  A frame format.
5.  A frame rate.
6.  A number of bits per slot.
7.  A number of bits per sample.

The ring buffer format parameter is the same as the streaming interface
`CreateRingBuffer` function, see
[Audio Streaming Interface](streaming.md).

Once `CreateRingBuffer` is successful, the DAI format configuration is considered
completed and samples can be sent across the DAI once the `RingBuffer` protocol
configuration is completed and the `RingBuffer` `Start` function has been called.

For a description of the ring buffer functionality, see the [Audio Streaming
Interface](streaming.md).

{% comment %}
> TODO(fxbug.dev/63522): Add support for power control.
> TODO(fxbug.dev/63523): Add support for content protection.
{% endcomment %}

## Signal processing {#signal-processing}

Defined at [Audio Signal Processing](signal-processing.md).

