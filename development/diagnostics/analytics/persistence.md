# Diagnostics Persistence Service

The Fuchsia Diagnostics Persistence Service persists Diagnostics data across
device reboots.

## How it works {#how-it-works}

When the service is triggered, Persistence reads sets of diagnostic data and
stores the data to disk. Persistence publishes the stored data in its own
Inspect hierarchy at the next boot.

Persistence is configured by entries in config files. Each entry specifies a
service name, a tag within that service, and a set of selectors. The service
name configures a FIDL service.

Requests to persist data are delivered to Persistence by FIDL, requesting that
that tag's selectors should be persisted. There's no periodic sampling. Each
message contains a single tag.

Each tag is limited in two ways - the size is capped, and the persistence rate
is throttled.

A request arriving too quickly will be deferred until the backoff has expired.
Multiple requests arriving during the backoff period will be combined into one.

Requests whose selectors fetch too much data will instead store an error
string, which will overwrite any previously stored data for that request.

## How to use Persistence {#how-to-use}

To persist your data and have it published, follow these steps:

### Define your data {#define-data}

Decide what data you want to persist. If it's not already being written to
Inspect, add code to publish it. Be aware that the data may be fetched by
Persistence some time after the request is sent, especially if the time
backoff is activated.

### Allowlist your data {#allowlist-data}

Persistence reads from `FeedbackArchiveAccessor`, so you'll need to have your
Inspect data allowlisted in a config file for that pipeline.

## Configure Persistence {#configure}

Put files into //src/diagnostics/config/persistence or
//vendor/*/diagnostics/config/persistence.
Files must be named *.persist. Add files to persistence_files in the BUILD.gn.

.persist files are in JSON5 format. Each file contains an array of objects.
Each object has the following schema:

```
{
        tag: 'any-name-you-like', // lowercase and hyphens only
        service_name: 'service-name', // lowercase and hyphens only
        max_bytes: 1000, // limit on size of JSON-format data persisted
        min_seconds_between_fetch: 60, // limit on frequency
        selectors: [
            'INSPECT:core/component:root/path:leaf_name',
            'INSPECT:core/component:root/*:another_leaf',
        ],
},
```

All fields are required. Data can be fetched from any component, but fetching
can only be requested by components that have the service routed to them.

Tag names must be unique per-service. Config file names are arbitrary but
can't be duplicated between config directories.

Note: These selectors must start with INSPECT, like Triage and Detect but
unlike allowlist selectors.

## Get Privacy approval {#privacy-approval}

Privacy needs to review Persistence configuration. To get a review,

*   Get the CL +2'd
*   Add cphoenix@ and miguelfrde@ to the CL.
*   We will +1 the CL and add the Privacy team to the CL.
*   Someone in Privacy will +2 the CL (or raise a concern).
*   When the CL is +2'd by Privacy (which may take about a week) it will show
    OWNERS approval and you can submit it.

Note: Data which has been approved for the Inspect allow-list may still not
be OK to persist to subsequent boots. For example, numbers such as
runtime-generated hashes could be used to link multiple boot records if
they were persisted.

## Route and use the service {#route-service}

Each service-name will be published as
fuchsia.diagnostics.persist.DataPersistence-service-name. This must be routed
to any component that will use a tag defined for that service.

The FIDL protocol for that service is in
//src/diagnostics/persistence/fidl/persist.fidl. The function call is

```
protocol DataPersistence {
    Persist(string:MAX_NAME_SIZE tag) -> (PersistResult result);
};
```

The desired result is `PersistResult::QUEUED`.

# Use the published data {#use-published-data}

On the next boot, the stored data will be published to Inspect after a delay
(currently 120 seconds).

For a selector of `INSPECT:core/test_component:root/path:number` with a service
of `my-service` and a tag of `my-tag`, the data will be found under
`core/diagnostics-persistence:root/persist/my-service/my-tag/core\test_component/root/path:number`.
