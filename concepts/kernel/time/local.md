# Local Time

Local time is an estimate of the standard time at the device’s location, aka
"wall clock time". Local time is derived from UTC and time zone. Local time is
more complex and has more failure modes than the other time standards so as a
developer you should prefer monotonic or UTC time unless there is an explicit
need to use local time. Please refer to [time overview](overview.md) for more
information on choosing between time standards and
[UTC overview](utc/overview.md) for more information on the implementation and
behavior of UTC for Fuchsia.

Time zone is maintained as one component of the internationalization settings
by [`setui_service`](/src/settings/service/meta/setui_service.cml) (for more information, see
[`fuchsia.settings.Intl`](https://fuchsia.dev/reference/fidl/fuchsia.settings#Intl)).
[`intl_services`](/src/intl/intl_services/meta/intl_services.cmx) exposes a more
convenient
[`fuchsia.intl.PropertyProvider`](https://fuchsia.dev/reference/fidl/fuchsia.intl#PropertyProvider)
FIDL interface that you may use to read the preferred time zone, read any
optional alternate time zones, and receive time zone change events.

Low level language runtimes such as C, C++, and Rust do not directly implement
local time because that would have required a dependency on
`fuchsia.intl.PropertyProvider` and ICU from all components written in these
languages. [`localtime`](/zircon/third_party/ulib/musl/src/time/localtime.c)
functions in the Fuchsia musl implementation always return a UTC time. If you
need to use local time from these languages you should use
[`fuchsia.intl.PropertyProvider.GetProfile`](https://fuchsia.dev/reference/fidl/fuchsia.intl#PropertyProvider.GetProfile)
to read the preferred timezone, use the
[ICU timezone data](development/internationalization/icu_data.md) to
determine the timezone offset, and then apply this offset to the current UTC
time.

The Dart and Flutter runners do support local time directly (because they are
wired up to a `fuchsia.intl.PropertyProvider`). If you are writing a Dart or
Flutter component you may use
[`DateTime.now().toLocal()`](https://api.flutter.dev/flutter/dart-core/DateTime/DateTime.now.html)
to read the current local time. The [intl/timezone](/src/tests/intl/timezone/)
tests ensure that a Dart component on a Fuchsia device can read the same local
time as a host device.

As with UTC, a Fuchsia device cannot produce a valid local time until it has
received a valid time from either the network or a real time clock. The time
zone is usually set based on user input so you should assume that the time zone
may change at any time and that the timezone is potentially inaccurate (i.e. the
reported time zone may not match the legally defined civil time at the device's
present location). Local time may jump either forwards or backwards, for example
at daylight savings time transitions.