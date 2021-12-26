<!---

# Digital Audio Interface (DAI)

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

--->

# 数字音频接口 （ DAI ）

DAI 接口是 DAI 驱动暴露的 FIDL 接口。 DAI 接口控制音频数据传输的 DAI 连接配置。

## 注释和术语表
- 所有指数都是从0开始。
- 元素向量以`<x0,x1,...,xn-1>`方式来表示，例如一个包含两个元素5和6的向量表示为`<5,6>`。
- 向量可嵌套，例如`<<5,6>,<7,8>>`代表一个向量包含两个向量。

| 术语 | 定义 |
| ---- | ---- |

| DAI          | 数字音频接口。接口位于音频硬件之间，      :
:              : 例如在控制器和编解码器之间的 TDM 和 PDM 链接。    :
| Frame Sync   |  DAI 信号标记了帧的便捷，又称 LRCLK, SYNC。 |
| Sclk         | DAI 信号用于标记数据链传输 bits，  :
:              : 又称 SCK,  BCLK 。                                            :
| Mclk         | 主时钟， 有时需要一个 DAI 信号来为 DAI 提供一个时钟。 |
:              : 有时 Sclk 被用作 Mclk（或者 Mclk 是       :
:              : 在 DAI 中起源于 sclk的）。                        :
| Frame        |   在不同的数据中对单一时间点的表述，   :
:              : 在 DAI 中的帧同步和 sclk 。                               :
| Frame format | 一帧的数据，帧同步和 sclk 约定， |
:              : 例如帧同步的位置，即数据线中的样本。          :
| Slot         | 在一帧中， 样本预留的 bit 位。  |
:              :  一个槽中可能比容纳样品所需的大，例如      :
:              : 一个 32 bit 的槽中保留24或者16 bit 的样本。                                :
| Channel      | 单一源或目的地的音频样本，       |
:              : 通常被一个单扬声器渲染或被单麦克风捕获。     :
:              : 在 DAI 的每一帧都将包含 :
:              : 一个固定数量槽的样本对应相同固定数量的通道                :
| Sample       |  特定时间获取的声音数字表示。   |

<!---

## Basic operation

The DAI client is responsible for configuring the DAI. The driver providing the
DAI interface (from here on the DAI driver) advertises supported formats and
allows the creation of a Ring Buffer for audio sample transferring.

Note that the DAI drivers are expected to perform their own shutdown, just like
any other driver (see [FDF](/docs/concepts/drivers/getting_started.md)).

--->

## 基本操作

 DAI 客户端负责配置 DAI 。驱动提供 DAI 接口（从这里的 DAI 驱动程序），用于发布支持的格式和允许音频样本传输的环形缓冲区创建。

注意， DAI 驱动期望自己实现关机操作，就像任意的其他驱动程序一样。（参见 [FDF](/docs/concepts/drivers/getting_started.md) ）。

<!---

## Protocol definition

The DAI protocol is defined in FIDL at
[dai.fidl](/sdk/fidl/fuchsia.hardware.audio/dai.fidl) and
[dai_format.fidl](/sdk/fidl/fuchsia.hardware.audio/dai_format.fidl).

Because the FDF does not currently provide a way to directly get a FIDL channel
for communication, we define a way to get a channel through
[Banjo](/docs/development/drivers/tutorials/banjo-tutorial.md) at
[fuchsia.hardware.audio](/sdk/banjo/fuchsia.hardware.audio/audio.fidl).

Direct connection to a DAI protocol server is provided by
[dai_connect.fidl](/sdk/fidl/fuchsia.hardware.audio/dai_connect.fidl).

--->

## 协议定义

 DAI 协议定义的 FIDL 的[dai.fidl](/sdk/fidl/fuchsia.hardware.audio/dai.fidl) 和[dai_format.fidl](/sdk/fidl/fuchsia.hardware.audio/dai_format.fidl) 中。

因为 FDF 当前不提供一种直接的方式来获取 FIDL 通道用于通信，所以我们定义了一个在[fuchsia.hardware.audio](/sdk/banjo/fuchsia.hardware.audio/audio.fidl)中的[Banjo](/docs/development/drivers/tutorials/banjo-tutorial.md) 的方式来获取通道。

[dai_connect.fidl](/sdk/fidl/fuchsia.hardware.audio/dai_connect.fidl)提供直接连接 DAI 协议服务器。

<!---

### Reset {#reset}

A DAI can be reset by a client at any time by issuing the `Reset` function.

--->

### 复位{#reset}

 DAI 可以通过一个客户端在任意时刻调用`Reset` 功能进行复位。

<!---

### GetInfo {#get-info}

The `GetInfo` function retrieves information from the DAI including:

1.  The manufacturer name.
1.  The product name.

--->

### 获取信息

`GetInfo`可以从 DAI 中查询获取信息包括：

1. 供应商名字。
2. 产品名。

<!---

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

--->

### 获取 DAI 格式

`GetDaiFormats`函数允许 DAI 驱动列出它支持的 DAI 格式。支持的格式可能包括多采样格式，比率等。 DAI 驱动提供其支持的格式和他们客户端授权，即可以在`CreateRingBuffer`功能中使用的格式。

 DAI 驱动依赖`DaiSupportedFormats`的向量，其中每个`DaiSupportedFormats`包含：

1. 通道数量的向量。列举了 DAI 支持的通道数量，例如 `<2,4,6,8>`。一个立体声 DAI 报告了一个有一个元素的向量 `<2>`。注意占用一个通道并输出内容到所有它的输出器（例如，2个立体声扬声器）的 DAI 可能报告一个有一个元素的向量`<1>`，如果它支持一个或两个输入通道，那么它会报告一个两个元素的向量`<1,2>`。
2. 样本格式的向量。 DAI 样本格式，例如`PCM_SIGNED`。
3. 框架格式的向量。例如`I2S`或者`TDM1`，或者`CUSTOM`选项，其中`DaiFrameFormatCustom`定义了帧单独配置的每一个参数。例如`frame_sync_size` 和`sclk_on_raising`。
4. 采样率向量。单帧采样率，例如44100，48000和96000。
5. 每个槽的比特数。在 DAI 中每个槽中的比特数，例如每个槽32比特。
6. 每个样本的比特向量。采样宽度，例如每个样本24比特。

<!---

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

--->

在单独的`DaiSupportedFormats`中，提供参数的任意组合都是可支持的。

当 DAI 支持的所有组不能用一个`DaiSupportedFormats`来描述时， DAI 在返回向量中返回超过一个`DaiSupportedFormats`。假如一个`DaiSupportedFormats`允许32比特的采样频率为48KHz，和一个16比特的采样频率为96KHz，但不允许32比特的采样频率为96KHz，然后 DAI 将回复两个 `DaiSupportedFormats`: `<<32bits>,<48KHz>>` 和`<<16bits>,<96KHz>>`的结果。为了简单起见，示例将忽略其他参数，包括速率和每个采样位数。在 DAI 支持16或者32比特样本在48或96KHz的场景中， DAI 将回复一个`DaiSupportedFormats`: `<<16bits,32bits>,<48KHz,96KHz>>`作为返回值。

另外，假设每个样本的比特总是小于等于每个槽的比特。因此编解码器可以报告`<<16bits_per_slot,32bits_per_slot>,<16bits_per_sample,32bits_per_sample>>`，并且这并不表明报告的每个样本32比特在16比特的样本上是有效的，它只规定了3种有效的组合：

1. 16比特槽，16比特采样
2. 32比特槽，32比特采样
3. 32比特槽，16比特采样

<!---

### GetRingBufferFormats {#get-ring-buffer-formats}

Same as the streaming interface `GetSupportedFormats` function, see
[Audio Streaming Interface](audio_streaming.md).

--->

### 获取环形缓冲区格式

类似于流媒体接口`GetSupportedFormats` 功能，参见[Audio Streaming Interface](audio_streaming.md)。

<!---

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
[Audio Streaming Interface](audio_streaming.md).

Once `CreateRingBuffer` is successful, the DAI format configuration is considered
completed and samples can be sent across the DAI once the `RingBuffer` protocol
configuration is completed and the `RingBuffer` `Start` function has been called.

For a description of the ring buffer functionality, see the [Audio Streaming
Interface](audio_streaming.md).

--->

### 创建环形缓冲区

客户端同时定义了 DAI 中使用的格式和作为在`CreateRingBuffer`函数中创建的环形缓冲区部分的环形缓冲。

 DAI 格式参数定义：

1. 通道数量。这是在 DAI 中的通道数量（例如在 TDM 总线上的通道数量，也就是“在线”）。
2. 使用位掩码的通道。这是在 DAI 中使用的用于数据转移的通道。例如明确两个通道在 I2S DAI 中必须为3。
3. 采样格式。
4. 帧格式。
5. 帧率
6. 每个槽中的 bits 数量
7. 每个采样的 bits 数量

环形缓冲区和流媒体接口`CreateRingBuffer`的格式参数相同，详情参见[Audio Streaming Interface](audio_streaming.md)。

一旦`CreateRingBuffer`调用成功， DAI 格式配置就被认为是完成；一旦`RingBuffer`协议配置完成， `RingBuffer` 的`Start`函数被调用，采样结果可以通过 DAI 发送。

对于环形缓冲区功能的介绍，详情参见[Audio Streaming Interface](audio_streaming.md)。

## Power Control {#power-control}

TODO(fxbug.dev/63522).

## Content Protection {#content-protection}

TODO(fxbug.dev/63523).

