<!---

# Audio Codec Interface

The codec interface is meant to be used when the codecs are secondary to a
controller driver. This interface is a FIDL protocol exposed by codec drivers.
In this arrangement the codec drivers are not directly exposing a streaming
interface, and they are configured through the codec interface by a controller.
This is a reference for driver-authors, defining the interface contract that
codec drivers must implement and that controllers can use.

--->

# 音频编解码接口

解码接口是为了在解码器作为控制器驱动的次要部分来使用。这个接口由解码器驱动程序暴露的一个 FIDL 协议。在这种安排下，编解码器驱动是不直接暴露一个流媒体接口，而是通过控制器的编解码器接口来进行配置。

这是驱动编写者的参考，定义了编解码器驱动必须实现，和控制器可使用的抽象接口。

<!---

## Notation and Terminology

In this document:

-   All indices start from 0.
-   Vectors of n elements are represented as `<x0,x1,...,xn-1>`, for example a
    vector with two elements 5 and 6 as `<5,6>`.
-   Vectors can be nested, i.e. `<<5,6>,<7,8>>` represents a vector with 2
    vectors in it.

--->

## 注释和术语表

在本文档中：
-	所有指数都从0开始
-	n个元素的矢量被表示为`<x0,x1,...,xn-1>`，例如一个矢量包含两个5和6的元素，则表示为`<5,6>`。
-	矢量可以嵌套，例如`<<5,6>,<7,8>>` 表示一个矢量中包含两个矢量。

<!---


| Term         | Definition                                                     |
| ----------   | -------------------------------------------------------------- |
| Codec        | A real or virtual device that encodes/decodes a signal from    |
:              : digital/analog to/from analog/digital including all            :
:              : combinations, e.g. digital to digital. Example codecs include  :
:              : DAC-Amplifiers combos and ADC converters.                      :
| Controller   | The part of a system that manages the audio signals, for       |
:              : example an SOC's audio subsystem or an independent sound card. :
| DAI          | Digital Audio Interface. Interface between audio HW, for       :
:              : instance a TDM or PDM link between controllers and codecs.     :
| Frame Sync   | A DAI signal that marks frame boundaries, a.k.a. LRCLK, SYNC.  |
| Sclk         | A DAI signal used to mark the data line(s) bits transferring,  :
:              : a.k.a. SCK, BCLK.                                              :
| Mclk         | Master clock, a DAI signal sometimes needed to provide a clock |
:              : to codecs. Sometimes Sclk is used as the Mclk (or Mclk is      :
:              : derived from the Sclk within the codec).                       :
| Frame        | The representation of a single moment in time across data,     :
:              : frame sync and sclk in the DAI.                                :
| Frame format | A frame's data, frame sync and sclk arrangement, e.g. location |
:              : of the frame sync w.r.t. samples in the data line(s).          :
| Slot         | Within a frame, the bits reserved for a sample. A slot may be  |
:              : bigger than needed to hold the samples, e.g. 32 bits slot      :
:              : holding 24 or 16 bits samples.                                 :
| Channel      | A single source or destination of audio samples, usually       |
:              : to be rendered by a single speaker or captured by a single     :
:              : microphone. Within a DAI every frame will contain samples in   :
:              : a fixed number of slots for the same fixed number of channels. :
| Sample       | A digital representation of sound taken at a particular time.  |

--->




| 术语  | 定义                       |
| ----- | -------------------------- |
| Codec | 一个真实或虚拟设备来实现从 |

:              : 数字/模拟信号到/来自模拟/数字信号包含所有组合方式的编码/解码。            :
:              : 例如数字信号到数字信号。示例代码包含  :
:              :  DAC-Amplifiers 结合和 ADC 转换器。                      :
| Controller   | 系统管理音频信号的部分，例如       |
:              : 一个 SOC 的音频子系统或者一个独立的声卡。 :
| DAI          | 数字音频接口。接口位于音频硬件中间，       :
:              : 例如一个 TDM 或者 PDM 连接在控制器和编解码器中间。   :
| Frame Sync   | 一个 DAI 信号标记结构边界，又叫做 LRCLK， SYNC。 |
| Sclk         | 一个 DAI 信号用于标记传输的数据线位，  :
:              : 又叫做 SCK , BCLK                                            :
| Mclk         | 主时钟，一个 DAI 信号有时需要提供一个时钟来编解码。 |
:              : 有时 Sclk 被用于作为 Mclk      :
:              : （或者 Mclk 是在编解码器中来源于 Sclk ）。                     :
| Frame        | 在 DAI 中跨越数据，帧同步和     :
:              :     sclk 的单一时间时刻的表示。                          :
| Frame format | 一帧的数据，帧同步和 sclk 安排，例如  |
:              : 帧同步的位置，关于数据线中的样本。       :
| Slot         | 在一帧中，保留给采样的比特位。一个槽 |
:              : 一般大于需要保留的采样。例如，32 bits 的槽    :
:              : 控制24或者16 bits 的采样                               :
| Channel      | 单一音频采样的源或目的地，     |
:              : 通常由一个扬声器呈现，或一个麦克风捕捉，   :
:              : 在 DAI 的每一帧都将包含   :
:              : 一个固定数量槽的采样对应相同固定数量的通道。 :
| Sample       | 特定时间获取的声音数字表示。  |

--->

<!---

## Basic Operation

The functionality provided by the codecs is divided into:

-   [Main controls](#main-controls)
-   [Bridged Mode](#bridged-mode)
-   [DAI format](#dai-format)
-   [Gain control](#gain-control)
-   [Plug detect](#plug-detect)
-   [Power control](#power-control)
-   [Peripheral control](#peripheral-control)
-   [Signal processing control](#signal-processing-control)
-   [Content protection](#content-protection)

The controller is responsible for configuring and controlling the codecs. Codecs
advertize capabilities and a controller determines how they are used as
described below. The controller can control the codec's state, such as through the
reset function. A reset is required to get codecs to an initialized state.
Note that the codec drivers are expected to perform their own shutdown, just like
any other driver.

Codecs are composite devices that provide the codec protocol to controllers. It
is expected that only one controller uses a codec's protocol, and one controller
may use multiple codecs at once.

The [simple-codec](/src/media/audio/lib/simple-codec/README.md) library facilitates
writing and using simple codec drivers implementing the codec protocol.

--->
## 基础操作

解码器提供的功能被拆分为以下部分：

-   [主要控制](#main-controls)
-   [桥接模式](#bridged-mode)
-   [DAI 格式](#dai-format)
-   [增益控制](#gain-control)
-   [插件检测](#plug-detect)
-   [电源控制](#power-control)
-   [周边件控制](#peripheral-control)
-   [信号处理控制](#signal-processing-control)
-   [内容保护](#content-protection)

控制器负责配置和控制编解码器。编解码器展示自己能力，控制器则决定如何如下所述方式来使用它们。控制器可以控制编解码器的状态，例如通过重置功能。当需要编解码器变成初始状态时，则需要重置功能。

注意，编解码器驱动将执行它们自己的关机操作，就像其他的驱动一样。

编解码器是一个复合设备，来提供控制器编解码协议。预计只有一个控制器使用一个编解码器协议，并且一个控制器可以同时使用多个编解码器。

 [simple-codec](/src/media/audio/lib/simple-codec/README.md) 库便于写和使用简单的编解码器驱动实现的解码器协议。

<!---

## Protocol definition

The codec protocol is defined in FIDL at
[codec.fidl](/sdk/fidl/fuchsia.hardware.audio/codec.fidl).

Note that because the FDF does not currently provide a way to directly get a FIDL
channel for communication, we define a way to get a channel through
[Banjo](/docs/development/drivers/tutorials/banjo-tutorial.md) at
[fuchsia.hardware.audio](/sdk/banjo/fuchsia.hardware.audio/audio.fidl).

Many codec protocol operations are "fire-and-forget", i.e. they do not expect a
reply. Codec protocol operations with a reply are not considered completed until
the reply of the function is received, and not considered completed successfully
unless the reply contains a status `ZX_OK`.

--->
## 协议定义
解码器协议的 FIDL 定义在[codec.fidl](/sdk/fidl/fuchsia.hardware.audio/codec.fidl)中。

注意，因为 FDF 现在不提供直接获取 FIDL 通信通道的方式，所以我们定义了一种在 [fuchsia.hardware.audio](/sdk/banjo/fuchsia.hardware.audio/audio.fidl) 通过 [Banjo](/docs/development/drivers/tutorials/banjo-tutorial.md) 来获取通道的方式。

许多编编码器协议操作都是”发后即忘“的，例如它们并不期望回复。有回复的编解码器协议操作在收到函数回复之前都不被视为完成，并且除非回复包含状态`ZX_OK`，否则也认为没有成功完成。

<!---


### Main Controls {#main-controls}

A codec can be reset by a controller at any time by issuing the `Reset`
function.

The `GetInfo` function retrieves information from the codec including:

1.  A unique and persistent identifier for the codec unit, e.g. a serial number
    or connection path.
1.  The manufacturer name.
1.  The product name.

The codec operation can be started and stopped at any time with the `Start` and
`Stop` functions. By default the codec state is stopped, so 'Start' must be
issued at least once for the codec to be fully operational. Stopping the codec
operation can be used for example to change the DAI configuration safely avoiding
glitches or errors in the codec operation.

--->
### 主要控制{#main-controls}

编解码器可以通过控制器调用 `Reset`功能在任意时刻复位。

`GetInfo` 功能可以从解码器中查询包括以下的信息：

1. 一个独特并持久的编解码器单元表示，例如一个序列号或者连接路径。
2. 制造商名字。
3. 产品名。

编解码器操作可以通过`Start`和`Stop`功能在任意时间开始或结束。因为编解码器的默认状态为停止，所以对于编解码器的完整操作，“开始”状态必须至少发布一次。停止编解码器操作可以用于例如在安全切换 DAI 配置，避免在编解码器操作中的故障或错误。

<!---

### Bridged Mode {#bridged-mode}

Before specifying the DAI format the controller must query the codec for its
bridging capabilites. If the codec is bridgeable, then the controller must
enable or disable bridging based on its knowledge of the system configuration.
Note that this is a singular property of a codec, i.e. a codec either supports
bridging or not, and it can be set in bridged mode or not. This protocol allows
configuring as bridged only 2 channel stereo codecs, with the 2 outputs of the
codec electrically bridged.

--->

### 桥接模式{#bridged-mode}

在指定 DAI 格式之前，控制器必须查询编解码器获取它的桥接能力。如果编解码器是可架桥的，那么控制器必须基于它获取的系统配置来使能或禁止桥接。

注意，这是一个编解码器的独特属性，例如，一个编解码器要么支持桥接，要么不支持，并且它支持可以或不可以被设置为桥接模式。这个协议只允许配置为桥接的两通道立体声解码器，采用编解码器两路输出电器桥接。

<!---


### DAI Format {#dai-format}

The DAI Format related protocol functions allow the codec to list its supported
formats for the DAI. The supported formats may include multiple sample formats,
rates, etc. Each codec advertises what it can support and the controller
mandates what DAI Format is to be used for each codec.

To find out what formats are supported by a given codec, the controller uses the
`GetDaiFormats` function. The codec replies with a vector of
`DaiSupportedFormats`, where each `DaiSupportedFormats` includes:

1.  A vector of number of channels. This lists the number of channels supported
    by the codec, for example `<2,4,6,8>`. A stereo codec reports a vector with
    one element `<2>`. Note that a codec that takes one channel and outputs its
    contents in all its outputs (e.g. 2 for a stereo amplifier) would report a
    vector with one element `<1>`, if it supports either one or two input
    channels, it would report a vector with two elements `<1,2>`.
2.  A vector of sample formats. DAI sample formats, e.g. `PCM_SIGNED`.
3.  A vector of frame formats, for example `STEREO_LEFT` and `STEREO_RIGHT`.
4.  A vector of rates. Frame rates, for example 44100, 48000, and 96000.
5.  A number of bits per slot. Number of bits in each slot in the DAI,
    e.g. 32 bits per slot.
6.  A vector of bits per sample. Sample widths, e.g. 24 bits per sample.

--->

### DAI 格式

相关协议功能的 DAI 格式允许编解码器列举它支持的 DAI 格式。支持的格式可能包括多种采样格式、采样率等。每一个编解码器展示它可以支持什么内容和控制器授权每个编解码器需要使用什么 DAI 格式。

为了找到给编定解码器支持什么格式，控制器使用`GetDaiFormats`功能。编解码器返回一组`DaiSupportedFormats` ，其中每个`DaiSupportedFormats`包含：

1. 通道数量的向量。列举了编解码器支持的通道数量，例如 `<2,4,6,8>`。一个立体声编解码器上报一个只有一个元素的向量 `<2>`。注意占用一个通道并输出内容到所有它的输出端（例如，2个立体声扬声器）的编解码器可能报告一个只有一个元素的向量`<1>`，如果它支持一个或两个输入通道，那么它会报告一个两个元素的向量`<1,2>`。
2. 采样格式的向量。 DAI 采样格式，例如`PCM_SIGNED`。
3. 单帧格式的向量。例如`STEREO_LEFT`和 `STEREO_RIGHT`。
4. 采样率的向量。单帧采样率，例如44100，48000和96000。
5. 每个槽的比特数。在 DAI 中每个槽中的比特数，例如每个槽32比特。
6. 每个样本的比特向量。采样宽度，例如每个采样24比特。

<!---

When not all combinations supported by the codec can be described with one
`DaiSupportedFormats`, the codec returns more than one `DaiSupportedFormats` in
the returned vector.

For example, if one `DaiSupportedFormats` allows for 32 bits samples at 48KHz,
and 16 bits samples at 96KHz, but not 32 bits samples at 96KHz, then the codec
will reply with 2 `DaiSupportedFormats`: `<<32bits>,<48KHz>>` and
`<<16bits>,<96KHz>>`. For simplicity, this example ignores parameters other than
rate and bits per sample. In the case where the codec supports either 16 or 32
bits samples at either 48 or 96KHz, the codec would reply with 1
`DaiSupportedFormats`: `<<16bits,32bits>,<48KHz,96KHz>>`.

Additionally, it is assumed that bits per sample is always smaller or equal to
bits per slot. Hence, a codec can report
`<<16bits_per_slot,32bits_per_slot>,<16bits_per_sample,32bits_per_sample>>`
and this does not imply that it is reporting that 32 bits per sample on 16 bits
samples is valid, it specifies only the 3 valid combinations:

1.  16 bits slot with 16 bits samples
2.  32 bits slot with 32 bits samples
3.  32 bits slot with 16 bits samples

--->

当不是所有组合都能被编解码器支持时，可以使用`DaiSupportedFormats`来描述，编解码器在其返回向量中返回超过一个`DaiSupportedFormats`。

假如一个`DaiSupportedFormats`允许32比特的采样频率为48KHz，和一个16比特的采样频率为96KHz，但不允许32比特的采样频率为96KHz，然后编解码器将回复两个 `DaiSupportedFormats`: `<<32bits>,<48KHz>>` 和`<<16bits>,<96KHz>>`的结果。为了简单起见，这个示例忽略其他参数，例如包括速率和每个样本的位数。在编解码器支持16或者32比特采样在48或96KHz的场景中，编解码器将回复一个`DaiSupportedFormats`: `<<16bits,32bits>,<48KHz,96KHz>>`作为返回值。

另外，假设每个采样的比特数总是小于等于每个槽的比特。因此编解码器可以报告`<<16bits_per_slot,32bits_per_slot>,<16bits_per_sample,32bits_per_sample>>`，并且这并不表明报告的每个采样32比特在16比特的样本上是有效的，它只规定了3种有效的组合：

1. 16比特槽，16比特采样
2. 32比特槽，32比特采样
3. 32比特槽，16比特采样

<!---

Using the information provided by the codec in `IsBridgeable` and
`GetDaiFormat`, what is supported by the controller, and any other requirements,
the controller specifies the format to use in the DAI with the `SetDaiFormat`
function. This functions takes a parameter that specifies:

1.  A number of channels. This is the number of channels to be used in the DAI
    (for instance number of channels on a TDM bus, i.e. "on the wire"). For I2S
    this must be 2.
2.  A vector of channels to use. These are the channels in the DAI to be used by
    the codec. For I2S this must be a vector with 2 indexes `<0,1>`, i.e. both
    left and right channels are used. In bridged mode this will list only the
    one channel to be used by the codec, for example a codec’s stereo amplifier
    output bridged into one electrical mono output from the right channel of an
    I2S DAI would list only channel `<1>`. If not bridged, a codec with multiple
    electrical outputs that is configured with one channel in `SetDaiFormat` is
    expected to replicate the samples in this mono input on all its outputs.
3.  A sample format.
4.  A frame format.
5.  A frame rate.
6.  A number of bits per slot.
7.  A number of bits per sample.

Once `SetDaiFormat` is successful, the DAI format configuration is considered
completed and samples can be sent across the DAI.

TODO(andresoportus): Add DAI format loss notification support once asynchronous
notifications are added to Banjo.

--->

使用编解码器提供的在`IsBridgeable`和`GetDaiFormat`中的信息，可以知道控制器支持的是什么，和任何其他的需求，控制器可以通过`SetDaiFormat`功能指定在 DAI 中使用的格式。这个函数接受一个参数，指定以下内容：

1. 通道数量。这是在 DAI 中使用的通道数量（例如在 TDM 总线上的通道数量。例如 ” 在线“ ）。对于 I2S 来说，通道数量必须为2。
2. 使用的通道向量。这是在 DAI 中编解码器使用的通道。对于 I2S 来说，这必须是有带有两个参数的向量`<0,1>`。例如，左右通道都使用。在桥接模式下，它将列出编解码器仅使用的一个通道，例如一个编解码器的立体声扬声器，从 I2S  DAI 的右通道桥接到一个电器单声道输出，将仅列举通道`<1>` 。如果不桥接的话，多电气通道输出的编解码器，在 `SetDaiFormat`中配置为一个通道，将复制这个单通道输入的样本到所有的输出上。
3. 采样格式。
4. 单帧格式。
5. 帧率。
6. 每个槽的比特数。
7. 每个采样率数。

当`SetDaiFormat`成功执行， DAI 格式配置就被认为完成，，并且采样通过 DAI 发送。

待完成项（andresoportus）：一旦异步通知被添加到 Banjo 中，添加 DAI 格式损失通知支持。

<!---

### Gain Control {#gain-control}

Gain related support by any given codec is returned by the codec in response to
a `GetGainFormat` function in the `GainFormat` structure. The controller can
control gain, mute and AGC states in a codec using the `SetGainState` function.

Clients may request that codecs send them asynchronous notifications of
gain state changes by using the `WatchGainState` command. The driver will reply to the
first |WatchGainState| sent by the client and will not respond to subsequent
client |WatchGainState| calls until the gain state changes from what was most recently
reported.

--->

### 增益控制

对于给定编解码器支持的相关增益通过`GetGainFormat`函数回复的`GainFormat` 结构体返回。控制器可以使用`SetGainState` 函数控制在编解码器中的增益，静音和 AGC 状态。

客户端可以通过使用`WatchGainState` 命令来请求编解码器发送增益状态变化的异步通知。驱动将回复客户端的第一个|WatchGainState| 发送，将等到增益状态与最近报告相比发生变化，才回复后续的客户端|WatchGainState| 请求。

<!---

### Plug Detect {#plug-detect}

Clients may request that codecs send them asynchronous notifications of
plug state changes by using the `WatchPlugState` command if the `CAN_ASYNC_NOTIFY`
flag was sent by the driver in `GetPlugDetectCapabilites`. Drivers for codecs that
do not set the `CAN_ASYNC_NOTIFY` flag are free to ignore the `WatchPlugState` sent
by clients. Drivers with `CAN_ASYNC_NOTIFY` set will reply to the first
|WatchPlugState| sent by the client and will not respond to subsequent client
|WatchPlugState| calls until the plug state changes from what was most recently reported.

--->

### 插件检测

如果驱动使用`GetPlugDetectCapabilites`发送`CAN_ASYNC_NOTIFY`标志，那么客户端可以通过`WatchPlugState`指令请求编解码器发送插件状态变化的异步通知。编解码器驱动没有设置 `CAN_ASYNC_NOTIFY` 标志，则可以自由地忽略客户端发送的`WatchPlugState`请求 。设置`CAN_ASYNC_NOTIFY` 了的驱动将回复第一个客户端发送的|WatchPlugState|，将等到插件状态与最近的报告发生变化，才回复后续客户端的|WatchPlugState|请求，。

### Power Control {#power-control}

TODO(fxbug.dev/63522).

### Peripheral Control {#peripheral-control}

TODO(fxbug.dev/64878).

### Signal Processing Control {#signal-processing-control}

TODO(fxbug.dev/64877).

### Content Protection {#content-protection}

TODO(fxbug.dev/63523).
