<!---

# Audio Driver Streaming Interface

This document describes the audio streaming interface exposed by audio drivers
in Fuchsia. It is meant to serve as a reference for both users and
driver-authors, and to unambiguously define the interface contract that drivers
must implement and users must follow.

--->

# 音频驱动流媒体接口

本文档中描述了由 Fuchsia 系统中音频驱动暴露的音频流媒体接口。本文档的目的在于作为使用者和驱动作者的参考，清楚地定义了驱动必须实现和用户必须遵守的驱动协议。

<!---

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

--->

## 概述

音频流是设备节点通过驱动服务发布，意图在 Fuchsia 设备中，供应用程序来捕获或渲染音频。

系统中的每一个音频流（输入或输出）代表了一个数字音频信息流，可能是设备接收或传输的信息。音频流为动态的，并且可以在系统任意时间创建或销毁。在本文中并没有讨论哪些流在任何给定时间点存在，以及什么控制他们的生命周期，这些被认为是音频策略和编解码器管理的问题。另外音频输出流中的信息是应用所有者独占的。音频混合不是音频流接口提供的服务。

<!---

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

--->

### 定义

| 术语   | 定义                     |
| ------ | ------------------------ |
| Sample | 单扬声器渲染的音频代表， |

:                               : 或者由单麦克风捕获      :
:                               :   在单一的时间点上。   :
| LPCM                          |  线性脉动编码调制。 |
:                               : 具体表现的音频采样   :
:                               : 所有在 Fuchsia 中未压缩的音频流。 :
:                               : LPCM 音频样本是     :
:                               : 音频信号在某一瞬间 :
:                               : 的振幅的代表，编码音频      :
:                               : 的数值是线性     :
:                               : 分布在采集 :
:                               : 设备上的 :
:                               : 这与 A-law和 &mu;-law 编码不同,      :
:                               : 是从数值到振幅水平的    :
:                               : 非线性映射。                             :
| Channel                       | 在音频流中，信息的子集        |
:                               :    是由单扬声器渲染，    :
:                               : 或者是由流中的   :
:                               : 单个麦克风捕获。             :
| Frame                         |一组在单一时间内  |
:                               : 捕获/渲染的音频流 :
:                               : 的每个通道的音频采样。                           :
| Frame Rate                    | 又称“采样比率”。 产生或消耗    |
:                               : 的音频比率（单位Hz）。 :
:                               : 常见的采样率包括：44.1 KHz, 48   :
:                               : KHz, 96 KHz等                    :
| Client or User or Application | 这些术语在本文档中可以 |
:                               : 互换使用，它们指使用这些接口    :
:                               : 与音频驱动/设备进行通信的      :
:                               : 软件。                         :

{% comment %}

> 待办项：我们需要扩展接口来支持非线性音频流编码吗？这可能对分发 &mu;-law 编码样本的面向电话的传声器很重要。
>
> {% endcomment %}

<!---

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

--->

### 基本操作

与音频流设备的通信是通过 [通道 ](/docs/reference/kernel_objects/channel.md)发送的消息进行的。应用程序为一个音频流打开设备节点，并且通过 FIDL 请求获取通道。当获取到通道后，设备节点可能会关闭。后续所有与流的通信都通过通道完成。

流通道用于大多数命令和控制任务，包括以下：

* 能力问询
* 格式协调
* 硬件增益控制
* 确定板外延迟
* 插入检测通知
* 访问控制能力检测和信号传递

{% comment %}

> 待办项：插入/拔出检测完成应该通过在通道中发送通知完成（当前做法），还是通过发布/撤销设备节点（并且在撤销通道的场景下，关闭所有通道）？
>
> {% endcomment %}

<!---

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

--->

为了真正达到在音频流上发送或接受音频信息，首先必须设置特定的格式。`CreateRingBuffer`操作的成功回复将包含一个新的“环形缓存区”通道。环形缓存区通道可用于从流中请求一个共享缓存（以 [VMO](/docs/reference/kernel_objects/vm_object.md)中的格式分发），共享缓存可以映射到应用的地址空间，并且按照预设用于发送和接受音频数据。

通常来说，在环形缓存通道上进行的操作包括：

* 请求共享缓存
* 开始和停止流媒体播放和捕捉
* 接收播放和捕捉进程的通知
* 在基于不同晶振支持的 [绝对单调时钟](/docs/reference/syscalls/clock_get_monotonic.md)作为音频输出时钟的情况下，接收时钟恢复信息。

<!---

## Operational Details

### Device nodes

Audio stream device nodes must be published by drivers using the protocol
preprocessor symbol given in the table below. This will cause stream device
nodes to be published in the locations given in the table. Applications can
monitor these directories in order to discover new streams as they are published
by the drivers.

--->

## 操作细节

### 设备节点

音频流设备节点必须通过驱动程序使用下表中给出的协议预处理符号发布。这样做可以让流设备节点在表中给定的位置进行发布。应用可以监控这些目录来发现新的驱动发布的流设备节点。

Stream Type | Protocol                   | Location
----------- | -------------------------- | -----------------------
Input       | `ZX_PROTOCOL_AUDIO_INPUT`  | /dev/class/audio-input
Output      | `ZX_PROTOCOL_AUDIO_OUTPUT` | /dev/class/audio-output

<!---

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

--->

### 建立流媒体通道

打开设备节点后，客户端应用可以获取一个流媒体通道，使用`fuchsia.hardware.audio.Device/GetChannel` 的 FIDL 消息进行通信。

### 流媒体通道的客户端终止

客户端可以在任意时间通过简单的在流媒体通道上调用 [zx_handle_close(...)](/docs/reference/syscalls/handle_close.md) 接口完成与流媒体终止连接。驱动程序必须关闭使用该流通道建立的任何活动的环形缓存通道，然后在这个过程中，尽可能优雅地停止任何正在进行的流媒体操作。

<!---

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

--->

### 在流媒体和环形缓存通道上发送和接受消息

所有可以在流媒体和环形缓存通道上发送或接受的消息和消息负载都定义在[stream.fidl](/sdk/fidl/fuchsia.hardware.audio/stream.fidl) 和[ring_buffer.fidl](/sdk/fidl/fuchsia.hardware.audio/ring_buffer.fidl) 中。

消息通过[zx_channel_write(...)](/docs/reference/syscalls/channel_write.md) 的系统调用发送到驱动。如果想要获取回复，则使用[zx_channel_read(...)](/docs/reference/syscalls/channel_read.md) 系统调用来获取。最好的做法推荐是排列数据包后对 [channel(s)](/docs/reference/kernel_objects/channel.md) [port](/docs/reference/kernel_objects/port.md) 使用[zx_port_queue(...)](/docs/reference/syscalls/port_queue.md) 系统调用，并且使用[zx_port_wait(...)](/docs/reference/syscalls/port_wait.md) 调用来决定什么时候你设置的通道读取信息（可以是期待回复或者异步通知）。

有不同的语言绑定以方便发送和接受 FIDL 消息，特别是对于 C++ 驱动程序，还有一个[SimpleAudioStream](/src/media/audio/lib/simple-audio-stream) 的库，便于在 C++ 中创建驱动程序，这个库使用[LLCPP](/docs/reference/fidl/bindings/llcpp-bindings.md) 绑定来发送和接收 FIDL 消息。

<!---

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

--->

## 格式协议

### 采样格式

`Format`相关的协议信息允许驱动程序向客户端列举它支持的格式。支持格式可能包含多种比率，每个采样的比特数等。每一个驱动发布它可支持的内容和客户端对于每个驱动的授权格式。

为了找出给定驱动支持的格式，客户端使用`GetSupportedFormats` 功能。驱动程序回复一组`SupportedFormats`信息，其中每个`SupportedFormats`包含`PcmSupportedFormats` 和：

- 通道数量向量。它列举了驱动支持的通道数量，例如`<2,4,6,8>`。包含两个元素的向量`<2,4>`标识驱动支持两个或者四个通道。必须按升序排列。
- 采样格式的向量，例如`PCM_SIGNED`。
- 采样率的向量。帧采样率，例如44100,48000和96000。必须按升序排列。
- 每个通道/槽/容器的比特数。每个通道分配的比特数保存采样，例如每个通道 32 bits。必须按升序排列。
- 每个样本的比特数。采样宽度应小于通道快读，例如每个样本 24 bits 在 32 bits的通道内。必须按升序排列。

<!---

When not all combinations supported by the driver can be described with one
`PcmSupportedFormats`, the driver returns more than one `PcmSupportedFormats` in
the returned vector. For example, if one `PcmSupportedFormats` allows for 16 or 32 bits samples at
48KHz, and 16 bits samples at 96KHz, but not 32 bits samples at 96KHz, then the driver
replies with 2 `PcmSupportedFormats`: `<<16bits,32bits>,<48KHz>>` and
`<<16bits>,<96KHz>>`. For simplicity, this example ignores parameters other than
rate and bits per sample. In the case where the driver supports either 16 or 32
bits samples at either 48 or 96KHz, the driver would reply with 1
`PcmSupportedFormats`: `<<16bits,32bits>,<48KHz,96KHz>>`.

--->

当采样格式不是驱动支持所有的组合时，则使用`PcmSupportedFormats`来描述，驱动在返回的向量中包含一个以上的`PcmSupportedFormats`。例如如果一个`PcmSupportedFormats` 格式允许 16 或者 32 bits 采样为频率 48KHz，16 bits 采样为96KHz，但不是 32 bits 采样频率为96KHz，那么驱动回复两个 `PcmSupportedFormats`: `<<16bits,32bits>,<48KHz>>`和`<<16bits>,<96KHz>>`。简单来说，这个示例中忽略了其他 bits 和每个采样的 bits 数的参数。在驱动支持16 或者 32 bits 采样频率为 48 或者 96 KHz时，驱动将回复一个`PcmSupportedFormats`: `<<16bits,32bits>,<48KHz,96KHz>>`。

<!---

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

--->

另外，假设每个采样的比特数总是小于等于每个通道的比特数。因此驱动可以上报`<<16bits_per_channel,32bits_per_channel>,<16bits_per_sample,32bits_per_sample>>`，这并不表明报告的 16 bits 采样上的每个采样 32 bits 上是有效的， 它只规定了3种有效的组合：

- 16 bits 通道，16 bits 采样
- 32 bits 通道，32 bits 采样 
- 32 bits 通道，16 bits 采样

客户端根据驱动在 `GetSupportedFormats`回复中提供的信息，指定使用`CreateRingBuffer`函数的格式，什么是客户端支持的和其他任意的需求。这个函数接受一个参数，指定以下内容：

- 通道数量。这是在缓存区内可用的通道数量。
- 通道使用的位掩码。在缓存区内驱动使用的通道。例如对于立体声来说，必须使用 2 位使能的位掩码`0x3`，例如通道0和1都被使用。
- 采样格式。
- 帧采样率。
- 每个通道的比特数。
- 每个采样的比特数。

<!---

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

--->

注意：

- 默认情况下，多字节采样格式被认为是使用主机字节序。
- `PCM_FLOAT` 编码特别用于[IEEE 754](https://en.wikipedia.org/wiki/IEEE_754) 浮点代表。
- 默认情况下，非浮点 PCM 编码被认为是使用[two's complement](https://en.wikipedia.org/wiki/Two%27s_complement) 的有符号整数表示。例如 16 bit 的 PCM 采样格式比特数将从[0x8000, 0x7FFF] 的范围中，使用0x0000 代表 0 扬声器偏转。如果使用`PCM_UNSIGNED`的样本格式，比特数将从范围为 [0x0000, 0xFFFF]  中使用0x8000 代表0偏转。
- 当编码一个较小的样本在较大的通道中（例如 20或24 bits 在 32 bits  的通道中），32 bit 容器中最重要的位被使用，最不重要的位将被忽略（向左对齐）。例如一个 20 bit 的样本将映射到范围为 \[12,31\] （位\[0,11\] 将被忽略）的 32 bit 容器中。

<!---

### Setting the desired stream format

In order to select a stream format, applications send a `CreateRingBuffer` message over the
stream channel. In the message, the application specifies the format to be used.

The client specifies the new ring buffer channel over which
streaming operations will be conducted. If a previous ring buffer channel had been
established and was still active, the driver must close this channel and
make every attempt to gracefully quiesce any on-going streaming operations in
the process.

> TODO: specify how compressed bitstream formats will be set

--->

### 设置期望流格式

为了选择流媒体格式，应用程序通过流通道发送`CreateRingBuffer` 消息。在这个消息中，应用定义了使用的格式。

客户端定义了新的环形缓存通道，通过该通道进行流媒体操作。如果早先的环形缓存通道已经建立并且依旧存活，驱动必须关闭这个通道并且尽可能优雅的关闭进程内的任意正在运行的流媒体操作。

> 待办项：定义如何设置压缩比特流格式。

<!---

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

--->

## 决定板外延迟

音频流的板外延迟被定义为输出音频从系统互连线到扬声器自身，或者输入音频从麦克风到系统互连线的总体耗时。举例来说，一个外部编解码器连接到系统使用 TDM 互连线：如果这个互连线在接收 TDM 帧和在扬声器本身渲染帧中间输入有4帧延迟，那么这个音频路径的外部延迟相当于4个音频帧的持续时间。

外部延迟在`RingBufferProperties`对`GetProperties`的响应的`external_delay` 字段中报告。驱动应当尽力准确的上报所有已知的源的总体延迟。延迟的信息可以在编解码数据表中周期查询到，作为编解码的特性使用内部 HDA 或者 USB 音频定义主动上报，或者当使用 HDMI 或者 DisplayPort 互连线时，使用例如 EDID 的机制，由下游设备上报。

<!---

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

--->

## 硬件增益控制

### 硬件增益控制能力报告

为了决定一个音频流的增益控制能力，如果之前并没有完成，那么应用将通过流通道发送一个`GetProperties` 消息。这条消息不需要添加参数。驱动将回复`StreamProperties`消息，包括其他驱动的增益能力。所有流媒体驱动必须响应这条消息，尽管这个流硬件是否具有任何的增益控制能力。所有增益数使用 32 bit 浮点数代表 dB。

驱动用表示当前流的增益控制能力数据响应这条消息。当前增益设置使用一个 bool 变量表示该流媒体是否被静音，另一个 bool 变量表示该流媒体是否可以 AGC，以及最大最小的增益设置和`gain_step_db`。`gain_step_db`表示了从最小增益值开始计数，可控制增益的最小增量。

例如，一个功放有5个每级 7.5 dB的增益级别，最大增益为0 dB ，则表示一个范围为(-30.0, 0.0) ，步长为7.5的功放。具有功能连续增益控制能力的功放可以将其增益步长编码为0.0。

不管静音功能如何，驱动对于固定增益流必须以(0.0, 0.0)格式上报它们的最小和最大增益。`gain_step_db`在这种情景下是没有意义的，但是驱动依然要以0.0来上报。

<!---

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

--->

### 设置硬件增益控制级别

为了改变流媒体当前增益设置，应用程序可以通过流媒体通道发送`SetGain`消息。这个消息包含参数`GainState`，用来表明配置的增益参数，其中包括应用于该流媒体的 dB 增益，静音和 AGC 使能。

假设这个请求是有效的，驱动应当对请求的参数做支持增益补偿大小最临近的取整。例如如果一个流可以控制其增益在范围为 -60.0 到 0.0 dB 上，增益步长为 0.5 dB ，那么设置增益为 -33.3 dB 的请求就会变成增益为 -33.5 dB 。同样对该音频流请求设置 -33.2 dB 就会变成 -33.0 dB。

<!---

### Gain state notifications

Clients may request that streams send them asynchronous notifications of
gain state changes by using the `WatchGainState` command. The driver will reply to the
first |WatchGainState| sent by the client and will not respond to subsequent
client |WatchGainState| calls until the gain state changes from what was most recently
reported.

--->

### 增益状态通知

客户端可以使用 `WatchGainState` 的指令请求流媒体发送它们的增益状态变化的异步通知。驱动将回复客户端发送的第一条|WatchGainState| ，等到增益状态从最近上报的数据发生变化时，才回复后续客户端请求的|WatchGainState|。

<!---

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

--->

## 插入检测

除了为了响应与总线连接或断开而发布/解除流之外，音频流还可以在任意给定时间点插入或拔出。例如一组 USB 话筒在连接到 USB 时可能发布一个新的输出流，但是从插入检测点选择为“硬连接”。当连接 USB 时，一个带有3.5 mm 标准拾音插座的不同的 USB 音频适配器可以发布一个输出音频流，但是在用户插入拔出一个 3.5 mm 插座的物理设备，可以选择改变它的插入/拔出状态。

<!---

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

--->

### 插入检测能力

为了决定流媒体插入检测能力，如果之前并没有设置，那么应用将通过流媒体通道发送`GetProperties`指令。驱动返回带有插入检测能力`plug_detect_capabilities`的`StreamProperties`回复。

有效插入检测能力标志现在定义为：

- `HARDWIRED`为当流硬件被认为是“硬连线”时设置。也就是说，音频流被认为是设备一发布就连接上。示例包括一组内置的扬声器，一对 USB 麦克风，或者一个没有插入检测的可插入的音频设备。
- `CAN_ASYNC_NOTIFY`标志在以下两种情况设置，当流硬件既能够异步检测设备的插入状态已变更，或者在客户端请求这些通知时发送消息时。

<!---

### Plug state notifications

Clients may request that streams send them asynchronous notifications of
plug state changes by using the `WatchPlugState` command if the `CAN_ASYNC_NOTIFY`
flag was sent by the driver in `StreamProperties`. I.e. drivers for streams that
do not set the `CAN_ASYNC_NOTIFY` flag are free to ignore the `WatchPlugState` sent
by applications. Driver with `CAN_ASYNC_NOTIFY` set will reply to the first
|WatchPlugState| sent by the client and will not respond to subsequent client
|WatchPlugState| calls until the plug state changes from what was most recently reported.

--->

### 插入状态通知

如果驱动内`StreamProperties`发送`CAN_ASYNC_NOTIFY`标志，那么客户端可以通过使用`WatchPlugState` 指令请求流发送它们的插入状态变更的异步通知。例如，对于流没有设置`CAN_ASYNC_NOTIFY` 标志，驱动可以自由的忽略由应用发送的`WatchPlugState`。设置了`CAN_ASYNC_NOTIFY`的驱动将回复第一个由客户端发送|WatchPlugState| ，等到增益状态与最近上报的数据发生变化时，才会回复后续客户端请求的|WatchPlugState|。

<!---

## Access control capability detection and signaling

> TODO: specify how this works. In particular, specify how drivers indicate to
> applications support for various digital access control mechanisms such as
> S/PDIF control words and HDCP.

--->

### 访问控制能力检测和信号发送

> 待办项：需要定义怎样工作。尤其是定义驱动怎样指示应用支持各种数字访问控制机制，例如 S/PDIF 控制字和 HDCP 。

<!---

## Stream purpose and association

{% comment %}
> TODO: specify how drivers can indicate the general "purpose" of an audio
> stream in the system (if known), as well as its relationship to other streams
> (if known). For example, an embedded target like a phone or a tablet needs to
> indicate which output stream is the built-in speaker vs. which is the headset
> jack output. In addition, it needs to make clear which input stream is the
> microphone associated with the headset output vs. the builtin speaker.
{% endcomment %}

--->

## 音频流目的和联系

{% comment %}

> 待办项：定义驱动如何指示系统（已知）中的音频流的通用“目的”，正如他和其他流（已知）的关系。例如，一个嵌入式对象就像电话或平板，需要支持哪一个输出流是内置扬声器或者哪一个耳机插孔输出。另外，也需要明确哪一个输入流是麦克风相关的耳机输出，或者内置扬声器。
>
> {% endcomment %}

<!---

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

--->

## 环形缓冲通道

### 概述

当应用一旦成功设置音频流格式后，它将收到一个新 [通道](/docs/reference/kernel_objects/channel.md) 的回复，该通道代表了它连接的音频流环形缓冲区。客户端使用环形缓冲通道来建立一个共享内存缓冲去，并且实现开始、停止播放和捕获音频流数据。

环形缓冲内容由客户端（用于播放）和驱动方（用于记录）产生。因此客户端作为播放的生产者，消费者用于记录，和驱动作为记录的生产者，消费者来播放。环形缓冲内容可以由音频硬件直接消费或生产，或者也可以通过驱动完成每个采样的软件处理。

从成功响应`Start`指令时给出的时间点开始，环形缓冲区数据以额定速率生产。注意尽管环形缓冲在内存总线和音频硬件之间几乎肯定有某种形式的 FIFO 缓存，这将造成音频流的预读取（在播放的场景下），或者潜在的持有数据（在捕获的场景下）。对于客户端来说，在开始操作之前获取这个缓存的大小是非常重要的，这样它就能知道在音频流的前段/最后，他们需要保留多少位置去读取/写入，来确保音频免遭干扰。同样需要注意的是，因为系统共享缓存区的特性，并且驱动可能直接从该缓存直接 DMA 到硬件，对于客户端运行在不能自动确保缓存连贯的架构上时，它们在写入播放数据到缓冲区中后正确写回高速缓存中，或者在读取捕获数据之前无效化高速缓存是非常重要的。

<!---

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

--->

### 确定 FIFO 的深度

为了确定音频流的`fifo_depth`，如果之前没有设置的话，应用会通过音频流通道发送`GetProperties`指令。驱动回复`StreamProperties`，其中包括其他域中的`fifo_depth` 。为了确保特有播放或音频捕获，应用和驱动必须严格遵守这个值。当播放音频流数据时，驱动不能读取超出流的额定播放位置加上这个字节数的地方。在捕获音频流数据时，应用必须在流的标称捕获点之后保留这个字节数长度。

环形缓冲数据可以由硬件直接被消耗/产生，在这种情况下，`fifo_depth`直接映射硬件 FIFO 块的大小，因为硬件 FIFO 块确定了读取头或保留末尾的数据量。

环形缓冲数据可以由音频驱动软件消耗/生成，在概念上位于环形缓冲区和音频硬件之间。在这种情况下，对于播放`fifo_depth`的预读取量要被设置的足够大，以便驱动保证没有未检测的欠载运行，这假设了客户端是正如`CreateRingBuffer`和`Start`指令设置的产生数据。对于捕获来说，`fifo_depth`保持数据量要被设置的足够大，当产生的数据正如`CreateRingBuffer`和`Start`指令所设置，以便驱动保证没有未检测的欠载运行。驱动必须设置 `fifo_depth`足够大，以便潜在的大多数情况下，任意软件接口和音频硬件之间所增加的延迟都不会发生，并且必须检测并上报欠载运行状态。但是欠载运行状态的上报如何定义则不在此处讨论。

当音频流的格式被设置，环形缓冲通道打开后，驱动就不能更改这个值了。从应用程序的角度来看，这是环形缓冲通道的固定特性。

<!---

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

--->

### 获取共享内存

为了发送或接受音频，应用程序必须首先创建一个共享内存缓冲区。通过环形缓冲通道发送 `CreateRingBuffer`请求完成创建。这只能在环形缓冲区停止时执行。

如果驱动实例要关闭 `CreateRingBuffer`创建的通道时，因为已经建立了一个缓冲区，并且环形缓冲区已经开启，则驱动既不能停止环形缓冲区，也不能丢弃已存在的共享内存。当环形缓冲区已经停止时，如果应用程序在已经创建了一个缓冲区后还要请求一个新的缓冲区，那么它必须考虑存在缓冲区将变为无效，旧的缓冲区现在不可用。

当请求新的环形缓冲区时，应用必须定义两个参数：`min_frames`和`clock_recovery_notifications_per_ring`。

<!---

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

--->

#### `min_frames`

客户端需要分配给环形缓冲区的最小音频帧数。驱动可以使这个缓冲区更大，以满足硬件要求。客户端必须使用返回的 VMOs 的大小（单位字节）来决定环形缓冲区的实际大小。客户端不能假设这个大小（由驱动实际确定）就正好是他们请求的大小。驱动必须确保环形缓冲区的大小是音频帧数的整数。

{% comment %}

> 待办项：要求驱动产生的缓冲区是一个长度为音频帧的整数是否合理？这确实可以让音频客户端处理更简单（客户端代码在处理之前不需要分离或重新装配帧），但是这可能使某些音频硬件为了满足不让缓冲区过于大于客户端请求的需求变得困难。
>
> {% endcomment %}

<!---

Optional number of position update notifications the client would like the driver to
send per cycle through the ring buffer, these notifications are meant to be used for clock
recovery. Drivers must only send these as a reply to a `WatchClockRecoveryPositionInfo` request.
Drivers should attempt to space notifications uniformly throughout the ring; however clients
must not rely on perfectly uniform spacing of the update notifications.

--->

客户端希望驱动在每个周期内通过环形缓冲区发送可选位置更新的通知，这些通知则意味着用于时钟恢复。驱动必须仅发送这些作为`WatchClockRecoveryPositionInfo`请求的回复。驱动应当试图在整个环形是均匀的安排通知信息；尽管客户端也不能依赖于更新通知具有完全统一间隔。

<!---

#### `ring_buffer`

If the request succeeds, the driver must return a handle to a
[VMO](/docs/reference/kernel_objects/vm_object.md) with permissions that allow applications to map
the VMO into their address space using [zx_vmar_map](/docs/reference/syscalls/vmar_map.md),
and to read/write data in the buffer in the case of playback, or simply to read
the data in the buffer in the case of capture.

--->

#### `ring_buffer`

如果请求成功，驱动必须返回一个句柄到 [VMO](/docs/reference/kernel_objects/vm_object.md)，允许应用程序使用[zx_vmar_map](/docs/reference/syscalls/vmar_map.md) 映射 VMO 到他们自己的地址空间中，满足在播放场景下在缓存区中读/写，或者捕获场景下简单读取缓存区中的数据。

<!---

#### `num_frames`

If the request succeeds, the driver will also return the actual number of frames of audio
it will use in the buffer. The size of the VMO returned (as reported
by [zx_vmo_get_size()](/docs/reference/syscalls/vmo_get_size.md)) must not be larger than
this number of frames (when converted to bytes). This number may be larger
than the `min_frames` request from the client but must not be smaller than this number.

--->

#### `num_frames`

如果请求成功，驱动也将返回缓存区中使用的实际音频帧数。 [zx_vmo_get_size()](/docs/reference/syscalls/vmo_get_size.md)) 指令返回的 VMO 大小不能大于帧数（当转换成字节时）。这个值可能大于来自客户端的`min_frames`请求，但不能小于`min_frames`。

<!---

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

--->

### 开始、停止环形缓冲区

客户端可以使用`Start`和`Stop`指令来请求开始或停止环形缓冲区。试图启动一个已经启动的音频流，必定会被认为是失败的。试图停止一个已经停止的音频流，应该被认为是成功的。环形缓冲区在使用 `CreateRingBuffer`操作建立共享内存缓冲之前都不能停止或启动。

在成功启动一个流后，驱动必须提供在回复的`start_time` 域中包含它们硬件开始传输或捕获音频流最佳估算时间。这个时间戳必须是来源于暴露的[zx_clock_get_monotonic()](/docs/reference/syscalls/clock_get_monotonic.md)系统调用。伴随着环形缓冲区的 FIFO 深度属性，这个时间戳允许了应用在不需要来自驱动的周期位置更新的前提下，发送或接受流数据。伴随着流通道提供的输出延迟估计，这个时间戳允许应用在多通道流中，或者甚至是多个设备中同步音频信息描述。（提供一个外部时间同步协议是用于在整个同步设备群组中同步[monotonic](/docs/reference/syscalls/clock_get_monotonic.md) 时间线）。

<!---

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

--->

{% comment %}

> 待办项：重定义`start_time`是允许任意音频流时钟替代`zx_clock_get_monotonic()`时钟。如果一个流时钟从开始时就以音频帧计数，那么这个`start_time` 可以被替换成一个线性变换的分段片段，在音频硬件时钟是源于与系统节拍计数器不同的晶振情况下，可以通过驱动发送的通知来进行后续更新。那么客户端可以使用这个转换要么控制输入音频流消耗频率，要么在输入音频流中何处采样以实现时钟校正。

当成功启动流后，驱动必须保证在开始回复已经入队到环形缓冲通道之前，都不发送位置通知。

在成功停止流后，驱动必须保证在停止回复入队之后，都不再有位置通知信息入队到环形缓冲通道中。

<!---

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

--->

### 位置通知信号

如果客户端在`CreateRingBuffer`操作中传递一个非零`clock_recovery_notifications_per_ring`请求，驱动将周期发送更新信息到客户端通知，表明它在缓存区中的当前生产或消耗的位置信息。这个位置信息在 `WatchClockRecoveryPositionInfo`请求中作为回复里的`RingBufferPositionInfo`结构体中的`position` 字段用字节表示。消息中同样也包含了 `timestamp`字段来包含这个字节位有效的时间（正如zx::time）。  仅当`clock_recovery_notifications_per_ring`在 `GetVmo`函数中被定义，并且等到`GetVmo`函数回复后，才能调用`WatchClockRecoveryPositionInfo`请求。注意这些位置通知信息表明了驱动已经在缓存区中哪里消耗或产生数据，而*不是*额定播放或者捕获位置（有时候也分别叫做“写指针”或者“读指针”）。他们达到的时间不是完全能保证一致，并且也不能被用作实现时钟恢复。尽管如此对应 (`timestamp`, `position`)值自身*是*用来恢复音频流的时钟。如果客户端发现驱动已经消耗了该客户端写入播放数据的环形缓冲区中的点，音频表现将会变成未定义的状态。客户端应该增加他们的时钟准备时间，并在未来一定要在这个点上保持领先。就像捕获音频的客户端不应该试图在驱动发送的大多数最近位置通知信息中，读取环形缓冲区指示点之前的点。

紧随着一个成功的`Start`指令后，驱动播放和捕获位置必须*始终*以环形缓冲区的字节0开始。当环形缓冲区位置到达 VMO 的末尾（[zx_vmo_get_size(...)](/docs/reference/syscalls/vmo_get_size.md)表示），环形缓冲区位置则会回到0。驱动不需要以音频帧的整数消耗或产生数据。对数据流位置的概念依赖于位置通知信号的客户端应当注意要求每个环发送足够数量的通知（最少两个），并快速处理这些通知，以免发生别名问题。

<!---

### Clock recovery and synchronization

Upon receiving `AUDIO_STREAM_CMD_GET_CLOCK_DOMAIN` message, the driver must
respond with the identifier of the clock domain containing that device. If the
audio device is locked to the local system monotonic clock and does not expose a
mechanism by which its rate would be fine-tuned, then it should return the value
0 to represent the local CLOCK_MONOTONIC domain. A client may use this
information (in addition to `AUDIO_RB_POSITION_NOTIFY` messages) to simplify the
process of recovering the audio device's clock.

--->

### 时钟恢复和同步

收到`AUDIO_STREAM_CMD_GET_CLOCK_DOMAIN`消息之后，驱动必须回复包含该设备的时钟域标识符。如果音频设备被锁定在本地系统单调时钟上，并且没有暴露一个可以微调其速率的机制，那么它应该返回值0来代表本地CLOCK_MONOTONIC  域。客户端可以使用这个信息（加上`AUDIO_RB_POSITION_NOTIFY`消息）来简化恢复音频设备时钟的步骤。

<!---

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

--->

{% comment %}

> 待办项：扩展该章节包含时钟恢复是怎样发生的，并且怎样暴露给客户端。同样，详细说明如何发现和控制可回转晶振。我们可能需要向可回转时钟的客户端发送速率变化的通知信息。
>
> 之前的内容：待办项：在音频输出晶振不是来自于`zx_clock_get_monotonic()`晶振的情况下，定义一种方式可以将时钟恢复信息发送给客户端。另外如果晶振是硬件可回转的，那么就要提供发现这个能力和控制回转频率的功能。鉴于该晶振可能被多个数据流共享，最好是返回某种形式的系统范围内的时钟标识符，并提供获取通道的能力，时钟恢复通知可以被传递给客户端，硬件回转命令可以从客户端发送到时钟。
>
> {% endcomment %}

<!---

### Error notifications

{% comment %}
TODO: define these and what driver behavior should be, if/when they occur.
{% endcomment %}

--->

### 异常通知信息

{% comment %}

待办项：定义异常通知信息，以及如果/当它们发生时，驱动的行为应该是什么。

{% endcomment %}

<!---

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

--->

### 客户端意外终止

如果环形缓冲区控制通道的客户端因任何原因关闭，驱动必须马上关闭控制通道，并且关闭环形缓冲区，以便更多音频不会发出或渲染。虽然鼓励驱动以优雅的方式从生产数据到关闭的方式进行，但他们必须确保音频流进入静默状态，而不是循环。一旦完全切换到静默状态，播放或捕获的相关资源将被驱动释放并且重用。

在这种方式下，如果一个播放客户端意外终止，系统将关闭客户端通道，以致音频停止播放而不是继续循环。

