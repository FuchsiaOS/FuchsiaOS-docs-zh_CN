<!--
# Benchmark Results Schema

* Updated: 2018 August 9

[TOC]

This document describes the JSON schema that Fuchsia benchmark results must
follow in order to be uploaded to the performance dashboard.
-->

# 基准测试结果模式

* 更新于：2018年8月9日

[TOC]

这个文档描述了Fuchsia基准测试结果必须遵守的JSON模式，只有这样才能将结果上传到性能面板上。

<!--
## Helper Libraries

If you're creating a [trace-based benchmark], your exported results will
already have the correct schema.

If you're writing your own benchmark program, then you can use the existing
Fuchsia libraries for your language for emitting the JSON data:

* [C/C++]
* [Go]

NOTE: If your benchmark is in a different language, please provide a reuseable
library or file a bug against IN to request one.

[C/C++]: https://fuchsia.googlesource.com/zircon/+/master/system/ulib/perftest
[Go]: https://fuchsia.googlesource.com/garnet/+/master/go/src/benchmarking
[Dart]: #
[trace-based benchmark]: trace_based_benchmarking.md
-->

## 辅助库

如果你正创建一个[可追踪的基准测试]，你的导出结果会有正确的模式。

如果你正编写你自己的基准测试程序，那么你就可以使用现有的Fuchsia库用于你自己的编程语言用于放出JSON数据：

* [C/C++]
* [Go]

注意：如果你的基准测试使用不同的语言，请提供一个可用库，或者添加一个bug用于请求添加一个该语言的辅助库。

[C/C++]: https://fuchsia.googlesource.com/zircon/+/master/system/ulib/perftest
[Go]: https://fuchsia.googlesource.com/garnet/+/master/go/src/benchmarking
[Dart]: #
[可追踪的基准测试]: trace_based_benchmarking.md

<!--
## JSON Description

```json
[
    {
        "label":       string     // Name of the test case in the performance dashboard.
        "test_suite":  string     // Name of the test suite in the performance dashboard.
        "unit":        string     // One of the supported units (see below)
        "values":      [v1, v2..] // Numeric values collected in this test case
        "split_first": bool       // Whether to split the first element in |values| from the rest.
    },
    {
        ...
    }
]
```
-->

## JSON描述

```json
[
    {
        "label": string     // 在性能显示面板中的测试用例名字
        "test_suite": string // 在性能面板中的测试包名字
        "unit": string      // 其中一个支持单元（参考下面）
        "values": [v1, v2..] // 在这个测试用例中几个要收集的值
        "split_first": bool  // 是否要将[values]中的第一个值和其余的分开
    },
    {
        ...
    }
]
```

<!--
## Supported Units:

In order to convert benchmark results to the format required by the performance
dashboard, `unit` must be one of the following strings, which describe the units
of the result's `values`.

* `nanoseconds`  or `ns`
* `milliseconds` or `ms`
* `bytes/second`
* `bytes`
-->

## 支持单元

为了将基准测试结果转换成性能信息显示面板要求格式，`unit`必须是下面字符串之一，这些字符串描述了测试单元的结果的`values`。

* `nanoseconds`  or `ns`
* `milliseconds` or `ms`
* `bytes/second`
* `bytes`

<!--
### Example

```json
[
    {
        "label": "Channel/WriteRead/64bytes",
        "test_suite": "fuchsia.zircon_benchmarks",
        "unit": "nanoseconds",
        "values": [105.45, 697.916667, 672.743056],
        "split_first": true
    },
    {
        "label":"Channel/WriteRead/1024bytes",
        "test_suite":"fuchsia.zircon_benchmarks",
        "unit":"nanoseconds",
        "values":[102.23, 1004.340278, 906.250000],
        "split_first": true
    }
]
```
-->

### 例子

```json
[
    {
        "label": "Channel/WriteRead/64bytes",
        "test_suite": "fuchsia.zircon_benchmarks",
        "unit": "nanoseconds",
        "values": [105.45, 697.916667, 672.743056],
        "split_first": true
    },
    {
        "label":"Channel/WriteRead/1024bytes",
        "test_suite":"fuchsia.zircon_benchmarks",
        "unit":"nanoseconds",
        "values":[102.23, 1004.340278, 906.250000],
        "split_first": true
    }
]
```

<!--
## split_first behavior

split_first is useful when the first value in the test results is usually skewed
due to external influence on the test (e.g. empty caches).  When true, benchmark
results will appear as two separate series in the performance dashboard:

1. `$label/samples_0_to_0` which tracks the first element in `values`, and
1. `$label/samples_1_to_N` which tracks the remaining `values`.
-->

## split_first的作用

当测试结果中第一个值由于外部的影响（比如空缓存）影响准确性时，`split_first`参数就非常有用了。当该参数取值`true`，基准测试结果会在性能显示面板以两个单独的序列出现。

1. `$label/samples_0_to_0`追踪`values`中第一个元素。
2. `$label/samples_1_to_N`追踪剩余的`values`。
