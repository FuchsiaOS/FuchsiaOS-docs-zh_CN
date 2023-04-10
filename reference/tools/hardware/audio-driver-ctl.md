<!--

// LINT.IfChange

-->

# audio-driver-ctl

Play, record, and configure audio streams.

## Usage {#usage}

```none
audio-driver-ctl [-d <id>] [-t (input|output)] agc (on|off)

audio-driver-ctl [-a <mask>] [-b (8|16|20|24|32)] [-c <channels>] \
    [-d <id>] [-r <hertz>] duplex <playpath> <recordpath>

audio-driver-ctl [-d <id>] [-t (input|output)] gain <decibels>

audio-driver-ctl [-d <id>] [-t (input|output)] info

audio-driver-ctl [-a <mask>] [-b (8|16|20|24|32)] [-c <channels>] \
    [-d <id>] loop <playpath>

audio-driver-ctl [-d <id>] [-t (input|output)] mute

audio-driver-ctl [-a <mask>] [-b (8|16|20|24|32)] [-c <channels>] \
    [-d <id>] [-r <hertz>] noise [<seconds>] [<amplitude>]

audio-driver-ctl [-a <mask>] [-b (8|16|20|24|32)] [-c <channels>] \
    [-d <id>] play <playpath>

audio-driver-ctl [-d <id>] [-t (input|output)] pmon [<seconds>]

audio-driver-ctl [-a <mask>] [-b (8|16|20|24|32)] [-c <channels>] \
    [-d <id>] [-r <hertz>] record <recordpath> [<seconds>]

audio-driver-ctl [-a <mask>] [-b (8|16|20|24|32)] [-c <channels>] \
    [-d <id>] [-r <hertz>] tone [<frequency>] [<seconds>] [<amplitude>

audio-driver-ctl [-d <id>] [-t (input|output)] unmute
```

## Options {#options}

### `-a <mask>` {#a}

Active channel mask. For example `0xf` or `15` for channels 0, 1, 2, and 3.
Defaults to all channels.

### `-b (8|16|20|24|32)` {#b}

Bits per sample. Defaults to `16`.

### `-c <channels>` {#c}

Number of channels to use when recording or generating tones/noises.
Does not affect WAV file playback because WAV files specify how many
channels to use in their headers. Defaults to the first driver-reported
value. Run [`info`](#info) to see how many channels your target Fuchsia device
has. The number of channels must match what the audio driver expects
because `audio-driver-ctl` does not do any mixing.

### `-d <id>` {#d}

The device node ID of the stream. Defaults to `0`. To figure out `<id>` run
[`info`](#info). You'll see a path value like `/dev/class/audio-input/000`.
`<id>` in this example is `000`.

### `-t (input|output)` {#t}

The device type. Defaults to `output`. This option is ignored for commands like
[`play`](#play) that only make sense for one of the types.

### `-r <hertz>` {#r}

The frame rate in hertz. Defaults to `48000`.

## Commands {#commands}

### `agc` {#agc>}

```none
audio-driver-ctl [-d <id>] [-t (input|output)] agc (on|off)
```

Enables or disables automatic gain control for the stream.

### `duplex` {#duplex}

```none
audio-driver-ctl [-a <mask>] [-b (8|16|20|24|32)] [-c <channels>] \
    [-d <id>] [-r <hertz>] duplex <playpath> <recordpath>
```

Simultaneously plays the WAV file located at `<playpath>` and records
another WAV file into `<recordpath>` in order to analyze the delays in the
system. The `-c` option if provided applies to the recording side since the
number of channels for playback is taken from the WAV file header.

### `gain` {#gain}

```none
audio-driver-ctl [-d <id>] [-t (input|output)] gain <decibels>
```

Sets the gain of the stream in decibels.

### `info` {#info}

```none
audio-driver-ctl [-d <id>] [-t (input|output)] info
```

Gets capability and status info for a stream.

### `loop` {#loop}

```none
audio-driver-ctl [-a <mask>] [-b (8|16|20|24|32)] [-c <channels>] \
    [-d <id>] loop <playpath>
```

Repeatedly plays the WAV file at `<playpath>` on the selected output until a key
is pressed.

### `mute` {#mute}

```none
audio-driver-ctl [-d <id>] [-t (input|output)] mute
```

Mutes a stream.

### `noise` {#noise}

```none
audio-driver-ctl [-a <mask>] [-b (8|16|20|24|32)] [-c <channels>] \
    [-d <id>] [-r <hertz>] noise [<seconds>] [<amplitude>]
```

Plays pseudo-white noise. `<seconds>` controls how long the noise plays and must
be at least `0.001` seconds. If `<seconds>` is not provided the noise plays until
a key is pressed.

### `play` {#play}

```none
audio-driver-ctl [-a <mask>] [-b (8|16|20|24|32)] [-c <channels>] \
    [-d <id>] play <playpath>
```

Plays a WAV file.

### `pmon` {#pmon}

```none
audio-driver-ctl [-d <id>] [-t (input|output)] pmon [<seconds>]
```

Monitors the plug state of a stream. `<seconds>` must be above `0.5` seconds
(default: `10.0` seconds).

### `record` {#record}

```none
audio-driver-ctl [-a <mask>] [-b (8|16|20|24|32)] [-c <channels>] \
    [-d <id>] [-r <hertz>] record <recordpath> [<seconds>]
```

Records to the specified WAV file from the selected input. If `<seconds>` is not
provided the input is recorded until a key is pressed.

### `tone` {#tone}

```none
audio-driver-ctl [-a <mask>] [-b (8|16|20|24|32)] [-c <channels>] \
    [-d <id>] [-r <hertz>] tone [<frequency>] [<seconds>] [<amplitude>
```

Plays a sinusoidal tone. `<frequency>` must be between `15.0` and `96000.0` hertz
(default: `440.0` hertz). `<seconds>` must be above `0.001` seconds. If <seconds> is
not provided the tone plays until a key is pressed. `<amplitude>` scales the
output if provided and must be an increment of 0.1 between `0.1` and `1.0`.

### `unmute` {#unmute}

```none
audio-driver-ctl [-d <id>] [-t (input|output)] unmute
```

Unmutes a stream. Note that the gain of the stream will be reset to its default
value.

## Examples {#examples}

### Enable automatic gain control on a stream {#examples-agc}

```none {:.devsite-disable-click-to-copy}
$ audio-driver-ctl agc on
```

### Get stream info {#examples-info}

```none {:.devsite-disable-click-to-copy}
# Equivalent to `audio-driver-ctl -t output -d 000 info`
$ audio-driver-ctl info
Info for audio output at \"/dev/class/audio-output/000\"
  Unique ID    : 0100000000000000-0000000000000000
  Manufacturer : Spacely Sprockets
  Product      : acme
  Current Gain : 0.00 dB (unmuted, AGC on)
  Gain Caps    : gain range [-103.00, 24.00] in 0.50 dB steps; can mute; can AGC
  Plug State   : plugged
  Plug Time    : 12297829382473034410
  PD Caps      : hardwired
Number of channels      : 1
Frame rate              : 8000Hz
Bits per channel        : 16
Valid bits per channel  : 16
...
```

### Set gain of a stream to -40 decibels {#examples-gain}

```none {:.devsite-disable-click-to-copy}
# Equivalent to `audio-driver-ctl -t output -d 000 gain -40`
$ audio-driver-ctl gain -40
```

### Mute a stream {#examples-mute}

```none {:.devsite-disable-click-to-copy}
# Equivalent to `audio-driver-ctl -t output -d 000 mute`
$ audio-driver-ctl mute
```

### Repeatedly play (loop) a WAV file on a stream {#examples-loop}

```none {:.devsite-disable-click-to-copy}
# Equivalent to `audio-driver-ctl -t output -d 000 loop /tmp/test.wav`
$ audio-driver-ctl loop /tmp/test.wav
Looping /tmp/test.wav until a key is pressed
```

### Play a WAV file once on a stream {#examples-play}

```none {:.devsite-disable-click-to-copy}
# Equivalent to `audio-driver-ctl -t output -d 000 play /tmp/test.wav`
$ audio-driver-ctl play /tmp/test.wav
```

### Play a 450 hertz tone for 1 second at 50% amplitude on a stream {#examples-tone}

```none {:.devsite-disable-click-to-copy}
# Equivalent to `audio-driver-ctl -t output -d 000 tone 450 1 0.5`
$ audio-driver-ctl tone 450 1 0.5
Playing 450.00 Hz tone for 1.00 seconds at 0.50 amplitude
```

### Unmute a stream {#examples-unmute}

```none {:.devsite-disable-click-to-copy}
# Equivalent to `audio-driver-ctl -t output -d 000 unmute`
audio-driver-ctl unmute
```

## Notes {#notes}

<<./_access.md>>

### Supported builds for commands that exercise streams {#builds}

Commands that exercise audio streams such as [`play`](#play) are only supported
in diagnostic [product bundles][glossary.product-bundle] like `core`.
In other builds only the informational commands like `info` are supported.

### Copying WAV files between a host and a target Fuchsia device {#copy}

To copy WAV files from your host to your target Fuchsia device or
vice versa, run `fx cp (--to-target|--to-host) <source> <destination>`
on your host. `<source>` is the file you want to copy and `<destination>`
is where you want to put the copied file.

Example of copying from host to target Fuchsia device:

```none {:.devsite-disable-click-to-copy}
$ fx cp --to-target /path/on/host/source.wav /path/on/target/destination.wav
```

Example of copying from target Fuchsia device to host:

```none {:.devsite-disable-click-to-copy}
$ fx cp --to-host /path/on/target/source.wav /path/on/host/destination.wav
```

Both commands should be run from your host, not the target Fuchsia device.

### Source code {#source}

Source code for `audio-driver-ctl`: [`//src/media/audio/tools/audio-driver-ctl/`][src]

[src]: https://cs.opensource.google/fuchsia/fuchsia/+/main:src/media/audio/tools/audio-driver-ctl/

<!--

// LINT.ThenChange(//src/media/audio/tools/audio-driver-ctl/audio.cc)

-->
