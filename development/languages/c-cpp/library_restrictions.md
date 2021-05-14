# 库限制

## third_party/abseil-cpp

决定：**请勿**在新代码中使用 `absl` 。通常，`absl` 不太适合 Fuchsia 的用例。一旦现有的客户端迁移完毕，将从树中删除 `absl`。在 [fxbug.dev/59428](https://fxbug.dev/59428) 处跟踪此删除工作。

## third_party/googletest

注意：googletest 库包括以前的 gtest 和 gmock 项目。

### Gtest

使用 Gtest 框架在除 Zircon 目录之外的任何地方编写测试。它提供了 `TEST` 和 `TEST_F` 宏以及 `ASSERT` 和 `EXPECT` 变量。

在 Zircon 目录中，改用 `system/ulib/zxtest`。它提供了一个类似 Gtest 的接口，减少了对诸如互斥锁(想要测试的东西)等高级操作系统概念的依赖。它还支持用 C 编写测试，这是某些层所必需的。

### Gmock

Gmock 有一些组件。允许使用 gmock 匹配器，例如 `ElementsAre()`。

关于函数 mocking 功能(`MOCK_METHOD` 和 `EXPECT_CALL`)，团队有不同的看法。

赞成：

  * 进行某些类型的 mocking 可能非常有效。
  * 有些人认为 Gmock 生成的 mock 比等效的自定义代码更容易阅读。
  * 缺少 mocking 库意味着有些人可能写不出好的测试。

反对：

  * Gmock 提供了特定于域的语言。 并非每个人都懂这种语言，并且模板和宏的复杂使用使诊断问题变得困难。
  * Gmock 的某些方面鼓励过度限制 mock。
  * 上面的组合可能使以后更难对 mock 代码进行更改。

决定：**请勿使用** gmock 的 mocking 功能（`MOCK_METHOD` 和 `EXPECT_CALL`）。
