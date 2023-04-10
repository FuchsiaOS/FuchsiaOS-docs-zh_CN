<!--

// LINT.IfChange

-->

# audio-codec-ctl

Audio hardware codec driver control. Used to control and get information from an audio hardware
codec driver via the FIDL codec interface.
See [`audio-driver-ctl`](/reference/tools/hardware/audio-driver-ctl.md)
to play, record, and configure audio streams.

## Usage {#usage}

```none
  audio-codec-ctl [-d|--device <device>] f[ormats]

  audio-codec-ctl [-d|--device <device>] i[nfo]

  audio-codec-ctl [-d|--device <device>] c[apabilities_plug_detect]

  audio-codec-ctl [-d|--device <device>] b[ridgeable]

  audio-codec-ctl [-d|--device <device>] r[eset]

  audio-codec-ctl [-d|--device <device>] m[ode_bridged] true|false

  audio-codec-ctl [-d|--device <device>] d[ai] <number_of_channels>
    <channels_to_use_bitmask> pdm|upcm|spcm|fpcm none|i2s|left-stereo|right-stereo|1tdm|2tdm|3tdm
    <frame_rate> <bits_per_slot> <bits_per_sample>

  audio-codec-ctl [-d|--device <device>] start

  audio-codec-ctl [-d|--device <device>] stop

  audio-codec-ctl [-d|--device <device>] p[lug_state]

  audio-codec-ctl [-d|--device <device>] e[elements]

  audio-codec-ctl [-d|--device <device>] t[opologies]

  audio-codec-ctl [-d|--device <device>] w[atch] <id>

  audio-codec-ctl [-d|--device <device>] set <id> [enable|disable] [gain <gain>] [latency <nsecs>]
    [vendor <hex> <hex> ...]

  audio-codec-ctl [-h|--help]
```

## Commands {#commands}

Audio hardware codec driver control on `<device>` (full path specified e.g. `/dev/class/codec/123` or
only the devfs node name specified e.g. `123`) or unspecified (picks the first device in
/dev/class/codec). Only one command can be specified per invocation.


### `formats` {#formats}

```none
  audio-codec-ctl [-d|--device <device>] f[ormats]
```

Retrieves the DAI formats supported by the codec.

### `info` {#info}

```none
  audio-codec-ctl [-d|--device <device>] i[nfo]
```

Retrieves textual information about the codec.

### `capabilities_plug_detect` {#plugdetect}

```none
  audio-codec-ctl [-d|--device <device>] c[apabilities_plug_detect]
```

Retrieves Plug Detect Capabilities.

### `bridgeable` {#bridgeable}

```none
  audio-codec-ctl [-d|--device <device>] b[ridgeable]
```

Returns whether a codec is bridgeable.

### `reset` {#reset}

```none
  audio-codec-ctl [-d|--device <device>] r[eset]
```

Resets the codec.

### `mode_bridged` {#bridged}

```none
  audio-codec-ctl [-d|--device <device>] m[ode_bridged] true|false
```

Sets a codec bridged mode to true or false.

### `dai` {#dai}

```none
  audio-codec-ctl [-d|--device <device>] d[ai] <number_of_channels>
    <channels_to_use_bitmask> pdm|upcm|spcm|fpcm none|i2s|left-stereo|right-stereo|1tdm|2tdm|3tdm
    <frame_rate> <bits_per_slot> <bits_per_sample>
```

Sets the DAI format to be used in the codec interface.

`<number_of_channels>`: Number of channels.

`<channels_to_use_bitmask>`: Sets which channels are active via a bitmask. The least significant
  bit corresponds to channel index 0.

`pdm`: Pulse Density Modulation samples.

`upcm`: Signed Linear Pulse Code Modulation samples at the host endianness.

`spcm`: Unsigned Linear Pulse Code Modulation samples at the host endianness.

`fpcm`: Floating point samples IEEE-754 encoded.

`none`: No frame format as in samples without a frame sync like PDM.

`i2s`: Format as specified in the I2S specification.

`left-stereo`: Left justified, 2 channels.

`right-stereo`: Right justified, 2 channels.

`1tdm`: Left justified, variable number of channels, data starts at frame sync changes from low to
  high clocked out at the rising edge of sclk. The frame sync must stay high for exactly 1
  clock cycle.

`2tdm`: Left justified, variable number of channels, data starts one clock cycle after the frame
  sync changes from low to high clocked out at the rising edge of sclk. The frame sync must
  stay high for exactly 1 clock cycle.

`3tdm`: Left justified, variable number of channels, data starts two clock cycles after the frame
  sync changes from low to high clocked out at the rising edge of sclk. The frame sync must
  stay high for exactly 1 clock cycle.

`<frame_rate>`: The frame rate for all samples.

`<bits_per_slot>`: The bits per slot for all channels.

`<bits_per_sample>`: The bits per sample for all samples.  Must be smaller than bits per channel
  for samples to fit.

### `start` {#start}

```none
  audio-codec-ctl [-d|--device <device>] start
```

Start/Re-start the codec operation.

### `stop` {#stop}

```none
  audio-codec-ctl [-d|--device <device>] stop
```

Stops the codec operation.

### `plug_state` {#plug}

```none
  audio-codec-ctl [-d|--device <device>] p[lug_state]
```

Get the plug detect state.

### `elements` {#elements}

```none
  audio-codec-ctl [-d|--device <device>] e[lements]
```

Returns a vector of supported processing elements.

### `topologies` {#topologies}

```none
  audio-codec-ctl [-d|--device <device>] t[opologies]
```

Returns a vector of supported topologies.

### `watch` {#watch}

```none
  audio-codec-ctl [-d|--device <device>] w[atch] <id>
```

Get a processing element state.

### `set` {#set}

```none
  audio-codec-ctl [-d|--device <device>] set <id> [enable|disable] [gain <gain>] [latency <nsecs>]
    [vendor <hex> <hex> ...]
```

Controls a processing element.

`<id>`: Processing element id.

`enable`: Process element enabled state.

`disable`: Process element disabled state.

`<gain>`: Current gain in GainType format reported in the supported processing elements vector.

`<nsecs>`: Latency added to the pipeline in nanoseconds.

`<hex>`: Vendor specific raw byte to feed to the processing element in hex format.

## Examples {#examples}

### Retrieves the DAI formats supported

```none
$ audio-codec-ctl f
Executing on device: /dev/class/codec/706
[ fuchsia_hardware_audio::DaiSupportedFormats{ number_of_channels = [ 2, 4, ], sample_formats = [ fuchsia_hardware_audio::DaiSampleFormat::kPcmSigned, ], frame_formats = [ fuchsia_hardware_audio::DaiFrameFormat::frame_format_standard(fuchsia_hardware_audio::DaiFrameFormatStandard::kI2S), fuchsia_hardware_audio::DaiFrameFormat::frame_format_standard(fuchsia_hardware_audio::DaiFrameFormatStandard::kTdm1), ], frame_rates = [ 48000, 96000, ], bits_per_slot = [ 16, 32, ], bits_per_sample = [ 16, 32, ], }, ]
```

### Retrieves textual information

```none
$ audio-codec-ctl i
Executing on device: /dev/class/codec/706
fuchsia_hardware_audio::CodecInfo{ unique_id = "", manufacturer = "Texas Instruments", product_name = "TAS5825m", }
```

### Retrieves Plug Detect Capabilities

```none
$ audio-codec-ctl c
Executing on device: /dev/class/codec/706
fuchsia_hardware_audio::PlugDetectCapabilities::kHardwired
```

### Returns whether the codec is bridgeable

```none
$ audio-codec-ctl b
Executing on device: /dev/class/codec/706
Is bridgeable: false
```

### Resets the codec

```none
$ audio-codec-ctl r
Executing on device: /dev/class/codec/706
Reset done
```

### Sets a codec's bridged mode

```none
$ audio-codec-ctl -m true
Setting bridged mode to: true
Executing on device: /dev/class/codec/706
```

### Sets the DAI format to be used in the codec interface

```none
$ audio-codec-ctl d 2 1 s i 48000 16 32
Setting DAI format:
fuchsia_hardware_audio::DaiFormat{ number_of_channels = 2, channels_to_use_bitmask = 1, sample_format = fuchsia_hardware_audio::DaiSampleFormat::kPcmSigned, frame_format = fuchsia_hardware_audio::DaiFrameFormat::frame_format_standard(fuchsia_hardware_audio::DaiFrameFormatStandard::kI2S), frame_rate = 48000, bits_per_slot = 16, bits_per_sample = 32, }
Executing on device: /dev/class/codec/706
```

### Start/Re-start the codec operation

```none
$ audio-codec-ctl start
Executing on device: /dev/class/codec/706
Start done
```

### Stops the codec operation

```none
$ audio-codec-ctl stop
Executing on device: /dev/class/codec/706
Stop done
```

### Get the plug detect state

```none
$ audio-codec-ctl p
Executing on device: /dev/class/codec/706
fuchsia_hardware_audio::PlugState{ plugged = true, plug_state_time = 1167863520, }
```

### Get the supported processing elements

```none
$ audio-codec-ctl e
Executing on device: /dev/class/codec/706
[ fuchsia_hardware_audio_signalprocessing::Element{ id = 1, type = fuchsia_hardware_audio_signalprocessing::ElementType::kGain, type_specific = fuchsia_hardware_audio_signalprocessing::TypeSpecificElement::gain(fuchsia_hardware_audio_signalprocessing::Gain{ type = fuchsia_hardware_audio_signalprocessing::GainType::kDecibels, min_gain = -63.5, max_gain = 0, min_gain_step = 0.5, }), }, fuchsia_hardware_audio_signalprocessing::Element{ id = 2, type = fuchsia_hardware_audio_signalprocessing::ElementType::kMute, }, ]
```

### Get the supported topologies

```none
$ audio-codec-ctl t
Executing on device: /dev/class/codec/706
[ fuchsia_hardware_audio_signalprocessing::Topology{ id = 1, processing_elements_edge_pairs = [ fuchsia_hardware_audio_signalprocessing::EdgePair{ processing_element_id_from = 1, processing_element_id_to = 2, }, fuchsia_hardware_audio_signalprocessing::EdgePair{ processing_element_id_from = 2, processing_element_id_to = 3, }, ], }, ]
```

### Get a processing element state

```none
$ audio-codec-ctl w 1
Executing on device: /dev/class/codec/706
fuchsia_hardware_audio_signalprocessing::ElementState{ type_specific = fuchsia_hardware_audio_signalprocessing::TypeSpecificElementState::gain(fuchsia_hardware_audio_signalprocessing::GainElementState{ gain = 0, }), enabled = true, }
```

### Controls a processing element

```none
$ audio-codec-ctl set 1 enable gain 1.23 vendor 0x12 0x98
Setting element state:
fuchsia_hardware_audio_signalprocessing::SignalProcessingSetElementStateRequest{ processing_element_id = 1, state = fuchsia_hardware_audio_signalprocessing::ElementState{ type_specific = fuchsia_hardware_audio_signalprocessing::TypeSpecificElementState::gain(fuchsia_hardware_audio_signalprocessing::GainElementState{ gain = 1.23, }), enabled = true, vendor_specific_data = [ 18, 152, ], }, }
Executing on device: /dev/class/codec/706
```

### Specify device

```none
$ audio-codec-ctl -d 706 p
Executing on device: /dev/class/codec/706
fuchsia_hardware_audio::PlugState{ plugged = true, plug_state_time = 1167863520, }
$ audio-codec-ctl -d 123 p
Executing on device /dev/class/codec/123
watch plug state failed: FIDL operation failed due to peer closed, status: ZX_ERR_PEER_CLOSED (-24)
$ audio-codec-ctl -d /dev/class/codec/706 p
Executing on device: /dev/class/codec/706
fuchsia_hardware_audio::PlugState{ plugged = true, plug_state_time = 1167863520, }
```

### Source code {#source}

Source code for `audio-codec-ctl`: [`//src/media/audio/tools/audio-codec-ctl/`][src]

[src]: https://cs.opensource.google/fuchsia/fuchsia/+/main:src/media/audio/tools/audio-codec-ctl/

<!--

// LINT.ThenChange(//src/media/audio/tools/audio-codec-ctl/main.cc)

-->
