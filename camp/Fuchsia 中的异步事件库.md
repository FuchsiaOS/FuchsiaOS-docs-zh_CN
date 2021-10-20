# Fuchsia 中的异步事件库 -- async

事件是重要的概念，是指系统硬件或软件的状态出现任何重大改变，程序时刻都在触发和接收着各种事件：鼠标点击事件，键盘事件，socket 数据收发等等

我们常见的事件触发机制模型有两种：

一种是轮询机制模型，一种是事件驱动模型

在 Linux 下，事件驱动模型主要是由 poll、epoll 实现，Fuchsia 也提供了类似的事件驱动机制，其提供 async 库使用

## Epoll 的使用

先来回忆一下 Linux 下 Epoll + socket 的典型 C/S 架构的具体使用

```cc
// 创建Epoll实例

int epfd = epoll_crete(100);

// 添加需要监听事件和监听对象

epoll_ctl(epfd, EPOLL_CTL_ADD, listen_fd, &listen_event);



while(1)

{

    // 阻塞中

    int active_cnt = epoll_wait(epfd, events, 1000 ，-1);

    // 对监听到的事件处理

    for(i=0; i< active_cnt; i++)

    {

        // 新到连接

        if(events[i].data.fd == listen_fd)

        {

            connfd = accept(listen_fd,(sockaddr *)&clientaddr, &clilen);             ev.data.fd=connfd;

            ev.events=EPOLLIN|EPOLLET;

            epoll_ctl(epfd,EPOLL_CTL_ADD,connfd,&ev);//将新的fd添加到epoll的监听队列中

        }

        else if( events[i].events & EPOLLIN ) // 接收到数据，读socket

        {

            n = read(sockfd, line, MAXLINE)) < 0;

            ev.data.ptr = md;     // 添加数据

            ev.events=EPOLLOUT|EPOLLET;

            epoll_ctl(epfd,EPOLL_CTL_MOD,sockfd,&ev);//修改标识符，等待下一个循环时发送数据，异步处理的精髓

        }

        else if(events[i].events & EPOLLOUT) // 有数据待发送，写socket

        {

            struct myepoll_data* md = (myepoll_data*)events[i].data.ptr;    //取数据

            sockfd = md->fd;

            send( sockfd, md->ptr, strlen((char*)md->ptr), 0 );        //发送数据

            ev.data.fd=sockfd;

            ev.events=EPOLLIN|EPOLLET;

            epoll_ctl(epfd,EPOLL_CTL_MOD,sockfd,&ev);

        }

        else

        {

            //其他的处理

        }

    }

}
```



## 实例 -- echo 

在 `fuchsia/garnet/examples/fidl/echo_server_cpp` 中我们看到 `echo_server.cc` 源码和编译配置文件

```cc
#include "echo_server_app.h"



#include <lib/async-loop/cpp/loop.h>

#include <lib/async-loop/default.h>

#include <string>



int main(int argc, const char** argv) {

  async::Loop loop(&kAsyncLoopConfigAttachToCurrentThread);

  bool quiet = (argc >= 2) && std::string("-q") == argv[1];

  echo::EchoServerApp app(quiet);

  loop.Run();

  return 0;

}
executable("bin") {

  output_name = "echo_server_cpp"



  sources = [ "echo_server.cc" ]



  deps = [

    ":lib",

    "//zircon/system/ulib/async-default",

    "//zircon/system/ulib/async-loop:async-loop-cpp",

    "//zircon/system/ulib/async-loop:async-loop-default",

  ]

}
```

编译配置文件中引入 async 异步事件库作为 component 依赖，在主程序头文件引入相关声明

在 `echo_server_cpp.cc` 中先创建 async 对象，配置好相关信息后调用 `loop.Run();` 进入死循环一直等待事件的发生（除非使用 loop.quit 主动退出监听循环）

相对于 epoll，async 库的 loop 使用屏蔽了许多细节，接下来我们深入源码层级，来看看 async 库中 loop 做了什么。



## Dive into ansyc source code

`async-loop-cpp` 和 `async-loop-defaut` 库均来自 `fuchsia/zircon/system/ulib/async-loop` 的编译

```cc
// 核心数据结构 -- async_loop_t对象定义

typedef struct async_loop {

  async_dispatcher_t dispatcher;  // must be first (the loop inherits from async_dispatcher_t)

  async_loop_config_t config;     // immutable

  zx_handle_t port;               // immutable

  zx_handle_t timer;              // immutable



  _Atomic async_loop_state_t state;

  atomic_uint active_threads;  // number of active dispatch threads



  mtx_t lock;                  // guards the lists and the dispatching tasks flag

  bool dispatching_tasks;      // true while the loop is busy dispatching tasks

  list_node_t wait_list;       // most recently added first

  list_node_t task_list;       // pending tasks, earliest deadline first

  list_node_t due_list;        // due tasks, earliest deadline first

  list_node_t thread_list;     // earliest created thread first

  list_node_t irq_list;        // list of IRQs

  list_node_t paged_vmo_list;  // most recently added first

  bool timer_armed;            // true if timer has been set and has not fired yet

} async_loop_t;
// loop对象

class Loop {

 public:

  explicit Loop(const async_loop_config_t* config);



  Loop(const Loop&) = delete;

  Loop(Loop&&) = delete;

  Loop& operator=(const Loop&) = delete;

  Loop& operator=(Loop&&) = delete;



  ~Loop();



  async_loop_t* loop() const { return loop_; }



  async_dispatcher_t* dispatcher() const { return async_loop_get_dispatcher(loop_); }



  void Shutdown();



  zx_status_t Run(zx::time deadline = zx::time::infinite(), bool once = false);



  zx_status_t RunUntilIdle();



  void Quit();

  

  zx_status_t ResetQuit();



  async_loop_state_t GetState() const;

  

  zx_status_t StartThread(const char* name = nullptr, thrd_t* out_thread = nullptr);



  void JoinThreads();



 private:

  async_loop_t* loop_;

};
// 在loop的构造函数里 使用async_loop_create创建zx_status_t实例

Loop::Loop(const async_loop_config_t* config) {

  zx_status_t status = async_loop_create(config, &loop_);

  ZX_ASSERT_MSG(status == ZX_OK, "status=%d", status);

}
// Run方法调用async_loop_run

zx_status_t Loop::Run(zx::time deadline, bool once) {

  return async_loop_run(loop_, deadline.get(), once);

}



zx_status_t async_loop_run(async_loop_t* loop, zx_time_t deadline, bool once) {

  ZX_DEBUG_ASSERT(loop);



  zx_status_t status;

  atomic_fetch_add_explicit(&loop->active_threads, 1u, memory_order_acq_rel);

  do {

    status = async_loop_run_once(loop, deadline);

  } while (status == ZX_OK && !once);

  atomic_fetch_sub_explicit(&loop->active_threads, 1u, memory_order_acq_rel);

  return status;

}
```

### 创建 async_loop_t 对象

接下来，我们深入理解创建 async_loop_t 对象

```cc
zx_status_t async_loop_create(const async_loop_config_t* config, async_loop_t** out_loop) {

  ZX_DEBUG_ASSERT(out_loop);

  ZX_DEBUG_ASSERT(config != NULL);

  // If a setter was given, a getter should have been, too.

  ZX_ASSERT((config->default_accessors.setter != NULL) ==

            (config->default_accessors.getter != NULL));



  async_loop_t* loop = calloc(1u, sizeof(async_loop_t));

  if (!loop)

    return ZX_ERR_NO_MEMORY;

  atomic_init(&loop->state, ASYNC_LOOP_RUNNABLE);

  atomic_init(&loop->active_threads, 0u);



  loop->dispatcher.ops = (const async_ops_t*)&async_loop_ops;

  loop->config = *config;

  mtx_init(&loop->lock, mtx_plain);

  list_initialize(&loop->wait_list);

  list_initialize(&loop->irq_list);

  list_initialize(&loop->task_list);

  list_initialize(&loop->due_list);

  list_initialize(&loop->thread_list);

  list_initialize(&loop->paged_vmo_list);



  zx_status_t status =

      zx_port_create(config->irq_support ? ZX_PORT_BIND_TO_INTERRUPT : 0, &loop->port);

  if (status == ZX_OK)

    status = zx_timer_create(ZX_TIMER_SLACK_LATE, ZX_CLOCK_MONOTONIC, &loop->timer);

  if (status == ZX_OK) {

    *out_loop = loop;

    if (loop->config.make_default_for_current_thread) {

      ZX_DEBUG_ASSERT(loop->config.default_accessors.getter() == NULL);

      loop->config.default_accessors.setter(&loop->dispatcher);

    }

  } else {

    // Adjust this flag so we don't trip an assert trying to clear a default dispatcher we never

    // installed.

    loop->config.make_default_for_current_thread = false;

    async_loop_destroy(loop);

  }

  return status;

}
```

可以看到，我们先是为 async_loop_t 分配空间，接着初始化字段与监听对象、执行的需要异步任务、需要执行的超时的异步任务、中断请求等相关链表。接着创建监听端口，定时器等配置。最后一步返回创建结果。

### 事件监听循环与分发

Loop::Run 方法调用 async_loop_run 函数，这个函数关键部分在这里

```cc
  do {

    status = async_loop_run_once(loop, deadline);

  } while (status == ZX_OK && !once);
```

这个循环读取底层上报事件循环体，async_loop_run_once 每次阻塞地读取一个事件，接着解析事件包，分发给对应函数进行处理

```cc
static zx_status_t async_loop_run_once(async_loop_t* loop, zx_time_t deadline) {

  async_loop_state_t state = atomic_load_explicit(&loop->state, memory_order_acquire);

  if (state == ASYNC_LOOP_SHUTDOWN)

    return ZX_ERR_BAD_STATE;

  if (state != ASYNC_LOOP_RUNNABLE)

    return ZX_ERR_CANCELED;



  zx_port_packet_t packet;

  zx_status_t status = zx_port_wait(loop->port, deadline, &packet);

  if (status != ZX_OK)

    return status;



  if (packet.key == KEY_CONTROL) {

    // Handle wake-up packets.

    if (packet.type == ZX_PKT_TYPE_USER)

      return ZX_OK;



    // Handle task timer expirations.

    if (packet.type == ZX_PKT_TYPE_SIGNAL_ONE && packet.signal.observed & ZX_TIMER_SIGNALED) {

      return async_loop_dispatch_tasks(loop);

    }

  } else {

    // Handle wait completion packets.

    if (packet.type == ZX_PKT_TYPE_SIGNAL_ONE) {

      async_wait_t* wait = (void*)(uintptr_t)packet.key;

      mtx_lock(&loop->lock);

      list_delete(wait_to_node(wait));

      mtx_unlock(&loop->lock);

      return async_loop_dispatch_wait(loop, wait, packet.status, &packet.signal);

    }



    // Handle queued user packets.

    if (packet.type == ZX_PKT_TYPE_USER) {

      async_receiver_t* receiver = (void*)(uintptr_t)packet.key;

      return async_loop_dispatch_packet(loop, receiver, packet.status, &packet.user);

    }



    // Handle guest bell trap packets.

    if (packet.type == ZX_PKT_TYPE_GUEST_BELL) {

      async_guest_bell_trap_t* trap = (void*)(uintptr_t)packet.key;

      return async_loop_dispatch_guest_bell_trap(loop, trap, packet.status, &packet.guest_bell);

    }



    // Handle interrupt packets.

    if (packet.type == ZX_PKT_TYPE_INTERRUPT) {

      async_irq_t* irq = (void*)(uintptr_t)packet.key;

      return async_loop_dispatch_irq(loop, irq, packet.status, &packet.interrupt);

    }

    // Handle pager packets.

    if (packet.type == ZX_PKT_TYPE_PAGE_REQUEST) {

      async_paged_vmo_t* paged_vmo = (void*)(uintptr_t)packet.key;

      return async_loop_dispatch_paged_vmo(loop, paged_vmo, packet.status, &packet.page_request);

    }

  }

  ZX_DEBUG_ASSERT(false);

  return ZX_ERR_INTERNAL;

}
```

从 packet.type 我们可以判断出 async 库支持的事件类型有定时器任务、等待 wait、data packet 事件、bell trap 事件、中断请求事件、内存共享事件

对事件的添加与取消这里以定时器为例，详细函数实现在

`static zx_status_t async_loop_post_task(async_dispatcher_t* async, async_task_t* task) ` -- 添加定时器任务

`static zx_status_t async_loop_cancel_task(async_dispatcher_t* async, async_task_t* task)` -- 取消定时器任务

这里不过多赘述



### handler

以定时器事件为例，handler 为 `async_loop_dispatch_tasks`，这个 handler 是由调用者在添加事件监听时，构造 `async_task_t` 传入

```cc
static void async_loop_dispatch_task(async_loop_t* loop, async_task_t* task, zx_status_t status) {

  // Invoke the handler.  Note that it might destroy itself.

  async_loop_invoke_prologue(loop);

  task->handler((async_dispatcher_t*)loop, task, status);

  async_loop_invoke_epilogue(loop);

}
```