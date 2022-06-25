# Mock DDK Migration

The fake_ddk driver testing library is being replaced by a new library, mock_ddk.
The driver framework team needs help to migrate the 100+ driver tests in Fuchsia.

## Goal & motivation
The Fake DDK has a number of ambiguities around what is being tested and made a
number of assumptions about the driver structure that are increasingly invalid.
The mock_ddk provides a more straightforward unit testing framework, with clear
boundaries about what is able to be tested.

## Technical background

## Simple example

There are a number of fundamental differences between fake-ddk and mock-ddk,
but here is how a very simple test might be migrated:

* {Fake DDK}

  ```c++
  TEST(FooDevice, BasicTest) {
    fake_ddk::Bind ddk;
    device = std::make_unique<FooDevice>(fake_ddk::FakeParent(), other_args);
    ASSERT_EQ(device->Init(), ZX_OK);
    // Do some testing here
    device->DdkAsyncRemove();
    EXPECT_TRUE(ddk.Ok());
  }
  ```

* {Mock DDK}

  ```c++
  TEST(FooDevice, BasicTest) {
    std::shared_ptr<MockDevice> fake_parent = MockDevice::FakeRootParent();
    device = std::make_unique<FooDevice>(fake_parent.get(), other_args);
    ASSERT_EQ(device->Init(), ZX_OK);
    device.release(); // let go of the reference to the device
    // Do some testing here

    // The mock-ddk will automatically call DdkRelease() on any remaining children of
    // fake_parent upon destruction.
  }
  ```


### Overview of the Mock-DDK

The mock ddk exists simply as a set of `zx_device_t`’s that track the
interactions a device has with the mocked driver host, and allow calls
into the device.  There is no global state - if the root “parent” device
ever goes out of scope, all the `zx_device_t`’s will destruct and delete
their accompanying device.

Here is an interaction model of how the mock-ddk interacts with a driver:

![Figure: Interaction Model](/docs/development/drivers/testing/images/interaction_model.png)

### Major changes from the fake-ddk

#### Bookkeeping

fake_ddk | mock-ddk
---------|------------------------
All driver information is contained in global fake_ddk::Bind variable. | Information for each device is stored in the zx_device_t for that device.
Multiple drivers not supported | The zx_device_t maintains information about parents and children, so all devices descending from the same root parent can be discovered.
Does not hold a reference to the created device | Just like the driver host, each zx_device_t stores the device context

*See the section [Getting the Device Context](#getting-device-context) for how to get device context in mock-ddk.*


#### Imitating Driverhost behavior {: imitating-driverhost-behavior }

fake_ddk | mock-ddk
---------|------------------------
* Replicates the driverhost behavior for Init and Remove/Unbind/Release  |  * DriverHost behavior is kept to a minimum.
| * There is one built-in behavior: each `zx_device_t` will call release() on its device context upon destruction.

Note: Needing to manually call `InitOp()` and `UnbindOp()` is the most common cause for migration errors.
If your driver has an `Init()` function or does significant work in `Unbind()`, you will need to call those
ops explicitly. An example of this test is provided below in the section [An example lifecycle test](#lifecycle-test).

#### No more `fake_ddk::Bind::Ok()` function {: no-bind-ok-function }

The `Ok()` function was not actually testing correct use of driverhost protocols.
Although there is no drop in replacement for the `Ok()` function, the test writer
can start and stop the driver just like the driver host does, to ensure the device
state is initialized and shut down properly.  An example of this test is provided
below in the section [An example lifecycle test](#lifecycle-test).

### Using the Mock DDK

### Interactions with the Driverhost

The mock_ddk mocks out and makes available calls to and from the driverhost.

Calling into the device <br> (device ops)  | Calling out to the driverhost <br> (Libdriver API)
--------------------------------------|--------------------------------------
Call device ops through the MockDevice. Functions are named as op name + `Op` <br> **Example:** <br> Call the `init` function using `InitOp()`  | All calls in the libdriver API are recorded on the appropriate device, but no action is taken. <br> **Example:**<br> To test if `device_init_reply()` has been called, call `InitReplyCalled()`<br> or to wait on the call, `WaitUntilInitReplyCalled()`.


##### An example lifecycle test {: #lifecycle-test}

* {Fake DDK}

    ```c++
    fake_ddk::Bind bind;
    TestDevice* device  = TestDevice::Create(fake_ddk::kFakeParent);
    device->DdkAsyncRemove();
    EXPECT_TRUE(ddk_.Ok());
    device->DdkRelease();
    ```

* {Mock DDK}

    ```c++
    auto parent = MockDevice::FakeRootParent();
    TestDevice::Create(parent.get());
    // make sure the child device is there
    ASSERT_EQ(1, parent->child_count());
    auto* child = parent->GetLatestChild();
    // If your device has an init function:
    child->InitOp();
    // Use this if init replies asynchronously:
    EXPECT_EQ(ZX_OK,  child->WaitUntilInitReplyCalled());
    // Otherwise, can just verify init replied:
    EXPECT_TRUE(child->InitReplyCalled());
    // If your device has an unbind function:
    child->UnbindOp();
    // Use this if unbind replies asynchronously:
    EXPECT_EQ(ZX_OK, child->WaitUntilUnbindReplyCalled());
    // Otherwise, can just verify init replied:
    EXPECT_TRUE(child->UnbindReplyCalled());
    // Mock-ddk will release all the devices on destruction, or you can do it manually.
    ```

##### Automatically Unbind and Release {: #auto-unbind-release }
The driverhost will always call unbind before releasing a driver, but that
step must be done manually in the mock-ddk.
If you have multiple drivers under test, it may be easier to automate the
unbinding and releasing behavior.  The Mock DDK has a helper function for this
purpose:

```c++
auto parent = MockDevice::FakeRootParent();
TestDevice* test_device_0 = TestDevice::Create(parent.get());
TestDevice* test_device_1 = TestDevice::Create(test_device_0.zxdev());
// The state of the tree is now:
//         parent   <--  FakeRootParent
//           |
//         child    <--  test_device_0
//           |
//       grandchild <--  test_device_1

// You want to remove both test devices, by calling unbind and release in the right order?
device_async_remove(test_device_0.zxdev());

// ReleaseFlaggedDevices performs the unbind and release of any device
// below the input device that has had device_async_remove called on it.
mock_ddk::ReleaseFlaggedDevices(parent.get());
```



#### Getting Device Context {: #getting-device-context }
The mock-ddk only deals with the `zx_device_t`'s that are associated with a device.
However, if you have assigned a device context, by for example using
the ddktl library, you may want to access corresponding the ddk::Device:

```c++
  auto parent = MockDevice::FakeRootParent();
  // May not get the device* back from bind:
  TestDevice::Bind(parent.get());
  // Never fear! Recover device from parent:
  MockDevice* child = parent->GetLatestChild();
  TestDevice* test_dev =
         child->GetDeviceContext<TestDevice>();
```

#### Interactions with other drivers

Mocking out parent functionality uses mostly the same calls as the fake ddk, but
setting mocks only affect the devices involved, instead of loading mocks into a
global state.

##### Mocking Parent Protocols

Parent protocols are added to the parent before a child device is expected to
access them with a call to `device_get_protocol()`


* {Fake DDK}

     ```c++
     fake_ddk::Bind bind;
     const fake_ddk::Protocol kTestProto = {
       .ctx = reinterpret_cast<void*>(0x10),
       .ops = nullptr,
     };

     bind.SetProtocol(8, &kTestProto);
     ```

* {Mock DDK}

     ```c++
     auto parent = MockDevice::FakeRootParent();
     const void* ctx = reinterpret_cast<void*>(0x10),
     const void* ops = nullptr,

     parent->AddProtocol(8, ops, ctx);
     ```


##### Fragment protocols

Composite devices get protocols from multiple parent “fragments”.  This is manifested
in protocols being keyed by a name.  Mock-ddk allows binding a name to a protocol,
to indicate it comes from a fragment.


* {Fake DDK}

     ```c++
     fake_ddk::Bind bind;
         fbl::Array<fake_ddk::FragmentEntry> fragments(new fake_ddk::FragmentEntry[2], 2);
         fragments[0].name = "fragment-1";
         fragments[0].protocols.emplace_back(
             fake_ddk::ProtocolEntry{0, fake_ddk::Protocol{nullptr, nullptr}});
         fragments[0].protocols.emplace_back(
             fake_ddk::ProtocolEntry{1, fake_ddk::Protocol{nullptr, nullptr}});
         fragments[1].name = "fragment-2";
    fragments[1].protocols.emplace_back(
        fake_ddk::ProtocolEntry{2, fake_ddk::Protocol{nullptr, nullptr}});
    bind.SetFragments(std::move(fragments));
    ```

* {Mock DDK}

     ```c++
     auto parent = MockDevice::FakeRootParent();
     void* ctx = reinterpret_cast<void*>(0x10),
     void* ops = nullptr,
     // Mock-ddk uses the same call as adding a
     // normal parent protocol:
     parent->AddProtocol(0, ops, ctx, "fragment-1");
     parent->AddProtocol(1, ops, ctx, "fragment-1");
     parent->AddProtocol(2, ops, ctx, "fragment-2");
     ```




##### Mocking FIDL connections

If the device serves a FIDL protocol, the test may want to call the fidl
functions provided.  This can be difficult as the fidl functions take a
completer as an argument.  You can create a client to communicate with
the device class over a fidl channel.


* {Fake DDK}

     ```c++
    fake_ddk::Bind bind;
    TestDevice* dev  = TestDevice::Create(fake_ddk::kFakeParent);
    FidlMessenger fidl;
    fidl.SetMessageOp((void *)dev,
       [](void* ctx,
          fidl_incoming_msg_t* msg,
          fidl_txn_t* txn) -> zx_status_t
              { return static_cast<Device*>(ctx)->DdkMessage(msg, txn)});
    <fidl_client_function> (
        <fake_ddk>.local().get(), <args>);
    ```

* {Mock DDK}

    ```c++
    auto parent = MockDevice::FakeRootParent();
    TestDevice* dev  =  TestDevice::Create(parent.get());
    async::Loop loop_(&kAsyncLoopConfigNoAttachToCurrentThread);
    auto endpoints = fidl::CreateEndpoints<fidl_proto>();
    std::optional<fidl::ServerBindingRef<fidl_proto>> binding_;
    binding_ = fidl::BindServer(loop_.dispatcher(),
                                std::move(endpoints->server),
                                child->GetDeviceContext<RpmbDevice>());
    loop_.StartThread("thread-name")
    rpmb_fidl_.Bind(std::move(endpoints->client), loop_.dispatcher());
    ```


##### Mocking Metadata

Metadata can be added to any ancestor of the device under test.
Metadata is propagated to be available to all descendants.


* {Fake DDK}

    ```c++
    fake_ddk::Bind bind;
    const char kSource[] = "test";
    bind.SetMetadata(kFakeMetadataType,
                     kSource, sizeof(kSource));
    ```

* {Mock DDK}

    ```c++
    auto parent = MockDevice::FakeRootParent();
    const char kSource[] = "test";
    parent->SetMetadata(kFakeMetadataType,
                       kSource, sizeof(kSource));
    ```

##### Load Firmware

Load firmware is an deprecated function, but is included for
the drivers that still need it:

* {Fake DDK}

    ```
    No Functionality.
    ```

* {Mock DDK}

    ```c++
    auto parent = MockDevice::FakeRootParent();
    auto result = TestDevice::Bind(parent.get());
    TestDevice* test_device = result.value();
    constexpr std::string_view kFirmwarePath = "test path";
    std::vector<uint8_t> kFirmware(200, 42);
    test_device->zxdev()->SetFirmware(kFirmware, kFirmwarePath);
    EXPECT_TRUE(test_device->LoadFirmware(kFirmwarePath).is_ok());
    ```


## How to help

### Picking a task

The drivers that are remaining to be converted are listed on the [mock-ddk migration sheet](https://goto.google.com/mock-ddk-migration-table)

If you can't acces that sheet, you can find targets that depend on fake_ddk by running:
```bash
scripts/gn/trim_visibility.py --target="//src/devices/testing/fake_ddk
```

Or check the allowlist in `src/devices/testing/fake_ddk/BUILD.gn`.

### Doing a task

1. Assign the test to yourself in the [mock-ddk migration sheet](https://goto.google.com/mock-ddk-migration-table)
by putting your name in the “Owner” column.  If the driver test is listed as "Blocked",
there may be something preventing it from being migrated.
2. Change build rules and includes to target mock-ddk instead of fake_ddk

    ```bash
    $ sed -i 's%testing/fake_ddk%testing/mock-ddk%' path/to/BUILD.gn
    $ sed -i 's%<lib/fake_ddk/fake_ddk.h>%"src/devices/testing/mock-ddk/mock-device.h"%' test.cc
    ```

3. Remove the driver folder from the fake_ddk allowlist in `src/devices/testing/fake_ddk/BUILD.gn`
4. Change usage of `fake_ddk::Bind` to `auto fake_parent = MockDevice::FakeRootParent();`
    1. Note that you may need to be more careful with the scope of `fake_parent`
       since it does not involve any global variables.
    2. If your test creates a class that inherits from fake_ddk::Bind,
       you should find that mock-ddk supports the features that necessitated
       creating the subclass.  If not, please contact garratt@.
5. Change usage of `fake_ddk::kFakeParent` and `fake_ddk::FakeParent()` to `fake_parent.get()`
6. Remove usage of `fake_ddk::Bind::Ok()` (see [explanation](#no-bind-ok-function)
   above.) Instead, check the specific device state to ensure initialization
   and shutdown operate as expected. The example:
   [An example lifecycle test](#lifecycle-test) may be a good start.
7. Make sure you do not explicitly delete the test device, except by calling
   the `ReleaseOp`.  Doing so violates how the mock-ddk (and the driverhost)
   operates, and will result in a double free error.  (see section
   [Imitating Driverhost behavior](#imitating-driverhost-behavior) above)
8. Port `Bind::SetProtocol `and `Bind:SetFragments` to `MockDevice::AddProtocol.`
   Note that `MockDevice::AddProtocol` takes the ops and the context separately.
9. Mocking the metadata should be unchanged, although it is called on the device’s
   parent instead of `fake_ddk::Bind`.
10. Port the instances of `fake_ddk::FidlMessenger` to the mock-ddk equivalent.
11. This might be the first time someone has looked at this driver’s unit test
    in a while.  If the test seems lacking, (for example it only contains the
    one “lifecycle” test) please file a bug and add the label
    “improve_driver_unit_tests”.  In the bug, point out a few potential tests
    that could be written.
12. Add the test target to your build and test it.  The test should not require specific hardware.

### Common Issues:

* Not calling Init/Unbind
    * Call Init using the `MockDevice::InitOp()`
    * Call Unbind using the `MockDevice::UnbindOp()`, or call `device_async_remove()` and call `mock_ddk::ReleaseFlaggedDevices`
* Deleting the device directly
    * Solution: release the Device from the current scope after calling `DdkAdd()`


### Completing a task

* Upload the change, and copy the gerrit link to the
    [mock-ddk migration sheet](https://goto.google.com/mock-ddk-migration-table)
* When the CL is merged, mark the test as “Done” in the migration sheet.


## Examples:
Change List | Example of when a test...
-------------|--------------
[fxr/560643](https://fuchsia-review.googlesource.com/c/fuchsia/+/560643) | Created subclass of fake_ddk::Bind
[fxr/557553](https://fuchsia-review.googlesource.com/c/fuchsia/+/557553) | Used fake_ddk::FidlMessenger
[fxr/560246](https://fuchsia-review.googlesource.com/c/fuchsia/+/560246) | Uses SetMetadata and SetProtocol
[fxr/552027](https://fuchsia-review.googlesource.com/c/fuchsia/+/552027) | Called fake_ddk::Bind::Ok()

## Sponsors

If at any point you need help or have questions, please reach out to garratt@ or tq-df-eng@.
