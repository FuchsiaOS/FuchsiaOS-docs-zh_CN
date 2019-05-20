# 工具链

Fuchsia 的官方编译器是 Clang。

## 预先准备

你需要至少 3.8.0 版本的  [CMake](https://cmake.org/download/) 来执行这些命令。这是支持 Fuchsia 的第一个版本。

CMake 支持许多不同的构建系统，我们建议使用安装在你的系统上的 [Ninja](https://github.com/ninja-build/ninja/releases) 。

## 获取源代码

以下指令使用 `${LLVM_SRCDIR}` 来指向你 LLVM 源代码树的根目录，并假定其使用 [monorepo layout](https://llvm.org/docs/Proposals/GitHubMove.html#monorepo-variant)。
当使用这种结构时，每一个子项目都拥有自己的顶层目录。

[https://fuchsia.googlesource.com/third_party/llvm-project](https://fuchsia.googlesource.com/third_party/llvm-project) 
仓库通过 Git submodules 模拟了这种结构，并由 Gerrit 自动更新。在设置了 `${LLVM_SRCDIR}` 环境变量之后，你可以使用以下指令来下载此仓库包含所有子模块的内容：

```bash
LLVM_SRCDIR=${HOME}/llvm-project
git clone --recurse-submodules https://fuchsia.googlesource.com/third_party/llvm-project ${LLVM_SRCDIR}
```

要更新包括所有子模块在内的仓库内容，使用如下指令：

```bash
git pull --recurse-submodules
```

你也可以选择使用半官方的 monorepo
[https://github.com/llvm-project/llvm-project-20170507](https://github.com/llvm-project/llvm-project-20170507)
这个仓库由 LLVM 社区维护。这个仓库没有使用子模块，这意味着你可以使用标准 Git 工作流：

```bash
git clone https://github.com/llvm-project/llvm-project-20170507 ${LLVM_SRCDIR}
```

### Fuchsia SDK

在构建工具链和运行时库之前，你需要一个 Fuchsia SDK。我们假设 SDK 位于 `${SDK_DIR}` 变量所指向的目录中。

```bash
SDK_DIR=${HOME}/sdk/garnet
```

要下载最新版本的 SDK， 使用如下指令：

```bash
./buildtools/cipd install fuchsia/sdk/linux-amd64 latest -root ${SDK_DIR}
```

你可以选择性的使用以下命令，从源码构建 Garnet SDK：

```bash
./scripts/build-zircon.sh

gn gen --args='target_cpu="x64" fuchsia_packages=["garnet/packages/sdk/garnet"]' out/x64
ninja -C out/x64

gn gen --args='target_cpu="arm64" fuchsia_packages=["garnet/packages/sdk/garnet"]' out/arm64
ninja -C out/arm64

./scripts/sdk/create_layout.py --manifest out/x64/gen/garnet/public/sdk/garnet_molecule.sdk --output ${SDK_DIR}
./scripts/sdk/create_layout.py --manifest out/arm64/gen/garnet/public/sdk/garnet_molecule.sdk --output ${SDK_DIR} --overlay
```

## 构建 Clang

Clang CMake 构建系统支持引导构建方式（例如多阶段构建）。我们使用分两个阶段的引导构建方式来构建 Fuchsia Clang 编译器。

第一阶段是构建一个只适用于主机的编译器，它支持一些第二阶段需要的选项。第二阶段构建的编译器是提供给用户的完全优化的编译器。

配置这些编译器构建选项十分复杂。为了简化流程，Fuchsia Clang 构建设置作为 Clang 代码库的一部分，包含在了 CMake cache 文件中。

你可以使用以下指令为 Fuchsia 构建Clang 工具链。这些指令必须在一个独立的构建目录中运行（你必须创建一个）。这个目录可以是 `${LLVM_SRCDIR}` 的子目录，以便于使用 `LLVM_SRCDIR=..`；或者这个目录可以在任何位置，只要为 `LLVM_SRCDIR` 设置了指向构建目录的绝对或相对路径。

```bash
cmake -GNinja \
  -DLLVM_ENABLE_PROJECTS="clang;lld" \
  -DLLVM_ENABLE_RUNTIMES="compiler-rt;libcxx;libcxxabi;libunwind" \
  -DSTAGE2_FUCHSIA_SDK=${SDK_DIR} \
  -C ${LLVM_SRCDIR}/clang/cmake/caches/Fuchsia.cmake \
  ${LLVM_SRCDIR}/llvm
ninja stage2-distribution
```

为了为 Linux 引入编译器运行时库和 C++ 库，你需要使用 `LINUX_<架构名>_SYSROOT` 标志指向 sysroot 并标识主机类型。例如要为使用 Fuchsia 分支 sysroot 的 `x86_64-linux-gnu` 构建运行时库，你可以使用：

```bash
  -DBOOTSTRAP_LLVM_DEFAULT_TARGET_TRIPLE=x86_64-linux-gnu \
  -DSTAGE2_LINUX_x86_64-linux-gnu_SYSROOT=${FUCHSIA}/buildtools/linux-x64/sysroot \
```

要将构建输出的编译器安装到 `/usr/local`，使用如下命令：

```bash
ninja stage2-install-distribution
```

要在不将其安装到全局共享位置的情况下使用百年一起，你可以使用对其构建输出地址的引用如 `${LLVM_OBJDIR}/tools/clang/stage2-bins/bin` （其中 `LLVM_OBJDIR` 指你的 LLVM 构建目录）

*** 重点
**注意：**第二阶段的构建工作使用了 LTO（Link Time Optimization 链接时优化）来使输出的编译器有更好的运行时性能表现。LTO 常常需要占用很大的内存空间，而且这个过程是非常缓慢的。因此在例行工作中最好不要有这一项内容。
***

## 对 Clang 进行开发

当对 Clang 进行开发时，你可能希望使用一个更加适合增量式开发、更加灵活的构建配置。

最简单的构建 LLVM 的方式是使用以下指令：

```bash
cmake -GNinja \
  -DCMAKE_BUILD_TYPE=Debug \
  -DLLVM_ENABLE_PROJECTS="clang;lld" \
  ${LLVM_SRCDIR}/llvm
ninja
```

你可以改变 `LLVM_ENABLE_PROJECTS` 来使能更多的项目。要使能所有项目，使用以下指令：

```bash
  -DLLVM_ENABLE_PROJECTS="clang;lld;compiler-rt;libcxx;libcxxabi;libunwind"
```

类似的，你可以标记一些项目为运行时编译，它们将不会由主机上的编译器来编译，取而代之的是编译输出的新编译器：

```bash
  -DLLVM_ENABLE_PROJECTS="clang;lld" \
  -DLLVM_ENABLE_RUNTIMES="compiler-rt;libcxx;libcxxabi;libunwind" \
```

Clang 是一个大型项目，编译它需要很长的时间。为了缩短编译时间，我们建议在主机上使用 Clang 作为编译器，如果可能的话，使用 LLD 作为主机上的链接器。这些工具是由 LTO 编译的，他们在使用 PGO (Profile-Guided Optimizations 配置指导优化) 时有最好的运行时性能表现。

要将主机使用的编译器设置为 Clang，连接器设置为 LLD，你可以使用一下附加 flags：

```bash
  -DCMAKE_C_COMPILER=${CLANG_TOOLCHAIN_PREFIX}clang \
  -DCMAKE_CXX_COMPILER=${CLANG_TOOLCHAIN_PREFIX}clang++ \
  -DLLVM_ENABLE_LLD=ON
```

这些指令假设 `${CLANG_TOOL_CHAIN_PREFIX}` 指向 Clang 安装中的 `bin` 目录，最后带有斜线（正如 Zircon 构件中使用的 Make 变量那样）。例如，要在 Linux 下使用你的 Fuchsia 分支中的编译器：

```bash
CLANG_TOOLCHAIN_PREFIX=${FUCHSIA}/buildtools/linux-x64/clang/bin/
```

***重点
**注意：** Fuchsia Clang 安装只包括主机（Linux）的标准 libc++，所以你需要添加以下两个 flags 来避免链接器报错：
```bash
  -DCMAKE_EXE_LINKER_FLAGS="-ldl -lpthread" \
  -DCMAKE_SHARED_LINKER_FLAGS="-ldl -lpthread"
```
***

### 排查工具（Sanitizers）

大多数的排查工具可以通过添加 `LLVM_USE_SANITIZER=<sanitizer name>` 到 cmake 调用过程中在 LLVM 工具上使用。MSan 是一个例外，因为一些 LLVM 工具会触发误报。要构建支持 MSan 的版本，你需要先构建支持 MSan 的 libc++。你可以在同一个构建中做这些工作。要设置一个支持 MSan 的构建，首先需要在运行 CMake 时添加 `LLVM_USE_SANITIZER=Memory` 和 `LLVM_ENABLE_LIBCXX=ON` 参数。

```bash
cmake -GNinja \
  -DCMAKE_BUILD_TYPE=Debug \
  -DCMAKE_C_COMPILER=${CLANG_TOOLCHAIN_PREFIX}clang \
  -DCMAKE_CXX_COMPILER=${CLANG_TOOLCHAIN_PREFIX}clang++ \
  -DLLVM_ENABLE_PROJECTS="clang;lld;libcxx;libcxxabi;libunwind" \
  -DLLVM_USE_SANITIZER=Memory \
  -DLLVM_ENABLE_LIBCXX=ON \
  -DLLVM_ENABLE_LLD=ON \
  ${LLVM_SRCDIR}/llvm
```

通常此时你会运行 Ninja，但我们希望使用支持 Sanitizer 的 libc++ 来构建所有的东西。如果我们现在使用来自 `${CLANG_TOOLCHAIN_PREFIX}` 的 libc++ 来构建，它并不支持 Sanitizer。
所以首先我们只构建 cxx 和 cxxabi 目标。这些目标在动态链接 libcxx 时将替代位于 `${CLANG_TOOLCHAIN_PREFIX}` 中的那部分。

```bash
ninja cxx cxxabi
```

Now that we have a sanitized version of libc++ we can have our build use
it instead of the one from `${CLANG_TOOLCHAIN_PREFIX}` and then build
everything.
现在我们拥有了一个支持 Sanitizer 的 libc++ 版本。我们可以使用它而不是来自 `${CLANG_TOOLCHAIN_PREFIX}` 的 libc++ 来构建所有东西。

```bash
ninja
```

将上述指令合并就是：

```bash
cmake -GNinja \
  -DCMAKE_BUILD_TYPE=Debug \
  -DCMAKE_C_COMPILER=${CLANG_TOOLCHAIN_PREFIX}clang \
  -DCMAKE_CXX_COMPILER=${CLANG_TOOLCHAIN_PREFIX}clang++ \
  -DLLVM_USE_SANITIZER=Address \
  -DLLVM_ENABLE_LIBCXX=ON \
  -DLLVM_ENABLE_LLD=ON \
  ${LLVM_SRCDIR}/llvm
ninja libcxx libcxxabi
ninja
```

### 【Google 员工专用】Goma

确保你的电脑上安装了 Goma 来加速构建。Goma 通过在多台计算机上进行分布式编译来加速构建。如果你将 `${GOMA_DIR}` 指向 Goma 安装目录（默认位于 `${HOME}/goma`），你可以通过下列附加 flags 来使能 Goma：

```bash
  -DCMAKE_C_COMPILER_LAUNCHER=${GOMA_DIR}/gomacc \
  -DCMAKE_CXX_COMPILER_LAUNCHER=${GOMA_DIR}/gomacc \
  -DLLVM_PARALLEL_LINK_JOBS=${LINK_JOBS}
```

链接工作的数量依赖于 RAM 的大小，要进行 LTO 构建，你需要为每一个工作准备至少 10GB 内存。

要使用 Goma 来构建 Clang，使用如下指令：
```bash
ninja -j${JOBS}
```

在 macOS 上使用 `-j100`，在 Linux 上使用 `-j1000`。你需要按照实际的硬件情况和工作量来调优工作量。

*** 重点
**注意：** 要使用 Goma，在你的主机上需要有一个支持 Goma 的编译器，例如 Fuchsia Clang installation。上文有关于如何配置 LLVM 构建来使用不同的主机编译器的内容。
***

要验证你的编译器是否适用于 Goma，你可以设置 `GOMA_USE_LOCAL=0 GOMA_FALLBACK=0` 环境变量。如果编译器不支持 Goma，你将会看到一条错误信息。

### Fuchsia 配置

当开发 Fuchsia 上的 Clang 时，你可以使用缓存文件来测试 Fuchsia 配置，但只在关闭 LTO 的情况下运行第二阶段构建，这样将在大幅减少编译时间的同时，省去手动配置所有选项的麻烦：

```bash
cmake -G Ninja -DCMAKE_BUILD_TYPE=Debug \
  -DCMAKE_C_COMPILER=${CLANG_TOOLCHAIN_PREFIX}clang \
  -DCMAKE_CXX_COMPILER=${CLANG_TOOLCHAIN_PREFIX}clang++ \
  -DLLVM_ENABLE_LTO=OFF \
  -DLLVM_ENABLE_PROJECTS="clang;lld" \
  -DLLVM_ENABLE_RUNTIMES="compiler-rt;libcxx;libcxxabi;libunwind" \
  -DLLVM_DEFAULT_TARGET_TRIPLE=x86_64-linux-gnu \
  -DLINUX_x86_64-linux-gnu_SYSROOT=${FUCHSIA}/buildtools/linux-x64/sysroot \
  -DFUCHSIA_SDK=${SDK_DIR} \
  -C ${LLVM_SRCDIR}/clang/cmake/caches/Fuchsia-stage2.cmake \
  ${LLVM_SRCDIR}/llvm
ninja distribution
```

使用 Goma 来进一步缩短时间：

```bash
cmake -G Ninja -DCMAKE_BUILD_TYPE=Debug \
  -DCMAKE_C_COMPILER=${CLANG_TOOLCHAIN_PREFIX}clang \
  -DCMAKE_CXX_COMPILER=${CLANG_TOOLCHAIN_PREFIX}clang++ \
  -DCMAKE_C_COMPILER_LAUNCHER=${GOMA_DIR}/gomacc \
  -DCMAKE_CXX_COMPILER_LAUNCHER=${GOMA_DIR}/gomacc \
  -DCMAKE_EXE_LINKER_FLAGS="-ldl -lpthread" \
  -DCMAKE_SHARED_LINKER_FLAGS="-ldl -lpthread" \
  -DLLVM_PARALLEL_LINK_JOBS=${LINK_JOBS} \
  -DLLVM_ENABLE_LTO=OFF \
  -DLLVM_ENABLE_PROJECTS="clang;lld" \
  -DLLVM_ENABLE_RUNTIMES="compiler-rt;libcxx;libcxxabi;libunwind" \
  -DLLVM_DEFAULT_TARGET_TRIPLE=x86_64-linux-gnu \
  -DLINUX_x86_64-linux-gnu_SYSROOT=${FUCHSIA}/buildtools/linux-x64/sysroot \
  -DFUCHSIA_SDK=${SDK_DIR} \
  -C ${LLVM_SRCDIR}/clang/cmake/caches/Fuchsia-stage2.cmake \
  ${LLVM_SRCDIR}/llvm
ninja distribution -j${JOBS}
```

## 测试 Clang

要运行 Clang 测试，你可以使用 `check-<组件名>`目标：

```
ninja check-llvm check-clang
```

你也可以使用 `check-all` 来运行所有测试，但值得注意的是，这将需要很长时间，具体时间长短取决于你构建中使能的项目数量。

### 使用定制的 Clang 来构建 Fuchsia

你可以使用位于
`${LLVM_OBJDIR}/bin/`，或者
`${LLVM_OBJDIR}/tools/clang/stage2-bins/bin/` (取决于你执行了两阶段还是单阶段的构建，这两种方式生成的二进制文件位于不同的位置) 的 Clang 立即开始构建测试二进制文件。然而，如果你想用你自己的 Clang 来构建 Fuchsia，你将需要设置一些额外的参数/变量。



如果你只对构建 Zircon 感兴趣，设置如下的 Make 变量：

```bash
make USE_CLANG=true CLANG_TOOLCHAIN_PREFIX=${CLANG_DIR}
```

`${CLANG_DIR}` 指向你的 Clang 所在位置的 `bin` 目录，
e.g. `${LLVM_OBJDIR}/bin/`。

*** 重要
**注意：** 末尾一定要加斜杠。
***

然后像往常一样运行 `fx build-zircon`。

要构建 Zircon 以上的层，只需向 `fx set` 传递：
`--args clang_prefix="${CLANG_DIR}"`，然后像往常一样运行 `fx build`。

*** 重要
**注意：** 由于 `fx full-build` 指令包含了对  Zircon 的构建，要执行完整构建，你同时需要完成对  Zircon 构建必要的环境变量的设置。
***

为了确保每一次你执行构建的时候，环境变量都已经设置好，你可以执行 `fx set`，然后手动编辑你的 `${FUCHSIA_SOURCE}/.config` 文件，向其后添加如下行：

```bash
export USE_CLANG=true CLANG_TOOLCHAIN_PREFIX=${LLVM_OBJDIR}/bin/
```

## 额外资源：

Documentation:
* [Getting Started with the LLVM System](http://llvm.org/docs/GettingStarted.html)
* [Building LLVM with CMake](http://llvm.org/docs/CMake.html)
* [Advanced Build Configurations](http://llvm.org/docs/AdvancedBuilds.html)

Talks:
* [2016 LLVM Developers’ Meeting: C. Bieneman "Developing and Shipping LLVM and Clang with CMake"](https://www.youtube.com/watch?v=StF77Cx7pz8)
* [2017 LLVM Developers’ Meeting: Petr Hosek "Compiling cross-toolchains with CMake and runtimes build"](https://www.youtube.com/watch?v=OCQGpUzXDsY)
