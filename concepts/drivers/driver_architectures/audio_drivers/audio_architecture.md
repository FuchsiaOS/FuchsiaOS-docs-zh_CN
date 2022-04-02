<!---

# Audio Drivers Architecture

In Fuchsia there are many ways drivers can be architected as defined by the
number of drivers used, how they communicate and their responsibilities. Audio
drivers responsibilities are determined by the interface(s) exposed to driver
clients, clients could be other drivers or applications users of the drivers
facilities.

--->

# 音频驱动架构

在 Fuchsia 中，有很多方法可以构建驱动程序，这是由使用的驱动数量，它们的通信方式和它们的职责来定义。音频驱动的职责就是由暴露给驱动客户端的接口决定，客户端可以是其他驱动或者驱动的应用程序用户。

<!---

## Definitions

| Term                 | Definition                                            |
| ---------------------| ------------------------------------------------------|
| Codec                | A real or virtual device that encodes/decodes a signal|
:                      : from digital/analog to/from analog/digital including  :
:                      : all combinations, e.g. digital to digital. Example    :
:                      : codecs include DAC-Amplifiers combos and ADC          :
:                      : converters.                                           :
| Controller or engine | The HW part of a system that manages the audio        |
:                      : signals, for example an SOC's audio subsystem.        :
| DAI                  | Digital Audio Interface. Interface between audio HW,  |
:                      : for instance a TDM or PDM link between controllers    :
:                      : and codecs.                                           :

--->

## 定义

## 

| 术语  | 定义                                                         |
| ----- | ------------------------------------------------------------ |
| Codec | 一个真实或者虚拟的设备对数字/模拟信号进行编码/解码，包括所有组合 |

:                      : 例如，数字到数字。
:                      : 示例 解码包含 DAC 放大器组合和 ADC 转换器。
| Controller or engine | 系统管理音频信号的硬件部分       |
:                      : 例如一个 SOC 的音频子系统。     :
| DAI                  |  Digital Audio Interface 。在音频硬件之间的接口  |
:                      : 例如在控制器间的 TDM 或者 PDM 连接和编解码:
:                      :                                        :

<!---

# Audio interfaces

The main API in use by applications is the [Audio Streaming
Interface](audio_streaming.md). This API allows applications to capture or
render audio. Examples of audio applications using audio drivers with the
streaming interface include [audio_core](/src/media/audio/audio_core/README.md)
and [audio-driver-ctl](/src/media/audio/tools/audio-driver-ctl). The former is
the core of the audio system (providing software mixing, routing, etc.) and the
latter is a utility used for testing and bringup of new platforms.

A driver providing the streaming interface abstracts the HW functionality
described in the API, but it does not need to be the driver actually configuring
all the HW. A common split in audio HW is to have an audio engine that
configures a DAI communicating with an audio HW codec. In this split we can have
one driver for the audio engine and one for the codec. The [Codec
Interface](audio_codec.md) allows having a driver for the codec implementation
and another driver configuring the audio engine HW including the DAI and driving
the codec(s) configuration. In this configuration the codec(s) are secondary to
the controller. For instance the
[mt8167-tdm-output](/src/media/audio/drivers/mt8167-tdm-output) is a driver for
the MediaTek MT8167S audio subsystem (audio engine) also providing the streaming
interface for applications and communicating with any codec driver, for example
a [tas58xx](/src/media/audio/drivers/codecs/tas58xx) exposing the codec
interface as shown below:

--->

# 音频接口

应用使用的主要 API 在 [Audio Streaming Interface](audio_streaming.md)中说明。 这个 API 允许应用捕获或者渲染音频。使用音频驱动的流接口的应用示例包括 [audio_core](/src/media/audio/audio_core/README.md)和[audio-driver-ctl](/src/media/audio/tools/audio-driver-ctl)。前者为音频系统的核心（提供软件混合，路由等），后者为测试和启动一个新平台的实例。

提供流媒体接口的驱动抽象了 API 中描述的硬件功能，但是它不需要成为实际配置所有硬件的驱动。在音频硬件中通用的分割方式为，一个音频引擎来配置 DAI 与音频硬件编解码器通信。[Codec Interface](audio_codec.md) 允许有一个编解码器实现的驱动，和其他驱动配置包括 DAI 和编解码器(s)配置的音频引擎硬件。在这个配置中，解码器(s)是控制器的次级部分。例如[mt8167-tdm-output](/src/media/audio/drivers/mt8167-tdm-output) 是一个 MediaTek MT8167S 音频子系统（音频引擎）的驱动，同时也提供了对于应用程序和与任意编解码器驱动通信的流媒体接口，例如[tas58xx](/src/media/audio/drivers/codecs/tas58xx) 就暴露了编解码器接口如下所示：

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

<!---

Another way to architect drivers with the engine/codec split is to have a codec
providing the streaming audio interface, and the audio engine providing a
[DAI interface](audio_dai.md). For example a
[aml-g12-tdm-dai](/src/media/audio/drivers/aml-g12-tdm/dai.cc) driver for the
AMLogic g12 audio subsystem (audio engine) providing the DAI interface for other
drivers or applications to use, and a codec can drive the engine and provide
the streaming interface to applications like audio-driver-ctl.

--->

引擎/编解码分离的架构驱动的另一种方式则是有一个编解码器来提供流媒体接口，并且音频引擎提供一个[DAI interface](audio_dai.md)。例如对于 AMLogic g12 音频子系统（音频引擎）的 [aml-g12-tdm-dai](/src/media/audio/drivers/aml-g12-tdm/dai.cc) 驱动提供 DAI 接口来供其他驱动或应用使用，并且编解码器可以驱动引擎，并对类似 audio-driver-ctl 的应用提供流媒体接口。

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

<!---

We can also have a non-driver component use either DAI or codec interface to
access an audio engine or audio HW codec directly. In the figure below we have
Bluetooth Audio providing the streaming interface and making use of the DAI
interface to configure the AMLogic g12 audio subsystem:

--->

我们同样可以有一个非驱动组件使用 DAI 或者编解码器接口来直接访问音频引擎或者音频硬件编解码器。在下图中，我们有一个下提供流式接口的蓝牙音频，和使用 DAI 接口来配置 AMLogic g12 音频子系统：

             +-----------------+    Streaming    +-----------------+
             |    BT Stack     +-----------------+   audio_core    |
             +-----------------+    Interface    +-----------------+
                      |
                 DAI Interface
                      |
             +-----------------+
             | aml-g12-tdm-dai |
             +-----------------+

<!---

It is also possible to have both the DAI and codec interfaces in use, for
example tied with another driver providing the streaming interface. An example
usage of this architecture for a system with two different codecs physically
connected to the same DAI stream.

--->

同样也可以同时使用 DAI 和解码接口，例如与另一个提供流媒体接口的驱动程序捆绑在一起。这种架构的使用示例是，在一个系统中，有两个不同编解码器物理连接的同一个 DAI 流。

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

<!---

Finally, it is also possible to just have one driver configuring all HW and exposing the
streaming interface. This should be used when there is no logical DAI or codec HW
separation:

--->

最后，还有一种可能是仅有一个驱动配置所有硬件和暴露的流媒体接口。当没有逻辑 DAI 或单独的硬件编解码器时，则使用这种方式。

                               +-----------------+
                               |   audio_core    |
                               +-----------------+
                                        |
                               Streaming Interface
                                        |
                               +-----------------+
                               |USB audio driver |
                               +-----------------+
