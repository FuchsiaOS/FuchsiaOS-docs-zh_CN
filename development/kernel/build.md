# <!--Kernel in the build-->

# 构建内核

## <!--Commandline options {#options}-->

## 命令行选项 {#options}

<!--Kernel commandline options are declared using the-->
<!--[`kernel_cmdline`](/build/zbi/kernel_cmdline.gni) template:-->

内核命令行选项是定义在 [`kernel_cmdline`](/build/zbi/kernel_cmdline.gni) 模版上：

```gn
import("//build/zbi/kernel_cmdline.gni")

kernel_cmdline("foobar") {
  args = [ "foobar=true" ]
}
```

<!--A single target may include multiple options:-->

一个命令参数可以包含多个选项：

```gn
import("//build/zbi/kernel_cmdline.gni")

kernel_cmdline("debug") {
  args = [
    "debug.this=true",
    "debug.that=false",
  ]
}
```

<!--The resulting GN labels should then be inserted into the build graph via a GN
argument. Note that options will be taken into account if they are within the
dependency tree defined by such a GN argument.-->

生成的GN标签应该通过GN参数插入到构建图中。注意，如果这些选项在由这样一个GN参数定义在依赖树中，那么它们将被考虑在内



### <!--Specifying options in board or product files-->

### 指定版型（board）和产品（product）文件的选项

<!--In the [board](/boards) or [product](/products) file, add the label(s) for the
desired cmdline option(s) to [`board_bootfs_labels`](/build/board.gni) and
[`product_bootfs_labels`](/build/product.gni) respectively.-->

在 [版型](boards) 或者 [产品](products) 文件中，将所需cmdline 选项的标签添加到 [`board_bootfs_babels`](/build/board.gni) 和 [`product_bootfs_labels`](/build/product.gni) 

<!--To alter kernel options for the zedboot or recovery build, add the labels
respectively to [`board_zedboot_bootfs_labels`](/build/board.gni) and
[`board_recovery_bootfs_labels`](/build/board.gni).-->

要改变 zedboot 或着 恢复版本的内核选项，请将标签添加到 [`board_zedboot_bootfs_labels`](/build/board.gni) 和 [`board_recovery_bootfs_labels`](/build/board.gni)

### <!--Specifying options locally-->

### 在本地指定选项

Create a `BUILD.gn` file somewhere under `//local` to host the options targets.
Note that this folder is not tracked by git and therefore might not exist yet in
your checkout.

在 `//local` 下的某个地方创建一个 `BUILD.gn` 文件，以设置目标的选项。注意，这个文件夹不没有git 跟踪，因此可能当你检查的时候不存在。

<!--Use [`dev_bootfs_labels`](/build/dev.gni) to inject the options into the build
graph via `fx set`:-->

使用[`dev_bootfs_labels`](/build/dev.gni) 将选项注入到通过`fx set` 构建中，

```posix-terminal
fx set ... --args='dev_bootfs_labels=["//local/path/to/my:options"]'
```

To locally alter kernel options for the zedboot or recovery build, follow the
same procedure but use respectively
[`dev_zedboot_bootfs_labels`](/build/dev.gni) and
[`dev_recovery_bootfs_labels`](/build/dev.gni) to introduce the options into the
build.

要在本地更改 zedboot 和 恢复版本的内核选项，请遵循相同的步骤，但分别使用 [`dev_zedboot_bootfs_labels`](/build/dev.gni) 和 [`dev_recovery_bootfs_labels`](/build/dev.gni) 将选项引入构建。 

