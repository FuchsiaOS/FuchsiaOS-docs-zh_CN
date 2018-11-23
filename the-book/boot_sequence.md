Fuchsia Boot Sequence
=====================

This document describes the boot sequence for Fuchsia from the time the Zircon
layer hands control over to the Garnet layer.  This document is a work in
progress that will need to be extended as we bring up more of the system.

# Layer 1: [appmgr](https://fuchsia.googlesource.com/garnet/+/master/bin/appmgr)

`appmgr`'s job is to host the environment tree and help create
processes in these environments.  Processes created by `appmgr`
have an `zx::channel` back to their environment, which lets them create other
processes in their environment and to create nested environments.

At startup, `appmgr` creates an empty root environment and creates
the initial apps listed in `/system/data/appmgr/initial.config` in
that environment. Typically, these applications create environments nested
directly in the root environment. The default configuration contains one initial
app: `bootstrap`.

# Layer 2: [sysmgr](https://fuchsia.googlesource.com/garnet/+/master/bin/sysmgr/)

`sysmgr`'s job is to create the boot environment and create a number of
 initial components in the boot environment.

The services that `sysmgr` offers in the boot environment are not provided by
bootstrap itself. Instead, when `sysmgr` receives a request for a service for
the first time, `sysmgr` lazily creates the appropriate app to implement that
service and routes the request to that app. The table of which components
implement which services is contained in the
`/system/data/bootstrap/services.config` file. Subsequent requests for the same
service are routed to the already running app. If the app terminates,
`sysmgr` will start it again the next time it receives a request for a
service implemented by that app.

`sysmgr` also runs a number of components in the boot environment at
startup. The list of components to run at startup is contained in the
`/system/data/bootstrap/apps.config` file.

# Layer 3: [basemgr](https://fuchsia.googlesource.com/peridot/+/master/bin/basemgr/)

`basemgr`'s job is to setup the interactive flow for user login and user
management.

It first gets access to the root view of the system, starts up Device Shell and
draws the Device Shell UI in the root view starting the interactive flow. It also
manages a user database that is exposed to Device Shell via the User Provider
FIDL API.

This API allows the Device Shell to add a new user, delete an existing user,
enumerate all existing users and login as an existing user or in incognito mode.

Adding a new user is done using an Account Manager service that can talk to an
identity provider to get an id token to access the user's
[Ledger](https://fuchsia.googlesource.com/peridot/+/master/bin/ledger/).

Logging-in as an existing user starts an instance of `user_runner` with that
user's id token and with a namespace that is mapped within and managed by
`basemgr`'s namespace.

Logging-in as a guest user (in incognito mode) starts an instance of
`user_runner` but without an id token and a temporary namespace.
