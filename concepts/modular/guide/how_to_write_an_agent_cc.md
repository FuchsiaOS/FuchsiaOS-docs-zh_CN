# How-To: Write an Agent in C++

Note: The Modular framework is being deprecated in favor of
the [Session Framework](/docs/concepts/session/introduction.md).

## Overview

An Agent is a component that runs without any direct user interaction. The lifetime of an Agent
component instance is bounded by its session.  A single agent instance is shared by all components
that ask for services from them.

Agents provide services to other components via their outgoing directory.

For legacy reasons, Agents can optionally expose the `fuchsia.modular.Agent`
service to receive new connections and provide services.

Agent components should implement the `fuchsia.modular.Lifecycle` service to
receive graceful termination signals and voluntarily exit.

## SimpleAgent

### fuchsia::modular::Agent Initialization

The first step to writing an Agent is setting up the scaffolding using the `modular::Agent` utility
class.

```c++
#include <lib/modular/cpp/agent.h>

int main(int /*argc*/, const char** /*argv*/) {
  async::Loop loop(&kAsyncLoopConfigAttachToCurrentThread);
  auto context = sys::ComponentContext::CreateAndServeOutgoingDirectory();
  // modular::Agent provides an implementation of fuchsia.modular.Lifecycle.
  // The agent can perform graceful teardown tasks in the callback.
  modular::Agent agent(context->outgoing(), [&loop] { loop.Quit(); });
  loop.Run();
  return 0;
}
```

The `modular::Agent` utility above implements and exposes `fuchsia.modular.Lifecycle`.

### Advertising the `Simple` Protocol

In order for the `SimpleAgent` to advertise the `Simple` protocol to other modular components,
it needs to expose it as an agent service. `sys::ComponentContext::outgoing()` provides a way to do
this:

```c++
  class SimpleImpl : Simple {
    SimpleImpl();
    ~SimpleImpl();

  private:
    void AMethod(AMethodResult reuslt) override{
      // do stuff
      result("all done");
    }
  };

  int main(int /*argc*/, const char** /*argv*/) {
    ...
    auto context = sys::ComponentContext::CreateAndServeOutgoingDirectory();

    SimpleImpl simple_impl;
    fidl::BindingSet<Simple> simple_bindings;

    context->outgoing()->AddPublicService(simple_bindings.GetHandler(&simple_impl));
    ...
  }
```

In the code above, `SimpleAgent` adds the `Simple` service as an outgoing service. Now, when a
component asks for the `Simple` service (see below), it will be served by `SimpleAgent`.

## Connecting to SimpleAgent

To connect to the `SimpleAgent` from a different component, a service mapping must be added to the
Modular [config](config.md) for your product:

```json
  agent_service_index = [
    {
      "service_name": "Simple",
      "agent_url": "fuchsia-pkg://url/to/your/agent"
    }
  ]
```

Then, in the component's implementation (i.e., `main.cc`):

```c++
auto sys_component_context = sys::ComponentContext::CreateAndServeOutgoingDirectory();
SimplePtr simple = sys_component_context->svc()->Connect<Simple>();
```

When your component asks for the `Simple` service, the `agent_service_index` is consulted.
The agent listed there is launched and is asked to provide the `Simple` service.

See the [SimpleModule](how_to_write_a_module_cc.md) guide for a more in-depth example.
