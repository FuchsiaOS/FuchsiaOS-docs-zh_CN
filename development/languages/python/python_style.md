<!--
# Python style guide

The Fuchsia project follows the [Google Python style guide](https://github.com/google/styleguide/blob/gh-pages/pyguide.md){:.external},
with a few [refinements](#refinements).

The Google Python style guide allows more variation (presumably to cover a large
breadth of existing source). This guide has a tighter set of choices. So a
Fuchsia Python file will also comply with the Google style guide, but a Google
Python file might not comply with this guide. See [refinements](#refinements)
below for details.
-->

# Python 风格指南
Fuchsia 项目遵循 [Google Python 风格指南](https://github.com/google/styleguide/blob/gh-pages/pyguide.md){:.external}，
并且有一些 [改进](#refinements)。

Google Python 风格指南允许更多的变化（大概涵盖了众多现有来源）。
本指南有更严格的选择。所以Fuchsia Python 文件也将遵循 Google 风格指南，但一个 Google Python 文件可能不符合本指南。见[改进](#refinements)
详情如下。

<!--
## Python versions {#python-versions}

### Scripts invoked by the build

Scripts invoked by the build (GN or Ninja) are executed with Python 3.8.

The build system ensures that all python scripts are executed by the
interpreter that is intalled as part of a Fuchsia source checkout.
-->

## Python 版本 {#python-versions}

### 构建调用的脚本
构建（GN 或 Ninja）调用的脚本使用 Python 3.8 执行。
构建的系统确保所有 python 脚本都被 Fuchsia 源代码分支内安装的解释器执行。

<!--
### Other scripts

Scripts that are invoked directly should use `python` in the shebang and be
compatible with both 2 and 3: `#!/usr/bin/env python`.

Developers working on Fuchsia modules may use various platforms. Some platforms
include Python 2 and not Python 3 and vice versa. Until Python 3 is
included in the prominent development environments we support, we should support
Python 2.

While Python 2 is supported, test scripts on both versions.

Any policy change will be reflected in this document.
-->

### 其他脚本

直接调用的脚本应该在shebang中使用`python`并且兼容python 版本2和3：`#!/usr/bin/env python`。

开发 Fuchsia 模块的开发人员可能会使用各种平台。 
一些平台包括 Python 2 而不是 Python 3，反之亦然。 
在我们支持的主要开发环境中包含 Python 3 之前，我们应该支持 Python 2。

虽然支持 Python 2，但在两个版本上测试脚本。

任何政策变更都将反映在本文件中。

<!--
## Multiple Inheritance

Multiple inheritance is strongly discouraged. This is for the same reason
listed in the
[Google C++ style guide: risk of "diamond" inheritance](https://google.github.io/styleguide/cppguide.html#Inheritance){:.external}
patterns, which are prone to confusion. If a case is found where avoiding
multiple inheritance is unreasonable, all classes involved must initially
inherit from the base class `object`, which governs which multiple inheritance
scheme is used.
-->

## 多重继承
强烈反对多重继承。 这与 [Google C++ 风格指南：“钻石”继承风险] (https://google.github.io/styleguide/cppguide.html#Inheritance){:.external} 
模式中列出的原因相同，原因为 容易混淆。 
如果发现避免多重继承不合理的情况，则所有涉及的类必须首先从基类“object”继承，该基类决定使用哪种多重继承方案。

<!--
## Use Unicode for Text

In scripts that support Python 2.x (see [Python versions](#python-versions)),
explicitly declare text strings as unicode and binary data as bytes, using
`u""`, `unicode()`, `unichr()` and  `b""`, `bytes()`, `byte()` respectively.
Python 3.x defaults to using Unicode for strings, so this guideline will be
removed when support for Python 2 is dropped.
-->

## 对文本使用 Unicode

在支持 Python 2.x 的脚本中（参见 [Python 版本](#python-versions)），
显式声明文本字符串为 unicode，二进制数据为字节，分别使用
`u""`、`unicode()`、`unichr()` 和`b""`、`bytes()`、`byte()` 。
Python 3.x 默认对字符串使用 Unicode，因此当对 Python 2 的支持时移除时，本指南将被删除 。

```python {.good}
Yes:

  a = u"Hello"  # Unicode constant.
  b = unicode(foo)  # Convert to Unicode.
  c = unichr(c)  # Convert to Unicode.
  d = io.open("bar.txt").read()  # Read text as Unicode.
```

```python {.bad}
No:

  a = "Hello"  # Ambiguous (depends on Python version).
  b = str(foo)  # Convert to ascii.
  c = chr(c)  # Convert to ascii.
  d = open("bar.txt").read()  # Read text as ascii.
```
<!--
## Refinements

The following refinements we make to the Google Python style guide are largely
choices between variations. For example, if the style guide says you may do A,
B, or C we may choose to favor B and avoid the other choices.
-->
## 改进

我们对 Google Python 风格指南所做的以下改进主要是对变体之间的选择。 
例如，如果风格指南说你可以做 A，B 或 C 我们可以选择偏爱 B 并避免其他选择。

<!--
### Indentation
Avoid aligning with opening delimiter. Prefer instead to indent using fixed
(4 space) indentation.

(See
[Indentation](https://github.com/google/styleguide/blob/gh-pages/pyguide.md#34-indentation){:.external}
in the Google Python style guide for comparison.)
-->

### 缩进
避免与起始的分隔符对齐。偏好为使用固定缩进(4个空格)。

（参考
[缩进](https://github.com/google/styleguide/blob/gh-pages/pyguide.md#34-indentation){:.external}
和 Google Python 风格指南进行比较。）

<!--
### Statements

Avoid creating single line statements, even with `if` statements.

```python {.good}
Yes:

    if foo:
        bar(foo)
```

```python {.bad}
No:

    if foo: bar(foo)
```

(See
[Statements](https://github.com/google/styleguide/blob/gh-pages/pyguide.md#314-statements){:.external}
in the Google Python style guide for comparison.)

-->

### 声明

避免创建单行语句，即使使用 `if` 语句。

```python {.good}
Yes:

    if foo:
        bar(foo)
```

```python {.bad}
No:

    if foo: bar(foo)
```

(参考
[声明](https://github.com/google/styleguide/blob/gh-pages/pyguide.md#314-statements){:.external}
和 Google Python 风格指南进行比较。)

<!--
### Type annotations

In scripts that support Python 2 (see [Python versions](#python-versions)),
type annotations will not be used.

(See
[Type Annotations](https://github.com/google/styleguide/blob/gh-pages/pyguide.md#319-type-annotations){:.external}
in the Google Python style guide for comparison.)
-->

### 类型注释

在支持 Python 2 的脚本中（参见 [Python 版本](#python-versions)），
不会使用类型注释。

（参考
[类型注释](https://github.com/google/styleguide/blob/gh-pages/pyguide.md#319-type-annotations){:.external}
和 Google Python 风格指南进行比较。）

<!--
### Strings

Prefer double quotes for strings (`"`). Use single quotes when the declaration is
more readable with single quotes. For example, `'The cat said "Meow"'` is more readable
than `"The cat said \\"Meow\\""`.

(See
[Strings](https://github.com/google/styleguide/blob/gh-pages/pyguide.md#310-strings){:.external}
in the Google Python style guide for comparison.)
-->

### 字符串
字符串优先使用双引号 (`"`)。当使用单引号更具可读性时，声明时使用单引号。 例如，`'猫说了 "喵"'` 比`"猫说\\"喵\\""`更具可读性
。

（参考
[字符串](https://github.com/google/styleguide/blob/gh-pages/pyguide.md#310-strings){:.external}
和 Google Python 风格指南进行比较。）


<!--
### Be consistent

Be consistent within a large scope. Avoid displaying small pockets of consistency
within Fuchsia. Being consistent within only a single file or directory is not
consistency.

Within `third_party`, the intent is to follow the existing style for that project
or library. Look for a style guide within that library as appropriate.

(See
[Parting Words](https://github.com/google/styleguide/blob/gh-pages/pyguide.md#4-parting-words){:.external}
in the Google Python style guide.)
-->

### 保持一致

在大范围内保持一致。 避免在 Fuchsia 中显示出小范围的一致性。 仅在单个文件或目录中保持一致并不是一致性。

在“第三方”中，目的是遵循该项目的现有风格或库。 根据需要在该库中查找样式指南。

（参考 
[分割词](https://github.com/google/styleguide/blob/gh-pages/pyguide.md#4-parting-words){:.external}
和 Google Python 风格指南进行比较。）
