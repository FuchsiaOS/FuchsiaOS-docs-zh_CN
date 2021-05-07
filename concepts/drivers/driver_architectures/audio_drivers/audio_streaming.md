# Audio Driver Streaming Interface

This document describes the audio streaming interface exposed by audio drivers
in Fuchsia. It is meant to serve as a reference for both users and
driver-authors, and to unambiguously define the interface contract that drivers
must implement and users must follow.

## Overview

Audio streams are device nodes published by driver services intended to be used
by applications in order to capture or render audio on a Fuchsia device.
Each stream in the system (input or output) represents a stream of digital audio
information that may be either received or transmitted by device. Streams are
dynamic and may created or destroyed by the system at any time. Which streams
exist at any given point in time, and what controls their lifecycles are
considered to be issues of audio policy and codec management and are not
discussed in this document. Additionally, the information present in audio
outputs streams is exclusive to the application owner of the stream. Mixing of
audio is not a service provided by the audio stream interface.

{% comment %}
> TODO: extend this interface to support the concept of low-latency hardware
> mixers.
{% endcomment %}

### Definitions

| Term                          | Definition                                   |
| ----------------------------- | -------------------------------------------- |
| Sample                        | A representation of the sound rendered by a  |
:                               : single speaker, or captured by a single      :
:                               : microphone, at a single instant in time.     :
| LPCM                          | Linear pulse code modulation. The specific   |
:                               : representation of audio samples present in   :
:                               : all Fuchsia uncompressed audio streams. LPCM :
:                               : audio samples are representations of the     :
:                               : amplitude of the audio signal at an instant  :
:                               : in time where the numeric values of the      :
:                               : encoded audio are linearly distributed       :
:                               : across the amplitude levels of the rendering :
:                               : or capture device. This is in contrast to    :
:                               : A-law and &mu;-law encodings, which have     :
:                               : non-linear mappings from numeric value to    :
:                               : amplitude level.                             :
| Channel                       | Within an audio stream, the subset of        |
:                               : information that will be rendered by a       :
:                               : single speaker, or which was captured by a   :
:                               : single microphone in a stream.               :
| Frame                         | A set of audio samples for every channel of  |
:                               : a audio stream captured/rendered at a single :
:                               : instant in time.                             :
| Frame Rate                    | a.k.a. "Sample Rate". The rate (in Hz) at    |
:                               : which audio frames are produced or consumed. :
:                               : Common sample rates include 44.1 KHz, 48     :
:                               : KHz, 96 KHz, and so on.                      :
| Client or User or Application | These terms are used interchangeably in this |
:                               : document. They refer to modules that use     :
:                               : these interfaces to communicate with an      :
:                               : audio driver/device.                         :

{% comment %}
> TODO: do we need to extend this interface to support non-linear audio sample
> encodings? This may be important for telephony oriented microphones that
> deliver &mu;-law encoded samples.
{% endcomment %}

### Basic Operation

Communication with an audio stream device is performed using messages sent over
a [channel](/docs/reference/kernel_objects/channel.md). Applications open the device node for a
stream and obtain a channel by issuing a FIDL request. After obtaining the
channel, the device node may be closed. All subsequent communication with the
stream occurs using channels.

The stream channel is used for most command and control tasks, including:

*   Capability interrogation
*   Format negotiation
*   Hardware gain control
*   Determining outboard latency
*   Plug detection notification
*   Access control capability detection and signalling

{% comment %}
> TODO: Should plug/unplug detection be done by sending notifications over the
> stream channel (as it is today), or by publishing/unpublishing the device
> nodes (and closing all channels in the case of unpublished channels)?
{% endcomment %}

In order to actually send or receive audio information on the stream, the
specific format to be used must first be set. The response to a successful
`CreateRingBuffer` operation will contain a new "ring-buffer" channel. The ring-buffer
channel may be used to request a shared buffer from the stream (delivered in the
form of a [VMO](/docs/reference/kernel_objects/vm_object.md)) that may be mapped into the address
space of the application and used to send or receive audio data as appropriate.
Generally, the operations conducted over the ring buffer channel include:

*   Requesting a shared buffer
*   Starting and Stopping stream playback and capture
*   Receiving notifications of playback and capture progress
*   Receiving clock recovery information in the case that the audio output clock
    is based on a different oscillator than the oscillator that backs
    the [monotonic clock](/docs/reference/syscalls/clock_get_monotonic.md)

## Operational Details

### Device nodes

Audio stream device nodes must be published by drivers using the protocol
preprocessor symbol given in the table below. This will cause stream device
nodes to be published in the locations given in the table. Applications can
monitor these directories in order to discover new streams as they are published
by the drivers.

Stream Type | Protocol                   | Location
----------- | -------------------------- | -----------------------
Input       | `ZX_PROTOCOL_AUDIO_INPUT`  | /dev/class/audio-input
Output      | `ZX_PROTOCOL_AUDIO_OUTPUT` | /dev/class/audio-output

### Establishing the stream channel

After opening the device node, client applications may obtain a stream channel
for subsequent communication using the
`fuchsia.hardware.audio.Device/GetChannel` FIDL message.

### Client side termination of the stream channel

Clients may terminate the connection to the stream at any time simply by
calling [zx_handle_close(...)](/docs/reference/syscalls/handle_close.md) on the stream
channel. Drivers must close any active ring-buffer channels established
using this stream channel and must make every attempt to gracefully quiesce
any on-going streaming operations in the process.

### Sending and receiving messages on the stream and ring-buffer channels

All of the messages and message payloads that may be sent or received over
stream and ring buffer channels are defined in
[stream.fidl](/sdk/fidl/fuchsia.hardware.audio/stream.fidl)
and [ring_buffer.fidl](/sdk/fidl/fuchsia.hardware.audio/ring_buffer.fidl).
Messages may be sent to the driver using the
[zx_channel_write(...)](/docs/reference/syscalls/channel_write.md) syscall. If a response is
expected, it may be read using the
[zx_channel_read(...)](/docs/reference/syscalls/channel_read.md) syscall. Best practice,
however, is to queue packets for your [channel(s)](/docs/reference/kernel_objects/channel.md)
[port](/docs/reference/kernel_objects/port.md) using the
[zx_port_queue(...)](/docs/reference/syscalls/port_queue.md) syscall, and use the
[zx_port_wait(...)](/docs/reference/syscalls/port_wait.md) syscall to determine when your set
of channels have messages (either expected responses or asynchronous
notifications) to be read.
There are bindings for different languages to facilitate sending and receiving
FIDL messages, and in particular for C++ drivers there is also a library
[SimpleAudioStream](/src/media/audio/lib/simple-audio-stream) that facilitates the creation
of drivers in C++, this library uses the
[LLCPP](/docs/reference/fidl/bindings/llcpp-bindings.md) bindings to send and receive
FIDL messages.

## Format Negotiation

### Sample Formats

The `Format` related protocol messages allow the driver to list its supported
formats to the client. The supported formats may include multiple rates, bit per sample,
etc. Each driver advertises what it can support and the client mandates what format
is to be used for each driver.

To find out what formats are supported by a given driver, the client uses the
`GetSupportedFormats` function. The driver replies with a vector of
`SupportedFormats`, where each `SupportedFormats` includes a `PcmSupportedFormats` with:

* A vector of number of channels. This lists the number of channels supported
 by the driver, for example `<2,4,6,8>`. A driver that supports either two or
 four channels would report a vector with two elements `<2,4>`. Must be in ascending order.
* A vector of sample formats, e.g. `PCM_SIGNED`.
* A vector of rates. Frame rates, for example 44100, 48000, and 96000. Must be in ascending order.
* A number of bits per channel/slot/container. Number of bits in each channel allocated
 to hold a sample, e.g. 32 bits per channel. Must be in ascending order.
* A vector of bits per sample. Sample widths, this could be smaller than the channel
e.g. 24 bits per sample in a 32 bits channel. Must be in ascending order.

When not all combinations supported by the driver can be described with one
`PcmSupportedFormats`, the driver returns more than one `PcmSupportedFormats` in
the returned vector. For example, if one `PcmSupportedFormats` allows for 16 or 32 bits samples at
48KHz, and 16 bits samples at 96KHz, but not 32 bits samples at 96KHz, then the driver
replies with 2 `PcmSupportedFormats`: `<<16bits,32bits>,<48KHz>>` and
`<<16bits>,<96KHz>>`. For simplicity, this example ignores parameters other than
rate and bits per sample. In the case where the driver supports either 16 or 32
bits samples at either 48 or 96KHz, the driver would reply with 1
`PcmSupportedFormats`: `<<16bits,32bits>,<48KHz,96KHz>>`.

Additionally, it is assumed that bits per sample is always smaller or equal to
bits per channel. Hence, a driver can report
`<<16bits_per_channel,32bits_per_channel>,<16bits_per_sample,32bits_per_sample>>`
and this does not imply that it is reporting that 32 bits per sample on 16 bits
samples is valid, it specifies only the 3 valid combinations:

* 16 bits channels with 16 bits samples
* 32 bits channels with 32 bits samples
* 32 bits channels with 16 bits samples

The client specifies the format to use with the `CreateRingBuffer` function based on
information that the driver provides in `GetSupportedFormats` reply, what is supported
by the client, and any other requirements. This function takes a parameter that specifies:

* A number of channels. This is the number of channels available in the buffer.
* A bitmask of channels to use. These are the channels in the buffer to be used by
 the driver. For instance for stereo this must be a bitmask with 2 bits enabled `0x3`,
 i.e. both channels 0 and 1 are used.
* A sample format.
* A frame rate.
* A number of bits per channel.
* A number of bits per sample.

Notes:

*   By default, multi-byte sample formats are assumed to use host-endianness.
*   The `PCM_FLOAT` encoding uses specifically the
    [IEEE 754](https://en.wikipedia.org/wiki/IEEE_754) floating point
    representation.
*   By default, non-floating point PCM encodings are assumed expressed using
    [two's complement](https://en.wikipedia.org/wiki/Two%27s_complement) signed
    integers. eg. the bit values for a 16 bit PCM sample format would range from
    [0x8000, 0x7FFF] with 0x0000 representing zero speaker deflection. If the
    `PCM_UNSIGNED` sample format is used, the bit values would range from [0x0000,
    0xFFFF] with 0x8000 representing zero deflection.
*   When encoding a smaller sample size in a larger channel (e.g. 20 or 24bit in
    32), the most significant bits of the 32 bit container are used while the
    least significant bits will be ignored (left justified). e.g. a 20 bit sample would be mapped
    onto the range \[12,31\] (bits \[0,11\] would be ignored) of the 32 bit container.

### Setting the desired stream format

In order to select a stream format, applications send a `CreateRingBuffer` message over the
stream channel. In the message, the application specifies the format to be used.

The client specifies the new ring buffer channel over which
streaming operations will be conducted. If a previous ring buffer channel had been
established and was still active, the driver must close this channel and
make every attempt to gracefully quiesce any on-going streaming operations in
the process.

> TODO: specify how compressed bitstream formats will be set

## Determining external latency

The external latency of an audio stream is defined as the amount of time it
takes outbound audio to travel from the system's interconnect to the speakers
themselves, or inbound audio to travel from the microphone to the system's
interconnect. As an example, consider an external codec connected to the system
using a TDM interconnect: if this interconnect introduces a 4 frame delay
between the reception of a TDM frame and the rendering of that frame at the
speakers themselves, then the external delay of this audio path is the time
duration equivalent to 4 audio frames.

External delay is reported in the `external_delay` field of a `RingBufferProperties`
response to a `GetProperties`.  Drivers should make their best attempt to
accurately report the total of all of the sources of delay the driver knows about.
Information about this delay can frequently be found in codec data sheets,
dynamically reported as properties of codecs using protocols such as Intel HDA
or the USB Audio specifications, or reported by down stream devices using
mechanisms such as EDID when using HDMI or DisplayPort interconnects.

## Hardware gain control

### Hardware gain control capability reporting

In order to determine a stream's gain control capabilities, if it has not done
so yet, an application sends a `GetProperties` message over the stream channel.
No parameters need to be supplied with this message. The driver replies with a
`StreamProperties` including gain capabilities among others. All stream drivers
must respond to this message, regardless of whether or not the stream
hardware is capable of any gain control. All gain values are expressed using 32
bit floating point numbers expressed in dB.

Drivers respond to this message with values that indicate the current stream's
gain control capabilities. Current gain settings are expressed using a bool
indicating whether the stream can be muted, a bool that indicates whether the
stream can AGC, the minimum and maximum gain settings, and a `gain_step_db`. The
`gain_step_db` indicates the smallest increment with which the gain can be
controlled counting from the minimum gain value.

For example, an amplifier that has 5 gain steps of 7.5 dB each and a maximum 0
dB gain would indicate a range of (-30.0, 0.0) and a step size of 7.5.
Amplifiers capable of functionally continuous gain control may encode their
gain step size as 0.0.

Regardless of mute capabilities, drivers for fixed gain streams must report
their min and max gain as (0.0, 0.0). `gain_step_db` is meaningless in this
situation, but drivers should report it as 0.0.

### Setting hardware gain control levels

In order to change a stream's current gain settings, applications send a
`SetGain` message over the stream channel. This message include a parameter
`GainState` indicating gain parameters to be configured including the dB gain that
should be applied to the stream, muted and AGC enablement.

Presuming that the request is valid, drivers should round the request to the
nearest supported gain step size. For example, if a stream can control its gain
on the range from -60.0 to 0.0 dB, using a gain step size of 0.5 dB, then a
request to set the gain to -33.3 dB should result in a gain of -33.5 being
applied. A request to that same stream for a gain of -33.2 dB should result
in a gain of -33.0 being applied.

### Gain state notifications

Clients may request that streams send them asynchronous notifications of
gain state changes by using the `WatchGainState` command. The driver will reply to the
first |WatchGainState| sent by the client and will not respond to subsequent
client |WatchGainState| calls until the gain state changes from what was most recently
reported.

## Plug detection

In addition to streams being published/unpublished in response to being
connected or disconnected to/from their bus, streams may have the ability to be
plugged or unplugged at any given point in time. For example, a set of USB
headphones may publish a new output stream when connected to USB, but choose to
be "hardwired" from a plug detection standpoint. A different USB audio adapter
with a standard 3.5mm phono jack might publish an output stream when connected
with USB, but choose to change its plugged/unplugged state as the user plugs and
unplugs an analog device with the 3.5mm jack.

The ability to query the currently plugged or unplugged state of a stream, and
to register for asynchonous notifications of plug state changes (if supported)
is handled through plug detection messages.

### Plug detect capabilities

In order to determine a stream's plug detection capabilities, if it has not done
so yet, an application sends a `GetProperties` command over the stream channel.
The driver replies with a `StreamProperties` including plug detect capabilities
in `plug_detect_capabilities` among others fields.

Valid plug-detect capabilities flags currently defined are:

*   `HARDWIRED` is set when the stream hardware is considered to be
    "hardwired". In other words, the stream is considered to be connected as
    long as the device is published. Examples include a set of built-in
    speakers, a pair of USB headphones, or a pluggable audio device with no plug
    detection functionality.
*   `CAN_ASYNC_NOTIFY` is set when the stream hardware is capable of both
    asynchronously detecting that a device's plug state has changed, and sending
    a notification message if the client has requested these notifications.

### Plug state notifications

Clients may request that streams send them asynchronous notifications of
plug state changes by using the `WatchPlugState` command if the `CAN_ASYNC_NOTIFY`
flag was sent by the driver in `StreamProperties`. I.e. drivers for streams that
do not set the `CAN_ASYNC_NOTIFY` flag are free to ignore the `WatchPlugState` sent
by applications. Driver with `CAN_ASYNC_NOTIFY` set will reply to the first
|WatchPlugState| sent by the client and will not respond to subsequent client
|WatchPlugState| calls until the plug state changes from what was most recently reported.

## Access control capability detection and signaling

> TODO: specify how this works. In particular, specify how drivers indicate to
> applications support for various digital access control mechanisms such as
> S/PDIF control words and HDCP.

## Stream purpose and association

{% comment %}
> TODO: specify how drivers can indicate the general "purpose" of an audio
> stream in the system (if known), as well as its relationship to other streams
> (if known). For example, an embedded target like a phone or a tablet needs to
> indicate which output stream is the built-in speaker vs. which is the headset
> jack output. In addition, it needs to make clear which input stream is the
> microphone associated with the headset output vs. the builtin speaker.
{% endcomment %}

## Ring-Buffer channels

### Overview

Once an application has successfully set the format of a stream, it receives in
the response a new [channel](/docs/reference/kernel_objects/channel.md) representing its connection
to the stream's ring-buffer. Clients use the ring-buffer channel to establish a
shared memory buffer and start and stop playback and capture of audio stream data.

The ring buffer contents are produced by the client side (for playback) and the
driver side (for recording). Hence, a client is a producer for playback and a consumer
for recording and a driver is a producer for recording and a consumer for playback.
The ring buffer contents may be directly consumed or produced by the audio hardware, or
it may go through software processing of each sample done by the driver.

Ring buffer data production proceeds at the nominal rate from the point in time
given in a successful response to the `Start` command. Note though that the ring-buffer
will almost certainly have some form of FIFO buffer
between the memory bus and the audio hardware, which causes it to either
read-ahead in the stream (in the case of playback), or potentially hold onto
data (in the case of capturing). It is important for clients to query the size
of this buffer before beginning
operation so they know how far ahead/behind the stream's nominal inferred
read/write position they need to stay in order to prevent audio glitching.
Also note that because of the shared buffer nature of the system, and the fact
that drivers are likely to be DMA-ing directly from this buffer to hardware, it
is important for clients running on architectures that are not automatically
cache coherent to be sure that they have properly written-back their cache after
writing playback data to the buffer, or invalidated their cache before reading
captured data.

### Determining the FIFO depth

In order to determine a stream's `fifo_depth`, if it has not done so yet, an application
sends a `GetProperties` command over the stream channel. The driver replies with a
`StreamProperties` including `fifo_depth` among others fields. To ensure proper
playback or capture of audio, applications and drivers must be careful to
respect this value. Drivers must not read beyond the nominal playback position
of the stream plus this number of bytes when playing audio stream data.
Applications must stay this number of bytes behind the nominal capture point of
the stream when capturing audio stream data.

The ring buffer data may be directly consumed/generated by hardware, in this case
`fifo_depth` maps directly to the size of a hardware FIFO block, since the hardware FIFO
block determines the amount of data read ahead or held back.

The ring buffer data may instead be consumed/generated by audio driver software that is
conceptually situated between the ring buffer and the audio hardware. In this case, for
playback the `fifo_depth` read ahead amount is set large enough such that the driver
guarantees no undetected underruns, this assuming the client is generating the data as
determined by the `CreateRingBuffer` and `Start` commands. For capture, the
`fifo_depth` held back amount is set large enough such that the driver guarantees no
undetected underruns when generating the data as determined by the `CreateRingBuffer` and
`Start` commands. The driver must set `fifo_depth` big enough such that the potential
delays added by any software interfacing with the audio hardware do not occur under most
scenarios, and must detect and report underruns. How an underrun is reported is not defined
here.

Once the format of a stream is set and a ring-buffer channel has been opened,
the driver must not change this value. From an application's point of view,
it is a constant property of the ring-buffer channel.

### Obtaining a shared buffer

To send or receive audio, the application must first establish a shared memory
buffer. This is done by sending an `CreateRingBuffer` request over the
ring-buffer channel. This may only be done while the ring-buffer is stopped.

If the channel created with `CreateRingBuffer` is closed by the driver for instance
because a buffer has already been established and the ring-buffer has already
been started, it must not either stop the ring-buffer, or discard the
existing shared memory. If the application requests a new buffer after having
already established a buffer while the ring buffer is stopped, it must
consider the existing buffer ii has to be invalid, the old buffer is now gone.

Applications must specify two parameters when requesting a ring buffer:
`min_frames` and `clock_recovery_notifications_per_ring`.

#### `min_frames`

The minimum number of frames of audio the client needs allocated for the ring
buffer. Drivers may make this buffer larger to meet hardware requirements.
Clients must use the returned VMOs size (in bytes) to determine the actual
size of the ring buffer. Clients must not assume that the size of the buffer
(as determined by the driver) is exactly the size they requested. Drivers
must ensure that the size of the ring buffer is an integral number of audio
frames.

{% comment %}
> TODO : Is it reasonable to require that drivers produce buffers that are an
> integral number of audio frames in length? It certainly makes the audio
> client's life easier (client code never needs to split or re-assemble a frame
> before processing), but it might make it difficult for some audio hardware to
> meet its requirements without making the buffer significantly larger than the
> client asked for.
{% endcomment %}

#### `clock_recovery_notifications_per_ring`

Optional number of position update notifications the client would like the driver to
send per cycle through the ring buffer, these notifications are meant to be used for clock
recovery. Drivers must only send these as a reply to a `WatchClockRecoveryPositionInfo` request.
Drivers should attempt to space notifications uniformly throughout the ring; however clients
must not rely on perfectly uniform spacing of the update notifications.

#### `ring_buffer`

If the request succeeds, the driver must return a handle to a
[VMO](/docs/reference/kernel_objects/vm_object.md) with permissions that allow applications to map
the VMO into their address space using [zx_vmar_map](/docs/reference/syscalls/vmar_map.md),
and to read/write data in the buffer in the case of playback, or simply to read
the data in the buffer in the case of capture.

#### `num_frames`

If the request succeeds, the driver will also return the actual number of frames of audio
it will use in the buffer. The size of the VMO returned (as reported
by [zx_vmo_get_size()](/docs/reference/syscalls/vmo_get_size.md)) must not be larger than
this number of frames (when converted to bytes). This number may be larger
than the `min_frames` request from the client but must not be smaller than this number.

### Starting and Stopping the ring-buffer

Clients may request that a ring-buffer start or stop using the `Start` and `Stop`
commands. Attempting to start a stream
which is already started must be considered a failure. Attempting to stop a
stream that is already stopped should be considered a success. Ring-buffers
cannot be either stopped or started until after a shared buffer has been
established using the `CreateRingBuffer` operation.

Upon successfully starting a stream, drivers must provide their best estimate of
the time at which their hardware began to transmit or capture the stream in the
`start_time` field of the response. This time stamp must be taken from the clock
exposed with the
[zx_clock_get_monotonic()](/docs/reference/syscalls/clock_get_monotonic.md)
syscall. Along with the FIFO depth property of the ring buffer, this timestamp
allows applications to send or receive stream data without the need for periodic
position updates from the driver. Along with the outboard latency estimate
provided by the stream channel, this timestamp allows applications to
synchronize presentation of audio information across multiple streams, or even
multiple devices (provided that an external time synchronization protocol is
used to synchronize the
[monotonic](/docs/reference/syscalls/clock_get_monotonic.md) timelines across
the cohort of synchronized devices).

{% comment %}
> TODO: Redefine `start_time` to allow it to be an arbitrary 'audio stream
> clock' instead of the `zx_clock_get_monotonic()` clock. If the stream clock is
> made to count in audio frames since start, then this `start_time` can be
> replaced with the terms for a segment of a piecewise linear transformation
> that can be subsequently updated through notifications sent by the driver in the
> case that the audio hardware clock is rooted in a different oscillator from
> the system's tick counter. Clients can then use this transformation either to
> control the rate of consumption of input streams, or to determine where to
> sample in the input stream to effect clock correction.
{% endcomment %}

Upon successfully starting a stream, drivers must guarantee that no position
notifications will be sent before the start response has been enqueued into the
ring-buffer channel.

Upon successfully stopping a stream, drivers must guarantee that no position
notifications will be enqueued into the ring-buffer channel after the stop
response has been enqueued.

### Position notifications

If requested by the client through a non-zero `clock_recovery_notifications_per_ring` in the
`CreateRingBuffer` operation, the driver will
periodically send updates to the client informing it of its current production
or consumption position in the buffer. This position is expressed in bytes in
the `position` field of the `RingBufferPositionInfo` struct sent on
a reply to the `WatchClockRecoveryPositionInfo` message. The
message also includes a `timestamp` field that contains the time (as
zx::time) that this byte position was valid. A `WatchClockRecoveryPositionInfo`
request must only be sent after `clock_recovery_notifications_per_ring` has been
specified in the `GetVmo` function and the `GetVmo` function has returned. Note,
these position
notifications indicate where in the buffer the driver has consumed or produced
data, *not* the nominal playback or capture position (sometimes called the
"write cursor" or "read cursor" respectively). The timing of their arrival is
not guaranteed to be perfectly uniform and should not be used to effect clock
recovery. However, the correspondence pair (`timestamp`, `position`)
values themselves ARE intended to be used to recover the clock for the audio
stream. If a client discovers that a driver has consumed past the point in the
ring buffer where that client has written playback data, audio presentation is
undefined. Clients should increase their clock lead time and be certain to stay
ahead of this point in the stream in the future. Likewise, clients that capture
audio should not attempt to read beyond the point in the ring buffer
indicated by the most recent position notification sent by the driver.

Driver playback and capture position must *always* begin at ring buffer byte 0,
immediately following a successful `Start` command. When the ring
buffer position reaches the end of the VMO (as indicated by
[zx_vmo_get_size(...)](/docs/reference/syscalls/vmo_get_size.md)), the ring buffer position
wraps back to zero. Drivers are not required to consume or produce data in
integral numbers of audio frames. Clients whose notion of stream position
depends on position notifications should take care to request that a sufficient
number of notifications per ring be sent (minimum 2) and to process them quickly
enough that aliasing does not occur.

### Clock recovery and synchronization

Upon receiving `AUDIO_STREAM_CMD_GET_CLOCK_DOMAIN` message, the driver must
respond with the identifier of the clock domain containing that device. If the
audio device is locked to the local system monotonic clock and does not expose a
mechanism by which its rate would be fine-tuned, then it should return the value
0 to represent the local CLOCK_MONOTONIC domain. A client may use this
information (in addition to `AUDIO_RB_POSITION_NOTIFY` messages) to simplify the
process of recovering the audio device's clock.

{% comment %}
> TODO: extend this section to include how clock recovery occurs, and how this
> is exposed to clients. Also, detail how slewable oscillators are discovered
> and controlled. We may need rate-change notifications to clients of slewable
> clocks.
>
> Previous content: TODO: define a way that clock recovery information can be
> sent to clients in the case that the audio output oscillator is not derived
> from the `zx_clock_get_monotonic()` oscillator. In addition, if the oscillator
> is slew-able in hardware, provide the ability to discover this capability and
> control the slew rate. Given the fact that this oscillator is likely to be
> shared by multiple streams, it might be best to return some form of system
> wide clock identifier and provide the ability to obtain a channel on which
> clock recovery notifications can be delivered to clients and hardware slewing
> command can be sent from clients to the clock.
{% endcomment %}

### Error notifications

{% comment %}
TODO: define these and what driver behavior should be, if/when they occur.
{% endcomment %}

### Unexpected client termination

If the client side of a ring buffer control channel is closed for any reason,
drivers must immediately close the control channel and shut down the ring
buffer, such that no further audio is emitted nor captured. While drivers are
encouraged to do so in a way that produces a graceful transition to silence,
they must ensure that the audio stream goes silent instead of looping. Once
the transition to silence is complete, resources associated with playback or
capture may be released and reused by the driver.

This way, if a playback client teminates unexpectedly, the system will close the
client channels, causing audio playback to stop instead of continuing to loop.
