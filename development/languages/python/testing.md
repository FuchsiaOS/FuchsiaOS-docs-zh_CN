<!--
# Python testing

To define a host-side python test that can be run by `fx`, CI and CQ:

*   Use the [python_host_test](/build/python/python_host_test.gni) GN template.
*   Ensure some `group("tests")` depends on the `python_host_test` rule,
    and specify the `($host_toolchain)` in the dependency.

[Here](/sdk/cts/build/scripts/BUILD.gn) is an example BUILD.gn.
-->

# Python 测试

定义一个可以由`fx`、CI 和 CQ 运行的主机端 python 测试：

* 使用 [python_host_test](/build/python/python_host_test.gni) GN 模板。
* 确保一些 `group("tests")` 依赖于 `python_host_test` 规则，
  并在依赖项中指定`($host_toolchain)`。

[这里](/sdk/cts/build/scripts/BUILD.gn) 是一个示例 BUILD.gn。