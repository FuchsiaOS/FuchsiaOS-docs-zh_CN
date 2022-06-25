# Settings

There are often discrepancies between the functionality offered by a platform
and the desired behavior in the end-user experience. The platform surfaces
capabilities from the underlying device as building blocks towards larger
features. On the other hand, developers require higher level affordances built
on-top of these features, such as persistence and update publishing. In
isolation, implementing such functionality reinvents the wheel and increases
maintenance. When multiple parties are involved, coordination across experiences
quickly becomes an involved effort.

Settings bridges this divide by interacting with the platform in a manner
amenable to end-user experiences. Part of the [Fuchsia SDK](/sdk/fidl/fuchsia.settings/),
it provides a uniform model for accessing and controlling various aspects of the
platform. Supported features benefit from additional functionality commonly
needed by applications. As the source of truth, Settings ensures consistency
across its consumers. Finally, Settings is built for configurability; The
manifest and behavior of features can be tailored to the product’s need at both
build and runtime.