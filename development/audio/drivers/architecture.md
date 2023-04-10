# Audio Drivers Architecture

Caution: This page may contain information that is specific to the legacy
version of the driver framework (DFv1).

In Fuchsia there are many ways drivers can be architected as defined by the
number of drivers used, how they communicate and their responsibilities. Audio
drivers responsibilities are determined by the interface(s) exposed to driver
clients, clients could be other drivers or applications users of the drivers
facilities.

## Definitions

| Term                 | Definition                                           |
| -------------------- | ---------------------------------------------------- |
| Codec                | A real or virtual device that encodes/decodes a      |
:                      : signal from digital/analog to/from analog/digital    :
:                      : including all combinations, e.g. digital to digital. :
:                      : Example codecs include DAC-Amplifiers combos and ADC :
:                      : converters.                                          :
| Controller or engine | The HW part of a system that manages the audio       |
:                      : signals, for example an SOC's audio subsystem.       :
| DAI                  | Digital Audio Interface. Interface between audio HW, |
:                      : for instance a TDM or PDM link between controllers   :
:                      : and codecs.                                          :

# Audio interfaces

The main API in use by applications is the
[Audio Streaming Interface](streaming.md). This API allows applications to
capture or render audio. Examples of audio applications using audio drivers with
the streaming interface include
[audio_core](/src/media/audio/audio_core/v1/README.md) and
[audio-driver-ctl](/src/media/audio/tools/audio-driver-ctl). The former is the
core of the audio system (providing software mixing, routing, etc.) and the
latter is a utility used for testing and bringup of new platforms.

A driver providing the streaming interface abstracts the HW functionality
described in the API, but it does not need to be the driver actually configuring
all the HW. A common split in audio HW is to have an audio engine that
configures a DAI communicating with an audio HW codec. In this split we can have
one driver for the audio engine and one for the codec. The
[Codec Interface](codec.md) allows having a driver for the codec implementation
and another driver configuring the audio engine HW including the DAI and driving
the codec(s) configuration. In this configuration the codec(s) are secondary to
the controller. For instance the mt8167-tdm-output was a driver for the MediaTek
MT8167S audio subsystem (audio engine) also providing the streaming interface
for applications and communicating with any codec driver, for example a
[tas58xx](/src/media/audio/drivers/codecs/tas58xx) exposing the codec interface
as shown below:

```
                           +-----------------+
                           |   audio_core    |
                           +-----------------+
                                    |
                           Streaming Interface
                                    |
                           +-----------------+
                           |mt8167-tdm-output|
                           +-----------------+
                                    |
                             Codec Interface
                                    |
                           +-----------------+
                           |     tas58xx     |
                           +-----------------+
```

Another way to architect drivers with the engine/codec split is to have a codec
providing the streaming audio interface, and the audio engine providing a
[DAI interface](dai.md). For example a
[aml-g12-tdm-dai](/src/media/audio/drivers/aml-g12-tdm/dai.cc) driver for the
AMLogic g12 audio subsystem (audio engine) providing the DAI interface for other
drivers or applications to use, and a codec can drive the engine and provide the
streaming interface to applications like audio-driver-ctl.

```
                           +-----------------+
                           |audio-driver-ctl |
                           +-----------------+
                                    |
                           Streaming Interface
                                    |
                           +-----------------+
                           |  codec-driver   |
                           +-----------------+
                                    |
                              DAI Interface
                                    |
                           +-----------------+
                           | aml-g12-tdm-dai |
                           +-----------------+
```

We can also have a non-driver component use either DAI or codec interface to
access an audio engine or audio HW codec directly. In the figure below we have
Bluetooth Audio providing the streaming interface and making use of the DAI
interface to configure the AMLogic g12 audio subsystem:

```
         +-----------------+    Streaming    +-----------------+
         |    BT Stack     +-----------------+   audio_core    |
         +-----------------+    Interface    +-----------------+
                  |
             DAI Interface
                  |
         +-----------------+
         | aml-g12-tdm-dai |
         +-----------------+
```

It is also possible to have both the DAI and codec interfaces in use, for
example tied with another driver providing the streaming interface. An example
usage of this architecture for a system with two different codecs physically
connected to the same DAI stream.

```
                           +-----------------+
                           |   audio_core    |
                           +-----------------+
                                    |
                           Streaming Interface
                                    |
                           +-----------------+
                +----------+   glue driver   +----------+
                |          +-----------------+          |
                |                   |                   |
          DAI Interface      Codec Interface     Codec Interface
                |                   |                   |
       +-----------------+ +-----------------+ +-----------------+
       | aml-g12-tdm-dai | |    tas-5720     | |    tas-5720     |
       +-----------------+ +-----------------+ +-----------------+
```

Finally, it is also possible to just have one driver configuring all HW and
exposing the streaming interface. This should be used when there is no logical
DAI or codec HW separation:

```
                           +-----------------+
                           |   audio_core    |
                           +-----------------+
                                    |
                           Streaming Interface
                                    |
                           +-----------------+
                           |USB audio driver |
                           +-----------------+
```
