<!--
# Catapult User Guide (Version 1)

* Updated: 2018 July 27

[TOC]
-->

# 用户指南（版本1）

* 更新于：2018年7月27日

[TOC]

<!--
## Overview

The Catapult dashboard is the UI we send benchmark results to for monitoring and
visualization.  The dashboard is maintained by the Chrome team.  This a short guide on how
to find and use the results of your benchmarks in the dashboard.
-->

## 概述

基准显示面板是我们发送基准测试结果用于监控和可视化的UI。显示面板由Chrome团队维护。这里又一个简短的指南，它说明如何在面板中找到和使用你基准测试的结果。

<!--
## Accessing the Dashboard

*** promo
**Be sure to sign into your google.com account or else Fuchsia data will be hidden.**

The login button is in the top right corner of the screen.
***

The dashboard can be found at https://chromeperf.appspot.com/report.
-->

## 访问基准显示面板

*** promo
**确定已经登陆上google.com的账户，否则Fuchsia数据会被隐藏。**
登陆按钮在屏幕顶部右上角
***

显示面板可以在`https://chromeperf.appspot.com/report`找到。

<!--
## Searching and Adding Graphs

The dashboard displays a list of search boxes.  The placeholder names are relics from the
days when Chrome infrastructure was still using BuildBot.  Since they are not relevant to
Fuchsia infrastructure, we map our own data into these fields with the following scheme:

* `Test suite` == the name of the benchmark suite.
* `Bot` == A Fuchsia LUCI builder that has run the benchmark at least once.
* `Subtest` == The name of the test case in your benchmark suite.

Type the name of your benchmark suite in the first box to begin searching.   As an
example, we can see the zircon_benchmarks suite if we type "zircon"

![test_suite_example](/images/benchmarking/test_suite_example.png "test_suite_example")
-->

## 搜索和添加图

显示面板中显示了一组搜索框。占位符的名字还是Chrome说明使用BuildBot时的遗迹。因为它们和Fuchsia说明没有关系，我们将自己的数据根据如下的模式映射到这些字段中：

* `Test suite` == 基准测试套件的名字。
* `Bot` == 至少运行了一次基准测试的Fuchsia LUCI编译器。
* `Subtest` == 在你的基准测试中的测试用例名字。

![test_suite_example](/images/benchmarking/test_suite_example.png "test_suite_example")

<!--
Select a builder and a subtest.  Note that if your subtest is named "foo", there will be
multiple "foo_<metric_name>" subtests to choose from.  Each of these represents a metric
computed from the sample(s) of that subtest.   For example: if "foo" generates N sample
points each time the benchmark is run, then the subtest "foo_avg" is a plot of the
averages of these N samples.

When you're finished filling out each field, click "Add" to add your graph to the UI.
You should see something like this:

![graph_example](/images/benchmarking/graph_example.png "graph_exmaple")
-->

选择一个编译器和子测试。注意如果你的子测试命名为`foo`，这里会有多个`foo_<metrics_name>`子测试供选择。这些子测试每一个代表这个子测试取样中计算出来的度量标准。例如，如果`foo`在每次基准测试运行时生成了N个采样点，那么子测试`foo_avg`是这N个采样的平均值的绘图。

当你完成了填充每一个域，点击`Add`按钮添加你的图到UI上。你会看到如下的一些图：

![graph_example](/images/benchmarking/graph_example.png "graph_exmaple")

<!--
## Viewing sample metadata

If you hover over a point in a graph, you can see some extra information such as the
point's value, the date it was recorded, and a link to the log page of the build that
generated it.

![tooltip_example](/images/benchmarking/tooltip_example.png "tooltip_example")
-->

## 查看采样元数据

如果你将鼠标放到图上某一点之上，你可以看到一些额外的信息，比如这一点的值，并且也记录了日期，以及编译生成这个结果的日志页。

![tooltip_example](/images/benchmarking/tooltip_example.png "tooltip_example")

<!--
## Saving the View

v1 of the Catapult dashboard UI does not have a built in mechanism for saving a collection
of Graphs.  If you want to save a list of graphs so that you can share with others or
re-open the list later, you can copy the URL from the Chrome Address Bar.

Beware, you will have to re-copy the URL each time you add, modify or remove a graph. This
includes moving the green slider beneath a graph or making any selections in the box to
the right of the graph.
-->

## 保存视图

第一个版本的基准测试结果显示面板的UI没有内建机制用于保存图形集合。如果你想要保存一列图，用于和其他人分享或在以后重新打开列表，可以从Chrome浏览器地址栏中拷贝URL。

注意，你必须在每次添加，修改或删除一个图之后重新拷贝URL。这包括移动绿色图形下方的滚动条或在图的右边列表中选择新值。

<!--
## Enabling Regression Detection

To enable regression detection, you must enable "monitoring" for a test by clicking the
"Request Monitoring for Tests" button under the "Report issue" dropdown at the top of the
page.

![monitoring_button_example](/images/benchmarking/monitoring_button_example.png "monitoring_button_example")

This will open a bug form you can fill out to enable monitoring for a benchmark.  The
Chrome team has a Sheriff rotation (oncall rotation) to triage regression alerts.  The
dashboard only allows triaging bugs in monorail, so we'll have to make due without JIRA
support.

See this link for more information about the [Sheriff rotation]

[Sheriff rotation]: https://chromium.googlesource.com/chromium/src/+/master/docs/speed/perf_regression_sheriffing.md
-->

## 开启复原检测

为了开启复原检测，你必须通过点击`Request Monitoring for Tests`按钮来为你的测试开启`monitoring`，这个按钮在页面顶部的`Report issue`下方。

![monitoring_button_example](/images/benchmarking/monitoring_button_example.png "monitoring_button_example")

这样就会打开一个bug表格，你可以填充它来为一次基准测试开启监控。Chrome团队有一个Sheriff循环（随时轮转）来分类修复警告。显示面板仅允许单向分类bugs，因此我们必须没有JIRA支持。

参考下面链接更多关于[Sheriff rotation]的信息。

[Sheriff rotation]: https://chromium.googlesource.com/chromium/src/+/master/docs/speed/perf_regression_sheriffing.md