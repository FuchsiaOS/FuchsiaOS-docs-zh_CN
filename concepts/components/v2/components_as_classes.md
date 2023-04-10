# Understanding components through object-oriented design

[Fuchsia Components][component-intro] are composable units of software
execution that emphasize reuse, isolation, and testability.

This document provides an analogy between Fuchsia
Components and [Object-Oriented][oop] Design with [Dependency
Injection][dependency-injection]. This analogy allows Fuchsia developers
to apply their existing knowledge of Object-Oriented Design to develop
Fuchsia components using familiar terms and design patterns.

## Introduction

In [Object-Oriented Programming][oop] (OOP), an **object** is an entity
that contains both *data* and *methods* that operate on that that data. A
**class** defines the data and methods associated with a particular type
of object. An object is an instantiation of a class.

Similarly, **components** contain internal program state (data) and expose
protocols (groups of methods) that operate on their internal state. Where
a class declares the callable methods on an object, a **[component
manifest][component-manifests]** declares the callable protocols on a
component. Components are instantiated as **component instances**.

**Protocols**, defined using [FIDL][fidl], declare interfaces between
components. Providing a protocol means that a component implements that
protocol, similar to how classes may *implement* an interface or trait.

Note: Generally in Component Framework, "[capabilities][capabilities]" are
exposed, used, and offered. A common representation of a capability is a
Zircon channel that speaks a particular FIDL protocol, and for this reason
this document uses "protocol" instead of the "capability" terminology.

This document explores the analogy between components implementing
protocols and classes implementing interfaces, and this analogy extends
to the ways in which components and objects relate to other components
or objects.

Two important relationships are "Has-A" (in which one object is *composed
of* other objects), and "Depends-On/Uses-A" (in which one object requires
another object be present to properly operate).

Components may exhibit these same relationships. A single component may
be composed of multiple child components, and like in OOP the presence
of these children is an implementation detail of the component. Similar
to a class constructor that takes in a required object, component
manifests declare the protocols they depend on. The Component Framework
is concerned with how these dependent protocols are routed and satisfied
so that a component may execute, which is analogous to how [Dependency
Injection][dependency-injection] works for OOP.

The other common relationship in OOP is "Is-A" (Inheritance), where a
class can extend another class' data and logic. In Component Framework,
there is no analog to Inheritance.

Together, these similarities provide the following mapping between OOP
and Component Framework concepts:

Object-Oriented Concept | Component Concepts
-----                   | -----
Interface | FIDL protocol
Class Definition | Component manifest
Object | Component instance
Inner / Associated Class | FIDL Data
Depends-On/Uses-A Relationship (Dependency) | Capability routed from parent
Has-A Relationship (Composition) | Child component
Implements Interface | Expose capability from self
Is-A Relationship (Inheritance) | N/A, prefer "Implements"

Component Framework provides abilities that go above and beyond OOP,
but starting from OOP principles will give you a reasonable approximation
of your final component design.

The rest of this document provides more detail and examples of how to
map the above OOP concepts into Component concepts.

## FIDL protocols as interfaces

Many OOP languages have the concept of an **interface** or a **trait**
that can be *implemented* by an object. Where classes define data and
behavior, interfaces only *declare* behavior that may be present on
a class. Implementers of an interface can be grouped and used interchangeably.

On Fuchsia, FIDL defines interfaces between components. Similar to OOP
interfaces, implementers of FIDL Protocols can be used interchangeably.

### Example

Consider an interface called `Duck`, with a method called `Quack`.

The FIDL protocol is as follows:

```fidl
library fuchsia.animals;

@discoverable
protocol Duck {
  Quack();
}
```

To implement this protocol, a component would include the following
snippet in their component manifest:

```json5
{
  // ...

  // Declare that this component implements a protocol.
  capabilities: [
    {
      protocol: ["fuchsia.animals.Duck"],
    }
  ],

  // Expose this protocol so that other components may call it.
  expose: [
    {
      protocol: ["fuchsia.animals.Duck"],
      from: "self",
    }
  ],
}
```

In various OOP languages, you would write this as follows:

* {C++}

  ```cpp
  // Duck is an abstract class with a pure virtual method "Quack".
  class Duck {
    public:
      virtual void Quack() = 0;
  };

  // Actually implement Duck.
  class MallardDuck : public Duck {
    public:
      void Quack() override { /* ... */ }
  }
  ```


* {Dart}

  ```dart
  // All classes in Dart define an interface.
  // Omitting the definition of Quack means it can be overridden in a child.
  abstract class Duck {
    Quack();
  }

  MallardDuck implements Duck {
    @override
    Quack() { /* ... */ }
  }
  ```

* {Rust}

  ```rust
  // Rust uses "traits" rather than interfaces.
  // A trait may be explicitly implemented for any type.
  trait Duck {
    fn quack(&self);
  }

  // The type MallardDuck implements the "Duck" trait.
  struct MallardDuck {
  }

  impl Duck for MallardDuck {
    fn quack(&self) { /* ... */ }
  }
  ```

* {Java}

  ```java
  // Explicitly define a public Duck interface.
  public interface Duck {
    public void Quack();
  }

  // Create a class that implements the interface.
  public class MallardDuck {
    @Override
    public void Quack() { /* ... */ }
  }
  ```

## Component manifests as classes

The core concept of OOP is the **object**, which contains both data
and methods that operate on that data. [Class-based OOP][class-oop]
languages define hierarchies of *classes of objects* to describe data
and their relationships. A class may be instantiated into an object
multiple times, and they may be used modularly.

Similarly, a Fuchsia system is defined as a hierarchy of components,
each of which is defined by its **component manifest**. A component
manifest defines a *class of component* that can be instantiated and
used modularly.

Both objects and components represent a reusable unit of behavior and
data, grouped by interfaces that they implement.

### Example

Consider a component that rolls an N-sided die. That is, the component
returns a value in the range `[1, N]` when it is requested.

In Fuchsia, we define a protocol to be part of the component's interface:

```fidl
library fuchsia.dice;

// fuchsia.dice.Roller supports rolling a single die
protocol Roller {

  // Method "Roll" takes no arguments, and it returns an "outcome" that is a 64-bit unsigned integer.
  Roll() -> (struct {
    outcome uint64
  });
}
```

The manifest for your component is as follows:

```json5
// dice_roller.cml

{
  // The execution details for the component.
  //
  // This section says the component should be run as a normal ELF binary
  // (e.g. C++ or Rust) by executing the file at path "bin/dice_roller"
  // in the containing package.
  //
  // It also says to pass the command line arguments '"--sides" "6"' to
  // the program. It is the program's responsibility to parse its command
  // line parameters. In this case we want a 6-sided die.
  program: {
    runner: "elf",
    binary: "bin/dice_roller",
    args = [
      "--sides",
      "6",
    ],
  },

  // Declare the protocols the component implements (see previous section)
  capabilities: [{
    protocol: ["fuchsia.dice.Roller"],
  }],

  // Expose the protocols the component implements.
  expose: [{
    protocol: ["fuchsia.dice.Roller"],
    from: "self",
  }],
}
```

<!-- TODO: We should show a C++/Rust component implementing the protocol as well -->

An analogous class definition would be as follows:

* {C++}

  ```cpp
  class DiceRoller : public Roller {
    public:
      DiceRoller(uint64_t number_of_sides) : number_of_sides(number_of_sides) {}
      uint64_t Roll() override;

    private:
      const uint64_t number_of_sides;
  };
  ```

* {Dart}

  ```dart
  class DiceRoller implements dice.Roller {
      final int numberOfSides;
      DiceRoller({required this.numberOfSides});

      @override
      int Roll() { /* ... */ }
  }
  ```

* {Rust}

  ```rust
  pub struct DiceRoller {
      number_of_sides: u64,
  }

  impl DiceRoller {
    pub fn new(number_of_sides: u64) -> Self {
      Self{ number_of_sides }
    }
  }

  impl Roller for DiceRoller {
    pub fn roll(&self) -> u64 { /* ... */ }
  }
  ```

* {Java}

  ```java
  class DiceRoller implements Roller {
      private long numberOfSides;

      public DiceRoller(long numberOfSides) {
        this.numberOfSides = numberOfSides;
      }

      @Override
      public long Roll() { /* ... */ }
  }
  ```

In each of these examples, there is a `DiceRoller` that implements the
`Roll` method from a `Roller` interface. `DiceRoller` is parameterized by
its input argument that specifies the number of sides the die will have.

In the OOP examples it is possible to define a `DiceRoller` with any
arbitrary number of sides, but the component manifest specifies the
value 6.

## Component instances as objects, children as composition

An object is an instantiation of a class in an OOP language, and a
*component instance* is an instantiation of a *component* as defined by
its manifest. Both objects and components can be instantiated multiple
times, which allows them to be reused in different contexts.

Objects and component instances primarily differ in how they are instantiated.

In OOP languages, objects may be created by calling their
*constructor*, and in some languages objects are destroyed by calling a
*destructor*. Various strategies and design patterns exist to abstract
away object creation (such as the [Factory Pattern][factory-method]),
but in the end the object is always explicitly created somewhere.

By contrast, component instances are typically defined in a static
hierarchy. Simply specifying a component as a child of another component
is sufficient to [make the child *exist*][component-creation]. Existence
does not imply that the component is actually running, however. Typically
a component runs only when a something [binds][component-binding]
to a capability it exposes. In OOP terms, it would be as if an object
came into existence the first time a method was called on it (a type
of [Late Binding][late-binding] or [Lazy Initialization][lazy-init]).
Components have their own [lifecycle][lifecycle] which largely does not need to
be observed.

The exception to static component initialization is [dynamic
component collections][component-collections]. The collection itself is
statically defined, but components in the collection may be dynamically
[created][realm-create], bound to by [opening][realm-open] an exposed capability,
and [destroyed][realm-destroy]. This would be represented as a collection
holding objects in OOP, though the Component Framework gives you lazy binding
for free.

The state of a component is composed of its own state and that of their
children, similar to how object state is composed of its own state
and that of contained objects. The behavior of a component consists of
its own behavior and its interaction with children through protocols,
similar to how object behavior consists of its own behavior and its
interaction with contained objects through methods.

### Example

In this example there exists a hierarchy of objects representing a "user
session." `UserSession` consists of one `User` and multiple `Apps`.

This structure may be implemented using components as follows:

```json5
// user_session.cml
{
  // ...
  // user_session has a static child called "user", declared in user.cml
  children: [
    {
      name: "user",
      url: "fuchsia-pkg://fuchsia.com/session_example#meta/user.cm",
    },
  ],
  // user_session has a collection for dynamic components, called "apps"
  collections: [
    {
      name: "apps",
    }
  ],
  // access the User protocol from the child called "user".
  use: [
    {
      protocol: "fuchsia.session_example.User",
      from: "#user",
    },
  ],
}
```

```json5
// user.cml
{
  // ..

  // Expose the User capability, which provides information and actions on the current user.
  capabilities: [
    { protocol: "fuchsia.session_example.User" },
  ],
  expose: [
    {
      protocol: "fuchsia.session_example.User",
      from: "self",
    },
  ],
}
```

```cpp
// C++-like pseudocode for interacting with the child components from the session.

// Create any arbitrarily named app in the apps collection with just a URL to execute.
CreateChild("apps", "my_app", "fuchsia-pkg://..." /* url to the app to run */);

// Accessing exposed protocols causes the component to actually run. The
// output parameter is a Directory handle over which capabilities are accessed.
OpenExposedDir("apps", "my_app", &out_dir);

// Open any arbitrary capability on the bound component.
// Assuming that the "Controller" protocol has a method called "ExecuteCommand".
out_dir.Open("fuchsia.my_app.Controller").ExecuteCommand();

// Connect to the protocol from the static child.
// This is available in the incoming namespace for session, since it
// "uses" the capability.
incoming_namespace.Open("fuchsia.session_example.User").GetName();
```

The Component Framework allows any arbitrary component to be started in
the "apps" collection, so long as its dependencies are satisfied
(see later section).

* {C++}

  ```cpp
  // User is an abstract class representing a user of the session.
  // It declares the "GetName" method all users must implement.
  class User {
    public:
      virtual std::string GetName()  = 0;
  };

  // App is class representing the interface to apps.
  class App {
    /* ... */
  };

  class Session {
    public:
      Session() : user(/* initialize user */) {}

      void AddApp(App app) {
        apps.push_back(std::move(app));
      }

    private:
      std::unique_ptr<User> user;

      // Note that in C++ the collection needs to be typed, while in component
      // terms all components share a base type.
      std::vector<App> apps;
  };
  ```

* {Dart}

  ```dart

  interface User {
    String GetName();
  }

  class Session {
      final User user;
      final List<Object> apps = [];
      Session() :  user = /* initialize user */;

      // Similar to how all components share a "base type", Dart's Object
      // type can be dynamically cast to a desired interface.
      //
      // Casting will fail if the Object does not implement the type
      // requested, similar to how connecting to a non-exposed capability
      // fails for a component.
      void AddApp(Object app) {
        apps.add(app);
      }
  }
  ```

* {Rust}

  ```rust
  pub trait User {
      fn get_name() -> String;
  }

  pub trait App {
    /* ... */
  }

  pub struct Session {
    user: User,

    // Note that in Rust the collection needs to be typed, while in component
    // terms all components share a base type.
    apps: Vec<Box<dyn App>>;
  }

  impl Session {
    pub fn new() -> Self {
      Self{ user: /* initialize user */, apps: Vec::new() }
    }

    pub fn add_app(&mut self, app: Box<dyn App>) {
      self.apps.push(app);
    }
  }
  ```

* {Java}

  ```java
  interface User {
    String GetName();
  }

  class Session {
      private User user;
      private List<Object> apps = new ArrayList<Object>();

      public Session() {
        user = /* initialize user */;
      }

      // Similar to how all components share a "base type", Java's Object
      // type can be dynamically cast to a desired interface.
      //
      // Casting will fail if the Object does not implement the type
      // requested, similar to how connecting to a non-exposed capability
      // fails for a component.
      public void AddApp(Object app) {
        apps.add(app);
      }
  }
  ```

## FIDL data as inner or associated classes

It is common in OOP to have objects that act upon other objects. Previous
sections of this document focused on cases where the object uses a
dependency for additional behavior, but also important are cases where
an object depends on data stored in other objects. This is common in
container interfaces, where one object maintains a collection of other
objects and exposes an interface to manipulate the collection in some way.

Components are best suited to represent objects with complex behaviors
rather than act as data containers. FIDL provides the ability to express
extensible data structures that can be passed to and from protocols,
and these types are more suitable for representing data than components.

Generally, if an interface calls for acting upon [plain old data
types][plain-data], the data should be stored within the component,
declared using FIDL `tables`, and exposed by a protocol providing
accessors and mutators on the `table`.

[Builder interfaces][builder-pattern] that imperatively construct a data
type before executing an operation can also be represented best in FIDL.

### Example

In this example we will create a `Store` interface that contains a number
of `Items` for sale. Customers can create a `Cart` to which they add
items and eventually `Checkout()`.

```fidl
library fuchsia.store;

// An Item is a plain data type describing individual items in the store.
type Item = table {
  // Each Item has a unique ID, which is used to reference the object.
  1: id uint64;
  2: name string;
  3: price_in_cents uint32;
  4: quantity_in_stock: uint32;
}

type StoreError = strict enum {
  ITEM_NOT_FOUND = 1;
  INVALID_QUANTITY = 2;
};

protocol Store {
  // Add new items to the store.
  // No return code, so this operation is asynchronous and can fail silently.
  AddItem(struct {
    item: Item;
  });

  // Set the price on an existing item, by id.
  // Fails if the item is not found.
  SetPrice(struct {
    item_id: uint64;
    new_price: uint32;
  }) error StoreError;

  // Add (or subtract) additional stock of an item.
  // Fails if the item is not found or if you would be left with an
  // invalid quantity of the item.
  AddStock(struct {
    item_id: uint64;
    additional_quantity: int32;
  }) error StoreError;

  // Create a new Cart interface to shop at the store.
  // Note that this takes a "resource" struct, because request is a
  // Zircon handle.
  CreateCart(resource struct {
    request: server_end:Cart;
  });
};

type CartError = strict enum {
  PAYMENT_FAILURE = 1;
  NOT_ENOUGH_IN_STOCK = 2;
};

// Cart uses the builder pattern to create a set of items and checkout atomically.
protocol Cart {
  // Add a specific quantity of an item by id to the cart.
  AddItem(struct {
    item_id: uint64;
    quantity: uint32;
  });

  // Add a coupon code to the cart.
  AddCouponCode(struct {
    code: string;
  });

  // Checkout all previously added items atomically.
  // Fails if payment fails or if there are not enough items in stock
  // to satisfy the request.
  Checkout() error CartError;
};
```

```cpp
// Pseudo-code for interacting with the store.

StoreProxy store = connect_to<Store>();
store.AddItem(Item{.id = 1, .name = "Fuchsia Coffee Mug", .price_in_cents = 1299, .quantity_in_stock = 30});
store.AddItem(Item{.id = 2, .name = "Fuchsia Blanket", .price_in_cents = 3499, .quantity_in_stock = 10});
store.SetPrice({.item_id = 2, .new_price = 2999});
store.AddStock({.item_id = 1, .additional_quantity = -10});

CartProxy cart;
store.CreateCart(cart.NewRequest());
cart.AddItem({.item_id = 2, .quantity = 1});
cart.AddItem({.item_id = 1, .quantity = 5});
cart.AddCouponCode("FUCHSIA-ROCKS");
cart.Checkout();
```

The component implementing the `Store` interface is responsible for
maintaining the set of items according to the protocol's contract.

* {C++}

  ```cpp

  // Create a plain old data type for Item.
  struct Item {
    uint64_t id;
    std::string name;
    uint32_t price_in_cents;
    uint32_t quantity_in_stock;
  };

  // Enumerate the return values of cart operations.
  enum CartResult {
    OK = 0,
    PAYMENT_FAILURE = 1,
    NOT_ENOUGH_IN_STOCK = 2,
  };

  class Cart {
    public:
      // Cart is owned by a store, and it requires the pointer back to
      // its owner to implement Checkout.
      Cart(Store* store);

      // Adding items and coupon codes cannot fail.
      void AddItem(uint64_t item_id, uint32_t quantity);
      void AddCouponCode(std::string code);

      // Perform the checkout operation by acting upon store_ in some way.
      CartResult Checkout();

    private:
      // Create an inner class representing the pair of item id and quantity.
      struct ItemQuantity {
        uint64_t item_id;
        uint32_t quantity;
      };

      // The parent store, not owned.
      Store* store_;
      std::vector<ItemQuantity> item_requests_;
      std::vector<std::string> coupon_codes_;
  };

  // Enumerate return values of store operations.
  enum StoreResult {
    OK = 0,
    ITEM_NOT_FOUND = 1,
  };

  class Store {
    public:
      // Add new items to the store.
      void AddItem(Item item);

      // Set properties of items based on id.
      StoreResult SetPrice(uint64_t item_id, uint32_t new_price);
      StoreResult AddStock(uint64_t item_id, int32_t additional_quantity);

      // Create a new Cart for this store, referencing the Store that owns the Cart.
      // Carts are owned by a store, and must be deleted before the Store is.
      Cart* CreateCart() {
        carts_.emplace_back(Cart(this));
        return &cards_.back();
      }
    private:
      std::vector<Item> items_;
      std::vector<Cart> carts_;
  };
  ```

* {Dart}

  ```dart

  // Create a class containing the data for items.
  class Item {
    final int id;
    final String name;
    int priceInCents;
    int quantityInStock;

    Item({
      required this.id,
      required this.name,
      required this.priceInCents,
      this.quantityInStock = 0
    });
  }

  // Since Dart doesn't have tuples, create a pair type for id and quantity.
  class ItemQuantity {
    final int itemId;
    int quantity;

    ItemQuantity({required this.itemId, required this.quantity});
  }

  // Represent the various results for cart operations.
  enum CartResult {
    ok,
    paymentFailure,
    notEnoughInStock,
  }

  class Cart {
    final Store store;

    final List<ItemQuantity> _items = [];
    final List<String> _couponCodes = [];

    // A Cart needs to refer back to its Store to implement Checkout.
    Cart({required this.store});

    void AddItem(int itemId, int quantity) {
      _items.add(ItemQuantity(itemId: itemId, quantity: quantity);
    }

    void AddCouponCode(String code) {
      _couponCodes.add(code);
    }

    CartResult Checkout() { /* ... */ }
  }

  // Represent the results for store operations.
  enum StoreResult {
    ok,
    itemNotFound,
  }

  class Store {
    final List<Item> _items = [];
    final List<Cart> _carts = [];

    void AddItem(Item item) { _items.add(item); }

    StoreResult SetPrice(int item_id, int new_price) { /* ... */ }
    StoreResult AddStock(int item_id, int additional_quantity) { /* ... */ }

    // Create a cart that refers back to this owning store.
    Cart CreateCart() {
      var ret = Cart(this);
      _carts.add(ret);
      return ret;
    }
  }
  ```

* {Rust}

  ```rust

  // Create a data struct for Item information.
  pub struct Item {
    pub id: u64,
    pub name: String,
    pub price_in_cents: u32,
    pub quantity_in_stock: u32,
  }

  pub struct Cart {
    // Carts need to act on their parent Store, but we want to avoid cyclic references.
    // Use a Weak pointer so that the Store can be deleted independent of its Carts.
    // Mutex is used for interior mutability.
    store: Weak<Mutex<Store>>,
    items: Vec<(u64, u32)>,
    coupon_codes: Vec<String>,
  }

  impl Cart {
    pub fn new(store: Weak<Mutex<Store>>) -> Self {
      Self {
        store,
        items: vec![],
        coupon_codes: vec![],
      }
    }

    pub fn add_item(&mut self, item_id: u64, quantity: u32) {
      self.items.push((item_id, quantity));
    }

    pub fn add_coupon_code(&mut self, code: String) {
      self.coupon_codes.push(code);
    }

    // Checkout consumes the Cart builder and returns the result.
    pub fn checkout(self) -> Result<(), Error> { /* ... */ }
  }

  pub struct Store {
    items: Vec<Item>,

    // Note that we do not need to maintain ownership over Carts, since
    // they can exist independent of the Store they are from. Checkout will
    // presumably fail if the Store was deleted before it is called.
  }

  impl Store {
    pub fn new() -> Arc<Mutex<Self>> {
      Arc::new(Mutex::new(Self {
        items: vec![],
        carts: vec![],
      }));
    }

    pub fn add_item(&mut self, item: Item) { items.push(item); }
    pub fn set_price(&mut self, item_id: u64, new_price: u32) -> Result<(), Error> { /* ... */ }
    pub fn add_stock(&mut self, item_id: u64, additional_quantity: i32) -> Result<(), Error> { /* ... */ }

    pub fn create_cart(self: Arc<Mutex<Self>>) -> Cart {
      Cart::new(self.downgrade())
    }
  }
  ```

* {Java}

  ```java
  // Create a class containing the data for items.
  public class Item {
    public int id;
    public String name;
    public int priceInCents;
    public int quantityInStock;
  }

  // Since Java doesn't have tuples, create a pair type for id and quantity.
  class ItemQuantity {
    public int item_id;
    public int quantity;

    public ItemQuantity(int item_id, int quantity) {
      this.item_id = item_id;
      this.quantity = quantity;
    }
  }

  // Represent the various results for cart operations.
  public enum CartResult {
    OK,
    PAYMENT_FAILURE,
    NOT_ENOUGH_IN_STOCK,
  }

  // Represent the results for store operations.
  enum StoreResult {
    ok,
    itemNotFound,
  }

  class Store {
    private final List<Item> items = new ArrayList<Item>();
    private final List<Cart> carts = new ArrayList<Cart>();

    public void AddItem(Item item) { items.add(item); }

    public StoreResult SetPrice(int item_id, int new_price) { /* ... */ }
    public StoreResult AddStock(int item_id, int additional_quantity) { /* ... */ }

    public Cart CreateCart() {
      Cart ret = new Cart();
      carts.add(ret);
      return ret;
    }

    // Inner classes in Java can refer to their containing class.
    // This is needed to Checkout can act upon Store.this.
    public class Cart {
      private final List<ItemQuantity> items = new ArrayList<ItemQuantity>();
      private final List<String> couponCodes = new ArrayList<String>();

      void AddItem(int item_id, int quantity) {
        _items.add(ItemQuantity(item_id, quantity));
      }

      void AddCouponCode(String code) {
        _couponCodes.add(code);
      }

      CartResult Checkout() { /* ... */ }
    }
  }

  ```

## Capability routing as dependency injection

[Dependency Injection][dependency-injection] is a technique in which an
object's dependencies are passed as arguments to the object rather than
constructed or found by the object itself. This gives the creator of the
object control over the implementation of behavior the object depends
on. It is especially powerful to allow testing of objects that interact
with external services without actually calling those services in test
settings. One popular usage of dependency injection is passing a Time
interface to objects that would otherwise read the system clock. The
caller then has the ability to pass in an implementation that provides
a fixed time value for testing, and in production they pass in an
implementation that reads the real time.

The use of protocols between components fundamentally builds on top
of dependency injection techniques. Each component explicitly defines
the protocols it `uses`, and these protocols must be provided for the
component to be instantiated (similar to how OOP classes declare all
needed dependencies in their constructor).

Unlike some dependency injection frameworks where dependencies can be
constructed by a registry, all capabilities are explicitly routed from
sources to destinations by component manifests. Previous sections of this
document show how a component can implement an interface by `exposing` a
protocol. This gives the parent of that component the ability to `offer`
that protocol to other components in order to satisfy their dependencies
(the protocols they `use`).

The reasons for constructing components in this manner are similar to
that of OOP dependency injection: dependent behavior can be swapped as
needed for testing, evolution, and extensibility.

### Example

This example implements a `Purchaser` that needs to process credit
cards as part of a purchase flow. In some settings (such as testing)
you don't want to actually charge credit cards (which could get
very expensive)! Instead, we will provide a `CreditCardCharger` that
`Purchaser` uses to charge credit cards. In testing scenarios we provide
a fake `CreditCardCharger` that doesn't actually charge cards.

Note: A fake or mock implementation can store the values of
parameters so they can be verified. If you were to use something like
`charge_credit_card = false` and forgot to set the parameter, you could
cause some expensive bugs.

```fidl
// fuchsia.store.fidl

type PurchaseError = enum {
  CREDIT_CARD_FAILURE = 1;
  ITEM_NOT_FOUND = 2;
};

protocol Purchaser {
  // Purchase an item by name with a specific credit card.
  // Fails if the item is not found or if the credit card failed to charge.
  Purchase(struct {
    item_name: string,
    credit_card: string,
  }) error PurchaseError;
};

protocol CreditCardCharger {
  // Charge a specific credit card a specific amount.
  // Returns whether the charge is successful.
  Charge(struct {
    credit_card: string,
    amount: int,
  }) -> (struct { success: bool });
};
```

```json5
// purchaser.cml
{
  program: {
    // Instructions for how to run this component.
    /* ... */
  },

  capabilities: [
    { protocol: "fuchsia.store.Purchaser" }
  ],

  // Purchaser is a public interface implemented by this component.
  expose: [
    {
      protocol: "fuchsia.store.Purchaser",
      from: "self",
    }
  ],

  // CreditCardCharger is an interface required by this component to function.
  use: [
    { protocol: "fuchsia.store.CreditCardCharger" }
  ]
}
```

```json5
// real_credit_card_charger.cml
// Implements CreditCardCharger and actually charges credit cards.
{
  // ...
  capabilities: [
    { protocol: "fuchsia.store.CreditCardCharger" }
  ],

  expose: [
    {
      protocol: "fuchsia.store.CreditCardCharger",
      from: "self",
    }
  ],
}
```

```json5
// fake_credit_card_charger.cml
// Implements CreditCardCharger, but does not really charge anything.
{
  // ...
  capabilities: [
    {
      protocol: [
        "fuchsia.store.CreditCardCharger",
        // Interface to control the output of this fake component (FIDL not pictured here).
        "fuchsia.store.testing.CreditCardChargerController",
      ]
    }
  ],

  expose: [
    {
      protocol: [
        "fuchsia.store.CreditCardCharger",
        "fuchsia.store.testing.CreditCardChargerController",
      ],
      from: "self",
    }
  ],
}
```

```json5
// core.cml
//
// Actually add a "Purchaser" to the system, its dependencies, and route
// its protocol to some component implementing a purchase flow.
{
  children: [
    // ...
    {
      name: "purchaser",
      url: "fuchsia-pkg://fuchsia.com/purchaser#meta/purchaser.cml",
    },
    {
      // We want to use the real credit card charger so that we actually charge customers.
      name: "credit_card_charger"
      url: "fuchsia-pkg://fuchsia.com/real_credit_card_charger#meta/real_credit_card_charger.cml",
    },
    {
      name: "real_graphical_purchase_flow"
      url: /* ... */,
    },
  ],
  offer: [
    // Route protocols to satisfy every component's dependencies.
    {
      protocol: "fuchsia.store.CreditCardCharger",
      from: "#credit_card_charger",
      to: "#purchaser",
    },
    {
      protocol: "fuchsia.store.Purchaser",
      from: "#purchaser",
      to: "#real_graphical_purchase_flow",
    },
  ]
}
```

```json5
// test_purchaser.cml
{
  children: [
    {
      // We're going to test the real purchaser component, which is safe since we are mocking its dependency.
      name: "purchaser",
      url: "fuchsia-pkg://fuchsia.com/purchaser#meta/purchaser.cml",
    },
    {
      // We want to use the fake credit card charger so that we don't actually charge cards in tests.
      name: "credit_card_charger"
      url: "fuchsia-pkg://fuchsia.com/fake_credit_card_charger#meta/fake_credit_card_charger.cml",
    },
  ],
  offer: [
    {
      // Inject the fake charger as a dependency to purchaser.
      protocol: "fuchsia.store.CreditCardCharger",
      from: "#credit_card_charger",
      to: "#purchaser",
    }
  ],
  use: [
    {
      // Use Purchaser so we can test it
      protocol: "fuchsia.store.Purchaser",
      from: "#purchaser",
    },
    {
      // Use test charger so we can control what the credit card charger returns.
      protocol: "fuchsia.store.testing.CreditCardChargerController",
      from: "#credit_card_charger",
    },
  ]
}
```

```cpp
// Pseudo-code for test_purchaser

PurchaserProxy purchaser = open_service<Purchaser>();
CreditCardChargerController charger = open_service<CreditCardChargerController>();

// Make the card charger always return true, then test successful charge for an existing item.
charger.SetChargeResponse(true);
assert(purchaser.Purchase("existing item", "fake-card"), isNotError);

// Now test what happens when an item is missing.
// Depending on how advanced the mock charger is, we could even check
// that it was not called as a result of this invalid Purchase call.
assert(purchaser.Purchase("missing item", "fake-card"), PurchaseError.ITEM_NOT_FOUND);

// Make the charger return false and try again with an existing item.
// This allows us to test our error handling code paths.
charger.SetChargeResponse(false);
assert(purchaser.Purchase("existing item", "fake-card"), PurchaseError.CREDIT_CARD_FAILURE);
```

The above system would be implemented in an OOP language as follows:

* {C++}

  ```cpp
  class Purchaser final {
    public:
      // Purchaser takes as input the credit card charger to use.
      Purchaser(CreditCardCharger* credit_card_charger) :
        credit_card_charger_(credit_card_charger) {}
      PurchaseError Purchase(std::string item_name, std::string credit_card) {
        /* ... */
        // Use the injected credit card charger when needed.
        credit_card_charger_->Charge(std::move(credit_card), /* amount */);
        /* ... */
      }
    private:
      CreditCardCharger* credit_card_charger_;
  };

  // Abstract base class for concrete credit card chargers.
  class CreditCardCharger {
    public:
      virtual bool Charge(std::string credit_card, int amount) = 0;
  };

  class RealCreditCardCharger : public CreditCardCharger {
    public:
      bool Charge(std::string credit_card, int amount) override {
        /* actually charge credit cards somehow */
      }
  };

  class MockCreditCardCharger : public CreditCardCharger {
    public:
      // Mock implementation of CreditCardCharger::Charge that returns
      // a configurable error value and records the arguments of its
      // previous call.
      bool Charge(std::string credit_card, int amount) override {
        calls_++;
        last_credit_card_ = std::move(credit_card);
        last_amount_ = amount;
        return return_value_;
      }

      // Set the value that will be returned when calling Charge
      void SetReturnValue(bool return_value) { return_value_ = return_value; }

      // Get the parameters of the last call to Charge.
      const std::string& GetLastCreditCard() const { return last_credit_card_; }
      int GetLastAmount() const { return last_amount_; }
      size_t GetCallCount() const { return calls_; }

    private:
      bool return_value_ = true;
      size_t calls_ = 0;
      std::string last_credit_card_;
      int last_amount_ = 0;
  };


  // Production code
  int main() {
    auto charger = std::make_unique<RealCreditCardCharger>();
    Purchaser purchaser(charger.get());
    // use purchaser in the program flow

    /* ... */
  }

  // Test code (assuming GoogleTest)
  TEST(Purchaser, Success) {
    // Test that a purchase can succeed.
    // We expect that when a purchase is completed for an item costing
    // $100 that the CreditCardCharger is called with amount = 100.
    auto charger = std::make_unique<MockCreditCardCharger>();
    Purchaser purchaser(charger.get());
    EXPECT_EQ(PurchaseResult::OK, purchaser.Purchase("Item costing $100", "1234567890"));
    EXPECT_EQ(1, charger->GetCallCount());
    EXPECT_EQ("1234567890", charger->GetLastCreditCard());
    EXPECT_EQ(100, charger->GetLastAmount());
  }

  TEST(Purchaser, ItemNotFound) {
    // Test that we do not actually try to charge a credit card if the item is not found.
    auto charger = std::make_unique<MockCreditCardCharger>();
    Purchaser purchaser(charger.get());
    EXPECT_EQ(PurchaseResult::ITEM_NOT_FOUND, purchaser.Purchase("Not found item", "1234567890"));
    EXPECT_EQ(0, charger->GetCallCount());
  }

  TEST(Purchaser, CardChargeFailure) {
    // Test that a purchase can fail.
    auto charger = std::make_unique<MockCreditCardCharger>();
    Purchaser purchaser(charger.get());
    charger->SetReturnValue(false);
    EXPECT_EQ(PurchaseResult::CREDIT_CARD_FAILURE,
              purchaser.Purchase("Item costing $100", "1234567890"));
    EXPECT_EQ(1, charger->GetCallCount());
    EXPECT_EQ("1234567890", charger->GetLastCreditCard());
    EXPECT_EQ(100, charger->GetLastAmount());
  }
  ```

* {Dart}

  ```dart
  class Purchaser {
    final CreditCardCharger creditCardCharger;

    // Purchaser takes as input the credit card charger to use.
    Purchaser({required this.creditCardCharger});
    PurchaseError Purchase(String itemName, String creditCard) {
      /* ... */
      // Use the injected credit card charger when needed.
      creditCardCharger.Charge(creditCard, /* amount */);
      /* ... */
    }
  };

  // Abstract base class for concrete credit card chargers.
  abstract class CreditCardCharger {
    bool Charge(String creditCard, int amount);
  };

  class RealCreditCardCharger implements CreditCardCharger {
    @override
    bool Charge(String creditCard, int amount) {
      /* actually charge credit cards somehow */
    }
  };

  class MockCreditCardCharger implements CreditCardCharger {
    bool _returnValue = true;
    int _calls = 0;
    String _lastCreditCard = '';
    int _lastAmount = 0;

    // Mock implementation of CreditCardCharger::Charge that returns
    // a configurable error value and records the arguments of its
    // previous call.
    @override
    bool Charge(String creditCard, int amount) {
      _calls++;
      _lastCreditCard = creditCard;
      _lastAmount = amount;
      return _returnValue;
    }

    // Set the value that will be returned when calling Charge
    void set returnValue(int v) {
      _returnValue = v;
    }

    // Get the parameters of the last call to Charge.
    int get calls => _calls;
    String get lastCreditCard => _lastCreditCard;
    int get lastAmount => _lastAmount;
  };


  // Production code
  void main() {
    final charger = RealCreditCardCharger();
    Purchaser purchaser(creditCardCharger: charger);
    // use purchaser in the program flow

    /* ... */
  }

  // Test code (assuming package:test)
  import 'package:test/test.dart';

  void main() {
    group('Purchaser', () {
      test('succeeds', () {
        // Test that a purchase can succeed.
        // We expect that when a purchase is completed for an item costing
        // $100 that the CreditCardCharger is called with amount = 100.
        final charger = MockCreditCardCharger();
        final purchaser = Purchaser(creditCardCharger: charger);

        expect(purchaser.Purchase("Item costing $100", "1234567890"), PurchaseResult.ok);
        expect(charger.calls, 1);
        expect(charger.lastCreditCard, "1234567890");
        expect(charger.amount, 100);
      });

      test('fails when item is not found', () {
        // Test that we do not actually try to charge a credit card if the item is not found.
        final charger = MockCreditCardCharger();
        final purchaser = Purchaser(creditCardCharger: charger);

        expect(purchaser.Purchase("Not found item", "1234567890"), PurchaseResult.itemNotFound);
        expect(charger.calls, 0);
      });

      test('fails when card cannot be charged', () {
        // Test that a purchase can fail.
        final charger = MockCreditCardCharger();
        final purchaser = Purchaser(creditCardCharger: charger);

        charger.returnValue = false;
        expect(purchaser.Purchase("Item costing $100", "1234567890"), PurchaseResult.creditCardFailure);
        expect(charger.calls, 1);
        expect(charger.lastCreditCard, "1234567890");
        expect(charger.amount, 100);
      });
    });
  }
  ```

* {Rust}

  ```rust
  pub struct Purchaser {
    credit_card_charger: Box<dyn CreditCardCharger>,
  }

  impl Purchaser {
    // Purchaser takes as input the credit card charger to use.
    pub fn new(credit_card_charger: Box<dyn CreditCardCharger>) -> Self {
      Self { credit_card_charger }
    }

    pub fn purchase(&mut self, item_name: String, credit_card: String) {
      /* ... */
      // Use the injected credit card charger when needed.
      self.credit_card_charger.charge(creditCard, /* amount */);
      /* ... */
    }

    // For testing only, allow a Purchaser to be destroyed and converted
    // back to it CreditCardCharger.
    //
    // Alternatively, we could take a non-owning reference to the dependency.
    #[cfg(test)]
    pub fn to_charger(mut self) -> Box<dyn CreditCardCharger> {
      self.credit_card_charger
    }
  }

  // Trait implemented by concrete credit card chargers.
  trait CreditCardCharger {
    fn charge(credit_card: String, amount: i32) -> bool;
  }

  struct RealCreditCardCharger {}

  impl CreditCardCharger for RealCreditCardCharger {
    fn charge(&mut self, credit_card: String, amount: i32) -> bool {
      /* actually charge credit cards somehow */
    }
  };

  // Mock implementation of CreditCardCharger that returns
  // a configurable error value and records the arguments of its
  // previous call.
  pub struct MockCreditCardCharger {
    return_value: bool,
    calls: usize,
    last_credit_card: Option<String>,
    last_amount: Option<i32>,
  }

  impl MockCreditCardCharger {
    pub fn new() -> Self {
      Self {
        return_value: true,
        calls: 0,
        last_credit_card: None,
        last_amount: None,
      }
    }

    // Set the value that will be returned when calling charge
    pub fn set_return_value(&mut self, return_value: bool) {
      self.return_value = return_value;
    }

    // Get the parameters of the last call to charge.

    pub fn get_last_credit_card<'a>(&'a self) -> Option<&'a str> {
      self.last_credit_card.as_deref()
    }

    pub fn get_last_amount(&self) -> Option<i32> {
      self.last_amount.clone()
    }

    pub fn get_calls(&self) -> usize {
      self.calls
    }
  }

  impl CreditCardCharger for MockCreditCardCharger {
    fn charge(&mut self, credit_card: String, amount: i32) -> bool {
      self.calls += 1;
      self.last_credit_card = Some(credit_card);
      self.last_amount = Some(amount);
      self.return_value
    }
  }

  // Production code
  fn main() {
    let mut purchaser = Purchaser::new(Box::new(RealCreditCardCharger::new()));
    // use purchaser in the program flow
    /* ... */
  }

  // Test code (assuming Rust tests)

  #[cfg(test)]
  mod tests {
    #[test]
    fn success() {
      // Test that a purchase can succeed.
      // We expect that when a purchase is completed for an item costing
      // $100 that the CreditCardCharger is called with amount = 100.
      let mut purchaser = Purchaser::new(Box::new(MockCreditCardCharger::new()));
      assert_eq!(purchaser.purchase("Item costing $100", "1234567890"), PurchaseResult::OK);
      let charger = purchaser.to_charger();
      assert_eq!(charger.get_calls(), 1);
      assert_eq!(charger.get_last_credit_card(), Some("1234567890"));
      assert_eq!(charger.get_last_amount, Some(100i32));
    }

    #[test]
    fn item_not_found() {
      // Test that we do not actually try to charge a credit card if the item is not found.
      let mut purchaser = Purchaser::new(Box::new(MockCreditCardCharger::new()));
      assert_eq!(purchaser.purchase("Item costing $100", "1234567890"), PurchaseResult.ok);
      let charger = purchaser.to_charger();

      assert_eq!(purchaser.purchase("Not found item", "1234567890"), PurchaseResult::ITEM_NOT_FOUND);
      let charger = purchaser.to_charger();
      assert_eq!(charger.get_calls(), 0);
    }

    #[test]
    fn card_charge_fails() {
      // Test that a purchase can fail.

      let mut charger = Box::new(MockCreditCardCharger::new());
      charger.set_return_value(false);
      let mut purchaser = Purchaser::new(charger);
      assert_eq!(purchaser.purchase("Item costing $100", "1234567890"), PurchaseResult::CREDIT_CARD_FAILURE);
      let charger = purchaser.to_charger();

      assert_eq!(charger.get_calls(), 1);
      assert_eq!(charger.get_last_credit_card(), Some("1234567890"));
      assert_eq!(charger.get_last_amount, Some(100i32));
    }
  }
  ```

* {Java}

  ```java
  class Purchaser {
    private CreditCardCharger creditCardCharger;

    // Purchaser takes as input the credit card charger to use.
    public Purchaser(CreditCardCharger creditCardCharger) {
      this.creditCardCharger = creditCardCharger;
    }

    public PurchaseError Purchase(String itemName, String creditCard) {
      /* ... */
      // Use the injected credit card charger when needed.
      creditCardCharger.Charge(creditCard, /* amount */);
      /* ... */
    }
  };

  // Interface for concrete credit card chargers.
  interface CreditCardCharger {
    public boolean Charge(String creditCard, int amount);
  };

  class RealCreditCardCharger implements CreditCardCharger {
    @Override
    boolean Charge(String creditCard, int amount) {
      /* actually charge credit cards somehow */
    }
  };

  class MockCreditCardCharger implements CreditCardCharger {
    private boolean returnValue = true;
    private int calls = 0;
    private String lastCreditCard = '';
    private int lastAmount = 0;

    // Mock implementation of CreditCardCharger::Charge that returns
    // a configurable error value and records the arguments of its
    // previous call.
    @override
    public boolean Charge(String creditCard, int amount) {
      calls++;
      lastCreditCard = creditCard;
      lastAmount = amount;
      return returnValue;
    }

    // Set the value that will be returned when calling Charge
    public void setReturnValue(int v) {
      returnValue = v;
    }

    // Get the parameters of the last call to Charge.
    public int getCalls() { return calls; }
    public String getLastCreditCard() { return lastCreditCard; }
    public int getLastAmount() { return lastAmount; }
  };


  // Production code
  void main() {
    CreditCardCharger charger = new RealCreditCardCharger();
    Purchaser purchaser = new Purchaser(charger);
    // use purchaser in the program flow

    /* ... */
  }

  // Test code (assuming JUnit)

  public class PurchaserTest extends TestCase {
    protected MockCreditCardCharger charger;
    protected Purchaser purchaser;

    protected void setUp() {
      charger = new MockCreditCardCharger();
      purchaser = new Purchaser(charger);
    }

    public void testPurchaseSucceeds() {
      // Test that a purchase can succeed.
      // We expect that when a purchase is completed for an item costing
      // $100 that the CreditCardCharger is called with amount = 100.
      assertEquals(purchaser.Purchase("Item costing $100", "1234567890"), PurchaseResult.OK);
      assertEquals(charger.getCalls(), 1);
      assertEquals(charger.getLastCreditCard(), "1234567890");
      assertEquals(charger.getLastAmount(), 100);
    }

    public void testItemNotFoundError() {
      // Test that we do not actually try to charge a credit card if the item is not found.

      assertEquals(purchaser.Purchase("Not found item", "1234567890"), PurchaseResult.ITEM_NOT_FOUND);
      assertEquals(charger.getCalls(), 0);
    }

    public void testCardChargeFailure() {
      // Test that a purchase can fail.

      charger.returnValue = false;
      assertEquals(purchaser.Purchase("Item costing $100", "1234567890"), PurchaseResult.CREDIT_CARD_FAILURE);
      assertEquals(charger.getCalls(), 1);
      assertEquals(charger.getLastCreditCard(), "1234567890");
      assertEquals(charger.getLastAmount(), 100);
    }
  }
  ```

Mocking frameworks exist for many languages that handle setting return
values and inspecting call arguments. The above code demonstrates how
the functionality of those frameworks is implemented.

[builder-pattern]: https://en.wikipedia.org/wiki/Builder_pattern
[capabilities]: /docs/concepts/components/v2/capabilities/README.md
[class-oop]: https://en.wikipedia.org/wiki/Class-based_programming
[component-binding]: /docs/concepts/components/v2/lifecycle.md#binding
[component-collections]: /docs/concepts/components/v2/realms.md#collections
[component-creation]: /docs/concepts/components/v2/lifecycle.md#creating
[component-intro]: /docs/concepts/components/v2/introduction.md
[component-manifests]: /docs/concepts/components/v2/component_manifests.md
[dependency-injection]: https://en.wikipedia.org/wiki/Dependency_injection
[factory-method]: https://en.wikipedia.org/wiki/Factory_method_pattern
[fidl]: /docs/concepts/fidl/overview.md
[late-binding]: https://en.wikipedia.org/wiki/Late_binding
[lazy-init]: https://en.wikipedia.org/wiki/Lazy_initialization
[lifecycle]: /docs/concepts/components/v2/lifecycle.md
[oop]: https://en.wikipedia.org/wiki/Object-oriented_programming
[plain-data]: https://en.wikipedia.org/wiki/Passive_data_structure
[realm-open]: https://fuchsia.dev/reference/fidl/fuchsia.sys2#Realm.OpenExposedDir
[realm-create]: https://fuchsia.dev/reference/fidl/fuchsia.sys2#Realm.CreateChild
[realm-destroy]: https://fuchsia.dev/reference/fidl/fuchsia.sys2#Realm.DestroyChild
