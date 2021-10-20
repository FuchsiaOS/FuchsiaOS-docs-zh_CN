
**此文章是为了方便想接触Fuchsia,但不知道从哪里下手朋友。**

@[TOC](Fuchsia的构建)
由于Fuchsia目前还处于开发阶段，所以fuchsia需要放在linux/macos上的模拟器进行试运行。
我自己的开发环境是macos, 后面的的所有命令行都默认在macos环境下。**(M1目前不支持)**

# 构建主要分为四个步骤：

##  1.从Fuchsia官方的github拉下源码

### 1.1下载

打开自己的终端(terminal), 输入以下命令：

第一次用的拉的是国内镜像源码，但可能有些问题。
这次是直接拉取的官方源码，但官方源码是需要代理才能访问，部分有代理的同学可能也会出现无法连接的问题，这是因为终端不是自动被代理的，所以需要进行终端代理。
终端代理教程：[终端使用代理加速的正确方式（Shadowsocks） - SegmentFault 思否](https://segmentfault.com/a/1190000039686752)

```javascript
curl -s "https://fuchsia.googlesource.com/fuchsia/+/HEAD/scripts/bootstrap?format=TEXT" | base64 --decode | bash
```

输入命令后，在正常连接的情况下， 可能需要30-60分钟的时间来下载源码，这时候只需要耐心等待或拿着手机看看算法题。

### 1.2设置环境变量

拉下源码后， 需要配置一下文件~/.bash_profile 如果是zsh用户，需要在 ~/.zprofile里面配置。
在这里我们使用nano来编辑文件：

```javascript
nano ~/.bash_profile
```

然后在终端界面输入两行命令：

```javascript
export PATH=~/fuchsia/.jiri_root/bin:$PATH
source ~/fuchsia/scripts/fx-env.sh
```

然后更新环境变量

```javascript
source ~/.bash_profile
```

检查jiri和fx命令是否可用：

```javascript
jiri help
```

```javascript
fx help
```

##  2.配置和构建Fuchsia

第二步的前提都是在第一步的基础上进行的。

### 2.1 Set

我们可以set两种产品：core 和 workstation。 我推荐使用core， 因为core需要的编译的时间较短，而且目前需要用到的功能core都可以满足。

```javascript
fx set core.qemu-x64
```

或

```javascript
fx set workstation.x64
```

set完毕后，直接进行编译。

### 2.1 build

```javascript
fx build
```

##  3.开启Fuchsia 模拟器

### 3.1 开启FEMU

FEMU 叫做fuchsia模拟器，是基于安卓模拟器（ Android Emulator (AEMU)）的。
简单输入一下命令行便可以启动FEMU。（过程中可能会出现一个权限设置，直接允许就好）

```javascript
fx vdl start
```

### 3.2 配置设备

开启FEMU后， 终端界面会要求用户输入set-device：

```javascript
fx set-device 127.0.0.1:SSH_PORT
// SSH_PORT 为ssh端口 一般情况会直接显示
//比如：To support fx tools on emulator, please run "fx set-device 127.0.0.1:50365"
```

此命令是用于终端和FEMU的连接，便于终端直接操作fuchsia环境。

##  4.测试Fuchsia

### 4.1简介

前三步实际上已经完成了fuchsia的所有配置，紧接着我们就可以测试fuchsia。

这里需要了解的一个概念是：
组件（component）是fuchsia里的最小单元执行单元，包（package）是fuchsia发布的软件单元。

想要在fuchsia内部执行一些操作，就需要一个组件来执行，这里我们简单的打印出一些文字。
以下是文件结构：![请添加图片描述](https://img-blog.csdnimg.cn/ebca4c9cf73e46969a0fd79ebcf5a21b.png?x-oss-process=image/watermark,type_ZHJvaWRzYW5zZmFsbGJhY2s,shadow_50,text_Q1NETiBA5b-r6L-b55CD5ZWK,size_20,color_FFFFFF,t_70,g_se,x_16)

### 4.2BUILD.gn

![在这里插入图片描述](https://img-blog.csdnimg.cn/399ab12855d548cb83633b772e42dff7.png?x-oss-process=image/watermark,type_ZHJvaWRzYW5zZmFsbGJhY2s,shadow_50,text_Q1NETiBA5b-r6L-b55CD5ZWK,size_20,color_FFFFFF,t_70,g_se,x_16)

### 4.3 fky_hello.cc

![在这里插入图片描述](https://img-blog.csdnimg.cn/1bc3ea9ae0dc4d0587fa2a22bee07915.png?x-oss-process=image/watermark,type_ZHJvaWRzYW5zZmFsbGJhY2s,shadow_50,text_Q1NETiBA5b-r6L-b55CD5ZWK,size_20,color_FFFFFF,t_70,g_se,x_16)

### 4.4 fky_hello.cmx

![在这里插入图片描述](https://img-blog.csdnimg.cn/bbf2139487de4abf9643231c459c2ac1.png?x-oss-process=image/watermark,type_ZHJvaWRzYW5zZmFsbGJhY2s,shadow_50,text_Q1NETiBA5b-r6L-b55CD5ZWK,size_20,color_FFFFFF,t_70,g_se,x_16)

配置好以上文件后， 回到之前的（3.2 set- device后）运行以下命令进行编译（set, build）：

```javascript
fx set core.qemu-x64 --with //examples/fky_hello
```

```javascript
fx build
```

编译完毕后，直接执行组件：

```javascript
fx shell run fuchsia-pkg://fuchsia.com/fky_hello#meta/fky_hello.cmx
```

此时终端会显示如下字符 “First time run Fuchsia!”， 意味着组建运行成功。

![在这里插入图片描述](https://img-blog.csdnimg.cn/2f5fe694a1e14b2eb5f9dcb64e1cd734.png?x-oss-process=image/watermark,type_ZHJvaWRzYW5zZmFsbGJhY2s,shadow_50,text_Q1NETiBA5b-r6L-b55CD5ZWK,size_20,color_FFFFFF,t_70,g_se,x_16)

后续还会讲一些在fuchsia中FIDL用于实现服务器端和客户端直接的通信，敬请关注。
