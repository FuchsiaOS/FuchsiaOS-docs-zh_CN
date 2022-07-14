# Build Clang toolchain

Fuchsia is using Clang as the official compiler.

## Prerequisites

You need [CMake](https://cmake.org/download/) version 3.13.4 or newer to
execute these commands. This is the [minimum required version](https://reviews.llvm.org/rGafa1afd4108)
to build LLVM.

While CMake supports different build systems, it is recommended to use
[Ninja](https://github.com/ninja-build/ninja/releases).

Both should be present in your Fuchsia checkout as prebuilts. The commands below
assume that `cmake` and `ninja` are in your `PATH`:

```
export PATH=${FUCHSIA}/prebuilt/third_party/cmake/${platform}/bin:${PATH}
export PATH=${FUCHSIA}/prebuilt/third_party/ninja/${platform}/bin:${PATH}
```

### Getting Source

The example commands below use `${LLVM_SRCDIR}` to refer to the root of
your LLVM source tree checkout. You can use the official monorepo
[https://github.com/llvm/llvm-project](https://github.com/llvm/llvm-project)
maintained by the LLVM community:

```bash
LLVM_SRCDIR=${HOME}/llvm/llvm-project
git clone https://github.com/llvm/llvm-project ${LLVM_SRCDIR}
```

Note: It is recommended checking out to the revision that's currently used for
Fuchsia.
The latest upstream revision may be broken or fail to build Fuchsia, whereas it is
guaranteed that the prebuilt revision can always build Fuchsia. This
revision can be found in `[//integration/toolchain]`. Search for the package
`fuchsia/third_party/clang/${platform}`, and checkout the `git_revision`
associated with it.

```bash
cd ${LLVM_SRCDIR}
git checkout ${REVISON_NUMBER}
```

### Fuchsia IDK

Before building the runtime libraries that are built along with the
toolchain, you need a Fuchsia [IDK](/development/idk)
(formerly known as the SDK).
The IDK must be located in the directory pointed to by the `${IDK_DIR}`
variable:

```bash
IDK_DIR=${HOME}/fuchsia-idk
```

To download the latest IDK, you can use the following:

```bash
# For Linux
cipd install fuchsia/sdk/core/linux-amd64 latest -root ${IDK_DIR}

# For macOS
cipd install fuchsia/sdk/core/mac-amd64 latest -root ${IDK_DIR}
```

### Sysroot for Linux

To include compiler runtimes and C++ library for Linux, download the sysroot.
It must be located in the directory pointed by the `${SYSROOT_DIR}` variable.

```bash
SYSROOT_DIR=${HOME}/fuchsia-sysroot/
```

To download the latest sysroot, you can use the following:

```bash
cipd install fuchsia/third_party/sysroot/linux latest -root ${SYSROOT_DIR}
```

{% dynamic if user.is_googler %}

### [Googlers only] Goma

Goma is a service for accelerating builds by distributing compilations across
many machines. Googlers should ensure Goma is installed on your machine for faster
builds. If you have Goma installed in `${GOMA_DIR}` (which should be provided in
`//prebuilt/third_party/goma/${platform}`),
you can enable Goma by adding these extra CMake flags to your CMake invocation:

```bash
  -DCMAKE_C_COMPILER_LAUNCHER=${GOMA_DIR}/gomacc \
  -DCMAKE_CXX_COMPILER_LAUNCHER=${GOMA_DIR}/gomacc \
```

Then you can take advantage of Goma by allowing multiple `ninja` jobs to run in
parallel:

```bash
ninja -j1000
```

Use `-j100` for Goma on macOS and `-j1000` for Goma on Linux. You may
need to tune the job count to suit your particular machine and workload.

Warning: The examples below assume you can use Goma. If you cannot use Goma, do not
add the provided CMake flags or use an absurdly high number of jobs.

Note: In order to use Goma, you need a host compiler that is
supported by Goma such as the Fuchsia Clang installation.
To verify your compiler is available on Goma, you can set
`GOMA_USE_LOCAL=0 GOMA_FALLBACK=0` environment variables. If the
compiler is not available, you will see an error.

{% dynamic endif %}

## Building a Clang Toolchain for Fuchsia

The Clang CMake build system supports bootstrap (aka multi-stage)
builds. Fuchsia uses [two-stage bootstrap build](#two-stage-build) for the
Clang compiler.
However, for toolchain related development it is recommended to use
the [single-stage build](#single-stage-build).

If your goal is to experiment with clang, the single-stage build is likely what you are looking for.
The first stage compiler is a host-only compiler with some options set
needed for the second stage. The second stage compiler is the fully
optimized compiler intended to ship to users.

Setting up these compilers requires a lot of options. To simplify the
configuration the Fuchsia Clang build settings are contained in CMake
cache files, which are part of the Clang codebase (`Fuchsia.cmake` and
`Fuchsia-stage2.cmake`).

In the following CMake invocations, `${CLANG_TOOLCHAIN_PREFIX}` refers to the directory
of binaries from a previous Clang toolchain. Normally, this refers to the
current toolchain shipped with Fuchsia, but any references to binaries
from this directory could theoretically be replaced with one's own binaries.

```bash
# FUCHSIA_SRCDIR refers to the root directory of your Fuchsia source tree
CLANG_TOOLCHAIN_PREFIX=${FUCHSIA_SRCDIR}/prebuilt/third_party/clang/linux-x64/bin/
```

Note: Clang must be built in a separate build directory. The directory itself
can be a subdirectory or in a whole other path.

```bash
mkdir llvm-build
mkdir llvm-install  # For placing stripped binaries here
INSTALL_DIR=${pwd}/llvm-install
cd llvm-build
```

### Single Stage Build Fuchsia Configuration {#single-stage-build}

When developing Clang for Fuchsia, you can use the cache file to
test the Fuchsia configuration, but run only the second stage, with LTO
disabled, which gives you a faster build time suitable even for
incremental development, without having to manually specify all options:

```bash
cmake -G Ninja -DCMAKE_BUILD_TYPE=Debug \
  -DCMAKE_C_COMPILER=${CLANG_TOOLCHAIN_PREFIX}clang \
  -DCMAKE_CXX_COMPILER=${CLANG_TOOLCHAIN_PREFIX}clang++ \
  -DCMAKE_ASM_COMPILER=${CLANG_TOOLCHAIN_PREFIX}clang \
  -DCMAKE_C_COMPILER_LAUNCHER=${GOMA_DIR}/gomacc \
  -DCMAKE_CXX_COMPILER_LAUNCHER=${GOMA_DIR}/gomacc \
  -DCMAKE_ASM_COMPILER_LAUNCHER=${GOMA_DIR}/gomacc \
  -DLLVM_ENABLE_LTO=OFF \
  -DLINUX_x86_64-unknown-linux-gnu_SYSROOT=${SYSROOT_DIR} \
  -DLINUX_aarch64-unknown-linux-gnu_SYSROOT=${SYSROOT_DIR} \
  -DFUCHSIA_SDK=${IDK_DIR} \
  -DCMAKE_INSTALL_PREFIX= \
  -C ${LLVM_SRCDIR}/clang/cmake/caches/Fuchsia-stage2.cmake \
  ${LLVM_SRCDIR}/llvm
ninja distribution  -j1000  # Build the distribution
```

If the above fails with an error related to Ninja, then you may need to add
`ninja` to your PATH. You can find the prebuilt executable at
`//prebuilt/third_party/ninja/${platform}/bin`.

`ninja distribution` should be enough for building all binaries, but the Fuchsia
build assumes some libraries are stripped so `ninja
install-distribution-stripped` is necessary.

Caution: Due to a [bug in Clang](https://bugs.llvm.org/show_bug.cgi?id=44097),
builds with assertions enabled might crash while building Fuchsia. As a
workaround, you can disable Clang assertions by setting
`-DLLVM_ENABLE_ASSERTIONS=OFF` or using a release build
(`-DCMAKE_BUILD_TYPE=Release`).

### Two-Stage Build Fuchsia Configuration {#two-stage-build}

This is roughly equivalent to what is run on the prod builders and used to build
a toolchain that Fuchsia ships to users.

```bash
cmake -GNinja \
  -DCMAKE_C_COMPILER=${CLANG_TOOLCHAIN_PREFIX}/clang \
  -DCMAKE_CXX_COMPILER=${CLANG_TOOLCHAIN_PREFIX}/clang++ \
  -DCMAKE_ASM_COMPILER=${CLANG_TOOLCHAIN_PREFIX}/clang \
  -DCMAKE_C_COMPILER_LAUNCHER=${GOMA_DIR}/gomacc \
  -DCMAKE_CXX_COMPILER_LAUNCHER=${GOMA_DIR}/gomacc \
  -DCMAKE_ASM_COMPILER_LAUNCHER=${GOMA_DIR}/gomacc \
  -DCMAKE_INSTALL_PREFIX= \
  -DSTAGE2_LINUX_aarch64-unknown-linux-gnu_SYSROOT=${SYSROOT_DIR} \
  -DSTAGE2_LINUX_x86_64-unknown-linux-gnu_SYSROOT=${SYSROOT_DIR} \
  -DSTAGE2_FUCHSIA_SDK=${IDK_DIR} \
  -C ${LLVM_SRCDIR}/clang/cmake/caches/Fuchsia.cmake \
  ${LLVM_SRCDIR}/llvm
ninja stage2-distribution -j1000
DESTDIR=${INSTALL_DIR} ninja stage2-install-distribution-stripped -j1000
```

Note: The second stage build uses LTO (Link Time Optimization) to
achieve better runtime performance of the final compiler. LTO often
requires a large amount of memory and is very slow. Therefore it may not
be very practical for day-to-day development.

### runtime.json

If the Fuchsia build fails due to a missing `runtime.json` file, you must generate a new `runtime.json` file by running the following command:

```bash
python3 ${FUCHSIA_SRCDIR}/scripts/clang/generate_runtimes.py  \
  --clang-prefix ${INSTALL_DIR} --sdk-dir ${IDK_DIR}          \
  --build-id-dir ${INSTALL_DIR}/lib/.build-id > ${INSTALL_DIR}/lib/runtime.json
```

The generated file contains relative paths used by the Fuchsia build to know where
various libraries from the toolchain are located.

### Putting it All Together

Copy-paste code for building a single-stage toolchain. This code can be run
from inside your LLVM build directory and assumes a linux environment.

```bash
cd ${LLVM_BUILD_DIR}  # The directory your toolchain will be installed in

# Environment setup
FUCHSIA_SRCDIR=${HOME}/fuchsia/  # Replace with wherever Fuchsia lives
LLVM_SRCDIR=${HOME}/llvm/llvm-project  # Replace with wherever llvm-project lives
IDK_DIR=${HOME}/fuchsia-idk/
SYSROOT_DIR=${HOME}/fuchsia-sysroot/
CLANG_TOOLCHAIN_PREFIX=${FUCHSIA_SRCDIR}/prebuilt/third_party/clang/linux-x64/bin/
GOMA_DIR=${FUCHSIA_SRCDIR}/prebuilt/third_party/goma/linux-x64/

# Download necessary dependencies
cipd install fuchsia/sdk/core/linux-amd64 latest -root ${IDK_DIR}
cipd install fuchsia/third_party/sysroot/linux latest -root ${SYSROOT_DIR}

# CMake invocation
cmake -G Ninja -DCMAKE_BUILD_TYPE=Release \
  -DCMAKE_C_COMPILER=${CLANG_TOOLCHAIN_PREFIX}clang \
  -DCMAKE_CXX_COMPILER=${CLANG_TOOLCHAIN_PREFIX}clang++ \
  -DCMAKE_C_COMPILER_LAUNCHER=${GOMA_DIR}/gomacc \
  -DCMAKE_CXX_COMPILER_LAUNCHER=${GOMA_DIR}/gomacc \
  -DLLVM_ENABLE_LTO=OFF \
  -DLINUX_x86_64-unknown-linux-gnu_SYSROOT=${SYSROOT_DIR} \
  -DLINUX_aarch64-unknown-linux-gnu_SYSROOT=${SYSROOT_DIR} \
  -DFUCHSIA_SDK=${IDK_DIR} \
  -DCMAKE_INSTALL_PREFIX= \
  -C ${LLVM_SRCDIR}/clang/cmake/caches/Fuchsia-stage2.cmake \
  ${LLVM_SRCDIR}/llvm

# Build and strip binaries and place them in the install directory
ninja distribution -j1000
DESTDIR=${INSTALL_DIR} ninja install-distribution-stripped -j1000

# Generate runtime.json

python3 ${FUCHSIA_SRCDIR}/scripts/clang/generate_runtimes.py    \
  --clang-prefix ${INSTALL_DIR} --sdk-dir ${IDK_DIR}            \
  --build-id-dir ${INSTALL_DIR}/lib/.build-id > ${INSTALL_DIR}/lib/runtime.json
```

### Building Fuchsia with a Custom Clang

To specify a custom clang toolchain for building Fuchsia, pass
`--args clang_prefix=\"${INSTALL_DIR}/bin\" --no-goma`
to `fx set` command and run `fx build`.

```bash
fx set core.x64 --args=clang_prefix=\"${INSTALL_DIR}/bin\" --no-goma
fx build
```

This file contains relative paths used by the Fuchsia build to know where
various libraries from the toolchain are located.

Note: If you make another change to Clang after building Fuchsia with a previous
version of Clang, re-running `fx build` may not always guarantee that all necessary
targets will be built with the new Clang. For this case, should instead run `fx
clean-build`, which will rebuild everything but definitely use the new Clang.

## Developing Clang

When developing Clang, you may want to use a setup that is more suitable for
incremental development and fast turnaround time.

The simplest way to build LLVM is to use the following commands:

```bash
cmake -GNinja \
  -DCMAKE_BUILD_TYPE=Debug \
  -DLLVM_ENABLE_PROJECTS="clang;clang-tools-extra;lld" \
  ${LLVM_SRCDIR}/llvm
ninja
```

You can enable additional projects using the `LLVM_ENABLE_PROJECTS`
variable. To enable all common projects, you would use:

```bash
  -DLLVM_ENABLE_PROJECTS="clang;clang-tools-extra;lld;compiler-rt;libcxx;libcxxabi;libunwind"
```

Similarly, you can also enable some projects to be built as runtimes
which means these projects will be built using the just-built rather
than the host compiler:

```bash
  -DLLVM_ENABLE_PROJECTS="clang;clang-tools-extra;lld" \
  -DLLVM_ENABLE_RUNTIMES="compiler-rt;libcxx;libcxxabi;libunwind" \
```

Both `LLVM_ENABLE_PROJECTS` and `LLVM_ENABLE_RUNTIMES` are already set in the
CMake cache files, so you normally don't need to set these unless you would
like to explicitly add more projects or runtimes.

Clang is a large project and compiler performance is absolutely critical. To
reduce the build time, it is recommended to use Clang as a host compiler, and if
possible, LLD as a host linker. These should be ideally built using LTO and
for best possible performance also using Profile-Guided Optimizations (PGO).

To set the host compiler to Clang and the host linker to LLD, you can
use the following extra flags:

```bash
  -DCMAKE_C_COMPILER=${CLANG_TOOLCHAIN_PREFIX}clang \
  -DCMAKE_CXX_COMPILER=${CLANG_TOOLCHAIN_PREFIX}clang++ \
  -DLLVM_ENABLE_LLD=ON
```

This assumes that `${CLANG_TOOLCHAIN_PREFIX}` points to the `bin` directory
of a Clang installation, with a trailing slash (as this Make variable is used
in the Zircon build). For example, to use the compiler from your Fuchsia
checkout (on Linux):

```bash
CLANG_TOOLCHAIN_PREFIX=${FUCHSIA}/prebuilt/third_party/clang/linux-x64/bin/
```

Note: To build Fuchsia, you need a stripped version of the toolchain runtime
binaries. Use `DESTDIR=${INSTALL_DIR} ninja install-distribution-stripped`
to get a stripped install and then point your build configuration to
`${INSTALL_DIR}/bin` as your toolchain.

### Sanitizers

Most sanitizers can be used on LLVM tools by adding
`LLVM_USE_SANITIZER=<sanitizer name>` to your cmake invocation. MSan is
special however because some LLVM tools trigger false positives. To
build with MSan support you first need to build libc++ with MSan
support. You can do this in the same build. To set up a build with MSan
support first run CMake with `LLVM_USE_SANITIZER=Memory` and
`LLVM_ENABLE_LIBCXX=ON`.

```bash
cmake -GNinja \
  -DCMAKE_BUILD_TYPE=Debug \
  -DCMAKE_C_COMPILER=${CLANG_TOOLCHAIN_PREFIX}clang \
  -DCMAKE_CXX_COMPILER=${CLANG_TOOLCHAIN_PREFIX}clang++ \
  -DLLVM_ENABLE_PROJECTS="clang;clang-tools-extra;lld;libcxx;libcxxabi;libunwind" \
  -DLLVM_USE_SANITIZER=Memory \
  -DLLVM_ENABLE_LIBCXX=ON \
  -DLLVM_ENABLE_LLD=ON \
  ${LLVM_SRCDIR}/llvm
```

Normally you would run Ninja at this point but we want to build
everything using a sanitized version of libc++ but if we build now it
will use libc++ from `${CLANG_TOOLCHAIN_PREFIX}`, which isn't sanitized.
So first we build just the cxx and cxxabi targets. These will be used in
place of the ones from `${CLANG_TOOLCHAIN_PREFIX}` when tools
dynamically link against libcxx

```bash
ninja cxx cxxabi
```

Now that you have a sanitized version of libc++ you can set your build to use
it instead of the one from `${CLANG_TOOLCHAIN_PREFIX}` and then build
everything.

```bash
ninja
```

Putting that all together:

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

## Testing Clang

To run Clang tests, you can use the `check-<component>` target:

```bash
ninja check-llvm check-clang
```

You can all use `check-all` to run all tests, but keep in mind that this
can take significant amount of time depending on the number of projects
you have enabled in your build.

To test only one specific test, you can use the environment variable
`LIT_FILTER`. If the path to the test is `clang/test/subpath/testname.cpp`, you
can use:

```bash
LIT_FILTER=testname.cpp ninja check-clang
```

The same trick can be applied for running tests in other sub-projects by
specifying different a different `check-<component>`.

{% dynamic if user.is_googler %}

## [Googlers only] Building Fuchsia with custom Clang on bots

Fuchsia's infrastructure has support for using a non-default version of Clang
to build. Only Clang instances that have been uploaded to CIPD or Isolate are
available for this type of build, and so any local changes must land in
upstream and be built by the CI or production toolchain bots.

You will need the infra codebase and prebuilts. Directions for checkout are on
the infra page.

To trigger a bot build with a specific revision of Clang, you will need the Git
revision of the Clang with which you want to build. This is on the [CIPD page](https://chrome-infra-packages.appspot.com/p/fuchsia/clang),
or can be retrieved using the CIPD CLI. You can then run the following command:

```bash
export FUCHSIA_SOURCE=<path_to_fuchsia>
export BUILDER=<builder_name>
export REVISION=<clang_revision>

export INFRA_PREBUILTS=${FUCHSIA_SOURCE}/fuchsia-infra/prebuilt/tools

cd ${FUCHSIA_SOURCE}/fuchsia-infra/recipes

${INFRA_PREBUILTS}/led get-builder 'luci.fuchsia.ci:${BUILDER}' | \
${INFRA_PREBUILTS}/led edit-recipe-bundle -O | \
jq '.userland.recipe_properties."$infra/fuchsia".clang_toolchain.type="cipd"' | \
jq '.userland.recipe_properties."$infra/fuchsia".clang_toolchain.instance="git_revision:${REVISION}"' | \
${INFRA_PREBUILTS}/led launch
```

It will provide you with a link to the BuildBucket page to track your build.

You will need to run `led auth-login` prior to triggering any builds, and may need to
file an infra ticket to request access to run led jobs.

## [Googlers Only] Downloading Toolchains from CAS

Our Clang Toolchain CI builders upload all build artifacts to Content Addressed Storage (CAS).
It provides a convenient way to quickly download a specific toolchain without having to build from scratch.
This can greatly speedup investigations into toolchain issues, since you can avoid the long LLVM build times, on top of building Fuchsia.

Below is an example of how download a specific toolchain into a corpus directory using `cas`:

```bash
$ ${INFRA_PREBUILTS}/cas download -cas-instance chromium-swarm -digest \
    ad53e1f315a849955190594fde6b07e11e76b40563db5779fcc69d6a6e04dc71/267 -dir corpus
```
In the example above the `-digest` field is passed a unique id, which is used by the `cas` tool to fetch the correct artifacts.
The `digest` can be obtained from Fuchsia's CI builder by selecting the builder you want the toolchain from, then expanding the `clang`->`cas`->`archive` fields and then clicking on the `CAS_UI` link.
The resulting page will give show you some information about the CAS upload, including the digest.

{% dynamic endif %}

## Downloading Toolchains from CAS

Our Clang Toolchain CI builders upload all build artifacts to Content Addressed Storage (CAS).
It provides a convenient way to quickly download a specific toolchain without having to build from scratch.
This can greatly speedup investigations into toolchain issues, since you can avoid the long LLVM build times, on top of building Fuchsia.

Below is an example of how to install the `cas` tool from scratch and download a specific toolchain into a corpus directory:

```bash
$ cipd install infra/tools/luci/cas/linux-amd64 latest -root luci
$ ./luci/cas download -cas-instance chromium-swarm -digest \
    ad53e1f315a849955190594fde6b07e11e76b40563db5779fcc69d6a6e04dc71/267 -dir corpus
```
In the example above the `-digest` field is passed a unique id, which is used by the `cas` tool to fetch the correct artifacts.
The `digest` can be obtained from Fuchsia's CI builder by selecting the builder you want the toolchain from, then expanding the `clang`->`cas`->`archive` fields and then clicking on the `CAS_UI` link.
The resulting page will give show you some information about the CAS upload, including the digest.


## Useful CMake Flags

There are many other [CMake flags](https://llvm.org/docs/CMake.html#id11) that
are useful for building, but these are some that may be useful for toolchain
building.

### `-DLLVM_PARALLEL_LINK_JOBS`

Increase the number of link jobs that can be run in parallel (locally). The number of
link jobs is dependent on RAM size. For LTO build you will
need at least 10GB for each job.

## Additional Resources

Documentation:

* [Getting Started with the LLVM System](http://llvm.org/docs/GettingStarted.html)
* [Building LLVM with CMake](http://llvm.org/docs/CMake.html)
* [Advanced Build Configurations](http://llvm.org/docs/AdvancedBuilds.html)

Talks:

* [2016 LLVM Developers’ Meeting: C. Bieneman "Developing and Shipping LLVM and Clang with CMake"](https://www.youtube.com/watch?v=StF77Cx7pz8)
* [2017 LLVM Developers’ Meeting: Petr Hosek "Compiling cross-toolchains with CMake and runtimes build"](https://www.youtube.com/watch?v=OCQGpUzXDsY)

[//prebuilt/integration]: https://fuchsia.googlesource.com/integration/+/HEAD/prebuilts
